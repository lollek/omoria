use std::ffi::CStr;
use libc::c_char;

use player;
use term;

extern {
    static wizard2: i8;
}


#[no_mangle]
pub extern fn eb__display_money() {
    let is_wizard2 = unsafe { wizard2 } != 0;
    let player_account = unsafe { player::player_account };
    let wallet = player::wallet();

    term::prt(&format!(" Gold remaining : {}", wallet.total), 18, 18);
    term::prt(&format!(" Account : {}", player_account), 16, 20);

    term::put_buffer("You have ", 6, 25);
    term::put_buffer(&format!("Mithril  : {:10}", wallet.mithril), 8, 25);
    term::put_buffer(&format!("Platinum : {:10}", wallet.platinum), 9, 25);
    term::put_buffer(&format!("Gold     : {:10}", wallet.gold), 10, 25);
    term::put_buffer(&format!("Silver   : {:10}", wallet.silver), 12, 25);
    term::put_buffer(&format!("Copper   : {:10}", wallet.copper), 13, 25);
    term::put_buffer(&format!("Iron     : {:10}", wallet.iron), 14, 25);

    if is_wizard2 {
        let bank_wallet = player::bank_wallet();

        term::put_buffer("Bank has ", 6, 50);
        term::put_buffer(&format!("{:10}", bank_wallet.mithril), 8, 50);
        term::put_buffer(&format!("{:10}", bank_wallet.platinum), 9, 50);
        term::put_buffer(&format!("{:10}", bank_wallet.gold), 10, 50);
    }
    term::refresh_screen();
}

#[no_mangle]
pub extern fn eb__display_store(shop_owner: *const c_char) {
    if shop_owner.is_null() {
        panic!("Null string received");
    }
    let rs_cstr = unsafe { CStr::from_ptr(shop_owner) };
    let rs_str = rs_cstr.to_str().unwrap();

    term::clear_screen();
    term::prt(rs_str, 4, 10);
    eb__display_money();
    term::prt("You may:", 19, 1);
    term::prt(" d) Deposit money.             w) Withdraw money.", 20, 1);
    term::prt(" c) Change small currency.     i) Buy insurance.", 21, 1);
    term::prt("^R) Redraw the screen.       Esc) Exit from building.", 22, 1);
    term::prt(" p) Put item in vault.         r) Remove item from vault.", 23, 1);
}

#[no_mangle]
pub extern fn reset_total_cash() {
    let mut player_money = player::wallet();
    player_money.calculate_total();
    player::set_wallet(&player_money);

    let mut bank = player::bank_wallet();
    bank.calculate_total();
    player::set_bank_wallet(&bank);
}
