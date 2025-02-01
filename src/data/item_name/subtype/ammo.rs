use std::borrow::Cow;
use crate::{
    data::item_name::generate,
    generate_item::{self, template::AmmunitionTemplate},
};
use crate::conversion::item_subtype::from_i64;
use crate::data::item_name::helpers::{attack_bonus, damage, full_number_of, plural_s};
use crate::model::Item;
use crate::model::item_subtype::{ArrowSubType, BoltSubType, ItemSubType, SlingAmmoSubType};

pub fn ammo(item: &Item) -> String {
    let mut parts = vec![
        full_number_of(item),
        match from_i64(item.item_type(), item.subval) {
            Some(subtype) => match subtype {
                ItemSubType::SlingAmmo(SlingAmmoSubType::RoundedPebble) => {
                    Cow::from("rounded pebble")
                }
                ItemSubType::SlingAmmo(SlingAmmoSubType::IronShot) => Cow::from("iron shot"),
                ItemSubType::Arrow(ArrowSubType::Arrow) => Cow::from("arrow"),
                ItemSubType::Bolt(BoltSubType::Bolt) => Cow::from("bolt"),
                t => panic!("Expected ammo, got {:?}", t),
            },
            None => Cow::from("alien ammo"),
        },
        plural_s(item),
        damage(item),
    ];

    if item.is_identified() {
        parts.push(attack_bonus(item));
    }
    parts.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rounded_pebble() {
        let mut item = generate_item::generate(Box::new(AmmunitionTemplate::RoundedPebble), 0);
        item.tohit = 12;
        item.todam = 24;

        item.number = 0;
        assert_eq!(generate(&item), "no more rounded pebbles (3d2)");

        item.number = 1;
        assert_eq!(generate(&item), "1 rounded pebble (3d2)");

        item.number = 2;
        assert_eq!(generate(&item), "2 rounded pebbles (3d2)");

        item.set_identified(true);
        assert_eq!(generate(&item), "2 rounded pebbles (3d2) (+12,+24)");

        item.tohit = 0;
        item.todam = 0;
        assert_eq!(generate(&item), "2 rounded pebbles (3d2) (0,0)");

        item.tohit = -1;
        item.todam = -2;
        assert_eq!(generate(&item), "2 rounded pebbles (3d2) (-1,-2)");
    }

    #[test]
    fn test_iron_shot() {
        let mut item = generate_item::generate(Box::new(AmmunitionTemplate::IronShot), 0);
        item.tohit = 12;
        item.todam = 24;

        item.number = 0;
        assert_eq!(generate(&item), "no more iron shots (3d3)");

        item.number = 1;
        assert_eq!(generate(&item), "1 iron shot (3d3)");

        item.number = 2;
        assert_eq!(generate(&item), "2 iron shots (3d3)");

        item.set_identified(true);
        assert_eq!(generate(&item), "2 iron shots (3d3) (+12,+24)");

        item.tohit = 0;
        item.todam = 0;
        assert_eq!(generate(&item), "2 iron shots (3d3) (0,0)");

        item.tohit = -1;
        item.todam = -2;
        assert_eq!(generate(&item), "2 iron shots (3d3) (-1,-2)");
    }

    #[test]
    fn test_arrow() {
        let mut item = generate_item::generate(Box::new(AmmunitionTemplate::Arrow), 0);
        item.tohit = 12;
        item.todam = 24;

        item.number = 0;
        assert_eq!(generate(&item), "no more arrows (3d4)");

        item.number = 1;
        assert_eq!(generate(&item), "1 arrow (3d4)");

        item.number = 2;
        assert_eq!(generate(&item), "2 arrows (3d4)");

        item.set_identified(true);
        assert_eq!(generate(&item), "2 arrows (3d4) (+12,+24)");

        item.tohit = 0;
        item.todam = 0;
        assert_eq!(generate(&item), "2 arrows (3d4) (0,0)");

        item.tohit = -1;
        item.todam = -2;
        assert_eq!(generate(&item), "2 arrows (3d4) (-1,-2)");
    }

    #[test]
    fn test_bolt() {
        let mut item = generate_item::generate(Box::new(AmmunitionTemplate::Bolt), 0);
        item.tohit = 12;
        item.todam = 24;

        item.number = 0;
        assert_eq!(generate(&item), "no more bolts (3d5)");

        item.number = 1;
        assert_eq!(generate(&item), "1 bolt (3d5)");

        item.number = 2;
        assert_eq!(generate(&item), "2 bolts (3d5)");

        item.set_identified(true);
        assert_eq!(generate(&item), "2 bolts (3d5) (+12,+24)");

        item.tohit = 0;
        item.todam = 0;
        assert_eq!(generate(&item), "2 bolts (3d5) (0,0)");

        item.tohit = -1;
        item.todam = -2;
        assert_eq!(generate(&item), "2 bolts (3d5) (-1,-2)");
    }
}
