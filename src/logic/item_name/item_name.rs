use std::borrow::Cow;

use item_template;
use model;
use identification;

use super::helpers;

pub fn generate_book_name<'a>(item: &model::Item, name: Cow<'a, str>) -> String {
    let mut parts = Vec::new();
    parts.push(helpers::number_of(item));
    parts.push(name);
    parts.join("")
}

pub fn generate_weapon_name<'a>(item: &model::Item, name: Cow<'a, str>) -> String {
    let mut parts = Vec::new();
    parts.push(name);
    parts.push(helpers::damage(&item));
    parts.push(helpers::attack_enchantment(&item));
    parts.push(helpers::armor(&item));
    if item.flags & 0x01598001 == 0x01598001{
        parts.push(Cow::from(" (HA)"));
    } else if item.flags & 0x07B80900 == 0x07B80900 {
        parts.push(Cow::from(" (DF)"));
    } else if item.flags & 0x00080000 == 0x00080000 && item.flags2 & 0x00000001 == 0x00000001 {
        parts.push(Cow::from(" (DB)"));
    } else if item.flags & 0x01000838 == 0x01000838 && item.flags2 & 0x00200002 == 0x00200002 {
        parts.push(Cow::from(" (SS)"));
    } else if item.flags & 0x00400000 == 0x00400000 && item.flags2 & 0x00000040 == 0x00000040 {
        parts.push(Cow::from(" (V)"));
    } else if item.flags & 0x00300960 == 0x00300960 {
        parts.push(Cow::from(" of Trollkind"));
    } else if item.flags & 0x01004000 == 0x01004000 {
        parts.push(Cow::from(" (SM)"));
    } else if item.flags & 0x00002000 == 0x00002000 {
        parts.push(Cow::from(" (SD)"));
    } else if item.flags & 0x00004000 == 0x00004000 {
        parts.push(Cow::from(" of Slay Monster"));
    } else if item.flags & 0x00008000 == 0x00008000 {
        parts.push(Cow::from(" of Slay Evil"));
    } else if item.flags & 0x00010000 == 0x00010000 {
        parts.push(Cow::from(" (SU)"));
    } else if item.flags & 0x00020000 == 0x00020000 {
        parts.push(Cow::from(" (FB)"));
    } else if item.flags & 0x00040000 == 0x00040000 {
        parts.push(Cow::from(" (FT)"));
    } else if item.flags2 & 0x00000010 == 0x00000010 {
        if let model::ItemType::Dagger(_) = item.item_type() {
            parts.push(Cow::from(" (WB)"));
        } else if let model::ItemType::Mace(_) = item.item_type() {
            parts.push(Cow::from(" (BB)"));
        } else {
            parts.push(Cow::from(" (Magic)"));
        }
    } else if item.flags2 & 0x00000004 == 0x00000004 {
        parts.push(Cow::from(" (SR)"));
    } else if item.flags2 & 0x00000040 == 0x00000040 {
        parts.push(Cow::from(" of Criticals"));
    } else if item.tohit > 4 && item.todam > 4 {
        parts.push(Cow::from(" of Slaying"));
    }
    parts.join("")
}

pub fn generate_armor_name<'a>(item: &model::Item, name: Cow<'a, str>) -> String {
    let mut parts = Vec::new();
    parts.push(name);
    if item.tohit != 0 || item.todam != 0 {
        parts.push(helpers::attack_enchantment(&item));
    }
    parts.push(helpers::armor(&item));
    if item.flags & 0x02380000 == 0x02380000 {
        parts.push(Cow::from(" (R)"));
    } else if item.flags & 0x00800007 == 0x00800007 {
        parts.push(Cow::from(" of Might"));
    } else if item.flags & 0x00800007 == 0x00800007 {
        parts.push(Cow::from(" of the Magi"));
    } else if item.flags & 0x00000030 == 0x00000030 {
        parts.push(Cow::from(" of Lordliness"));
    } else if item.flags & 0x41800040 == 0x41800040 {
        parts.push(Cow::from(" of Hobbitkind"));
    } else if item.flags & 0x01400120 == 0x01400120 {
        parts.push(Cow::from(" of Elvenkin"));
    } else if item.flags & 0x60400000 == 0x60400000 && item.flags2 & 0x00000010 == 0x00000010 {
        parts.push(Cow::from(" of Dwarvenkind"));
    } else if item.flags & 0x05800020 == 0x05800020 && item.flags2 & 0x00000010 == 0x00000010 {
        parts.push(Cow::from(" of Dryadkind"));
    } else if item.flags & 0x03B80800 == 0x03B80800 && item.flags2 & 0x00000010 == 0x00000010 {
        parts.push(Cow::from(" of Titan Strength"));
    } else if item.flags & 0x03100000 == 0x03100000 {
        if item.flags2 & 0x00000010 == 0x00000010 {
            parts.push(Cow::from(" of Storm Giant Strength"));
        } else {
            parts.push(Cow::from(" of Cloud Giant Strength"));
        }
    } else if item.flags & 0x01080000 == 0x01080000 {
        parts.push(Cow::from(" of Fire Giant Strength"));
    } else if item.flags & 0x01200000 == 0x01200000 {
        parts.push(Cow::from(" of Frost Giant Strength"));
    } else if item.flags & 0x01000000 == 0x01000000 {
        if item.p1 > 1  {
            parts.push(Cow::from(" of Stone Giant Strength"));
        } else {
            parts.push(Cow::from(" of Hill Giant Strength"));
        }
    } else if item.flags & 0x00000081 == 0x00000081 {
        parts.push(Cow::from(" of Ogre Power"));
    } else if item.flags & 0x01000040 == 0x01000040 {
        parts.push(Cow::from(" of Seeing"));
    } else if item.flags & 0x9800073F == 0x9800073F && item.flags2 & 0x80400000 == 0x80400000 {
        parts.push(Cow::from(" of **TOTAL DOOM**"));
    } else if item.flags & 0x80000001 == 0x80000001 {
        parts.push(Cow::from(" of Weakness"));
    } else if item.flags & 0x80000002 == 0x80000002 {
        parts.push(Cow::from(" of Clumsiness"));
    } else if item.flags & 0x80000008 == 0x80000008 {
        parts.push(Cow::from(" of Stupidity"));
    } else if item.flags & 0x80000010 == 0x80000010 {
        parts.push(Cow::from(" of Dullness"));
    } else if item.flags & 0x80000020 == 0x80000020 {
        parts.push(Cow::from(" of Ugliness"));
    } else if item.flags & 0x80000200 == 0x80000200 {
        parts.push(Cow::from(" of Noise"));
    } else if item.flags & 0x80000400 == 0x80000400 {
        parts.push(Cow::from(" of Teleportation"));
    } else if item.flags & 0x80001000 == 0x80001000 {
        parts.push(Cow::from(" of Slowness"));
    } else if item.flags & 0x80400000 == 0x80400000 {
        parts.push(Cow::from(" of Hunger"));
    } else if item.flags & 0x88000000 == 0x88000000 {
        parts.push(Cow::from(" of Blindness"));
    } else if item.flags & 0x90000000 == 0x90000000 {
        parts.push(Cow::from(" of Fear"));
    } else if item.flags & 0x80000000 == 0x80000000 {
        parts.push(Cow::from(" of Great Mass"));
    } else if item.flags & 0x05000000 == 0x05000000 && item.flags2 & 0x00000020 == 0x00000020 {
        parts.push(Cow::from(" of Thievery"));
    } else if item.flags2 & 0x00000010 == 0x00000010 && item.toac > 0 {
        parts.push(Cow::from(" of Deflection"));
    } else if item.flags & 0x00400080 == 0x00400080 {
        parts.push(Cow::from(" of Improved Digestion"));
    } else if item.flags & 0x00100000 == 0x00100000 {
        parts.push(Cow::from(" (RA)"));
    } else if item.flags & 0x00080000 == 0x00080000 {
        parts.push(Cow::from(" (RF)"));
    } else if item.flags & 0x00200000 == 0x00200000 {
        parts.push(Cow::from(" (RC)"));
    } else if item.flags & 0x02000000 == 0x02000000 {
        parts.push(Cow::from(" (RL)"));
    } else if item.flags & 0x00000002 == 0x00000002 {
        parts.push(Cow::from(" of the Hive"));
    } else if item.flags & 0x00001000 == 0x00001000 {
        parts.push(Cow::from(" of Speed"));
    } else if item.flags & 0x00000100 == 0x00000100 {
        parts.push(Cow::from(" of Stealth"));
    } else if item.flags & 0x00800000 == 0x00800000 {
        parts.push(Cow::from(" of Free Action"));
    } else if item.flags & 0x04000000 == 0x04000000 {
        parts.push(Cow::from(" of Slow Descent"));
    } else if item.tohit != 0 || item.todam != 0 {
        parts.push(Cow::from(" of Slaying"));
    } else if item.flags & 0x00000008 == 0x00000008 {
        parts.push(Cow::from(" of Intelligence"));
    } else if item.flags & 0x00000010 == 0x00000010 {
        parts.push(Cow::from(" of Wisdom"));
    } else if item.flags & 0x00000020 == 0x00000020 {
        parts.push(Cow::from(" of Beauty"));
    } else if item.flags & 0x40000000 == 0x40000000 {
        parts.push(Cow::from(" of Infravision"));
    } else if item.flags & 0x00000800 == 0x00000800 {
        parts.push(Cow::from(" of Regeneration"));
    }
    parts.join("")
}

fn misc_object_name(item: &model::Item, subtype: item_template::MiscTemplate) -> String {
        let mut parts = Vec::new();
        parts.push(helpers::number_of(item));
        parts.push(Cow::from(match subtype {
            item_template::MiscTemplate::RatSkeleton => "Rat Skeleton",
            item_template::MiscTemplate::GiantCentipedeSkeleton => "Giant Centipede Skeleton",
            item_template::MiscTemplate::EmptyBottle => "Empty Bottle",
            item_template::MiscTemplate::PotteryShard => "Some Shards of Pottery",
            item_template::MiscTemplate::HumanSkeleton => "Human Skeleton",
            item_template::MiscTemplate::DwarfSkeleton => "Dwarf Skeleton",
            item_template::MiscTemplate::ElfSkeleton => "Elf Skeleton",
            item_template::MiscTemplate::GnomeSkeleton => "Gnome Skeleton",
            item_template::MiscTemplate::BrokenTeeth => "Broken Set of Teeth",
            item_template::MiscTemplate::LargeBrokenBone => "Large Broken Bone",
            item_template::MiscTemplate::BrokenStick => "Broken Stick",
        }));
        parts.join("")
}

fn chest_name(item: &model::Item, subtype: item_template::ChestTemplate) -> String {
    let mut parts = Vec::new();

    parts.push(Cow::from(match subtype {
        item_template::ChestTemplate::SmallWoodenChest => "Small wooden chest",
        item_template::ChestTemplate::LargeWoodenChest => "Large wooden chest",
        item_template::ChestTemplate::SmallIronChest => "Small iron chest",
        item_template::ChestTemplate::LargeIronChest => "Large iron chest",
        item_template::ChestTemplate::SmallSteelChest => "Small steel chest",
        item_template::ChestTemplate::LargeSteelChest => "Large steel chest",
    }));

    if item.is_identified() {
        if item.subval == 5 { // Dead human body
            parts.push(Cow::from(" (Looted)"));
        } else if item.flags & 0x00000181 == 0x00000181 {
            parts.push(Cow::from(" (Multiple Traps)"));
        } else if item.flags & 0x00000071 == 0x00000071 {
            parts.push(Cow::from(" (Multiple Traps)"));
        } else if item.flags & 0x00000101 == 0x00000101 {
            parts.push(Cow::from(" (Summoning Runes)"));
        } else if item.flags & 0x00000081 == 0x00000081 {
            parts.push(Cow::from(" (Explosion Device)"));
        } else if item.flags & 0x00000041 == 0x00000041 {
            parts.push(Cow::from(" (Gas Trap)"));
        } else if item.flags & 0x00000021 == 0x00000021 {
            parts.push(Cow::from(" (Poison Needle)"));
        } else if item.flags & 0x00000011 == 0x00000011 {
            parts.push(Cow::from(" (Poison Needle)"));
        } else if item.flags & 0x00000001 == 0x00000001 {
            parts.push(Cow::from(" (Locked)"));
        }
    }
    parts.join("")
}

fn misc_name(item: &model::Item, subtype: item_template::MiscUsableTemplate) -> String {
    let mut parts = Vec::new();
    parts.push(helpers::number_of(item));
    parts.push(match subtype {
        item_template::MiscUsableTemplate::FlaskOfOil
            => Cow::from(format!("Flask{} of Oil", helpers::plural_s(&item))),
        item_template::MiscUsableTemplate::IronSpike
            => Cow::from(format!("Iron Spike{}", helpers::plural_s(&item))),
        item_template::MiscUsableTemplate::Statue
            => Cow::from(format!("Iron Statue{}", helpers::plural_s(&item))),
        item_template::MiscUsableTemplate::Teeth
            => Cow::from("Teeth"),
        item_template::MiscUsableTemplate::SilverCross
            => Cow::from(format!("Silver Cross{}", helpers::plural_es(&item))),
        item_template::MiscUsableTemplate::GoldCross
            => Cow::from(format!("Gold Cross{}", helpers::plural_es(&item))),
        item_template::MiscUsableTemplate::MithrilCross
            => Cow::from(format!("Mithril Cross{}", helpers::plural_es(&item))),
        item_template::MiscUsableTemplate::Cross
            => Cow::from(format!("Iron Cross{}", helpers::plural_es(&item))),
        item_template::MiscUsableTemplate::CorkedBottle
            => Cow::from(format!("Corked Bottle{}", helpers::plural_s(&item))),
    });

    match subtype {
        item_template::MiscUsableTemplate::Teeth => {
            if item.flags & 0x20000000 == 0x20000000 {
                parts.push(Cow::from(" from a Dragon"));
            } else if item.flags & 0x40000000 == 0x40000000 {
                parts.push(Cow::from(" of a Demon"));
            }
        },
        item_template::MiscUsableTemplate::Statue => {
            if item.flags & 0x00000100 == 0x00000100 {
                parts.push(Cow::from(" Major of Undead Summoning"));
            } else if item.flags & 0x00000200 == 0x00000200 {
                parts.push(Cow::from(" Major of Demon Summoning"));
            } else if item.flags & 0x00000400 == 0x00000400 {
                parts.push(Cow::from(" Life Giving"));
            }
        },
        item_template::MiscUsableTemplate::SilverCross |
        item_template::MiscUsableTemplate::GoldCross |
        item_template::MiscUsableTemplate::MithrilCross => {
                if item.flags & 0x00000001 == 0x00000001 {
                    parts.push(Cow::from(" of Turning"));
                } else if item.flags & 0x00000002 == 0x00000002 {
                    parts.push(Cow::from(" of Demon Dispelling"));
                }
            },
        item_template::MiscUsableTemplate::Cross => {
            if item.flags & 0x00000004 == 0x00000004 {
                parts.push(Cow::from(" of Summon Undead"));
            }
        },
        item_template::MiscUsableTemplate::CorkedBottle => {
            if item.flags & 0x00000010 == 0x00000010 {
                parts.push(Cow::from(" containing a Djinni"));
            } else if item.flags & 0x00000020 == 0x00000020 {
                parts.push(Cow::from(" containing some Demons"));
            }
        },
        _ => {},
    };
    parts.join("")
}

fn jewelry_name(item: &model::Item, subtype: item_template::JewelryTemplate) -> String {
    let mut parts = Vec::new();
    parts.push(helpers::number_of(item));
    parts.push(Cow::from(match subtype {
        item_template::JewelryTemplate::SmallGoldPendant
            => format!("Small gold pendant{}", helpers::plural_s(&item)),
        item_template::JewelryTemplate::SmallMithrilPendant
            => format!("Small mithril pendant{}", helpers::plural_s(&item)),
        item_template::JewelryTemplate::LargeMithrilGarterBelt
            => format!("Large mithril garter-belt{}", helpers::plural_s(&item)),
        item_template::JewelryTemplate::SmallSilverPendant
            => format!("Small silver pendant{}", helpers::plural_s(&item)),
    }));
    parts.join("")
}

fn bag_name(item: &model::Item, subtype: item_template::BagTemplate) -> String {
    let mut parts = Vec::new();
    parts.push("Bag");

    if identification::is_item_type_identified(item.item_type(), item.subval) {
        parts.push(match subtype {
            item_template::BagTemplate::BagOfHolding250 => " of Holding (250)",
            item_template::BagTemplate::BagOfHolding500 => " of Holding (500)",
            item_template::BagTemplate::BagOfHolding1000 => " of Holding (1000)",
            item_template::BagTemplate::BagOfHolding1500 => " of Holding (1500)",
            item_template::BagTemplate::BagOfDevouring => " of Devouring",
        });
    }
    parts.join("")
}

fn gem_name(item: &model::Item, subtype: item_template::GemTemplate) -> String {
    let mut parts = Vec::new();
    parts.push(helpers::number_of(item));

    parts.push(match subtype {
        item_template::GemTemplate::GemOfDetectMonsters |
        item_template::GemTemplate::GemOfDispelEvil |
        item_template::GemTemplate::GemOfDarkness |
        item_template::GemTemplate::GemOfAcidBalls |
        item_template::GemTemplate::GemOfDetectInvisible |
        item_template::GemTemplate::GemOfIdentify |
        item_template::GemTemplate::GemOfLight |
        item_template::GemTemplate::GemOfSummoning |
        item_template::GemTemplate::GemOfRemoveCurse |
        item_template::GemTemplate::GemOfAnnihilation |
        item_template::GemTemplate::GemOfRecall =>
            Cow::from("Finely Cut Gem"),
        item_template::GemTemplate::FineAgate =>
            Cow::from(format!("Finely cut Agate{}", helpers::plural_s(&item))),
        item_template::GemTemplate::FineDiamond =>
            Cow::from(format!("Finely cut Diamond{}", helpers::plural_s(&item))),
        item_template::GemTemplate::RoughDiamond =>
            Cow::from(format!("Rough cut Diamond{}", helpers::plural_s(&item))),
        item_template::GemTemplate::RoughSapphire =>
            Cow::from(format!("Rough cut Sapphire{}", helpers::plural_s(&item))),
        item_template::GemTemplate::FineSapphire =>
            Cow::from(format!("Finely cut Sapphire{}", helpers::plural_s(&item))),
        item_template::GemTemplate::SmallBagOfOpals =>
            Cow::from(format!("Small bag{} of Opals", helpers::plural_s(&item))),
        item_template::GemTemplate::SmallBagOfSapphires =>
            Cow::from(format!("Small bag{} of Sapphires", helpers::plural_s(&item))),
        item_template::GemTemplate::SmallPouchOfDiamonds =>
            Cow::from(format!("Small pouch{} of Diamonds", helpers::plural_s(&item))),
        item_template::GemTemplate::LargeSackOfPearls =>
            Cow::from(format!("Large sack{} of Pearls", helpers::plural_s(&item))),
        item_template::GemTemplate::LargeSackOfSapphires =>
            Cow::from(format!("Large sack{} of Sapphires", helpers::plural_s(&item))),
        item_template::GemTemplate::LargePouchOfDiamonds =>
            Cow::from(format!("Large pouch{} of Diamonds", helpers::plural_s(&item))),
    });

    if identification::is_item_type_identified(item.item_type(), item.subval) {
        parts.push(Cow::from(match subtype {
            item_template::GemTemplate::GemOfDetectMonsters => " of Detect Monsters",
            item_template::GemTemplate::GemOfDispelEvil => " of Dispel Evil",
            item_template::GemTemplate::GemOfDarkness => " of Darkness",
            item_template::GemTemplate::GemOfAcidBalls => " of Acid Balls",
            item_template::GemTemplate::GemOfDetectInvisible => " of Detect Invisible",
            item_template::GemTemplate::GemOfIdentify => " of Identify",
            item_template::GemTemplate::GemOfLight => " of Light",
            item_template::GemTemplate::GemOfSummoning => " of Summoning",
            item_template::GemTemplate::GemOfRemoveCurse => " of Remove Curse",
            item_template::GemTemplate::GemOfAnnihilation => " of Annihilation",
            item_template::GemTemplate::GemOfRecall => " of Recall",
            _ => "",
        }));
    }

    if item.is_identified() {
        parts.push(match subtype {
            item_template::GemTemplate::GemOfDetectMonsters |
            item_template::GemTemplate::GemOfDispelEvil |
            item_template::GemTemplate::GemOfDarkness |
            item_template::GemTemplate::GemOfAcidBalls |
            item_template::GemTemplate::GemOfDetectInvisible |
            item_template::GemTemplate::GemOfIdentify |
            item_template::GemTemplate::GemOfLight |
            item_template::GemTemplate::GemOfSummoning |
            item_template::GemTemplate::GemOfRemoveCurse |
            item_template::GemTemplate::GemOfAnnihilation |
            item_template::GemTemplate::GemOfRecall
                => helpers::charges(item),
            _ => Cow::from(""),
        });
    }
    parts.join("")
}

fn wearable_gem_name(item: &model::Item, subtype: item_template::WearableGemTemplate) -> String {
    let mut parts = Vec::new();
    parts.push(helpers::number_of(&item));
    parts.push(Cow::from("Finely cut Gem"));
    if identification::is_item_type_identified(item.item_type(), item.subval) {
        parts.push(Cow::from(match subtype {
            item_template::WearableGemTemplate::GemOfTeleportation => " of Teleportation",
            item_template::WearableGemTemplate::GemOfResistCold => " of Resist Cold",
            item_template::WearableGemTemplate::GemOfResistAcid => " of Resist Acid",
            item_template::WearableGemTemplate::GemOfSeeInvisible => " of See Invisible",
            item_template::WearableGemTemplate::GemOfStealth => " of Stealth",
            item_template::WearableGemTemplate::GemOfSlowDigestion => " of Slow Digestion",
            item_template::WearableGemTemplate::GemOfProtectFire => " of Lordly Protection (FIRE)",
        }));
    }
    parts.join("")
}

fn ammo_name(item: &model::Item, subtype: item_template::AmmunitionTemplate) -> String {
    let mut parts = Vec::new();
    parts.push(helpers::number_of(item));
    parts.push(Cow::from(match subtype {
        item_template::AmmunitionTemplate::Arrow =>
            format!("Arrow{}", helpers::plural_s(&item)),
        item_template::AmmunitionTemplate::Bolt =>
            format!("Bolt{}", helpers::plural_s(&item)),
        item_template::AmmunitionTemplate::RoundedPebble =>
            format!("Rounded Pebble{}", helpers::plural_s(&item)),
        item_template::AmmunitionTemplate::IronShot =>
            format!("Iron Shot{}", helpers::plural_s(&item)),
    }));
    parts.push(helpers::attack_enchantment(&item));
    parts.join("")
}

fn light_source_name(item: &model::Item, subtype: item_template::LightSourceTemplate) -> String {
    let mut parts = Vec::new();
    parts.push(helpers::number_of(&item));
    parts.push(match subtype {
        item_template::LightSourceTemplate::WoodenTorch =>
            Cow::from(format!("Wooden Torch{}", helpers::plural_es(&item))),
        item_template::LightSourceTemplate::BrassLantern =>
            Cow::from(format!("Brass Lantern{}", helpers::plural_s(&item))),
        item_template::LightSourceTemplate::MagicTorch =>
            Cow::from("Magic Torch"),
        item_template::LightSourceTemplate::MagicLantern =>
            Cow::from("Magic Lantern"),
    });
    let turns_plural = if item.p1 == 1 { "" } else { "s" };
    parts.push(Cow::from(format!(" with {} turn{} of light", item.p1, turns_plural)));
    parts.join("")
}

fn bow_name(item: &model::Item, subtype: item_template::BowTemplate) -> String {
    generate_weapon_name(item, Cow::from(match subtype {
        item_template::BowTemplate::Shortbow => "Shortbow",
        item_template::BowTemplate::HuntersBow => "Hunters Bow",
        item_template::BowTemplate::CompositeBow => "Composite Bow",
        item_template::BowTemplate::WarBow => "War Bow",
        item_template::BowTemplate::DoubleBow => "Double Bow",
        item_template::BowTemplate::SiegeBow => "Siege Bow",
        item_template::BowTemplate::WardedBow => "Warded Bow",
    }))
}

fn crossbow_name(item: &model::Item, subtype: item_template::CrossbowTemplate) -> String {
    generate_weapon_name(item, Cow::from(match subtype {
        item_template::CrossbowTemplate::SiegeCrossbow => "Siege Crossbow",
        item_template::CrossbowTemplate::Ballista => "Ballista",
        item_template::CrossbowTemplate::LightCrossbow => "Light Crossbow",
        item_template::CrossbowTemplate::HeavyCrossbow => "Heavy Crossbow",
    }))
}

fn sling_name(item: &model::Item, subtype: item_template::SlingTemplate) -> String {
    let mut parts = Vec::new();
    parts.push(Cow::from("Sling"));
    parts.push(helpers::damage(&item));
    parts.push(helpers::attack_enchantment(&item));
    parts.join("")
}

fn axe_name(item: &model::Item, subtype: item_template::AxeTemplate) -> String {
    generate_weapon_name(item, Cow::from(match subtype {
        item_template::AxeTemplate::Balestarius => "Balestarius",
        item_template::AxeTemplate::BattleAxe => "Battle Axe",
        item_template::AxeTemplate::BroadAxe => "Broad Axe",
        item_template::AxeTemplate::HandAxe => "Hand Axe",
        item_template::AxeTemplate::WarAxe => "War Axe",
        item_template::AxeTemplate::LargeAxe => "Large Axe",
        item_template::AxeTemplate::BeardedAxe => "Bearded Axe",
        item_template::AxeTemplate::SilverEdgedAxe => "Silved Edged Axe",
        item_template::AxeTemplate::ChampionAxe => "Champion Axe",
    }))
}

fn polearm_name(item: &model::Item, subtype: item_template::PolearmTemplate) -> String {
    generate_weapon_name(item, Cow::from(match subtype {
        item_template::PolearmTemplate::AwlPike => "Awl-Pike",
        item_template::PolearmTemplate::BeakedAxe => "Beaked Axe",
        item_template::PolearmTemplate::Fauchard => "Fauchard",
        item_template::PolearmTemplate::Glaive => "Glaive",
        item_template::PolearmTemplate::Halberd => "Halberd",
        item_template::PolearmTemplate::LucerneHammer => "Lucerne Hammer",
        item_template::PolearmTemplate::Pike => "Pike",
        item_template::PolearmTemplate::Spike => "Spear",
        item_template::PolearmTemplate::Lance => "Lance",
        item_template::PolearmTemplate::Javelin => "Javelin",
        item_template::PolearmTemplate::Naginata => "Naginata",
        item_template::PolearmTemplate::WarScythe => "War Scythe",
    }))
}

fn dagger_name(item: &model::Item, subtype: item_template::DaggerTemplate) -> String {
    generate_weapon_name(item, Cow::from(match subtype {
        item_template::DaggerTemplate::MainGauche =>"Main Gauche",
        item_template::DaggerTemplate::Misercorde =>"Misercorde",
        item_template::DaggerTemplate::Stiletto =>"Stiletto",
        item_template::DaggerTemplate::Bodkin =>"Bodkin",
        item_template::DaggerTemplate::BrokenDagger =>"Broken Dagger",
        item_template::DaggerTemplate::CatONineTails =>"Cat-O-Nine Tails",
        item_template::DaggerTemplate::Bilbo =>"Bilbo",
        item_template::DaggerTemplate::Baselard =>"Baselard",
        item_template::DaggerTemplate::Foil =>"Foil",
        item_template::DaggerTemplate::Rapier =>"Rapier",
        item_template::DaggerTemplate::SmallSword =>"Small Sword",
    }))
}

fn sword_name(item: &model::Item, subtype: item_template::SwordTemplate) -> String {
    generate_weapon_name(item, Cow::from(match subtype {
        item_template::SwordTemplate::Backsword => "Backsword",
        item_template::SwordTemplate::BastardSword => "Bastard Sword",
        item_template::SwordTemplate::Broadsword => "Broadsword",
        item_template::SwordTemplate::Claymore => "Claymore",
        item_template::SwordTemplate::Cutlass => "Cutlass",
        item_template::SwordTemplate::Espadon => "Espadon",
        item_template::SwordTemplate::ExecutionersSword => "Executioner's Sword",
        item_template::SwordTemplate::Flamberge => "Flamberge",
        item_template::SwordTemplate::Katana => "Katana",
        item_template::SwordTemplate::Longsword => "Longsword",
        item_template::SwordTemplate::Nodachi => "No-Dachi",
        item_template::SwordTemplate::Sabre => "Sabre",
        item_template::SwordTemplate::Zweihander => "Zweihander",
        item_template::SwordTemplate::BrokenSword => "Broken Sword",
    }))
}

fn pick_name(item: &model::Item, subtype: item_template::PickTemplate) -> String {
    generate_weapon_name(item, Cow::from(match subtype {
        item_template::PickTemplate::Pick => "Pick",
        item_template::PickTemplate::Shovel => "Shovel",
        item_template::PickTemplate::OrcishPick1 => "Orcish Pick",
        item_template::PickTemplate::OrcishPick2 => "Orcish Pick",
        item_template::PickTemplate::DwarvenPick => "Dwarven Pick",
        item_template::PickTemplate::GnomishShovel => "Gnomish Shovel",
        item_template::PickTemplate::DwarvenShovel => "Dwarven Shovel",
    }))
}

fn mace_name(item: &model::Item, subtype: item_template::MaceTemplate) -> String {
    generate_weapon_name(item, Cow::from(match subtype {
        item_template::MaceTemplate::BallAndChain => "Ball and Chain",
        item_template::MaceTemplate::WoodenClub => "Wooden Club",
        item_template::MaceTemplate::Flail => "Flail",
        item_template::MaceTemplate::GreatFlail => "Two Handed Great Flail",
        item_template::MaceTemplate::MorningStar => "Morningstar",
        item_template::MaceTemplate::Mace => "Mace",
        item_template::MaceTemplate::WarHammer => "War Hammer",
        item_template::MaceTemplate::LeadFilledMace => "Lead Filled Mace",
        item_template::MaceTemplate::IronShodQuarterstaff => "Iron Shod Quarterstaff",
        item_template::MaceTemplate::OgreMaul => "Ogre Maul",
    }))
}

fn boots_name(item: &model::Item, subtype: item_template::BootsTemplate) -> String {
    generate_armor_name(item, Cow::from(match subtype {
       item_template::BootsTemplate::SoftLeatherShoes => "Soft Leather Shoes",
       item_template::BootsTemplate::SoftLeatherBoots => "Soft Leather Boots",
       item_template::BootsTemplate::HardLeatherBoots => "Hard Leather Boots",
       item_template::BootsTemplate::Sandals => "Sandals",
       item_template::BootsTemplate::ChainBoots => "Chain Boots",
       item_template::BootsTemplate::LightPlatedBoots => "Light Plated Boots",
       item_template::BootsTemplate::SharkskinBoots => "Sharkskin Boots",
       item_template::BootsTemplate::DemonhideBoots => "Demonhide Boots",
       item_template::BootsTemplate::WyrmhideBoot => "Wyrmhide Boots",
   }))
}

fn gloves_name(item: &model::Item, subtype: item_template::GlovesTemplate) -> String {
    generate_armor_name(item, Cow::from(match subtype {
       item_template::GlovesTemplate::LeatherGloves => "Leather Gloves",
       item_template::GlovesTemplate::HeavyGloves => "Heavy Gloves",
       item_template::GlovesTemplate::ClothGloves => "Cloth Gloves",
       item_template::GlovesTemplate::ChainGloves => "Chain Gloves",
       item_template::GlovesTemplate::LightGauntlets => "Light Gauntlets",
       item_template::GlovesTemplate::HeavyGauntlets => "Heavy Gauntlets",
       item_template::GlovesTemplate::SharkskinGloves => "Sharkskin Gloves",
       item_template::GlovesTemplate::WarGauntlets => "War Gauntlets",
       item_template::GlovesTemplate::DemonhideGloves => "Demonhide Gloves",
       item_template::GlovesTemplate::WyrmhideGloves => "Wyrmhide Gloves",
   }))
}

fn cloak_name(item: &model::Item, subtype: item_template::CloakTemplate) -> String {
    generate_armor_name(item, Cow::from(match subtype {
       item_template::CloakTemplate::LightCloak => "Light Cloak",
       item_template::CloakTemplate::HeavyCloak => "Heavy Cloak",
       item_template::CloakTemplate::SharkskinCloak => "Sharkskin Cloak",
       item_template::CloakTemplate::DemonhideCloak => "Demonhide Cloak",
       item_template::CloakTemplate::WyrmhideCloak => "Wyrmhide Cloak",
   }))
}
fn helm_name(item: &model::Item, subtype: item_template::HelmTemplate) -> String {
    generate_armor_name(item, Cow::from(match subtype {
       item_template::HelmTemplate::ClothHat => "Cloth Hat",
       item_template::HelmTemplate::SoftLeatherCap => "Soft Leather Cap",
       item_template::HelmTemplate::HardLeatherCap => "Hard Leather Cap",
       item_template::HelmTemplate::MetalCap => "Metal Cap",
       item_template::HelmTemplate::FullHelm => "Full Helm",
       item_template::HelmTemplate::GreatHelm => "Great Helm",
       item_template::HelmTemplate::WingedHelm => "Winged Helm",
       item_template::HelmTemplate::SilverCrown => "Silver Crown",
       item_template::HelmTemplate::SilverMask => "Silver Mask",
       item_template::HelmTemplate::GoldenCrown => "Golden Crown",
       item_template::HelmTemplate::GoldenMask => "Golden Mask",
       item_template::HelmTemplate::JewelEncrustedCrown => "Jewel Encrusted Crown",
   }))
}

fn shield_name(item: &model::Item, subtype: item_template::ShieldTemplate) -> String {
    generate_armor_name(item, Cow::from(match subtype {
        item_template::ShieldTemplate::SmallLeatherShield => "Small Leather Shield",
        item_template::ShieldTemplate::MediumLeatherShield => "Medium Leather Shield",
        item_template::ShieldTemplate::LargeLeatherShield => "Large Leather Shield",
        item_template::ShieldTemplate::Buckler => "Buckler",
        item_template::ShieldTemplate::KiteShield => "Kite Shield",
        item_template::ShieldTemplate::TowerShield => "Tower Shield",
        item_template::ShieldTemplate::SharkskinShield => "Sharkskin Shield",
        item_template::ShieldTemplate::DemonhideShield => "Demonhide Shield",
        item_template::ShieldTemplate::WyrmhideShield => "Wyrmhide Shield",
    }))
}

fn hard_armor_name(item: &model::Item, subtype: item_template::HardArmorTemplate) -> String {
    generate_armor_name(item, Cow::from(match subtype {
       item_template::HardArmorTemplate::AugmentedChainMail => "Augmented Chain Mail",
       item_template::HardArmorTemplate::BarChainMail => "Bar Chain Mail",
       item_template::HardArmorTemplate::BronzePlateMail => "Bronze Plate Mail",
       item_template::HardArmorTemplate::ChainMail => "Chain Mail",
       item_template::HardArmorTemplate::DoubleChainMail => "Double Chain Mail",
       item_template::HardArmorTemplate::FullPlateArmor => "Full Plate Armor",
       item_template::HardArmorTemplate::LacqueredPlate => "Lacquered Plate",
       item_template::HardArmorTemplate::LaminatedArmor => "Laminated Armor",
       item_template::HardArmorTemplate::MetalBrigandineArmor => "Metal Brigandine Armor",
       item_template::HardArmorTemplate::MetalLamellarArmor => "Metal Lamellar Armor",
       item_template::HardArmorTemplate::MetalScaleMail => "Metal Scale Mail",
       item_template::HardArmorTemplate::MithrilChainMail => "Mithril Chain Mail",
       item_template::HardArmorTemplate::MithrilPlateArmor => "Mithril Plate Armor",
       item_template::HardArmorTemplate::PartialPlateArmor => "Partial Plate Armor",
       item_template::HardArmorTemplate::RustyChainMail => "Rusty Chain Mail",
       item_template::HardArmorTemplate::StonePlateArmor => "Stone Plate Armor",
   }))
}

fn soft_armor_name(item: &model::Item, subtype: item_template::SoftArmorTemplate) -> String {
    generate_armor_name(item, Cow::from(match subtype {
       item_template::SoftArmorTemplate::CoolSetOfThreads => "Cool Set of Threads",
       item_template::SoftArmorTemplate::DemonhideArmor => "Demonhide Armor",
       item_template::SoftArmorTemplate::DuskShroud => "Dusk Shroud",
       item_template::SoftArmorTemplate::ElvenChainMail => "Elven Chain Mail",
       item_template::SoftArmorTemplate::FilthyNagaHideArmor => "Filthy Naga Hide Armor",
       item_template::SoftArmorTemplate::FilthyRags => "Filthy Rags",
       item_template::SoftArmorTemplate::HardLeatherArmor => "Hard Leather Armor",
       item_template::SoftArmorTemplate::HardLeatherRingMail => "Hard Leather Ring Mail",
       item_template::SoftArmorTemplate::HardStuddedLeather => "Hard Studded Leather",
       item_template::SoftArmorTemplate::LeatherScaleMail => "Leather Scale Mail",
       item_template::SoftArmorTemplate::Robe => "Robe",
       item_template::SoftArmorTemplate::SoftLeatherArmor => "Soft Leather Armor",
       item_template::SoftArmorTemplate::SoftLeatherRingMail => "Soft Leather Ring Mail",
       item_template::SoftArmorTemplate::SoftStuddedLeather => "Soft Studded Armor",
       item_template::SoftArmorTemplate::WovenCordArmor => "Woven Cord Armor",
       item_template::SoftArmorTemplate::WyrmhideArmor => "Wyrmhide Armor",
       item_template::SoftArmorTemplate::LeatherBrigantineArmor => "Leather Brigantine Armor",
   }))
}

fn bracers_name(item: &model::Item, subtype: item_template::BracersTemplate) -> String {
    generate_armor_name(item, Cow::from(match subtype {
        item_template::BracersTemplate::BracersOfProtection =>
            if identification::is_item_type_identified(item.item_type(), item.subval) {
                "Bracers of Protection"
            } else {
                "Bracers"
            },
        item_template::BracersTemplate::BracersOfDefense =>
            if identification::is_item_type_identified(item.item_type(), item.subval) {
                "Bracers of Defense"
            } else {
                "Bracers"
            },
        item_template::BracersTemplate::BracersOfShielding =>
            if identification::is_item_type_identified(item.item_type(), item.subval) {
                "Bracers of Shielding"
            } else {
                "Bracers"
            },
        item_template::BracersTemplate::MithrilBracers => "Mithril Bracers",
        item_template::BracersTemplate::AdamantiteBracers => "Adamantite Bracers",
        item_template::BracersTemplate::BracersOfWeaponAttraction =>
            if identification::is_item_type_identified(item.item_type(), item.subval) {
                "Bracers of Weapon Attaction"
            } else {
                "Bracers"
            },
        item_template::BracersTemplate::SilverBraceletOfWarding =>
            if identification::is_item_type_identified(item.item_type(), item.subval) {
                "Silver Bracers of Warding"
            } else {
                "Silver Bracers"
            },
        item_template::BracersTemplate::SilverBracelet => "Silver Bracelet",
        item_template::BracersTemplate::GoldBracelet => "Gold Bracelet",
        item_template::BracersTemplate::PlatinumBracelet => "Platinum Bracelet",
        item_template::BracersTemplate::LeatherBracers => "Leather Bracers",
        item_template::BracersTemplate::StuddedLeatherBracers => "Studded Leather Bracers",
        item_template::BracersTemplate::LightPlatedBracers => "Light Plated Bracers",
        item_template::BracersTemplate::SharkskinBracers => "Sharkskin Bracers",
        item_template::BracersTemplate::DemonhideBracers => "Demonhide Bracers",
        item_template::BracersTemplate::WyrmhideBracers => "Wyrmhide Bracers",
        item_template::BracersTemplate::ChainmailBracers => "Chainmail Bracers",
        item_template::BracersTemplate::LamellarBracers => "Lamellar Bracers",
    }))
}

fn belt_name(item: &model::Item, subtype: item_template::BeltTemplate) -> String {
    generate_armor_name(item, Cow::from(match subtype {
       item_template::BeltTemplate::Sash => "Sash",
       item_template::BeltTemplate::LightBelt => "Light Belt",
       item_template::BeltTemplate::Belt => "Belt",
       item_template::BeltTemplate::HeavyBelt => "Heavy Belt",
       item_template::BeltTemplate::LightPlatedBelt => "Light Plated Belt",
       item_template::BeltTemplate::SharkskinBelt => "Sharkskin Belt",
       item_template::BeltTemplate::DemonhideBelt => "Demonhide Belt",
       item_template::BeltTemplate::WyrmhideBelt => "Wyrmhide Belt",
   }))
}

fn amulet_name(item: &model::Item, subtype: item_template::AmuletTemplate) -> String {
    let mut parts = Vec::new();
    parts.push(Cow::from("Amulet"));
    if identification::is_item_type_identified(item.item_type(), item.subval) {
        parts.push(match subtype {
            item_template::AmuletTemplate::AmuletOfAdornment1 => Cow::from(" of Adornment"),
            item_template::AmuletTemplate::AmuletOfAdornment2 => Cow::from(" of Adornment"),
            item_template::AmuletTemplate::AmuletOfWisdom => Cow::from(" of Wisdom"),
            item_template::AmuletTemplate::AmuletOfCharisma => Cow::from(" of Charisma"),
            item_template::AmuletTemplate::AmuletOfSearching => Cow::from(" of Searching"),
            item_template::AmuletTemplate::AmuletOfTeleportation => Cow::from(" of Teleportation"),
            item_template::AmuletTemplate::AmuletOfSlowDigestion => Cow::from(" of Slow Digestion"),
            item_template::AmuletTemplate::AmuletOfResistAcid => Cow::from(" of Resist Acid"),
            item_template::AmuletTemplate::AmuletOfTheMagi => Cow::from(" of the Magi"),
            item_template::AmuletTemplate::AmuletOfDoom => Cow::from(" of Doom"),
            item_template::AmuletTemplate::SilverNecklace =>
                Cow::from(format!("{} (Silver)", helpers::plural_s(&item))),
            item_template::AmuletTemplate::GoldNecklace =>
                Cow::from(format!("{} (Gold)", helpers::plural_s(&item))),
            item_template::AmuletTemplate::MithrilNecklace =>
                Cow::from(format!("{} (Mithril)", helpers::plural_s(&item))),
        });
    }
    if item.p1 != 0 {
        parts.push(helpers::p1(&item));
    }
    if item.tohit != 0 {
        parts.push(helpers::to_hit(&item));
    }
    if item.todam != 0 {
        parts.push(helpers::to_damage(&item));
    }
    if item.toac != 0 {
        parts.push(helpers::to_ac(&item));
    }
    parts.join("")
}

fn ring_name(item: &model::Item, subtype: item_template::RingTemplate) -> String {
    let mut parts = Vec::new();
    parts.push(Cow::from("Ring"));
    if identification::is_item_type_identified(item.item_type(), item.subval) {
        parts.push(Cow::from(match subtype {
            item_template::RingTemplate::RingOfGainStrength => " of Gain Strength",
            item_template::RingTemplate::RingOfGainDexterity => " of Gain Dexterity",
            item_template::RingTemplate::RingOfGainConstitution => " of Gain Constitution",
            item_template::RingTemplate::RingOfGainIntelligence => " of Gain Intelligence",
            item_template::RingTemplate::RingOfSpeed1 => " of Speed",
            item_template::RingTemplate::RingOfSpeed2 => " of Speed",
            item_template::RingTemplate::RingOfSearching => " of Searching",
            item_template::RingTemplate::RingOfTeleportation => " of Teleportation",
            item_template::RingTemplate::RingOfSlowDigestion => " of Slow Digestion",
            item_template::RingTemplate::RingOfResistFire => " of Resist Fire",
            item_template::RingTemplate::RingOfResistCold => " of Resist Cold",
            item_template::RingTemplate::RingOfFeatherFalling => " of Feather Falling",
            item_template::RingTemplate::RingOfAdornment1 => " of Adornment",
            item_template::RingTemplate::RingOfAdornment2 => " of Adornment",
            item_template::RingTemplate::RingOfWeakness => " of Weakness",
            item_template::RingTemplate::RingOfLordlyProtectionFire => " of Lordly Protection (Fire)",
            item_template::RingTemplate::RingOfLordlyProtectionAcid => " of Lordly Protection (Acid)",
            item_template::RingTemplate::RingOfLordlyProtectionCold => " of Lordly Protection (Cold)",
            item_template::RingTemplate::RingOfWoe => " of Woe",
            item_template::RingTemplate::RingOfStupidity => " of Stupidity",
            item_template::RingTemplate::RingOfIncreaseDamage => " of Increase Damage",
            item_template::RingTemplate::RingOfIncreaseToHit => " of Increase To-hit",
            item_template::RingTemplate::RingOfProtection => " of Protection",
            item_template::RingTemplate::RingOfAggravateMonsters => " of Aggravate Monster",
            item_template::RingTemplate::RingOfSeeInvisible => " of See Invisible",
            item_template::RingTemplate::RingOfSustainStrength => " of Sustain Strength",
            item_template::RingTemplate::RingOfSustainIntelligence => " of Sustain Intelligence",
            item_template::RingTemplate::RingOfSustainWisdom => " of Sustain Wisdom",
            item_template::RingTemplate::RingOfSustainConstitution => " of Sustain Constitution",
            item_template::RingTemplate::RingOfSustainDexterity => " of Sustain Dexterity",
            item_template::RingTemplate::RingOfSustainCharisma => " of Sustain Charisma",
            item_template::RingTemplate::RingOfSlaying => " of Slaying",
            item_template::RingTemplate::RingOfGnomekind => " of Gnomekind",
        }));
    }
    if item.p1 != 0 {
        parts.push(helpers::p1(&item));
    }
    if item.tohit != 0 {
        parts.push(helpers::to_hit(&item));
    }
    if item.todam != 0 {
        parts.push(helpers::to_damage(&item));
    }
    if item.toac != 0 {
        parts.push(helpers::to_ac(&item));
    }
    parts.join("")
}

fn staff_name(item: &model::Item, subtype: item_template::StaffTemplate) -> String {
    let mut parts = Vec::new();
    parts.push(Cow::from("Staff"));
    if identification::is_item_type_identified(item.item_type(), item.subval) {
        parts.push(Cow::from(match subtype {
            item_template::StaffTemplate::StaffOfLight => " of Light",
            item_template::StaffTemplate::StaffOfDoorStairLocation => " of Door/Stair Location",
            item_template::StaffTemplate::StaffOfTrapLocation => "of Trap Location",
            item_template::StaffTemplate::StaffOfTreasureLocation => " of Treasure Location",
            item_template::StaffTemplate::StaffOfObjectLocation => " of Object Location",
            item_template::StaffTemplate::StaffOfTeleportation => " of Teleportation",
            item_template::StaffTemplate::StaffOfEarthquakes => " of Earthquakes",
            item_template::StaffTemplate::StaffOfSummoning => " of Summoning",
            item_template::StaffTemplate::StaffOfDestruction => " of *Destruction*",
            item_template::StaffTemplate::StaffOfStarlite => " of Starlite",
            item_template::StaffTemplate::StaffOfHasteMonsters => " of Haste Monsters",
            item_template::StaffTemplate::StaffOfSlowMonsters => " of Slow Monsters",
            item_template::StaffTemplate::StaffOfSleepMonsters => " of Sleep Monsters",
            item_template::StaffTemplate::StaffOfCureLightWounds => " of Cure Light Wounds",
            item_template::StaffTemplate::StaffOfDetectInvisible => " of Detect Invisible",
            item_template::StaffTemplate::StaffOfSpeed => " of Speed",
            item_template::StaffTemplate::StaffOfSlowness => " of Slowness",
            item_template::StaffTemplate::StaffOfMassPolymorph => " of Mass Polymorph",
            item_template::StaffTemplate::StaffOfRemoveCurse => " of Remove Curse",
            item_template::StaffTemplate::StaffOfDetectEvil => " of Detect Evil",
            item_template::StaffTemplate::StaffOfCuring => " of Curing",
            item_template::StaffTemplate::StaffOfDispelEvil => " of Dispel Evil",
            item_template::StaffTemplate::StaffOfDarkness => " of Darkness",
            item_template::StaffTemplate::StaffOfIdentify => " of Identify",
        }));
    }
    parts.push(helpers::charges(&item));
    parts.join("")
}

fn wand_name(item: &model::Item, subtype: item_template::WandTemplate) -> String {
    let mut parts = Vec::new();
    parts.push(Cow::from("Wand"));
    if identification::is_item_type_identified(item.item_type(), item.subval) {
        parts.push(
            Cow::from(match subtype {
                item_template::WandTemplate::WandOfProbing => " of Probing",
                item_template::WandTemplate::WandOfLight => " of Light",
                item_template::WandTemplate::WandOfLightningBolts => "of Lightning Bolts",
                item_template::WandTemplate::WandOfFrostBolts => " of Frost Bolts",
                item_template::WandTemplate::WandOfFireBolts => " of Fire Bolts",
                item_template::WandTemplate::WandOfStoneToMud => " of Stone-to-Mud",
                item_template::WandTemplate::WandOfPolymorph => " of Polymorph",
                item_template::WandTemplate::WandOfHealMonster => " of Heal Monster",
                item_template::WandTemplate::WandOfHasteMonster => " of Haste Monster",
                item_template::WandTemplate::WandOfSlowMonster => " of Slow Monster",
                item_template::WandTemplate::WandOfConfuseMonster => " of Confuse Monster",
                item_template::WandTemplate::WandOfSleepMonster => " of Sleep Monster",
                item_template::WandTemplate::WandOfDrainLife => " of Drain Life",
                item_template::WandTemplate::WandOfTrapDoorDestruction => " of Trap/Door destruction",
                item_template::WandTemplate::WandOfMagicMissile => " of Magic Missile",
                item_template::WandTemplate::WandOfWallBuilding => " of Wall Building",
                item_template::WandTemplate::WandOfCloneMonster => " of Clone Monster",
                item_template::WandTemplate::WandOfTeleportAway => " of Teleport Away",
                item_template::WandTemplate::WandOfDisarming => " of Disarming",
                item_template::WandTemplate::WandOfLightningBalls => " of Lightning Balls",
                item_template::WandTemplate::WandOfColdBalls => " of Cold Balls",
                item_template::WandTemplate::WandOfFireBalls => " of Fire Balls",
                item_template::WandTemplate::WandOfStinkingCloud => " of Stinking Cloud",
                item_template::WandTemplate::WandOfAcidBalls => " of Acid Balls",
                item_template::WandTemplate::WandOfWonder => " of Wonder",
            }));
    }
    parts.push(helpers::charges(&item));
    parts.join("")
}

fn scroll_name(item: &model::Item, subtype: item_template::ScrollTemplate) -> String {
    let mut parts = Vec::new();
    parts.push(helpers::number_of(&item));
    parts.push(Cow::from(if item.number == 1 { "Scroll" } else { "Scrolls" }));
    if identification::is_item_type_identified(item.item_type(), item.subval) {
        parts.push(Cow::from(match subtype {
            item_template::ScrollTemplate::AggravateMonster => " of Trap/Door Destruction",
            item_template::ScrollTemplate::Blessing => " of Blessing",
            item_template::ScrollTemplate::CreateFood => " of Create Food",
            item_template::ScrollTemplate::CurseArmor => " of Curse Armor",
            item_template::ScrollTemplate::CurseWeapon => " of Curse Weapon",
            item_template::ScrollTemplate::Darkness => " of Darkness",
            item_template::ScrollTemplate::Destruction => " of Destruction",
            item_template::ScrollTemplate::DetectInvisible => " of Detect Invisible",
            item_template::ScrollTemplate::DispelUndead => " of Dispel Undead",
            item_template::ScrollTemplate::DoorCreation => " of Door Creation",
            item_template::ScrollTemplate::DoorStairLocation => " of Door/Stair Location",
            item_template::ScrollTemplate::EnchantArmor => " of Enchant Armor",
            item_template::ScrollTemplate::EnchantWeapon => " of Enchant Weapon",
            item_template::ScrollTemplate::EnchantWeaponToDam => " of Enchant Weapon To Dam",
            item_template::ScrollTemplate::EnchantWeaponToHit => " of Enchant Weapon To Hit",
            item_template::ScrollTemplate::FeignDeath => " of Feign Death",
            item_template::ScrollTemplate::Genocide => " of Genocide",
            item_template::ScrollTemplate::HolyChant => " of Holy Chant",
            item_template::ScrollTemplate::HolyPrayer => " of Holy Prayer",
            item_template::ScrollTemplate::Identify => " of Identify",
            item_template::ScrollTemplate::Light => " of Light",
            item_template::ScrollTemplate::MagicMapping => " of Magic Mapping",
            item_template::ScrollTemplate::MakeMunchies => " of Make Munchies",
            item_template::ScrollTemplate::MassGenocide => " of Mass Genocide",
            item_template::ScrollTemplate::MonsterConfusion => " of Monster Confusion",
            item_template::ScrollTemplate::ObjectDetection => " of Object Detection",
            item_template::ScrollTemplate::PhaseDoor => " of Phase Door",
            item_template::ScrollTemplate::ProtectionFromEvil => " of Protection from Evil",
            item_template::ScrollTemplate::Recharging => " of Recharging",
            item_template::ScrollTemplate::RemoveCurse => " of Remove Curse",
            item_template::ScrollTemplate::RuneOfProtection => " of Rune of Protection",
            item_template::ScrollTemplate::SleepMonster => " of Sleep Monster",
            item_template::ScrollTemplate::SummonMonster => " of Summon Monster",
            item_template::ScrollTemplate::SummonUndead => " of Summon Undead",
            item_template::ScrollTemplate::Teleport => " of Teleport",
            item_template::ScrollTemplate::TeleportLevel => " of Teleport Level",
            item_template::ScrollTemplate::TrapCreation => " of Trap Creation",
            item_template::ScrollTemplate::TrapDetection => " of Trap Detection",
            item_template::ScrollTemplate::TrapDoorDestruction => " of Trap/Door Destruction",
            item_template::ScrollTemplate::TreasureDetection => " of Treasure Detection",
            item_template::ScrollTemplate::Wishing => " of Wishing",
            item_template::ScrollTemplate::WordOfRecall => " of Word of Recall",
        }));
    }
    parts.join("")
}

fn potion_name(item: &model::Item, subtype: item_template::PotionTemplate) -> String {
    let mut parts = Vec::new();
    parts.push(helpers::number_of(item));
    parts.push(Cow::from(if item.number == 1 { "Potion" } else { "Potions" }));
    if identification::is_item_type_identified(item.item_type(), item.subval) {
        parts.push(Cow::from(match subtype {
            item_template::PotionTemplate::AppleJuice => " of Apple Juice",
            item_template::PotionTemplate::Blindness => " of Blindness",
            item_template::PotionTemplate::Boldliness => " of Boldliness",
            item_template::PotionTemplate::Charisma => " of Charisma",
            item_template::PotionTemplate::Confusion => " of Confusion",
            item_template::PotionTemplate::CureCriticalWounds => " of Cure Critical Wounds",
            item_template::PotionTemplate::CureLightWounds => " of Cure Light Wounds",
            item_template::PotionTemplate::CureSeriousWounds => " of Cure Serious Wounds",
            item_template::PotionTemplate::DetectInvisible => " of Detect Invisible",
            item_template::PotionTemplate::FleaBile => " of Flea Bile",
            item_template::PotionTemplate::GainConstitution => " of Gain Constitution",
            item_template::PotionTemplate::GainDexterity => " of Gain Dexterity",
            item_template::PotionTemplate::GainExperience => " of Gain Experience",
            item_template::PotionTemplate::GainIntelligence => " of Restore Strength",
            item_template::PotionTemplate::GainStrength => " of Gain Strength",
            item_template::PotionTemplate::GainWisdom => " of Gain Wisdom",
            item_template::PotionTemplate::HasteSelf => " of Haste Self",
            item_template::PotionTemplate::Healing => " of Healing",
            item_template::PotionTemplate::Heroism => " of Heroism",
            item_template::PotionTemplate::InfraVision => " of Infra-Vision",
            item_template::PotionTemplate::Invulnerability => " of Invulnerability",
            item_template::PotionTemplate::Learning => " of Learning",
            item_template::PotionTemplate::LoseIntelligence => " of Lose Intelligence",
            item_template::PotionTemplate::LoseMemories => " of Lose Memories",
            item_template::PotionTemplate::LoseWisdom => " of Lose Wisdom",
            item_template::PotionTemplate::NeutralizePoison => " of Neutralize Poison",
            item_template::PotionTemplate::Poison => " of Poison",
            item_template::PotionTemplate::ResistCold => " of Resist Cold",
            item_template::PotionTemplate::ResistHeat => " of Resist Heat",
            item_template::PotionTemplate::RestoreCharisma => " of Restore Charisma",
            item_template::PotionTemplate::RestoreConstitution => " of Restore Consitution",
            item_template::PotionTemplate::RestoreDexterity => " of Restore Dexterity",
            item_template::PotionTemplate::RestoreIntelligence => " of Restore Intelligence",
            item_template::PotionTemplate::RestoreLifeLevels => " of Restore Life Levels",
            item_template::PotionTemplate::RestoreMana => " of Restore Mana",
            item_template::PotionTemplate::RestoreStrength => " of Restore Strength",
            item_template::PotionTemplate::RestoreWisdom => " of Restore Wisdom",
            item_template::PotionTemplate::SaltWater => " of Salt Water",
            item_template::PotionTemplate::Sleep => " of Sleep",
            item_template::PotionTemplate::SlimeMoldJuice => " of Slime Mold Juice",
            item_template::PotionTemplate::SlowPoison => " of Slow Poison",
            item_template::PotionTemplate::Slowness => " of Slowness",
            item_template::PotionTemplate::SuperHeroism => " of Super Heroism",
            item_template::PotionTemplate::Ugliness => " of Ugliness",
            item_template::PotionTemplate::Water => " of Water",
        }))
    }
    parts.join("")
}

fn food_name(item: &model::Item, subtype: item_template::FoodTemplate) -> String {
    let mut parts = Vec::new();
    parts.push(helpers::number_of(item));
    parts.push(Cow::from(match subtype {
        item_template::FoodTemplate::Mushroom |
        item_template::FoodTemplate::Mushroom2 |
        item_template::FoodTemplate::MushroomOfPoison |
        item_template::FoodTemplate::MushroomOfBlindness |
        item_template::FoodTemplate::MushroomOfParanoia |
        item_template::FoodTemplate::MushroomOfConfusion |
        item_template::FoodTemplate::MushroomOfHallucination |
        item_template::FoodTemplate::MushroomOfCurePoison |
        item_template::FoodTemplate::MushroomOfCureBlindness |
        item_template::FoodTemplate::MushroomOfCureParanoia |
        item_template::FoodTemplate::MushroomOfCureConfusion |
        item_template::FoodTemplate::MushroomOfWeakness |
        item_template::FoodTemplate::MushroomOfUnhealth |
        item_template::FoodTemplate::MushroomOfRestoreConstitution |
        item_template::FoodTemplate::MushroomOfFirstAid |
        item_template::FoodTemplate::MushroomOfMinorCures |
        item_template::FoodTemplate::MushroomOfLightCures |
        item_template::FoodTemplate::MushroomOfRestoring |
        item_template::FoodTemplate::MushroomOfPoison2 |
        item_template::FoodTemplate::MushroomOfHallucination2 |
        item_template::FoodTemplate::MushroomOfCurePoison2 |
        item_template::FoodTemplate::MushroomOfUnhealth2 |
        item_template::FoodTemplate::MushroomOfCureSeriousWounds
            => format!("Mushroom{}", helpers::plural_s(&item)),
        item_template::FoodTemplate::PintOfFineGradeMush
            => format!("Pint{} of Fine Grade Mush", helpers::plural_s(&item)),
        item_template::FoodTemplate::RationOfFood
            => format!("Ration{} of Food", helpers::plural_s(&item)),
        item_template::FoodTemplate::HardBiscuit
            => format!("Hard Biscuit{}", helpers::plural_s(&item)),
        item_template::FoodTemplate::BeefJerky
            => format!("Strip{} of Beef Jerky", helpers::plural_s(&item)),
        item_template::FoodTemplate::FineAle
            => format!("Pint{} of Fine Ale", helpers::plural_s(&item)),
        item_template::FoodTemplate::FineWine
            => format!("Pint{} of Fine Wine", helpers::plural_s(&item)),
        item_template::FoodTemplate::ElvishWaybread
            => format!("Piece{} of Elvish Waybread", helpers::plural_s(&item)),
        item_template::FoodTemplate::Stew
            => format!("Stew{}", helpers::plural_s(&item)),
        item_template::FoodTemplate::GreenJelly
            => format!("Green Jelly{}", helpers::plural_s(&item)),
        item_template::FoodTemplate::BerriesPoisonous |
        item_template::FoodTemplate::BerriesSmurfberries |
        item_template::FoodTemplate::BerriesGoodberries
            => format!("Handful{} of Berries", helpers::plural_s(&item)),
        item_template::FoodTemplate::EyeballOfNed
            => format!("Eyeball{}", helpers::plural_s(&item)),
    }));

    if identification::is_item_type_identified(item.item_type(), item.subval) {
        parts.push(Cow::from(match subtype {
            item_template::FoodTemplate::MushroomOfPoison => " of Poison",
            item_template::FoodTemplate::MushroomOfBlindness => " of Blindness",
            item_template::FoodTemplate::MushroomOfParanoia => " of Paranoia",
            item_template::FoodTemplate::MushroomOfConfusion => " of Confusion",
            item_template::FoodTemplate::MushroomOfHallucination => " of Hallucination",
            item_template::FoodTemplate::MushroomOfCurePoison => " of Cure Poison",
            item_template::FoodTemplate::MushroomOfCureBlindness => " of Cure Blindness",
            item_template::FoodTemplate::MushroomOfCureParanoia => " of Cure Paranoia",
            item_template::FoodTemplate::MushroomOfCureConfusion => " of Cure Confusion",
            item_template::FoodTemplate::MushroomOfWeakness => " of Weakness",
            item_template::FoodTemplate::MushroomOfUnhealth => " of Unhealth",
            item_template::FoodTemplate::MushroomOfRestoreConstitution => " of Restore Constitution",
            item_template::FoodTemplate::MushroomOfFirstAid => " of First-Aid",
            item_template::FoodTemplate::MushroomOfMinorCures => " of Minor Cures",
            item_template::FoodTemplate::MushroomOfLightCures => " of Light Cures",
            item_template::FoodTemplate::MushroomOfRestoring => " of Restoring",
            item_template::FoodTemplate::MushroomOfPoison2 => " of Poison",
            item_template::FoodTemplate::MushroomOfHallucination2 => " of Hallucination",
            item_template::FoodTemplate::MushroomOfCurePoison2 => " of Cure Poison",
            item_template::FoodTemplate::MushroomOfUnhealth2 => " of Unhealth",
            item_template::FoodTemplate::MushroomOfCureSeriousWounds => " of Cure Serious Wounds",
            item_template::FoodTemplate::BerriesPoisonous => " (Poisonous)",
            item_template::FoodTemplate::BerriesSmurfberries => " (Smurfberries)",
            item_template::FoodTemplate::BerriesGoodberries => " (Goodberries)",
            item_template::FoodTemplate::EyeballOfNed => " of Ned",
            _ => "",
        }));
    }
    parts.join("")
}

fn junk_food_name(item: &model::Item, subtype: item_template::JunkFoodTemplate) -> String {
    let mut parts = Vec::new();
    parts.push(helpers::number_of(item));
    parts.push(Cow::from(match subtype {
        item_template::JunkFoodTemplate::BoxOfPiranhaCrackers =>
            format!("Box{} of Piranha Crackers", helpers::plural_s(&item)),
        item_template::JunkFoodTemplate::CanOfOrcaCola =>
            format!("Can{} of Orca-Cola", helpers::plural_s(&item)),
        item_template::JunkFoodTemplate::TwelvePoundTrollBuger =>
            format!("Twelve-Pound Troll Burger{}", helpers::plural_s(&item)),
        item_template::JunkFoodTemplate::BagOfBrontosaurusChips =>
            format!("Bag{} of Brontosaurus Chips", helpers::plural_s(&item)),
        item_template::JunkFoodTemplate::SliceOfPurpleMushroomPizza =>
            format!("Slice{} of Purple Mushroom Pizza", helpers::plural_s(&item)),
        item_template::JunkFoodTemplate::PeanutButterAndGrapeJellySandwich =>
            format!("Peanut Butter and Grape Jelly Sandwich{}", helpers::plural_s(&item)),
        item_template::JunkFoodTemplate::DragonSteak =>
            format!("Dragon Steak{}", helpers::plural_s(&item)),
        item_template::JunkFoodTemplate::VorpalBunnyThroatLozenge =>
            format!("Vorpal Bunny Throat Lozenge{}", helpers::plural_s(&item)),
        item_template::JunkFoodTemplate::DeepFriedGiantCentipede =>
            format!("Deep-Fried Giant Centipede{}", helpers::plural_s(&item)),
        item_template::JunkFoodTemplate::PintOfBeetleJuice =>
            format!("Pint{} of Beetle Juice", helpers::plural_s(&item)),
        item_template::JunkFoodTemplate::BownOfBatStew =>
            format!("Bowl{} of Bat Stew", helpers::plural_s(&item)),
        item_template::JunkFoodTemplate::JarOfPickledLeeches =>
            format!("Jar{} of Pickled Leeches", helpers::plural_s(&item)),
        item_template::JunkFoodTemplate::PackOfKittenMcNuggets =>
            format!("Pack{} of Kitten McNuggets", helpers::plural_s(&item)),
    }));
    parts.join("")
}

fn chime_name(item: &model::Item, subtype: item_template::ChimeTemplate) -> String {
    let mut parts = Vec::new();
    parts.push(Cow::from("Chime"));
    if identification::is_item_type_identified(item.item_type(), item.subval) {
        parts.push(Cow::from(match subtype {
            item_template::ChimeTemplate::ChimeOfLight => " of Light",
            item_template::ChimeTemplate::ChimeOfDetectDoorsStairs => " of Detect Doors/Stairs",
            item_template::ChimeTemplate::ChimeOfDetectTraps => " of Detect Traps",
            item_template::ChimeTemplate::ChimeOfTeleportation => " of Teleportation",
            item_template::ChimeTemplate::ChimeOfThunderblast => " of Thunderblasts",
            item_template::ChimeTemplate::ChimeOfSummonMonster => " of Summon Monster",
            item_template::ChimeTemplate::ChimeOfDisarming => " of Disarming",
            item_template::ChimeTemplate::ChimeOfAggravation => " of Aggravation",
            item_template::ChimeTemplate::ChimeOfSlowMonster => " of Slow Monster",
            item_template::ChimeTemplate::ChimeOfSootheMonster => " of Soothe Monster",
            item_template::ChimeTemplate::ChimeOfCureLightWound => " of Cure Light Wound",
            item_template::ChimeTemplate::ChimeOfChanging => " of Changing",
            item_template::ChimeTemplate::ChimeOfRemoveCurse => " of Remove Curse",
            item_template::ChimeTemplate::ChimeOfCuring => " of Curing",
            item_template::ChimeTemplate::ChimeOfDispelEvil => " of Dispel Evil",
            item_template::ChimeTemplate::ChimeOfDarkness => " of Darkness",
        }));
    }
    parts.push(helpers::charges(&item));
    parts.join("")
}

fn horn_name(item: &model::Item, subtype: item_template::HornTemplate) -> String {
    let mut parts = Vec::new();
    parts.push(Cow::from("Horn"));
    if identification::is_item_type_identified(item.item_type(), item.subval) {
        parts.push(Cow::from(match subtype {
            item_template::HornTemplate::HornOfBubbles => " of Bubbles",
            item_template::HornTemplate::HornOfCalling => " of Calling",
            item_template::HornTemplate::HornOfSoftSounds => " of Soft Sounds",
            item_template::HornTemplate::HornOfBlasting => " of *Blasting*",
            item_template::HornTemplate::HornOfCold => " of Cold",
            item_template::HornTemplate::HornOfHeat => " of Heat",
            item_template::HornTemplate::HornOfGas => " of Gas",
            item_template::HornTemplate::HornOfRecall => " of Recall",
            item_template::HornTemplate::HornOfChaos => " of *Chaos*",
            item_template::HornTemplate::HornOfGlue => " of Glue",
            item_template::HornTemplate::HornOfValhalla => " of Valhalla",
            item_template::HornTemplate::HornOfTritons => " of Tritons",
            item_template::HornTemplate::HornOfFog => " of Fog",
        }));
    }
    if item.is_identified() {
        parts.push(helpers::charges(item))
    }
    parts.join("")
}

fn magic_book_name(item: &model::Item, subtype: item_template::MagicBookTemplate) -> String {
    generate_book_name(item, Cow::from(match subtype {
        item_template::MagicBookTemplate::BeginnersMagic => "Book of Magic Spells [Beginners-Magik]",
        item_template::MagicBookTemplate::Magic1 => "Book of Magic Spells [Magik I]",
        item_template::MagicBookTemplate::Magic2 => "Book of Magic Spells [Magik II]",
        item_template::MagicBookTemplate::MagesGuideToPower => "Book of Magic Spells [The Mages Guide to Power]",
    }))
}

fn prayer_book_name(item: &model::Item, subtype: item_template::PrayerBookTemplate) -> String {
    generate_book_name(item, Cow::from(match subtype {
        item_template::PrayerBookTemplate::BeginnersHandbook => "Holy Book of Prayers [Beginners Handbook]",
        item_template::PrayerBookTemplate::WordsOfWisdom => "Holy Book of Prayers [Words of Wisdom]",
        item_template::PrayerBookTemplate::ChantsAndBlessings => "Holy Book of Prayers [Chants and Blessings]",
        item_template::PrayerBookTemplate::ExorcismAndDispelling => "Holy Book of Prayers [Exorcism and Dispelling]",
    }))
}

fn instrument_name(item: &model::Item, subtype: item_template::InstrumentTemplate) -> String {
    let mut parts = Vec::new();
    parts.push(helpers::number_of(item));
    parts.push(Cow::from(match subtype {
        item_template::InstrumentTemplate::PipesOfPeace => "Pipes of Peace",
        item_template::InstrumentTemplate::LyreOfNature => "Lyre of Nature",
        item_template::InstrumentTemplate::LuteOfTheWoods => "Lute of the Woods",
        item_template::InstrumentTemplate::HarpOfTheDruids =>"Harp of the Druids" ,
    }));
    parts.join("")
}

fn song_book_name(item: &model::Item, subtype: item_template::SongBookTemplate) -> String {
    generate_book_name(item, Cow::from(match subtype {
        item_template::SongBookTemplate::BeginnersHandbook => "Book of Bard Lyrics [Beginners Handbook]",
        item_template::SongBookTemplate::SongBook1 => "Songs of Charming [Song Book I]",
        item_template::SongBookTemplate::SongBook2 => "Ballads of Knowledge [Song Book II]",
        item_template::SongBookTemplate::GreaterSongBook => "Epics of the Bards [Greater Song Book]",
    }))
}

fn lodging_at_inn_name(item: &model::Item, subtype: item_template::LodgingAtInnTemplate) -> String {
    match subtype {
        item_template::LodgingAtInnTemplate::LodgingForOneDay => "Lodging for one day",
        item_template::LodgingAtInnTemplate::LodgingForThreeDays => "Lodging for three days",
        item_template::LodgingAtInnTemplate::LodgingForOneWeek => "Lodging for the week",
        item_template::LodgingAtInnTemplate::RoomAndBoardForOneDay => "Room and board for one day",
    }.to_string()
}

fn dungeon_feature_name(item: &model::Item, subtype: item_template::DungeonFeatureTemplate) -> String {
    let mut parts = Vec::new();
    match subtype {
        item_template::DungeonFeatureTemplate::Money => {
            parts.push(helpers::number_of(item));
            Cow::from(match item.subval {
                1 => format!(" iron piece{}", helpers::plural_s(&item)),
                2 => format!(" copper piece{}", helpers::plural_s(&item)),
                3 => format!(" iron piece{}", helpers::plural_s(&item)),
                4 => format!(" copper piece{}", helpers::plural_s(&item)),
                5 => format!(" silver piece{}", helpers::plural_s(&item)),
                6 => format!(" copper piece{}", helpers::plural_s(&item)),
                7 => format!(" silver piece{}", helpers::plural_s(&item)),
                12 => format!(" gold piece{}", helpers::plural_s(&item)),
                16 => format!(" silver piece{}", helpers::plural_s(&item)),
                18 => format!(" gold piece{}", helpers::plural_s(&item)),
                20 => format!(" platinum piece{}", helpers::plural_s(&item)),
                24 => format!(" gold piece{}", helpers::plural_s(&item)),
                28 => format!(" platinum piece{}", helpers::plural_s(&item)),
                32 => format!(" mithril piece{}", helpers::plural_s(&item)),
                50 => format!(" gold piece{}", helpers::plural_s(&item)),
                55 => format!(" platinum piece{}", helpers::plural_s(&item)),
                60 => format!(" mithril piece{}", helpers::plural_s(&item)),
            })
        },
        item_template::DungeonFeatureTemplate::UnseenTrap |
        item_template::DungeonFeatureTemplate::SeenTrap => Cow::from(match item.subval {
            1 => "an open pit",
            2 => "an arrow trap",
            3 => "a covered pit",
            4 => "a trap door",
            5 => "a gas trap",
            6 => "a loose rock",
            7 => "a dart trap",
            8 => "a strange rune",
            9 => "some loose rock",
            10 => "a gas trap",
            11 => "a strange rune",
            12 => "a blackened spot",
            13 => "some corroded rock",
            14 => "a gas trap",
            15 => "a gas trap",
            16 => "a gas trap",
            17 => "a dart trap",
            18 => "a dart trap",
            20 => "a chute",
            _ => "an unnamed trap <TODO>",
        }),
        item_template::DungeonFeatureTemplate::Rubble => Cow::from("some rubble"),
        item_template::DungeonFeatureTemplate::OpenDoor => Cow::from("an open door"),
        item_template::DungeonFeatureTemplate::ClosedDoor => Cow::from("a closed door"),
        item_template::DungeonFeatureTemplate::UpStaircase => Cow::from("an up staircase"),
        item_template::DungeonFeatureTemplate::DownStaircase => Cow::from("a down staircase"),
        item_template::DungeonFeatureTemplate::SecretDoor => Cow::from("a secret door"),
        item_template::DungeonFeatureTemplate::EntranceToStore => Cow::from(match item.subval {
            101 => "the entrance to the General Store",
            102 => "the entrance to the Armory",
            103 => "the entrance to the Weapon Smiths",
            104 => "the entrance to the Temple",
            105 => "the entrance to the Alchemy Shop",
            106 => "the entrance to the Magic Shop",
            107 => "the entrance to the Inn",
            108 => "the entrance to the Trading Post",
            109 => "the entrance to the Library",
            110 => "the entrance to the Music Shop",
            111 => "the entrance to the Insurance Shop",
            112 => "the entrance to the Bank",
            113 => "the entrance to the Gem Store",
            114 => "the entrance to the Money Exchange",
            115 => "the entrance to the Casino",
            116 => "the entrance to the All-Nite Deli",
            117 => "the entrance to a strange building",
            118 => "the entrance to a building",
            120 => "the entrance to a building",
            121 => "the entrance to a building",
            122 => "the entrance to a building",
            123 => "the entrance to a building",
            124 => "the entrance to a building",
        }),
        item_template::DungeonFeatureTemplate::UpSteepStaircase => Cow::from("a steep staircase"),
        item_template::DungeonFeatureTemplate::DownSteepStaircase => Cow::from("a steep staircase"),
        item_template::DungeonFeatureTemplate::Whirlpool => Cow::from("a whirlpool"),
    };
    parts.join("")
}


pub fn generate(item: &model::Item) -> String {
    match item.item_type() {
            model::ItemType::MiscObject(subtype) => misc_object_name(&item, subtype),
            model::ItemType::Chest(subtype) => chest_name(&item, subtype),
            model::ItemType::MiscUsable(subtype) => misc_name(&item, subtype),
            model::ItemType::Spike(subtype) => misc_name(&item, subtype),
            model::ItemType::FlaskOfOil(subtype) => misc_name(&item, subtype),
            model::ItemType::Jewelry(subtype) => jewelry_name(&item, subtype),
            model::ItemType::Bag(subtype) => bag_name(&item, subtype),
            model::ItemType::Gem(subtype) => gem_name(&item, subtype),
            model::ItemType::WearableGem(subtype) => wearable_gem_name(&item, subtype),
            model::ItemType::SlingAmmo(subtype) => ammo_name(&item, subtype),
            model::ItemType::Bolt(subtype) => ammo_name(&item, subtype),
            model::ItemType::Arrow(subtype) => ammo_name(&item, subtype),
            model::ItemType::LightSource(subtype) => light_source_name(&item, subtype),
            model::ItemType::Bow(subtype) => bow_name(&item, subtype),
            model::ItemType::Crossbow(subtype) => crossbow_name(&item, subtype),
            model::ItemType::Sling(subtype) => sling_name(&item, subtype),
            model::ItemType::Axe(subtype) => axe_name(&item, subtype),
            model::ItemType::Polearm(subtype) => polearm_name(&item, subtype),
            model::ItemType::Dagger(subtype) => dagger_name(&item, subtype),
            model::ItemType::Sword(subtype) => sword_name(&item, subtype),
            model::ItemType::Pick(subtype) => pick_name(&item, subtype),
            model::ItemType::Mace(subtype) => mace_name(&item, subtype),
            model::ItemType::Boots(subtype) => boots_name(&item, subtype),
            model::ItemType::Gloves(subtype) => gloves_name(&item, subtype),
            model::ItemType::Cloak(subtype) => cloak_name(&item, subtype),
            model::ItemType::Helm(subtype) => helm_name(&item, subtype),
            model::ItemType::Shield(subtype) => shield_name(&item, subtype),
            model::ItemType::HardArmor(subtype) => hard_armor_name(&item, subtype),
            model::ItemType::SoftArmor(subtype) => soft_armor_name(&item, subtype),
            model::ItemType::Bracers(subtype) => bracers_name(&item, subtype),
            model::ItemType::Belt(subtype) => belt_name(&item, subtype),
            model::ItemType::Amulet(subtype) => amulet_name(&item, subtype),
            model::ItemType::Ring(subtype) => ring_name(&item, subtype),
            model::ItemType::Staff(subtype) => staff_name(&item, subtype),
            model::ItemType::Wand(subtype) => wand_name(&item, subtype),
            model::ItemType::Scroll(subtype) => scroll_name(&item, subtype),
            model::ItemType::Potion(subtype) => potion_name(&item, subtype),
            model::ItemType::Food(subtype) => food_name(&item, subtype),
            model::ItemType::JunkFood(subtype) => junk_food_name(&item, subtype),
            model::ItemType::Chime(subtype) => chime_name(&item, subtype),
            model::ItemType::Horn(subtype) => horn_name(&item, subtype),
            model::ItemType::MagicBook(subtype) => magic_book_name(&item, subtype),
            model::ItemType::PrayerBook(subtype) => prayer_book_name(&item, subtype),
            model::ItemType::Instrument(subtype) => instrument_name(&item, subtype),
            model::ItemType::SongBook(subtype) => song_book_name(&item, subtype),
            model::ItemType::LodgingAtInn(subtype) => lodging_at_inn_name(&item, subtype),
            model::ItemType::Money(subtype) => dungeon_feature_name(&item, subtype),
            model::ItemType::UnseenTrap(subtype) => dungeon_feature_name(&item, subtype),
            model::ItemType::SeenTrap(subtype) => dungeon_feature_name(&item, subtype),
            model::ItemType::Rubble(subtype) => dungeon_feature_name(&item, subtype),
            model::ItemType::OpenDoor(subtype) => dungeon_feature_name(&item, subtype),
            model::ItemType::ClosedDoor(subtype) => dungeon_feature_name(&item, subtype),
            model::ItemType::UpStaircase(subtype) => dungeon_feature_name(&item, subtype),
            model::ItemType::DownStaircase(subtype) => dungeon_feature_name(&item, subtype),
            model::ItemType::SecretDoor(subtype) => dungeon_feature_name(&item, subtype),
            model::ItemType::EntranceToStore(subtype) => dungeon_feature_name(&item, subtype),
            model::ItemType::UpSteepStaircase(subtype) => dungeon_feature_name(&item, subtype),
            model::ItemType::DownSteepStaircase(subtype) => dungeon_feature_name(&item, subtype),
            model::ItemType::Whirlpool(subtype) => dungeon_feature_name(&item, subtype),
    }
}
