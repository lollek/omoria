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
    fn name(&self, item: &model::Item) -> String {
        let plural_s = || if item.number == 1 { "" } else { "s" };

        let mut parts = Vec::new();
        match self {
            DungeonFeatureTemplate::Money => {
                parts.push(item_name::number_of(item));
                Cow::from(match item.subval {
                    1 => format!(" iron piece{}", plural_s()),
                    2 => format!(" copper piece{}", plural_s()),
                    3 => format!(" iron piece{}", plural_s()),
                    4 => format!(" copper piece{}", plural_s()),
                    5 => format!(" silver piece{}", plural_s()),
                    6 => format!(" copper piece{}", plural_s()),
                    7 => format!(" silver piece{}", plural_s()),
                    12 => format!(" gold piece{}", plural_s()),
                    16 => format!(" silver piece{}", plural_s()),
                    18 => format!(" gold piece{}", plural_s()),
                    20 => format!(" platinum piece{}", plural_s()),
                    24 => format!(" gold piece{}", plural_s()),
                    28 => format!(" platinum piece{}", plural_s()),
                    32 => format!(" mithril piece{}", plural_s()),
                    50 => format!(" gold piece{}", plural_s()),
                    55 => format!(" platinum piece{}", plural_s()),
                    60 => format!(" mithril piece{}", plural_s()),
                })
            },
            DungeonFeatureTemplate::UnseenTrap |
            DungeonFeatureTemplate::SeenTrap
                => Cow::from(match item.subval {
                    1 => "an open pit",
                    2 => "an arrow trap",
                    3 => "a covered pit",
                    4 => "a trap door",
                    5 => "a gas trap",
                    6 => "a loose rock",
                    7 => "a dart trap",
                    8 => "a strange rune",
                    9 => "some loose rock",
                    10 => "a gas trap",
                    11 => "a strange rune",
                    12 => "a blackened spot",
                    13 => "some corroded rock",
                    14 => "a gas trap",
                    15 => "a gas trap",
                    16 => "a gas trap",
                    17 => "a dart trap",
                    18 => "a dart trap",
                    20 => "a chute",
                    _ => "an unnamed trap <TODO>",
                }),
            DungeonFeatureTemplate::Rubble => Cow::from("some rubble"),
            DungeonFeatureTemplate::OpenDoor => Cow::from("an open door"),
            DungeonFeatureTemplate::ClosedDoor => Cow::from("a closed door"),
            DungeonFeatureTemplate::UpStaircase => Cow::from("an up staircase"),
            DungeonFeatureTemplate::DownStaircase => Cow::from("a down staircase"),
            DungeonFeatureTemplate::SecretDoor => Cow::from("a secret door"),
            DungeonFeatureTemplate::EntranceToStore =>
                Cow::from(match item.subval {
                    101 => "the entrance to the General Store",
                    102 => "the entrance to the Armory",
                    103 => "the entrance to the Weapon Smiths",
                    104 => "the entrance to the Temple",
                    105 => "the entrance to the Alchemy Shop",
                    106 => "the entrance to the Magic Shop",
                    107 => "the entrance to the Inn",
                    108 => "the entrance to the Trading Post",
                    109 => "the entrance to the Library",
                    110 => "the entrance to the Music Shop",
                    111 => "the entrance to the Insurance Shop",
                    112 => "the entrance to the Bank",
                    113 => "the entrance to the Gem Store",
                    114 => "the entrance to the Money Exchange",
                    115 => "the entrance to the Casino",
                    116 => "the entrance to the All-Nite Deli",
                    117 => "the entrance to a strange building",
                    118 => "the entrance to a building",
                    120 => "the entrance to a building",
                    121 => "the entrance to a building",
                    122 => "the entrance to a building",
                    123 => "the entrance to a building",
                    124 => "the entrance to a building",
                }),
            DungeonFeatureTemplate::UpSteepStaircase => Cow::from("a steep staircase"),
            DungeonFeatureTemplate::DownSteepStaircase => Cow::from("a steep staircase"),
            DungeonFeatureTemplate::Whirlpool => Cow::from("<TODO>"),
        };
        parts.join("")
    }

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



