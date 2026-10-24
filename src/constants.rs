pub static SAVE_FOLDER: &str = "save";
pub static DATA_FOLDER: &str = "data";
pub static OMORIA_VERSION: &str = env!("OMORIA_VERSION");
pub static IMORIA_VERSION: &str = "4.85.22";

// Number of Cave tiles on y-axis
pub const MAX_HEIGHT: usize = 66;

// Number of Cave tiles on x-axis
pub const MAX_WIDTH: usize = 198;

// Max number of items in a store
pub const STORE_INVEN_MAX: usize = 24;

// Max number of items or creatures allocated
pub const MAX_MALLOC: usize = 101;

// Max number of objects in game
pub const MAX_OBJECTS: usize = 473;

// Max number of treasures
pub const MAX_TALLOC: usize = 225;

// Number of stores
pub const MAX_STORES: usize = 12;
pub const MAX_UNNAMED: usize = 6;

// Level of items generated in stores
pub const STORE_ITEM_LEVEL: u8 = 7;

// Max depth level of items in stores
pub const STORE_MAX_ITEM_LEVEL: u8 = 10;

pub const MAX_PLAYER_LEVEL: usize = 40;

#[cfg(test)]
mod tests {
    use super::OMORIA_VERSION;

    #[test]
    fn omoria_version_includes_a_compile_timestamp() {
        let (prefix, timestamp) = OMORIA_VERSION
            .rsplit_once('.')
            .expect("version should separate its timestamp with a dot");

        assert_eq!(prefix, "5-alpha", "version should retain the alpha prefix");
        assert_eq!(timestamp.len(), 12, "timestamp should use YYYYMMDDHHMM");
        assert!(
            timestamp.bytes().all(|byte| byte.is_ascii_digit()),
            "timestamp should contain only digits"
        );
    }
}
