use crate::conversion::item_subtype;
use crate::identification;
use crate::model::{Item, ItemType};

pub fn wand(item: &Item) -> String {
    let subtype = item_subtype::from_i64(ItemType::Wand, item.subval)
        .unwrap_or_else(|| panic!("Subtype for item is not a wand? {:?}", item));

    if !identification::is_identified(subtype) {
        return "unknown wand".to_string();
    }
    let attribute = match item.subval {
        1 => " of Light",
        2 => " of Lightning Bolts",
        3 => " of Frost Bolts",
        4 => " of Fire Bolts",
        5 => " of Stone-to-Mud",
        6 => " of Polymorph",
        7 => " of Heal Monster",
        8 => " of Haste Monster",
        9 => " of Slow Monster",
        10 => " of Confuse Monster",
        11 => " of Sleep Monster",
        12 => " of Drain Life",
        13 => " of Trap/Door destruction",
        14 => " of Magic Missile",
        15 => " of Wall Building",
        16 => " of Clone Monster",
        17 => " of Teleport Away",
        18 => " of Disarming",
        19 => " of Lightning Balls",
        20 => " of Cold Balls",
        21 => " of Fire Balls",
        22 => " of Stinking Cloud",
        23 => " of Acid Balls",
        24 => " of Wonder",
        25 => " of Probing",
        _ => "of ???",
    };

    if !item.is_identified() {
        return format!("wand{attribute}");
    }

    let charges = item.p1;
    format!("wand{attribute} ({charges} charges)")
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use crate::data::item_name::generate;
    use crate::generate_item::template::WandTemplate;
    use crate::model::item_subtype::{ItemSubType, WandSubType};
    use crate::{generate_item, identification};
    use crate::generate_item::ItemQuality;

    #[test]
    #[serial]
    fn test_wand_unidentified() {
        let mut item = generate_item::generate(Box::new(WandTemplate::WandOfCloneMonster), 0, ItemQuality::Normal);

        identification::set_identified(ItemSubType::Wand(WandSubType::WandOfCloneMonster), false);
        item.set_identified(false);
        assert_eq!(generate(&item), "unknown wand");
    }

    #[test]
    #[serial]
    fn test_wand_type_identified() {
        let mut item = generate_item::generate(Box::new(WandTemplate::WandOfCloneMonster), 0, ItemQuality::Normal);

        identification::set_identified(ItemSubType::Wand(WandSubType::WandOfCloneMonster), true);
        item.set_identified(false);
        assert_eq!(generate(&item), "wand of Clone Monster");
    }

    #[test]
    #[serial]
    fn test_wand_identified() {
        let mut item = generate_item::generate(Box::new(WandTemplate::WandOfCloneMonster), 0, ItemQuality::Normal);

        identification::set_identified(ItemSubType::Wand(WandSubType::WandOfCloneMonster), true);
        item.set_identified(true);
        item.p1 = 0;
        assert_eq!(generate(&item), "wand of Clone Monster (0 charges)");
    }
}
