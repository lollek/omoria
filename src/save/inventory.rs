use libc;

use std::ptr;

use crate::inventory;
use crate::model::InventoryItem;

extern "C" {
    static mut inventory_list: *mut InventoryItem;
    static mut inven_ctr: libc::c_long;
}

pub fn record() -> Vec<InventoryItem> {
    inventory::not_safe::inventory_clone()
}

pub fn set_record(record: Vec<InventoryItem>) {
    fn mallocfn() -> *mut InventoryItem {
        unsafe { libc::malloc(size_of::<InventoryItem>()) as *mut InventoryItem }
    }

    let size = record.len();
    if size == 0 {
        unsafe {
            inventory_list = ptr::null_mut();
            inven_ctr = 0;
        }
        return;
    }
    unsafe { inventory_list = mallocfn() };
    let mut ptr = unsafe { inventory_list };

    for (i, item) in record.into_iter().enumerate() {
        unsafe { *ptr = item };
        if i != size - 1 {
            unsafe {
                let next = mallocfn();
                (*ptr).next = next;
                ptr = (*ptr).next;
            }
        }
    }
    unsafe { inven_ctr = size as libc::c_long };
}
