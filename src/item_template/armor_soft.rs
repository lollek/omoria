use model;
use item_template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum SoftArmorTemplate {
    CoolSetOfThreads,
    DemonhideArmor,
    DuskShroud,
    ElvenChainMail,
    FilthyNagaHideArmor,
    FilthyRags,
    HardLeatherArmor,
    HardLeatherRingMail,
    HardStuddedLeather,
    LeatherScaleMail,
    Robe,
    SoftLeatherArmor,
    SoftLeatherRingMail,
    SoftStuddedLeather,
    WovenCordArmor,
    WyrmhideArmor,
    LeatherBrigantineArmor,
}


impl SoftArmorTemplate {
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
        vec![
            Box::new(SoftArmorTemplate::CoolSetOfThreads),
            Box::new(SoftArmorTemplate::DemonhideArmor),
            Box::new(SoftArmorTemplate::DuskShroud),
            Box::new(SoftArmorTemplate::ElvenChainMail),
            Box::new(SoftArmorTemplate::FilthyNagaHideArmor),
            Box::new(SoftArmorTemplate::FilthyRags),
            Box::new(SoftArmorTemplate::HardLeatherArmor),
            Box::new(SoftArmorTemplate::HardLeatherRingMail),
            Box::new(SoftArmorTemplate::HardStuddedLeather),
            Box::new(SoftArmorTemplate::LeatherScaleMail),
            Box::new(SoftArmorTemplate::Robe),
            Box::new(SoftArmorTemplate::SoftLeatherArmor),
            Box::new(SoftArmorTemplate::SoftLeatherRingMail),
            Box::new(SoftArmorTemplate::SoftStuddedLeather),
            Box::new(SoftArmorTemplate::WovenCordArmor),
            Box::new(SoftArmorTemplate::WyrmhideArmor),
            Box::new(SoftArmorTemplate::LeatherBrigantineArmor),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        SoftArmorTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            11 => Box::new(SoftArmorTemplate::CoolSetOfThreads),
            15 => Box::new(SoftArmorTemplate::DemonhideArmor),
            14 => Box::new(SoftArmorTemplate::DuskShroud),
            13 => Box::new(SoftArmorTemplate::ElvenChainMail),
            12 => Box::new(SoftArmorTemplate::FilthyNagaHideArmor),
            99 => Box::new(SoftArmorTemplate::FilthyRags),
            4 => Box::new(SoftArmorTemplate::HardLeatherArmor),
            8 => Box::new(SoftArmorTemplate::HardLeatherRingMail),
            5 => Box::new(SoftArmorTemplate::HardStuddedLeather),
            9 => Box::new(SoftArmorTemplate::LeatherScaleMail),
            1 => Box::new(SoftArmorTemplate::Robe),
            2 => Box::new(SoftArmorTemplate::SoftLeatherArmor),
            7 => Box::new(SoftArmorTemplate::SoftLeatherRingMail),
            3 => Box::new(SoftArmorTemplate::SoftStuddedLeather),
            6 => Box::new(SoftArmorTemplate::WovenCordArmor),
            16 => Box::new(SoftArmorTemplate::WyrmhideArmor),
            10 => Box::new(SoftArmorTemplate::LeatherBrigantineArmor),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for SoftArmorTemplate {

    fn name(&self) -> &str {
        match self {
            SoftArmorTemplate::CoolSetOfThreads => "Cool Set of Threads^ [%P6,%P4]",
            SoftArmorTemplate::DemonhideArmor => "Demonhide Armor^ [%P6,%P4]",
            SoftArmorTemplate::DuskShroud => "Dusk Shroud^ [%P6,%P4]",
            SoftArmorTemplate::ElvenChainMail => "Elven Chain Mail^ [%P6,%P4]",
            SoftArmorTemplate::FilthyNagaHideArmor => "Filthy Naga Hide Armor^ [%P6,%P4]",
            SoftArmorTemplate::FilthyRags => "Filthy Rags^ [%P6,%P4]",
            SoftArmorTemplate::HardLeatherArmor => "Hard Leather Armor^ [%P6,%P4]",
            SoftArmorTemplate::HardLeatherRingMail => "Hard Leather Ring Mail^ [%P6,%P4]",
            SoftArmorTemplate::HardStuddedLeather => "Hard Studded Leather^ [%P6,%P4]",
            SoftArmorTemplate::LeatherScaleMail => "Leather Scale Mail^ [%P6,%P4]",
            SoftArmorTemplate::Robe => "Robe^ [%P6,%P4]",
            SoftArmorTemplate::SoftLeatherArmor => "Soft Leather Armor^ [%P6,%P4]",
            SoftArmorTemplate::SoftLeatherRingMail => "Soft Leather Ring Mail^ [%P6,%P4]",
            SoftArmorTemplate::SoftStuddedLeather => "Soft Studded Armor^ [%P6,%P4]",
            SoftArmorTemplate::WovenCordArmor => "Woven Cord Armor^ [%P6,%P4]",
            SoftArmorTemplate::WyrmhideArmor => "Wyrmhide Armor^ [%P6,%P4]",
            SoftArmorTemplate::LeatherBrigantineArmor => "Leather Brigantine Armor^ [%P6,%P4]",
        }
    }

    fn item_type(&self) -> model::ItemType { model::ItemType::SoftArmor }
    fn flags1(&self) -> u64 { 0 }
    fn flags2(&self) -> u64 { 0 }
    fn p1(&self) -> i64 { 0 }
    fn cost(&self) -> i64 {
        match self {
            SoftArmorTemplate::CoolSetOfThreads => 45,
            SoftArmorTemplate::DemonhideArmor => 1000,
            SoftArmorTemplate::DuskShroud => 600,
            SoftArmorTemplate::ElvenChainMail => 900,
            SoftArmorTemplate::FilthyNagaHideArmor => 45,
            SoftArmorTemplate::FilthyRags => 0,
            SoftArmorTemplate::HardLeatherArmor => 55,
            SoftArmorTemplate::HardLeatherRingMail => 230,
            SoftArmorTemplate::HardStuddedLeather => 100,
            SoftArmorTemplate::LeatherScaleMail => 330,
            SoftArmorTemplate::Robe => 4,
            SoftArmorTemplate::SoftLeatherArmor => 8,
            SoftArmorTemplate::SoftLeatherRingMail => 160,
            SoftArmorTemplate::SoftStuddedLeather => 35,
            SoftArmorTemplate::WovenCordArmor => 45,
            SoftArmorTemplate::WyrmhideArmor => 1200,
            SoftArmorTemplate::LeatherBrigantineArmor => 300,
        }
    }


    fn subtype(&self) -> i64 {
        match self {
            SoftArmorTemplate::CoolSetOfThreads => 11,
            SoftArmorTemplate::DemonhideArmor => 15,
            SoftArmorTemplate::DuskShroud => 14,
            SoftArmorTemplate::ElvenChainMail => 13,
            SoftArmorTemplate::FilthyNagaHideArmor => 12,
            SoftArmorTemplate::FilthyRags => 99,
            SoftArmorTemplate::HardLeatherArmor => 4,
            SoftArmorTemplate::HardLeatherRingMail => 8,
            SoftArmorTemplate::HardStuddedLeather => 5,
            SoftArmorTemplate::LeatherScaleMail => 9,
            SoftArmorTemplate::Robe => 1,
            SoftArmorTemplate::SoftLeatherArmor => 2,
            SoftArmorTemplate::SoftLeatherRingMail => 7,
            SoftArmorTemplate::SoftStuddedLeather => 3,
            SoftArmorTemplate::WovenCordArmor => 6,
            SoftArmorTemplate::WyrmhideArmor => 16,
            SoftArmorTemplate::LeatherBrigantineArmor => 10,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            SoftArmorTemplate::CoolSetOfThreads => 75,
            SoftArmorTemplate::DemonhideArmor => 150,
            SoftArmorTemplate::DuskShroud => 150,
            SoftArmorTemplate::ElvenChainMail => 160,
            SoftArmorTemplate::FilthyNagaHideArmor => 300,
            SoftArmorTemplate::FilthyRags => 20,
            SoftArmorTemplate::HardLeatherArmor => 100,
            SoftArmorTemplate::HardLeatherRingMail => 150,
            SoftArmorTemplate::HardStuddedLeather => 110,
            SoftArmorTemplate::LeatherScaleMail => 140,
            SoftArmorTemplate::Robe => 20,
            SoftArmorTemplate::SoftLeatherArmor => 80,
            SoftArmorTemplate::SoftLeatherRingMail => 130,
            SoftArmorTemplate::SoftStuddedLeather => 90,
            SoftArmorTemplate::WovenCordArmor => 150,
            SoftArmorTemplate::WyrmhideArmor => 150,
            SoftArmorTemplate::LeatherBrigantineArmor => 190,
        }
    }

    fn number(&self) -> u16 { 1 }

    fn modifier_to_hit(&self) -> i16 {
        match self {
            SoftArmorTemplate::CoolSetOfThreads => -1,
            SoftArmorTemplate::DemonhideArmor => -1,
            SoftArmorTemplate::DuskShroud => -1,
            SoftArmorTemplate::ElvenChainMail => -1,
            SoftArmorTemplate::FilthyNagaHideArmor => -1,
            SoftArmorTemplate::FilthyRags => 0,
            SoftArmorTemplate::HardLeatherArmor => -1,
            SoftArmorTemplate::HardLeatherRingMail => -2,
            SoftArmorTemplate::HardStuddedLeather => -1,
            SoftArmorTemplate::LeatherScaleMail => -1,
            SoftArmorTemplate::Robe => 0,
            SoftArmorTemplate::SoftLeatherArmor => 0,
            SoftArmorTemplate::SoftLeatherRingMail => -1,
            SoftArmorTemplate::SoftStuddedLeather => 0,
            SoftArmorTemplate::WovenCordArmor => -1,
            SoftArmorTemplate::WyrmhideArmor => -1,
            SoftArmorTemplate::LeatherBrigantineArmor => -1,
        }
    }

    fn modifier_to_damage(&self) -> i16 { 0 }

    fn base_ac(&self) -> i16 {
        match self {
            SoftArmorTemplate::CoolSetOfThreads => 3,
            SoftArmorTemplate::DemonhideArmor => 20,
            SoftArmorTemplate::DuskShroud => 14,
            SoftArmorTemplate::ElvenChainMail => 17,
            SoftArmorTemplate::FilthyNagaHideArmor => 9,
            SoftArmorTemplate::FilthyRags => 0,
            SoftArmorTemplate::HardLeatherArmor => 6,
            SoftArmorTemplate::HardLeatherRingMail => 8,
            SoftArmorTemplate::HardStuddedLeather => 7,
            SoftArmorTemplate::LeatherScaleMail => 11,
            SoftArmorTemplate::Robe => 2,
            SoftArmorTemplate::SoftLeatherArmor => 2,
            SoftArmorTemplate::SoftLeatherRingMail => 6,
            SoftArmorTemplate::SoftStuddedLeather => 5,
            SoftArmorTemplate::WovenCordArmor => 6,
            SoftArmorTemplate::WyrmhideArmor => 25,
            SoftArmorTemplate::LeatherBrigantineArmor => 12,
        }
    }

    fn modifier_to_ac(&self) -> i16 { 0 }
    fn damage(&self) -> &str { "1d1" }

    fn item_level(&self) -> u8 {
        match self {
            SoftArmorTemplate::CoolSetOfThreads => 255,
            SoftArmorTemplate::DemonhideArmor => 255,
            SoftArmorTemplate::DuskShroud => 30,
            SoftArmorTemplate::ElvenChainMail => 255,
            SoftArmorTemplate::FilthyNagaHideArmor => 255,
            SoftArmorTemplate::FilthyRags => 0,
            SoftArmorTemplate::HardLeatherArmor => 5,
            SoftArmorTemplate::HardLeatherRingMail => 12,
            SoftArmorTemplate::HardStuddedLeather => 7,
            SoftArmorTemplate::LeatherBrigantineArmor => 20,
            SoftArmorTemplate::LeatherScaleMail => 16,
            SoftArmorTemplate::Robe => 1,
            SoftArmorTemplate::SoftLeatherArmor => 2,
            SoftArmorTemplate::SoftLeatherRingMail => 10,
            SoftArmorTemplate::SoftStuddedLeather => 3,
            SoftArmorTemplate::WovenCordArmor => 7,
            SoftArmorTemplate::WyrmhideArmor => 50,
        }
    }

    fn is_identified(&self) -> bool { false }
}


