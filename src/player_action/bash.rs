use crate::model::{Cave, Item, ItemType};

#[cfg(not(test))]
mod globals;
#[cfg(not(test))]
mod interop;

/// Keeps legacy production RNG sources separate while allowing seeded tests.
trait BashRandom {
    /// Rolls in `[1, max]`, using the legacy zero result for nonpositive maxima.
    fn roll(&mut self, max: i64) -> i64;
    /// Uses the melee hit calculation with zero level and to-hit modifiers.
    fn hit(&mut self, base: i64, armor: i64) -> bool;
}

impl<R: rand::Rng> BashRandom for R {
    fn roll(&mut self, max: i64) -> i64 {
        crate::rng::randint_with_rng(self, max)
    }

    fn hit(&mut self, base: i64, armor: i64) -> bool {
        crate::combat::fighting::managed_to_hit_with_rng(self, base, 0, 0, armor)
    }
}

/// Snapshot access and synchronous effects for bashing on the game thread.
trait BashContext {
    /// Returns cell/item copies; invalid coordinates return `None`.
    fn read(&mut self, y: i64, x: i64) -> Option<(Cave, Option<Item>)>;
    /// Commits the snapshots without redrawing; invalid targets are ignored.
    fn write(&mut self, y: i64, x: i64, cell: Cave, item: Item);
    /// Returns the template for a successfully bashed-open door.
    fn open_door(&mut self) -> Item;
    /// Returns current strength, including temporary stat modifiers.
    fn strength(&mut self) -> i64;
    /// Returns player body weight for the door hit calculation.
    fn weight(&mut self) -> i64;
    /// Delivers a player-facing message in action order.
    fn message(&mut self, message: &str);
    /// Refreshes a tile after its open-door state is committed.
    fn redraw(&mut self, y: i64, x: i64);
    /// Replaces the paralysis counter rather than adding to it.
    fn paralyze(&mut self, turns: i64);
    /// Reports whether fear prevents attacking a monster target.
    fn afraid(&mut self) -> bool;
    /// Returns current dexterity for the off-balance check.
    fn dexterity(&mut self) -> i64;
    /// Copies the current primary weapon for restoration after the attack.
    fn primary(&mut self) -> Item;
    /// Replaces the primary weapon without recalculating player bonuses.
    fn set_primary(&mut self, item: Item);
    /// Returns the blank treasure template used for the temporary bash weapon.
    fn blank_item(&mut self) -> Item;
    /// Copies the shield whose damage is used by the temporary bash weapon.
    fn shield(&mut self) -> Item;
    /// Calls C combat synchronously; returns whether a hit monster remains alive.
    fn attack(&mut self, y: i64, x: i64) -> bool;
    /// Calls C stun with a -10 save bonus and a two-turn duration.
    fn stun(&mut self, monster: u8);
}

fn bash_with_rng(context: &mut impl BashContext, rng: &mut impl BashRandom, y: i64, x: i64) {
    let Some((mut cell, item)) = context.read(y, x) else {
        context.message("I do not see anything you can bash there.");
        return;
    };
    if cell.cptr > 1 {
        if context.afraid() {
            context.message("You are afraid!");
            return;
        }
        let previous = context.primary();
        let mut weapon = context.blank_item();
        weapon.damage = context.shield().damage;
        weapon.weight = ((context.strength() * 10 + 20) * 100) as u16;
        weapon.tval = 1;
        context.set_primary(weapon);
        if context.attack(y, x) {
            if let Some((cell, _)) = context.read(y, x) {
                context.stun(cell.cptr);
            }
        }
        context.set_primary(previous);
        if rng.roll(300) > context.dexterity() * 10 {
            context.message("You are off-balance.");
            context.paralyze(rng.roll(3));
        }
        return;
    }
    let Some(mut item) = item else {
        context.message("I do not see anything you can bash there.");
        return;
    };
    if item.tval == u8::from(ItemType::ClosedDoor) {
        let from_strength = context.strength() * 10;
        if rng.hit(
            context.weight() + from_strength * from_strength / 500,
            item.p1.abs() + 150,
        ) {
            context.message("You smash into the door! The door crashes open!");
            let mut door = context.open_door();
            door.p1 = 1;
            cell.fopen = 1;
            context.write(y, x, cell, door);
            context.redraw(y, x);
        } else {
            context.message("You smash into the door! The door holds firm.");
            context.paralyze(2);
        }
    } else if item.tval == u8::from(ItemType::Chest) {
        if rng.roll(10) == 1 {
            context.message("You have destroyed the chest...");
            context.message("and its contents!");
            for (destination, byte) in item.name.iter_mut().zip(b"& ruined chest\0") {
                *destination = *byte as libc::c_char;
            }
            item.flags = 0;
            context.write(y, x, cell, item);
        } else if item.flags & 1 != 0 && rng.roll(10) == 1 {
            context.message("The lock breaks open!");
            item.flags &= 0xFFFFFFFE;
            context.write(y, x, cell, item);
        }
    } else {
        context.message("I do not see anything you can bash there.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::cell::test_support::{blank_item, TestMap};
    use rand::{SeedableRng, StdRng};

    struct Context {
        map: TestMap,
        template: Item,
        messages: Vec<String>,
        redraws: Vec<(i64, i64)>,
        paralysis: i64,
        strength: i64,
        weight: i64,
        afraid: bool,
        dexterity: i64,
        primary: Item,
        shield: Item,
        attacks: Vec<(i64, i64)>,
        attack_result: bool,
        stuns: Vec<u8>,
        equipment_writes: usize,
    }

    impl Context {
        fn new() -> Self {
            let mut map = TestMap::reset();
            map.put(3, 4, 1, 105, b"a closed door");
            let mut template = blank_item();
            template.tval = 104;
            template.cost = 17;
            let mut primary = blank_item();
            primary.tval = 10;
            primary.cost = 42;
            primary.flags = 123;
            let mut shield = blank_item();
            shield.damage[0] = b'2' as libc::c_char;
            Self {
                map,
                template,
                messages: Vec::new(),
                redraws: Vec::new(),
                paralysis: 0,
                strength: 18,
                weight: 1000,
                afraid: false,
                dexterity: 30,
                primary,
                shield,
                attacks: Vec::new(),
                attack_result: true,
                stuns: Vec::new(),
                equipment_writes: 0,
            }
        }

        fn bash(&mut self) {
            bash_with_rng(self, &mut StdRng::from_seed(&[1, 2, 3, 4][..]), 3, 4);
        }
    }

    impl BashContext for Context {
        fn read(&mut self, y: i64, x: i64) -> Option<(Cave, Option<Item>)> {
            self.map.cells().read(y, x)
        }
        fn write(&mut self, y: i64, x: i64, cell: Cave, item: Item) {
            self.map.cells().write(y, x, cell, item);
        }
        fn open_door(&mut self) -> Item {
            self.template
        }
        fn strength(&mut self) -> i64 {
            self.strength
        }
        fn weight(&mut self) -> i64 {
            self.weight
        }
        fn message(&mut self, message: &str) {
            self.messages.push(message.to_owned());
        }
        fn redraw(&mut self, y: i64, x: i64) {
            assert_eq!(self.map.cave[y as usize][x as usize].fopen, 1);
            assert_eq!(self.map.items[1].tval, 104);
            self.redraws.push((y, x));
        }
        fn paralyze(&mut self, turns: i64) {
            assert_eq!(
                self.primary.tval, 10,
                "Restore the weapon before off-balance effects"
            );
            self.paralysis = turns;
        }
        fn afraid(&mut self) -> bool {
            self.afraid
        }
        fn dexterity(&mut self) -> i64 {
            self.dexterity
        }
        fn primary(&mut self) -> Item {
            self.primary
        }
        fn set_primary(&mut self, item: Item) {
            self.equipment_writes += 1;
            self.primary = item;
        }
        fn blank_item(&mut self) -> Item {
            blank_item()
        }
        fn shield(&mut self) -> Item {
            self.shield
        }
        fn attack(&mut self, y: i64, x: i64) -> bool {
            assert_eq!(self.primary.tval, 1);
            assert_eq!(self.primary.weight, 20000);
            assert_eq!(self.primary.damage, self.shield.damage);
            assert_eq!(self.primary.flags, 0);
            assert_eq!(self.primary.cost, 0);
            self.attacks.push((y, x));
            self.attack_result
        }
        fn stun(&mut self, monster: u8) {
            self.stuns.push(monster);
        }
    }

    #[test]
    fn seeded_door_success_replaces_item_and_redraws_open_cell() {
        let mut context = Context::new();
        let mut rng = StdRng::from_seed(&[1, 2, 3, 4][..]);
        bash_with_rng(&mut context, &mut rng, 3, 4);
        assert_eq!(context.map.items[1].tval, 104);
        assert_eq!(context.map.items[1].p1, 1);
        assert_eq!(context.map.items[1].cost, 17);
        assert_eq!(context.map.cave[3][4].fopen, 1);
        assert_eq!(context.redraws, [(3, 4)]);
        assert_eq!(context.paralysis, 0);
        assert_eq!(
            context.messages,
            ["You smash into the door! The door crashes open!"]
        );
    }

    #[test]
    fn seeded_door_failure_keeps_door_and_sets_two_turn_paralysis() {
        let mut context = Context::new();
        context.weight = 1;
        context.strength = 0;
        context.map.items[1].p1 = -1000;
        context.bash();
        assert_eq!(context.map.items[1].tval, 105);
        assert_eq!(context.map.items[1].p1, -1000);
        assert_eq!(context.map.cave[3][4].fopen, 0);
        assert_eq!(context.paralysis, 2);
        assert_eq!(
            context.messages,
            ["You smash into the door! The door holds firm."]
        );
        assert!(context.redraws.is_empty());
    }

    fn chest_rng(destroy: bool, unlock: bool) -> StdRng {
        for seed in 1..1000 {
            let mut rng = StdRng::from_seed(&[seed, 2, 3, 4][..]);
            let first = crate::rng::randint_with_rng(&mut rng, 10);
            let second = crate::rng::randint_with_rng(&mut rng, 10);
            if (first == 1) == destroy && (second == 1) == unlock {
                return StdRng::from_seed(&[seed, 2, 3, 4][..]);
            }
        }
        panic!("Expected a seeded chest roll for each branch");
    }

    #[test]
    fn seeded_chest_destruction_clears_flags_and_renames_only() {
        let mut context = Context::new();
        context.map.items[1].tval = 2;
        context.map.items[1].flags = u64::MAX;
        context.map.items[1].flags2 = 42;
        context.map.items[1].cost = 99;
        bash_with_rng(&mut context, &mut chest_rng(true, false), 3, 4);
        let item = context.map.items[1];
        assert_eq!(item.name_bytes(), b"& ruined chest");
        assert_eq!(item.flags, 0);
        assert_eq!(item.flags2, 42);
        assert_eq!(item.cost, 99);
        assert_eq!(item.tval, 2);
        assert_eq!(
            context.messages,
            ["You have destroyed the chest...", "and its contents!"]
        );
        assert!(context.redraws.is_empty());
        assert_eq!(context.paralysis, 0);
    }

    #[test]
    fn seeded_chest_unlock_preserves_traps_and_legacy_32_bit_mask() {
        let mut context = Context::new();
        context.map.items[1].tval = 2;
        context.map.items[1].flags = u64::MAX;
        bash_with_rng(&mut context, &mut chest_rng(false, true), 3, 4);
        assert_eq!(context.map.items[1].flags, 0xFFFFFFFE);
        assert_eq!(context.map.items[1].name_bytes(), b"a closed door");
        assert_eq!(context.messages, ["The lock breaks open!"]);
        assert!(context.redraws.is_empty());
    }

    #[test]
    fn seeded_chest_failure_is_silent_and_unchanged() {
        let mut context = Context::new();
        context.map.items[1].tval = 2;
        context.map.items[1].flags = 7;
        bash_with_rng(&mut context, &mut chest_rng(false, false), 3, 4);
        assert_eq!(context.map.items[1].flags, 7);
        assert!(context.messages.is_empty());
        assert!(context.redraws.is_empty());
        assert_eq!(context.paralysis, 0);
    }

    #[test]
    fn unlocked_chest_skips_lock_roll_and_is_silent_when_not_destroyed() {
        let mut context = Context::new();
        context.map.items[1].tval = 2;
        context.map.items[1].flags = 6;
        let mut rng = chest_rng(false, true);
        let mut expected = rng;
        crate::rng::randint_with_rng(&mut expected, 10);
        bash_with_rng(&mut context, &mut rng, 3, 4);
        assert_eq!(context.map.items[1].flags, 6);
        assert!(context.messages.is_empty());
        assert_eq!(
            crate::rng::randint_with_rng(&mut rng, 1000),
            crate::rng::randint_with_rng(&mut expected, 1000)
        );
    }

    #[test]
    fn monster_takes_priority_over_door_and_restores_weapon_after_attack() {
        let mut context = Context::new();
        context.map.cave[3][4].cptr = 2;
        context.bash();
        assert_eq!(context.attacks, [(3, 4)]);
        assert_eq!(context.stuns, [2]);
        assert_eq!(context.primary.tval, 10);
        assert_eq!(context.primary.cost, 42);
        assert_eq!(context.primary.flags, 123);
        assert_eq!(context.equipment_writes, 2);
        assert_eq!(context.map.items[1].tval, 105);
        assert_eq!(context.map.cave[3][4].fopen, 0);
        assert_eq!(context.paralysis, 0);
        assert!(context.messages.is_empty());
    }

    #[test]
    fn afraid_monster_target_does_not_attack_change_equipment_or_roll() {
        let mut context = Context::new();
        context.map.cave[3][4].cptr = 2;
        context.map.cave[3][4].tptr = 0;
        context.afraid = true;
        let mut rng = StdRng::from_seed(&[1, 2, 3, 4][..]);
        let mut expected = rng;
        bash_with_rng(&mut context, &mut rng, 3, 4);
        assert_eq!(context.messages, ["You are afraid!"]);
        assert!(context.attacks.is_empty());
        assert!(context.stuns.is_empty());
        assert_eq!(context.equipment_writes, 0);
        assert_eq!(
            crate::rng::randint_with_rng(&mut rng, 1000),
            crate::rng::randint_with_rng(&mut expected, 1000)
        );
    }

    #[test]
    fn unsuccessful_monster_attack_restores_weapon_and_can_leave_player_off_balance() {
        let mut context = Context::new();
        context.map.cave[3][4].cptr = 2;
        context.map.cave[3][4].tptr = 0;
        context.attack_result = false;
        context.dexterity = 0;
        let mut expected = StdRng::from_seed(&[1, 2, 3, 4][..]);
        crate::rng::randint_with_rng(&mut expected, 300);
        let turns = crate::rng::randint_with_rng(&mut expected, 3);
        context.bash();
        assert_eq!(context.attacks, [(3, 4)]);
        assert!(context.stuns.is_empty());
        assert_eq!(context.primary.tval, 10);
        assert_eq!(context.equipment_writes, 2);
        assert_eq!(context.paralysis, turns);
        assert_eq!(context.messages, ["You are off-balance."]);
    }

    #[test]
    fn empty_non_bashable_and_invalid_targets_only_report_missing_target() {
        for target in [0, 1, 255] {
            let mut context = Context::new();
            context.map.cave[3][4].tptr = target;
            context.map.items[1].tval = 104;
            context.bash();
            assert_eq!(
                context.messages,
                ["I do not see anything you can bash there."]
            );
            assert!(context.attacks.is_empty());
            assert!(context.redraws.is_empty());
            assert_eq!(context.paralysis, 0);
        }
        let mut context = Context::new();
        bash_with_rng(
            &mut context,
            &mut StdRng::from_seed(&[1, 2, 3, 4][..]),
            -1,
            4,
        );
        assert_eq!(
            context.messages,
            ["I do not see anything you can bash there."]
        );
    }
}
