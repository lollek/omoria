use libc;

use std::ptr;
use std::mem;

use save::types::*;

extern "C" {
    static mut inventory_list: *mut TreasureRec;
    static mut inven_ctr: libc::c_long;
}

pub fn record() -> Vec<TreasureRec> {
    let mut ptr = unsafe { inventory_list };
    let mut vec = Vec::new();

    while !ptr.is_null() {
        let elem = unsafe { &*ptr };
        vec.push(elem.to_owned());
        ptr = elem.next;
    }
    vec
}

pub fn set_record(record: Vec<TreasureRec>) {
    fn mallocfn() -> *mut TreasureRec {
        unsafe { libc::malloc(mem::size_of::<TreasureRec>()) as *mut TreasureRec }
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
        mem::replace(unsafe { &mut *ptr }, item);
        if i != size -1 {
            unsafe {
                let next = mallocfn();
                (*ptr).next = next;
                ptr = (*ptr).next;
            }
        }
    }
    unsafe { inven_ctr = size as libc::c_long };
}
