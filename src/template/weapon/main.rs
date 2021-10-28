use std::collections::HashMap;

use model;
use template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Options {
    Axe(template::AxeTemplate),
    Bow(template::BowTemplate),
    Crossbow(template::CrossbowTemplate),
    Dagger(template::DaggerTemplate),
    Mace(template::MaceTemplate),
    Polearm(template::PolearmTemplate),
    Sling(template::SlingTemplate),
    Sword(template::SwordTemplate),
}

pub fn generate_weapon(item_level: u8) -> model::Item {
    let usable_level: HashMap<Options, u8> = [
        (Options::Axe(template::AxeTemplate::Balestarius), 30),
        (Options::Axe(template::AxeTemplate::BattleAxe), 13),
        (Options::Axe(template::AxeTemplate::BroadAxe), 17),
        (Options::Axe(template::AxeTemplate::HandAxe), 1),
        (Options::Axe(template::AxeTemplate::WarAxe), 4),
        (Options::Axe(template::AxeTemplate::LargeAxe), 7),
        (Options::Axe(template::AxeTemplate::BeardedAxe), 9),
        (Options::Axe(template::AxeTemplate::SilverEdgedAxe), 30),
        (Options::Axe(template::AxeTemplate::ChampionAxe), 40),

        (Options::Bow(template::BowTemplate::Shortbow), 3),
        (Options::Bow(template::BowTemplate::HuntersBow), 10),
        (Options::Bow(template::BowTemplate::CompositeBow), 40),
        (Options::Bow(template::BowTemplate::WarBow), 15),
        (Options::Bow(template::BowTemplate::DoubleBow), 20),
        (Options::Bow(template::BowTemplate::SiegeBow), 25),
        (Options::Bow(template::BowTemplate::WardedBow), 30),

        (Options::Crossbow(template::CrossbowTemplate::SiegeCrossbow), 15),
        (Options::Crossbow(template::CrossbowTemplate::Ballista), 30),
        (Options::Crossbow(template::CrossbowTemplate::LightCrossbow), 3),
        (Options::Crossbow(template::CrossbowTemplate::HeavyCrossbow), 10),

        (Options::Dagger(template::DaggerTemplate::MainGauche), 2),
        (Options::Dagger(template::DaggerTemplate::Misercorde), 0),
        (Options::Dagger(template::DaggerTemplate::Stiletto), 0),
        (Options::Dagger(template::DaggerTemplate::Bodkin), 1),
        (Options::Dagger(template::DaggerTemplate::BrokenDagger), 0),
        (Options::Dagger(template::DaggerTemplate::CatONineTails), 3),
        (Options::Dagger(template::DaggerTemplate::Bilbo), 4),
        (Options::Dagger(template::DaggerTemplate::Baselard), 5),
        (Options::Dagger(template::DaggerTemplate::Foil), 2),
        (Options::Dagger(template::DaggerTemplate::Rapier), 4),
        (Options::Dagger(template::DaggerTemplate::SmallSword), 5),

        (Options::Mace(template::MaceTemplate::BallAndChain), 20),
        (Options::Mace(template::MaceTemplate::WoodenClub), 0),
        (Options::Mace(template::MaceTemplate::Flail), 12),
        (Options::Mace(template::MaceTemplate::GreatFlail), 45),
        (Options::Mace(template::MaceTemplate::MorningStar), 10),
        (Options::Mace(template::MaceTemplate::Mace), 6),
        (Options::Mace(template::MaceTemplate::WarHammer), 5),
        (Options::Mace(template::MaceTemplate::LeadFilledMace), 15),
        (Options::Mace(template::MaceTemplate::IronShodQuarterstaff), 2),
        (Options::Mace(template::MaceTemplate::OgreMaul), 50),


        (Options::Polearm(template::PolearmTemplate::AwlPike), 8),
        (Options::Polearm(template::PolearmTemplate::BeakedAxe), 15),
        (Options::Polearm(template::PolearmTemplate::Fauchard), 17),
        (Options::Polearm(template::PolearmTemplate::Glaive), 20),
        (Options::Polearm(template::PolearmTemplate::Halberd), 22),
        (Options::Polearm(template::PolearmTemplate::LucerneHammer), 11),
        (Options::Polearm(template::PolearmTemplate::Pike), 15),
        (Options::Polearm(template::PolearmTemplate::Spike), 5),
        (Options::Polearm(template::PolearmTemplate::Lance), 10),
        (Options::Polearm(template::PolearmTemplate::Javelin), 4),
        (Options::Polearm(template::PolearmTemplate::Naginata), 50),
        (Options::Polearm(template::PolearmTemplate::WarScythe), 30),

        (Options::Sling(template::SlingTemplate::Sling), 0),

        (Options::Sword(template::SwordTemplate::Backsword), 7),
        (Options::Sword(template::SwordTemplate::BastardSword), 14),
        (Options::Sword(template::SwordTemplate::Broadsword), 9),
        (Options::Sword(template::SwordTemplate::Claymore), 30),
        (Options::Sword(template::SwordTemplate::Cutlass), 7),
        (Options::Sword(template::SwordTemplate::Espadon), 35),
        (Options::Sword(template::SwordTemplate::ExecutionersSword), 40),
        (Options::Sword(template::SwordTemplate::Flamberge), 45),
        (Options::Sword(template::SwordTemplate::Katana), 18),
        (Options::Sword(template::SwordTemplate::Longsword), 12),
        (Options::Sword(template::SwordTemplate::Nodachi), 45),
        (Options::Sword(template::SwordTemplate::Sabre), 5),
        (Options::Sword(template::SwordTemplate::Zweihander), 50),
        (Options::Sword(template::SwordTemplate::BrokenSword), 0),
    ].iter().cloned().collect();

    let available_templates: Vec<Options> = usable_level.into_iter()
        .filter(|(_option, level)| level >= &item_level)
        .map(|(option, _level)| option)
        .collect();

    match available_templates[rand::random::<usize>() % available_templates.len()] {
        Options::Axe(axe) => template::generate_axe(item_level, axe),
        Options::Bow(bow) => template::generate_bow(item_level, bow),
        Options::Crossbow(crossbow) => template::generate_crossbow(item_level, crossbow),
        Options::Dagger(dagger) => template::generate_dagger(item_level, dagger),
        Options::Mace(mace) => template::generate_mace(item_level, mace),
        Options::Polearm(polearm) => template::generate_polearm(item_level, polearm),
        Options::Sling(sling) => template::generate_sling(item_level, sling),
        Options::Sword(sword) => template::generate_sword(item_level, sword),
    }
}



