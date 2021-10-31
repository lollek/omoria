use misc;
use model;

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

    pub fn create(&self) -> model::Item {
        model::Item {
            name: misc::rs2item_name(self.name()),
            tval: model::ItemType::SongBook as u8,
            flags: self.flags1(),
            flags2: self.flags2(),
            p1: 0,
            cost: self.cost() * model::Currency::Gold.value(),
            subval: self.subval(),
            weight: 60,
            number: 1,
            tohit: -100,
            todam: 0,
            ac: 0,
            toac: 0,
            damage: misc::rs2item_damage("1d1"),
            level: 0,
            identified: 0,
        }
    }

    fn name(&self) -> &str {
        match self {
            SongBookTemplate::BeginnersHandbook => "& Book of Bard Lyrics [Beginners Handbook]",
            SongBookTemplate::SongBook1 => "& Songs of Charming [Song Book I]",
            SongBookTemplate::SongBook2 => "& Ballads of Knowledge [Song Book II]",
            SongBookTemplate::GreaterSongBook => "& Epics of the Bards [Greater Song Book]",
        }
    }

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

    fn cost(&self) -> i64 {
        match self {
            SongBookTemplate::BeginnersHandbook => 30,
            SongBookTemplate::SongBook1 => 105,
            SongBookTemplate::SongBook2 => 305,
            SongBookTemplate::GreaterSongBook => 950,
        }
    }

    fn subval(&self) -> i64 {
        match self {
            SongBookTemplate::BeginnersHandbook => 262,
            SongBookTemplate::SongBook1 => 263,
            SongBookTemplate::SongBook2 => 264,
            SongBookTemplate::GreaterSongBook => 265,
        }
    }
}
