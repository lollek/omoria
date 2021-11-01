use model;
use template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum SongBookTemplate {
    BeginnersHandbook,
    SongBook1,
    SongBook2,
    GreaterSongBook,
}

impl SongBookTemplate {
    pub fn iter() -> impl Iterator<Item=SongBookTemplate> {
        [
            SongBookTemplate::BeginnersHandbook,
            SongBookTemplate::SongBook1,
            SongBookTemplate::SongBook2,
            SongBookTemplate::GreaterSongBook,
        ].iter().copied()
    }
}

impl template::Template for SongBookTemplate {

    fn name(&self) -> &str {
        match self {
            SongBookTemplate::BeginnersHandbook => "& Book of Bard Lyrics [Beginners Handbook]",
            SongBookTemplate::SongBook1 => "& Songs of Charming [Song Book I]",
            SongBookTemplate::SongBook2 => "& Ballads of Knowledge [Song Book II]",
            SongBookTemplate::GreaterSongBook => "& Epics of the Bards [Greater Song Book]",
        }
    }

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
