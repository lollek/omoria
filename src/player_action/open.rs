use crate::model::{Cave, Item, ItemType};

/// Snapshot access and effects for opening doors and chests on the game thread.
trait OpenContext {
    /// Returns the cell and optional referenced item; invalid coordinates return `None`.
    fn read(&mut self, y: i64, x: i64) -> Option<(Cave, Option<Item>)>;
    /// Commits snapshots without redrawing; invalid coordinates or indices are ignored.
    fn write(&mut self, y: i64, x: i64, cell: Cave, item: Item);
    /// Returns the template that replaces a successfully opened closed door.
    fn open_door(&mut self) -> Item;
    /// Returns the player's current disarming skill, including equipment modifiers.
    fn disarm(&mut self) -> i64;
    /// Reports whether confusion prevents lockpicking (but not unlocked targets).
    fn confused(&mut self) -> bool;
    /// Delivers a player-facing message in action order.
    fn message(&mut self, message: &str);
    /// Grants lockpicking experience before the stat redraw.
    fn add_exp(&mut self, amount: i64);
    /// Refreshes player stats after granting experience.
    fn redraw_stats(&mut self);
    /// Refreshes the tile after its open-door state is committed.
    fn redraw(&mut self, y: i64, x: i64);
    /// Activates C chest traps, which may mutate or delete the referenced item.
    fn trigger_trap(&mut self, y: i64, x: i64);
    /// Calls C treasure generation at the chest's stored level, restoring dungeon level afterward.
    fn treasure(&mut self, y: i64, x: i64, flags: u64, level: i64);
}

#[cfg(not(test))]
mod globals;
#[cfg(not(test))]
mod interop;

#[cfg(test)]
fn open_with_rng(context: &mut impl OpenContext, rng: &mut impl rand::Rng, y: i64, x: i64) {
    open(
        context,
        &mut || crate::rng::randint_with_rng(rng, 100),
        y,
        x,
    );
}

fn pick_lock(
    context: &mut impl OpenContext,
    roll: &mut impl FnMut() -> i64,
    difficulty: i64,
    exp: i64,
) -> bool {
    let skill = context.disarm();
    if context.confused() {
        context.message("You are too confused to pick the lock.");
        return false;
    }
    if roll() >= skill - difficulty {
        context.message("You failed to pick the lock.");
        return false;
    }
    context.message("You have picked the lock.");
    context.add_exp(exp);
    context.redraw_stats();
    true
}

fn mark_empty(item: &mut Item) {
    let mut name = item.name_bytes();
    if let Some(index) = name.windows(2).position(|pair| pair == b" (") {
        name.truncate(index);
    }
    name.truncate(item.name.len() - b" (Empty)".len() - 1);
    name.extend_from_slice(b" (Empty)");
    if let Some(index) = name.iter().position(|&byte| byte == b'^') {
        name.remove(index);
    }
    item.set_name_bytes(&name);
    item.flags &= 0xFFFFFFFE;
    item.cost = 0;
}

fn open(context: &mut impl OpenContext, roll: &mut impl FnMut() -> i64, y: i64, x: i64) {
    let Some((mut cell, Some(mut item))) = context.read(y, x) else {
        context.message("I do not see anything you can open there.");
        return;
    };
    if item.tval == u8::from(ItemType::ClosedDoor) {
        if item.p1 > 0 {
            if pick_lock(context, roll, item.p1, 1) {
                item.p1 = 0;
                context.write(y, x, cell, item);
            }
        } else if item.p1 < 0 {
            context.message("It appears to be stuck.");
        }
        if item.p1 == 0 {
            cell.fopen = 1;
            let template = context.open_door();
            context.write(y, x, cell, template);
            context.redraw(y, x);
        }
    } else if item.tval == u8::from(ItemType::Chest) {
        if item.flags & 1 != 0
            && !pick_lock(
                context,
                roll,
                2 * i64::from(item.level),
                i64::from(item.level),
            )
        {
            return;
        }
        mark_empty(&mut item);
        context.write(y, x, cell, item);
        context.trigger_trap(y, x);
        if let Some((_, Some(item))) = context.read(y, x) {
            context.treasure(y, x, item.flags, item.p1);
            if let Some((cell, Some(mut item))) = context.read(y, x) {
                item.flags = 0;
                context.write(y, x, cell, item);
            }
        }
    } else {
        context.message("I do not see anything you can open there.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::cell::test_support::{blank_item, TestMap};
    use rand::{Rng, SeedableRng, StdRng};

    struct Context {
        map: TestMap,
        template: Item,
        skill: i64,
        confused: bool,
        messages: Vec<String>,
        events: Vec<&'static str>,
        exp: i64,
        destroys_chest: bool,
        treasure: Vec<(u64, i64)>,
    }

    impl Context {
        fn new() -> Self {
            let mut map = TestMap::reset();
            map.put(3, 4, 1, 105, b"a closed door");
            let mut template = blank_item();
            template.tval = 104;
            template.cost = 17;
            Self {
                map,
                template,
                skill: 200,
                confused: false,
                messages: Vec::new(),
                events: Vec::new(),
                exp: 0,
                destroys_chest: false,
                treasure: Vec::new(),
            }
        }

        fn open(&mut self) {
            open_with_rng(self, &mut StdRng::from_seed(&[1, 2, 3, 4][..]), 3, 4);
        }
    }

    impl OpenContext for Context {
        fn read(&mut self, y: i64, x: i64) -> Option<(Cave, Option<Item>)> {
            self.map.cells().read(y, x)
        }
        fn write(&mut self, y: i64, x: i64, cell: Cave, item: Item) {
            self.events.push("write");
            self.map.cells().write(y, x, cell, item);
        }
        fn open_door(&mut self) -> Item {
            self.template
        }
        fn disarm(&mut self) -> i64 {
            self.skill
        }
        fn confused(&mut self) -> bool {
            self.confused
        }
        fn message(&mut self, message: &str) {
            self.events.push("message");
            self.messages.push(message.to_owned());
        }
        fn add_exp(&mut self, amount: i64) {
            self.events.push("exp");
            self.exp += amount;
        }
        fn redraw_stats(&mut self) {
            self.events.push("stats");
        }
        fn redraw(&mut self, y: i64, x: i64) {
            assert_eq!(self.map.cave[y as usize][x as usize].fopen, 1);
            assert_eq!(self.map.items[1].tval, 104);
            self.events.push("redraw");
        }
        fn trigger_trap(&mut self, y: i64, x: i64) {
            let (_, Some(item)) = self.read(y, x).unwrap() else {
                panic!("Trap callback requires a surviving chest");
            };
            assert_eq!(item.flags & 1, 0);
            assert_eq!(item.cost, 0);
            assert!(crate::misc::c_string_lossy(&item.name).ends_with(" (Empty)"));
            self.events.push("trap");
            if self.destroys_chest {
                self.map.cave[y as usize][x as usize].tptr = 0;
            }
        }
        fn treasure(&mut self, _y: i64, _x: i64, flags: u64, level: i64) {
            self.events.push("treasure");
            self.treasure.push((flags, level));
        }
    }

    #[test]
    fn opens_unlocked_door_with_template_before_redraw() {
        let mut context = Context::new();
        context.map.cave[3][4].cptr = 2;
        context.open();
        assert_eq!(context.map.items[1].tval, 104);
        assert_eq!(context.map.items[1].cost, 17);
        assert_eq!(context.map.cave[3][4].fopen, 1);
        assert_eq!(context.map.cave[3][4].cptr, 2);
        assert_eq!(context.events, ["write", "redraw"]);
        assert!(context.messages.is_empty());
    }

    #[test]
    fn seeded_door_lockpick_success_grants_exp_before_opening() {
        let mut context = Context::new();
        context.map.items[1].p1 = 7;
        context.open();
        assert_eq!(context.messages, ["You have picked the lock."]);
        assert_eq!(context.exp, 1);
        assert_eq!(context.map.items[1].tval, 104);
        assert_eq!(
            context.events,
            ["message", "exp", "stats", "write", "write", "redraw"]
        );
    }

    #[test]
    fn lockpick_requires_strictly_more_than_the_seeded_roll() {
        let mut context = Context::new();
        context.map.items[1].p1 = 7;
        let mut rng = StdRng::from_seed(&[1, 2, 3, 4][..]);
        context.skill = 7 + crate::rng::randint_with_rng(&mut rng, 100);
        context.open();
        assert_eq!(context.messages, ["You failed to pick the lock."]);
        assert_eq!(context.map.items[1].p1, 7);
        assert_eq!(context.events, ["message"]);
        assert_eq!(context.exp, 0);
    }

    #[test]
    fn confusion_refuses_a_locked_door_without_using_rng() {
        let mut context = Context::new();
        context.map.items[1].p1 = 7;
        context.confused = true;
        let mut rng = StdRng::from_seed(&[1, 2, 3, 4][..]);
        let mut expected = rng;
        open_with_rng(&mut context, &mut rng, 3, 4);
        assert_eq!(context.messages, ["You are too confused to pick the lock."]);
        assert_eq!(context.map.items[1].p1, 7);
        assert_eq!(rng.gen::<u64>(), expected.gen::<u64>());
    }

    #[test]
    fn stuck_door_is_unchanged() {
        let mut context = Context::new();
        context.map.items[1].p1 = -20;
        context.open();
        assert_eq!(context.messages, ["It appears to be stuck."]);
        assert_eq!(context.map.items[1].p1, -20);
        assert_eq!(context.events, ["message"]);
    }

    #[test]
    fn non_openable_and_invalid_targets_are_refused() {
        let mut context = Context::new();
        context.map.items[1].tval = 80;
        for (row, col) in [(3, 4), (2, 2), (-1, 4), (1, 4), (6, 4), (3, i64::MAX)] {
            context.messages.clear();
            context.events.clear();
            open_with_rng(
                &mut context,
                &mut StdRng::from_seed(&[1, 2, 3, 4][..]),
                row,
                col,
            );
            assert_eq!(
                context.messages,
                ["I do not see anything you can open there."]
            );
            assert_eq!(context.events, ["message"]);
        }
        context.map.cave[3][4].tptr = u8::MAX;
        context.messages.clear();
        context.open();
        assert_eq!(
            context.messages,
            ["I do not see anything you can open there."]
        );
    }

    fn chest(context: &mut Context, flags: u64) {
        context.map.put(3, 4, 1, 2, b"& Small chest (Locked)");
        context.map.items[1].flags = flags;
        context.map.items[1].level = 5;
        context.map.items[1].p1 = 12;
        context.map.items[1].cost = 50;
    }

    #[test]
    fn locked_chest_success_marks_empty_before_trap_and_treasure() {
        let mut context = Context::new();
        chest(&mut context, 0x11000041);
        context.open();
        assert_eq!(context.messages, ["You have picked the lock."]);
        assert_eq!(context.exp, 5);
        assert_eq!(context.treasure, [(0x11000040, 12)]);
        assert_eq!(context.map.items[1].flags, 0);
        assert_eq!(context.map.items[1].cost, 0);
        assert_eq!(
            crate::misc::c_string_lossy(&context.map.items[1].name),
            "& Small chest (Empty)"
        );
        assert_eq!(
            context.events,
            ["message", "exp", "stats", "write", "trap", "treasure", "write"]
        );
    }

    #[test]
    fn chest_lockpick_failure_preserves_lock_and_contents() {
        let mut context = Context::new();
        chest(&mut context, 0x11000041);
        context.skill = 0;
        context.open();
        assert_eq!(context.messages, ["You failed to pick the lock."]);
        assert_eq!(context.map.items[1].flags, 0x11000041);
        assert_eq!(context.map.items[1].cost, 50);
        assert_eq!(context.events, ["message"]);
        assert_eq!(context.exp, 0);
    }

    #[test]
    fn confused_chest_lockpick_does_not_trigger_traps() {
        let mut context = Context::new();
        chest(&mut context, 1);
        context.confused = true;
        context.open();
        assert_eq!(context.messages, ["You are too confused to pick the lock."]);
        assert_eq!(context.events, ["message"]);
        assert_eq!(context.map.items[1].flags, 1);
    }

    #[test]
    fn unlocked_empty_chest_opens_even_when_confused() {
        let mut context = Context::new();
        chest(&mut context, 0);
        context.confused = true;
        context.open();
        context.open();
        assert!(context.messages.is_empty());
        assert_eq!(context.exp, 0);
        assert_eq!(context.treasure, [(0, 12), (0, 12)]);
        assert_eq!(
            crate::misc::c_string_lossy(&context.map.items[1].name),
            "& Small chest (Empty)"
        );
    }

    #[test]
    fn exploding_chest_skips_treasure_and_does_not_restore_deleted_item() {
        let mut context = Context::new();
        chest(&mut context, 0x80);
        context.destroys_chest = true;
        context.open();
        assert_eq!(context.map.cave[3][4].tptr, 0);
        assert!(context.treasure.is_empty());
        assert_eq!(context.events, ["write", "trap"]);
    }

    #[test]
    fn chest_name_without_suffix_preserves_bytes_and_removes_only_first_caret() {
        let mut context = Context::new();
        context.map.items[1].name.fill(0);
        context.map.put(3, 4, 1, 2, b"& \xE9^ chest^");
        context.open();
        let expected = b"& \xE9 chest^ (Empty)\0";
        assert_eq!(
            context.map.items[1].name[..expected.len()].to_vec(),
            expected
        );
    }

    #[test]
    fn long_unterminated_chest_name_is_bounded_and_terminated() {
        let mut context = Context::new();
        context.map.items[1].tval = 2;
        context.map.items[1].name.fill(b'x' as libc::c_char);
        context.open();
        let name = &context.map.items[1].name;
        assert_eq!(name[69], 0);
        assert_eq!(
            crate::misc::c_string_lossy(name),
            format!("{} (Empty)", "x".repeat(61))
        );
    }

    #[test]
    fn chest_unlock_preserves_the_legacy_32_bit_flag_mask() {
        let mut context = Context::new();
        chest(&mut context, 0x1_11000040);
        context.open();
        assert_eq!(context.treasure, [(0x11000040, 12)]);
        assert_eq!(context.map.items[1].flags, 0);
    }
}
