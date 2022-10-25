use libc;

use crate::conversion;

use crate::misc;
use crate::term;
use crate::player;

#[no_mangle]
pub extern fn max_allowable_weight() -> u16 {
    misc::max_allowable_weight()
}

#[no_mangle]
pub extern fn min_allowable_weight() -> u16 {
    misc::min_allowable_weight()
}

#[no_mangle]
pub extern fn C_print_known_spells() {
    misc::print_known_spells();
}

// Utility function for misc.c::print_new_spells to use rust-strings
#[no_mangle]
pub extern fn C_print_new_spell_line(i: u8, slot: libc::c_long, failchance: libc::c_long) {
    let to_print = if slot < 0 {
        "".to_owned() // leave gaps for unknown spells
    } else {
        let spell = conversion::spell::from_usize_or_blank(player::class(), slot as usize);
        format!("{}) {:30} {:3}    {:3}      {:2}",
            (('a' as u8) + i) as char,
            spell.name,
            spell.level,
            spell.mana,
            failchance)
    };

    term::prt(&to_print, (2 + i) as i32, 0);
}
