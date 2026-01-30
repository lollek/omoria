use crate::conversion::item_subtype;
use crate::data::item_name::helpers::{maybe_armor_bonus, maybe_number_of, plural_s};
use crate::model::item_subtype::{ItemSubType, ShieldSubType};
use crate::model::Item;

pub fn shield(item: &Item) -> String {
    let Some(subtype) = item_subtype::from_i64(item.item_type(), item.subval) else {
        return "alien shield".to_string();
    };

    let mut parts: Vec<String> = Vec::new();
    if let Some(number_of_string) = maybe_number_of(item) {
        parts.push(number_of_string.to_string());
    }

    parts.push(subtype_name(item, subtype));

    if let Some(armor_string) = maybe_armor_bonus(item) {
        parts.push(armor_string.to_string());
    }

    parts.join("")
}

fn subtype_name(item: &Item, subtype: ItemSubType) -> String {
    let base = match subtype {
        ItemSubType::Shield(shield_type) => match shield_type {
            ShieldSubType::SmallLeatherShield => "small leather shield",
            ShieldSubType::MediumLeatherShield => "medium leather shield",
            ShieldSubType::LargeLeatherShield => "large leather shield",
            ShieldSubType::Buckler => "buckler",
            ShieldSubType::KiteShield => "kite shield",
            ShieldSubType::TowerShield => "tower shield",
            ShieldSubType::SharkskinShield => "sharkskin shield",
            ShieldSubType::DemonhideShield => "demonhide shield",
            ShieldSubType::WyrmhideShield => "wyrmhide shield",
        },
        _ => panic!("coding error, unexpected item type: {:?}", item.item_type()),
    };

    format!("{}{}", base, plural_s(item))
}

#[cfg(test)]
mod tests {
    use crate::data::item_name::generate;
    use crate::generate_item;
    use crate::generate_item::template::ShieldTemplate;
    use crate::generate_item::{ItemQuality, ItemTemplate};
    use crate::identification;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_shield_unidentified_single_shows_base_ac_only() {
        let template = ShieldTemplate::SmallLeatherShield;
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        item.set_identified(false);
        item.number = 1;
        item.ac = 2;
        item.toac = 1;

        // Unidentified shields should not show toac bonus.
        assert_eq!(generate(&item), "small leather shield [2]");
    }

    #[test]
    #[serial]
    fn test_shield_identified_single_shows_ac_and_toac() {
        let template = ShieldTemplate::SmallLeatherShield;
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        item.set_identified(true);
        item.number = 1;
        item.ac = 2;
        item.toac = 1;

        assert_eq!(generate(&item), "small leather shield [2,+1]");
    }

    #[test]
    #[serial]
    fn test_shield_multiple_prefixes_with_count() {
        let template = ShieldTemplate::SmallLeatherShield;
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        item.set_identified(false);
        item.number = 2;
        item.ac = 2;

        assert_eq!(generate(&item), "2 small leather shields [2]");
    }

    #[test]
    #[serial]
    fn test_shield_none_prefixes_with_no_more() {
        let template = ShieldTemplate::SmallLeatherShield;
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        item.set_identified(false);
        item.number = 0;
        item.ac = 2;

        assert_eq!(generate(&item), "no more small leather shields [2]");
    }

    #[test]
    #[serial]
    fn test_shield_unknown_subtype_is_not_alien_shield() {
        // The old generic/subtype path used subval integer mapping and returned "Alien shield"
        // for many valid shields. With enum-based templates we should never hit that.
        let template = ShieldTemplate::WyrmhideShield;
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        item.set_identified(false);
        item.number = 1;
        item.ac = 3;

        assert_eq!(generate(&item), "wyrmhide shield [3]");
    }

    #[test]
    #[serial]
    fn test_shield_known_subtype_but_not_identified_matches_unidentified_format() {
        let template = ShieldTemplate::SmallLeatherShield;
        let subtype = template.subtype();
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        item.set_identified(false);
        item.number = 1;
        item.ac = 2;
        item.toac = 1;

        // Unknown subtype
        identification::set_identified(subtype, false);
        assert_eq!(generate(&item), "small leather shield [2]");

        // Known subtype (but still not identified item)
        identification::set_identified(subtype, true);
        assert_eq!(generate(&item), "small leather shield [2]");

        // Avoid leaking global state to other tests.
        identification::set_identified(subtype, false);
    }
}
