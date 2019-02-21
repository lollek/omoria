use debug;
use dungeon;

#[no_mangle]
pub extern fn C_dungeon_show_class_restrictions() {
    debug::enter("dungeon_extern::C_dungeon_show_class_restrictions");

    dungeon::show_class_restrictions();

    debug::leave("dungeon_extern::C_dungeon_show_class_restrictions");
}

