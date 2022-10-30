use crate::model::item_subtype::ChestSubType;

pub fn from_usize(subtype: usize) -> Option<ChestSubType> {
    match subtype {
        1 => Some(ChestSubType::SmallWoodenChest),
        4 => Some(ChestSubType::LargeWoodenChest),
        7 => Some(ChestSubType::SmallIronChest),
        10 => Some(ChestSubType::LargeIronChest),
        13 => Some(ChestSubType::SmallSteelChest),
        16 => Some(ChestSubType::LargeSteelChest),
        _ => None,
    }
}

pub fn to_usize(chest: ChestSubType) -> usize {
    match chest {
        ChestSubType::SmallWoodenChest => 1,
        ChestSubType::LargeWoodenChest => 4,
        ChestSubType::SmallIronChest => 7,
        ChestSubType::LargeIronChest => 10,
        ChestSubType::SmallSteelChest => 13,
        ChestSubType::LargeSteelChest => 16,
    }
}
