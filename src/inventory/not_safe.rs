use crate::{model::InventoryItem, debug};

extern "C" {
    static mut inventory_list: *mut InventoryItem;
}

pub fn inventory_clone() -> Vec<InventoryItem> {
    let mut ptr = unsafe { inventory_list };
    let mut vec = Vec::new();

    while !ptr.is_null() {
        let elem = unsafe { &*ptr };
        vec.push(elem.to_owned());
        ptr = elem.next;
    }
    vec
}
