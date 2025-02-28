use crate::conversion::item_subtype;
use crate::data::item_name::helpers::armor_bonus;
use crate::identification::is_identified;
use crate::model::item_subtype::{BeltSubType, BootsSubType, CloakSubType, ItemSubType};
use crate::model::Item;

pub fn small_armor(item: &Item) -> String {
    let Some(subtype) = item_subtype::from_i64(item.item_type(), item.subval)
    else {
        return "alien cloak".to_string();
    };

    let name = match subtype {
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
    };

    if is_identified(subtype) || item.is_identified() {
        return format!("{}{}", name, armor_bonus(item));
    }
    name.to_string()
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
            (Box::new(BeltTemplate::Sash), "sash"),
            (Box::new(BeltTemplate::LightBelt), "light belt"),
            (Box::new(BeltTemplate::Belt), "belt"),
            (Box::new(BeltTemplate::HeavyBelt), "heavy belt"),
            (Box::new(BeltTemplate::LightPlatedBelt), "light plated belt"),
            (Box::new(BeltTemplate::SharkskinBelt), "sharkskin belt"),
            (Box::new(BeltTemplate::DemonhideBelt), "demonhide belt"),
            (Box::new(BeltTemplate::WyrmhideBelt), "wyrmhide belt"),
            (Box::new(BootsTemplate::SoftLeatherShoes), "soft leather shoes"),
            (Box::new(BootsTemplate::SoftLeatherBoots), "soft leather boots"),
            (Box::new(BootsTemplate::HardLeatherBoots), "hard leather boots"),
            (Box::new(BootsTemplate::Sandals), "sandals"),
            (Box::new(BootsTemplate::ChainBoots), "chain boots"),
            (Box::new(BootsTemplate::LightPlatedBoots), "light plated boots"),
            (Box::new(BootsTemplate::SharkskinBoots), "sharkskin boots"),
            (Box::new(BootsTemplate::DemonhideBoots), "demonhide boots"),
            (Box::new(BootsTemplate::WyrmhideBoot), "wyrmhide boots"),
            (Box::new(CloakTemplate::LightCloak), "light cloak"),
            (Box::new(CloakTemplate::HeavyCloak), "heavy cloak"),
            (Box::new(CloakTemplate::SharkskinCloak), "sharkskin cloak"),
            (Box::new(CloakTemplate::DemonhideCloak), "demonhide cloak"),
            (Box::new(CloakTemplate::WyrmhideCloak), "wyrmhide cloak"),
        ];
        for (template, expected_name) in tests {
            let sub_type = template.subtype();
            let item = generate_item::generate(template, 0);
            identification::set_identified(sub_type, false);
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
