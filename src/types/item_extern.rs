use types::{ Item, ItemType };
use debug;

use pancurses;

#[no_mangle]
pub extern fn C_item_get_tchar(item_ptr: *const Item) -> pancurses::chtype {
    debug::enter(&format!("C_item_get_tchar"));

    let item = unsafe { *item_ptr };
    debug::info(&format!("(enter) symbol: {}, {}", item.tval, item.subval));
    let res = ItemType::from(item.tval).symbol(item.subval);

    debug::leave("C_item_get_tchar");
    res
}
