use std::borrow::Cow;
use crate::{
    data::item_name::generate,
    generate_item::{self, template::ChestTemplate},
};
use crate::conversion::item_subtype::from_i64;
use crate::data::item_name::helpers::no_more;
use crate::model::{Item, ItemType};
use crate::model::item_subtype::{ChestSubType, ItemSubType};

pub fn chest(item: &Item) -> String {
    vec![
        no_more(item),
        Cow::Borrowed(match from_i64(ItemType::Chest, item.subval) {
            Some(subtype) => match subtype {
                ItemSubType::Chest(ChestSubType::SmallWoodenChest) => "small wooden chest",
                ItemSubType::Chest(ChestSubType::LargeWoodenChest) => "large wooden chest",
                ItemSubType::Chest(ChestSubType::SmallIronChest) => "small iron chest",
                ItemSubType::Chest(ChestSubType::LargeIronChest) => "large iron chest",
                ItemSubType::Chest(ChestSubType::SmallSteelChest) => "small steel chest",
                ItemSubType::Chest(ChestSubType::LargeSteelChest) => "large steel chest",
                t => panic!("Expected chest, got {:?}", t),
            },
            None => "alien chest",
        }),
    ]
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chest_small_wooden_chest() {
        let mut item = generate_item::generate(Box::new(ChestTemplate::SmallWoodenChest), 0);
        assert_eq!(generate(&item), "small wooden chest");

        item.number = 0;
        assert_eq!(generate(&item), "no more small wooden chest");
    }

    #[test]
    fn test_chest_large_wooden_chest() {
        let mut item = generate_item::generate(Box::new(ChestTemplate::LargeWoodenChest), 0);
        assert_eq!(generate(&item), "large wooden chest");

        item.number = 0;
        assert_eq!(generate(&item), "no more large wooden chest");
    }

    #[test]
    fn test_chest_small_iron_chest() {
        let mut item = generate_item::generate(Box::new(ChestTemplate::SmallIronChest), 0);
        assert_eq!(generate(&item), "small iron chest");

        item.number = 0;
        assert_eq!(generate(&item), "no more small iron chest");
    }

    #[test]
    fn test_chest_large_iron_chest() {
        let mut item = generate_item::generate(Box::new(ChestTemplate::LargeIronChest), 0);
        assert_eq!(generate(&item), "large iron chest");

        item.number = 0;
        assert_eq!(generate(&item), "no more large iron chest");
    }

    #[test]
    fn test_chest_small_steel_chest() {
        let mut item = generate_item::generate(Box::new(ChestTemplate::SmallSteelChest), 0);
        assert_eq!(generate(&item), "small steel chest");

        item.number = 0;
        assert_eq!(generate(&item), "no more small steel chest");
    }

    #[test]
    fn test_chest_large_steel_chest() {
        let mut item = generate_item::generate(Box::new(ChestTemplate::LargeSteelChest), 0);
        assert_eq!(generate(&item), "large steel chest");

        item.number = 0;
        assert_eq!(generate(&item), "no more large steel chest");
    }
}
