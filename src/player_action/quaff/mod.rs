use crate::model::{Item, PlayerFlags, Stat};

#[cfg(not(test))]
mod globals;
#[cfg(not(test))]
mod interop;

const IS_POISONED: u64 = 0x0000_0020;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Status {
    Blind,
    Confused,
    Poisoned,
    Afraid,
    Image,
}

impl Status {
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

/// The selected stack remains live through `remaining`; `destroy` invalidates it.
/// Spell calls may change player state, so flags must not be cached across calls.
trait QuaffContext {
    fn item(&mut self) -> Item;
    fn flags(&mut self) -> &mut PlayerFlags;
    fn experience(&mut self) -> i64;
    fn level(&mut self) -> u16;
    fn restore_mana(&mut self);
    fn spell(&mut self, spell: Spell) -> bool;
    fn add_experience(&mut self, amount: i64);
    fn eat(&mut self, amount: i64);
    fn message(&mut self, message: &str);
    fn redraw(&mut self);
    fn redraw_poisoned(&mut self);
    fn identify(&mut self);
    fn remaining(&mut self);
    fn destroy(&mut self);
}

fn effect_with_roll(
    effect: u32,
    context: &mut impl QuaffContext,
    roll: &mut impl FnMut(i64) -> i64,
) -> bool {
    use Stat::*;
    match effect {
        1 => context.spell(Spell::Gain(Strength)),
        2 => context.spell(Spell::Lose(Strength)),
        3 => context.spell(Spell::Restore(Strength)),
        4 => context.spell(Spell::Gain(Intelligence)),
        5 => {
            context.message("This potion tastes very dull.");
            context.spell(Spell::Lose(Intelligence))
        }
        6 => context.spell(Spell::Restore(Intelligence)),
        7 => context.spell(Spell::Gain(Wisdom)),
        8 => context.spell(Spell::Lose(Wisdom)),
        9 => context.spell(Spell::Restore(Wisdom)),
        10 => context.spell(Spell::Gain(Charisma)),
        11 => context.spell(Spell::Lose(Charisma)),
        12 => context.spell(Spell::Restore(Charisma)),
        13..=16 => {
            let healing = if effect == 16 {
                1000
            } else {
                (0..(effect - 12) * 2).map(|_| roll(7)).sum()
            };
            context.spell(Spell::Heal(healing))
        }
        17 => {
            let ident = context.spell(Spell::Gain(Constitution));
            context.redraw();
            ident
        }
        18 => {
            let gain = (context.experience() / 2 + 10).min(100_000);
            context.add_experience(gain);
            context.message("You feel more experienced.");
            context.redraw();
            true
        }
        19 => {
            if context.flags().free_act != 0 {
                return false;
            }
            context.message("You fall asleep.");
            context.flags().paralysis += roll(4) + 4;
            true
        }
        20 => {
            context.message("You are covered by a veil of darkness.");
            context.flags().blind += roll(100) + 100;
            true
        }
        21 => {
            context.message("Hey!  This is good stuff!  * Hick! *");
            context.flags().confused += roll(20) + 12;
            true
        }
        22 => {
            context.message("You feel very sick.");
            context.flags().poisoned += roll(15) + 10;
            true
        }
        23 => {
            context.flags().fast += roll(25) + 15;
            true
        }
        24 => {
            context.flags().slow += roll(25) + 15;
            true
        }
        25 => context.spell(Spell::DetectCreatures),
        26 => context.spell(Spell::Gain(Dexterity)),
        27 => context.spell(Spell::Restore(Dexterity)),
        28 => context.spell(Spell::Restore(Constitution)),
        29..=31 => {
            let status = match effect {
                29 => Status::Blind,
                30 => Status::Confused,
                _ => Status::Poisoned,
            };
            context.spell(Spell::Cure(status));
            false
        }
        32 | 48 => {
            context.spell(Spell::Learn);
            false
        }
        33 => {
            context.message("You feel your memories fade...");
            context.message("");
            let fifth = (context.experience() as f64 / 5.0).trunc() as i64;
            context.spell(Spell::LoseExperience(roll(fifth) + fifth));
            true
        }
        34 => {
            context.flags().poisoned = 0;
            context.flags().status &= !IS_POISONED;
            context.redraw_poisoned();
            context.flags().foodc = context.flags().foodc.min(150);
            context.flags().paralysis = 4;
            context.message("The potion makes you vomit!");
            true
        }
        35 => {
            context.flags().invuln += roll(10) + 10;
            true
        }
        36 => {
            context.flags().hero += roll(25) + 25;
            true
        }
        37 => {
            context.flags().shero += roll(25) + 25;
            true
        }
        38 => context.spell(Spell::Cure(Status::Afraid)),
        39 => {
            let ident = context.spell(Spell::RestoreLevels);
            context.eat(5000);
            ident
        }
        40 => {
            context.flags().resist_heat += roll(10) + 10;
            false
        }
        41 => {
            context.flags().resist_cold += roll(10) + 10;
            false
        }
        42 => {
            context.spell(Spell::DetectInvisible(roll(12) + 12));
            false
        }
        43 => context.spell(Spell::SlowPoison),
        44 => context.spell(Spell::Cure(Status::Poisoned)),
        45 => {
            context.restore_mana();
            context.message("Your feel your head clear...");
            true
        }
        46 => {
            context.flags().tim_infra += 100 + roll(100);
            context.message("Your eyes begin to tingle.");
            true
        }
        47 => {
            context.message("Pretty colors!");
            context.flags().confused += roll(5) + 5;
            context.spell(Spell::Cure(Status::Image))
        }
        49 => {
            let counter = &mut context.flags().petrification;
            if *counter > 0 {
                *counter = (*counter - 100).max(0);
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

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
            ident = effect_with_roll(effect, context, roll);
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
