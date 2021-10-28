use misc;
use model;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum PickTemplate {
    OrcishPick1,
    OrcishPick2,
    DwarvenPick,
    GnomishShovel,
    DwarvenShovel,
}

pub fn generate_pick(item_level: u8, template: PickTemplate) -> model::Item {
    model::Item {
        name: misc::rs2item_name(template.name()),
        tval: model::ItemType::Pick as u8,
        flags: template.flags1(),
        flags2: template.flags2(),
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

impl PickTemplate {
    fn name(&self) -> &str {
        match self {
            PickTemplate::OrcishPick1 => "& Orcish Pick^ (%P1) (%P2,%P3)",
            PickTemplate::OrcishPick2 => "& Orcish Pick^ (%P1) (%P2,%P3)",
            PickTemplate::DwarvenPick => "& Dwarven Pick^ (%P1) (%P2,%P3)",
            PickTemplate::GnomishShovel => "& Gnomish Shovel^ (%P1) (%P2,%P3)",
            PickTemplate::DwarvenShovel => "& Dwarven Shovel^ (%P1) (%P2,%P3)",
        }
    }

    fn flags1(&self) -> u64 {
        match self {
            PickTemplate::OrcishPick1 => 0x10000000,
            PickTemplate::OrcishPick2 => 0x10000000,
            PickTemplate::DwarvenPick => 0x10000000,
            PickTemplate::GnomishShovel => 0,
            PickTemplate::DwarvenShovel => 0,
        }
    }

    fn flags2(&self) -> u64 {
        match self {
            PickTemplate::OrcishPick1 => 0x20000000,
            PickTemplate::OrcishPick2 => 0x20100084,
            PickTemplate::DwarvenPick => 0x20000000,
            PickTemplate::GnomishShovel => 0x20000000,
            PickTemplate::DwarvenShovel => 0x20000000,
        }
    }

    fn p1(&self) -> i64 {
        match self {
            PickTemplate::OrcishPick1 => 2,
            PickTemplate::OrcishPick2 => 3,
            PickTemplate::DwarvenPick => 3,
            PickTemplate::GnomishShovel => 1,
            PickTemplate::DwarvenShovel => 2,
        }
    }

    fn cost(&self) -> i64 {
        match self {
            PickTemplate::OrcishPick1 => 500,
            PickTemplate::OrcishPick2 => 1500,
            PickTemplate::DwarvenPick => 1200,
            PickTemplate::GnomishShovel => 100,
            PickTemplate::DwarvenShovel => 250,
        }
    }

    fn subval(&self) -> i64 {
        match self {
            PickTemplate::OrcishPick1 => 2,
            PickTemplate::OrcishPick2 => 7,
            PickTemplate::DwarvenPick => 3,
            PickTemplate::GnomishShovel => 5,
            PickTemplate::DwarvenShovel => 6,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            PickTemplate::OrcishPick1 => 180,
            PickTemplate::OrcishPick2 => 180,
            PickTemplate::DwarvenPick => 200,
            PickTemplate::GnomishShovel => 50,
            PickTemplate::DwarvenShovel => 120,
        }
    }

    fn damage(&self) -> &str {
        match self {
            PickTemplate::OrcishPick1 => "1d3",
            PickTemplate::OrcishPick2 => "2d3",
            PickTemplate::DwarvenPick => "1d4",
            PickTemplate::GnomishShovel => "1d2",
            PickTemplate::DwarvenShovel => "1d3",
        }
    }
}
