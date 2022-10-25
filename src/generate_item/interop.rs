use crate::model::Item;
use crate::generate_item;

#[no_mangle]
pub extern fn generate_item_level_for_dungeon_level(dungeon_level: u8, tries: u8) -> u8 {
    generate_item::generate_item_level_for_dungeon_level(dungeon_level, tries)
}

#[no_mangle]
pub extern fn generate_item_for_dungeon_level(dungeon_level: u8) -> Item {
    generate_item::generate_item_for_dungeon_level(dungeon_level)
}

#[no_mangle]
pub extern fn generate_item_for_item_level(item_level: u8) -> Item {
    generate_item::generate_item_for_item_level(item_level)
}

#[no_mangle]
pub extern fn generate_item_for_general_store() -> Item {
    generate_item::generate_item_for_general_store()
}

#[no_mangle]
pub extern fn generate_item_for_armorsmith() -> Item {
    generate_item::generate_item_for_armorsmith()
}

#[no_mangle]
pub extern fn generate_item_for_weaponsmith() -> Item {
    generate_item::generate_item_for_weaponsmith()
}

#[no_mangle]
pub extern fn generate_item_for_temple() -> Item {
    generate_item::generate_item_for_temple()
}

#[no_mangle]
pub extern fn generate_item_for_alchemist_store() -> Item {
    generate_item::generate_item_for_alchemist_store()
}

#[no_mangle]
pub extern fn generate_item_for_magic_store() -> Item {
    generate_item::generate_item_for_magic_store()
}

#[no_mangle]
pub extern fn generate_item_for_inn() -> Item {
    generate_item::generate_item_for_inn()
}

#[no_mangle]
pub extern fn generate_item_for_library() -> Item {
    generate_item::generate_item_for_library()
}

#[no_mangle]
pub extern fn generate_item_for_music_store() -> Item {
    generate_item::generate_item_for_music_store()
}

#[no_mangle]
pub extern fn generate_item_for_gem_store() -> Item {
    generate_item::generate_item_for_gem_store()
}

#[no_mangle]
pub extern fn generate_item_for_all_night_deli() -> Item {
    generate_item::generate_item_for_all_night_deli()
}

#[no_mangle]
pub extern fn generate_item_for_black_market() -> Item {
    generate_item::generate_item_for_black_market()
}

