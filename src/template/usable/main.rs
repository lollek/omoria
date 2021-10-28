use std::collections::HashMap;

use model;
use template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Options {
    Ammo(template::AmmunitionTemplate),
    Bag(template::BagTemplate),
    LightSource(template::LightSourceTemplate),
    MiscUsable(template::MiscUsableTemplate),
    Pick(template::PickTemplate),
}

pub fn generate_usable(item_level: u8) -> model::Item {
    let usable_level: HashMap<Options, u8> = [
        (Options::Ammo(template::AmmunitionTemplate::RoundedPebble), 0),
        (Options::Ammo(template::AmmunitionTemplate::RoundedPebble), 0),
        (Options::MiscUsable(template::MiscUsableTemplate::FlaskOfOil), 1),
        (Options::MiscUsable(template::MiscUsableTemplate::IronSpike), 1),
        (Options::LightSource(template::LightSourceTemplate::WoodenTorch), 1),
        (Options::LightSource(template::LightSourceTemplate::BrassLantern), 1),
        (Options::Ammo(template::AmmunitionTemplate::Arrow), 2),
        (Options::Ammo(template::AmmunitionTemplate::Bolt), 3),
        (Options::Ammo(template::AmmunitionTemplate::IronShot), 3),
        (Options::MiscUsable(template::MiscUsableTemplate::Statue), 5),
        (Options::LightSource(template::LightSourceTemplate::MagicTorch), 10),
        (Options::MiscUsable(template::MiscUsableTemplate::CorkedBottle), 10),
        (Options::MiscUsable(template::MiscUsableTemplate::SilverCross), 15),
        (Options::LightSource(template::LightSourceTemplate::MagicLantern), 20),
        (Options::Pick(template::PickTemplate::OrcishPick1), 20),
        (Options::Pick(template::PickTemplate::GnomishShovel), 20),
        (Options::MiscUsable(template::MiscUsableTemplate::Cross), 20),
        (Options::Bag(template::BagTemplate::BagOfDevouring), 20),
        (Options::MiscUsable(template::MiscUsableTemplate::GoldCross), 25),
        (Options::Bag(template::BagTemplate::BagOfHolding250), 35),
        (Options::Pick(template::PickTemplate::OrcishPick2), 40),
        (Options::Pick(template::PickTemplate::DwarvenShovel), 40),
        (Options::Bag(template::BagTemplate::BagOfHolding500), 45),
        (Options::MiscUsable(template::MiscUsableTemplate::MithrilCross), 45),
        (Options::Bag(template::BagTemplate::BagOfHolding1000), 50),
        (Options::Bag(template::BagTemplate::BagOfHolding1500), 50),
        (Options::Pick(template::PickTemplate::DwarvenPick), 50),
    ].iter().cloned().collect();

    let available_templates: Vec<Options> = usable_level.into_iter()
        .filter(|(_option, level)| level >= &item_level)
        .map(|(option, _level)| option)
        .collect();

    match available_templates[rand::random::<usize>() % available_templates.len()] {
        Options::Ammo(ammo) => template::generate_ammunition(item_level, ammo),
        Options::Bag(bag) => template::generate_bag(item_level, bag),
        Options::LightSource(light_source) => template::generate_light_source(item_level, light_source),
        Options::MiscUsable(misc_usable) => template::generate_misc_usable(item_level, misc_usable),
        Options::Pick(pick) => template::generate_pick(item_level, pick),
    }
}
