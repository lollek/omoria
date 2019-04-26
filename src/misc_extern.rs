use libc;

use debug;
use magic;
use misc;
use term;

#[no_mangle]
pub extern fn max_allowable_weight() -> libc::uint16_t {
    debug::enter("misc_extern::max_allowable_weight");

    let res = misc::max_allowable_weight();

    debug::leave("misc_extern::max_allowable_weight");
    res
}

#[no_mangle]
pub extern fn min_allowable_weight() -> libc::uint16_t {
    debug::enter("misc_extern::min_allowable_weight");

    let res = misc::min_allowable_weight();

    debug::leave("misc_extern::min_allowable_weight");
    res
}

#[no_mangle]
pub extern fn C_print_known_spells() {
    debug::enter("misc_extern::print_known_spells");
    misc::print_known_spells();
    debug::leave("misc_extern::print_known_spells");
}

// Utility function for misc.c::print_new_spells to use rust-strings
#[no_mangle]
pub extern fn C_print_new_spell_line(i: libc::uint8_t, slot: libc::c_long, failchance: libc::c_long) {
    let to_print = if slot < 0 {
        "".to_owned() // leave gaps for unknown spells
    } else {
        let spell = magic::spell(slot as usize);
        format!("{}) {:30} {:3}    {:3}      {:2}",
            (('a' as u8) + i) as char,
            spell.name,
            spell.level,
            spell.mana,
            failchance)
    };

    term::prt(&to_print, (2 + i) as i32, 0);
}
