use screen;

#[no_mangle]
pub extern fn C_print_stat_block() {
    screen::print_stat_block();
}

#[no_mangle]
pub extern fn C_print_equipment_block() {
    screen::print_equipment_block();
}
