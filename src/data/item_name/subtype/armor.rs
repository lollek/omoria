use crate::conversion::item_subtype;
use crate::data::item_name::helpers::{maybe_armor_bonus, maybe_number_of, maybe_p1_bonus, maybe_special_attribute, to_hit_if_exists};
use crate::model::item_subtype::{HardArmorSubType, ItemSubType, SoftArmorSubType};
use crate::model::Item;
use std::borrow::Cow;

pub fn armor(item: &Item) -> String {
    let item_type = item.item_type().expect("Item has no type");
    let Some(subtype) = item_subtype::from_i64(item_type, item.subval) else {
        return "alien thing".to_string();
    };

    let mut parts = Vec::new();
    // Quantity prefix (e.g. "2 ", "no more ") should be consistent across item types.
    if let Some(number_of_string) = maybe_number_of(item) {
        parts.push(number_of_string);
    }

    let mut name = subtype_name(item, subtype);
    if item.number != 1 {
        // Keep this intentionally simple; if we need irregular plurals later, we'll
        // add a helper and tests.
        name.push('s');
    }
    parts.push(Cow::from(name));

    if let Some(to_hit) = to_hit_if_exists(item) {
        parts.push(to_hit);
    }
    if let Some(armor_string) = maybe_armor_bonus(item) {
        parts.push(armor_string);
    }
    if let Some(p1_bonus_string) = maybe_p1_bonus(item) {
        parts.push(p1_bonus_string);
    }
    if let Some(special_string) = maybe_special_attribute(item) {
        parts.push(special_string);
    }

    parts.join("")
}

fn subtype_name(item: &Item, subtype: ItemSubType) -> String {
    match subtype {
        ItemSubType::HardArmor(armor_type) => match armor_type {
            HardArmorSubType::AugmentedChainMail => "augmented chain mail",
            HardArmorSubType::BarChainMail => "bar chain mail",
            HardArmorSubType::BronzePlateMail => "bronze plate mail",
            HardArmorSubType::ChainMail => "chain mail",
            HardArmorSubType::DoubleChainMail => "double chain mail",
            HardArmorSubType::FullPlateArmor => "full plate armor",
            HardArmorSubType::LacqueredPlate => "lacquered plate",
            HardArmorSubType::LaminatedArmor => "laminated armor",
            HardArmorSubType::MetalBrigandineArmor => "metal brigandine armor",
            HardArmorSubType::MetalLamellarArmor => "metal lamellar armor",
            HardArmorSubType::MetalScaleMail => "metal scale mail",
            HardArmorSubType::MithrilChainMail => "mithril chain mail",
            HardArmorSubType::MithrilPlateArmor => "mithril plate armor",
            HardArmorSubType::PartialPlateArmor => "partial plate armor",
            HardArmorSubType::RustyChainMail => "rusty chain mail",
            HardArmorSubType::StonePlateArmor => "stone plate armor",
        },
        ItemSubType::SoftArmor(armor_type) => match armor_type {
            SoftArmorSubType::CoolSetOfThreads => "cool set of threads",
            SoftArmorSubType::DemonhideArmor => "demonhide armor",
            SoftArmorSubType::DuskShroud => "dusk shroud",
            SoftArmorSubType::ElvenChainMail => "elven chain mail",
            SoftArmorSubType::FilthyNagaHideArmor => "filthy naga hide armor",
            SoftArmorSubType::FilthyRags => "filthy rags",
            SoftArmorSubType::HardLeatherArmor => "hard leather armor",
            SoftArmorSubType::HardLeatherRingMail => "hard leather ring mail",
            SoftArmorSubType::HardStuddedLeather => "hard studded leather",
            SoftArmorSubType::LeatherScaleMail => "leather scale mail",
            SoftArmorSubType::Robe => "robe",
            SoftArmorSubType::SoftLeatherArmor => "soft leather armor",
            SoftArmorSubType::SoftLeatherRingMail => "soft leather ring mail",
            SoftArmorSubType::SoftStuddedLeather => "soft studded armor",
            SoftArmorSubType::WovenCordArmor => "woven cord armor",
            SoftArmorSubType::WyrmhideArmor => "wyrmhide armor",
            SoftArmorSubType::LeatherBrigantineArmor => "leather brigantine armor",
        },
        _ => panic!("coding error, unexpected item type: {:?}", item.item_type()),
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use crate::data::item_name::generate;
    use crate::generate_item::template::ArmorTemplate;
    use crate::generate_item::{ItemQuality, ItemTemplate};
    use crate::model::item_subtype::ItemSubType;
    use crate::model::Item;
    use crate::{generate_item, identification};
    use serial_test::serial;
    use std::fmt::Display;

    fn assert_special_attribute<S1, S2>(
        mut item: Item,
        item_subtype: ItemSubType,
        base_name: S1,
        suffix: S2,
    ) where
        S1: AsRef<str> + Display,
        S2: AsRef<str> + Display,
    {
        item.set_identified(false);
        identification::set_identified(item_subtype, false);
        assert_eq!(generate(&item), format!("{base_name} (+1) [1]"));

        identification::set_identified(item_subtype, true);
        assert_eq!(generate(&item), format!("{base_name} (+1) [1]"));

        item.set_identified(true);
        assert_eq!(
            generate(&item),
            format!("{base_name} (+1) [1,+1] ({suffix})")
        );
    }

    #[test]
    #[serial]
    fn test_special_attribute_resist_acid() {
        let template = ArmorTemplate::AugmentedChainMail;
        let subtype = template.subtype();
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        ArmorTemplate::apply_armor_resist_acid(&template, &mut item);
        item.ac = 1;
        item.toac = 1;
        item.tohit = 1;
        assert_special_attribute(item, subtype, "augmented chain mail", "RA")
    }

    #[test]
    #[serial]
    fn test_special_attribute_resist_fire() {
        let template = ArmorTemplate::AugmentedChainMail;
        let subtype = template.subtype();
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        ArmorTemplate::apply_armor_resist_fire(&template, &mut item);
        item.ac = 1;
        item.toac = 1;
        item.tohit = 1;
        assert_special_attribute(item, subtype, "augmented chain mail", "RF")
    }

    #[test]
    #[serial]
    fn test_special_attribute_resist_cold() {
        let template = ArmorTemplate::AugmentedChainMail;
        let subtype = template.subtype();
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        ArmorTemplate::apply_armor_resist_cold(&template, &mut item);
        item.ac = 1;
        item.toac = 1;
        item.tohit = 1;
        assert_special_attribute(item, subtype, "augmented chain mail", "RC")
    }

    #[test]
    #[serial]
    fn test_special_attribute_resist_lightning() {
        let template = ArmorTemplate::AugmentedChainMail;
        let subtype = template.subtype();
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        ArmorTemplate::apply_armor_resist_lightning(&template, &mut item);
        item.ac = 1;
        item.toac = 1;
        item.tohit = 1;
        assert_special_attribute(item, subtype, "augmented chain mail", "RL")
    }

    #[test]
    #[serial]
    fn test_special_attribute_resist() {
        let template = ArmorTemplate::AugmentedChainMail;
        let subtype = template.subtype();
        let mut item = generate_item::generate(Box::new(template), 0, ItemQuality::Normal);
        ArmorTemplate::apply_armor_resist(&template, &mut item);
        item.ac = 1;
        item.toac = 1;
        item.tohit = 1;
        assert_special_attribute(item, subtype, "augmented chain mail", "R")
    }

    #[test]
    #[serial]
    fn test_unidentified() {
        let tests: Vec<(Box<dyn ItemTemplate>, &str)> = vec![
            (
                Box::new(ArmorTemplate::AugmentedChainMail),
                "augmented chain mail",
            ),
            (Box::new(ArmorTemplate::BarChainMail), "bar chain mail"),
            (
                Box::new(ArmorTemplate::BronzePlateMail),
                "bronze plate mail",
            ),
            (Box::new(ArmorTemplate::ChainMail), "chain mail"),
            (
                Box::new(ArmorTemplate::CoolSetOfThreads),
                "cool set of threads",
            ),
            (Box::new(ArmorTemplate::DemonhideArmor), "demonhide armor"),
            (
                Box::new(ArmorTemplate::DoubleChainMail),
                "double chain mail",
            ),
            (Box::new(ArmorTemplate::DuskShroud), "dusk shroud"),
            (Box::new(ArmorTemplate::ElvenChainMail), "elven chain mail"),
            (
                Box::new(ArmorTemplate::FilthyNagaHideArmor),
                "filthy naga hide armor",
            ),
            (Box::new(ArmorTemplate::FilthyRags), "filthy rags"),
            (Box::new(ArmorTemplate::FullPlateArmor), "full plate armor"),
            (
                Box::new(ArmorTemplate::HardLeatherArmor),
                "hard leather armor",
            ),
            (
                Box::new(ArmorTemplate::HardLeatherRingMail),
                "hard leather ring mail",
            ),
            (
                Box::new(ArmorTemplate::HardStuddedLeather),
                "hard studded leather",
            ),
            (Box::new(ArmorTemplate::LacqueredPlate), "lacquered plate"),
            (Box::new(ArmorTemplate::LaminatedArmor), "laminated armor"),
            (
                Box::new(ArmorTemplate::LeatherScaleMail),
                "leather scale mail",
            ),
            (
                Box::new(ArmorTemplate::MetalBrigandineArmor),
                "metal brigandine armor",
            ),
            (
                Box::new(ArmorTemplate::MetalLamellarArmor),
                "metal lamellar armor",
            ),
            (Box::new(ArmorTemplate::MetalScaleMail), "metal scale mail"),
            (
                Box::new(ArmorTemplate::MithrilChainMail),
                "mithril chain mail",
            ),
            (
                Box::new(ArmorTemplate::MithrilPlateArmor),
                "mithril plate armor",
            ),
            (
                Box::new(ArmorTemplate::PartialPlateArmor),
                "partial plate armor",
            ),
            (Box::new(ArmorTemplate::Robe), "robe"),
            (Box::new(ArmorTemplate::RustyChainMail), "rusty chain mail"),
            (
                Box::new(ArmorTemplate::SoftLeatherArmor),
                "soft leather armor",
            ),
            (
                Box::new(ArmorTemplate::SoftLeatherRingMail),
                "soft leather ring mail",
            ),
            (
                Box::new(ArmorTemplate::SoftStuddedLeather),
                "soft studded armor",
            ),
            (
                Box::new(ArmorTemplate::StonePlateArmor),
                "stone plate armor",
            ),
            (Box::new(ArmorTemplate::WovenCordArmor), "woven cord armor"),
            (Box::new(ArmorTemplate::WyrmhideArmor), "wyrmhide armor"),
            (
                Box::new(ArmorTemplate::LeatherBrigantineArmor),
                "leather brigantine armor",
            ),
        ];
        for (template, expected_name) in tests {
            let subtype = template.subtype();
            let mut item = generate_item::generate(template, 0, ItemQuality::Normal);
            item.ac = 1;
            item.toac = 1;
            item.tohit = 1;
            item.set_identified(false);
            identification::set_identified(subtype, false);
            assert_eq!(generate(&item), format!("{expected_name} (+1) [1]"));
        }
    }

    #[test]
    #[serial]
    fn test_unidentified_to_hit() {
        let tests: Vec<(i16, &str)> =
            vec![(1, "robe (+1) [1]"), (0, "robe [1]"), (-1, "robe (-1) [1]")];
        for (tohit, expected_name) in tests {
            let template = Box::new(ArmorTemplate::Robe);
            let subtype = template.subtype();
            let mut item = generate_item::generate(template, 0, ItemQuality::Normal);
            item.ac = 1;
            item.toac = 1;
            item.tohit = tohit;
            item.set_identified(false);
            identification::set_identified(subtype, false);
            assert_eq!(generate(&item), expected_name);
        }
    }

    #[test]
    #[serial]
    fn test_known() {
        let tests: Vec<(Box<dyn ItemTemplate>, &str)> = vec![(
            Box::new(ArmorTemplate::AugmentedChainMail),
            "augmented chain mail (-2) [1]",
        )];
        for (template, expected_name) in tests {
            let subtype = template.subtype();
            let mut item = generate_item::generate(template, 0, ItemQuality::Normal);
            item.ac = 1;
            item.toac = 1;
            item.set_identified(false);
            identification::set_identified(subtype, true);
            assert_eq!(generate(&item), expected_name);
        }
    }

    #[test]
    #[serial]
    fn test_identified() {
        let tests: Vec<(Box<dyn ItemTemplate>, i16, i16, &str)> = vec![
            (
                Box::new(ArmorTemplate::AugmentedChainMail),
                1,
                1,
                "augmented chain mail (-2) [1,+1]",
            ),
            (
                Box::new(ArmorTemplate::AugmentedChainMail),
                1,
                0,
                "augmented chain mail (-2) [1,0]",
            ),
            (
                Box::new(ArmorTemplate::AugmentedChainMail),
                1,
                -1,
                "augmented chain mail (-2) [1,-1]",
            ),
            (
                Box::new(ArmorTemplate::AugmentedChainMail),
                1,
                1,
                "augmented chain mail (-2) [1,+1]",
            ),
            (
                Box::new(ArmorTemplate::AugmentedChainMail),
                1,
                1,
                "augmented chain mail (-2) [1,+1]",
            ),
        ];
        for (template, base_ac, bonus_ac, expected_name) in tests {
            let mut item = generate_item::generate(template, 0, ItemQuality::Normal);
            item.ac = base_ac;
            item.toac = bonus_ac;
            item.set_identified(true);
            assert_eq!(generate(&item), expected_name);
        }
    }

    #[test]
    #[serial]
    fn test_unidentified_quantity_prefixes() {
        let template = Box::new(ArmorTemplate::Robe);
        let subtype = template.subtype();

        let mut item = generate_item::generate(template, 0, ItemQuality::Normal);
        item.ac = 1;
        item.toac = 0;
        item.tohit = 0;
        item.set_identified(false);
        identification::set_identified(subtype, false);

        item.number = 2;
        assert_eq!(generate(&item), "2 robes [1]");

        item.number = 0;
        assert_eq!(generate(&item), "no more robes [1]");
    }
}
