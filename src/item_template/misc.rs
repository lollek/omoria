use std::borrow::Cow;

use model;
use item_template;
use logic::item_name;

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
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
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

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        MiscTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            1 => Box::new(MiscTemplate::RatSkeleton),
            2 => Box::new(MiscTemplate::GiantCentipedeSkeleton),
            4 => Box::new(MiscTemplate::EmptyBottle),
            5 => Box::new(MiscTemplate::PotteryShard),
            7 => Box::new(MiscTemplate::HumanSkeleton),
            8 => Box::new(MiscTemplate::DwarfSkeleton),
            9 => Box::new(MiscTemplate::ElfSkeleton),
            10 => Box::new(MiscTemplate::GnomeSkeleton),
            11 => Box::new(MiscTemplate::BrokenTeeth),
            12 => Box::new(MiscTemplate::LargeBrokenBone),
            13 => Box::new(MiscTemplate::BrokenStick),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for MiscTemplate {
    fn name(&self, item: &model::Item) -> String {
        let mut parts = Vec::new();
        parts.push(item_name::number_of(item));
        parts.push(Cow::from(
                match self {
                    MiscTemplate::RatSkeleton => "Rat Skeleton",
                    MiscTemplate::GiantCentipedeSkeleton => "Giant Centipede Skeleton",
                    MiscTemplate::EmptyBottle => "Empty Bottle",
                    MiscTemplate::PotteryShard => "Some Shards of Pottery",
                    MiscTemplate::HumanSkeleton => "Human Skeleton",
                    MiscTemplate::DwarfSkeleton => "Dwarf Skeleton",
                    MiscTemplate::ElfSkeleton => "Elf Skeleton",
                    MiscTemplate::GnomeSkeleton => "Gnome Skeleton",
                    MiscTemplate::BrokenTeeth => "Broken Set of Teeth",
                    MiscTemplate::LargeBrokenBone => "Large Broken Bone",
                    MiscTemplate::BrokenStick => "Broken Stick",
                }));
        parts.join("")
    }

    fn item_type(&self) -> model::ItemType { model::ItemType::MiscObject }
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

