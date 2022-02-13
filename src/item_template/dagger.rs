use model;
use item_template;

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
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
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

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        DaggerTemplate::vec().into_iter()
    }
}

impl item_template::ItemTemplate for DaggerTemplate {

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

    fn item_type(&self) -> model::ItemType { model::ItemType::Dagger }
    fn flags1(&self) -> u64 { 0x10000000 }
    fn flags2(&self) -> u64 { 0 }
    fn p1(&self) -> i64 { 0 }

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

    fn subtype(&self) -> i64 {
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

    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }
    fn base_ac(&self) -> i16 { 0 }
    fn modifier_to_ac(&self) -> i16 { 0 }

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

    fn is_identified(&self) -> bool { false }
}