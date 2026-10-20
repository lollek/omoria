use crate::model::{Item, ItemType, PlayerFlags, Stat};

#[cfg(not(test))]
use crate::model::InventoryItem;

#[cfg(not(test))]
mod globals;

#[derive(Clone, Copy, Debug)]
enum Status {
    Poisoned,
    Blind,
    Afraid,
    Confused,
    Hoarse,
}

impl Status {
    fn counter(self, flags: &mut PlayerFlags) -> &mut i64 {
        match self {
            Self::Poisoned => &mut flags.poisoned,
            Self::Blind => &mut flags.blind,
            Self::Afraid => &mut flags.afraid,
            Self::Confused => &mut flags.confused,
            Self::Hoarse => &mut flags.hoarse,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum StatEffect {
    Gain,
    Lose,
    Restore,
}

#[derive(Clone, Copy)]
enum FoodEffect {
    Poison,
    Blind,
    Afraid,
    Confused,
    Hallucination,
    CurePoison,
    CureBlind,
    CureAfraid,
    CureConfused,
    LoseStat(Stat),
    RestoreStat(Stat),
    Damage(i64, i64),
    Heal(i64),
    CureHoarse,
    Fill,
    Flavor,
}

impl FoodEffect {
    fn from_legacy_code(code: u32) -> Option<Self> {
        use FoodEffect::*;
        match code {
            1 => Some(Poison),
            2 => Some(Blind),
            3 => Some(Afraid),
            4 => Some(Confused),
            5 => Some(Hallucination),
            6 => Some(CurePoison),
            7 => Some(CureBlind),
            8 => Some(CureAfraid),
            9 => Some(CureConfused),
            10 => Some(LoseStat(Stat::Strength)),
            11 => Some(LoseStat(Stat::Constitution)),
            12 => Some(LoseStat(Stat::Intelligence)),
            13 => Some(LoseStat(Stat::Wisdom)),
            14 => Some(LoseStat(Stat::Dexterity)),
            15 => Some(LoseStat(Stat::Charisma)),
            16 => Some(RestoreStat(Stat::Strength)),
            17 => Some(RestoreStat(Stat::Constitution)),
            18 => Some(RestoreStat(Stat::Intelligence)),
            19 => Some(RestoreStat(Stat::Wisdom)),
            20 => Some(RestoreStat(Stat::Dexterity)),
            21 => Some(RestoreStat(Stat::Charisma)),
            22 => Some(Damage(1, 3)),
            23 => Some(Damage(1, 6)),
            24 => Some(Damage(1, 12)),
            25 => Some(Damage(2, 12)),
            26 => Some(Damage(4, 12)),
            27 => Some(Heal(4)),
            28 => Some(Heal(8)),
            29 => Some(CureHoarse),
            30 => Some(Fill),
            31 => Some(Flavor),
            _ => None,
        }
    }
}

trait EatContext {
    fn item(&mut self) -> Item;
    fn modify_flags(&mut self, modify: impl FnOnce(&mut PlayerFlags));
    fn set_flags2(&mut self, flags2: u64);
    fn cure(&mut self, status: Status) -> bool;
    fn stat_effect(
        &mut self,
        effect: StatEffect,
        stat: Stat,
        message: &'static str,
        second_message: &'static str,
    ) -> bool;
    fn hp_player(&mut self, amount: i64, cause: &'static [u8]) -> bool;
    fn take_hit(&mut self, amount: i64, cause: &'static [u8]);
    fn bless(&mut self, amount: i64);
    fn aggravate_monsters(&mut self, distance: i64);
    fn set_full(&mut self);
    fn player_level(&self) -> u16;
    fn add_experience(&mut self, amount: i64);
    fn eat(&mut self, amount: i64);
    fn message(&mut self, message: &'static str);
    fn draw_cave(&mut self);
    fn redraw_stats(&mut self);
    fn identify(&mut self);
    fn remaining(&mut self);
    fn destroy(&mut self);
}

fn apply_eyeball_effects_with_roll(
    context: &mut impl EatContext,
    roll: &mut impl FnMut(i64) -> i64,
) -> bool {
    context.take_hit(
        100 + (0..10).map(|_| roll(8)).sum::<i64>(),
        b"the Wrath of Ned\0",
    );
    context.cure(Status::Afraid);
    context.bless(roll(100) + 100);
    context.modify_flags(|flags| {
        flags.hero += roll(100) + 100;
        flags.shero += roll(50) + 75;
        flags.invuln += roll(15) + 10;
        flags.image += roll(500) + roll(500) + roll(1000) + 5000;
        flags.poisoned += roll(100) + 150;
        flags.confused += roll(30) + 50;
        flags.blind += roll(3) + 10;
        flags.paralysis = 4;
    });

    context.message("You hear a distant rumble of laughter...");
    context.message("You throw back your head and laugh back!");
    context.aggravate_monsters(30);

    if roll(5) == 1 {
        for (stat, message) in [
            (Stat::Strength, "Ned smiles upon you."),
            (Stat::Intelligence, ""),
            (Stat::Wisdom, ""),
            (Stat::Dexterity, ""),
            (Stat::Constitution, ""),
            (Stat::Charisma, ""),
        ] {
            context.stat_effect(StatEffect::Gain, stat, message, "");
        }
    }

    context.message("Your head begins to spin. Spots swirl before you.");
    context.message("A veil of darkness clings to your eyes.");
    context.message("You are unable to move!");
    true
}

fn apply_food_effects_with_roll(
    item: Item,
    context: &mut impl EatContext,
    roll: &mut impl FnMut(i64) -> i64,
) -> bool {
    if item.tval == u8::from(ItemType::JunkFood) && item.subval == 270 {
        return apply_eyeball_effects_with_roll(context, roll);
    }

    let mut flags = item.flags;
    let mut damage = 0;
    let mut identified = false;
    while flags > 0 {
        let code = flags.trailing_zeros() + 1;
        flags &= flags - 1;
        match FoodEffect::from_legacy_code(code) {
            Some(FoodEffect::Poison) => {
                context.modify_flags(|player| player.poisoned += roll(10) + i64::from(item.level));
                identified = true;
            }
            Some(FoodEffect::Blind) => {
                context.modify_flags(|player| {
                    player.blind += roll(250) + 10 * i64::from(item.level) + 100
                });
                context.draw_cave();
                context.message("A veil of darkness surrounds you.");
                identified = true;
            }
            Some(FoodEffect::Afraid) => {
                context.modify_flags(|player| player.afraid += roll(10) + i64::from(item.level));
                context.message("You feel terrified!");
                identified = true;
            }
            Some(FoodEffect::Confused) => {
                context.modify_flags(|player| player.confused += roll(10) + i64::from(item.level));
                context.message("You feel drugged.");
            }
            Some(FoodEffect::Hallucination) => {
                context.modify_flags(|player| {
                    player.image += roll(200) + 25 * i64::from(item.level) + 200
                });
            }
            Some(FoodEffect::CurePoison) => identified = context.cure(Status::Poisoned),
            Some(FoodEffect::CureBlind) => identified = context.cure(Status::Blind),
            Some(FoodEffect::CureAfraid) => {
                context.modify_flags(|player| {
                    if player.afraid > 1 {
                        player.afraid = 1;
                        identified = true;
                    }
                });
            }
            Some(FoodEffect::CureConfused) => identified = context.cure(Status::Confused),
            Some(FoodEffect::LoseStat(stat)) => {
                identified = context.stat_effect(StatEffect::Lose, stat, "X", "X");
            }
            Some(FoodEffect::RestoreStat(stat)) => {
                let message = match stat {
                    Stat::Strength => "You feel your strength returning.",
                    Stat::Intelligence => "Your head spins for a moment.",
                    Stat::Dexterity => "You feel more agile.",
                    Stat::Wisdom | Stat::Constitution | Stat::Charisma => "X",
                };
                identified = context.stat_effect(StatEffect::Restore, stat, message, "");
            }
            Some(FoodEffect::Damage(dice, sides)) => {
                damage += (0..dice).map(|_| roll(sides)).sum::<i64>();
            }
            Some(FoodEffect::Heal(sides)) => {
                damage -= roll(sides);
            }
            Some(FoodEffect::CureHoarse) => identified = context.cure(Status::Hoarse),
            Some(FoodEffect::Fill) => {
                context.set_full();
                context.message("Yum!");
            }
            Some(FoodEffect::Flavor) => {
                let mut flags2 = item.flags2;
                let flavor = if flags2 == 0 {
                    0
                } else {
                    let index = flags2.trailing_zeros();
                    flags2 &= flags2 - 1;
                    context.set_flags2(flags2);
                    index + 1
                };
                match flavor {
                    1 => match roll(4) {
                        1 => context.message("*munch* *munch*"),
                        2 => context.message("*crunch* *crunch*"),
                        3 => context.message("*munch* *crunch*"),
                        4 => context.message("*crunch* *munch*"),
                        _ => {}
                    },
                    2 => {
                        match roll(3) {
                            1 => context.message("*guzzle* *guzzle*"),
                            2 => context.message("*glug* *glug* *glug*"),
                            3 => context.message("*slurp*"),
                            _ => {}
                        }
                        if roll(3) == 1 {
                            context.message("*Burp*");
                            context.message("('scuse me....)");
                        }
                    }
                    _ => {}
                }
            }
            None => {}
        }

        if damage != 0 {
            identified = context.hp_player(damage, b"poisonous food.\0");
        }
    }
    identified
}

fn eat_selected_with_roll(context: &mut impl EatContext, roll: &mut impl FnMut(i64) -> i64) {
    let item = context.item();
    if apply_food_effects_with_roll(item, context, roll) {
        context.identify();
    }
    if item.flags != 0 && item.level > 0 {
        let experience = (f32::from(item.level) / f32::from(context.player_level()) + 0.5) as i64;
        context.add_experience(experience);
        context.redraw_stats();
    }
    context.eat(item.p1);
    context.remaining();
    context.destroy();
    context.redraw_stats();
}

#[cfg(not(test))]
#[no_mangle]
/// # Safety
/// `item` must be the live C-selected inventory node, with exclusive access
/// for this action. Consuming its last item frees the node.
pub unsafe extern "C" fn C_player_action_eat_selected(item: *mut InventoryItem) {
    unsafe { globals::eat_global(item) }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Context {
        item: Item,
        flags: PlayerFlags,
        events: Vec<String>,
        destroyed: bool,
        level: u16,
        hp_identified: bool,
    }

    impl Context {
        fn new(item: Item) -> Self {
            Self {
                item,
                flags: PlayerFlags::new(),
                events: Vec::new(),
                destroyed: false,
                level: 2,
                hp_identified: true,
            }
        }

        fn record(&mut self, event: impl Into<String>) {
            self.events.push(event.into());
        }
    }

    impl EatContext for Context {
        fn item(&mut self) -> Item {
            assert!(!self.destroyed, "must not read a freed inventory node");
            self.item
        }

        fn modify_flags(&mut self, modify: impl FnOnce(&mut PlayerFlags)) {
            modify(&mut self.flags);
        }

        fn set_flags2(&mut self, flags2: u64) {
            self.item.flags2 = flags2;
        }

        fn cure(&mut self, status: Status) -> bool {
            let flag = status.counter(&mut self.flags);
            let identified = *flag > 1;
            if identified {
                *flag = 1;
            }
            self.record(format!("cure {status:?}"));
            identified
        }

        fn stat_effect(
            &mut self,
            effect: StatEffect,
            stat: Stat,
            message: &'static str,
            second_message: &'static str,
        ) -> bool {
            self.record(format!(
                "stat {effect:?} {stat:?} {message} {second_message}"
            ));
            true
        }

        fn hp_player(&mut self, amount: i64, cause: &'static [u8]) -> bool {
            self.record(format!(
                "hp {amount} {}",
                String::from_utf8_lossy(&cause[..cause.len() - 1])
            ));
            self.hp_identified
        }

        fn take_hit(&mut self, amount: i64, cause: &'static [u8]) {
            self.record(format!(
                "damage {amount} {}",
                String::from_utf8_lossy(&cause[..cause.len() - 1])
            ));
        }

        fn bless(&mut self, amount: i64) {
            self.record(format!("bless {amount}"));
        }

        fn aggravate_monsters(&mut self, distance: i64) {
            self.record(format!("aggravate {distance}"));
        }

        fn set_full(&mut self) {
            self.flags.foodc = 15_000;
            self.record("full");
        }

        fn player_level(&self) -> u16 {
            self.level
        }

        fn add_experience(&mut self, amount: i64) {
            self.record(format!("experience {amount}"));
        }

        fn eat(&mut self, amount: i64) {
            self.record(format!("eat {amount}"));
        }

        fn message(&mut self, message: &'static str) {
            self.record(message);
        }

        fn draw_cave(&mut self) {
            self.record("draw cave");
        }

        fn redraw_stats(&mut self) {
            self.record("redraw stats");
        }

        fn identify(&mut self) {
            assert!(!self.destroyed);
            self.item.identified = 1;
            self.record("identify");
        }

        fn remaining(&mut self) {
            assert!(!self.destroyed);
            self.record(format!("remaining {}", self.item.number - 1));
        }

        fn destroy(&mut self) {
            assert!(!self.destroyed);
            self.destroyed = true;
            self.record("destroy");
        }
    }

    #[test]
    fn poison_food_adds_a_seeded_poison_duration() {
        let item = Item {
            tval: ItemType::Food.into(),
            flags: 1,
            level: 3,
            ..Item::default()
        };
        let mut context = Context::new(item);

        let identified = apply_food_effects_with_roll(item, &mut context, &mut |_| 4);

        assert_eq!(context.flags.poisoned, 7);
        assert!(identified, "poisonous food identifies itself");
    }

    #[test]
    fn confused_food_prints_its_legacy_message() {
        let item = Item {
            tval: ItemType::Food.into(),
            flags: 1 << 3,
            level: 3,
            ..Item::default()
        };
        let mut context = Context::new(item);

        let identified = apply_food_effects_with_roll(item, &mut context, &mut |_| 4);

        assert!(!identified);
        assert_eq!(context.flags.confused, 7);
        assert_eq!(context.events, ["You feel drugged."]);
    }

    #[test]
    fn plain_food_eats_nutrition_and_consumes_one_item_without_identifying() {
        let item = Item {
            tval: ItemType::Food.into(),
            number: 2,
            p1: 5000,
            ..Item::default()
        };
        let mut context = Context::new(item);

        eat_selected_with_roll(&mut context, &mut |_| 1);

        assert_eq!(
            context.events,
            ["eat 5000", "remaining 1", "destroy", "redraw stats"]
        );
    }

    #[test]
    fn status_food_identifies_and_consumes_after_applying_poison() {
        let item = Item {
            tval: ItemType::Food.into(),
            flags: 1,
            level: 3,
            p1: 75,
            number: 1,
            ..Item::default()
        };
        let mut context = Context::new(item);

        eat_selected_with_roll(&mut context, &mut |_| 4);

        assert_eq!(context.flags.poisoned, 7);
        assert_eq!(context.item.identified, 1);
        assert_eq!(
            context.events,
            [
                "identify",
                "experience 2",
                "redraw stats",
                "eat 75",
                "remaining 0",
                "destroy",
                "redraw stats"
            ]
        );
    }

    #[test]
    fn food_healing_overwrites_identification_with_the_hp_result() {
        let item = Item {
            tval: ItemType::Food.into(),
            flags: 1 | (1 << 26),
            level: 3,
            ..Item::default()
        };
        let mut context = Context::new(item);
        context.hp_identified = false;

        let identified = apply_food_effects_with_roll(item, &mut context, &mut |_| 1);

        assert!(
            !identified,
            "the later HP result replaces earlier identification"
        );
        assert_eq!(context.events, ["hp -1 poisonous food."]);
    }

    #[test]
    fn eyeball_of_drong_applies_its_effects_and_identifies() {
        let item = Item {
            tval: ItemType::JunkFood.into(),
            subval: 270,
            number: 1,
            ..Item::default()
        };
        let mut context = Context::new(item);
        context.flags.afraid = 10;

        eat_selected_with_roll(&mut context, &mut |_| 1);

        assert_eq!(context.flags.afraid, 1);
        assert_eq!(context.flags.hero, 101);
        assert_eq!(context.flags.shero, 76);
        assert_eq!(context.flags.invuln, 11);
        assert_eq!(context.flags.image, 5003);
        assert_eq!(context.flags.poisoned, 151);
        assert_eq!(context.flags.confused, 51);
        assert_eq!(context.flags.blind, 11);
        assert_eq!(context.flags.paralysis, 4);
        assert_eq!(context.events[0], "damage 110 the Wrath of Ned");
        assert_eq!(context.events[1], "cure Afraid");
        assert_eq!(context.events[2], "bless 101");
        assert_eq!(context.events[5], "aggravate 30");
        assert_eq!(
            &context.events[6..12],
            [
                "stat Gain Strength Ned smiles upon you. ",
                "stat Gain Intelligence  ",
                "stat Gain Wisdom  ",
                "stat Gain Dexterity  ",
                "stat Gain Constitution  ",
                "stat Gain Charisma  "
            ]
        );
        assert_eq!(
            &context.events[12..],
            [
                "Your head begins to spin. Spots swirl before you.",
                "A veil of darkness clings to your eyes.",
                "You are unable to move!",
                "identify",
                "eat 0",
                "remaining 0",
                "destroy",
                "redraw stats"
            ]
        );
    }

    #[test]
    fn flavor_food_clears_its_lowest_flavor_flag_and_reports_crunching() {
        let item = Item {
            tval: ItemType::Food.into(),
            flags: 1 << 30,
            flags2: 0b11,
            ..Item::default()
        };
        let mut context = Context::new(item);

        let identified = apply_food_effects_with_roll(item, &mut context, &mut |_| 2);

        assert!(!identified);
        assert_eq!(context.item.flags2, 0b10);
        assert_eq!(context.events, ["*crunch* *crunch*"]);
    }

    #[test]
    fn food_effect_codes_cover_legacy_codes_one_through_thirty_one() {
        assert!((1..=31).all(|code| FoodEffect::from_legacy_code(code).is_some()));
        assert!(FoodEffect::from_legacy_code(32).is_none());
    }
}
