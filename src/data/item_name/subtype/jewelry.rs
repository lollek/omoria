use crate::conversion::item_subtype::from_i64;
use crate::data::item_name::helpers::{maybe_number_of, plural_s};
use crate::model::item_subtype::{ItemSubType, JewelrySubType};
use crate::model::{Item, ItemType};
use std::borrow::Cow;

pub fn jewelry(item: &Item) -> String {
    let mut parts = vec![];

    if let Some(number_of_string) = maybe_number_of(item) {
        parts.push(number_of_string);
    }
    parts.push(Cow::Borrowed(
        match from_i64(ItemType::Jewelry, item.subval) {
            Some(subtype) => match subtype {
                ItemSubType::Jewelry(JewelrySubType::SmallGoldPendant) => "small gold pendant",
                ItemSubType::Jewelry(JewelrySubType::SmallMithrilPendant) => {
                    "small mithril pendant"
                }
                ItemSubType::Jewelry(JewelrySubType::LargeMithrilGarterBelt) => {
                    "large mithril garter belt"
                }
                ItemSubType::Jewelry(JewelrySubType::SmallSilverPendant) => "small silver pendant",
                t => panic!("Expected jewelry, got {:?}", t),
            },
            None => "alien jewelry",
        },
    ));
    parts.push(plural_s(item));
    parts.join("")
}

#[cfg(test)]
mod tests {
    use crate::data::item_name::generate;
    use crate::generate_item;
    use crate::generate_item::ItemQuality;
    use crate::generate_item::template::ValuableTemplate;

    #[test]
    fn test_jewelry_small_gold_pendant() {
        let mut item = generate_item::generate(Box::new(ValuableTemplate::SmallGoldPendant), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "small gold pendant");

        item.number = 0;
        assert_eq!(generate(&item), "no more small gold pendants");

        item.number = 5;
        assert_eq!(generate(&item), "5 small gold pendants");
    }

    #[test]
    fn test_jewelry_small_mithril_pendant() {
        let mut item = generate_item::generate(Box::new(ValuableTemplate::SmallMithrilPendant), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "small mithril pendant");

        item.number = 0;
        assert_eq!(generate(&item), "no more small mithril pendants");

        item.number = 5;
        assert_eq!(generate(&item), "5 small mithril pendants");
    }

    #[test]
    fn test_jewelry_large_mithril_garter_belt() {
        let mut item =
            generate_item::generate(Box::new(ValuableTemplate::LargeMithrilGarterBelt), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "large mithril garter belt");

        item.number = 0;
        assert_eq!(generate(&item), "no more large mithril garter belts");

        item.number = 5;
        assert_eq!(generate(&item), "5 large mithril garter belts");
    }

    #[test]
    fn test_jewelry_small_silver_pendant() {
        let mut item = generate_item::generate(Box::new(ValuableTemplate::SmallSilverPendant), 0, ItemQuality::Normal);
        assert_eq!(generate(&item), "small silver pendant");

        item.number = 0;
        assert_eq!(generate(&item), "no more small silver pendants");

        item.number = 5;
        assert_eq!(generate(&item), "5 small silver pendants");
    }
}
