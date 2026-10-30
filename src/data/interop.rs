use crate::conversion;
use crate::data;
use crate::model::{Item, ItemType};
use std::convert::TryFrom;

#[no_mangle]
pub extern "C" fn C_class_melee_bonus(class: i32) -> i8 {
    data::class::melee_bonus(&conversion::class::from_usize(class as usize).unwrap())
}

#[no_mangle]
pub extern "C" fn C_class_ranged_bonus(class: i32) -> i8 {
    data::class::ranged_bonus(&conversion::class::from_usize(class as usize).unwrap())
}

#[no_mangle]
pub extern "C" fn C_item_get_tchar(item_ptr: *const Item) -> pancurses::chtype {
    let item = unsafe { *item_ptr };
    let item_type = ItemType::try_from(item.tval).unwrap();
    data::item_type::symbol(&item_type, item.subval)
}

#[no_mangle]
pub extern "C" fn C_item_get_glyph(item_ptr: *const Item) -> pancurses::chtype {
    let item = unsafe { *item_ptr };
    data::item_color::glyph(&item)
}

#[no_mangle]
/// Returns the terminal-independent color key used for item rendering and stacking.
///
/// # Safety
/// `item_ptr` must be non-null and point to a valid `Item` for this call.
pub extern "C" fn C_item_color_key(item_ptr: *const Item) -> libc::c_int {
    let item = unsafe { *item_ptr };
    data::item_color::color_key(&item)
}

#[no_mangle]
pub extern "C" fn coin_value(currency: u8) -> i64 {
    data::currency::value(&conversion::currency::from_usize(currency.into()).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_item_get_tchar_matches_plain_symbol_for_each_renderable_item_type() {
        for raw_type in u8::MIN..=u8::MAX {
            let Ok(item_type) = ItemType::try_from(raw_type) else {
                continue;
            };
            let subval = match item_type {
                ItemType::MiscObject => 1,
                ItemType::MiscUsable => 14,
                ItemType::SeenTrap => 1,
                ItemType::EntranceToStore => 101,
                ItemType::Whirlpool => continue,
                _ => 0,
            };
            let item = Item {
                name: [0; 70],
                tval: raw_type,
                flags2: 0,
                flags: 0,
                p1: 0,
                cost: 0,
                subval,
                weight: 0,
                number: 0,
                tohit: 0,
                todam: 0,
                ac: 0,
                toac: 0,
                damage: [0; 7],
                level: 0,
                identified: 0,
            };
            assert_eq!(
                C_item_get_tchar(&item),
                data::item_type::symbol(&item_type, subval),
                "item type {:?}",
                item_type
            );
        }
    }
}
