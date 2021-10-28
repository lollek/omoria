use misc;
use model;

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

pub fn generate_dagger(item_level: u8, template: DaggerTemplate) -> model::Item {
    model::Item {
        name: misc::rs2item_name(template.name()),
        tval: model::ItemType::Dagger as u8,
        flags: 0x10000000,
        flags2: 0,
        p1: 0,
        cost: template.cost(),
        subval: template.subval(),
        weight: template.weight(),
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage(template.damage()),
        level: item_level as i8,
        identified: 0,
    }
}

impl DaggerTemplate {
    fn name(&self) -> &str {
        match self {
            DaggerTemplate::MainGauche =>"Main Gauche (%P0)^ (%P2,%P3)",
            DaggerTemplate::Misercorde =>"Misercorde (%P0)^ (%P2,%P3)",
            DaggerTemplate::Stiletto =>"Stiletto (%P0)^ (%P2,%P3)",
            DaggerTemplate::Bodkin =>"Bodkin (%P0)^ (%P2,%P3)",
            DaggerTemplate::BrokenDagger =>"Broken Dagger (%P0)^ (%P2,%P3)",
            DaggerTemplate::CatONineTails =>"Cat-O-Nine Tails (%P0)^ (%P2,%P3)",
            DaggerTemplate::Bilbo =>"Bilbo (%P0)^ (%P2,%P3)",
            DaggerTemplate::Baselard =>"Baselard (%P0)^ (%P2,%P3)",
            DaggerTemplate::Foil =>"Foil (%P0)^ (%P2,%P3)",
            DaggerTemplate::Rapier =>"Rapier (%P0)^ (%P2,%P3)",
            DaggerTemplate::SmallSword =>"Small Sword (%P0)^ (%P2,%P3)",
        }
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

    fn subval(&self) -> i64 {
        match self {
            DaggerTemplate::MainGauche => 1,
            DaggerTemplate::Misercorde => 2,
            DaggerTemplate::Stiletto => 3,
            DaggerTemplate::Bodkin => 4,
            DaggerTemplate::BrokenDagger => 5,
            DaggerTemplate::CatONineTails => 5,
            DaggerTemplate::Bilbo => 8,
            DaggerTemplate::Baselard => 9,
            DaggerTemplate::Foil => 16,
            DaggerTemplate::Rapier => 20,
            DaggerTemplate::SmallSword => 22,
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

    fn damage(&self) -> &str {
        match self {
            DaggerTemplate::MainGauche =>"1d5",
            DaggerTemplate::Misercorde =>"1d4",
            DaggerTemplate::Stiletto =>"1d4",
            DaggerTemplate::Bodkin =>"1d4",
            DaggerTemplate::BrokenDagger =>"1d1",
            DaggerTemplate::CatONineTails =>"1d4",
            DaggerTemplate::Bilbo =>"1d6",
            DaggerTemplate::Baselard =>"1d7",
            DaggerTemplate::Foil =>"1d5",
            DaggerTemplate::Rapier =>"1d6",
            DaggerTemplate::SmallSword =>"1d6",
        }
    }
}
