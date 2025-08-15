use crate::identification;
use crate::model::item_subtype::{
    ItemSubType, MagicBookSubType, PrayerBookSubType, SongBookSubType,
};
use crate::model::{Item, ItemType};

pub fn book(item: &Item) -> String {
    let subtype = item.item_subtype();
    if !identification::is_identified(subtype) && !item.is_identified() {
        return "book".to_string();
    }

    match item.item_type() {
        ItemType::SongBook => match subtype {
            ItemSubType::SongBook(SongBookSubType::BeginnersHandbook) => {
                "song book I [book of bard lyrics]".to_string()
            }
            ItemSubType::SongBook(SongBookSubType::SongBook1) => {
                "song book II [songs of charming]".to_string()
            }
            ItemSubType::SongBook(SongBookSubType::SongBook2) => {
                "song book III [ballads of knowledge]".to_string()
            }
            ItemSubType::SongBook(SongBookSubType::GreaterSongBook) => {
                "song book IV [epics of the bards]".to_string()
            }
            _ => panic!("Expected item subtype to be a song book, got {:?}", subtype),
        },
        ItemType::MagicBook => match subtype {
            ItemSubType::MagicBook(MagicBookSubType::BeginnersMagic) => {
                "magic book I [book of beginner's magic]".to_string()
            }
            ItemSubType::MagicBook(MagicBookSubType::Magic1) => {
                "magic book II [book of magic spells]".to_string()
            }
            ItemSubType::MagicBook(MagicBookSubType::Magic2) => {
                "magic book III [book of greated spells]".to_string()
            }
            ItemSubType::MagicBook(MagicBookSubType::MagesGuideToPower) => {
                "magic book IV [mage's guide to power]".to_string()
            }
            _ => panic!(
                "Expected item subtype to be a magic book, got {:?}",
                subtype
            ),
        },
        ItemType::PrayerBook => match subtype {
            ItemSubType::PrayerBook(PrayerBookSubType::BeginnersHandbook) => {
                "prayer book I [beginners handbook]".to_string()
            }
            ItemSubType::PrayerBook(PrayerBookSubType::WordsOfWisdom) => {
                "prayer book II [words of wisdom]".to_string()
            }
            ItemSubType::PrayerBook(PrayerBookSubType::ChantsAndBlessings) => {
                "prayer book III [chants and blessings]".to_string()
            }
            ItemSubType::PrayerBook(PrayerBookSubType::ExorcismAndDispelling) => {
                "prayer book IV [exorcism and dispelling]".to_string()
            }
            _ => panic!(
                "Expected item subtype to be a prayer book, got {:?}",
                subtype
            ),
        },
        _ => panic!(
            "Expected item type to be a book, got {:?}",
            item.item_type()
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::item_name::generate;
    use crate::generate_item;
    use crate::generate_item::template::MagicBookTemplate;
    use crate::generate_item::ItemQuality;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_magic_book_2() {
        let mut item =
            generate_item::generate(Box::new(MagicBookTemplate::Magic1), 0, ItemQuality::Normal);
        identification::set_identified(ItemSubType::MagicBook(MagicBookSubType::Magic1), false);
        assert_eq!(generate(&item), "book");

        identification::set_identified(ItemSubType::MagicBook(MagicBookSubType::Magic1), true);
        assert_eq!(generate(&item), "magic book II [book of magic spells]");

        identification::set_identified(ItemSubType::MagicBook(MagicBookSubType::Magic1), false);
        item.set_identified(true);
        assert_eq!(generate(&item), "magic book II [book of magic spells]");
    }
}
