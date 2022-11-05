use std::borrow::Cow;

use crate::conversion::item_subtype::from_i64;
use crate::identification;
use crate::model::item_subtype::{
    ChestSubType, GemSubType, ItemSubType, JewelrySubType, LightSourceSubType, MiscObjectSubType,
    MiscUsableSubType, BagSubType, WearableGemSubType, SlingAmmoSubType, ArrowSubType, BoltSubType,
};
use crate::model::{Item, ItemType};

use super::helpers::{no_more, number_of, p1_plural_s, plural_es, plural_s, damage, attack_bonus, full_number_of};

pub(crate) fn misc_object(item: &Item) -> String {
    vec![
        no_more(item),
        Cow::Borrowed(match from_i64(ItemType::MiscObject, item.subval) {
            Some(subtype) => match subtype {
                ItemSubType::MiscObject(MiscObjectSubType::RatSkeleton) => "rat skeleton",
                ItemSubType::MiscObject(MiscObjectSubType::GiantCentipedeSkeleton) => {
                    "giant centipede skeleton"
                }
                ItemSubType::MiscObject(MiscObjectSubType::EmptyBottle) => "empty bottle",
                ItemSubType::MiscObject(MiscObjectSubType::PotteryShard) => "shards of pottery",
                ItemSubType::MiscObject(MiscObjectSubType::HumanSkeleton) => "human skeleton",
                ItemSubType::MiscObject(MiscObjectSubType::DwarfSkeleton) => "dwarf skeleton",
                ItemSubType::MiscObject(MiscObjectSubType::ElfSkeleton) => "elf skeleton",
                ItemSubType::MiscObject(MiscObjectSubType::GnomeSkeleton) => "gnome skeleton",
                ItemSubType::MiscObject(MiscObjectSubType::BrokenTeeth) => "broken set of teeth",
                ItemSubType::MiscObject(MiscObjectSubType::LargeBrokenBone) => "large broken bone",
                ItemSubType::MiscObject(MiscObjectSubType::BrokenStick) => "broken stick",
                t => panic!("Expected misc object, got {:?}", t),
            },
            None => "alien object",
        }),
    ]
    .join("")
}

pub(crate) fn chest(item: &Item) -> String {
    vec![
        no_more(item),
        Cow::Borrowed(match from_i64(ItemType::Chest, item.subval) {
            Some(subtype) => match subtype {
                ItemSubType::Chest(ChestSubType::SmallWoodenChest) => "small wooden chest",
                ItemSubType::Chest(ChestSubType::LargeWoodenChest) => "large wooden chest",
                ItemSubType::Chest(ChestSubType::SmallIronChest) => "small iron chest",
                ItemSubType::Chest(ChestSubType::LargeIronChest) => "large iron chest",
                ItemSubType::Chest(ChestSubType::SmallSteelChest) => "small steel chest",
                ItemSubType::Chest(ChestSubType::LargeSteelChest) => "large steel chest",
                t => panic!("Expected chest, got {:?}", t),
            },
            None => "alien chest",
        }),
    ]
    .join("")
}

pub(crate) fn misc_usable(item: &Item) -> String {
    vec![
        no_more(item),
        Cow::Borrowed(match from_i64(ItemType::MiscUsable, item.subval) {
            Some(subtype) => match subtype {
                ItemSubType::MiscUsable(MiscUsableSubType::Statue) => "statue",
                ItemSubType::MiscUsable(MiscUsableSubType::SilverCross) => "silver cross",
                ItemSubType::MiscUsable(MiscUsableSubType::GoldCross) => "gold cross",
                ItemSubType::MiscUsable(MiscUsableSubType::MithrilCross) => "mithril cross",
                ItemSubType::MiscUsable(MiscUsableSubType::Cross) => "cross",
                ItemSubType::MiscUsable(MiscUsableSubType::CorkedBottle) => "corked bottle",
                t => panic!("Expected misc usable, got {:?}", t),
            },
            None => "alien usable item",
        }),
    ]
    .join("")
}

pub(crate) fn jewelry(item: &Item) -> String {
    vec![
        number_of(item),
        Cow::Borrowed(match from_i64(ItemType::Jewelry, item.subval) {
            Some(subtype) => match subtype {
                ItemSubType::Jewelry(JewelrySubType::SmallGoldPendant) => "small gold pendant",
                ItemSubType::Jewelry(JewelrySubType::SmallMithrilPendant) => {
                    "small mithril pendant"
                }
                ItemSubType::Jewelry(JewelrySubType::LargeMithrilGarterBelt) => {
                    "large mithril garter belt"
                }
                ItemSubType::Jewelry(JewelrySubType::SmallSilverPendant) => "small silver pendant",
                t => panic!("Expected jewelry, got {:?}", t),
            },
            None => "alien jewelry",
        }),
        plural_s(item),
    ]
    .join("")
}

pub(crate) fn gem(item: &Item) -> String {
    let subtype = from_i64(ItemType::Gem, item.subval)
        .unwrap_or_else(|| panic!("Subtype for item is not a gem? {:?}", item));

    let mut parts = vec![
        number_of(item),
        match subtype {
            ItemSubType::Gem(GemSubType::GemOfDetectMonsters)
            | ItemSubType::Gem(GemSubType::GemOfDispelEvil)
            | ItemSubType::Gem(GemSubType::GemOfDarkness)
            | ItemSubType::Gem(GemSubType::GemOfAcidBalls)
            | ItemSubType::Gem(GemSubType::GemOfDetectInvisible)
            | ItemSubType::Gem(GemSubType::GemOfIdentify)
            | ItemSubType::Gem(GemSubType::GemOfLight)
            | ItemSubType::Gem(GemSubType::GemOfSummoning)
            | ItemSubType::Gem(GemSubType::GemOfRemoveCurse)
            | ItemSubType::Gem(GemSubType::GemOfAnnihilation)
            | ItemSubType::Gem(GemSubType::GemOfRecall) => {
                Cow::Borrowed(if identification::is_identified(subtype) {
                    "gem"
                } else {
                    "finely cut gem"
                })
            }
            ItemSubType::Gem(GemSubType::FineAgate) => {
                Cow::from(format!("finely cut agate{}", plural_s(item)))
            }
            ItemSubType::Gem(GemSubType::FineDiamond) => {
                Cow::from(format!("finely cut diamond{}", plural_s(item)))
            }
            ItemSubType::Gem(GemSubType::RoughDiamond) => {
                Cow::from(format!("roughly cut diamond{}", plural_s(item)))
            }
            ItemSubType::Gem(GemSubType::RoughSapphire) => {
                Cow::from(format!("roughly cut sapphire{}", plural_s(item)))
            }
            ItemSubType::Gem(GemSubType::FineSapphire) => {
                Cow::from(format!("finely cut sapphire{}", plural_s(item)))
            }
            ItemSubType::Gem(GemSubType::SmallBagOfOpals) => {
                Cow::from(format!("small bag{} of opals", plural_s(item)))
            }
            ItemSubType::Gem(GemSubType::SmallBagOfSapphires) => {
                Cow::from(format!("small bag{} of sapphires", plural_s(item)))
            }
            ItemSubType::Gem(GemSubType::SmallPouchOfDiamonds) => {
                Cow::from(format!("small pouch{} of diamonds", plural_es(item)))
            }
            ItemSubType::Gem(GemSubType::LargeSackOfPearls) => {
                Cow::from(format!("large sack{} of pearls", plural_s(item)))
            }
            ItemSubType::Gem(GemSubType::LargeSackOfSapphires) => {
                Cow::from(format!("large sack{} of sapphires", plural_s(item)))
            }
            ItemSubType::Gem(GemSubType::LargePouchOfDiamonds) => {
                Cow::from(format!("large pouch{} of diamonds", plural_es(item)))
            }
            t => panic!("Expected jewelry, got {:?}", t),
        },
    ];

    if identification::is_identified(subtype) {
        parts.push(Cow::from(match subtype {
            ItemSubType::Gem(GemSubType::GemOfDetectMonsters) => " of detect monsters",
            ItemSubType::Gem(GemSubType::GemOfDispelEvil) => " of dispel evil",
            ItemSubType::Gem(GemSubType::GemOfDarkness) => " of darkness",
            ItemSubType::Gem(GemSubType::GemOfAcidBalls) => " of acid balls",
            ItemSubType::Gem(GemSubType::GemOfDetectInvisible) => " of detect invisible",
            ItemSubType::Gem(GemSubType::GemOfIdentify) => " of identify",
            ItemSubType::Gem(GemSubType::GemOfLight) => " of light",
            ItemSubType::Gem(GemSubType::GemOfSummoning) => " of summoning",
            ItemSubType::Gem(GemSubType::GemOfRemoveCurse) => " of remove curse",
            ItemSubType::Gem(GemSubType::GemOfAnnihilation) => " of annihilation",
            ItemSubType::Gem(GemSubType::GemOfRecall) => " of recall",
            _ => "",
        }))
    }

    if item.is_identified() {
        parts.push(Cow::from(match subtype {
            ItemSubType::Gem(GemSubType::GemOfDetectMonsters)
            | ItemSubType::Gem(GemSubType::GemOfDispelEvil)
            | ItemSubType::Gem(GemSubType::GemOfDarkness)
            | ItemSubType::Gem(GemSubType::GemOfAcidBalls)
            | ItemSubType::Gem(GemSubType::GemOfDetectInvisible)
            | ItemSubType::Gem(GemSubType::GemOfIdentify)
            | ItemSubType::Gem(GemSubType::GemOfLight)
            | ItemSubType::Gem(GemSubType::GemOfSummoning)
            | ItemSubType::Gem(GemSubType::GemOfRemoveCurse)
            | ItemSubType::Gem(GemSubType::GemOfAnnihilation)
            | ItemSubType::Gem(GemSubType::GemOfRecall) => {
                format!(" ({} charge{})", item.p1, p1_plural_s(item))
            }
            _ => "".to_owned(),
        }))
    }

    parts.join("")
}

pub(crate) fn bag(item: &Item) -> String {
    let subtype = from_i64(ItemType::Bag, item.subval)
        .unwrap_or_else(|| panic!("Subtype for item is not a bag? {:?}", item));

    let mut parts = vec![
        Cow::from("bag")
    ];

    if identification::is_identified(subtype) {
        parts.push(Cow::from(match subtype {
            ItemSubType::Bag(BagSubType::BagOfHolding250) => " of holding (250)",
            ItemSubType::Bag(BagSubType::BagOfHolding500) => " of holding (500)",
            ItemSubType::Bag(BagSubType::BagOfHolding1000) => " of holding (1000)",
            ItemSubType::Bag(BagSubType::BagOfHolding1500) => " of holding (1500)",
            ItemSubType::Bag(BagSubType::BagOfDevouring) => " of devouring",
            t => panic!("Expected bag, got {:?}", t),
        }));
    }

    parts.join("")
}

pub(crate) fn wearable_gem(item: &Item) -> String {
    let subtype = from_i64(ItemType::WearableGem, item.subval)
        .unwrap_or_else(|| panic!("Subtype for item is not a wearable gem? {:?}", item));


    let mut parts = vec![
        number_of(item),
        Cow::Borrowed(if identification::is_identified(subtype) {
            "gem"
        } else {
            "finely cut gem"
        }),
    ];

    if identification::is_identified(subtype) {
        parts.push(Cow::from(match subtype {
            ItemSubType::WearableGem(WearableGemSubType::GemOfTeleportation) => " of teleportation",
            ItemSubType::WearableGem(WearableGemSubType::GemOfResistCold) => " of resist cold",
            ItemSubType::WearableGem(WearableGemSubType::GemOfResistAcid) => " of resist acid",
            ItemSubType::WearableGem(WearableGemSubType::GemOfSeeInvisible) => " of see invisible",
            ItemSubType::WearableGem(WearableGemSubType::GemOfStealth) => " of stealth",
            ItemSubType::WearableGem(WearableGemSubType::GemOfSlowDigestion) => " of slow digestion",
            ItemSubType::WearableGem(WearableGemSubType::GemOfProtectFire) => " of lordly protection (FIRE)",
            _ => "",
        }))
    }

    parts.join("")
}

pub(crate) fn ammo(item: &Item) -> String {
    let mut parts = vec![
        full_number_of(item),
        match from_i64(item.item_type(), item.subval) {
            Some(subtype) => match subtype {
                ItemSubType::SlingAmmo(SlingAmmoSubType::RoundedPebble) => {
                    Cow::from("rounded pebble")
                }
                ItemSubType::SlingAmmo(SlingAmmoSubType::IronShot) => {
                    Cow::from("iron shot")
                }
                ItemSubType::Arrow(ArrowSubType::Arrow) => {
                    Cow::from("arrow")
                }
                ItemSubType::Bolt(BoltSubType::Bolt) => {
                    Cow::from("bolt")
                }
                t => panic!("Expected ammo, got {:?}", t),
            },
            None => Cow::from("alien ammo"),
        },
        plural_s(item),
        damage(item),
    ];

    if item.is_identified() {
        parts.push(attack_bonus(item));
    }
    parts.join("")
}

pub(crate) fn light_source(item: &Item) -> String {
    vec![
        number_of(item),
        match from_i64(ItemType::LightSource, item.subval) {
            Some(subtype) => match subtype {
                ItemSubType::LightSource(LightSourceSubType::WoodenTorch) => {
                    Cow::from("wooden torch")
                }
                ItemSubType::LightSource(LightSourceSubType::BrassLantern) => {
                    Cow::from("brass lantern")
                }
                ItemSubType::LightSource(LightSourceSubType::MagicTorch) => {
                    Cow::from("magic torch")
                }
                ItemSubType::LightSource(LightSourceSubType::MagicLantern) => {
                    Cow::from("magic lantern")
                }
                t => panic!("Expected light source, got {:?}", t),
            },
            None => Cow::from("alien lightsource"),
        },
        Cow::from(format!(
            " with {} turn{} of light",
            item.p1,
            p1_plural_s(item)
        )),
    ]
    .join("")
}
