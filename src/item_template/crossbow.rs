use model;
use item_template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum CrossbowTemplate {
    SiegeCrossbow,
    Ballista,
    LightCrossbow,
    HeavyCrossbow,
}


impl CrossbowTemplate {
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
        vec![
            Box::new(CrossbowTemplate::SiegeCrossbow),
            Box::new(CrossbowTemplate::Ballista),
            Box::new(CrossbowTemplate::LightCrossbow),
            Box::new(CrossbowTemplate::HeavyCrossbow),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        CrossbowTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            10 => Box::new(CrossbowTemplate::SiegeCrossbow),
            11 => Box::new(CrossbowTemplate::Ballista),
            12 => Box::new(CrossbowTemplate::LightCrossbow),
            13 => Box::new(CrossbowTemplate::HeavyCrossbow),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for CrossbowTemplate {
    fn name(&self) -> &str {
        match self {
            CrossbowTemplate::SiegeCrossbow => "Siege Crossbow (%P0)^ (%P2,%P3)",
            CrossbowTemplate::Ballista => "Ballista (%P0)^ (%P2,%P3)",
            CrossbowTemplate::LightCrossbow => "Light Crossbow (%P0)^ (%P2,%P3)",
            CrossbowTemplate::HeavyCrossbow => "Heavy Crossbow (%P0)^ (%P2,%P3)",
       }
    }

    fn item_type(&self) -> model::ItemType { model::ItemType::Crossbow }
    fn flags1(&self) -> u64 { 0 }
    fn flags2(&self) -> u64 { 0 }

    fn p1(&self) -> i64 {
        match self {
            CrossbowTemplate::SiegeCrossbow => 5,
            CrossbowTemplate::Ballista => 6,
            CrossbowTemplate::LightCrossbow => 6,
            CrossbowTemplate::HeavyCrossbow => 7,
        }
    }

    fn cost(&self) -> i64 {
        match self {
            CrossbowTemplate::SiegeCrossbow => 140,
            CrossbowTemplate::Ballista => 300,
            CrossbowTemplate::LightCrossbow => 50,
            CrossbowTemplate::HeavyCrossbow => 120,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            CrossbowTemplate::SiegeCrossbow => 10,
            CrossbowTemplate::Ballista => 11,
            CrossbowTemplate::LightCrossbow => 12,
            CrossbowTemplate::HeavyCrossbow => 13,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            CrossbowTemplate::SiegeCrossbow => 200,
            CrossbowTemplate::Ballista => 250,
            CrossbowTemplate::LightCrossbow => 110,
            CrossbowTemplate::HeavyCrossbow => 200,
        }
    }

    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }
    fn base_ac(&self) -> i16 { 0 }
    fn modifier_to_ac(&self) -> i16 { 0 }
    fn damage(&self) -> &str { "0d0" }

    fn item_level(&self) -> u8 {
        match self {
            CrossbowTemplate::SiegeCrossbow => 15,
            CrossbowTemplate::Ballista => 30,
            CrossbowTemplate::LightCrossbow => 3,
            CrossbowTemplate::HeavyCrossbow => 10,
        }
    }

    fn is_identified(&self) -> bool { false }
}
