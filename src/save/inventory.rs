use libc;

use std::ptr;
use std::mem;

use debug;
use model::{ InventoryItem };

extern "C" {
    static mut inventory_list: *mut InventoryItem;
    static mut inven_ctr: libc::c_long;
}

pub fn record() -> Vec<InventoryItem> {
    let mut ptr = unsafe { inventory_list };
    let mut vec = Vec::new();

    while !ptr.is_null() {
        let elem = unsafe { &*ptr };
        vec.push(elem.to_owned());
        ptr = elem.next;
    }
    vec
}

pub fn set_record(record: Vec<InventoryItem>) {
    debug::enter("inventory::set_record");
    fn mallocfn() -> *mut InventoryItem {
        unsafe { libc::malloc(mem::size_of::<InventoryItem>()) as *mut InventoryItem }
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
        if i != size -1 {
            unsafe {
                let next = mallocfn();
                (*ptr).next = next;
                ptr = (*ptr).next;
            }
        }
    }
    unsafe { inven_ctr = size as libc::c_long };
    debug::leave("inventory::set_record");
}
