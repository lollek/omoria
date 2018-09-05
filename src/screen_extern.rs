use screen;

#[no_mangle]
pub extern fn C_print_stat_block() {
    screen::print_stat_block();
}
