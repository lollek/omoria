use model;
use item_template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum SlingTemplate {
    Sling,
}


impl SlingTemplate {
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
        vec![
            Box::new(SlingTemplate::Sling),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        SlingTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            20 => Box::new(SlingTemplate::Sling),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for SlingTemplate {
    fn name(&self) -> &str {
        match self {
            SlingTemplate::Sling => "Sling (%P0)^ (%P2,%P3)",
       }
    }

    fn item_type(&self) -> model::ItemType { model::ItemType::Sling }
    fn flags1(&self) -> u64 { 0 }
    fn flags2(&self) -> u64 { 0 }

    fn p1(&self) -> i64 {
        match self {
            SlingTemplate::Sling => 1,
        }
    }

    fn cost(&self) -> i64 {
        match self {
            SlingTemplate::Sling => 5,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            SlingTemplate::Sling => 20,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            SlingTemplate::Sling => 5,
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
            SlingTemplate::Sling => 0,
        }
    }

    fn is_identified(&self) -> bool { false }
}
