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

    term::prt_r(&format!(" Gold remaining : {}", wallet.total), 18, 18);
    term::prt_r(&format!(" Account : {}", player_account), 16, 20);

    term::put_buffer_r("You have ", 6, 25);
    term::put_buffer_r(&format!("Mithril  : {:10}", wallet.mithril), 8, 25);
    term::put_buffer_r(&format!("Platinum : {:10}", wallet.platinum), 9, 25);
    term::put_buffer_r(&format!("Gold     : {:10}", wallet.gold), 10, 25);
    term::put_buffer_r(&format!("Silver   : {:10}", wallet.silver), 12, 25);
    term::put_buffer_r(&format!("Copper   : {:10}", wallet.copper), 13, 25);
    term::put_buffer_r(&format!("Iron     : {:10}", wallet.iron), 14, 25);

    if is_wizard2 {
        let bank_wallet = player::bank_wallet();

        term::put_buffer_r("Bank has ", 6, 50);
        term::put_buffer_r(&format!("{:10}", bank_wallet.mithril), 8, 50);
        term::put_buffer_r(&format!("{:10}", bank_wallet.platinum), 9, 50);
        term::put_buffer_r(&format!("{:10}", bank_wallet.gold), 10, 50);
    }
    term::refresh_screen();
}

#[no_mangle]
pub extern fn eb__display_store(shop_owner: *const c_char) {
    term::clear_screen();
    term::prt(shop_owner, 4, 10);
    eb__display_money();
    term::prt_r("You may:", 19, 1);
    term::prt_r(" d) Deposit money.             w) Withdraw money.", 20, 1);
    term::prt_r(" c) Change small currency.     i) Buy insurance.", 21, 1);
    term::prt_r("^R) Redraw the screen.       Esc) Exit from building.", 22, 1);
    term::prt_r(" p) Put item in vault.         r) Remove item from vault.", 23, 1);
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
