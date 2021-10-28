use std::collections::HashMap;

use model;
use template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Options {
    Chime(template::ChimeTemplate),
    Horn(template::HornTemplate),
    Staff(template::StaffTemplate),
    Wand(template::WandTemplate),
}

pub fn generate_magic_item(item_level: u8) -> model::Item {
    let usable_level: HashMap<Options, u8> = [
        (Options::Chime(template::ChimeTemplate::ChimeOfLight), 10),
        (Options::Chime(template::ChimeTemplate::ChimeOfDetectDoorsStairs), 15),
        (Options::Chime(template::ChimeTemplate::ChimeOfDetectTraps), 15),
        (Options::Chime(template::ChimeTemplate::ChimeOfTeleportation), 23),
        (Options::Chime(template::ChimeTemplate::ChimeOfThunderblast), 42),
        (Options::Chime(template::ChimeTemplate::ChimeOfSummonMonster), 10),
        (Options::Chime(template::ChimeTemplate::ChimeOfDisarming), 30),
        (Options::Chime(template::ChimeTemplate::ChimeOfAggravation), 15),
        (Options::Chime(template::ChimeTemplate::ChimeOfSlowMonster), 15),
        (Options::Chime(template::ChimeTemplate::ChimeOfSootheMonster), 15),
        (Options::Chime(template::ChimeTemplate::ChimeOfCureLightWound), 10),
        (Options::Chime(template::ChimeTemplate::ChimeOfChanging), 46),
        (Options::Chime(template::ChimeTemplate::ChimeOfRemoveCurse), 47),
        (Options::Chime(template::ChimeTemplate::ChimeOfCuring), 27),
        (Options::Chime(template::ChimeTemplate::ChimeOfDispelEvil), 49),
        (Options::Chime(template::ChimeTemplate::ChimeOfDarkness), 20),

        (Options::Horn(template::HornTemplate::HornOfBubbles), 15),
        (Options::Horn(template::HornTemplate::HornOfCalling), 10),
        (Options::Horn(template::HornTemplate::HornOfSoftSounds), 8),
        (Options::Horn(template::HornTemplate::HornOfBlasting), 49),
        (Options::Horn(template::HornTemplate::HornOfCold), 40),
        (Options::Horn(template::HornTemplate::HornOfHeat), 40),
        (Options::Horn(template::HornTemplate::HornOfGas), 35),
        (Options::Horn(template::HornTemplate::HornOfRecall), 30),
        (Options::Horn(template::HornTemplate::HornOfChaos), 43),
        (Options::Horn(template::HornTemplate::HornOfGlue), 20),
        (Options::Horn(template::HornTemplate::HornOfValhalla), 50),
        (Options::Horn(template::HornTemplate::HornOfTritons), 15),
        (Options::Horn(template::HornTemplate::HornOfFog), 25),

        (Options::Staff(template::StaffTemplate::StaffOfLight), 5),
        (Options::Staff(template::StaffTemplate::StaffOfDoorStairLocation), 10),
        (Options::Staff(template::StaffTemplate::StaffOfTrapLocation), 10),
        (Options::Staff(template::StaffTemplate::StaffOfTreasureLocation), 5),
        (Options::Staff(template::StaffTemplate::StaffOfObjectLocation), 5),
        (Options::Staff(template::StaffTemplate::StaffOfTeleportation), 20),
        (Options::Staff(template::StaffTemplate::StaffOfEarthquakes), 40),
        (Options::Staff(template::StaffTemplate::StaffOfSummoning), 10),
        (Options::Staff(template::StaffTemplate::StaffOfDestruction), 50),
        (Options::Staff(template::StaffTemplate::StaffOfStarlite), 20),
        (Options::Staff(template::StaffTemplate::StaffOfHasteMonsters), 10),
        (Options::Staff(template::StaffTemplate::StaffOfSlowMonsters), 10),
        (Options::Staff(template::StaffTemplate::StaffOfSleepMonsters), 10),
        (Options::Staff(template::StaffTemplate::StaffOfCureLightWounds), 5),
        (Options::Staff(template::StaffTemplate::StaffOfDetectInvisible), 5),
        (Options::Staff(template::StaffTemplate::StaffOfSpeed), 40),
        (Options::Staff(template::StaffTemplate::StaffOfSlowness), 40),
        (Options::Staff(template::StaffTemplate::StaffOfMassPolymorph), 46),
        (Options::Staff(template::StaffTemplate::StaffOfRemoveCurse), 47),
        (Options::Staff(template::StaffTemplate::StaffOfDetectEvil), 20),
        (Options::Staff(template::StaffTemplate::StaffOfCuring), 25),
        (Options::Staff(template::StaffTemplate::StaffOfDispelEvil), 49),
        (Options::Staff(template::StaffTemplate::StaffOfDarkness), 5),
        (Options::Staff(template::StaffTemplate::StaffOfIdentify), 20),

        (Options::Wand(template::WandTemplate::WandOfProbing), 30),
        (Options::Wand(template::WandTemplate::WandOfLight), 0),
        (Options::Wand(template::WandTemplate::WandOfLightningBolts), 15),
        (Options::Wand(template::WandTemplate::WandOfFrostBolts), 20),
        (Options::Wand(template::WandTemplate::WandOfFireBolts), 30),
        (Options::Wand(template::WandTemplate::WandOfStoneToMud), 12),
        (Options::Wand(template::WandTemplate::WandOfPolymorph), 20),
        (Options::Wand(template::WandTemplate::WandOfHealMonster), 2),
        (Options::Wand(template::WandTemplate::WandOfHasteMonster), 2),
        (Options::Wand(template::WandTemplate::WandOfSlowMonster), 2),
        (Options::Wand(template::WandTemplate::WandOfConfuseMonster), 2),
        (Options::Wand(template::WandTemplate::WandOfSleepMonster), 7),
        (Options::Wand(template::WandTemplate::WandOfDrainLife), 50),
        (Options::Wand(template::WandTemplate::WandOfTrapDoorDestruction), 12),
        (Options::Wand(template::WandTemplate::WandOfMagicMissile), 2),
        (Options::Wand(template::WandTemplate::WandOfWallBuilding), 25),
        (Options::Wand(template::WandTemplate::WandOfCloneMonster), 2),
        (Options::Wand(template::WandTemplate::WandOfTeleportAway), 20),
        (Options::Wand(template::WandTemplate::WandOfDisarming), 20),
        (Options::Wand(template::WandTemplate::WandOfLightningBalls), 35),
        (Options::Wand(template::WandTemplate::WandOfColdBalls), 40),
        (Options::Wand(template::WandTemplate::WandOfFireBalls), 50),
        (Options::Wand(template::WandTemplate::WandOfStinkingCloud), 5),
        (Options::Wand(template::WandTemplate::WandOfAcidBalls), 48),
        (Options::Wand(template::WandTemplate::WandOfWonder), 2),
    ].iter().cloned().collect();

    let available_templates: Vec<Options> = usable_level.into_iter()
        .filter(|(_option, level)| level >= &item_level)
        .map(|(option, _level)| option)
        .collect();

    match available_templates[rand::random::<usize>() % available_templates.len()] {
        Options::Chime(chime) => template::generate_chime(item_level, chime),
        Options::Horn(horn) => template::generate_horn(item_level, horn),
        Options::Staff(staff) => template::generate_staff(item_level, staff),
        Options::Wand(wand) => template::generate_wand(item_level, wand),
    }
}


