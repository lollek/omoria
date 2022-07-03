use model::Race;

pub fn from_usize(pos: usize) -> Option<Race> {
    match pos {
        0 => Some(Race::Human),
        1 => Some(Race::HalfElf),
        2 => Some(Race::Elf),
        3 => Some(Race::Halfling),
        4 => Some(Race::Gnome),
        5 => Some(Race::Dwarf),
        6 => Some(Race::HalfOrc),
        7 => Some(Race::HalfTroll),
        8 => Some(Race::Phraint),
        9 => Some(Race::Dryad),
        _ => None,
    }
}

pub fn to_usize(pos: &Race) -> usize {
    match pos {
        Race::Human => 0,
        Race::HalfElf => 1,
        Race::Elf => 2,
        Race::Halfling => 3,
        Race::Gnome => 4,
        Race::Dwarf => 5,
        Race::HalfOrc => 6,
        Race::HalfTroll => 7,
        Race::Phraint => 8,
        Race::Dryad => 9,
    }
}
