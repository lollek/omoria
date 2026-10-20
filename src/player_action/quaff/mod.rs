use crate::model::{Item, PlayerFlags, Stat};

#[cfg(not(test))]
mod globals;
#[cfg(not(test))]
mod interop;

const IS_POISONED: u64 = 0x0000_0020;

/// Potion behavior decoded from legacy flag positions, independent of storage.
#[derive(Clone, Copy, Debug, PartialEq)]
enum PotionEffect {
    GainStat(Stat),
    LoseStat(Stat),
    RestoreStat(Stat),
    HealLight,
    HealSerious,
    HealCritical,
    HealFully,
    GainExperience,
    Sleep,
    Blindness,
    Confusion,
    Poison,
    Haste,
    Slowness,
    DetectCreatures,
    /// The legacy potion ignores the cure spell's identification result.
    CureUnidentified(Status),
    LearnMagic,
    LoseMemories,
    SaltWater,
    Invulnerability,
    Heroism,
    SuperHeroism,
    Cure(Status),
    RestoreLevel,
    ResistHeat,
    ResistCold,
    DetectInvisible,
    SlowPoison,
    RestoreMana,
    InfraVision,
    CureHallucination,
    ReducePetrification,
    /// Unassigned legacy bits still suppress identification when processed last.
    Unused,
}

impl PotionEffect {
    /// Decodes the one-based C effect code after applying word/type offsets.
    fn from_legacy_code(code: u32) -> Self {
        use Stat::*;
        match code {
            1 => Self::GainStat(Strength),
            2 => Self::LoseStat(Strength),
            3 => Self::RestoreStat(Strength),
            4 => Self::GainStat(Intelligence),
            5 => Self::LoseStat(Intelligence),
            6 => Self::RestoreStat(Intelligence),
            7 => Self::GainStat(Wisdom),
            8 => Self::LoseStat(Wisdom),
            9 => Self::RestoreStat(Wisdom),
            10 => Self::GainStat(Charisma),
            11 => Self::LoseStat(Charisma),
            12 => Self::RestoreStat(Charisma),
            13 => Self::HealLight,
            14 => Self::HealSerious,
            15 => Self::HealCritical,
            16 => Self::HealFully,
            17 => Self::GainStat(Constitution),
            18 => Self::GainExperience,
            19 => Self::Sleep,
            20 => Self::Blindness,
            21 => Self::Confusion,
            22 => Self::Poison,
            23 => Self::Haste,
            24 => Self::Slowness,
            25 => Self::DetectCreatures,
            26 => Self::GainStat(Dexterity),
            27 => Self::RestoreStat(Dexterity),
            28 => Self::RestoreStat(Constitution),
            29 => Self::CureUnidentified(Status::Blind),
            30 => Self::CureUnidentified(Status::Confused),
            31 => Self::CureUnidentified(Status::Poisoned),
            32 | 48 => Self::LearnMagic,
            33 => Self::LoseMemories,
            34 => Self::SaltWater,
            35 => Self::Invulnerability,
            36 => Self::Heroism,
            37 => Self::SuperHeroism,
            38 => Self::Cure(Status::Afraid),
            39 => Self::RestoreLevel,
            40 => Self::ResistHeat,
            41 => Self::ResistCold,
            42 => Self::DetectInvisible,
            43 => Self::SlowPoison,
            44 => Self::Cure(Status::Poisoned),
            45 => Self::RestoreMana,
            46 => Self::InfraVision,
            47 => Self::CureHallucination,
            49 => Self::ReducePetrification,
            _ => Self::Unused,
        }
    }
}

/// Player status counters accepted by the C cure spell.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Status {
    Blind,
    Confused,
    Poisoned,
    Afraid,
    Image,
}

impl Status {
    /// Selects the counter in caller-owned state or at the C globals boundary.
    fn counter(self, flags: &mut PlayerFlags) -> &mut i64 {
        match self {
            Self::Blind => &mut flags.blind,
            Self::Confused => &mut flags.confused,
            Self::Poisoned => &mut flags.poisoned,
            Self::Afraid => &mut flags.afraid,
            Self::Image => &mut flags.image,
        }
    }
}

/// Spell requests delegated to C in production and recorded by test doubles.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Spell {
    Gain(Stat),
    Lose(Stat),
    Restore(Stat),
    Heal(i64),
    DetectCreatures,
    Cure(Status),
    Learn,
    LoseExperience(i64),
    RestoreLevels,
    DetectInvisible(i64),
    SlowPoison,
}

/// Player, spell, inventory, and UI boundaries needed by the selected potion.
///
/// The selected stack remains live through `remaining`; `destroy` invalidates it.
/// Spell calls may change player state, so flag snapshots must not span them.
trait QuaffContext {
    /// Copies the live selected item; must not be called after `destroy`.
    fn item(&mut self) -> Item;
    /// Reads current player flags without exposing mutable global state.
    fn flags(&self) -> PlayerFlags;
    /// Applies only the supplied flag edits, without replacing unrelated state.
    /// The closure must not call spells or other code accessing player globals.
    fn modify_flags(&mut self, modify: impl FnOnce(&mut PlayerFlags));
    /// Reads current experience, including any changes from preceding spells.
    fn experience(&mut self) -> i64;
    /// Reads the current player level for discovery-experience rounding.
    fn level(&mut self) -> u16;
    /// Raises current mana to maximum without reducing an above-maximum value.
    fn restore_mana(&mut self);
    /// Runs a spell and returns its identification result; callers may ignore it
    /// to preserve legacy potion behavior. Spells may update player state/UI.
    fn spell(&mut self, spell: Spell) -> bool;
    /// Awards experience through the normal player progression path.
    fn add_experience(&mut self, amount: i64);
    /// Applies nutrition through the hunger subsystem, including its messages.
    fn eat(&mut self, amount: i64);
    /// Prints one message; an empty message retains the C message-flush behavior.
    fn message(&mut self, message: &str);
    /// Refreshes the player stat block.
    fn redraw(&mut self);
    /// Refreshes the poison indicator after salt water clears poison state.
    fn redraw_poisoned(&mut self);
    /// Identifies the live selected potion using the shared item lifecycle.
    fn identify(&mut self);
    /// Reports the count that will remain, before destroying the selected item.
    fn remaining(&mut self);
    /// Consumes one selected potion and invalidates the selection, even when
    /// other potions remain in its stack. Must be called exactly once.
    fn destroy(&mut self);
}

/// Applies one decoded effect and returns its legacy identification result.
/// `roll` follows C `randint`: 1..=max, or zero when max is nonpositive.
fn effect_with_roll(
    effect: PotionEffect,
    context: &mut impl QuaffContext,
    roll: &mut impl FnMut(i64) -> i64,
) -> bool {
    use PotionEffect::*;
    match effect {
        GainStat(stat) => {
            let ident = context.spell(Spell::Gain(stat));
            if stat == Stat::Constitution {
                context.redraw();
            }
            ident
        }
        LoseStat(stat) => {
            if stat == Stat::Intelligence {
                context.message("This potion tastes very dull.");
            }
            context.spell(Spell::Lose(stat))
        }
        RestoreStat(stat) => context.spell(Spell::Restore(stat)),
        HealLight => context.spell(Spell::Heal((0..2).map(|_| roll(7)).sum())),
        HealSerious => context.spell(Spell::Heal((0..4).map(|_| roll(7)).sum())),
        HealCritical => context.spell(Spell::Heal((0..6).map(|_| roll(7)).sum())),
        HealFully => context.spell(Spell::Heal(1000)),
        GainExperience => {
            let gain = (context.experience() / 2 + 10).min(100_000);
            context.add_experience(gain);
            context.message("You feel more experienced.");
            context.redraw();
            true
        }
        Sleep => {
            if context.flags().free_act != 0 {
                return false;
            }
            context.message("You fall asleep.");
            let turns = roll(4) + 4;
            context.modify_flags(|flags| flags.paralysis += turns);
            true
        }
        Blindness => {
            context.message("You are covered by a veil of darkness.");
            let turns = roll(100) + 100;
            context.modify_flags(|flags| flags.blind += turns);
            true
        }
        Confusion => {
            context.message("Hey!  This is good stuff!  * Hick! *");
            let turns = roll(20) + 12;
            context.modify_flags(|flags| flags.confused += turns);
            true
        }
        Poison => {
            context.message("You feel very sick.");
            let turns = roll(15) + 10;
            context.modify_flags(|flags| flags.poisoned += turns);
            true
        }
        Haste => {
            let turns = roll(25) + 15;
            context.modify_flags(|flags| flags.fast += turns);
            true
        }
        Slowness => {
            let turns = roll(25) + 15;
            context.modify_flags(|flags| flags.slow += turns);
            true
        }
        DetectCreatures => context.spell(Spell::DetectCreatures),
        CureUnidentified(status) => {
            context.spell(Spell::Cure(status));
            false
        }
        LearnMagic => {
            context.spell(Spell::Learn);
            false
        }
        LoseMemories => {
            context.message("You feel your memories fade...");
            context.message("");
            let fifth = (context.experience() as f64 / 5.0).trunc() as i64;
            context.spell(Spell::LoseExperience(roll(fifth) + fifth));
            true
        }
        SaltWater => {
            context.modify_flags(|flags| {
                flags.poisoned = 0;
                flags.status &= !IS_POISONED;
            });
            context.redraw_poisoned();
            context.modify_flags(|flags| {
                flags.foodc = flags.foodc.min(150);
                flags.paralysis = 4;
            });
            context.message("The potion makes you vomit!");
            true
        }
        Invulnerability => {
            let turns = roll(10) + 10;
            context.modify_flags(|flags| flags.invuln += turns);
            true
        }
        Heroism => {
            let turns = roll(25) + 25;
            context.modify_flags(|flags| flags.hero += turns);
            true
        }
        SuperHeroism => {
            let turns = roll(25) + 25;
            context.modify_flags(|flags| flags.shero += turns);
            true
        }
        Cure(status) => context.spell(Spell::Cure(status)),
        RestoreLevel => {
            let ident = context.spell(Spell::RestoreLevels);
            context.eat(5000);
            ident
        }
        ResistHeat => {
            let turns = roll(10) + 10;
            context.modify_flags(|flags| flags.resist_heat += turns);
            false
        }
        ResistCold => {
            let turns = roll(10) + 10;
            context.modify_flags(|flags| flags.resist_cold += turns);
            false
        }
        DetectInvisible => {
            context.spell(Spell::DetectInvisible(roll(12) + 12));
            false
        }
        SlowPoison => context.spell(Spell::SlowPoison),
        RestoreMana => {
            context.restore_mana();
            context.message("Your feel your head clear...");
            true
        }
        InfraVision => {
            let turns = 100 + roll(100);
            context.modify_flags(|flags| flags.tim_infra += turns);
            context.message("Your eyes begin to tingle.");
            true
        }
        CureHallucination => {
            context.message("Pretty colors!");
            let turns = roll(5) + 5;
            context.modify_flags(|flags| flags.confused += turns);
            context.spell(Spell::Cure(Status::Image))
        }
        ReducePetrification => {
            if context.flags().petrification > 0 {
                context.modify_flags(|flags| {
                    flags.petrification = (flags.petrification - 100).max(0);
                });
                true
            } else {
                false
            }
        }
        Unused => false,
    }
}

/// Applies every stored effect, then identifies, feeds, reports, and consumes.
/// Selection must stay live until `destroy`; only the stat redraw follows it.
fn quaff_with_roll(context: &mut impl QuaffContext, roll: &mut impl FnMut(i64) -> i64) {
    let item = context.item();
    let mut ident = false;
    // bit_pos64 exhausts the low word first and offsets the high word by 32,
    // even though each word is now 64 bits. Potion2 keeps its old +31 offset.
    for (mut flags, offset) in [(item.flags, 0), (item.flags2, 32)] {
        while flags != 0 {
            let effect = flags.trailing_zeros()
                + 1
                + offset
                + if item.tval == u8::from(crate::model::ItemType::Potion2) {
                    31
                } else {
                    0
                };
            flags &= flags - 1;
            // C overwrites, rather than accumulates, identification per effect.
            ident = effect_with_roll(PotionEffect::from_legacy_code(effect), context, roll);
        }
    }
    if ident {
        context.identify();
    }
    let item = context.item();
    if item.flags != 0 {
        let gain = (f64::from(f32::from(item.level) / f32::from(context.level())) + 0.5) as i64;
        context.add_experience(gain);
        context.redraw();
    }
    context.eat(item.p1);
    context.remaining();
    context.destroy();
    context.redraw();
}

#[cfg(test)]
mod tests;
