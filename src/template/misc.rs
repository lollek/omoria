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

impl MiscTemplate {
    pub fn iter() -> impl Iterator<Item=MiscTemplate> {
        [
            MiscTemplate::RatSkeleton,
            MiscTemplate::GiantCentipedeSkeleton,
            MiscTemplate::EmptyBottle,
            MiscTemplate::PotteryShard,
            MiscTemplate::HumanSkeleton,
            MiscTemplate::DwarfSkeleton,
            MiscTemplate::ElfSkeleton,
            MiscTemplate::GnomeSkeleton,
            MiscTemplate::BrokenTeeth,
            MiscTemplate::LargeBrokenBone,
            MiscTemplate::BrokenStick,
        ].iter().copied()
    }

    pub fn create(&self) -> model::Item {
        model::Item {
            name: misc::rs2item_name(self.name()),
            tval: model::ItemType::MiscObject as u8,
            flags: 0,
            flags2: 0,
            p1: 0,
            cost: 0,
            subval: self.subval(),
            weight: self.weight(),
            number: 1,
            tohit: 0,
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


    fn level(&self) -> u8 {
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
}

