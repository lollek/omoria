use crate::generate_item::item_template::create_melee_weapon;
use crate::generate_item::ItemQuality;
use super::super::item_template::ItemTemplate;
use crate::model::{self, item_subtype::{DaggerSubType, ItemSubType}, Item, WornFlag2};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum DaggerTemplate {
    MainGauche,
    Misercorde,
    Stiletto,
    Bodkin,
    BrokenDagger,
    CatONineTails,
    Bilbo,
    Baselard,
    Foil,
    Rapier,
    SmallSword,
}

impl DaggerTemplate {
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(DaggerTemplate::MainGauche),
            Box::new(DaggerTemplate::Misercorde),
            Box::new(DaggerTemplate::Stiletto),
            Box::new(DaggerTemplate::Bodkin),
            Box::new(DaggerTemplate::BrokenDagger),
            Box::new(DaggerTemplate::CatONineTails),
            Box::new(DaggerTemplate::Bilbo),
            Box::new(DaggerTemplate::Baselard),
            Box::new(DaggerTemplate::Foil),
            Box::new(DaggerTemplate::Rapier),
            Box::new(DaggerTemplate::SmallSword),
        ]
    }

    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        DaggerTemplate::vec().into_iter()
    }
}

impl ItemTemplate for DaggerTemplate {
    fn create(&self, item_quality: ItemQuality, _item_level: u8) -> Item {
        create_melee_weapon(self, item_quality)
    }

    fn name(&self) -> &str {
        match self {
            DaggerTemplate::MainGauche => "Main Gauche (%P0)^ (%P2,%P3)",
            DaggerTemplate::Misercorde => "Misercorde (%P0)^ (%P2,%P3)",
            DaggerTemplate::Stiletto => "Stiletto (%P0)^ (%P2,%P3)",
            DaggerTemplate::Bodkin => "Bodkin (%P0)^ (%P2,%P3)",
            DaggerTemplate::BrokenDagger => "Broken Dagger (%P0)^ (%P2,%P3)",
            DaggerTemplate::CatONineTails => "Cat-O-Nine Tails (%P0)^ (%P2,%P3)",
            DaggerTemplate::Bilbo => "Bilbo (%P0)^ (%P2,%P3)",
            DaggerTemplate::Baselard => "Baselard (%P0)^ (%P2,%P3)",
            DaggerTemplate::Foil => "Foil (%P0)^ (%P2,%P3)",
            DaggerTemplate::Rapier => "Rapier (%P0)^ (%P2,%P3)",
            DaggerTemplate::SmallSword => "Small Sword (%P0)^ (%P2,%P3)",
        }
    }

    fn item_type(&self) -> model::ItemType {
        model::ItemType::Dagger
    }
    fn flags1(&self) -> u64 {
        0
    }
    fn flags2(&self) -> u64 {
        WornFlag2::Sharp as u64
    }
    fn p1(&self) -> i64 {
        0
    }

    fn cost(&self) -> i64 {
        match self {
            DaggerTemplate::MainGauche => 25,
            DaggerTemplate::Misercorde => 10,
            DaggerTemplate::Stiletto => 10,
            DaggerTemplate::Bodkin => 10,
            DaggerTemplate::BrokenDagger => 0,
            DaggerTemplate::CatONineTails => 14,
            DaggerTemplate::Bilbo => 60,
            DaggerTemplate::Baselard => 80,
            DaggerTemplate::Foil => 35,
            DaggerTemplate::Rapier => 42,
            DaggerTemplate::SmallSword => 48,
        }
    }

    fn subtype(&self) -> ItemSubType {
        match self {
            DaggerTemplate::MainGauche => ItemSubType::Dagger(DaggerSubType::MainGauche),
            DaggerTemplate::Misercorde => ItemSubType::Dagger(DaggerSubType::Misercorde),
            DaggerTemplate::Stiletto => ItemSubType::Dagger(DaggerSubType::Stiletto),
            DaggerTemplate::Bodkin => ItemSubType::Dagger(DaggerSubType::Bodkin),
            DaggerTemplate::BrokenDagger => ItemSubType::Dagger(DaggerSubType::BrokenDagger),
            DaggerTemplate::CatONineTails => ItemSubType::Dagger(DaggerSubType::CatONineTails),
            DaggerTemplate::Bilbo => ItemSubType::Dagger(DaggerSubType::Bilbo),
            DaggerTemplate::Baselard => ItemSubType::Dagger(DaggerSubType::Baselard),
            DaggerTemplate::Foil => ItemSubType::Dagger(DaggerSubType::Foil),
            DaggerTemplate::Rapier => ItemSubType::Dagger(DaggerSubType::Rapier),
            DaggerTemplate::SmallSword => ItemSubType::Dagger(DaggerSubType::SmallSword),
        }
    }

    fn weight(&self) -> u16 {
        match self {
            DaggerTemplate::MainGauche => 30,
            DaggerTemplate::Misercorde => 15,
            DaggerTemplate::Stiletto => 12,
            DaggerTemplate::Bodkin => 20,
            DaggerTemplate::BrokenDagger => 15,
            DaggerTemplate::CatONineTails => 40,
            DaggerTemplate::Bilbo => 80,
            DaggerTemplate::Baselard => 100,
            DaggerTemplate::Foil => 30,
            DaggerTemplate::Rapier => 40,
            DaggerTemplate::SmallSword => 75,
        }
    }

    fn number(&self) -> u16 {
        1
    }
    fn modifier_to_hit(&self) -> i16 {
        0
    }
    fn modifier_to_damage(&self) -> i16 {
        0
    }
    fn base_ac(&self) -> i16 {
        0
    }
    fn modifier_to_ac(&self) -> i16 {
        0
    }

    fn damage(&self) -> &str {
        match self {
            DaggerTemplate::MainGauche => "1d5",
            DaggerTemplate::Misercorde => "1d4",
            DaggerTemplate::Stiletto => "1d4",
            DaggerTemplate::Bodkin => "1d4",
            DaggerTemplate::BrokenDagger => "1d1",
            DaggerTemplate::CatONineTails => "1d4",
            DaggerTemplate::Bilbo => "1d6",
            DaggerTemplate::Baselard => "1d7",
            DaggerTemplate::Foil => "1d5",
            DaggerTemplate::Rapier => "1d6",
            DaggerTemplate::SmallSword => "1d6",
        }
    }

    fn item_level(&self) -> u8 {
        match self {
            DaggerTemplate::MainGauche => 2,
            DaggerTemplate::Misercorde => 0,
            DaggerTemplate::Stiletto => 0,
            DaggerTemplate::Bodkin => 1,
            DaggerTemplate::BrokenDagger => 0,
            DaggerTemplate::CatONineTails => 3,
            DaggerTemplate::Bilbo => 4,
            DaggerTemplate::Baselard => 5,
            DaggerTemplate::Foil => 2,
            DaggerTemplate::Rapier => 4,
            DaggerTemplate::SmallSword => 5,
        }
    }

    fn is_identified(&self) -> bool {
        false
    }
}
