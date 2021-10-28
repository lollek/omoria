use misc;
use model;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum AxeTemplate {
    Balestarius,
    BattleAxe,
    BroadAxe,
    HandAxe,
    WarAxe,
    LargeAxe,
    BeardedAxe,
    SilverEdgedAxe,
    ChampionAxe,
}

pub fn generate_axe(item_level: u8, template: AxeTemplate) -> model::Item {
    model::Item {
        name: misc::rs2item_name(template.name()),
        tval: model::ItemType::HaftedWeapon as u8,
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

impl AxeTemplate {
    fn name(&self) -> &str {
        match self {
            AxeTemplate::Balestarius => "Balestarius (%P0)^ (%P2,%P3)",
            AxeTemplate::BattleAxe => "Battle Axe (%P0)^ (%P2,%P3)",
            AxeTemplate::BroadAxe => "Broad Axe (%P0)^ (%P2,%P3)",
            AxeTemplate::HandAxe => "Hand Axe (%P0)^ (%P2,%P3)",
            AxeTemplate::WarAxe => "War Axe (%P0)^ (%P2,%P3)",
            AxeTemplate::LargeAxe => "Large Axe (%P0)^ (%P2,%P3)",
            AxeTemplate::BeardedAxe => "Bearded Axe (%P0)^ (%P2,%P3)",
            AxeTemplate::SilverEdgedAxe => "Silved Edged Axe (%P0)^ (%P2,%P3)",
            AxeTemplate::ChampionAxe => "Champion Axe (%P0)^ (%P2,%P3)",
        }
    }

    fn cost(&self) -> i64 {
        match self {
            AxeTemplate::Balestarius => 500,
            AxeTemplate::BattleAxe => 334,
            AxeTemplate::BroadAxe => 304,
            AxeTemplate::HandAxe => 20,
            AxeTemplate::WarAxe => 60,
            AxeTemplate::LargeAxe => 60,
            AxeTemplate::BeardedAxe => 205,
            AxeTemplate::SilverEdgedAxe => 750,
            AxeTemplate::ChampionAxe => 850,
        }
    }

    fn subval(&self) -> i64 {
        match self {
            AxeTemplate::Balestarius => 1,
            AxeTemplate::BattleAxe => 3,
            AxeTemplate::BroadAxe => 4,
            AxeTemplate::HandAxe => 5,
            AxeTemplate::WarAxe => 6,
            AxeTemplate::LargeAxe => 7,
            AxeTemplate::BeardedAxe => 8,
            AxeTemplate::SilverEdgedAxe => 9,
            AxeTemplate::ChampionAxe => 10,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            AxeTemplate::Balestarius => 180,
            AxeTemplate::BattleAxe => 170,
            AxeTemplate::BroadAxe => 160,
            AxeTemplate::HandAxe => 30,
            AxeTemplate::WarAxe => 80,
            AxeTemplate::LargeAxe => 140,
            AxeTemplate::BeardedAxe => 170,
            AxeTemplate::SilverEdgedAxe => 170,
            AxeTemplate::ChampionAxe => 250,
        }
    }

    fn damage(&self) -> &str {
        match self {
            AxeTemplate::Balestarius => "2d8",
            AxeTemplate::BattleAxe => "3d4",
            AxeTemplate::BroadAxe => "2d6",
            AxeTemplate::HandAxe => "1d4",
            AxeTemplate::WarAxe => "1d6",
            AxeTemplate::LargeAxe => "1d9",
            AxeTemplate::BeardedAxe => "2d5",
            AxeTemplate::SilverEdgedAxe => "3d6",
            AxeTemplate::ChampionAxe => "5d3",
        }
    }
}

