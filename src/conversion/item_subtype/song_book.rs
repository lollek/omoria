use crate::model::item_subtype::SongBookSubType;

pub fn from_usize(subtype: usize) -> Option<SongBookSubType> {
    match subtype {
        262 => Some(SongBookSubType::BeginnersHandbook),
        263 => Some(SongBookSubType::SongBook1),
        264 => Some(SongBookSubType::SongBook2),
        265 => Some(SongBookSubType::GreaterSongBook),
        _ => None,
    }
}

pub fn to_usize(subtype: SongBookSubType) -> usize {
    match subtype {
        SongBookSubType::BeginnersHandbook => 262,
        SongBookSubType::SongBook1 => 263,
        SongBookSubType::SongBook2 => 264,
        SongBookSubType::GreaterSongBook => 265,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_conversion() {
        (0..1000).for_each(|i| {
            if let Some(subtype) = from_usize(i) {
                assert_eq!(i, to_usize(subtype));
            }
        })
    }
}
