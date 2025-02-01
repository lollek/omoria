use crate::conversion::item_subtype::hafted_weapon;
use crate::data;
use crate::data::item_name::helpers::{armor_bonus, attack_bonus, damage, number_of};
use crate::model::item_subtype::HaftedWeaponSubType;
use crate::model::{Item, ItemType};
use std::borrow::Cow;

pub fn melee_weapon(item: &Item) -> String {
    let mut parts = Vec::new();
    parts.push(number_of(item));
    parts.push(Cow::from(subtype_name(item)));
    parts.push(damage(item));
    if data::item_type::has_attack_enhancement(&item.item_type()) && item.is_identified() {
        parts.push(attack_bonus(item));
    }
    parts.push(armor_bonus(item));
    parts.join("")
}

fn subtype_name(item: &Item) -> String {
    match item.item_type() {
        ItemType::HaftedWeapon => {
            match hafted_weapon::from_usize(item.subval as usize) {
                None => "Alien hafted weapon",
                Some(subtype) => match subtype {
                    HaftedWeaponSubType::Balestarius => "Balestarius",
                    HaftedWeaponSubType::BattleAxe => "Battle axe",
                    HaftedWeaponSubType::BroadAxe => "Broad axe",
                    HaftedWeaponSubType::HandAxe => "Hand axe",
                    HaftedWeaponSubType::WarAxe => "War axe",
                    HaftedWeaponSubType::LargeAxe => "Large axe",
                    HaftedWeaponSubType::BeardedAxe => "Bearded axe",
                    HaftedWeaponSubType::SilverEdgedAxe => "Silver edged axe",
                    HaftedWeaponSubType::ChampionAxe => "Champion axe",
                },
            }
        }
        .to_string(),
        _ => "Alien weapon".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::data::item_name::generate;
    use crate::generate_item;
    use crate::generate_item::template::AxeTemplate;

    #[test]
    fn test_generic_attributes() {
        let mut item = generate_item::generate(Box::new(AxeTemplate::Balestarius), 0);

        item.set_identified(false);
        assert_eq!(generate(&item), "Balestarius (2d8)");

        item.set_identified(true);
        assert_eq!(generate(&item), "Balestarius (2d8) (0,0)");

        item.tohit = 1;
        item.todam = 2;
        assert_eq!(generate(&item), "Balestarius (2d8) (+1,+2)");

        item.tohit = -1;
        item.todam = -2;
        assert_eq!(generate(&item), "Balestarius (2d8) (-1,-2)");
    }

    #[test]
    fn test_axe_names() {
        assert_eq!(
            generate(&generate_item::generate(
                Box::new(AxeTemplate::Balestarius),
                0
            )),
            "Balestarius (2d8)"
        );
        assert_eq!(
            generate(&generate_item::generate(
                Box::new(AxeTemplate::BattleAxe),
                0
            )),
            "Battle axe (3d4)"
        );
        assert_eq!(
            generate(&generate_item::generate(Box::new(AxeTemplate::BroadAxe), 0)),
            "Broad axe (2d6)"
        );
        assert_eq!(
            generate(&generate_item::generate(Box::new(AxeTemplate::HandAxe), 0)),
            "Hand axe (1d4)"
        );
        assert_eq!(
            generate(&generate_item::generate(Box::new(AxeTemplate::WarAxe), 0)),
            "War axe (1d6)"
        );
        assert_eq!(
            generate(&generate_item::generate(Box::new(AxeTemplate::LargeAxe), 0)),
            "Large axe (1d9)"
        );
        assert_eq!(
            generate(&generate_item::generate(
                Box::new(AxeTemplate::BeardedAxe),
                0
            )),
            "Bearded axe (2d5)"
        );
        assert_eq!(
            generate(&generate_item::generate(
                Box::new(AxeTemplate::SilverEdgedAxe),
                0
            )),
            "Silver edged axe (3d6)"
        );
        assert_eq!(
            generate(&generate_item::generate(
                Box::new(AxeTemplate::ChampionAxe),
                0
            )),
            "Champion axe (5d3)"
        );
    }
}
