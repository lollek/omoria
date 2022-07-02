use model;
use item_template;

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
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
        vec![
            Box::new(PolearmTemplate::AwlPike),
            Box::new(PolearmTemplate::BeakedAxe),
            Box::new(PolearmTemplate::Fauchard),
            Box::new(PolearmTemplate::Glaive),
            Box::new(PolearmTemplate::Halberd),
            Box::new(PolearmTemplate::LucerneHammer),
            Box::new(PolearmTemplate::Pike),
            Box::new(PolearmTemplate::Spike),
            Box::new(PolearmTemplate::Lance),
            Box::new(PolearmTemplate::Javelin),
            Box::new(PolearmTemplate::Naginata),
            Box::new(PolearmTemplate::WarScythe),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        PolearmTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            1 => Box::new(PolearmTemplate::AwlPike),
            2 => Box::new(PolearmTemplate::BeakedAxe),
            3 => Box::new(PolearmTemplate::Fauchard),
            4 => Box::new(PolearmTemplate::Glaive),
            5 => Box::new(PolearmTemplate::Halberd),
            6 => Box::new(PolearmTemplate::LucerneHammer),
            7 => Box::new(PolearmTemplate::Pike),
            8 => Box::new(PolearmTemplate::Spike),
            9 => Box::new(PolearmTemplate::Lance),
            10 => Box::new(PolearmTemplate::Javelin),
            11 => Box::new(PolearmTemplate::Naginata),
            12 => Box::new(PolearmTemplate::WarScythe),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for PolearmTemplate {
    fn item_type(&self) -> model::ItemType { model::ItemType::Polearm }
    fn flags1(&self) -> u64 { 0x10000000 }
    fn flags2(&self) -> u64 { 0 }
    fn p1(&self) -> i64 { 0 }

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

    fn subtype(&self) -> i64 {
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

    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }
    fn base_ac(&self) -> i16 { 0 }
    fn modifier_to_ac(&self) -> i16 { 0 }

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

    fn item_level(&self) -> u8 {
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

    fn is_identified(&self) -> bool { false }
}
