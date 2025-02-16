use crate::conversion::item_subtype;
use crate::identification::is_identified;
use crate::model::item_subtype::{BracersSubType, ItemSubType};
use crate::model::{Item, ItemType};

pub fn bracers(item: &Item) -> String {
    let Some(item_subtype) = item_subtype::from_i64(ItemType::Bracers, item.subval) else {
        return "alien bracers".to_string();
    };

    let subtype_is_known = is_identified(item_subtype);
    match item_subtype {
        ItemSubType::Bracers(bracers_subtype) => match bracers_subtype {
            BracersSubType::BracersOfProtection => if subtype_is_known {
                "bracers of protection"
            } else {
                "bracers"
            },
            BracersSubType::BracersOfDefense => if subtype_is_known {
                "bracers of defense"
            } else {
                "bracers"
            },
            BracersSubType::BracersOfShielding => if subtype_is_known {
                "bracers of shielding"
            } else {
                "bracers"
            },
            BracersSubType::MithrilBracers => "mithril bracers",
            BracersSubType::AdamantiteBracers => "adamantite bracers",
            BracersSubType::BracersOfWeaponAttraction => if subtype_is_known {
                "bracers of weapon attraction"
            } else {
                "bracers"
            },
            BracersSubType::SilverBraceletOfWarding => if subtype_is_known {
                "silver bracelet of warding"
            } else {
                "silver bracelet"
            },
            BracersSubType::SilverBracelet => "silver bracelet",
            BracersSubType::GoldBracelet => "gold bracelet",
            BracersSubType::PlatinumBracelet => "platinum bracelet",
            BracersSubType::LeatherBracers => "leather bracers",
            BracersSubType::StuddedLeatherBracers => "studded leather bracers",
            BracersSubType::LightPlatedBracers => "light plated bracers",
            BracersSubType::SharkskinBracers => "sharkskin bracers",
            BracersSubType::DemonhideBracers => "demonhide bracers",
            BracersSubType::WyrmhideBracers => "wymhide bracers",
            BracersSubType::ChainmailBracers => "chainmail bracers",
            BracersSubType::LamellarBracers => "lamellar bracers",
        }
        _ => "alien bracers",
    }.to_string()
}

#[cfg(test)]
mod tests {
    use crate::data::item_name::generate;
    use crate::generate_item::template::BracersTemplate;
    use crate::generate_item::ItemTemplate;
    use crate::{generate_item, identification};
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_names_unidentified_type() {
        for (template, expected_name) in [
            (Box::new(BracersTemplate::BracersOfProtection), "bracers"),
            (Box::new(BracersTemplate::BracersOfDefense), "bracers"),
            (Box::new(BracersTemplate::BracersOfShielding), "bracers"),
            (Box::new(BracersTemplate::MithrilBracers), "mithril bracers"),
            (Box::new(BracersTemplate::AdamantiteBracers), "adamantite bracers"),
            (Box::new(BracersTemplate::BracersOfWeaponAttraction), "bracers"),
            (Box::new(BracersTemplate::SilverBraceletOfWarding), "silver bracelet"),
            (Box::new(BracersTemplate::SilverBracelet), "silver bracelet"),
            (Box::new(BracersTemplate::GoldBracelet), "gold bracelet"),
            (Box::new(BracersTemplate::PlatinumBracelet), "platinum bracelet"),
            (Box::new(BracersTemplate::LeatherBracers), "leather bracers"),
            (Box::new(BracersTemplate::StuddedLeatherBracers), "studded leather bracers"),
            (Box::new(BracersTemplate::LightPlatedBracers), "light plated bracers"),
            (Box::new(BracersTemplate::SharkskinBracers), "sharkskin bracers"),
            (Box::new(BracersTemplate::DemonhideBracers), "demonhide bracers"),
            (Box::new(BracersTemplate::WyrmhideBracers), "wymhide bracers"),
            (Box::new(BracersTemplate::ChainmailBracers), "chainmail bracers"),
            (Box::new(BracersTemplate::LamellarBracers), "lamellar bracers"),
        ] {
            let item = generate_item::generate(template.clone(), 0);
            identification::set_identified(template.subtype(), false);
            assert_eq!(generate(&item), expected_name);
        }
    }

    #[test]
    #[serial]
    fn test_names_identified_type() {
        for (template, expected_name) in [
            (Box::new(BracersTemplate::BracersOfProtection), "bracers of protection"),
            (Box::new(BracersTemplate::BracersOfDefense), "bracers of defense"),
            (Box::new(BracersTemplate::BracersOfShielding), "bracers of shielding"),
            (Box::new(BracersTemplate::MithrilBracers), "mithril bracers"),
            (Box::new(BracersTemplate::AdamantiteBracers), "adamantite bracers"),
            (Box::new(BracersTemplate::BracersOfWeaponAttraction), "bracers of weapon attraction"),
            (Box::new(BracersTemplate::SilverBraceletOfWarding), "silver bracelet of warding"),
            (Box::new(BracersTemplate::SilverBracelet), "silver bracelet"),
            (Box::new(BracersTemplate::GoldBracelet), "gold bracelet"),
            (Box::new(BracersTemplate::PlatinumBracelet), "platinum bracelet"),
            (Box::new(BracersTemplate::LeatherBracers), "leather bracers"),
            (Box::new(BracersTemplate::StuddedLeatherBracers), "studded leather bracers"),
            (Box::new(BracersTemplate::LightPlatedBracers), "light plated bracers"),
            (Box::new(BracersTemplate::SharkskinBracers), "sharkskin bracers"),
            (Box::new(BracersTemplate::DemonhideBracers), "demonhide bracers"),
            (Box::new(BracersTemplate::WyrmhideBracers), "wymhide bracers"),
            (Box::new(BracersTemplate::ChainmailBracers), "chainmail bracers"),
            (Box::new(BracersTemplate::LamellarBracers), "lamellar bracers"),
        ] {
            let item = generate_item::generate(template.clone(), 0);
            identification::set_identified(template.subtype(), true);
            assert_eq!(generate(&item), expected_name);
        }
    }
}