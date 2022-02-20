use model;
use item_template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum HardArmorTemplate {
    AugmentedChainMail,
    BarChainMail,
    BronzePlateMail,
    ChainMail,
    DoubleChainMail,
    FullPlateArmor,
    LacqueredPlate,
    LaminatedArmor,
    MetalBrigandineArmor,
    MetalLamellarArmor,
    MetalScaleMail,
    MithrilChainMail,
    MithrilPlateArmor,
    PartialPlateArmor,
    RustyChainMail,
    StonePlateArmor,
}


impl HardArmorTemplate {
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
        vec![
            Box::new(HardArmorTemplate::AugmentedChainMail),
            Box::new(HardArmorTemplate::BarChainMail),
            Box::new(HardArmorTemplate::BronzePlateMail),
            Box::new(HardArmorTemplate::ChainMail),
            Box::new(HardArmorTemplate::DoubleChainMail),
            Box::new(HardArmorTemplate::FullPlateArmor),
            Box::new(HardArmorTemplate::LacqueredPlate),
            Box::new(HardArmorTemplate::LaminatedArmor),
            Box::new(HardArmorTemplate::MetalBrigandineArmor),
            Box::new(HardArmorTemplate::MetalLamellarArmor),
            Box::new(HardArmorTemplate::MetalScaleMail),
            Box::new(HardArmorTemplate::MithrilChainMail),
            Box::new(HardArmorTemplate::MithrilPlateArmor),
            Box::new(HardArmorTemplate::PartialPlateArmor),
            Box::new(HardArmorTemplate::RustyChainMail),
            Box::new(HardArmorTemplate::StonePlateArmor),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        HardArmorTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            5 => Box::new(HardArmorTemplate::AugmentedChainMail),
            6 => Box::new(HardArmorTemplate::BarChainMail),
            13 => Box::new(HardArmorTemplate::BronzePlateMail),
            2 => Box::new(HardArmorTemplate::ChainMail),
            4 => Box::new(HardArmorTemplate::DoubleChainMail),
            11 => Box::new(HardArmorTemplate::FullPlateArmor),
            12 => Box::new(HardArmorTemplate::LacqueredPlate),
            8 => Box::new(HardArmorTemplate::LaminatedArmor),
            7 => Box::new(HardArmorTemplate::MetalBrigandineArmor),
            10 => Box::new(HardArmorTemplate::MetalLamellarArmor),
            1 => Box::new(HardArmorTemplate::MetalScaleMail),
            15 => Box::new(HardArmorTemplate::MithrilChainMail),
            16 => Box::new(HardArmorTemplate::MithrilPlateArmor),
            9 => Box::new(HardArmorTemplate::PartialPlateArmor),
            3 => Box::new(HardArmorTemplate::RustyChainMail),
            14 => Box::new(HardArmorTemplate::StonePlateArmor),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for HardArmorTemplate {
    fn item_type(&self) -> model::ItemType { model::ItemType::HardArmor }
    fn flags1(&self) -> u64 { 0 }
    fn flags2(&self) -> u64 { 0 }
    fn p1(&self) -> i64 { 0 }
    fn cost(&self) -> i64 {
        match self {
            HardArmorTemplate::AugmentedChainMail => 675,
            HardArmorTemplate::BarChainMail => 720,
            HardArmorTemplate::BronzePlateMail => 700,
            HardArmorTemplate::ChainMail => 530,
            HardArmorTemplate::DoubleChainMail => 630,
            HardArmorTemplate::FullPlateArmor => 1050,
            HardArmorTemplate::LacqueredPlate => 1200,
            HardArmorTemplate::LaminatedArmor => 825,
            HardArmorTemplate::MetalBrigandineArmor => 775,
            HardArmorTemplate::MetalLamellarArmor => 950,
            HardArmorTemplate::MetalScaleMail => 430,
            HardArmorTemplate::MithrilChainMail => 1800,
            HardArmorTemplate::MithrilPlateArmor => 3600,
            HardArmorTemplate::PartialPlateArmor => 900,
            HardArmorTemplate::RustyChainMail => 330,
            HardArmorTemplate::StonePlateArmor => 45,
        }
    }


    fn subtype(&self) -> i64 {
        match self {
            HardArmorTemplate::AugmentedChainMail => 5,
            HardArmorTemplate::BarChainMail => 6,
            HardArmorTemplate::BronzePlateMail => 13,
            HardArmorTemplate::ChainMail => 2,
            HardArmorTemplate::DoubleChainMail => 4,
            HardArmorTemplate::FullPlateArmor => 11,
            HardArmorTemplate::LacqueredPlate => 12,
            HardArmorTemplate::LaminatedArmor => 8,
            HardArmorTemplate::MetalBrigandineArmor => 7,
            HardArmorTemplate::MetalLamellarArmor => 10,
            HardArmorTemplate::MetalScaleMail => 1,
            HardArmorTemplate::MithrilChainMail => 15,
            HardArmorTemplate::MithrilPlateArmor => 16,
            HardArmorTemplate::PartialPlateArmor => 9,
            HardArmorTemplate::RustyChainMail => 3,
            HardArmorTemplate::StonePlateArmor => 14,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            HardArmorTemplate::AugmentedChainMail => 270,
            HardArmorTemplate::BarChainMail => 280,
            HardArmorTemplate::BronzePlateMail => 380,
            HardArmorTemplate::ChainMail => 220,
            HardArmorTemplate::DoubleChainMail => 260,
            HardArmorTemplate::FullPlateArmor => 380,
            HardArmorTemplate::LacqueredPlate => 380,
            HardArmorTemplate::LaminatedArmor => 300,
            HardArmorTemplate::MetalBrigandineArmor => 290,
            HardArmorTemplate::MetalLamellarArmor => 340,
            HardArmorTemplate::MetalScaleMail => 250,
            HardArmorTemplate::MithrilChainMail => 240,
            HardArmorTemplate::MithrilPlateArmor => 400,
            HardArmorTemplate::PartialPlateArmor => 260,
            HardArmorTemplate::RustyChainMail => 0,
            HardArmorTemplate::StonePlateArmor => 600,
        }
    }

    fn number(&self) -> u16 { 1 }

    fn modifier_to_hit(&self) -> i16 {
        match self {
            HardArmorTemplate::AugmentedChainMail => -2,
            HardArmorTemplate::BarChainMail => -2,
            HardArmorTemplate::BronzePlateMail => -4,
            HardArmorTemplate::ChainMail => -2,
            HardArmorTemplate::DoubleChainMail => -2,
            HardArmorTemplate::FullPlateArmor => -3,
            HardArmorTemplate::LacqueredPlate => -3,
            HardArmorTemplate::LaminatedArmor => -3,
            HardArmorTemplate::MetalBrigandineArmor => -3,
            HardArmorTemplate::MetalLamellarArmor => -3,
            HardArmorTemplate::MetalScaleMail => -2,
            HardArmorTemplate::MithrilChainMail => -1,
            HardArmorTemplate::MithrilPlateArmor => -1,
            HardArmorTemplate::PartialPlateArmor => -3,
            HardArmorTemplate::RustyChainMail => -5,
            HardArmorTemplate::StonePlateArmor => -6,
        }
    }

    fn modifier_to_damage(&self) -> i16 { 0 }

    fn base_ac(&self) -> i16 {
        match self {
            HardArmorTemplate::AugmentedChainMail => 16,
            HardArmorTemplate::BarChainMail => 18,
            HardArmorTemplate::BronzePlateMail => 21,
            HardArmorTemplate::ChainMail => 14,
            HardArmorTemplate::DoubleChainMail => 15,
            HardArmorTemplate::FullPlateArmor => 25,
            HardArmorTemplate::LacqueredPlate => 28,
            HardArmorTemplate::LaminatedArmor => 20,
            HardArmorTemplate::MetalBrigandineArmor => 19,
            HardArmorTemplate::MetalLamellarArmor => 23,
            HardArmorTemplate::MetalScaleMail => 13,
            HardArmorTemplate::MithrilChainMail => 24,
            HardArmorTemplate::MithrilPlateArmor => 32,
            HardArmorTemplate::PartialPlateArmor => 22,
            HardArmorTemplate::RustyChainMail => 14,
            HardArmorTemplate::StonePlateArmor => 10,
        }
    }

    fn modifier_to_ac(&self) -> i16 { 0 }
    fn damage(&self) -> &str { "1d1" }

    fn item_level(&self) -> u8 {
        match self {
            HardArmorTemplate::AugmentedChainMail => 30,
            HardArmorTemplate::BarChainMail => 34,
            HardArmorTemplate::BronzePlateMail => 32,
            HardArmorTemplate::ChainMail => 26,
            HardArmorTemplate::DoubleChainMail => 28,
            HardArmorTemplate::FullPlateArmor => 48,
            HardArmorTemplate::LacqueredPlate => 40,
            HardArmorTemplate::LaminatedArmor => 38,
            HardArmorTemplate::MetalBrigandineArmor => 36,
            HardArmorTemplate::MetalLamellarArmor => 44,
            HardArmorTemplate::MetalScaleMail => 24,
            HardArmorTemplate::MithrilChainMail => 255,
            HardArmorTemplate::MithrilPlateArmor => 255,
            HardArmorTemplate::PartialPlateArmor => 42,
            HardArmorTemplate::RustyChainMail => 26,
            HardArmorTemplate::StonePlateArmor => 255,
        }
    }

    fn is_identified(&self) -> bool { false }
}

