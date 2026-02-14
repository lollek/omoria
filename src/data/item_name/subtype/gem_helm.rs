use std::borrow::Cow;

use crate::conversion::item_subtype;
use crate::data::item_name::helpers::{maybe_armor_bonus, maybe_number_of};
use crate::model::item_subtype::{GemHelmSubType, ItemSubType};
use crate::model::{Item, ItemType};

pub fn gem_helm(item: &Item) -> String {
    let mut parts: Vec<Cow<'static, str>> = Vec::new();

    if let Some(number_of_string) = maybe_number_of(item) {
        parts.push(number_of_string);
    }

    let Some(subtype) = item_subtype::from_i64(ItemType::GemHelm, item.subval) else {
        parts.push(Cow::Borrowed("alien helm"));
        if let Some(armor_string) = maybe_armor_bonus(item) {
            parts.push(armor_string);
        }
        return parts.join("");
    };

    let base = match subtype {
        ItemSubType::GemHelm(GemHelmSubType::IronHelm) => "iron helm",
        ItemSubType::GemHelm(GemHelmSubType::SteelHelm) => "steel helm",
        _ => "alien helm",
    };

    parts.push(Cow::Borrowed(base));
    if item.is_identified() {
        parts.push(Cow::Borrowed(" of gems"));
    }

    if let Some(armor_string) = maybe_armor_bonus(item) {
        parts.push(armor_string);
    }

    parts.join("")
}

#[cfg(test)]
mod tests {
    use crate::conversion;
    use crate::data::item_name::generate;
    use crate::model::{Item, ItemType};
    use serial_test::serial;

    use crate::model::item_subtype::{GemHelmSubType, ItemSubType};

    fn base_item() -> Item {
        let mut item = Item::default();
        item.tval = ItemType::GemHelm.into();
        item.ac = 1;
        item.toac = 2;
        item
    }

    fn subval(t: GemHelmSubType) -> i64 {
        conversion::item_subtype::to_usize(&ItemSubType::GemHelm(t)) as i64
    }

    #[test]
    #[serial]
    fn test_gem_helm_iron_unidentified() {
        let mut item = base_item();
        item.subval = subval(GemHelmSubType::IronHelm);
        item.number = 1;
        item.set_identified(false);

        // Wearable armor shows base AC even when unidentified.
        assert_eq!(generate(&item), "iron helm [1]");
    }

    #[test]
    #[serial]
    fn test_gem_helm_iron_identified_adds_of_gems() {
        let mut item = base_item();
        item.subval = subval(GemHelmSubType::IronHelm);
        item.number = 1;
        item.set_identified(true);

        assert_eq!(generate(&item), "iron helm of gems [1,+2]");
    }

    #[test]
    #[serial]
    fn test_gem_helm_steel_unidentified() {
        let mut item = base_item();
        item.subval = subval(GemHelmSubType::SteelHelm);
        item.number = 1;
        item.set_identified(false);

        assert_eq!(generate(&item), "steel helm [1]");
    }

    #[test]
    #[serial]
    fn test_gem_helm_multiple_prefix_no_plural() {
        let mut item = base_item();
        item.subval = subval(GemHelmSubType::IronHelm);
        item.number = 2;
        item.set_identified(false);

        assert_eq!(generate(&item), "2 iron helm [1]");
    }

    #[test]
    #[serial]
    fn test_gem_helm_none_prefix_no_plural() {
        let mut item = base_item();
        item.subval = subval(GemHelmSubType::IronHelm);
        item.number = 0;
        item.set_identified(false);

        assert_eq!(generate(&item), "no more iron helm [1]");
    }

    #[test]
    #[serial]
    fn test_gem_helm_of_gems_is_gated_by_item_identification() {
        let mut item = base_item();
        item.subval = subval(GemHelmSubType::IronHelm);
        item.number = 1;

        item.set_identified(false);
        assert_eq!(generate(&item), "iron helm [1]");

        item.set_identified(true);
        assert_eq!(generate(&item), "iron helm of gems [1,+2]");
    }
}
