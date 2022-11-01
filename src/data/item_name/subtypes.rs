use std::borrow::Cow;

use crate::conversion::item_subtype::from_i64;
use crate::model::item_subtype::{
    ChestSubType, ItemSubType, JewelrySubType, LightSourceSubType, MiscObjectSubType,
    MiscUsableSubType,
};
use crate::model::{Item, ItemType};

use super::helpers::{no_more, number_of, plural_s};

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
            if item.p1 == 1 { "" } else { "s" }
        )),
    ]
    .join("")
}
