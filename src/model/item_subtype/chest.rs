#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChestSubType {
    //  DeadHumanBody,
    SmallWoodenChest,
    LargeWoodenChest,
    SmallIronChest,
    LargeIronChest,
    SmallSteelChest,
    LargeSteelChest,
}
