use misc;
use model;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum SwordTemplate {
    Backsword,
    BastardSword,
    Broadsword,
    Claymore,
    Cutlass,
    Espadon,
    ExecutionersSword,
    Flamberge,
    Katana,
    Longsword,
    Nodachi,
    Sabre,
    Zweihander,
    BrokenSword,
}

pub fn generate_sword(item_level: u8, template: SwordTemplate) -> model::Item {
    model::Item {
        name: misc::rs2item_name(template.name()),
        tval: model::ItemType::Sword as u8,
        flags: 0x10000000,
        flags2: 0,
        p1: 0,
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

impl SwordTemplate {
    fn name(&self) -> &str {
        match self {
            SwordTemplate::Backsword => "Backsword (%P0)^ (%P2,%P3)",
            SwordTemplate::BastardSword => "Bastard Sword (%P0)^ (%P2,%P3)",
            SwordTemplate::Broadsword => "Broadsword (%P0)^ (%P2,%P3)",
            SwordTemplate::Claymore => "Claymore (%P0)^ (%P2,%P3)",
            SwordTemplate::Cutlass => "Cutlass (%P0)^ (%P2,%P3)",
            SwordTemplate::Espadon => "Espadon (%P0)^ (%P2,%P3)",
            SwordTemplate::ExecutionersSword => "Executioner's Sword (%P0)^ (%P2,%P3)",
            SwordTemplate::Flamberge => "Flamberge (%P0)^ (%P2,%P3)",
            SwordTemplate::Katana => "Katana (%P0)^ (%P2,%P3)",
            SwordTemplate::Longsword => "Longsword (%P0)^ (%P2,%P3)",
            SwordTemplate::Nodachi => "No-Dachi (%P0)^ (%P2,%P3)",
            SwordTemplate::Sabre => "Sabre (%P0)^ (%P2,%P3)",
            SwordTemplate::Zweihander => "Zweihander (%P0)^ (%P2,%P3)",
            SwordTemplate::BrokenSword => "Broken Sword (%P0)^ (%P2,%P3)",
        }
    }

    fn cost(&self) -> i64 {
        match self {
            SwordTemplate::Backsword => 60,
            SwordTemplate::BastardSword => 225,
            SwordTemplate::Broadsword => 775,
            SwordTemplate::Claymore => 85,
            SwordTemplate::Cutlass => 655,
            SwordTemplate::Espadon => 850,
            SwordTemplate::ExecutionersSword => 1000,
            SwordTemplate::Flamberge => 400,
            SwordTemplate::Katana => 300,
            SwordTemplate::Longsword => 675,
            SwordTemplate::Nodachi => 50,
            SwordTemplate::Sabre => 1000,
            SwordTemplate::Zweihander => 0,
            SwordTemplate::BrokenSword => 0,
        }
    }

    fn subval(&self) -> i64 {
        match self {
            SwordTemplate::Backsword => 6,
            SwordTemplate::BastardSword => 7,
            SwordTemplate::Broadsword => 10,
            SwordTemplate::Claymore => 11,
            SwordTemplate::Cutlass => 12,
            SwordTemplate::Espadon => 13,
            SwordTemplate::ExecutionersSword => 14,
            SwordTemplate::Flamberge => 15,
            SwordTemplate::Katana => 17,
            SwordTemplate::Longsword => 18,
            SwordTemplate::Nodachi => 19,
            SwordTemplate::Sabre => 21,
            SwordTemplate::Zweihander => 23,
            SwordTemplate::BrokenSword => 24,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            SwordTemplate::Backsword => 95,
            SwordTemplate::BastardSword => 140,
            SwordTemplate::Broadsword => 150,
            SwordTemplate::Claymore => 200,
            SwordTemplate::Cutlass => 110,
            SwordTemplate::Espadon => 180,
            SwordTemplate::ExecutionersSword => 260,
            SwordTemplate::Flamberge => 240,
            SwordTemplate::Katana => 120,
            SwordTemplate::Longsword => 130,
            SwordTemplate::Nodachi => 200,
            SwordTemplate::Sabre => 50,
            SwordTemplate::Zweihander => 280,
            SwordTemplate::BrokenSword => 75,
        }
    }

    fn damage(&self) -> &str {
        match self {
            SwordTemplate::Backsword => "1d9",
            SwordTemplate::BastardSword => "3d4",
            SwordTemplate::Broadsword => "2d5",
            SwordTemplate::Claymore => "3d6",
            SwordTemplate::Cutlass => "1d7",
            SwordTemplate::Espadon => "3d6",
            SwordTemplate::ExecutionersSword => "4d5",
            SwordTemplate::Flamberge => "4d5",
            SwordTemplate::Katana => "3d4",
            SwordTemplate::Longsword => "1d10",
            SwordTemplate::Nodachi => "4d4",
            SwordTemplate::Sabre => "1d7",
            SwordTemplate::Zweihander => "4d6",
            SwordTemplate::BrokenSword => "1d4",
        }
    }
}
