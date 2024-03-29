use super::super::item_template::ItemTemplate;
use crate::model::{
    self,
    item_subtype::{HardArmorSubType, ItemSubType, SoftArmorSubType},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum ArmorTemplate {
    AugmentedChainMail,
    BarChainMail,
    BronzePlateMail,
    ChainMail,
    CoolSetOfThreads,
    DemonhideArmor,
    DoubleChainMail,
    DuskShroud,
    ElvenChainMail,
    FilthyNagaHideArmor,
    FilthyRags,
    FullPlateArmor,
    HardLeatherArmor,
    HardLeatherRingMail,
    HardStuddedLeather,
    LacqueredPlate,
    LaminatedArmor,
    LeatherBrigantineArmor,
    LeatherScaleMail,
    MetalBrigandineArmor,
    MetalLamellarArmor,
    MetalScaleMail,
    MithrilChainMail,
    MithrilPlateArmor,
    PartialPlateArmor,
    Robe,
    RustyChainMail,
    SoftLeatherArmor,
    SoftLeatherRingMail,
    SoftStuddedLeather,
    StonePlateArmor,
    WovenCordArmor,
    WyrmhideArmor,
}

impl ArmorTemplate {
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(ArmorTemplate::AugmentedChainMail),
            Box::new(ArmorTemplate::BarChainMail),
            Box::new(ArmorTemplate::BronzePlateMail),
            Box::new(ArmorTemplate::ChainMail),
            Box::new(ArmorTemplate::CoolSetOfThreads),
            Box::new(ArmorTemplate::DemonhideArmor),
            Box::new(ArmorTemplate::DoubleChainMail),
            Box::new(ArmorTemplate::DuskShroud),
            Box::new(ArmorTemplate::ElvenChainMail),
            Box::new(ArmorTemplate::FilthyNagaHideArmor),
            Box::new(ArmorTemplate::FilthyRags),
            Box::new(ArmorTemplate::FullPlateArmor),
            Box::new(ArmorTemplate::HardLeatherArmor),
            Box::new(ArmorTemplate::HardLeatherRingMail),
            Box::new(ArmorTemplate::HardStuddedLeather),
            Box::new(ArmorTemplate::LacqueredPlate),
            Box::new(ArmorTemplate::LaminatedArmor),
            Box::new(ArmorTemplate::LeatherBrigantineArmor),
            Box::new(ArmorTemplate::LeatherScaleMail),
            Box::new(ArmorTemplate::MetalBrigandineArmor),
            Box::new(ArmorTemplate::MetalLamellarArmor),
            Box::new(ArmorTemplate::MetalScaleMail),
            Box::new(ArmorTemplate::MithrilChainMail),
            Box::new(ArmorTemplate::MithrilPlateArmor),
            Box::new(ArmorTemplate::PartialPlateArmor),
            Box::new(ArmorTemplate::Robe),
            Box::new(ArmorTemplate::RustyChainMail),
            Box::new(ArmorTemplate::SoftLeatherArmor),
            Box::new(ArmorTemplate::SoftLeatherRingMail),
            Box::new(ArmorTemplate::SoftStuddedLeather),
            Box::new(ArmorTemplate::StonePlateArmor),
            Box::new(ArmorTemplate::WovenCordArmor),
            Box::new(ArmorTemplate::WyrmhideArmor),
        ]
    }

    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        ArmorTemplate::vec().into_iter()
    }
}

impl ItemTemplate for ArmorTemplate {
    fn name(&self) -> &str {
        match self {
            ArmorTemplate::AugmentedChainMail => "Augmented Chain Mail^ [%P6,%P4]",
            ArmorTemplate::BarChainMail => "Bar Chain Mail^ [%P6,%P4]",
            ArmorTemplate::BronzePlateMail => "Bronze Plate Mail^ [%P6,%P4]",
            ArmorTemplate::ChainMail => "Chain Mail^ [%P6,%P4]",
            ArmorTemplate::CoolSetOfThreads => "Cool Set of Threads^ [%P6,%P4]",
            ArmorTemplate::DemonhideArmor => "Demonhide Armor^ [%P6,%P4]",
            ArmorTemplate::DoubleChainMail => "Double Chain Mail^ [%P6,%P4]",
            ArmorTemplate::DuskShroud => "Dusk Shroud^ [%P6,%P4]",
            ArmorTemplate::ElvenChainMail => "Elven Chain Mail^ [%P6,%P4]",
            ArmorTemplate::FilthyNagaHideArmor => "Filthy Naga Hide Armor^ [%P6,%P4]",
            ArmorTemplate::FilthyRags => "Filthy Rags^ [%P6,%P4]",
            ArmorTemplate::FullPlateArmor => "Full Plate Armor^ [%P6,%P4]",
            ArmorTemplate::HardLeatherArmor => "Hard Leather Armor^ [%P6,%P4]",
            ArmorTemplate::HardLeatherRingMail => "Hard Leather Ring Mail^ [%P6,%P4]",
            ArmorTemplate::HardStuddedLeather => "Hard Studded Leather^ [%P6,%P4]",
            ArmorTemplate::LacqueredPlate => "Lacquered Plate^ [%P6,%P4]",
            ArmorTemplate::LaminatedArmor => "Laminated Armor^ [%P6,%P4]",
            ArmorTemplate::LeatherScaleMail => "Leather Scale Mail^ [%P6,%P4]",
            ArmorTemplate::MetalBrigandineArmor => "Metal Brigandine Armor^ [%P6,%P4]",
            ArmorTemplate::MetalLamellarArmor => "Metal Lamellar Armor^ [%P6,%P4]",
            ArmorTemplate::MetalScaleMail => "Metal Scale Mail^ [%P6,%P4]",
            ArmorTemplate::MithrilChainMail => "Mithril Chain Mail^ [%P6,%P4]",
            ArmorTemplate::MithrilPlateArmor => "Mithril Plate Armor^ [%P6,%P4]",
            ArmorTemplate::PartialPlateArmor => "Partial Plate Armor^ [%P6,%P4]",
            ArmorTemplate::Robe => "Robe^ [%P6,%P4]",
            ArmorTemplate::RustyChainMail => "Rusty Chain Mail^ [%P6,%P4]",
            ArmorTemplate::SoftLeatherArmor => "Soft Leather Armor^ [%P6,%P4]",
            ArmorTemplate::SoftLeatherRingMail => "Soft Leather Ring Mail^ [%P6,%P4]",
            ArmorTemplate::SoftStuddedLeather => "Soft Studded Armor^ [%P6,%P4]",
            ArmorTemplate::StonePlateArmor => "Stone Plate Armor^ [%P6,%P4]",
            ArmorTemplate::WovenCordArmor => "Woven Cord Armor^ [%P6,%P4]",
            ArmorTemplate::WyrmhideArmor => "Wyrmhide Armor^ [%P6,%P4]",
            ArmorTemplate::LeatherBrigantineArmor => "Leather Brigantine Armor^ [%P6,%P4]",
        }
    }

    fn item_type(&self) -> model::ItemType {
        match self {
            ArmorTemplate::AugmentedChainMail => model::ItemType::HardArmor,
            ArmorTemplate::BarChainMail => model::ItemType::HardArmor,
            ArmorTemplate::BronzePlateMail => model::ItemType::HardArmor,
            ArmorTemplate::ChainMail => model::ItemType::HardArmor,
            ArmorTemplate::CoolSetOfThreads => model::ItemType::SoftArmor,
            ArmorTemplate::DemonhideArmor => model::ItemType::SoftArmor,
            ArmorTemplate::DoubleChainMail => model::ItemType::HardArmor,
            ArmorTemplate::DuskShroud => model::ItemType::SoftArmor,
            ArmorTemplate::ElvenChainMail => model::ItemType::SoftArmor,
            ArmorTemplate::FilthyNagaHideArmor => model::ItemType::SoftArmor,
            ArmorTemplate::FilthyRags => model::ItemType::SoftArmor,
            ArmorTemplate::FullPlateArmor => model::ItemType::HardArmor,
            ArmorTemplate::HardLeatherArmor => model::ItemType::SoftArmor,
            ArmorTemplate::HardLeatherRingMail => model::ItemType::SoftArmor,
            ArmorTemplate::HardStuddedLeather => model::ItemType::SoftArmor,
            ArmorTemplate::LacqueredPlate => model::ItemType::HardArmor,
            ArmorTemplate::LaminatedArmor => model::ItemType::HardArmor,
            ArmorTemplate::LeatherScaleMail => model::ItemType::SoftArmor,
            ArmorTemplate::MetalBrigandineArmor => model::ItemType::HardArmor,
            ArmorTemplate::MetalLamellarArmor => model::ItemType::HardArmor,
            ArmorTemplate::MetalScaleMail => model::ItemType::HardArmor,
            ArmorTemplate::MithrilChainMail => model::ItemType::HardArmor,
            ArmorTemplate::MithrilPlateArmor => model::ItemType::HardArmor,
            ArmorTemplate::PartialPlateArmor => model::ItemType::HardArmor,
            ArmorTemplate::Robe => model::ItemType::SoftArmor,
            ArmorTemplate::RustyChainMail => model::ItemType::HardArmor,
            ArmorTemplate::SoftLeatherArmor => model::ItemType::SoftArmor,
            ArmorTemplate::SoftLeatherRingMail => model::ItemType::SoftArmor,
            ArmorTemplate::SoftStuddedLeather => model::ItemType::SoftArmor,
            ArmorTemplate::StonePlateArmor => model::ItemType::HardArmor,
            ArmorTemplate::WovenCordArmor => model::ItemType::SoftArmor,
            ArmorTemplate::WyrmhideArmor => model::ItemType::SoftArmor,
            ArmorTemplate::LeatherBrigantineArmor => model::ItemType::SoftArmor,
        }
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
        match self {
            ArmorTemplate::AugmentedChainMail => 675,
            ArmorTemplate::BarChainMail => 720,
            ArmorTemplate::BronzePlateMail => 700,
            ArmorTemplate::ChainMail => 530,
            ArmorTemplate::CoolSetOfThreads => 45,
            ArmorTemplate::DemonhideArmor => 1000,
            ArmorTemplate::DoubleChainMail => 630,
            ArmorTemplate::DuskShroud => 600,
            ArmorTemplate::ElvenChainMail => 900,
            ArmorTemplate::FilthyNagaHideArmor => 45,
            ArmorTemplate::FilthyRags => 0,
            ArmorTemplate::FullPlateArmor => 1050,
            ArmorTemplate::HardLeatherArmor => 55,
            ArmorTemplate::HardLeatherRingMail => 230,
            ArmorTemplate::HardStuddedLeather => 100,
            ArmorTemplate::LacqueredPlate => 1200,
            ArmorTemplate::LaminatedArmor => 825,
            ArmorTemplate::LeatherScaleMail => 330,
            ArmorTemplate::MetalBrigandineArmor => 775,
            ArmorTemplate::MetalLamellarArmor => 950,
            ArmorTemplate::MetalScaleMail => 430,
            ArmorTemplate::MithrilChainMail => 1800,
            ArmorTemplate::MithrilPlateArmor => 3600,
            ArmorTemplate::PartialPlateArmor => 900,
            ArmorTemplate::Robe => 4,
            ArmorTemplate::RustyChainMail => 330,
            ArmorTemplate::SoftLeatherArmor => 8,
            ArmorTemplate::SoftLeatherRingMail => 160,
            ArmorTemplate::SoftStuddedLeather => 35,
            ArmorTemplate::StonePlateArmor => 45,
            ArmorTemplate::WovenCordArmor => 45,
            ArmorTemplate::WyrmhideArmor => 1200,
            ArmorTemplate::LeatherBrigantineArmor => 300,
        }
    }

    fn subtype(&self) -> ItemSubType {
        match self {
            ArmorTemplate::AugmentedChainMail => {
                ItemSubType::HardArmor(HardArmorSubType::AugmentedChainMail)
            }
            ArmorTemplate::BarChainMail => ItemSubType::HardArmor(HardArmorSubType::BarChainMail),
            ArmorTemplate::BronzePlateMail => {
                ItemSubType::HardArmor(HardArmorSubType::BronzePlateMail)
            }
            ArmorTemplate::ChainMail => ItemSubType::HardArmor(HardArmorSubType::ChainMail),
            ArmorTemplate::CoolSetOfThreads => {
                ItemSubType::SoftArmor(SoftArmorSubType::CoolSetOfThreads)
            }
            ArmorTemplate::DemonhideArmor => {
                ItemSubType::SoftArmor(SoftArmorSubType::DemonhideArmor)
            }
            ArmorTemplate::DoubleChainMail => {
                ItemSubType::HardArmor(HardArmorSubType::DoubleChainMail)
            }
            ArmorTemplate::DuskShroud => ItemSubType::SoftArmor(SoftArmorSubType::DuskShroud),
            ArmorTemplate::ElvenChainMail => {
                ItemSubType::SoftArmor(SoftArmorSubType::ElvenChainMail)
            }
            ArmorTemplate::FilthyNagaHideArmor => {
                ItemSubType::SoftArmor(SoftArmorSubType::FilthyNagaHideArmor)
            }
            ArmorTemplate::FilthyRags => ItemSubType::SoftArmor(SoftArmorSubType::FilthyRags),
            ArmorTemplate::FullPlateArmor => {
                ItemSubType::HardArmor(HardArmorSubType::FullPlateArmor)
            }
            ArmorTemplate::HardLeatherArmor => {
                ItemSubType::SoftArmor(SoftArmorSubType::HardLeatherArmor)
            }
            ArmorTemplate::HardLeatherRingMail => {
                ItemSubType::SoftArmor(SoftArmorSubType::HardLeatherRingMail)
            }
            ArmorTemplate::HardStuddedLeather => {
                ItemSubType::SoftArmor(SoftArmorSubType::HardStuddedLeather)
            }
            ArmorTemplate::LacqueredPlate => {
                ItemSubType::HardArmor(HardArmorSubType::LacqueredPlate)
            }
            ArmorTemplate::LaminatedArmor => {
                ItemSubType::HardArmor(HardArmorSubType::LaminatedArmor)
            }
            ArmorTemplate::LeatherScaleMail => {
                ItemSubType::SoftArmor(SoftArmorSubType::LeatherScaleMail)
            }
            ArmorTemplate::MetalBrigandineArmor => {
                ItemSubType::HardArmor(HardArmorSubType::MetalBrigandineArmor)
            }
            ArmorTemplate::MetalLamellarArmor => {
                ItemSubType::HardArmor(HardArmorSubType::MetalLamellarArmor)
            }
            ArmorTemplate::MetalScaleMail => {
                ItemSubType::HardArmor(HardArmorSubType::MetalScaleMail)
            }
            ArmorTemplate::MithrilChainMail => {
                ItemSubType::HardArmor(HardArmorSubType::MithrilChainMail)
            }
            ArmorTemplate::MithrilPlateArmor => {
                ItemSubType::HardArmor(HardArmorSubType::MithrilPlateArmor)
            }
            ArmorTemplate::PartialPlateArmor => {
                ItemSubType::HardArmor(HardArmorSubType::PartialPlateArmor)
            }
            ArmorTemplate::Robe => ItemSubType::SoftArmor(SoftArmorSubType::Robe),
            ArmorTemplate::RustyChainMail => {
                ItemSubType::HardArmor(HardArmorSubType::RustyChainMail)
            }
            ArmorTemplate::SoftLeatherArmor => {
                ItemSubType::SoftArmor(SoftArmorSubType::SoftLeatherArmor)
            }
            ArmorTemplate::SoftLeatherRingMail => {
                ItemSubType::SoftArmor(SoftArmorSubType::SoftLeatherRingMail)
            }
            ArmorTemplate::SoftStuddedLeather => {
                ItemSubType::SoftArmor(SoftArmorSubType::SoftStuddedLeather)
            }
            ArmorTemplate::StonePlateArmor => {
                ItemSubType::HardArmor(HardArmorSubType::StonePlateArmor)
            }
            ArmorTemplate::WovenCordArmor => {
                ItemSubType::SoftArmor(SoftArmorSubType::WovenCordArmor)
            }
            ArmorTemplate::WyrmhideArmor => ItemSubType::SoftArmor(SoftArmorSubType::WyrmhideArmor),
            ArmorTemplate::LeatherBrigantineArmor => {
                ItemSubType::SoftArmor(SoftArmorSubType::LeatherBrigantineArmor)
            }
        }
    }

    fn weight(&self) -> u16 {
        match self {
            ArmorTemplate::AugmentedChainMail => 270,
            ArmorTemplate::BarChainMail => 280,
            ArmorTemplate::BronzePlateMail => 380,
            ArmorTemplate::ChainMail => 220,
            ArmorTemplate::CoolSetOfThreads => 75,
            ArmorTemplate::DemonhideArmor => 150,
            ArmorTemplate::DoubleChainMail => 260,
            ArmorTemplate::DuskShroud => 150,
            ArmorTemplate::ElvenChainMail => 160,
            ArmorTemplate::FilthyNagaHideArmor => 300,
            ArmorTemplate::FilthyRags => 20,
            ArmorTemplate::FullPlateArmor => 380,
            ArmorTemplate::HardLeatherArmor => 100,
            ArmorTemplate::HardLeatherRingMail => 150,
            ArmorTemplate::HardStuddedLeather => 110,
            ArmorTemplate::LacqueredPlate => 380,
            ArmorTemplate::LaminatedArmor => 300,
            ArmorTemplate::LeatherScaleMail => 140,
            ArmorTemplate::MetalBrigandineArmor => 290,
            ArmorTemplate::MetalLamellarArmor => 340,
            ArmorTemplate::MetalScaleMail => 250,
            ArmorTemplate::MithrilChainMail => 240,
            ArmorTemplate::MithrilPlateArmor => 400,
            ArmorTemplate::PartialPlateArmor => 260,
            ArmorTemplate::Robe => 20,
            ArmorTemplate::RustyChainMail => 0,
            ArmorTemplate::SoftLeatherArmor => 80,
            ArmorTemplate::SoftLeatherRingMail => 130,
            ArmorTemplate::SoftStuddedLeather => 90,
            ArmorTemplate::StonePlateArmor => 600,
            ArmorTemplate::WovenCordArmor => 150,
            ArmorTemplate::WyrmhideArmor => 150,
            ArmorTemplate::LeatherBrigantineArmor => 190,
        }
    }

    fn number(&self) -> u16 {
        1
    }

    fn modifier_to_hit(&self) -> i16 {
        match self {
            ArmorTemplate::AugmentedChainMail => -2,
            ArmorTemplate::BarChainMail => -2,
            ArmorTemplate::BronzePlateMail => -4,
            ArmorTemplate::ChainMail => -2,
            ArmorTemplate::CoolSetOfThreads => -1,
            ArmorTemplate::DemonhideArmor => -1,
            ArmorTemplate::DoubleChainMail => -2,
            ArmorTemplate::DuskShroud => -1,
            ArmorTemplate::ElvenChainMail => -1,
            ArmorTemplate::FilthyNagaHideArmor => -1,
            ArmorTemplate::FilthyRags => 0,
            ArmorTemplate::FullPlateArmor => -3,
            ArmorTemplate::HardLeatherArmor => -1,
            ArmorTemplate::HardLeatherRingMail => -2,
            ArmorTemplate::HardStuddedLeather => -1,
            ArmorTemplate::LacqueredPlate => -3,
            ArmorTemplate::LaminatedArmor => -3,
            ArmorTemplate::LeatherScaleMail => -1,
            ArmorTemplate::MetalBrigandineArmor => -3,
            ArmorTemplate::MetalLamellarArmor => -3,
            ArmorTemplate::MetalScaleMail => -2,
            ArmorTemplate::MithrilChainMail => -1,
            ArmorTemplate::MithrilPlateArmor => -1,
            ArmorTemplate::PartialPlateArmor => -3,
            ArmorTemplate::Robe => 0,
            ArmorTemplate::RustyChainMail => -5,
            ArmorTemplate::SoftLeatherArmor => 0,
            ArmorTemplate::SoftLeatherRingMail => -1,
            ArmorTemplate::SoftStuddedLeather => 0,
            ArmorTemplate::StonePlateArmor => -6,
            ArmorTemplate::WovenCordArmor => -1,
            ArmorTemplate::WyrmhideArmor => -1,
            ArmorTemplate::LeatherBrigantineArmor => -1,
        }
    }

    fn modifier_to_damage(&self) -> i16 {
        0
    }

    fn base_ac(&self) -> i16 {
        match self {
            ArmorTemplate::AugmentedChainMail => 16,
            ArmorTemplate::BarChainMail => 18,
            ArmorTemplate::BronzePlateMail => 21,
            ArmorTemplate::ChainMail => 14,
            ArmorTemplate::CoolSetOfThreads => 3,
            ArmorTemplate::DemonhideArmor => 20,
            ArmorTemplate::DoubleChainMail => 15,
            ArmorTemplate::DuskShroud => 14,
            ArmorTemplate::ElvenChainMail => 17,
            ArmorTemplate::FilthyNagaHideArmor => 9,
            ArmorTemplate::FilthyRags => 0,
            ArmorTemplate::FullPlateArmor => 25,
            ArmorTemplate::HardLeatherArmor => 6,
            ArmorTemplate::HardLeatherRingMail => 8,
            ArmorTemplate::HardStuddedLeather => 7,
            ArmorTemplate::LacqueredPlate => 28,
            ArmorTemplate::LaminatedArmor => 20,
            ArmorTemplate::LeatherScaleMail => 11,
            ArmorTemplate::MetalBrigandineArmor => 19,
            ArmorTemplate::MetalLamellarArmor => 23,
            ArmorTemplate::MetalScaleMail => 13,
            ArmorTemplate::MithrilChainMail => 24,
            ArmorTemplate::MithrilPlateArmor => 32,
            ArmorTemplate::PartialPlateArmor => 22,
            ArmorTemplate::Robe => 2,
            ArmorTemplate::RustyChainMail => 14,
            ArmorTemplate::SoftLeatherArmor => 2,
            ArmorTemplate::SoftLeatherRingMail => 6,
            ArmorTemplate::SoftStuddedLeather => 5,
            ArmorTemplate::StonePlateArmor => 10,
            ArmorTemplate::WovenCordArmor => 6,
            ArmorTemplate::WyrmhideArmor => 25,
            ArmorTemplate::LeatherBrigantineArmor => 12,
        }
    }

    fn modifier_to_ac(&self) -> i16 {
        0
    }
    fn damage(&self) -> &str {
        "1d1"
    }

    fn item_level(&self) -> u8 {
        match self {
            ArmorTemplate::AugmentedChainMail => 30,
            ArmorTemplate::BarChainMail => 34,
            ArmorTemplate::BronzePlateMail => 32,
            ArmorTemplate::ChainMail => 26,
            ArmorTemplate::CoolSetOfThreads => 255,
            ArmorTemplate::DemonhideArmor => 255,
            ArmorTemplate::DoubleChainMail => 28,
            ArmorTemplate::DuskShroud => 30,
            ArmorTemplate::ElvenChainMail => 255,
            ArmorTemplate::FilthyNagaHideArmor => 255,
            ArmorTemplate::FilthyRags => 0,
            ArmorTemplate::FullPlateArmor => 48,
            ArmorTemplate::HardLeatherArmor => 5,
            ArmorTemplate::HardLeatherRingMail => 12,
            ArmorTemplate::HardStuddedLeather => 7,
            ArmorTemplate::LacqueredPlate => 40,
            ArmorTemplate::LaminatedArmor => 38,
            ArmorTemplate::LeatherBrigantineArmor => 20,
            ArmorTemplate::LeatherScaleMail => 16,
            ArmorTemplate::MetalBrigandineArmor => 36,
            ArmorTemplate::MetalLamellarArmor => 44,
            ArmorTemplate::MetalScaleMail => 24,
            ArmorTemplate::MithrilChainMail => 255,
            ArmorTemplate::MithrilPlateArmor => 255,
            ArmorTemplate::PartialPlateArmor => 42,
            ArmorTemplate::Robe => 1,
            ArmorTemplate::RustyChainMail => 26,
            ArmorTemplate::SoftLeatherArmor => 2,
            ArmorTemplate::SoftLeatherRingMail => 10,
            ArmorTemplate::SoftStuddedLeather => 3,
            ArmorTemplate::StonePlateArmor => 255,
            ArmorTemplate::WovenCordArmor => 7,
            ArmorTemplate::WyrmhideArmor => 50,
        }
    }

    fn is_identified(&self) -> bool {
        false
    }
}
