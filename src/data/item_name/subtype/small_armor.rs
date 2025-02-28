use crate::conversion::item_subtype;
use crate::data::item_name::helpers::{maybe_armor_bonus, maybe_p1_bonus};
use crate::model::item_subtype::{
    BeltSubType, BootsSubType, CloakSubType, GlovesSubType, HelmSubType, ItemSubType,
};
use crate::model::Item;
use std::borrow::Cow;

pub fn small_armor(item: &Item) -> String {
    let mut parts = Vec::new();
    parts.push(Cow::from(subtype_name(item)));
    if let Some(armor_string) = maybe_armor_bonus(item) {
        parts.push(armor_string);
    }
    if let Some(p1_bonus_string) = maybe_p1_bonus(item) {
        parts.push(p1_bonus_string);
    }
    parts.join("")
}

fn subtype_name(item: &Item) -> String {
    let Some(subtype) = item_subtype::from_i64(item.item_type(), item.subval) else {
        return "alien cloak".to_string();
    };

    match subtype {
        ItemSubType::Belt(belt_type) => match belt_type {
            BeltSubType::Sash => "sash",
            BeltSubType::LightBelt => "light belt",
            BeltSubType::Belt => "belt",
            BeltSubType::HeavyBelt => "heavy belt",
            BeltSubType::LightPlatedBelt => "light plated belt",
            BeltSubType::SharkskinBelt => "sharkskin belt",
            BeltSubType::DemonhideBelt => "demonhide belt",
            BeltSubType::WyrmhideBelt => "wyrmhide belt",
        },
        ItemSubType::Boots(boots_type) => match boots_type {
            BootsSubType::SoftLeatherShoes => "soft leather shoes",
            BootsSubType::SoftLeatherBoots => "soft leather boots",
            BootsSubType::HardLeatherBoots => "hard leather boots",
            BootsSubType::Sandals => "sandals",
            BootsSubType::ChainBoots => "chain boots",
            BootsSubType::LightPlatedBoots => "light plated boots",
            BootsSubType::SharkskinBoots => "sharkskin boots",
            BootsSubType::DemonhideBoots => "demonhide boots",
            BootsSubType::WyrmhideBoot => "wyrmhide boots",
        },
        ItemSubType::Cloak(cloak_type) => match cloak_type {
            CloakSubType::LightCloak => "light cloak",
            CloakSubType::HeavyCloak => "heavy cloak",
            CloakSubType::SharkskinCloak => "sharkskin cloak",
            CloakSubType::DemonhideCloak => "demonhide cloak",
            CloakSubType::WyrmhideCloak => "wyrmhide cloak",
        },
        ItemSubType::Gloves(gloves_type) => match gloves_type {
            GlovesSubType::LeatherGloves => "leather gloves",
            GlovesSubType::HeavyGloves => "heavy gloves",
            GlovesSubType::ClothGloves => "cloth gloves",
            GlovesSubType::ChainGloves => "chain gloves",
            GlovesSubType::LightGauntlets => "light gauntlets",
            GlovesSubType::HeavyGauntlets => "heavy gauntlets",
            GlovesSubType::SharkskinGloves => "sharkskin gloves",
            GlovesSubType::WarGauntlets => "war gauntlets",
            GlovesSubType::DemonhideGloves => "demonhide gloves",
            GlovesSubType::WyrmhideGloves => "wyrmhide gloves",
        },
        ItemSubType::Helm(helm_type) => match helm_type {
            HelmSubType::ClothHat => "cloth hat",
            HelmSubType::SoftLeatherCap => "soft leather cap",
            HelmSubType::HardLeatherCap => "hard leather cap",
            HelmSubType::MetalCap => "metal cap",
            HelmSubType::FullHelm => "full helm",
            HelmSubType::GreatHelm => "great helm",
            HelmSubType::WingedHelm => "winged helm",
            HelmSubType::SilverCrown => "silver crown",
            HelmSubType::SilverMask => "silver mask",
            HelmSubType::GoldenCrown => "golden crown",
            HelmSubType::GoldenMask => "golden mask",
            HelmSubType::JewelEncrustedCrown => "jewel encrusted crown",
        },
        _ => panic!("coding error, unexpected item type: {:?}", item.item_type()),
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use crate::data::item_name::generate;
    use crate::generate_item::template::{
        BeltTemplate, BootsTemplate, CloakTemplate, GlovesTemplate, HelmTemplate,
    };
    use crate::generate_item::ItemTemplate;
    use crate::{generate_item, identification};
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_unidentified() {
        let tests: Vec<(Box<dyn ItemTemplate>, &str)> = vec![
            (Box::new(BeltTemplate::Sash), "sash [1]"),
            (Box::new(BeltTemplate::LightBelt), "light belt [1]"),
            (Box::new(BeltTemplate::Belt), "belt [1]"),
            (Box::new(BeltTemplate::HeavyBelt), "heavy belt [1]"),
            (
                Box::new(BeltTemplate::LightPlatedBelt),
                "light plated belt [1]",
            ),
            (Box::new(BeltTemplate::SharkskinBelt), "sharkskin belt [1]"),
            (Box::new(BeltTemplate::DemonhideBelt), "demonhide belt [1]"),
            (Box::new(BeltTemplate::WyrmhideBelt), "wyrmhide belt [1]"),
            (
                Box::new(BootsTemplate::SoftLeatherShoes),
                "soft leather shoes [1]",
            ),
            (
                Box::new(BootsTemplate::SoftLeatherBoots),
                "soft leather boots [1]",
            ),
            (
                Box::new(BootsTemplate::HardLeatherBoots),
                "hard leather boots [1]",
            ),
            (Box::new(BootsTemplate::Sandals), "sandals [1]"),
            (Box::new(BootsTemplate::ChainBoots), "chain boots [1]"),
            (
                Box::new(BootsTemplate::LightPlatedBoots),
                "light plated boots [1]",
            ),
            (
                Box::new(BootsTemplate::SharkskinBoots),
                "sharkskin boots [1]",
            ),
            (
                Box::new(BootsTemplate::DemonhideBoots),
                "demonhide boots [1]",
            ),
            (Box::new(BootsTemplate::WyrmhideBoot), "wyrmhide boots [1]"),
            (Box::new(CloakTemplate::LightCloak), "light cloak [1]"),
            (Box::new(CloakTemplate::HeavyCloak), "heavy cloak [1]"),
            (
                Box::new(CloakTemplate::SharkskinCloak),
                "sharkskin cloak [1]",
            ),
            (
                Box::new(CloakTemplate::DemonhideCloak),
                "demonhide cloak [1]",
            ),
            (Box::new(CloakTemplate::WyrmhideCloak), "wyrmhide cloak [1]"),
            (
                Box::new(GlovesTemplate::LeatherGloves),
                "leather gloves [1]",
            ),
            (Box::new(GlovesTemplate::HeavyGloves), "heavy gloves [1]"),
            (Box::new(GlovesTemplate::ClothGloves), "cloth gloves [1]"),
            (Box::new(GlovesTemplate::ChainGloves), "chain gloves [1]"),
            (
                Box::new(GlovesTemplate::LightGauntlets),
                "light gauntlets [1]",
            ),
            (
                Box::new(GlovesTemplate::HeavyGauntlets),
                "heavy gauntlets [1]",
            ),
            (
                Box::new(GlovesTemplate::SharkskinGloves),
                "sharkskin gloves [1]",
            ),
            (Box::new(GlovesTemplate::WarGauntlets), "war gauntlets [1]"),
            (
                Box::new(GlovesTemplate::DemonhideGloves),
                "demonhide gloves [1]",
            ),
            (
                Box::new(GlovesTemplate::WyrmhideGloves),
                "wyrmhide gloves [1]",
            ),
            (Box::new(HelmTemplate::ClothHat), "cloth hat [1]"),
            (
                Box::new(HelmTemplate::SoftLeatherCap),
                "soft leather cap [1]",
            ),
            (
                Box::new(HelmTemplate::HardLeatherCap),
                "hard leather cap [1]",
            ),
            (Box::new(HelmTemplate::MetalCap), "metal cap [1]"),
            (Box::new(HelmTemplate::FullHelm), "full helm [1]"),
            (Box::new(HelmTemplate::GreatHelm), "great helm [1]"),
            (Box::new(HelmTemplate::WingedHelm), "winged helm [1]"),
            (Box::new(HelmTemplate::SilverCrown), "silver crown [1]"),
            (Box::new(HelmTemplate::SilverMask), "silver mask [1]"),
            (Box::new(HelmTemplate::GoldenCrown), "golden crown [1]"),
            (Box::new(HelmTemplate::GoldenMask), "golden mask [1]"),
            (
                Box::new(HelmTemplate::JewelEncrustedCrown),
                "jewel encrusted crown [1]",
            ),
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
            (
                Box::new(BootsTemplate::SoftLeatherShoes),
                1,
                "soft leather shoes [1]",
            ),
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
        let tests: Vec<(Box<dyn ItemTemplate>, i16, i64, &str)> = vec![
            (Box::new(BeltTemplate::LightBelt), 1, 0, "light belt [1,+1]"),
            (
                Box::new(BootsTemplate::SoftLeatherShoes),
                1,
                0,
                "soft leather shoes [1,+1]",
            ),
            (
                Box::new(CloakTemplate::LightCloak),
                1,
                0,
                "light cloak [1,+1]",
            ),
            (Box::new(CloakTemplate::HeavyCloak), 0, 0, "heavy cloak"),
            (
                Box::new(CloakTemplate::SharkskinCloak),
                -1,
                0,
                "sharkskin cloak [-1,-1]",
            ),
            (
                Box::new(CloakTemplate::DemonhideCloak),
                10,
                0,
                "demonhide cloak [10,+10]",
            ),
            (
                Box::new(HelmTemplate::SilverCrown),
                1,
                1,
                "silver crown [1,+1] (+1)",
            ),
            (
                Box::new(HelmTemplate::SilverMask),
                -1,
                -1,
                "silver mask [-1,-1] (-1)",
            ),
            (
                Box::new(HelmTemplate::GoldenCrown),
                2,
                2,
                "golden crown [2,+2] (+2)",
            ),
            (
                Box::new(HelmTemplate::GoldenMask),
                -2,
                -2,
                "golden mask [-2,-2] (-2)",
            ),
            (
                Box::new(HelmTemplate::JewelEncrustedCrown),
                10,
                10,
                "jewel encrusted crown [10,+10] (+10)",
            ),
        ];
        for (template, ac_bonus, p1, expected_name) in tests {
            let mut item = generate_item::generate(template, 0);
            item.ac = ac_bonus;
            item.toac = ac_bonus;
            item.p1 = p1;
            item.set_identified(true);
            assert_eq!(generate(&item), expected_name);
        }
    }
}
