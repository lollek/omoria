use crate::model::Magic;

pub fn from_usize(pos: usize) -> Option<Magic> {
    match pos {
        0 => Some(Magic::Arcane),
        1 => Some(Magic::Divine),
        2 => Some(Magic::Nature),
        3 => Some(Magic::Song),
        4 => Some(Magic::Chakra),
        _ => None,
    }
}
