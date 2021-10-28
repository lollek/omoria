use libc;

use debug;
use model::Item;
use logic::generate_item;

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
