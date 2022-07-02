use model;
use item_template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum SongBookTemplate {
    BeginnersHandbook,
    SongBook1,
    SongBook2,
    GreaterSongBook,
}

impl SongBookTemplate {
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
        vec![
            Box::new(SongBookTemplate::BeginnersHandbook),
            Box::new(SongBookTemplate::SongBook1),
            Box::new(SongBookTemplate::SongBook2),
            Box::new(SongBookTemplate::GreaterSongBook),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        SongBookTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            262 => Box::new(SongBookTemplate::BeginnersHandbook),
            263 => Box::new(SongBookTemplate::SongBook1),
            264 => Box::new(SongBookTemplate::SongBook2),
            265 => Box::new(SongBookTemplate::GreaterSongBook),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for SongBookTemplate {
    fn item_type(&self) -> model::ItemType { model::ItemType::SongBook }

    fn flags1(&self) -> u64 {
        match self {
            SongBookTemplate::BeginnersHandbook => 0,
            SongBookTemplate::SongBook1 => 0,
            SongBookTemplate::SongBook2 => 0,
            SongBookTemplate::GreaterSongBook => 0x000001FF,
        }
    }

    fn flags2(&self) -> u64 {
        match self {
            SongBookTemplate::BeginnersHandbook => 0x000007FF,
            SongBookTemplate::SongBook1 => 0x000FF800,
            SongBookTemplate::SongBook2 => 0x7FF00000,
            SongBookTemplate::GreaterSongBook => 0,
        }
    }

    fn p1(&self) -> i64 { 0 }


    fn cost(&self) -> i64 {
        match self {
            SongBookTemplate::BeginnersHandbook => 30,
            SongBookTemplate::SongBook1 => 105,
            SongBookTemplate::SongBook2 => 305,
            SongBookTemplate::GreaterSongBook => 950,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            SongBookTemplate::BeginnersHandbook => 262,
            SongBookTemplate::SongBook1 => 263,
            SongBookTemplate::SongBook2 => 264,
            SongBookTemplate::GreaterSongBook => 265,
        }
    }

    fn weight(&self) -> u16 { 60 }
    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }
    fn base_ac(&self) -> i16 { 0 }
    fn modifier_to_ac(&self) -> i16 { 0 }
    fn damage(&self) -> &str { "1d1" }
    fn item_level(&self) -> u8 { 0 }
    fn is_identified(&self) -> bool { false }
}
