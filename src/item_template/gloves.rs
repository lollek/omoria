use model;
use item_template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum GlovesTemplate {
    LeatherGloves,
    HeavyGloves,
    ClothGloves,
    ChainGloves,
    LightGauntlets,
    HeavyGauntlets,
    SharkskinGloves,
    WarGauntlets,
    DemonhideGloves,
    WyrmhideGloves,
}

impl GlovesTemplate {
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
        vec![
            Box::new(GlovesTemplate::LeatherGloves),
            Box::new(GlovesTemplate::HeavyGloves),
            Box::new(GlovesTemplate::ClothGloves),
            Box::new(GlovesTemplate::ChainGloves),
            Box::new(GlovesTemplate::LightGauntlets),
            Box::new(GlovesTemplate::HeavyGauntlets),
            Box::new(GlovesTemplate::SharkskinGloves),
            Box::new(GlovesTemplate::WarGauntlets),
            Box::new(GlovesTemplate::DemonhideGloves),
            Box::new(GlovesTemplate::WyrmhideGloves),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        GlovesTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            1 => Box::new(GlovesTemplate::LeatherGloves),
            2 => Box::new(GlovesTemplate::HeavyGloves),
            5 => Box::new(GlovesTemplate::ClothGloves),
            6 => Box::new(GlovesTemplate::ChainGloves),
            7 => Box::new(GlovesTemplate::LightGauntlets),
            8 => Box::new(GlovesTemplate::HeavyGauntlets),
            9 => Box::new(GlovesTemplate::SharkskinGloves),
            10 => Box::new(GlovesTemplate::WarGauntlets),
            11 => Box::new(GlovesTemplate::DemonhideGloves),
            12 => Box::new(GlovesTemplate::WyrmhideGloves),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for GlovesTemplate {
    fn item_type(&self) -> model::ItemType {
        model::ItemType::Gloves
    }

    fn flags1(&self) -> u64 { 0 }
    fn flags2(&self) -> u64 { 0 }
    fn p1(&self) -> i64 { 0 }

    fn cost(&self) -> i64 {
        match self {
            GlovesTemplate::LeatherGloves => 3,
            GlovesTemplate::HeavyGloves => 35,
            GlovesTemplate::ClothGloves => 1,
            GlovesTemplate::ChainGloves => 100,
            GlovesTemplate::LightGauntlets => 150,
            GlovesTemplate::HeavyGauntlets => 200,
            GlovesTemplate::SharkskinGloves => 300,
            GlovesTemplate::WarGauntlets => 300,
            GlovesTemplate::DemonhideGloves => 400,
            GlovesTemplate::WyrmhideGloves => 1000,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            GlovesTemplate::LeatherGloves => 1,
            GlovesTemplate::HeavyGloves => 2,
            GlovesTemplate::ClothGloves => 5,
            GlovesTemplate::ChainGloves => 6,
            GlovesTemplate::LightGauntlets => 7,
            GlovesTemplate::HeavyGauntlets => 8,
            GlovesTemplate::SharkskinGloves => 9,
            GlovesTemplate::WarGauntlets => 10,
            GlovesTemplate::DemonhideGloves => 11,
            GlovesTemplate::WyrmhideGloves => 12,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            GlovesTemplate::LeatherGloves => 5,
            GlovesTemplate::HeavyGloves => 25,
            GlovesTemplate::ClothGloves => 1,
            GlovesTemplate::ChainGloves => 30,
            GlovesTemplate::LightGauntlets => 60,
            GlovesTemplate::HeavyGauntlets => 100,
            GlovesTemplate::SharkskinGloves => 30,
            GlovesTemplate::WarGauntlets => 150,
            GlovesTemplate::DemonhideGloves => 30,
            GlovesTemplate::WyrmhideGloves => 30,
        }
    }

    fn number(&self) -> u16 { 1 }


    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }

    fn base_ac(&self) -> i16 {
        match self {
            GlovesTemplate::LeatherGloves => 1,
            GlovesTemplate::HeavyGloves => 2,
            GlovesTemplate::ClothGloves => 0,
            GlovesTemplate::ChainGloves => 4,
            GlovesTemplate::LightGauntlets => 5,
            GlovesTemplate::HeavyGauntlets => 7,
            GlovesTemplate::SharkskinGloves => 6,
            GlovesTemplate::WarGauntlets => 8,
            GlovesTemplate::DemonhideGloves => 7,
            GlovesTemplate::WyrmhideGloves => 8,
        }
    }

    fn modifier_to_ac(&self) -> i16 { 0 }
    fn damage(&self) -> &str { "1d1" }

    fn item_level(&self) -> u8 {
        match self {
            GlovesTemplate::LeatherGloves => 1,
            GlovesTemplate::HeavyGloves => 5,
            GlovesTemplate::ClothGloves => 1,
            GlovesTemplate::ChainGloves => 10,
            GlovesTemplate::LightGauntlets => 16,
            GlovesTemplate::HeavyGauntlets => 16,
            GlovesTemplate::SharkskinGloves => 30,
            GlovesTemplate::WarGauntlets => 30,
            GlovesTemplate::DemonhideGloves => 40,
            GlovesTemplate::WyrmhideGloves => 50,
        }
    }

    fn is_identified(&self) -> bool { false }
}
