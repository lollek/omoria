use misc;
use model;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum LightSourceTemplate {
    WoodenTorch,
    BrassLantern,
    MagicTorch,
    MagicLantern,
}

pub fn generate_light_source(item_level: u8, template: LightSourceTemplate) -> model::Item {
    model::Item {
        name: misc::rs2item_name(template.name()),
        tval: model::ItemType::LightSource as u8,
        flags: 0,
        flags2: 0,
        p1: template.p1(),
        cost: template.cost(),
        subval: template.subval(),
        weight: template.weight(),
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage(template.damage()),
        level: item_level as i8,
        identified: 0,
    }
}

impl LightSourceTemplate {
    fn name(&self) -> &str {
        match self {
            LightSourceTemplate::WoodenTorch => "& Wooden Torch~ with %P5 turns of light",
            LightSourceTemplate::BrassLantern => "& Brass Lantern~ with %P5 turns of light",
            LightSourceTemplate::MagicTorch => "& Magic Torch^ with %P5 turns of light",
            LightSourceTemplate::MagicLantern => "& Magic Lantern^ with %P5 turns of light",
        }
    }

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

    fn subval(&self) -> i64 {
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

    fn damage(&self) -> &str {
        match self {
            LightSourceTemplate::WoodenTorch => "1d4",
            LightSourceTemplate::BrassLantern => "1d3",
            LightSourceTemplate::MagicTorch => "2d6",
            LightSourceTemplate::MagicLantern => "1d8",
        }
    }
}
