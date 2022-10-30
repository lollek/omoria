use super::super::item_template::ItemTemplate;
use crate::model::{
    self,
    item_subtype::{ItemSubType, MiscObjectSubType},
};

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
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
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

    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        MiscTemplate::vec().into_iter()
    }
}

impl ItemTemplate for MiscTemplate {
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

    fn item_type(&self) -> model::ItemType {
        model::ItemType::MiscUsable
    }
    fn flags1(&self) -> u64 {
        0
    }
    fn flags2(&self) -> u64 {
        0
    }
    fn p1(&self) -> i64 {
        0
    }
    fn cost(&self) -> i64 {
        0
    }

    fn subtype(&self) -> ItemSubType {
        match self {
            MiscTemplate::RatSkeleton => ItemSubType::MiscObject(MiscObjectSubType::RatSkeleton),
            MiscTemplate::GiantCentipedeSkeleton => {
                ItemSubType::MiscObject(MiscObjectSubType::GiantCentipedeSkeleton)
            }
            MiscTemplate::EmptyBottle => ItemSubType::MiscObject(MiscObjectSubType::EmptyBottle),
            MiscTemplate::PotteryShard => ItemSubType::MiscObject(MiscObjectSubType::PotteryShard),
            MiscTemplate::HumanSkeleton => {
                ItemSubType::MiscObject(MiscObjectSubType::HumanSkeleton)
            }
            MiscTemplate::DwarfSkeleton => {
                ItemSubType::MiscObject(MiscObjectSubType::DwarfSkeleton)
            }
            MiscTemplate::ElfSkeleton => ItemSubType::MiscObject(MiscObjectSubType::ElfSkeleton),
            MiscTemplate::GnomeSkeleton => {
                ItemSubType::MiscObject(MiscObjectSubType::GnomeSkeleton)
            }
            MiscTemplate::BrokenTeeth => ItemSubType::MiscObject(MiscObjectSubType::BrokenTeeth),
            MiscTemplate::LargeBrokenBone => {
                ItemSubType::MiscObject(MiscObjectSubType::LargeBrokenBone)
            }
            MiscTemplate::BrokenStick => ItemSubType::MiscObject(MiscObjectSubType::BrokenStick),
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

    fn number(&self) -> u16 {
        1
    }
    fn modifier_to_hit(&self) -> i16 {
        0
    }
    fn modifier_to_damage(&self) -> i16 {
        0
    }
    fn base_ac(&self) -> i16 {
        0
    }
    fn modifier_to_ac(&self) -> i16 {
        0
    }
    fn damage(&self) -> &str {
        "1d1"
    }

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

    fn is_identified(&self) -> bool {
        true
    }
}
