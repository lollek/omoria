use std::collections::HashMap;

use model;
use template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Options {
    Chest(template::ChestTemplate),
    Misc(template::MiscTemplate),
}

pub fn generate_dungeon_items(item_level: u8) -> model::Item {
    let usable_level: HashMap<Options, u8> = [
        (Options::Misc(template::MiscTemplate::RatSkeleton), 1),
        (Options::Misc(template::MiscTemplate::GiantCentipedeSkeleton), 1),
        (Options::Misc(template::MiscTemplate::EmptyBottle), 0),
        (Options::Misc(template::MiscTemplate::PotteryShard), 1),
        (Options::Misc(template::MiscTemplate::HumanSkeleton), 1),
        (Options::Misc(template::MiscTemplate::DwarfSkeleton), 1),
        (Options::Misc(template::MiscTemplate::ElfSkeleton), 1),
        (Options::Misc(template::MiscTemplate::GnomeSkeleton), 1),
        (Options::Misc(template::MiscTemplate::BrokenTeeth), 0),
        (Options::Misc(template::MiscTemplate::LargeBrokenBone), 1),
        (Options::Misc(template::MiscTemplate::BrokenStick), 0),
        (Options::Chest(template::ChestTemplate::SmallWoodenChest), 7),
        (Options::Chest(template::ChestTemplate::LargeWoodenChest), 15),
        (Options::Chest(template::ChestTemplate::SmallIronChest), 25),
        (Options::Chest(template::ChestTemplate::LargeIronChest), 35),
        (Options::Chest(template::ChestTemplate::SmallSteelChest), 45),
        (Options::Chest(template::ChestTemplate::LargeSteelChest), 50),
    ].iter().cloned().collect();

    let available_templates: Vec<Options> = usable_level.into_iter()
        .filter(|(_option, level)| level >= &item_level)
        .map(|(option, _level)| option)
        .collect();

    match available_templates[rand::random::<usize>() % available_templates.len()] {
        Options::Chest(chest) => template::generate_chest(item_level, chest),
        Options::Misc(misc) => template::generate_misc(item_level, misc),
    }
}
