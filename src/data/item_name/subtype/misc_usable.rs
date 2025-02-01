use crate::conversion::item_subtype::from_i64;
use crate::data::item_name::helpers::no_more;
use crate::model::item_subtype::{ItemSubType, MiscUsableSubType};
use crate::model::{Item, ItemType};
use crate::{
    data::item_name::generate,
    generate_item::{self, template::MiscUsableTemplate},
};
use std::borrow::Cow;

pub fn misc_usable(item: &Item) -> String {
    vec![
        no_more(item),
        Cow::Borrowed(match from_i64(ItemType::MiscUsable, item.subval) {
            Some(subtype) => match subtype {
                ItemSubType::MiscUsable(MiscUsableSubType::Statue) => "statue",
                ItemSubType::MiscUsable(MiscUsableSubType::SilverCross) => "silver cross",
                ItemSubType::MiscUsable(MiscUsableSubType::GoldCross) => "gold cross",
                ItemSubType::MiscUsable(MiscUsableSubType::MithrilCross) => "mithril cross",
                ItemSubType::MiscUsable(MiscUsableSubType::Cross) => "cross",
                ItemSubType::MiscUsable(MiscUsableSubType::CorkedBottle) => "corked bottle",
                t => panic!("Expected misc usable, got {:?}", t),
            },
            None => "alien usable item",
        }),
    ]
    .join("")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_misc_usable_statue() {
        let mut item = generate_item::generate(Box::new(MiscUsableTemplate::Statue), 0);
        assert_eq!(generate(&item), "statue");

        item.number = 0;
        assert_eq!(generate(&item), "no more statue");
    }

    #[test]
    fn test_misc_usable_silver_cross() {
        let mut item = generate_item::generate(Box::new(MiscUsableTemplate::SilverCross), 0);
        assert_eq!(generate(&item), "silver cross");

        item.number = 0;
        assert_eq!(generate(&item), "no more silver cross");
    }

    #[test]
    fn test_misc_usable_gold_cross() {
        let mut item = generate_item::generate(Box::new(MiscUsableTemplate::GoldCross), 0);
        assert_eq!(generate(&item), "gold cross");

        item.number = 0;
        assert_eq!(generate(&item), "no more gold cross");
    }

    #[test]
    fn test_misc_usable_mithril_cross() {
        let mut item = generate_item::generate(Box::new(MiscUsableTemplate::MithrilCross), 0);
        assert_eq!(generate(&item), "mithril cross");

        item.number = 0;
        assert_eq!(generate(&item), "no more mithril cross");
    }

    #[test]
    fn test_misc_usable_cross() {
        let mut item = generate_item::generate(Box::new(MiscUsableTemplate::Cross), 0);
        assert_eq!(generate(&item), "cross");

        item.number = 0;
        assert_eq!(generate(&item), "no more cross");
    }

    #[test]
    fn test_misc_usable_corked_bottle() {
        let mut item = generate_item::generate(Box::new(MiscUsableTemplate::CorkedBottle), 0);
        assert_eq!(generate(&item), "corked bottle");

        item.number = 0;
        assert_eq!(generate(&item), "no more corked bottle");
    }
}
