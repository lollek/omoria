#[derive(PartialEq, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum Class {
    Fighter,
    Wizard,
    Cleric,
    Rogue,
    Ranger,
    Paladin,
    Druid,
    Bard,
    Adventurer,
    Monk,
    Barbarian,
}
