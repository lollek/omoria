use crate::model::{Race, Sex};
use crate::rng;

pub fn generate(race: &Race, sex: &Sex) -> u16 {
    match race {
        Race::Human => match sex {
            Sex::Male => rng::randnor(72, 6) as u16,
            Sex::Female => rng::randnor(66, 4) as u16,
        },
        Race::HalfElf => match sex {
            Sex::Male => rng::randnor(66, 6) as u16,
            Sex::Female => rng::randnor(62, 6) as u16,
        },
        Race::Elf => match sex {
            Sex::Male => rng::randnor(60, 4) as u16,
            Sex::Female => rng::randnor(54, 4) as u16,
        },
        Race::Halfling => match sex {
            Sex::Male => rng::randnor(36, 3) as u16,
            Sex::Female => rng::randnor(33, 3) as u16,
        },
        Race::Gnome => match sex {
            Sex::Male => rng::randnor(42, 3) as u16,
            Sex::Female => rng::randnor(39, 3) as u16,
        },
        Race::Dwarf => match sex {
            Sex::Male => rng::randnor(48, 3) as u16,
            Sex::Female => rng::randnor(46, 3) as u16,
        },
        Race::HalfOrc => match sex {
            Sex::Male => rng::randnor(66, 1) as u16,
            Sex::Female => rng::randnor(62, 3) as u16,
        },
        Race::HalfTroll => match sex {
            Sex::Male => rng::randnor(96, 10) as u16,
            Sex::Female => rng::randnor(84, 12) as u16,
        },
        Race::Phraint => match sex {
            Sex::Male => rng::randnor(96, 24) as u16,
            Sex::Female => rng::randnor(84, 12) as u16,
        },
        Race::Dryad => match sex {
            Sex::Male => rng::randnor(60, 4) as u16,
            Sex::Female => rng::randnor(40, 4) as u16,
        },
    }
}
