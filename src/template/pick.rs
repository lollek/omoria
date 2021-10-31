use misc;
use model;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum PickTemplate {
    Pick,
    Shovel,
    OrcishPick1,
    OrcishPick2,
    DwarvenPick,
    GnomishShovel,
    DwarvenShovel,
}

impl PickTemplate {
    pub fn iter() -> impl Iterator<Item=PickTemplate> {
        [
            PickTemplate::Pick,
            PickTemplate::Shovel,
            PickTemplate::OrcishPick1,
            PickTemplate::OrcishPick2,
            PickTemplate::DwarvenPick,
            PickTemplate::GnomishShovel,
            PickTemplate::DwarvenShovel,
        ].iter().copied()
    }

    pub fn create(&self) -> model::Item {
        model::Item {
            name: misc::rs2item_name(self.name()),
            tval: model::ItemType::Pick as u8,
            flags: self.flags1(),
            flags2: self.flags2(),
            p1: self.p1(),
            cost: self.cost() * model::Currency::Gold.value(),
            subval: self.subval(),
            weight: self.weight(),
            number: 1,
            tohit: 0,
            todam: 0,
            ac: 0,
            toac: 0,
            damage: misc::rs2item_damage(self.damage()),
            level: 0,
            identified: 0,
        }
    }

    fn name(&self) -> &str {
        match self {
            PickTemplate::Pick => "& Pick (%P1) (%P2,%P3)",
            PickTemplate::Shovel => "& Shovel (%P1) (%P2,%P3)",
            PickTemplate::OrcishPick1 => "& Orcish Pick^ (%P1) (%P2,%P3)",
            PickTemplate::OrcishPick2 => "& Orcish Pick^ (%P1) (%P2,%P3)",
            PickTemplate::DwarvenPick => "& Dwarven Pick^ (%P1) (%P2,%P3)",
            PickTemplate::GnomishShovel => "& Gnomish Shovel^ (%P1) (%P2,%P3)",
            PickTemplate::DwarvenShovel => "& Dwarven Shovel^ (%P1) (%P2,%P3)",
        }
    }

    fn flags1(&self) -> u64 {
        match self {
            PickTemplate::Pick => 0x10000000,
            PickTemplate::Shovel => 0,
            PickTemplate::OrcishPick1 => 0x10000000,
            PickTemplate::OrcishPick2 => 0x10000000,
            PickTemplate::DwarvenPick => 0x10000000,
            PickTemplate::GnomishShovel => 0,
            PickTemplate::DwarvenShovel => 0,
        }
    }

    fn flags2(&self) -> u64 {
        match self {
            PickTemplate::Pick => 0x20000000,
            PickTemplate::Shovel => 0x20000000,
            PickTemplate::OrcishPick1 => 0x20000000,
            PickTemplate::OrcishPick2 => 0x20100084,
            PickTemplate::DwarvenPick => 0x20000000,
            PickTemplate::GnomishShovel => 0x20000000,
            PickTemplate::DwarvenShovel => 0x20000000,
        }
    }

    fn p1(&self) -> i64 {
        match self {
            PickTemplate::Pick => 1,
            PickTemplate::Shovel=> 0,
            PickTemplate::OrcishPick1 => 2,
            PickTemplate::OrcishPick2 => 3,
            PickTemplate::DwarvenPick => 3,
            PickTemplate::GnomishShovel => 1,
            PickTemplate::DwarvenShovel => 2,
        }
    }

    fn cost(&self) -> i64 {
        match self {
            PickTemplate::Pick => 50,
            PickTemplate::Shovel => 15,
            PickTemplate::OrcishPick1 => 500,
            PickTemplate::OrcishPick2 => 1500,
            PickTemplate::DwarvenPick => 1200,
            PickTemplate::GnomishShovel => 100,
            PickTemplate::DwarvenShovel => 250,
        }
    }

    fn subval(&self) -> i64 {
        match self {
            PickTemplate::Pick => 1,
            PickTemplate::Shovel => 2,
            PickTemplate::OrcishPick1 => 2,
            PickTemplate::OrcishPick2 => 7,
            PickTemplate::DwarvenPick => 3,
            PickTemplate::GnomishShovel => 5,
            PickTemplate::DwarvenShovel => 6,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            PickTemplate::Pick => 150,
            PickTemplate::Shovel => 60,
            PickTemplate::OrcishPick1 => 180,
            PickTemplate::OrcishPick2 => 180,
            PickTemplate::DwarvenPick => 200,
            PickTemplate::GnomishShovel => 50,
            PickTemplate::DwarvenShovel => 120,
        }
    }

    fn damage(&self) -> &str {
        match self {
            PickTemplate::Pick => "1d3",
            PickTemplate::Shovel => "1d2",
            PickTemplate::OrcishPick1 => "1d3",
            PickTemplate::OrcishPick2 => "2d3",
            PickTemplate::DwarvenPick => "1d4",
            PickTemplate::GnomishShovel => "1d2",
            PickTemplate::DwarvenShovel => "1d3",
        }
    }

    fn level(&self) -> &u8 {
        match self {
            PickTemplate::Pick => 0,
            PickTemplate::Shovel => 0,
            PickTemplate::OrcishPick1 => 50,
            PickTemplate::OrcishPick2 => 40,
            PickTemplate::DwarvenPick => 20,
            PickTemplate::GnomishShovel => 20,
            PickTemplate::DwarvenShovel => 40,
        }
    }
}
