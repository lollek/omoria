use crate::conversion::item_subtype;
use crate::data;
use crate::data::item_name::helpers::{maybe_armor_bonus, attack_bonus, damage, maybe_number_of};
use crate::model::item_subtype::{DaggerSubType, HaftedWeaponSubType, ItemSubType};
use crate::model::Item;
use std::borrow::Cow;

pub fn melee_weapon(item: &Item) -> String {
    let mut parts = Vec::new();
    if let Some(number_of_string) = maybe_number_of(item) {
        parts.push(number_of_string);
    }
    parts.push(Cow::from(subtype_name(item)));
    parts.push(damage(item));
    if data::item_type::has_attack_enhancement(&item.item_type()) && item.is_identified() {
        parts.push(attack_bonus(item));
    }
    if let Some(armor_string) = maybe_armor_bonus(item) {
        parts.push(armor_string);
    }
    parts.join("")
}

fn subtype_name(item: &Item) -> String {
    let Some(subtype) = item_subtype::from_i64(item.item_type(), item.subval) else {
        return "alien weapon".to_string();
    };

    match subtype {
        ItemSubType::Dagger(dagger_type) => {
            match dagger_type {
                DaggerSubType::MainGauche => "main gauche",
                DaggerSubType::Misercorde => "misercorde",
                DaggerSubType::Stiletto => "stiletto",
                DaggerSubType::Bodkin => "bodkin",
                DaggerSubType::BrokenDagger => "broken dagger",
                DaggerSubType::CatONineTails => "cat-o-nine tails",
                DaggerSubType::Bilbo => "bilbo",
                DaggerSubType::Baselard => "baselard",
                DaggerSubType::Foil => "foil",
                DaggerSubType::Rapier => "rapier",
                DaggerSubType::SmallSword => "small sword",
            }
        }
        ItemSubType::HaftedWeapon(axe_type) => {
            match axe_type {
                HaftedWeaponSubType::Balestarius => "balestarius",
                HaftedWeaponSubType::BattleAxe => "battle axe",
                HaftedWeaponSubType::BroadAxe => "broad axe",
                HaftedWeaponSubType::HandAxe => "hand axe",
                HaftedWeaponSubType::WarAxe => "war axe",
                HaftedWeaponSubType::LargeAxe => "large axe",
                HaftedWeaponSubType::BeardedAxe => "bearded axe",
                HaftedWeaponSubType::SilverEdgedAxe => "silver edged axe",
                HaftedWeaponSubType::ChampionAxe => "champion axe",
            }
        }
        _ => "alien weapon",
    }.to_string()
}

#[cfg(test)]
mod tests {
    use crate::data::item_name::generate;
    use crate::generate_item;
    use crate::generate_item::template::{AxeTemplate, DaggerTemplate};

    #[test]
    fn test_generic_attributes() {
        let mut item = generate_item::generate(Box::new(AxeTemplate::Balestarius), 0);

        item.set_identified(false);
        assert_eq!(generate(&item), "balestarius (2d8)");

        item.set_identified(true);
        assert_eq!(generate(&item), "balestarius (2d8) (0,0)");

        item.tohit = 1;
        item.todam = 2;
        assert_eq!(generate(&item), "balestarius (2d8) (+1,+2)");

        item.tohit = -1;
        item.todam = -2;
        assert_eq!(generate(&item), "balestarius (2d8) (-1,-2)");
    }

    #[test]
    fn test_axe_names() {
        assert_eq!(
            generate(&generate_item::generate(
                Box::new(AxeTemplate::Balestarius),
                0
            )),
            "balestarius (2d8)"
        );
        assert_eq!(
            generate(&generate_item::generate(
                Box::new(AxeTemplate::BattleAxe),
                0
            )),
            "battle axe (3d4)"
        );
        assert_eq!(
            generate(&generate_item::generate(Box::new(AxeTemplate::BroadAxe), 0)),
            "broad axe (2d6)"
        );
        assert_eq!(
            generate(&generate_item::generate(Box::new(AxeTemplate::HandAxe), 0)),
            "hand axe (1d4)"
        );
        assert_eq!(
            generate(&generate_item::generate(Box::new(AxeTemplate::WarAxe), 0)),
            "war axe (1d6)"
        );
        assert_eq!(
            generate(&generate_item::generate(Box::new(AxeTemplate::LargeAxe), 0)),
            "large axe (1d9)"
        );
        assert_eq!(
            generate(&generate_item::generate(
                Box::new(AxeTemplate::BeardedAxe),
                0
            )),
            "bearded axe (2d5)"
        );
        assert_eq!(
            generate(&generate_item::generate(
                Box::new(AxeTemplate::SilverEdgedAxe),
                0
            )),
            "silver edged axe (3d6)"
        );
        assert_eq!(
            generate(&generate_item::generate(
                Box::new(AxeTemplate::ChampionAxe),
                0
            )),
            "champion axe (5d3)"
        );
    }

    #[test]
    fn test_dagger_names() {
        assert_eq!(
            generate(&generate_item::generate(
                Box::new(DaggerTemplate::MainGauche),
                0
            )),
            "main gauche (1d5)"
        );
        assert_eq!(
            generate(&generate_item::generate(
                Box::new(DaggerTemplate::Misercorde),
                0
            )),
            "misercorde (1d4)"
        );
        assert_eq!(
            generate(&generate_item::generate(
                Box::new(DaggerTemplate::Stiletto),
                0
            )),
            "stiletto (1d4)"
        );
        assert_eq!(
            generate(&generate_item::generate(
                Box::new(DaggerTemplate::Bodkin),
                0
            )),
            "bodkin (1d4)"
        );
        assert_eq!(
            generate(&generate_item::generate(
                Box::new(DaggerTemplate::BrokenDagger),
                0
            )),
            "broken dagger (1d1)"
        );
        assert_eq!(
            generate(&generate_item::generate(
                Box::new(DaggerTemplate::CatONineTails),
                0
            )),
            "cat-o-nine tails (1d4)"
        );
        assert_eq!(
            generate(&generate_item::generate(Box::new(DaggerTemplate::Bilbo), 0)),
            "bilbo (1d6)"
        );
        assert_eq!(
            generate(&generate_item::generate(
                Box::new(DaggerTemplate::Baselard),
                0
            )),
            "baselard (1d7)"
        );
        assert_eq!(
            generate(&generate_item::generate(Box::new(DaggerTemplate::Foil), 0)),
            "foil (1d5)"
        );
        assert_eq!(
            generate(&generate_item::generate(
                Box::new(DaggerTemplate::Rapier),
                0
            )),
            "rapier (1d6)"
        );
        assert_eq!(
            generate(&generate_item::generate(
                Box::new(DaggerTemplate::SmallSword),
                0
            )),
            "small sword (1d6)"
        );
    }
}
