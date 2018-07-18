use random;

#[no_mangle]
pub extern fn squish_stat(stat: i32) -> u8 {
    if stat > 250 {
        250
    } else if stat < 0 {
        0
    } else {
        stat as u8
    }
}

/*	{ Decreases a stat by one randomized level		-RAK- }*/
#[no_mangle]
pub extern fn de_statp(stat: u8) -> u8 {
    if stat < 11 {
        stat
    } else if stat < 151 {
        10
    } else if stat < 241 {
        let mut duh = random::randint(10) as u8 + 5;
        if stat - duh < 150 {
            duh = stat - 150;
        }
        duh
    } else {
        random::randint(3) as u8
    }
}

	/*	{ Increases a stat by one randomized level		-RAK- }*/
#[no_mangle]
pub extern fn in_statp(stat: u8) -> u8 {
    if stat < 150 {
        10
    } else if stat < 220 {
        random::randint(25) as u8
    } else if stat < 240 {
        random::randint(10) as u8
    } else if stat < 250 {
        1
    } else {
        0
    }
}
