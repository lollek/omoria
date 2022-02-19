use std::borrow::Cow;

use model;
use item_template;
use logic::item_name;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum PickTemplate {
    Pick,
    Shovel,
    OrcishPick1,
    OrcishPick2,
    DwarvenPick,
    GnomishShovel,
    DwarvenShovel,
}

impl PickTemplate {
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
        vec![
            Box::new(PickTemplate::Pick),
            Box::new(PickTemplate::Shovel),
            Box::new(PickTemplate::OrcishPick1),
            Box::new(PickTemplate::OrcishPick2),
            Box::new(PickTemplate::DwarvenPick),
            Box::new(PickTemplate::GnomishShovel),
            Box::new(PickTemplate::DwarvenShovel),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        PickTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            1 => Box::new(PickTemplate::Pick),
            2 => Box::new(PickTemplate::Shovel),
            4 => Box::new(PickTemplate::OrcishPick1),
            7 => Box::new(PickTemplate::OrcishPick2),
            3 => Box::new(PickTemplate::DwarvenPick),
            5 => Box::new(PickTemplate::GnomishShovel),
            6 => Box::new(PickTemplate::DwarvenShovel),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for PickTemplate {

    fn name(&self, item: &model::Item) -> String {
        item_name::generate_weapon_name(item,
            Cow::from(match self {
                PickTemplate::Pick => "Pick",
                PickTemplate::Shovel => "Shovel",
                PickTemplate::OrcishPick1 => "Orcish Pick",
                PickTemplate::OrcishPick2 => "Orcish Pick",
                PickTemplate::DwarvenPick => "Dwarven Pick",
                PickTemplate::GnomishShovel => "Gnomish Shovel",
                PickTemplate::DwarvenShovel => "Dwarven Shovel",
            }))
    }

    fn item_type(&self) -> model::ItemType { model::ItemType::Pick }

    fn flags1(&self) -> u64 {
        match self {
            PickTemplate::Pick => 0x10000000,
            PickTemplate::Shovel => 0,
            PickTemplate::OrcishPick1 => 0x10000000,
            PickTemplate::OrcishPick2 => 0x10000000,
            PickTemplate::DwarvenPick => 0x10000000,
            PickTemplate::GnomishShovel => 0,
            PickTemplate::DwarvenShovel => 0,
        }
    }

    fn flags2(&self) -> u64 {
        match self {
            PickTemplate::Pick => 0x20000000,
            PickTemplate::Shovel => 0x20000000,
            PickTemplate::OrcishPick1 => 0x20000000,
            PickTemplate::OrcishPick2 => 0x20100084,
            PickTemplate::DwarvenPick => 0x20000000,
            PickTemplate::GnomishShovel => 0x20000000,
            PickTemplate::DwarvenShovel => 0x20000000,
        }
    }

    fn p1(&self) -> i64 {
        match self {
            PickTemplate::Pick => 1,
            PickTemplate::Shovel=> 0,
            PickTemplate::OrcishPick1 => 2,
            PickTemplate::OrcishPick2 => 3,
            PickTemplate::DwarvenPick => 3,
            PickTemplate::GnomishShovel => 1,
            PickTemplate::DwarvenShovel => 2,
        }
    }

    fn cost(&self) -> i64 {
        match self {
            PickTemplate::Pick => 50,
            PickTemplate::Shovel => 15,
            PickTemplate::OrcishPick1 => 500,
            PickTemplate::OrcishPick2 => 1500,
            PickTemplate::DwarvenPick => 1200,
            PickTemplate::GnomishShovel => 100,
            PickTemplate::DwarvenShovel => 250,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            PickTemplate::Pick => 1,
            PickTemplate::Shovel => 2,
            PickTemplate::OrcishPick1 => 4,
            PickTemplate::OrcishPick2 => 7,
            PickTemplate::DwarvenPick => 3,
            PickTemplate::GnomishShovel => 5,
            PickTemplate::DwarvenShovel => 6,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            PickTemplate::Pick => 150,
            PickTemplate::Shovel => 60,
            PickTemplate::OrcishPick1 => 180,
            PickTemplate::OrcishPick2 => 180,
            PickTemplate::DwarvenPick => 200,
            PickTemplate::GnomishShovel => 50,
            PickTemplate::DwarvenShovel => 120,
        }
    }


    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }
    fn base_ac(&self) -> i16 { 0 }
    fn modifier_to_ac(&self) -> i16 { 0 }

    fn damage(&self) -> &str {
        match self {
            PickTemplate::Pick => "1d3",
            PickTemplate::Shovel => "1d2",
            PickTemplate::OrcishPick1 => "1d3",
            PickTemplate::OrcishPick2 => "2d3",
            PickTemplate::DwarvenPick => "1d4",
            PickTemplate::GnomishShovel => "1d2",
            PickTemplate::DwarvenShovel => "1d3",
        }
    }

    fn item_level(&self) -> u8 {
        match self {
            PickTemplate::Pick => 0,
            PickTemplate::Shovel => 0,
            PickTemplate::OrcishPick1 => 50,
            PickTemplate::OrcishPick2 => 40,
            PickTemplate::DwarvenPick => 20,
            PickTemplate::GnomishShovel => 20,
            PickTemplate::DwarvenShovel => 40,
        }
    }

    fn is_identified(&self) -> bool { false }
}
