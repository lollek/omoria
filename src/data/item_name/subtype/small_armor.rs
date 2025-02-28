use std::borrow::Cow;
use crate::conversion::item_subtype;
use crate::data::item_name::helpers::armor_bonus;
use crate::model::item_subtype::{BeltSubType, BootsSubType, CloakSubType, ItemSubType};
use crate::model::Item;

pub fn small_armor(item: &Item) -> String {
    let mut parts = Vec::new();
    parts.push(Cow::from(subtype_name(item)));
    parts.push(armor_bonus(item));
    parts.join("")
}

fn subtype_name(item: &Item) -> String {
    let Some(subtype) = item_subtype::from_i64(item.item_type(), item.subval)
    else {
        return "alien cloak".to_string();
    };

    match subtype {
        ItemSubType::Belt(belt_type) => {
            match belt_type {
                BeltSubType::Sash => "sash",
                BeltSubType::LightBelt => "light belt",
                BeltSubType::Belt => "belt",
                BeltSubType::HeavyBelt => "heavy belt",
                BeltSubType::LightPlatedBelt => "light plated belt",
                BeltSubType::SharkskinBelt => "sharkskin belt",
                BeltSubType::DemonhideBelt => "demonhide belt",
                BeltSubType::WyrmhideBelt => "wyrmhide belt",
            }
        },
        ItemSubType::Boots(boots_type) => {
            match boots_type {
                BootsSubType::SoftLeatherShoes => "soft leather shoes",
                BootsSubType::SoftLeatherBoots => "soft leather boots",
                BootsSubType::HardLeatherBoots => "hard leather boots",
                BootsSubType::Sandals => "sandals",
                BootsSubType::ChainBoots => "chain boots",
                BootsSubType::LightPlatedBoots => "light plated boots",
                BootsSubType::SharkskinBoots => "sharkskin boots",
                BootsSubType::DemonhideBoots => "demonhide boots",
                BootsSubType::WyrmhideBoot => "wyrmhide boots",
            }
        },
        ItemSubType::Cloak(cloak_type) => {
            match cloak_type {
                CloakSubType::LightCloak => "light cloak",
                CloakSubType::HeavyCloak => "heavy cloak",
                CloakSubType::SharkskinCloak => "sharkskin cloak",
                CloakSubType::DemonhideCloak => "demonhide cloak",
                CloakSubType::WyrmhideCloak => "wyrmhide cloak",
            }
        },
        _ => panic!("coding error, unexpected item type: {:?}", item.item_type())
    }.to_string()
}

#[cfg(test)]
mod tests {
    use crate::data::item_name::generate;
    use crate::generate_item::template::{BeltTemplate, BootsTemplate, CloakTemplate};
    use crate::generate_item::ItemTemplate;
    use crate::{generate_item, identification};
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_unidentified() {
        let tests : Vec<(Box<dyn ItemTemplate>, &str)> = vec![
            (Box::new(BeltTemplate::Sash), "sash [1]"),
            (Box::new(BeltTemplate::LightBelt), "light belt [1]"),
            (Box::new(BeltTemplate::Belt), "belt [1]"),
            (Box::new(BeltTemplate::HeavyBelt), "heavy belt [1]"),
            (Box::new(BeltTemplate::LightPlatedBelt), "light plated belt [1]"),
            (Box::new(BeltTemplate::SharkskinBelt), "sharkskin belt [1]"),
            (Box::new(BeltTemplate::DemonhideBelt), "demonhide belt [1]"),
            (Box::new(BeltTemplate::WyrmhideBelt), "wyrmhide belt [1]"),
            (Box::new(BootsTemplate::SoftLeatherShoes), "soft leather shoes [1]"),
            (Box::new(BootsTemplate::SoftLeatherBoots), "soft leather boots [1]"),
            (Box::new(BootsTemplate::HardLeatherBoots), "hard leather boots [1]"),
            (Box::new(BootsTemplate::Sandals), "sandals [1]"),
            (Box::new(BootsTemplate::ChainBoots), "chain boots [1]"),
            (Box::new(BootsTemplate::LightPlatedBoots), "light plated boots [1]"),
            (Box::new(BootsTemplate::SharkskinBoots), "sharkskin boots [1]"),
            (Box::new(BootsTemplate::DemonhideBoots), "demonhide boots [1]"),
            (Box::new(BootsTemplate::WyrmhideBoot), "wyrmhide boots [1]"),
            (Box::new(CloakTemplate::LightCloak), "light cloak [1]"),
            (Box::new(CloakTemplate::HeavyCloak), "heavy cloak [1]"),
            (Box::new(CloakTemplate::SharkskinCloak), "sharkskin cloak [1]"),
            (Box::new(CloakTemplate::DemonhideCloak), "demonhide cloak [1]"),
            (Box::new(CloakTemplate::WyrmhideCloak), "wyrmhide cloak [1]"),
        ];
        for (template, expected_name) in tests {
            let subtype = template.subtype();
            let mut item = generate_item::generate(template, 0);
            item.ac = 1;
            item.toac = 1;
            identification::set_identified(subtype, false);
            assert_eq!(generate(&item), expected_name);
        }
    }

    #[test]
    #[serial]
    fn test_known_type() {
        let tests: Vec<(Box<dyn ItemTemplate>, i16, &str)> = vec![
            (Box::new(BeltTemplate::LightBelt), 1, "light belt [1]"),
            (Box::new(BootsTemplate::SoftLeatherShoes), 1, "soft leather shoes [1]"),
            (Box::new(CloakTemplate::LightCloak), 1, "light cloak [1]"),
            (Box::new(CloakTemplate::HeavyCloak), 0, "heavy cloak"),
            (
                Box::new(CloakTemplate::SharkskinCloak),
                -1,
                "sharkskin cloak [-1]",
            ),
            (
                Box::new(CloakTemplate::DemonhideCloak),
                10,
                "demonhide cloak [10]",
            ),
        ];
        for (template, bonus, expected_name) in tests {
            let subtype = template.subtype();
            let mut item = generate_item::generate(template, 0);
            item.ac = bonus;
            item.toac = bonus;
            identification::set_identified(subtype, true);
            assert_eq!(generate(&item), expected_name);
        }
    }

    #[test]
    #[serial]
    fn test_identified() {
        let tests: Vec<(Box<dyn ItemTemplate>, i16, &str)> = vec![
            (Box::new(BeltTemplate::LightBelt), 1, "light belt [1,+1]"),
            (Box::new(BootsTemplate::SoftLeatherShoes), 1, "soft leather shoes [1,+1]"),
            (Box::new(CloakTemplate::LightCloak), 1, "light cloak [1,+1]"),
            (Box::new(CloakTemplate::HeavyCloak), 0, "heavy cloak"),
            (
                Box::new(CloakTemplate::SharkskinCloak),
                -1,
                "sharkskin cloak [-1,-1]",
            ),
            (
                Box::new(CloakTemplate::DemonhideCloak),
                10,
                "demonhide cloak [10,+10]",
            ),
        ];
        for (template, bonus, expected_name) in tests {
            let mut item = generate_item::generate(template, 0);
            item.ac = bonus;
            item.toac = bonus;
            item.set_identified(true);
            assert_eq!(generate(&item), expected_name);
        }
    }
}
