use libc;

use crate::model::Item;

#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
pub struct TreasureAndCoordinate {
    pub treasure: Item,
    pub y: libc::c_long,
    pub x: libc::c_long,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let treasure_and_coordinate = TreasureAndCoordinate {
            treasure: Item::default(),
            y: 0,
            x: 0,
        };
        serde_json::to_string(&treasure_and_coordinate).expect("Failed to serialize TreasureAndCoordinate");
    }
}