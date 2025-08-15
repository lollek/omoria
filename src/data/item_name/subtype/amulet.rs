use crate::conversion::item_subtype;
use crate::data::item_name::helpers::p1_bonus;
use crate::identification;
use crate::model::item_subtype::{AmuletSubType, ItemSubType};
use crate::model::{Item, ItemType};

pub fn amulet(item: &Item) -> String {
    let Some(ItemSubType::Amulet(amulet_subtype)) =
        item_subtype::from_i64(ItemType::Amulet, item.subval)
    else {
        return "alien amulet".to_string();
    };

    let known_type = identification::is_identified(ItemSubType::Amulet(amulet_subtype)) || item.is_identified();
    if !known_type {
        return match amulet_subtype {
            AmuletSubType::AmuletOfAdornment1
            | AmuletSubType::AmuletOfAdornment2
            | AmuletSubType::AmuletOfWisdom
            | AmuletSubType::AmuletOfCharisma
            | AmuletSubType::AmuletOfSearching
            | AmuletSubType::AmuletOfTeleportation
            | AmuletSubType::AmuletOfSlowDigestion
            | AmuletSubType::AmuletOfResistAcid
            | AmuletSubType::AmuletOfTheMagi
            | AmuletSubType::AmuletOfDoom => "amulet".to_string(),
            AmuletSubType::SilverNecklace => {
                if item.number == 1 {
                    "silver necklace".to_string()
                } else {
                    format!("{} silver necklaces", item.number)
                }
            }
            AmuletSubType::GoldNecklace => {
                if item.number == 1 {
                    "gold necklace".to_string()
                } else {
                    format!("{} gold necklaces", item.number)
                }
            }
            AmuletSubType::MithrilNecklace => {
                if item.number == 1 {
                    "mithril necklace".to_string()
                } else {
                    format!("{} mithril necklaces", item.number)
                }
            }
        };
    }

    match amulet_subtype {
        AmuletSubType::AmuletOfAdornment1 => "amulet of adornment".to_string(),
        AmuletSubType::AmuletOfAdornment2 => "amulet of adornment".to_string(),
        AmuletSubType::AmuletOfWisdom => {
            if item.is_identified() {
                format!("amulet of wisdom{}", p1_bonus(item))
            } else {
                "amulet of wisdom".to_string()
            }
        }
        AmuletSubType::AmuletOfCharisma => {
            if item.is_identified() {
                format!("amulet of charisma{}", p1_bonus(item))
            } else {
                "amulet of charisma".to_string()
            }
        }
        AmuletSubType::AmuletOfSearching => {
            if item.is_identified() {
                format!("amulet of searching{}", p1_bonus(item))
            } else {
                "amulet of searching".to_string()
            }
        }
        AmuletSubType::AmuletOfTeleportation => "amulet of teleportation".to_string(),
        AmuletSubType::AmuletOfSlowDigestion => "amulet of slow digestion".to_string(),
        AmuletSubType::AmuletOfResistAcid => "amulet of resist acid".to_string(),
        AmuletSubType::AmuletOfTheMagi => "amulet of the magi".to_string(),
        AmuletSubType::AmuletOfDoom => "amulet of doom".to_string(),
        AmuletSubType::SilverNecklace => {
            if item.number == 1 {
                "silver necklace".to_string()
            } else {
                format!("{} silver necklaces", item.number)
            }
        }
        AmuletSubType::GoldNecklace => {
            if item.number == 1 {
                "gold necklace".to_string()
            } else {
                format!("{} gold necklaces", item.number)
            }
        }
        AmuletSubType::MithrilNecklace => {
            if item.number == 1 {
                "mithril necklace".to_string()
            } else {
                format!("{} mithril necklaces", item.number)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::data::item_name::generate;
    use crate::generate_item::template::AmuletTemplate;
    use crate::generate_item::{ItemQuality, ItemTemplate};
    use crate::{generate_item, identification};
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_names_single_unidentified() {
        for (template, expected_name) in [
            (Box::new(AmuletTemplate::AmuletOfAdornment1), "amulet"),
            (Box::new(AmuletTemplate::AmuletOfAdornment2), "amulet"),
            (Box::new(AmuletTemplate::AmuletOfWisdom), "amulet"),
            (Box::new(AmuletTemplate::AmuletOfCharisma), "amulet"),
            (Box::new(AmuletTemplate::AmuletOfSearching), "amulet"),
            (Box::new(AmuletTemplate::AmuletOfTeleportation), "amulet"),
            (Box::new(AmuletTemplate::AmuletOfSlowDigestion), "amulet"),
            (Box::new(AmuletTemplate::AmuletOfResistAcid), "amulet"),
            (Box::new(AmuletTemplate::AmuletOfTheMagi), "amulet"),
            (Box::new(AmuletTemplate::AmuletOfDoom), "amulet"),
            (Box::new(AmuletTemplate::SilverNecklace), "silver necklace"),
            (Box::new(AmuletTemplate::GoldNecklace), "gold necklace"),
            (
                Box::new(AmuletTemplate::MithrilNecklace),
                "mithril necklace",
            ),
        ] {
            let item = generate_item::generate(template.clone(), 0, ItemQuality::Normal);
            identification::set_identified(template.subtype(), false);
            assert_eq!(generate(&item), expected_name);
        }
    }

    #[test]
    #[serial]
    fn test_names_multiple_unidentified() {
        for (template, expected_name) in [
            (
                Box::new(AmuletTemplate::SilverNecklace),
                "2 silver necklaces",
            ),
            (Box::new(AmuletTemplate::GoldNecklace), "2 gold necklaces"),
            (
                Box::new(AmuletTemplate::MithrilNecklace),
                "2 mithril necklaces",
            ),
        ] {
            let mut item = generate_item::generate(template.clone(), 0, ItemQuality::Normal);
            item.number = 2;
            identification::set_identified(template.subtype(), false);
            assert_eq!(generate(&item), expected_name);
        }
    }

    #[test]
    #[serial]
    fn test_names_single_known_type_unidentified() {
        for (template, expected_name) in [
            (
                Box::new(AmuletTemplate::AmuletOfAdornment1),
                "amulet of adornment",
            ),
            (
                Box::new(AmuletTemplate::AmuletOfAdornment2),
                "amulet of adornment",
            ),
            (Box::new(AmuletTemplate::AmuletOfWisdom), "amulet of wisdom"),
            (
                Box::new(AmuletTemplate::AmuletOfCharisma),
                "amulet of charisma",
            ),
            (
                Box::new(AmuletTemplate::AmuletOfSearching),
                "amulet of searching",
            ),
            (
                Box::new(AmuletTemplate::AmuletOfTeleportation),
                "amulet of teleportation",
            ),
            (
                Box::new(AmuletTemplate::AmuletOfSlowDigestion),
                "amulet of slow digestion",
            ),
            (
                Box::new(AmuletTemplate::AmuletOfResistAcid),
                "amulet of resist acid",
            ),
            (
                Box::new(AmuletTemplate::AmuletOfTheMagi),
                "amulet of the magi",
            ),
            (Box::new(AmuletTemplate::AmuletOfDoom), "amulet of doom"),
            (Box::new(AmuletTemplate::SilverNecklace), "silver necklace"),
            (Box::new(AmuletTemplate::GoldNecklace), "gold necklace"),
            (
                Box::new(AmuletTemplate::MithrilNecklace),
                "mithril necklace",
            ),
        ] {
            let item = generate_item::generate(template.clone(), 0, ItemQuality::Normal);
            identification::set_identified(template.subtype(), true);
            assert_eq!(generate(&item), expected_name);
        }
    }

    #[test]
    #[serial]
    fn test_names_single_identified() {
        for (template, expected_name) in [
            (
                Box::new(AmuletTemplate::AmuletOfAdornment1),
                "amulet of adornment",
            ),
            (
                Box::new(AmuletTemplate::AmuletOfAdornment2),
                "amulet of adornment",
            ),
            (
                Box::new(AmuletTemplate::AmuletOfWisdom),
                "amulet of wisdom (+1)",
            ),
            (
                Box::new(AmuletTemplate::AmuletOfCharisma),
                "amulet of charisma (+1)",
            ),
            (
                Box::new(AmuletTemplate::AmuletOfSearching),
                "amulet of searching (+1)",
            ),
            (
                Box::new(AmuletTemplate::AmuletOfTeleportation),
                "amulet of teleportation",
            ),
            (
                Box::new(AmuletTemplate::AmuletOfSlowDigestion),
                "amulet of slow digestion",
            ),
            (
                Box::new(AmuletTemplate::AmuletOfResistAcid),
                "amulet of resist acid",
            ),
            (
                Box::new(AmuletTemplate::AmuletOfTheMagi),
                "amulet of the magi",
            ),
            (Box::new(AmuletTemplate::AmuletOfDoom), "amulet of doom"),
            (Box::new(AmuletTemplate::SilverNecklace), "silver necklace"),
            (Box::new(AmuletTemplate::GoldNecklace), "gold necklace"),
            (
                Box::new(AmuletTemplate::MithrilNecklace),
                "mithril necklace",
            ),
        ] {
            let mut item = generate_item::generate(template.clone(), 0, ItemQuality::Normal);
            item.p1 = 1;
            item.set_identified(true);
            assert_eq!(generate(&item), expected_name);
        }
    }
}
