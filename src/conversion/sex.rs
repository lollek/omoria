use crate::model::Sex;

pub fn from_char(ch: char) -> Option<Sex> {
    match ch {
        'F' => Some(Sex::Female),
        'M' => Some(Sex::Male),
        _ => None,
    }
}