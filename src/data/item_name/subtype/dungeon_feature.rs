use std::borrow::Cow;

use crate::data::item_name::helpers::{maybe_number_of, plural_s};
use crate::model::{Item, ItemType};

pub fn dungeon_feature(item: &Item) -> String {
    let base = match item.item_type() {
        ItemType::ClosedDoor => "closed door",
        ItemType::UnseenTrap => "unseen trap",

        // Other dungeon features are wired here too; keep a sane fallback while we
        // gradually add tests.
        ItemType::SeenTrap => "seen trap",
        ItemType::OpenDoor => "open door",
        ItemType::SecretDoor => "secret door",
        ItemType::UpStaircase => "up staircase",
        ItemType::DownStaircase => "down staircase",
        ItemType::UpSteepStaircase => "up steep staircase",
        ItemType::DownSteepStaircase => "down steep staircase",
        ItemType::EntranceToStore => "entrance to store",
        ItemType::Rubble => "rubble",
        ItemType::Whirlpool => "whirlpool",
        _ => "dungeon feature",
    };

    let mut parts: Vec<Cow<'static, str>> = Vec::new();
    if let Some(number_of_string) = maybe_number_of(item) {
        parts.push(number_of_string);
    }

    parts.push(Cow::Borrowed(base));

    // We pluralize all dungeon features consistently for now. If we later decide
    // some should never pluralize (e.g. rubble), we can split that out with tests.
    parts.push(plural_s(item));

    parts.join("")
}

#[cfg(test)]
mod tests {
    use crate::conversion;
    use crate::data::item_name::generate;
    use crate::model::{Item, ItemType};

    fn base_item(item_type: ItemType) -> Item {
        let mut item = Item::default();
        item.tval = conversion::item_type::to_usize(item_type) as u8;
        item
    }

    #[test]
    fn test_closed_door_single() {
        let mut item = base_item(ItemType::ClosedDoor);
        item.number = 1;
        assert_eq!(generate(&item), "closed door");
    }

    #[test]
    fn test_closed_door_multiple_prefix() {
        let mut item = base_item(ItemType::ClosedDoor);
        item.number = 2;
        assert_eq!(generate(&item), "2 closed doors");
    }

    #[test]
    fn test_unseen_trap_none_prefix() {
        let mut item = base_item(ItemType::UnseenTrap);
        item.number = 0;
        assert_eq!(generate(&item), "no more unseen traps");
    }
}
