use crate::model::{Cave, Item, ItemType};

#[cfg(not(test))]
mod globals;
#[cfg(not(test))]
mod interop;

/// Snapshot access and ordered effects for disarming on the game thread.
trait DisarmContext {
    /// Returns a bounded cell and its optional item.
    fn read(&mut self, y: i64, x: i64) -> Option<(Cave, Option<Item>)>;
    /// Commits a surviving chest without redrawing.
    fn write(&mut self, y: i64, x: i64, cell: Cave, item: Item);
    /// Frees the floor trap and clears its cell reference and field mark.
    fn remove_trap(&mut self, y: i64, x: i64);
    /// Returns the current equipment-adjusted disarming skill.
    fn skill(&mut self) -> i64;
    fn is_blind(&mut self) -> bool;
    fn has_no_light(&mut self) -> bool;
    fn is_confused(&mut self) -> bool;
    /// Delivers messages in action order.
    fn message(&mut self, message: &str);
    /// Grants experience before subsequent movement or stat redraws.
    fn add_exp(&mut self, amount: i64);
    /// Calls the existing C movement path with the prompted direction.
    fn move_player(&mut self, direction: i64);
    /// Redraws the target after successful floor disarming and movement.
    fn redraw(&mut self, y: i64, x: i64);
    /// Redraws player stats after experience changes.
    fn redraw_stats(&mut self);
    /// Activates C chest traps after identification is committed.
    fn trigger_trap(&mut self, y: i64, x: i64);
}

#[cfg(test)]
fn disarm_with_rng(
    context: &mut impl DisarmContext,
    rng: &mut impl rand::Rng,
    y: i64,
    x: i64,
    direction: i64,
) {
    disarm(
        context,
        &mut |max| crate::rng::randint_with_rng(rng, max),
        y,
        x,
        direction,
    );
}

fn chest_is_locked(item: &Item) -> bool {
    item.flags & 1 != 0
}

fn chest_is_trapped(item: &Item) -> bool {
    item.flags & 0x1F0 != 0
}

fn disarm(
    context: &mut impl DisarmContext,
    roll: &mut impl FnMut(i64) -> i64,
    y: i64,
    x: i64,
    direction: i64,
) {
    let Some((cell, Some(mut item))) = context.read(y, x) else {
        context.message("I do not see anything to disarm there.");
        return;
    };
    let floor = item.tval == u8::from(ItemType::SeenTrap);
    if !floor && item.tval != u8::from(ItemType::Chest) {
        context.message("I do not see anything to disarm there.");
        return;
    }
    if !floor {
        if item.name_bytes().contains(&b'^') {
            context.message("I don't see a trap...");
            return;
        }
        if !chest_is_trapped(&item) {
            context.message("The chest was not trapped.");
            return;
        }
    }
    let mut skill = context.skill();
    if context.is_blind() {
        skill /= 5;
    } else if context.has_no_light() {
        skill /= 2;
    }
    if context.is_confused() {
        skill /= 3;
    }
    if skill - i64::from(item.level) <= roll(100) {
        if roll(skill) > 5 {
            context.message(if floor {
                "You failed to disarm the trap."
            } else {
                "You failed to disarm the chest."
            });
        } else if floor {
            context.message("You set the trap off!");
            context.move_player(direction);
        } else {
            context.message("You set a trap off!");
            crate::misc::remove_first_c_string_byte(&mut item.name, b'^');
            context.write(y, x, cell, item);
            context.trigger_trap(y, x);
        }
        return;
    }

    if floor {
        context.message("You have disarmed the trap.");
        context.add_exp(item.p1);
        context.remove_trap(y, x);
        context.move_player(direction);
        context.redraw(y, x);
    } else {
        item.flags &= 0xFFFFFE0F;
        let mut name = item.name_bytes();
        if let Some(index) = name.windows(2).position(|pair| pair == b" (") {
            name.truncate(index);
        }
        let suffix = if chest_is_locked(&item) {
            &b" (Locked)"[..]
        } else {
            &b" (Disarmed)"[..]
        };
        name.truncate(item.name.len() - suffix.len() - 1);
        name.extend_from_slice(suffix);
        item.set_name_bytes(&name);
        crate::misc::remove_first_c_string_byte(&mut item.name, b'^');
        context.write(y, x, cell, item);
        context.message("You have disarmed the chest.");
        context.add_exp(i64::from(item.level));
    }
    context.redraw_stats();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::cell::test_support::TestMap;
    use rand::{SeedableRng, StdRng};

    struct Context {
        map: TestMap,
        skill: i64,
        penalties: (bool, bool, bool),
        messages: Vec<String>,
        events: Vec<&'static str>,
        exp: i64,
        moves: Vec<i64>,
    }

    impl Context {
        fn new() -> Self {
            let mut map = TestMap::reset();
            map.put(3, 4, 1, 102, b"a trap");
            map.items[1].level = 7;
            map.items[1].p1 = 12;
            map.cave[3][4].fm = 1;
            Self {
                map,
                skill: 200,
                penalties: (false, false, false),
                messages: Vec::new(),
                events: Vec::new(),
                exp: 0,
                moves: Vec::new(),
            }
        }

        fn disarm(&mut self) {
            disarm_with_rng(self, &mut StdRng::from_seed(&[1, 2, 3, 4][..]), 3, 4, 6);
        }

        fn chest(&mut self, flags: u64, name: &[u8]) {
            self.map.items[1].name.fill(0);
            self.map.put(3, 4, 1, 2, name);
            self.map.items[1].flags = flags;
        }
    }

    impl DisarmContext for Context {
        fn read(&mut self, y: i64, x: i64) -> Option<(Cave, Option<Item>)> {
            self.map.cells().read(y, x)
        }
        fn write(&mut self, y: i64, x: i64, cell: Cave, item: Item) {
            self.events.push("write");
            self.map.cells().write(y, x, cell, item);
        }
        fn remove_trap(&mut self, y: i64, x: i64) {
            self.events.push("remove");
            self.map.cave[y as usize][x as usize].tptr = 0;
            self.map.cave[y as usize][x as usize].fm = 0;
        }
        fn skill(&mut self) -> i64 {
            self.skill
        }
        fn is_blind(&mut self) -> bool {
            self.penalties.0
        }
        fn has_no_light(&mut self) -> bool {
            self.penalties.1
        }
        fn is_confused(&mut self) -> bool {
            self.penalties.2
        }
        fn message(&mut self, message: &str) {
            self.events.push("message");
            self.messages.push(message.to_owned());
        }
        fn add_exp(&mut self, amount: i64) {
            self.events.push("exp");
            self.exp += amount;
        }
        fn move_player(&mut self, direction: i64) {
            self.events.push("move");
            self.moves.push(direction);
        }
        fn redraw(&mut self, _y: i64, _x: i64) {
            self.events.push("redraw");
        }
        fn redraw_stats(&mut self) {
            self.events.push("stats");
        }
        fn trigger_trap(&mut self, y: i64, x: i64) {
            assert!(
                !crate::misc::c_string_lossy(&self.read(y, x).unwrap().1.unwrap().name)
                    .contains('^')
            );
            self.events.push("trap");
            self.map.cave[y as usize][x as usize].tptr = 0;
        }
    }

    #[test]
    fn seeded_success_removes_floor_trap_before_moving_and_grants_exp() {
        let mut context = Context::new();
        context.disarm();
        assert_eq!(context.messages, ["You have disarmed the trap."]);
        assert_eq!(context.exp, 12);
        assert_eq!(context.map.cave[3][4].tptr, 0);
        assert_eq!(context.map.cave[3][4].fm, 0);
        assert_eq!(context.moves, [6]);
        assert_eq!(
            context.events,
            ["message", "exp", "remove", "move", "redraw", "stats"]
        );
    }

    #[test]
    fn equality_fails_and_second_roll_five_sets_off_but_six_does_not() {
        for (second, message, moves) in [
            (5, "You set the trap off!", vec![6]),
            (6, "You failed to disarm the trap.", vec![]),
        ] {
            let mut context = Context::new();
            context.skill = 50;
            let mut maxima = Vec::new();
            disarm(
                &mut context,
                &mut |max| {
                    maxima.push(max);
                    if maxima.len() == 1 {
                        43
                    } else {
                        second
                    }
                },
                3,
                4,
                6,
            );
            assert_eq!(maxima, [100, 50]);
            assert_eq!(context.messages, [message]);
            assert_eq!(context.moves, moves);
            assert_eq!(context.map.cave[3][4].tptr, 1);
            assert_eq!(context.exp, 0);
        }
    }

    #[test]
    fn success_rolls_once_when_skill_exceeds_roll_by_one() {
        let mut context = Context::new();
        context.skill = 50;
        let mut maxima = Vec::new();
        disarm(
            &mut context,
            &mut |max| {
                maxima.push(max);
                assert_eq!(maxima.len(), 1, "Success must not consume a second roll");
                42
            },
            3,
            4,
            6,
        );
        assert_eq!(maxima, [100]);
        assert_eq!(context.messages, ["You have disarmed the trap."]);
        assert_eq!(context.exp, 12);
    }

    #[test]
    fn long_chest_names_preserve_bytes_and_fit_complete_suffixes() {
        for (flags, suffix) in [(0x11, &b" (Locked)"[..]), (0x10, &b" (Disarmed)"[..])] {
            for length in [64, 70] {
                let mut context = Context::new();
                context.chest(flags, &vec![0xFF; length]);
                context.disarm();
                let name = &context.map.items[1].name;
                let prefix_len = name.len() - suffix.len() - 1;
                assert!(name[..prefix_len].iter().all(|&byte| byte == 0xFF));
                assert_eq!(name[prefix_len..name.len() - 1].to_vec(), suffix);
                assert_eq!(name[name.len() - 1], 0);
                assert_eq!(context.map.items[1].flags, flags & !0x1F0);
                assert_eq!(context.exp, 7);
            }
        }
    }

    #[test]
    fn failed_floor_disarm_keeps_trap_without_moving() {
        let mut context = Context::new();
        context.skill = 50;
        disarm(&mut context, &mut |max| max, 3, 4, 6);
        assert_eq!(context.messages, ["You failed to disarm the trap."]);
        assert_eq!(context.map.cave[3][4].tptr, 1);
        assert_eq!(context.map.cave[3][4].fm, 1);
        assert_eq!(context.exp, 0);
        assert_eq!(context.events, ["message"]);
    }

    #[test]
    fn seeded_set_off_moves_onto_existing_trap() {
        let mut context = Context::new();
        context.skill = 1;
        context.disarm();
        assert_eq!(context.messages, ["You set the trap off!"]);
        assert_eq!(context.map.cave[3][4].tptr, 1);
        assert_eq!(context.map.cave[3][4].fm, 1);
        assert_eq!(context.moves, [6]);
        assert_eq!(context.events, ["message", "move"]);
    }

    #[test]
    fn penalties_preserve_blind_precedence_and_integer_truncation() {
        let mut context = Context::new();
        context.skill = 59;
        for (penalties, expected) in [
            ((false, false, false), 59),
            ((false, true, false), 29),
            ((true, false, false), 11),
            ((true, true, false), 11),
            ((false, false, true), 19),
            ((false, true, true), 9),
            ((true, true, true), 3),
        ] {
            context.penalties = penalties;
            let mut maxima = Vec::new();
            disarm(
                &mut context,
                &mut |max| {
                    maxima.push(max);
                    max
                },
                3,
                4,
                6,
            );
            assert_eq!(maxima, [100, expected], "Wrong penalty for {penalties:?}");
        }
    }

    #[test]
    fn chest_success_clears_only_trap_flags_and_preserves_lock_and_name_bytes() {
        let mut context = Context::new();
        context.chest(0x800001F1, b"& \xffchest (Poison Needle)|");
        context.disarm();
        assert_eq!(context.messages, ["You have disarmed the chest."]);
        assert_eq!(context.map.items[1].flags, 0x80000001);
        assert_eq!(context.exp, 7);
        let name = &context.map.items[1].name;
        let expected = b"& \xffchest (Locked)";
        assert_eq!(name[..expected.len()].to_vec(), expected);
        assert_eq!(name[expected.len()], 0);
        assert_eq!(context.events, ["write", "message", "exp", "stats"]);
    }

    #[test]
    fn unlocked_chest_gets_disarmed_suffix() {
        let mut context = Context::new();
        context.chest(0x10, b"a chest");
        context.disarm();
        assert_eq!(
            crate::misc::c_string_lossy(&context.map.items[1].name),
            "a chest (Disarmed)"
        );
    }

    #[test]
    fn chest_set_off_identifies_before_callback_without_stale_write() {
        let mut context = Context::new();
        context.skill = 1;
        context.chest(0x10, b"a chest (Poison Needle)|");
        context.disarm();
        assert_eq!(context.messages, ["You set a trap off!"]);
        assert_eq!(context.events, ["message", "write", "trap"]);
        assert_eq!(context.map.cave[3][4].tptr, 0);
        assert_eq!(
            crate::misc::c_string_lossy(&context.map.items[1].name),
            "a chest (Poison Needle)|"
        );
        assert_eq!(context.exp, 0);
    }

    #[test]
    fn chest_failure_keeps_flags_and_name() {
        let mut context = Context::new();
        context.skill = 50;
        context.chest(0x10, b"a chest (Poison Needle)|");
        let name = context.map.items[1].name;
        disarm(&mut context, &mut |max| max, 3, 4, 6);
        assert_eq!(context.messages, ["You failed to disarm the chest."]);
        assert_eq!(context.map.items[1].flags, 0x10);
        assert_eq!(context.map.items[1].name, name);
        assert_eq!(context.events, ["message"]);
    }

    #[test]
    fn unknown_and_untrapped_chests_do_not_roll() {
        let mut context = Context::new();
        for (flags, name, message) in [
            (0x10, &b"a chest^"[..], "I don't see a trap..."),
            (1, &b"a chest (Locked)"[..], "The chest was not trapped."),
        ] {
            context.chest(flags, name);
            disarm(
                &mut context,
                &mut |_| panic!("Refusal must not roll"),
                3,
                4,
                6,
            );
            assert_eq!(context.messages.last().unwrap(), message);
            assert_eq!(context.map.items[1].flags, flags);
        }
    }

    #[test]
    fn missing_unseen_non_trap_and_invalid_targets_are_refused_without_rng() {
        let mut context = Context::new();
        for tval in [0, 101, 105] {
            context.map.items[1].tval = tval;
            disarm(
                &mut context,
                &mut |_| panic!("Refusal must not roll"),
                3,
                4,
                6,
            );
            assert_eq!(
                context.messages.last().unwrap(),
                "I do not see anything to disarm there."
            );
        }
        for (y, x) in [(3, 3), (1, 4), (-1, 4), (3, i64::MAX)] {
            disarm(
                &mut context,
                &mut |_| panic!("Refusal must not roll"),
                y,
                x,
                6,
            );
            assert_eq!(
                context.messages.last().unwrap(),
                "I do not see anything to disarm there."
            );
        }
    }
}
