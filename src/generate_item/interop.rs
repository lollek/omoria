use libc;

use debug;
use model::Item;
use generate_item;

#[no_mangle]
pub extern fn generate_item_level_for_dungeon_level(dungeon_level: libc::uint8_t, tries: libc::uint8_t) -> libc::uint8_t {
    debug::enter("generate_item_level_for_dungeon_level");
    let result = generate_item::generate_item_level_for_dungeon_level(dungeon_level, tries);
    debug::leave("generate_item_level_for_dungeon_level");
    result
}

#[no_mangle]
pub extern fn generate_item_for_dungeon_level(dungeon_level: libc::uint8_t) -> Item {
    debug::enter("generate_item_for_dungeon_level");
    let result = generate_item::generate_item_for_dungeon_level(dungeon_level);
    debug::leave("generate_item_for_dungeon_level");
    result
}

#[no_mangle]
pub extern fn generate_item_for_item_level(item_level: libc::uint8_t) -> Item {
    debug::enter("generate_item_for_item_level");
    let result = generate_item::generate_item_for_item_level(item_level);
    debug::leave("generate_item_for_item_level");
    result
}


#[no_mangle]
pub extern fn generate_item_for_general_store() -> Item {
    debug::enter("generate_item_for_general_store");
    let result = generate_item::generate_item_for_general_store();
    debug::leave("generate_item_for_general_store");
    result
}

#[no_mangle]
pub extern fn generate_item_for_armorsmith() -> Item {
    debug::enter("generate_item_for_armorsmith");
    let result = generate_item::generate_item_for_armorsmith();
    debug::leave("generate_item_for_armorsmith");
    result
}

#[no_mangle]
pub extern fn generate_item_for_weaponsmith() -> Item {
    debug::enter("generate_item_for_weaponsmith");
    let result = generate_item::generate_item_for_weaponsmith();
    debug::leave("generate_item_for_weaponsmith");
    result
}

#[no_mangle]
pub extern fn generate_item_for_temple() -> Item {
    debug::enter("generate_item_for_temple");
    let result = generate_item::generate_item_for_temple();
    debug::leave("generate_item_for_temple");
    result
}

#[no_mangle]
pub extern fn generate_item_for_alchemist_store() -> Item {
    debug::enter("generate_item_for_alchemist_store");
    let result = generate_item::generate_item_for_alchemist_store();
    debug::leave("generate_item_for_alchemist_store");
    result
}

#[no_mangle]
pub extern fn generate_item_for_magic_store() -> Item {
    debug::enter("generate_item_for_magic_store");
    let result = generate_item::generate_item_for_magic_store();
    debug::leave("generate_item_for_magic_store");
    result
}

#[no_mangle]
pub extern fn generate_item_for_inn() -> Item {
    debug::enter("generate_item_for_inn");
    let result = generate_item::generate_item_for_inn();
    debug::leave("generate_item_for_inn");
    result
}

#[no_mangle]
pub extern fn generate_item_for_library() -> Item {
    debug::enter("generate_item_for_library");
    let result = generate_item::generate_item_for_library();
    debug::leave("generate_item_for_library");
    result
}

#[no_mangle]
pub extern fn generate_item_for_music_store() -> Item {
    debug::enter("generate_item_for_music_store");
    let result = generate_item::generate_item_for_music_store();
    debug::leave("generate_item_for_music_store");
    result
}

#[no_mangle]
pub extern fn generate_item_for_gem_store() -> Item {
    debug::enter("generate_item_for_gem_store");
    let result = generate_item::generate_item_for_gem_store();
    debug::leave("generate_item_for_gem_store");
    result
}

#[no_mangle]
pub extern fn generate_item_for_all_night_deli() -> Item {
    debug::enter("generate_item_for_all_night_deli");
    let result = generate_item::generate_item_for_all_night_deli();
    debug::leave("generate_item_for_all_night_deli");
    result
}

#[no_mangle]
pub extern fn generate_item_for_black_market() -> Item {
    debug::enter("generate_item_for_black_market");
    let result = generate_item::generate_item_for_black_market();
    debug::leave("generate_item_for_black_market");
    result
}

