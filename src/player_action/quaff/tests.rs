use super::*;
use crate::model::ItemType;
use crate::rng::randint_with_rng;
use rand::{SeedableRng, StdRng};

struct Context {
    item: Item,
    flags: PlayerFlags,
    exp: i64,
    level: u16,
    mana: f32,
    max_mana: i16,
    spell_result: bool,
    spells: Vec<Spell>,
    events: Vec<String>,
    destroyed: bool,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            item: Item {
                tval: ItemType::Potion1.into(),
                number: 2,
                level: 5,
                p1: 100,
                ..Item::default()
            },
            flags: PlayerFlags::default(),
            exp: 100,
            level: 2,
            mana: 3.5,
            max_mana: 10,
            spell_result: true,
            spells: Vec::new(),
            events: Vec::new(),
            destroyed: false,
        }
    }
}

impl QuaffContext for Context {
    fn item(&mut self) -> Item {
        assert!(!self.destroyed, "must not read a freed inventory node");
        self.item
    }
    fn flags(&self) -> PlayerFlags {
        self.flags
    }
    fn modify_flags(&mut self, modify: impl FnOnce(&mut PlayerFlags)) {
        modify(&mut self.flags);
    }
    fn experience(&mut self) -> i64 {
        self.exp
    }
    fn level(&mut self) -> u16 {
        self.level
    }
    fn restore_mana(&mut self) {
        self.mana = self.mana.max(f32::from(self.max_mana));
    }
    fn spell(&mut self, spell: Spell) -> bool {
        self.spells.push(spell);
        self.events.push(format!("{spell:?}"));
        if let Spell::Cure(status) = spell {
            *status.counter(&mut self.flags) = 1;
        }
        self.spell_result
    }
    fn add_experience(&mut self, amount: i64) {
        self.events.push(format!("experience {amount}"));
        self.exp += amount;
    }
    fn eat(&mut self, amount: i64) {
        self.events.push(format!("eat {amount}"));
    }
    fn message(&mut self, message: &str) {
        self.events.push(message.to_owned());
    }
    fn redraw(&mut self) {
        self.events.push("redraw".to_owned());
    }
    fn redraw_poisoned(&mut self) {
        self.events.push("poisoned".to_owned());
    }
    fn identify(&mut self) {
        assert!(!self.destroyed);
        self.item.identified = 1;
        self.events.push("identify".to_owned());
    }
    fn remaining(&mut self) {
        assert!(!self.destroyed);
        self.events
            .push(format!("remaining {}", self.item.number - 1));
    }
    fn destroy(&mut self) {
        assert!(!self.destroyed);
        self.item.number -= 1;
        self.destroyed = true;
        self.events.push("destroy".to_owned());
    }
}

fn effect(effect: u32, context: &mut Context) -> bool {
    let mut rng = StdRng::from_seed(&[1, 2, 3, 4][..]);
    effect_with_roll(
        PotionEffect::from_legacy_code(effect),
        context,
        &mut |max| randint_with_rng(&mut rng, max),
    )
}

#[test]
fn legacy_effect_codes_decode_to_named_effects() {
    use PotionEffect::*;
    use Stat::*;
    let expected = [
        GainStat(Strength),
        LoseStat(Strength),
        RestoreStat(Strength),
        GainStat(Intelligence),
        LoseStat(Intelligence),
        RestoreStat(Intelligence),
        GainStat(Wisdom),
        LoseStat(Wisdom),
        RestoreStat(Wisdom),
        GainStat(Charisma),
        LoseStat(Charisma),
        RestoreStat(Charisma),
        HealLight,
        HealSerious,
        HealCritical,
        HealFully,
        GainStat(Constitution),
        GainExperience,
        Sleep,
        Blindness,
        Confusion,
        Poison,
        Haste,
        Slowness,
        DetectCreatures,
        GainStat(Dexterity),
        RestoreStat(Dexterity),
        RestoreStat(Constitution),
        CureUnidentified(Status::Blind),
        CureUnidentified(Status::Confused),
        CureUnidentified(Status::Poisoned),
        LearnMagic,
        LoseMemories,
        SaltWater,
        Invulnerability,
        Heroism,
        SuperHeroism,
        Cure(Status::Afraid),
        RestoreLevel,
        ResistHeat,
        ResistCold,
        DetectInvisible,
        SlowPoison,
        Cure(Status::Poisoned),
        RestoreMana,
        InfraVision,
        CureHallucination,
        LearnMagic,
        ReducePetrification,
    ];
    for (code, expected) in (1..=49).zip(expected) {
        assert_eq!(
            PotionEffect::from_legacy_code(code),
            expected,
            "code {code}"
        );
    }
}

#[test]
fn unassigned_legacy_effect_codes_decode_to_explicit_noop() {
    for code in std::iter::once(0)
        .chain(50..=127)
        .chain(std::iter::once(u32::MAX))
    {
        assert_eq!(PotionEffect::from_legacy_code(code), PotionEffect::Unused);
    }
}

#[test]
fn stat_potions_dispatch_the_legacy_stat_and_propagate_identification() {
    use Stat::*;
    let cases = [
        (1, Spell::Gain(Strength)),
        (2, Spell::Lose(Strength)),
        (3, Spell::Restore(Strength)),
        (4, Spell::Gain(Intelligence)),
        (5, Spell::Lose(Intelligence)),
        (6, Spell::Restore(Intelligence)),
        (7, Spell::Gain(Wisdom)),
        (8, Spell::Lose(Wisdom)),
        (9, Spell::Restore(Wisdom)),
        (10, Spell::Gain(Charisma)),
        (11, Spell::Lose(Charisma)),
        (12, Spell::Restore(Charisma)),
        (17, Spell::Gain(Constitution)),
        (26, Spell::Gain(Dexterity)),
        (27, Spell::Restore(Dexterity)),
        (28, Spell::Restore(Constitution)),
    ];
    for (number, spell) in cases {
        for result in [false, true] {
            let mut context = Context {
                spell_result: result,
                ..Context::default()
            };
            assert_eq!(effect(number, &mut context), result, "effect {number}");
            assert_eq!(context.spells, [spell], "effect {number}");
            assert_eq!(
                context.events,
                match number {
                    5 => vec![
                        "This potion tastes very dull.".to_owned(),
                        format!("{spell:?}")
                    ],
                    17 => vec![format!("{spell:?}"), "redraw".to_owned()],
                    _ => vec![format!("{spell:?}")],
                }
            );
        }
    }
}

#[test]
fn healing_potions_use_seeded_dice_in_c_order() {
    for (number, dice) in [(13, 2), (14, 4), (15, 6), (16, 0)] {
        let mut reference = StdRng::from_seed(&[1, 2, 3, 4][..]);
        let expected = if dice == 0 {
            1000
        } else {
            (0..dice).map(|_| randint_with_rng(&mut reference, 7)).sum()
        };
        let mut context = Context::default();
        assert!(effect(number, &mut context));
        assert_eq!(context.spells, [Spell::Heal(expected)]);
    }
}

#[test]
fn timed_effects_add_the_legacy_seeded_duration_and_messages() {
    fn counter(number: u32, flags: &mut PlayerFlags) -> &mut i64 {
        match number {
            19 => &mut flags.paralysis,
            20 => &mut flags.blind,
            21 => &mut flags.confused,
            22 => &mut flags.poisoned,
            23 => &mut flags.fast,
            24 => &mut flags.slow,
            35 => &mut flags.invuln,
            36 => &mut flags.hero,
            37 => &mut flags.shero,
            40 => &mut flags.resist_heat,
            41 => &mut flags.resist_cold,
            46 => &mut flags.tim_infra,
            _ => unreachable!(),
        }
    }
    for (number, max, base, message, identified) in [
        (19, 4, 4, Some("You fall asleep."), true),
        (
            20,
            100,
            100,
            Some("You are covered by a veil of darkness."),
            true,
        ),
        (
            21,
            20,
            12,
            Some("Hey!  This is good stuff!  * Hick! *"),
            true,
        ),
        (22, 15, 10, Some("You feel very sick."), true),
        (23, 25, 15, None, true),
        (24, 25, 15, None, true),
        (35, 10, 10, None, true),
        (36, 25, 25, None, true),
        (37, 25, 25, None, true),
        (40, 10, 10, None, false),
        (41, 10, 10, None, false),
        (46, 100, 100, Some("Your eyes begin to tingle."), true),
    ] {
        let mut context = Context::default();
        *counter(number, &mut context.flags) = 7;
        let mut reference = StdRng::from_seed(&[1, 2, 3, 4][..]);
        let expected = 7 + base + randint_with_rng(&mut reference, max);
        assert_eq!(effect(number, &mut context), identified, "effect {number}");
        assert_eq!(
            *counter(number, &mut context.flags),
            expected,
            "effect {number}"
        );
        assert_eq!(context.events, message.into_iter().collect::<Vec<_>>());
    }
}

#[test]
fn free_action_prevents_sleep_without_drawing_rng() {
    let mut context = Context::default();
    context.flags.free_act = 1;
    assert!(!effect_with_roll(
        PotionEffect::Sleep,
        &mut context,
        &mut |_| panic!("unexpected roll")
    ));
    assert_eq!(context.flags.paralysis, 0);
    assert!(context.events.is_empty());
}

#[test]
fn experience_gain_is_capped_and_redraws_after_message() {
    for (exp, gain) in [(0, 10), (100, 60), (1_000_000, 100_000)] {
        let mut context = Context {
            exp,
            ..Context::default()
        };
        assert!(effect(18, &mut context));
        assert_eq!(context.exp, exp + gain);
        assert_eq!(
            context.events,
            [
                format!("experience {gain}"),
                "You feel more experienced.".to_owned(),
                "redraw".to_owned()
            ]
        );
    }
}

#[test]
fn spell_effects_preserve_the_legacy_identification_quirks() {
    for (number, spell, uses_result) in [
        (25, Spell::DetectCreatures, true),
        (29, Spell::Cure(Status::Blind), false),
        (30, Spell::Cure(Status::Confused), false),
        (31, Spell::Cure(Status::Poisoned), false),
        (32, Spell::Learn, false),
        (38, Spell::Cure(Status::Afraid), true),
        (39, Spell::RestoreLevels, true),
        (43, Spell::SlowPoison, true),
        (44, Spell::Cure(Status::Poisoned), true),
        (48, Spell::Learn, false),
    ] {
        for result in [false, true] {
            let mut context = Context {
                spell_result: result,
                ..Context::default()
            };
            assert_eq!(
                effect(number, &mut context),
                uses_result && result,
                "effect {number}"
            );
            assert_eq!(context.spells, [spell]);
            assert_eq!(
                context.events.last().unwrap(),
                if number == 39 {
                    "eat 5000".to_owned()
                } else {
                    format!("{spell:?}")
                }
                .as_str()
            );
        }
    }
}

#[test]
fn memory_loss_and_invisibility_use_seeded_legacy_rolls() {
    for exp in [0, 1, 99, 1000] {
        let mut context = Context {
            exp,
            ..Context::default()
        };
        let mut reference = StdRng::from_seed(&[1, 2, 3, 4][..]);
        let fifth = (exp as f64 / 5.0).trunc() as i64;
        assert!(effect(33, &mut context));
        assert_eq!(
            context.spells,
            [Spell::LoseExperience(
                fifth + randint_with_rng(&mut reference, fifth)
            )]
        );
        assert_eq!(&context.events[..2], ["You feel your memories fade...", ""]);
    }
    let mut context = Context::default();
    let mut reference = StdRng::from_seed(&[1, 2, 3, 4][..]);
    assert!(!effect(42, &mut context));
    assert_eq!(
        context.spells,
        [Spell::DetectInvisible(
            12 + randint_with_rng(&mut reference, 12)
        )]
    );
}

#[test]
fn salt_water_clears_poison_caps_food_and_overwrites_paralysis() {
    for (food, expected) in [(100, 100), (200, 150)] {
        let mut context = Context::default();
        context.flags.poisoned = 20;
        context.flags.status = 0x20 | 0x80;
        context.flags.foodc = food;
        context.flags.paralysis = 50;
        assert!(effect(34, &mut context));
        assert_eq!(context.flags.poisoned, 0);
        assert_eq!(context.flags.status, 0x80);
        assert_eq!(context.flags.foodc, expected);
        assert_eq!(context.flags.paralysis, 4);
        assert_eq!(context.events, ["poisoned", "The potion makes you vomit!"]);
    }
}

#[test]
fn mana_potion_never_lowers_mana_and_always_identifies() {
    for (mana, expected) in [(3.5, 10.0), (10.0, 10.0), (12.5, 12.5)] {
        let mut context = Context {
            mana,
            ..Context::default()
        };
        assert!(effect(45, &mut context));
        assert_eq!(context.mana, expected);
        assert_eq!(context.events, ["Your feel your head clear..."]);
    }
}

#[test]
fn hallucination_cure_adds_confusion_before_calling_spell() {
    let mut context = Context::default();
    context.flags.confused = 7;
    let mut reference = StdRng::from_seed(&[1, 2, 3, 4][..]);
    assert!(effect(47, &mut context));
    assert_eq!(
        context.flags.confused,
        12 + randint_with_rng(&mut reference, 5)
    );
    assert_eq!(context.spells, [Spell::Cure(Status::Image)]);
    assert_eq!(context.events, ["Pretty colors!", "Cure(Image)"]);
}

#[test]
fn petrification_reduction_clamps_to_zero_and_identifies_only_when_positive() {
    for (before, after, identified) in [
        (-1, -1, false),
        (0, 0, false),
        (50, 0, true),
        (200, 100, true),
    ] {
        let mut context = Context::default();
        context.flags.petrification = before;
        assert_eq!(effect(49, &mut context), identified);
        assert_eq!(context.flags.petrification, after);
    }
}

#[test]
fn unused_effects_are_unidentified_noops() {
    for number in [0, 50, 62, 64, 95, 127] {
        let mut context = Context::default();
        assert!(!effect_with_roll(
            PotionEffect::from_legacy_code(number),
            &mut context,
            &mut |_| panic!("unexpected roll")
        ));
        assert!(context.events.is_empty());
    }
}

#[test]
fn lifecycle_identifies_reports_then_consumes_including_the_last_potion() {
    for number in [1, 2] {
        let mut context = Context::default();
        context.item.flags = 1;
        context.item.number = number;
        quaff_with_roll(&mut context, &mut |_| panic!("unexpected roll"));
        assert_eq!(context.item.number, number - 1);
        assert_eq!(context.item.identified, 1);
        assert_eq!(context.exp, 103);
        assert_eq!(
            context.events,
            [
                "Gain(Strength)",
                "identify",
                "experience 3",
                "redraw",
                "eat 100",
                &format!("remaining {}", number - 1),
                "destroy",
                "redraw"
            ]
        );
    }
}

#[test]
fn final_effect_not_any_effect_controls_identification() {
    let mut context = Context::default();
    context.item.flags = 1 | (1 << 39);
    quaff_with_roll(&mut context, &mut |_| 1);
    assert_eq!(context.item.identified, 0);
    assert_eq!(context.flags.resist_heat, 11);
    assert!(context.destroyed);
}

#[test]
fn flags2_and_potion2_keep_the_legacy_32_and_31_offsets() {
    let mut context = Context::default();
    context.item.flags2 = 1 << 12;
    quaff_with_roll(&mut context, &mut |_| panic!("unexpected roll"));
    assert_eq!(context.mana, 10.0);
    assert_eq!(
        context.exp, 100,
        "flags2 alone does not award discovery experience"
    );
    assert_eq!(context.item.identified, 1);

    let mut context = Context::default();
    context.item.tval = ItemType::Potion2.into();
    context.item.flags = (1 << 0) | (1 << 13) | (1 << 17);
    quaff_with_roll(&mut context, &mut |_| panic!("unexpected roll"));
    assert_eq!(context.spells, [Spell::Learn]);
    assert_eq!(context.mana, 10.0);
    assert_eq!(
        context.item.identified, 0,
        "last effect is inactive petrification"
    );
}

#[test]
fn low_word_is_exhausted_before_high_word_even_when_bits_overlap() {
    let mut context = Context::default();
    context.item.flags = 1 << 44;
    context.item.flags2 = 1 << 0;
    quaff_with_roll(&mut context, &mut |_| 1);
    assert_eq!(context.events[0], "Your feel your head clear...");
    assert_eq!(context.events[1], "You feel your memories fade...");
}

#[test]
fn plain_potion_only_feeds_reports_consumes_and_redraws() {
    let mut context = Context::default();
    quaff_with_roll(&mut context, &mut |_| panic!("unexpected roll"));
    assert_eq!(
        context.events,
        ["eat 100", "remaining 1", "destroy", "redraw"]
    );
    assert_eq!(context.item.identified, 0);
}
