use crate::model::ItemType;

#[cfg(not(test))]
mod globals;
#[cfg(not(test))]
mod interop;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
}

impl Direction {
    fn tvals(self) -> (u8, u8) {
        match self {
            Direction::Up => (
                u8::from(ItemType::UpStaircase),
                u8::from(ItemType::UpSteepStaircase),
            ),
            Direction::Down => (
                u8::from(ItemType::DownStaircase),
                u8::from(ItemType::DownSteepStaircase),
            ),
        }
    }

    fn sign(self) -> i64 {
        match self {
            Direction::Up => -1,
            Direction::Down => 1,
        }
    }

    fn word(self) -> &'static str {
        match self {
            Direction::Up => "up",
            Direction::Down => "down",
        }
    }
}

trait StairsContext {
    fn tile_tval(&mut self) -> Option<u8>;
    fn dun_level(&mut self) -> i64;
    fn set_dun_level(&mut self, level: i64);
    fn set_moria_flag(&mut self);
    fn message(&mut self, message: &str);
}

fn take_stairs(context: &mut impl StairsContext, direction: Direction, roll: impl FnOnce() -> i64) {
    let (normal, steep) = direction.tvals();
    let level = context.dun_level();
    let (long_maze, next_level) = match context.tile_tval() {
        Some(tval) if tval == normal => (false, level + direction.sign()),
        Some(tval) if tval == steep => {
            let next = level + direction.sign() * (roll() + 1);
            match direction {
                Direction::Up => (true, next.max(0)),
                Direction::Down => (true, next),
            }
        }
        _ => {
            context.message(&format!("I see no {} staircase here.", direction.word()));
            return;
        }
    };
    context.set_dun_level(next_level);
    context.set_moria_flag();
    let kind = if long_maze { "long maze" } else { "maze" };
    context.message(&format!(
        "You enter a {kind} of {} staircases.",
        direction.word()
    ));
    context.message("You pass through a one-way door.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::cell::test_support::TestMap;
    use rand::{Rng, SeedableRng, StdRng};

    struct Context {
        map: TestMap,
        level: i64,
        moria_flag: bool,
        messages: Vec<String>,
    }

    impl Context {
        fn on_tile(tile: Option<ItemType>, level: i64) -> Self {
            let mut map = TestMap::reset();
            if let Some(tile) = tile {
                map.put(3, 3, 1, u8::from(tile), b"a staircase");
            }
            Self {
                map,
                level,
                moria_flag: false,
                messages: Vec::new(),
            }
        }
    }

    impl StairsContext for Context {
        fn tile_tval(&mut self) -> Option<u8> {
            self.map
                .cells()
                .read(3, 3)
                .and_then(|(_, item)| item)
                .map(|item| item.tval)
        }
        fn dun_level(&mut self) -> i64 {
            self.level
        }
        fn set_dun_level(&mut self, level: i64) {
            self.level = level;
        }
        fn set_moria_flag(&mut self) {
            self.moria_flag = true;
        }
        fn message(&mut self, message: &str) {
            self.messages.push(message.to_owned());
        }
    }

    fn no_roll() -> i64 {
        panic!("normal stairs must not roll");
    }

    #[test]
    fn up_staircase_moves_one_level_without_rolling() {
        let mut context = Context::on_tile(Some(ItemType::UpStaircase), 5);
        take_stairs(&mut context, Direction::Up, no_roll);
        assert_eq!(context.level, 4);
        assert!(context.moria_flag);
        assert_eq!(
            context.messages,
            [
                "You enter a maze of up staircases.",
                "You pass through a one-way door."
            ]
        );
    }

    #[test]
    fn down_staircase_moves_one_level_without_rolling() {
        let mut context = Context::on_tile(Some(ItemType::DownStaircase), 5);
        take_stairs(&mut context, Direction::Down, no_roll);
        assert_eq!(context.level, 6);
        assert!(context.moria_flag);
        assert_eq!(
            context.messages,
            [
                "You enter a maze of down staircases.",
                "You pass through a one-way door."
            ]
        );
    }

    #[test]
    fn normal_up_staircase_does_not_clamp_at_level_zero() {
        let mut context = Context::on_tile(Some(ItemType::UpStaircase), 0);
        take_stairs(&mut context, Direction::Up, no_roll);
        assert_eq!(context.level, -1, "C only clamps steep up stairs");
    }

    #[test]
    fn steep_up_staircase_subtracts_roll_plus_one_and_clamps_at_zero() {
        let mut context = Context::on_tile(Some(ItemType::UpSteepStaircase), 2);
        take_stairs(&mut context, Direction::Up, || 3);
        assert_eq!(context.level, 0);
        assert!(context.moria_flag);
        assert_eq!(
            context.messages,
            [
                "You enter a long maze of up staircases.",
                "You pass through a one-way door."
            ]
        );
    }

    #[test]
    fn steep_up_staircase_above_zero_is_not_clamped() {
        let mut context = Context::on_tile(Some(ItemType::UpSteepStaircase), 10);
        take_stairs(&mut context, Direction::Up, || 2);
        assert_eq!(context.level, 7);
    }

    #[test]
    fn steep_down_staircase_adds_roll_plus_one() {
        let mut context = Context::on_tile(Some(ItemType::DownSteepStaircase), 5);
        take_stairs(&mut context, Direction::Down, || 2);
        assert_eq!(context.level, 8);
        assert!(context.moria_flag);
        assert_eq!(
            context.messages,
            [
                "You enter a long maze of down staircases.",
                "You pass through a one-way door."
            ]
        );
    }

    #[test]
    fn steep_stairs_roll_exactly_once() {
        let mut rng = StdRng::from_seed(&[1, 2, 3, 4][..]);
        let mut expected = StdRng::from_seed(&[1, 2, 3, 4][..]);
        let step = crate::rng::randint_with_rng(&mut expected, 3) + 1;
        let mut context = Context::on_tile(Some(ItemType::DownSteepStaircase), 5);
        take_stairs(&mut context, Direction::Down, || {
            crate::rng::randint_with_rng(&mut rng, 3)
        });
        assert_eq!(context.level, 5 + step);
        assert_eq!(
            rng.gen::<u64>(),
            expected.gen::<u64>(),
            "steep stairs should consume exactly one d3 roll"
        );
    }

    #[test]
    fn missing_or_non_stair_tile_leaves_state_unchanged() {
        for direction in [Direction::Up, Direction::Down] {
            let word = direction.word();
            for tile in [None, Some(ItemType::Chest)] {
                let mut context = Context::on_tile(tile, 5);
                take_stairs(&mut context, direction, no_roll);
                assert_eq!(context.level, 5);
                assert!(!context.moria_flag);
                assert_eq!(
                    context.messages,
                    [format!("I see no {word} staircase here.")]
                );
            }
        }
    }

    #[test]
    fn opposite_direction_stairs_are_not_taken() {
        let mut context = Context::on_tile(Some(ItemType::DownSteepStaircase), 5);
        take_stairs(&mut context, Direction::Up, no_roll);
        assert_eq!(context.level, 5);
        assert!(!context.moria_flag);
        assert_eq!(context.messages, ["I see no up staircase here."]);
    }
}
