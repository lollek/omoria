use std::collections::HashMap;

use model;
use template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Options {
    Amulet(template::AmuletTemplate),
    Valuable(template::ValuableTemplate),
    Ring(template::RingTemplate),
}

pub fn generate_jewelry(item_level: u8) -> model::Item {
    let usable_level: HashMap<Options, u8> = [
    (Options::Amulet(template::AmuletTemplate::AmuletOfAdornment1), 16),
    (Options::Amulet(template::AmuletTemplate::AmuletOfAdornment2), 16),
    (Options::Amulet(template::AmuletTemplate::AmuletOfWisdom), 20),
    (Options::Amulet(template::AmuletTemplate::AmuletOfCharisma), 20),
    (Options::Amulet(template::AmuletTemplate::AmuletOfSearching), 14),
    (Options::Amulet(template::AmuletTemplate::AmuletOfTeleportation), 14),
    (Options::Amulet(template::AmuletTemplate::AmuletOfSlowDigestion), 14),
    (Options::Amulet(template::AmuletTemplate::AmuletOfResistAcid), 24),
    (Options::Amulet(template::AmuletTemplate::AmuletOfTheMagi), 50),
    (Options::Amulet(template::AmuletTemplate::AmuletOfDoom), 50),
    (Options::Amulet(template::AmuletTemplate::SilverNecklace), 0),
    (Options::Amulet(template::AmuletTemplate::GoldNecklace), 7),
    (Options::Amulet(template::AmuletTemplate::MithrilNecklace), 9),

    (Options::Valuable(template::ValuableTemplate::GemOfTeleportation), 5),
    (Options::Valuable(template::ValuableTemplate::GemOfResistCold), 14),
    (Options::Valuable(template::ValuableTemplate::GemOfResistAcid), 14),
    (Options::Valuable(template::ValuableTemplate::GemOfSeeInvisible), 40),
    (Options::Valuable(template::ValuableTemplate::GemOfStealth), 35),
    (Options::Valuable(template::ValuableTemplate::GemOfSlowDigestion), 14),
    (Options::Valuable(template::ValuableTemplate::GemOfProtectFire), 40),
    (Options::Valuable(template::ValuableTemplate::GemOfDetectMonsters), 14),
    (Options::Valuable(template::ValuableTemplate::GemOfDispelEvil), 49),
    (Options::Valuable(template::ValuableTemplate::GemOfDarkness), 7),
    (Options::Valuable(template::ValuableTemplate::GemOfAcidBalls), 50),
    (Options::Valuable(template::ValuableTemplate::GemOfDetectInvisible), 47),
    (Options::Valuable(template::ValuableTemplate::GemOfIdentify), 40),
    (Options::Valuable(template::ValuableTemplate::GemOfLight), 2),
    (Options::Valuable(template::ValuableTemplate::GemOfSummoning), 3),
    (Options::Valuable(template::ValuableTemplate::GemOfRemoveCurse), 25),
    (Options::Valuable(template::ValuableTemplate::GemOfAnnihilation), 40),
    (Options::Valuable(template::ValuableTemplate::GemOfRecall), 22),
    (Options::Valuable(template::ValuableTemplate::FineAgate), 5),
    (Options::Valuable(template::ValuableTemplate::FineDiamond), 35),
    (Options::Valuable(template::ValuableTemplate::RoughDiamond), 15),
    (Options::Valuable(template::ValuableTemplate::RoughSapphire), 7),
    (Options::Valuable(template::ValuableTemplate::FineSapphire), 12),
    (Options::Valuable(template::ValuableTemplate::SmallBagOfOpals), 10),
    (Options::Valuable(template::ValuableTemplate::SmallBagOfSapphires), 15),
    (Options::Valuable(template::ValuableTemplate::SmallPouchOfDiamonds), 45),
    (Options::Valuable(template::ValuableTemplate::LargeSackOfPearls), 25),
    (Options::Valuable(template::ValuableTemplate::LargeSackOfSapphires), 30),
    (Options::Valuable(template::ValuableTemplate::LargePouchOfDiamonds), 65),
    (Options::Valuable(template::ValuableTemplate::SmallGoldPendant), 5),
    (Options::Valuable(template::ValuableTemplate::SmallMithrilPendant), 10),
    (Options::Valuable(template::ValuableTemplate::LargeMithrilGarterBelt), 45),
    (Options::Valuable(template::ValuableTemplate::SmallSilverPendant), 5),

    (Options::Ring(template::RingTemplate::RingOfGainStrength), 30),
    (Options::Ring(template::RingTemplate::RingOfGainDexterity), 30),
    (Options::Ring(template::RingTemplate::RingOfGainConstitution), 30),
    (Options::Ring(template::RingTemplate::RingOfGainIntelligence), 30),
    (Options::Ring(template::RingTemplate::RingOfSpeed1), 50),
    (Options::Ring(template::RingTemplate::RingOfSpeed2), 5),
    (Options::Ring(template::RingTemplate::RingOfSearching), 7),
    (Options::Ring(template::RingTemplate::RingOfTeleportation), 7),
    (Options::Ring(template::RingTemplate::RingOfSlowDigestion), 7),
    (Options::Ring(template::RingTemplate::RingOfResistFire), 14),
    (Options::Ring(template::RingTemplate::RingOfResistCold), 14),
    (Options::Ring(template::RingTemplate::RingOfFeatherFalling), 7),
    (Options::Ring(template::RingTemplate::RingOfAdornment1), 7),
    (Options::Ring(template::RingTemplate::RingOfAdornment2), 7),
    (Options::Ring(template::RingTemplate::RingOfWeakness), 7),
    (Options::Ring(template::RingTemplate::RingOfLordlyProtectionFire), 50),
    (Options::Ring(template::RingTemplate::RingOfLordlyProtectionAcid), 50),
    (Options::Ring(template::RingTemplate::RingOfLordlyProtectionCold), 50),
    (Options::Ring(template::RingTemplate::RingOfWoe), 50),
    (Options::Ring(template::RingTemplate::RingOfStupidity), 20),
    (Options::Ring(template::RingTemplate::RingOfIncreaseDamage), 20),
    (Options::Ring(template::RingTemplate::RingOfIncreaseToHit), 20),
    (Options::Ring(template::RingTemplate::RingOfProtection), 7),
    (Options::Ring(template::RingTemplate::RingOfAggravateMonsters), 7),
    (Options::Ring(template::RingTemplate::RingOfSeeInvisible), 40),
    (Options::Ring(template::RingTemplate::RingOfSustainStrength), 44),
    (Options::Ring(template::RingTemplate::RingOfSustainIntelligence), 44),
    (Options::Ring(template::RingTemplate::RingOfSustainWisdom), 44),
    (Options::Ring(template::RingTemplate::RingOfSustainConstitution), 44),
    (Options::Ring(template::RingTemplate::RingOfSustainDexterity), 44),
    (Options::Ring(template::RingTemplate::RingOfSustainCharisma), 7),
    (Options::Ring(template::RingTemplate::RingOfSlaying), 50),
    (Options::Ring(template::RingTemplate::RingOfGnomekind), 40),
    ].iter().cloned().collect();

    let available_templates: Vec<Options> = usable_level.into_iter()
        .filter(|(_option, level)| level >= &item_level)
        .map(|(option, _level)| option)
        .collect();

    match available_templates[rand::random::<usize>() % available_templates.len()] {
        Options::Amulet(amulet) => template::generate_amulet(item_level, amulet),
        Options::Ring(ring) => template::generate_ring(item_level, ring),
        Options::Valuable(valuable) => template::generate_valuable(item_level, valuable),
    }
}

