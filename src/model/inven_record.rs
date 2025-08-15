use crate::model::Item;

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub struct InvenRecord {
    pub scost: i64,
    pub sitem: Item,
}

impl Default for InvenRecord {
    fn default() -> Self {
        InvenRecord {
            scost: 0,
            sitem: Item::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        serde_json::to_string(&InvenRecord::default()).expect("Failed to serialize InvenRecord");
    }
}