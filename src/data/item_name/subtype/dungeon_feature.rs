use std::borrow::Cow;

use crate::data::item_name::helpers::{maybe_number_of, plural_s};
use crate::model::{ActivationKind, Item, ItemType};

pub fn dungeon_feature(item: &Item) -> String {
    let base = match item.item_type().expect("Item has no type") {
        ItemType::ClosedDoor => "closed door",
        ItemType::UnseenTrap => "unseen trap",

        // Other dungeon features are wired here too; keep a sane fallback while we
        // gradually add tests.
        ItemType::SeenTrap => "trap",
        ItemType::OpenDoor => "open door",
        ItemType::SecretDoor => "secret door",
        ItemType::UpStaircase => "up staircase",
        ItemType::DownStaircase => "down staircase",
        ItemType::UpSteepStaircase => "up steep staircase",
        ItemType::DownSteepStaircase => "down steep staircase",
        ItemType::EntranceToStore => match ActivationKind::from(item.subval) {
            ActivationKind::GeneralStore => "entrance to the General Store",
            ActivationKind::Armory => "entrance to the Armory",
            ActivationKind::WeaponSmiths => "entrance to the Weapon Smiths",
            ActivationKind::Temple => "entrance to the Temple",
            ActivationKind::AlchemyShop => "entrance to the Alchemy Shop",
            ActivationKind::MagicShop => "entrance to the Magic Shop",
            ActivationKind::Inn => "entrance to the Inn",
            ActivationKind::TradingPost => "entrance to the Trading Post",
            ActivationKind::Library => "entrance to the Library",
            ActivationKind::MusicShop => "entrance to the Music Shop",
            ActivationKind::InsuranceShop => "entrance to the Insurance Shop",
            ActivationKind::Bank => "entrance to the Bank",
            ActivationKind::GemStore => "entrance to the Gem Store",
            ActivationKind::MoneyExchange => "entrance to the Money Exchange",
            ActivationKind::Casino => "entrance to the Casino",
            ActivationKind::AllNiteDeli => "entrance to the All-Nite Deli",
            ActivationKind::Fortress => "entrance to the Fortress",
            ActivationKind::BlackMarket => "entrance to the Black Market",
            _ => "entrance to store",
        },
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
    use crate::data::item_name::generate;
    use crate::model::{Item, ItemType};

    fn base_item(item_type: ItemType) -> Item {
        Item {
            tval: item_type.into(),
            ..Item::default()
        }
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

    #[test]
    fn store_entrances_include_their_store_type() {
        for (subval, expected) in [
            (101, "entrance to the General Store"),
            (102, "entrance to the Armory"),
            (103, "entrance to the Weapon Smiths"),
            (104, "entrance to the Temple"),
            (105, "entrance to the Alchemy Shop"),
            (106, "entrance to the Magic Shop"),
            (107, "entrance to the Inn"),
            (108, "entrance to the Trading Post"),
            (109, "entrance to the Library"),
            (110, "entrance to the Music Shop"),
            (111, "entrance to the Insurance Shop"),
            (112, "entrance to the Bank"),
            (113, "entrance to the Gem Store"),
            (114, "entrance to the Money Exchange"),
            (115, "entrance to the Casino"),
            (116, "entrance to the All-Nite Deli"),
            (117, "entrance to the Fortress"),
            (118, "entrance to the Black Market"),
        ] {
            let mut item = base_item(ItemType::EntranceToStore);
            item.number = 1;
            item.subval = subval;
            assert_eq!(generate(&item), expected);
        }
    }
}
