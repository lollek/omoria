use model;
use item_template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum DungeonFeatureTemplate {
    Money,
    UnseenTrap,
    SeenTrap,
    Rubble,
    OpenDoor,
    ClosedDoor,
    UpStaircase,
    DownStaircase,
    SecretDoor,
    EntranceToStore,
    UpSteepStaircase,
    DownSteepStaircase,
    Whirlpool,
}

impl DungeonFeatureTemplate {
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
        vec![
            Box::new(DungeonFeatureTemplate::Money),
            Box::new(DungeonFeatureTemplate::UnseenTrap),
            Box::new(DungeonFeatureTemplate::SeenTrap),
            Box::new(DungeonFeatureTemplate::Rubble),
            Box::new(DungeonFeatureTemplate::OpenDoor),
            Box::new(DungeonFeatureTemplate::ClosedDoor),
            Box::new(DungeonFeatureTemplate::UpStaircase),
            Box::new(DungeonFeatureTemplate::DownStaircase),
            Box::new(DungeonFeatureTemplate::SecretDoor),
            Box::new(DungeonFeatureTemplate::EntranceToStore),
            Box::new(DungeonFeatureTemplate::UpSteepStaircase),
            Box::new(DungeonFeatureTemplate::DownSteepStaircase),
            Box::new(DungeonFeatureTemplate::Whirlpool),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        DungeonFeatureTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for DungeonFeatureTemplate {
    fn item_type(&self) -> model::ItemType {
        match self {
            DungeonFeatureTemplate::Money => model::ItemType::Money,
            DungeonFeatureTemplate::UnseenTrap => model::ItemType::UnseenTrap,
            DungeonFeatureTemplate::SeenTrap => model::ItemType::SeenTrap,
            DungeonFeatureTemplate::Rubble => model::ItemType::Rubble,
            DungeonFeatureTemplate::OpenDoor => model::ItemType::OpenDoor,
            DungeonFeatureTemplate::ClosedDoor => model::ItemType::ClosedDoor,
            DungeonFeatureTemplate::UpStaircase => model::ItemType::UpStaircase,
            DungeonFeatureTemplate::DownStaircase => model::ItemType::DownStaircase,
            DungeonFeatureTemplate::SecretDoor => model::ItemType::SecretDoor,
            DungeonFeatureTemplate::EntranceToStore => model::ItemType::EntranceToStore,
            DungeonFeatureTemplate::UpSteepStaircase => model::ItemType::UpSteepStaircase,
            DungeonFeatureTemplate::DownSteepStaircase => model::ItemType::DownSteepStaircase,
            DungeonFeatureTemplate::Whirlpool => model::ItemType::Whirlpool,
        }
    }
    fn flags1(&self) -> u64 { 0 }
    fn flags2(&self) -> u64 { 0 }
    fn p1(&self) -> i64 { 0 }
    fn cost(&self) -> i64 { 0 }

    fn subtype(&self) -> i64 {
        match self {
            DungeonFeatureTemplate::Money => 0,
            DungeonFeatureTemplate::UnseenTrap => 0,
            DungeonFeatureTemplate::SeenTrap => 0,
            DungeonFeatureTemplate::Rubble => 0,
            DungeonFeatureTemplate::OpenDoor => 1,
            DungeonFeatureTemplate::ClosedDoor => 19,
            DungeonFeatureTemplate::UpStaircase => 0,
            DungeonFeatureTemplate::DownStaircase => 0,
            DungeonFeatureTemplate::SecretDoor => 19,
            DungeonFeatureTemplate::EntranceToStore => 0,
            DungeonFeatureTemplate::UpSteepStaircase => 0,
            DungeonFeatureTemplate::DownSteepStaircase => 0,
            DungeonFeatureTemplate::Whirlpool => 0,
        }
    }

    fn weight(&self) -> u16 { 0 }

    fn number(&self) -> u16 { 0 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }
    fn base_ac(&self) -> i16 { 0 }
    fn modifier_to_ac(&self) -> i16 { 0 }
    fn damage(&self) -> &str { "1d1" }
    fn item_level(&self) -> u8 { 0 }
    fn is_identified(&self) -> bool { true }
}



