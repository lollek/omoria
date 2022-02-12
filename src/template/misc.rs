use model;
use template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum MiscTemplate {
    RatSkeleton,
    GiantCentipedeSkeleton,
    EmptyBottle,
    PotteryShard,
    HumanSkeleton,
    DwarfSkeleton,
    ElfSkeleton,
    GnomeSkeleton,
    BrokenTeeth,
    LargeBrokenBone,
    BrokenStick,
}

impl MiscTemplate {
    pub fn vec() -> Vec<Box<dyn template::Template>> {
        vec![
            Box::new(MiscTemplate::RatSkeleton),
            Box::new(MiscTemplate::GiantCentipedeSkeleton),
            Box::new(MiscTemplate::EmptyBottle),
            Box::new(MiscTemplate::PotteryShard),
            Box::new(MiscTemplate::HumanSkeleton),
            Box::new(MiscTemplate::DwarfSkeleton),
            Box::new(MiscTemplate::ElfSkeleton),
            Box::new(MiscTemplate::GnomeSkeleton),
            Box::new(MiscTemplate::BrokenTeeth),
            Box::new(MiscTemplate::LargeBrokenBone),
            Box::new(MiscTemplate::BrokenStick),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn template::Template>> {
        MiscTemplate::vec().into_iter()
    }
}

impl template::Template for MiscTemplate {
    fn name(&self) -> &str {
        match self {
            MiscTemplate::RatSkeleton => "& Rat Skeleton",
            MiscTemplate::GiantCentipedeSkeleton => "& Giant Centipede Skeleton",
            MiscTemplate::EmptyBottle => "& Empty Bottle^",
            MiscTemplate::PotteryShard => "Some Shards of Pottery^",
            MiscTemplate::HumanSkeleton => "& Human Skeleton",
            MiscTemplate::DwarfSkeleton => "& Dwarf Skeleton",
            MiscTemplate::ElfSkeleton => "& Elf Skeleton",
            MiscTemplate::GnomeSkeleton => "& Gnome Skeleton",
            MiscTemplate::BrokenTeeth => "& Broken Set of Teeth^",
            MiscTemplate::LargeBrokenBone => "& Large Broken Bone^",
            MiscTemplate::BrokenStick => "& Broken Stick^",
        }
    }

    fn item_type(&self) -> model::ItemType { model::ItemType::MiscUsable }
    fn flags1(&self) -> u64 { 0 }
    fn flags2(&self) -> u64 { 0 }
    fn p1(&self) -> i64 { 0 }
    fn cost(&self) -> i64 { 0 }

    fn subtype(&self) -> i64 {
        match self {
            MiscTemplate::RatSkeleton => 1,
            MiscTemplate::GiantCentipedeSkeleton => 2,
            MiscTemplate::EmptyBottle => 4,
            MiscTemplate::PotteryShard => 5,
            MiscTemplate::HumanSkeleton => 7,
            MiscTemplate::DwarfSkeleton => 8,
            MiscTemplate::ElfSkeleton => 9,
            MiscTemplate::GnomeSkeleton => 10,
            MiscTemplate::BrokenTeeth => 11,
            MiscTemplate::LargeBrokenBone => 12,
            MiscTemplate::BrokenStick => 13,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            MiscTemplate::RatSkeleton => 10,
            MiscTemplate::GiantCentipedeSkeleton => 25,
            MiscTemplate::EmptyBottle => 2,
            MiscTemplate::PotteryShard => 5,
            MiscTemplate::HumanSkeleton => 50,
            MiscTemplate::DwarfSkeleton => 60,
            MiscTemplate::ElfSkeleton => 40,
            MiscTemplate::GnomeSkeleton => 25,
            MiscTemplate::BrokenTeeth => 3,
            MiscTemplate::LargeBrokenBone => 2,
            MiscTemplate::BrokenStick => 1,
        }
    }

    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }
    fn base_ac(&self) -> i16 { 0 }
    fn modifier_to_ac(&self) -> i16 { 0 }
    fn damage(&self) -> &str { "1d1" }

    fn item_level(&self) -> u8 {
        match self {
            MiscTemplate::RatSkeleton => 1,
            MiscTemplate::GiantCentipedeSkeleton => 1,
            MiscTemplate::EmptyBottle => 0,
            MiscTemplate::PotteryShard => 1,
            MiscTemplate::HumanSkeleton => 1,
            MiscTemplate::DwarfSkeleton => 1,
            MiscTemplate::ElfSkeleton => 1,
            MiscTemplate::GnomeSkeleton => 1,
            MiscTemplate::BrokenTeeth => 0,
            MiscTemplate::LargeBrokenBone => 1,
            MiscTemplate::BrokenStick => 0,
        }
    }

    fn is_identified(&self) -> bool { true }
}

