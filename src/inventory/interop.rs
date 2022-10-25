use crate::inventory::display_inventory;

#[no_mangle]
pub extern "C" fn display_inventory() {
    display_inventory::display_inventory();
}
