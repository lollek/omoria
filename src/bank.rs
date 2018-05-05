use term::put_buffer_r;
use term::prt;
use term::prt_r;
use term::refresh_screen;
use term::clear_screen;

enum Currency {
    Total = 0,
    Iron = 1,
    Copper = 2,
    Silver = 3,
    Gold = 4,
    Platinum = 5,
    Mithril = 6,
}

extern {
    static player_account: i64;
    static player_money: [i64; 7];
    static bank: [i64; 7];
    static wizard2: i8;
}

#[no_mangle]
pub extern fn eb__display_money() {
    unsafe {
        prt_r(&format!(" Gold remaining : {}", player_money[Currency::Total as usize]), 18, 18);
        prt_r(&format!(" Account : {}", player_account), 16, 20);

        put_buffer_r("You have ", 6, 25);
        put_buffer_r(&format!("Mithril  : {:10}", player_money[Currency::Mithril as usize]),
        8, 25);
        put_buffer_r(&format!("Platinum : {:10}", player_money[Currency::Platinum as usize]),
        9, 25);
        put_buffer_r(&format!("Gold     : {:10}", player_money[Currency::Gold as usize]),
        10, 25);
        put_buffer_r(&format!("Silver   : {:10}", player_money[Currency::Silver as usize]),
        12, 25);
        put_buffer_r(&format!("Copper   : {:10}", player_money[Currency::Copper as usize]),
        13, 25);
        put_buffer_r(&format!("Iron     : {:10}", player_money[Currency::Iron as usize]),
        14, 25);

        if wizard2 != 0 {
            put_buffer_r("Bank has ", 6, 50);
            put_buffer_r(&format!("{:10}", bank[Currency::Mithril as usize]), 8, 50);
            put_buffer_r(&format!("{:10}", bank[Currency::Platinum as usize]), 9, 50);
            put_buffer_r(&format!("{:10}", bank[Currency::Gold as usize]), 10, 50);
        }
    }
    refresh_screen();
}

#[no_mangle]
pub extern fn eb__display_store(shop_owner: *const u8) {
    clear_screen();
    prt(shop_owner, 4, 10);
    eb__display_money();
    prt_r("You may:", 19, 1);
    prt_r(" d) Deposit money.             w) Withdraw money.", 20, 1);
    prt_r(" c) Change small currency.     i) Buy insurance.", 21, 1);
    prt_r("^R) Redraw the screen.       Esc) Exit from building.", 22, 1);
    prt_r(" p) Put item in vault.         r) Remove item from vault.", 23, 1);
}
