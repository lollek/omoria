use crate::conversion::item_subtype;
use crate::data;
use crate::data::item_name::helpers::{attack_bonus, damage, maybe_armor_bonus, maybe_number_of, maybe_p1_bonus};
use crate::model::item_subtype::{DaggerSubType, HaftedWeaponSubType, ItemSubType, PickSubType, SwordSubType};
use crate::model::Item;

pub fn melee_weapon(item: &Item) -> String {
    let Some(subtype) = item_subtype::from_i64(item.item_type(), item.subval) else {
        return "alien weapon".to_string();
    };

    let mut parts = Vec::new();
    if let Some(number_of_string) = maybe_number_of(item) {
        parts.push(number_of_string);
    }
    parts.push(subtype_name(item, subtype).into());
    if let ItemSubType::Pick(_) = subtype {
        if let Some(p1_bonus_string) = maybe_p1_bonus(item) {
            parts.push(p1_bonus_string);
        }
    }
    parts.push(damage(item));
    if data::item_type::has_attack_enhancement(&item.item_type()) && item.is_identified() {
        parts.push(attack_bonus(item));
    }
    if let Some(armor_string) = maybe_armor_bonus(item) {
        parts.push(armor_string);
    }
    parts.join("")
}

fn subtype_name(item: &Item, subtype: ItemSubType) -> String {
    match subtype {
        ItemSubType::Dagger(dagger_type) => match dagger_type {
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
        },
        ItemSubType::HaftedWeapon(axe_type) => match axe_type {
            HaftedWeaponSubType::Balestarius => "balestarius",
            HaftedWeaponSubType::BattleAxe => "battle axe",
            HaftedWeaponSubType::BroadAxe => "broad axe",
            HaftedWeaponSubType::HandAxe => "hand axe",
            HaftedWeaponSubType::WarAxe => "war axe",
            HaftedWeaponSubType::LargeAxe => "large axe",
            HaftedWeaponSubType::BeardedAxe => "bearded axe",
            HaftedWeaponSubType::SilverEdgedAxe => "silver edged axe",
            HaftedWeaponSubType::ChampionAxe => "champion axe",
        },
        ItemSubType::Pick(pick_type) => match pick_type {
            PickSubType::Pick => "pick",
            PickSubType::Shovel => "shovel",
            PickSubType::OrcishPick1 => "orcish pick",
            PickSubType::OrcishPick2 => "orcish pick",
            PickSubType::DwarvenPick => "dwarven pick",
            PickSubType::GnomishShovel => "gnomish shovel",
            PickSubType::DwarvenShovel => "dwarven shovel",
        }
        ItemSubType::Sword(sword_type) => match sword_type {
            SwordSubType::Backsword => "backsword",
            SwordSubType::BastardSword => "bastard sword",
            SwordSubType::Broadsword => "broadsword",
            SwordSubType::Claymore => "claymore",
            SwordSubType::Cutlass => "cutlass",
            SwordSubType::Espadon => "espadon",
            SwordSubType::ExecutionersSword => "executioners sword",
            SwordSubType::Flamberge => "flamberge",
            SwordSubType::Katana => "katana",
            SwordSubType::Longsword => "longsword",
            SwordSubType::Nodachi => "nodachi",
            SwordSubType::Sabre => "sabre",
            SwordSubType::Zweihander => "zweihander",
            SwordSubType::BrokenSword => "broken sword",
        }
        _ => panic!("coding error, unexpected item type: {:?}", item.item_type()),
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use crate::data::item_name::generate;
    use crate::generate_item::template::{AxeTemplate, DaggerTemplate, PickTemplate, SwordTemplate};
    use crate::generate_item::ItemTemplate;
    use crate::{generate_item, identification};
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_identified() {
        let tests: Vec<(Box<dyn ItemTemplate>, i16, i16, &str)> = vec![
            (Box::new(AxeTemplate::Balestarius), 0, 0, "balestarius (2d8) (0,0)"),
            (Box::new(AxeTemplate::Balestarius), 1, 2, "balestarius (2d8) (+1,+2)"),
            (Box::new(AxeTemplate::Balestarius), -1, -2, "balestarius (2d8) (-1,-2)"),
            (Box::new(PickTemplate::DwarvenPick), 1, -2, "dwarven pick (+3) (1d4) (+1,-2)"),
        ];
        for (template, tohit, todam, expected_name) in tests {
            let mut item = generate_item::generate(template, 0);
            item.set_identified(true);
            item.tohit = tohit;
            item.todam = todam;
            assert_eq!(generate(&item), expected_name);
        }
    }

    #[test]
    #[serial]
    fn test_names_unidentified() {
        let tests: Vec<(Box<dyn ItemTemplate>, &str)> = vec![
            (Box::new(AxeTemplate::Balestarius), "balestarius (2d8)"),
            (Box::new(AxeTemplate::BattleAxe), "battle axe (3d4)"),
            (Box::new(AxeTemplate::BroadAxe), "broad axe (2d6)"),
            (Box::new(AxeTemplate::HandAxe), "hand axe (1d4)"),
            (Box::new(AxeTemplate::WarAxe), "war axe (1d6)"),
            (Box::new(AxeTemplate::LargeAxe), "large axe (1d9)"),
            (Box::new(AxeTemplate::BeardedAxe), "bearded axe (2d5)"),
            (Box::new(AxeTemplate::SilverEdgedAxe), "silver edged axe (3d6)"),
            (Box::new(AxeTemplate::ChampionAxe), "champion axe (5d3)"),
            (Box::new(DaggerTemplate::MainGauche), "main gauche (1d5)"),
            (Box::new(DaggerTemplate::Misercorde), "misercorde (1d4)"),
            (Box::new(DaggerTemplate::Stiletto), "stiletto (1d4)"),
            (Box::new(DaggerTemplate::Bodkin), "bodkin (1d4)"),
            (Box::new(DaggerTemplate::BrokenDagger), "broken dagger (1d1)"),
            (Box::new(DaggerTemplate::CatONineTails), "cat-o-nine tails (1d4)"),
            (Box::new(DaggerTemplate::Bilbo), "bilbo (1d6)"),
            (Box::new(DaggerTemplate::Baselard), "baselard (1d7)"),
            (Box::new(DaggerTemplate::Foil), "foil (1d5)"),
            (Box::new(DaggerTemplate::Rapier), "rapier (1d6)"),
            (Box::new(DaggerTemplate::SmallSword), "small sword (1d6)"),
            (Box::new(PickTemplate::Pick), "pick (1d3)"),
            (Box::new(PickTemplate::Shovel), "shovel (1d2)"),
            (Box::new(PickTemplate::OrcishPick1), "orcish pick (1d3)"),
            (Box::new(PickTemplate::OrcishPick2), "orcish pick (2d3)"),
            (Box::new(PickTemplate::DwarvenPick), "dwarven pick (1d4)"),
            (Box::new(PickTemplate::GnomishShovel), "gnomish shovel (1d2)"),
            (Box::new(PickTemplate::DwarvenShovel), "dwarven shovel (1d3)"),
            (Box::new(SwordTemplate::Backsword), "backsword (1d9)"),
            (Box::new(SwordTemplate::BastardSword), "bastard sword (3d4)"),
            (Box::new(SwordTemplate::Broadsword), "broadsword (2d5)"),
            (Box::new(SwordTemplate::Claymore), "claymore (3d6)"),
            (Box::new(SwordTemplate::Cutlass), "cutlass (1d7)"),
            (Box::new(SwordTemplate::Espadon), "espadon (3d6)"),
            (Box::new(SwordTemplate::ExecutionersSword), "executioners sword (4d5)"),
            (Box::new(SwordTemplate::Flamberge), "flamberge (4d5)"),
            (Box::new(SwordTemplate::Katana), "katana (3d4)"),
            (Box::new(SwordTemplate::Longsword), "longsword (1d10)"),
            (Box::new(SwordTemplate::Nodachi), "nodachi (4d4)"),
            (Box::new(SwordTemplate::Sabre), "sabre (1d7)"),
            (Box::new(SwordTemplate::Zweihander), "zweihander (4d6)"),
            (Box::new(SwordTemplate::BrokenSword), "broken sword (1d4)"),
        ];

        for (template, expected_name) in tests {
            let subtype = template.subtype();
            let item = generate_item::generate(template, 0);
            identification::set_identified(subtype, false);
            assert_eq!(generate(&item), expected_name);
        }
    }
}
