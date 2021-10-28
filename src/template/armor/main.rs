use std::collections::HashMap;

use model;
use template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Options {
    Armor(template::ArmorTemplate),
    Boots(template::BootsTemplate),
    Belt(template::BeltTemplate),
    Bracers(template::BracersTemplate),
    Cloak(template::CloakTemplate),
    Gloves(template::GlovesTemplate),
    Helm(template::HelmTemplate),
    Shield(template::ShieldTemplate),
}

pub fn generate_armor_types(item_level: u8) -> model::Item {
    let usable_level: HashMap<Options, u8> = [
        (Options::Boots(template::BootsTemplate::SoftLeatherShoes), 1),
        (Options::Boots(template::BootsTemplate::SoftLeatherBoots), 4),
        (Options::Boots(template::BootsTemplate::HardLeatherBoots), 6),
        (Options::Boots(template::BootsTemplate::Sandals), 1),
        (Options::Boots(template::BootsTemplate::ChainBoots), 10),
        (Options::Boots(template::BootsTemplate::LightPlatedBoots), 16),
        (Options::Boots(template::BootsTemplate::SharkskinBoots), 30),
        (Options::Boots(template::BootsTemplate::DemonhideBoots), 40),
        (Options::Boots(template::BootsTemplate::WyrmhideBoot), 50),

        (Options::Belt(template::BeltTemplate::Sash), 0),
        (Options::Belt(template::BeltTemplate::LightBelt), 0),
        (Options::Belt(template::BeltTemplate::Belt), 0),
        (Options::Belt(template::BeltTemplate::HeavyBelt), 6),
        (Options::Belt(template::BeltTemplate::LightPlatedBelt), 15),
        (Options::Belt(template::BeltTemplate::SharkskinBelt), 30),
        (Options::Belt(template::BeltTemplate::DemonhideBelt), 40),
        (Options::Belt(template::BeltTemplate::WyrmhideBelt), 50),

        (Options::Armor(template::ArmorTemplate::AugmentedChainMail), 30),
        (Options::Armor(template::ArmorTemplate::BarChainMail), 34),
        (Options::Armor(template::ArmorTemplate::BronzePlateMail), 32),
        (Options::Armor(template::ArmorTemplate::ChainMail), 26),
        (Options::Armor(template::ArmorTemplate::CoolSetOfThreads), 255),
        (Options::Armor(template::ArmorTemplate::DemonhideArmor), 255),
        (Options::Armor(template::ArmorTemplate::DoubleChainMail), 28),
        (Options::Armor(template::ArmorTemplate::DuskShroud), 30),
        (Options::Armor(template::ArmorTemplate::ElvenChainMail), 255),
        (Options::Armor(template::ArmorTemplate::FilthyNagaHideArmor), 255),
        (Options::Armor(template::ArmorTemplate::FilthyRags), 0),
        (Options::Armor(template::ArmorTemplate::FullPlateArmor), 48),
        (Options::Armor(template::ArmorTemplate::HardLeatherArmor), 5),
        (Options::Armor(template::ArmorTemplate::HardLeatherRingMail), 12),
        (Options::Armor(template::ArmorTemplate::HardStuddedLeather), 7),
        (Options::Armor(template::ArmorTemplate::LacqueredPlate), 40),
        (Options::Armor(template::ArmorTemplate::LaminatedArmor), 38),
        (Options::Armor(template::ArmorTemplate::LeatherBrigantineArmor), 20),
        (Options::Armor(template::ArmorTemplate::LeatherScaleMail), 16),
        (Options::Armor(template::ArmorTemplate::MetalBrigandineArmor), 36),
        (Options::Armor(template::ArmorTemplate::MetalLamellarArmor), 44),
        (Options::Armor(template::ArmorTemplate::MetalScaleMail), 24),
        (Options::Armor(template::ArmorTemplate::MithrilChainMail), 255),
        (Options::Armor(template::ArmorTemplate::MithrilPlateArmor), 255),
        (Options::Armor(template::ArmorTemplate::PartialPlateArmor), 42),
        (Options::Armor(template::ArmorTemplate::Robe), 1),
        (Options::Armor(template::ArmorTemplate::RustyChainMail), 26),
        (Options::Armor(template::ArmorTemplate::SoftLeatherArmor), 2),
        (Options::Armor(template::ArmorTemplate::SoftLeatherRingMail), 10),
        (Options::Armor(template::ArmorTemplate::SoftStuddedLeather), 3),
        (Options::Armor(template::ArmorTemplate::StonePlateArmor), 255),
        (Options::Armor(template::ArmorTemplate::WovenCordArmor), 7),
        (Options::Armor(template::ArmorTemplate::WyrmhideArmor), 50),

        (Options::Bracers(template::BracersTemplate::BracersOfProtection), 35),
        (Options::Bracers(template::BracersTemplate::BracersOfDefense), 40),
        (Options::Bracers(template::BracersTemplate::BracersOfShielding), 45),
        (Options::Bracers(template::BracersTemplate::MithrilBracers), 50),
        (Options::Bracers(template::BracersTemplate::AdamantiteBracers), 55),
        (Options::Bracers(template::BracersTemplate::BracersOfWeaponAttraction), 30),
        (Options::Bracers(template::BracersTemplate::SilverBraceletOfWarding), 50),
        (Options::Bracers(template::BracersTemplate::SilverBracelet), 2),
        (Options::Bracers(template::BracersTemplate::GoldBracelet), 5),
        (Options::Bracers(template::BracersTemplate::PlatinumBracelet), 8),
        (Options::Bracers(template::BracersTemplate::LeatherBracers), 1),
        (Options::Bracers(template::BracersTemplate::StuddedLeatherBracers), 5),
        (Options::Bracers(template::BracersTemplate::LightPlatedBracers), 15),
        (Options::Bracers(template::BracersTemplate::SharkskinBracers), 30),
        (Options::Bracers(template::BracersTemplate::DemonhideBracers), 40),
        (Options::Bracers(template::BracersTemplate::WyrmhideBracers), 50),
        (Options::Bracers(template::BracersTemplate::ChainmailBracers), 25),
        (Options::Bracers(template::BracersTemplate::LamellarBracers), 44),

        (Options::Cloak(template::CloakTemplate::LightCloak), 0),
        (Options::Cloak(template::CloakTemplate::HeavyCloak), 0),
        (Options::Cloak(template::CloakTemplate::SharkskinCloak), 30),
        (Options::Cloak(template::CloakTemplate::DemonhideCloak), 40),
        (Options::Cloak(template::CloakTemplate::WyrmhideCloak), 50),

        (Options::Gloves(template::GlovesTemplate::LeatherGloves), 1),
        (Options::Gloves(template::GlovesTemplate::HeavyGloves), 5),
        (Options::Gloves(template::GlovesTemplate::ClothGloves), 1),
        (Options::Gloves(template::GlovesTemplate::ChainGloves), 10),
        (Options::Gloves(template::GlovesTemplate::LightGauntlets), 16),
        (Options::Gloves(template::GlovesTemplate::HeavyGauntlets), 16),
        (Options::Gloves(template::GlovesTemplate::SharkskinGloves), 30),
        (Options::Gloves(template::GlovesTemplate::WarGauntlets), 30),
        (Options::Gloves(template::GlovesTemplate::DemonhideGloves), 40),
        (Options::Gloves(template::GlovesTemplate::WyrmhideGloves), 50),

        (Options::Helm(template::HelmTemplate::ClothHat), 1),
        (Options::Helm(template::HelmTemplate::SoftLeatherCap), 2),
        (Options::Helm(template::HelmTemplate::HardLeatherCap), 5),
        (Options::Helm(template::HelmTemplate::MetalCap), 10),
        (Options::Helm(template::HelmTemplate::FullHelm), 20),
        (Options::Helm(template::HelmTemplate::GreatHelm), 30),
        (Options::Helm(template::HelmTemplate::WingedHelm), 50),
        (Options::Helm(template::HelmTemplate::SilverCrown), 20),
        (Options::Helm(template::HelmTemplate::SilverMask), 30),
        (Options::Helm(template::HelmTemplate::GoldenCrown), 40),
        (Options::Helm(template::HelmTemplate::GoldenMask), 50),
        (Options::Helm(template::HelmTemplate::JewelEncrustedCrown), 75),

        (Options::Shield(template::ShieldTemplate::SmallLeatherShield), 3),
        (Options::Shield(template::ShieldTemplate::MediumLeatherShield), 8),
        (Options::Shield(template::ShieldTemplate::LargeLeatherShield), 15),
        (Options::Shield(template::ShieldTemplate::Buckler), 10),
        (Options::Shield(template::ShieldTemplate::KiteShield), 20),
        (Options::Shield(template::ShieldTemplate::TowerShield), 30),
        (Options::Shield(template::ShieldTemplate::SharkskinShield), 30),
        (Options::Shield(template::ShieldTemplate::DemonhideShield), 40),
        (Options::Shield(template::ShieldTemplate::WyrmhideShield), 50),
    ].iter().cloned().collect();

    let available_templates: Vec<Options> = usable_level.into_iter()
        .filter(|(_option, level)| level >= &item_level)
        .map(|(option, _level)| option)
        .collect();

    match available_templates[rand::random::<usize>() % available_templates.len()] {
        Options::Armor(armor) => template::generate_armor(item_level, armor),
        Options::Belt(belt) => template::generate_belt(item_level, belt),
        Options::Boots(boots) => template::generate_boots(item_level, boots),
        Options::Bracers(bracers) => template::generate_bracers(item_level, bracers),
        Options::Cloak(cloak) => template::generate_cloak(item_level, cloak),
        Options::Gloves(gloves) => template::generate_gloves(item_level, gloves),
        Options::Helm(helm) => template::generate_helm(item_level, helm),
        Options::Shield(shield) => template::generate_shield(item_level, shield),
    }
}

