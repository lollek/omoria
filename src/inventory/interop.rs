use crate::inventory::display_inventory;

#[no_mangle]
pub extern fn display_inventory() {
    display_inventory::display_inventory();
}
