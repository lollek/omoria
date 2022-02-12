use model;
use template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum LightSourceTemplate {
    WoodenTorch,
    BrassLantern,
    MagicTorch,
    MagicLantern,
}

impl LightSourceTemplate {
    pub fn vec() -> Vec<Box<dyn template::Template>> {
        vec![
            Box::new(LightSourceTemplate::WoodenTorch),
            Box::new(LightSourceTemplate::BrassLantern),
            Box::new(LightSourceTemplate::MagicTorch),
            Box::new(LightSourceTemplate::MagicLantern),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn template::Template>> {
        LightSourceTemplate::vec().into_iter()
    }
}

impl template::Template for LightSourceTemplate {
    fn name(&self) -> &str {
        match self {
            LightSourceTemplate::WoodenTorch => "& Wooden Torch~ with %P5 turns of light",
            LightSourceTemplate::BrassLantern => "& Brass Lantern~ with %P5 turns of light",
            LightSourceTemplate::MagicTorch => "& Magic Torch^ with %P5 turns of light",
            LightSourceTemplate::MagicLantern => "& Magic Lantern^ with %P5 turns of light",
        }
    }

    fn item_type(&self) -> model::ItemType { model::ItemType::LightSource }
    fn flags1(&self) -> u64 { 0 }
    fn flags2(&self) -> u64 { 0 }


    fn p1(&self) -> i64 {
        match self {
            LightSourceTemplate::WoodenTorch => 4000,
            LightSourceTemplate::BrassLantern => 7500,
            LightSourceTemplate::MagicTorch => 9000,
            LightSourceTemplate::MagicLantern => 20000,
        }
    }

    fn cost(&self) -> i64 {
        match self {
            LightSourceTemplate::WoodenTorch => 2,
            LightSourceTemplate::BrassLantern => 35,
            LightSourceTemplate::MagicTorch => 30,
            LightSourceTemplate::MagicLantern => 200,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            LightSourceTemplate::WoodenTorch => 13,
            LightSourceTemplate::BrassLantern => 1,
            LightSourceTemplate::MagicTorch => 30,
            LightSourceTemplate::MagicLantern => 17,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            LightSourceTemplate::WoodenTorch => 30,
            LightSourceTemplate::BrassLantern => 50,
            LightSourceTemplate::MagicTorch => 1,
            LightSourceTemplate::MagicLantern => 45,
        }
    }

    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }
    fn base_ac(&self) -> i16 { 0 }
    fn modifier_to_ac(&self) -> i16 { 0 }

    fn damage(&self) -> &str {
        match self {
            LightSourceTemplate::WoodenTorch => "1d4",
            LightSourceTemplate::BrassLantern => "1d3",
            LightSourceTemplate::MagicTorch => "2d6",
            LightSourceTemplate::MagicLantern => "1d8",
        }
    }

    fn item_level(&self) -> u8 {
        match self {
            LightSourceTemplate::WoodenTorch => 1,
            LightSourceTemplate::BrassLantern => 20,
            LightSourceTemplate::MagicTorch => 10,
            LightSourceTemplate::MagicLantern => 1,
        }
    }

    fn is_identified(&self) -> bool { false }
}
