use crate::identification::IdentifiedSubTypes;
use crate::model::{DungeonRecord, InventoryItem, Item, MonsterRecord, PlayerRecord, TownRecord};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct SaveRecord {
    pub(crate) player: PlayerRecord,
    pub(crate) inventory: Vec<InventoryItem>,
    pub(crate) equipment: Vec<Item>,
    pub(crate) town: TownRecord,
    pub(crate) dungeon: DungeonRecord,
    pub(crate) identified: IdentifiedSubTypes,
    pub(crate) monsters: MonsterRecord,
}

impl Default for SaveRecord {
    fn default() -> Self {
        SaveRecord {
            player: PlayerRecord::default(),
            inventory: Vec::default(),
            equipment: Vec::default(),
            town: TownRecord::default(),
            dungeon: DungeonRecord::default(),
            identified: IdentifiedSubTypes::default(),
            monsters: MonsterRecord::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serializeable() {
        serde_json::to_string(&SaveRecord::default()).expect("Failed to serialize player");
    }
}
