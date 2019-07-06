use dungeon;

#[no_mangle]
pub extern fn C_create_monster(y: libc::uint8_t, x: libc::uint8_t, z: libc::c_long, is_asleep: libc::uint_8t) {
    dungeon::create_monster(y, x, z as uint, is_asleep != 0);
}
