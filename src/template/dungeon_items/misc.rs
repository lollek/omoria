use misc;
use model;

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

pub fn generate_misc(item_level: u8, template: MiscTemplate) -> model::Item {
    model::Item {
        name: misc::rs2item_name(template.name()),
        tval: model::ItemType::MiscObject as u8,
        flags: 0,
        flags2: 0,
        p1: 0,
        cost: 0,
        subval: template.subval(),
        weight: template.weight(),
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("1d1"),
        level: item_level as i8,
        identified: 0,
    }
}

impl MiscTemplate {
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

    fn subval(&self) -> i64 {
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
}

