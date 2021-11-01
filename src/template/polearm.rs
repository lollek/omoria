use misc;
use model;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum PolearmTemplate {
    AwlPike,
    BeakedAxe,
    Fauchard,
    Glaive,
    Halberd,
    LucerneHammer,
    Pike,
    Spike,
    Lance,
    Javelin,
    Naginata,
    WarScythe,
}


impl PolearmTemplate {
    pub fn iter() -> impl Iterator<Item=PolearmTemplate> {
        [
            PolearmTemplate::AwlPike,
            PolearmTemplate::BeakedAxe,
            PolearmTemplate::Fauchard,
            PolearmTemplate::Glaive,
            PolearmTemplate::Halberd,
            PolearmTemplate::LucerneHammer,
            PolearmTemplate::Pike,
            PolearmTemplate::Spike,
            PolearmTemplate::Lance,
            PolearmTemplate::Javelin,
            PolearmTemplate::Naginata,
            PolearmTemplate::WarScythe,
        ].iter().copied()
    }

    pub fn create(&self) -> model::Item {
        model::Item {
            name: misc::rs2item_name(self.name()),
            tval: model::ItemType::PoleArm as u8,
            flags: 0x10000000,
            flags2: 0,
            p1: 0,
            cost: self.cost() * model::Currency::Gold.value(),
            subval: self.subval(),
            weight: self.weight(),
            number: 1,
            tohit: 0,
            todam: 0,
            ac: 0,
            toac: 0,
            damage: misc::rs2item_damage(self.damage()),
            level: 0,
            identified: 0,
        }
    }

    fn name(&self) -> &str {
        match self {
            PolearmTemplate::AwlPike => "Awl-Pike (%P0)^ (%P2,%P3)",
            PolearmTemplate::BeakedAxe => "Beaked Axe (%P0)^ (%P2,%P3)",
            PolearmTemplate::Fauchard => "Fauchard (%P0)^ (%P2,%P3)",
            PolearmTemplate::Glaive => "Glaive (%P0)^ (%P2,%P3)",
            PolearmTemplate::Halberd => "Halberd (%P0)^ (%P2,%P3)",
            PolearmTemplate::LucerneHammer => "Lucerne Hammer (%P0)^ (%P2,%P3)",
            PolearmTemplate::Pike => "Pike (%P0)^ (%P2,%P3)",
            PolearmTemplate::Spike => "Spear (%P0)^ (%P2,%P3)",
            PolearmTemplate::Lance => "Lance (%P0)^ (%P2,%P3)",
            PolearmTemplate::Javelin => "Javelin (%P0)^ (%P2,%P3)",
            PolearmTemplate::Naginata => "Naginata (%P0)^ (%P2,%P3)",
            PolearmTemplate::WarScythe => "War Scythe (%P0)^ (%P2,%P3)",

        }
    }

    fn cost(&self) -> i64 {
        match self {
            PolearmTemplate::AwlPike => 340,
            PolearmTemplate::BeakedAxe => 408,
            PolearmTemplate::Fauchard => 376,
            PolearmTemplate::Glaive => 363,
            PolearmTemplate::Halberd => 430,
            PolearmTemplate::LucerneHammer => 376,
            PolearmTemplate::Pike => 358,
            PolearmTemplate::Spike => 36,
            PolearmTemplate::Lance => 230,
            PolearmTemplate::Javelin => 18,
            PolearmTemplate::Naginata => 1500,
            PolearmTemplate::WarScythe => 550,
        }
    }

    fn subval(&self) -> i64 {
        match self {
            PolearmTemplate::AwlPike => 1,
            PolearmTemplate::BeakedAxe => 2,
            PolearmTemplate::Fauchard => 3,
            PolearmTemplate::Glaive => 4,
            PolearmTemplate::Halberd => 5,
            PolearmTemplate::LucerneHammer => 6,
            PolearmTemplate::Pike => 7,
            PolearmTemplate::Spike => 8,
            PolearmTemplate::Lance => 9,
            PolearmTemplate::Javelin => 10,
            PolearmTemplate::Naginata => 11,
            PolearmTemplate::WarScythe => 12,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            PolearmTemplate::AwlPike => 160,
            PolearmTemplate::BeakedAxe => 180,
            PolearmTemplate::Fauchard => 170,
            PolearmTemplate::Glaive => 190,
            PolearmTemplate::Halberd => 190,
            PolearmTemplate::LucerneHammer => 120,
            PolearmTemplate::Pike => 160,
            PolearmTemplate::Spike => 50,
            PolearmTemplate::Lance => 300,
            PolearmTemplate::Javelin => 30,
            PolearmTemplate::Naginata => 250,
            PolearmTemplate::WarScythe => 210,
        }
    }

    fn damage(&self) -> &str {
        match self {
            PolearmTemplate::AwlPike => "1d8",
            PolearmTemplate::BeakedAxe => "2d6",
            PolearmTemplate::Fauchard => "1d10",
            PolearmTemplate::Glaive => "2d6",
            PolearmTemplate::Halberd => "3d3",
            PolearmTemplate::LucerneHammer => "2d5",
            PolearmTemplate::Pike => "2d5",
            PolearmTemplate::Spike => "1d6",
            PolearmTemplate::Lance => "2d8",
            PolearmTemplate::Javelin => "1d4",
            PolearmTemplate::Naginata => "5d5",
            PolearmTemplate::WarScythe => "3d5",
        }
    }

    pub fn level(&self) -> u8 {
        match self {
            PolearmTemplate::AwlPike => 8,
            PolearmTemplate::BeakedAxe => 15,
            PolearmTemplate::Fauchard => 17,
            PolearmTemplate::Glaive => 20,
            PolearmTemplate::Halberd => 22,
            PolearmTemplate::LucerneHammer => 11,
            PolearmTemplate::Pike => 15,
            PolearmTemplate::Spike => 5,
            PolearmTemplate::Lance => 10,
            PolearmTemplate::Javelin => 4,
            PolearmTemplate::Naginata => 50,
            PolearmTemplate::WarScythe => 30,
        }
    }
}