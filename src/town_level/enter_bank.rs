use libc::c_char;
use std::ffi::CStr;

use crate::{ncurses, player};
use crate::term;

extern "C" {
    static wizard2: i8;
}

#[no_mangle]
pub extern "C" fn eb__display_money() {
    let is_wizard2 = unsafe { wizard2 } != 0;
    let player_account = unsafe { player::player_account };
    let wallet = player::wallet();

    term::prt(&format!(" Gold remaining : {}", wallet.total), 18, 18);
    term::prt(&format!(" Account : {}", player_account), 16, 20);

    ncurses::mvaddstr(5, 24, "You have ");
    let msg = &format!("Mithril  : {:10}", wallet.mithril);
    ncurses::mvaddstr(7, 24, msg);
    let msg = &format!("Platinum : {:10}", wallet.platinum);
    ncurses::mvaddstr(8, 24, msg);
    let msg = &format!("Gold     : {:10}", wallet.gold);
    ncurses::mvaddstr(9, 24, msg);
    let msg = &format!("Silver   : {:10}", wallet.silver);
    ncurses::mvaddstr(11, 24, msg);
    let msg = &format!("Copper   : {:10}", wallet.copper);
    ncurses::mvaddstr(12, 24, msg);
    let msg = &format!("Iron     : {:10}", wallet.iron);
    ncurses::mvaddstr(13, 24, msg);

    if is_wizard2 {
        let bank_wallet = player::bank_wallet();

        ncurses::mvaddstr(5, 49, "Bank has ");
        let msg = &format!("{:10}", bank_wallet.mithril);
        ncurses::mvaddstr(7, 49, msg);
        let msg = &format!("{:10}", bank_wallet.platinum);
        ncurses::mvaddstr(8, 49, msg);
        let msg = &format!("{:10}", bank_wallet.gold);
        ncurses::mvaddstr(9, 49, msg);
    }
    ncurses::refresh();
}

#[no_mangle]
pub extern "C" fn eb__display_store(shop_owner: *const c_char) {
    if shop_owner.is_null() {
        panic!("Null string received");
    }
    let rs_cstr = unsafe { CStr::from_ptr(shop_owner) };
    let rs_str = rs_cstr.to_str().unwrap();

    term::clear_screen();
    term::prt(rs_str, 3, 9);
    eb__display_money();
    term::prt("You may:", 18, 0);
    term::prt(" d) Deposit money.             w) Withdraw money.", 19, 0);
    term::prt(" c) Change small currency.     i) Buy insurance.", 20, 0);
    term::prt(
        "^R) Redraw the screen.       Esc) Exit from building.",
        21,
        0,
    );
    term::prt(
        " p) Put item in vault.         r) Remove item from vault.",
        22,
        0,
    );
}
