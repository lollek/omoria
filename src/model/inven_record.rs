use crate::model::Item;

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize, Debug, Default)]
pub struct InvenRecord {
    pub scost: i64,
    pub sitem: Item,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        serde_json::to_string(&InvenRecord::default()).expect("Failed to serialize InvenRecord");
    }
}
