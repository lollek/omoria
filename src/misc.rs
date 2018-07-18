use std::ffi::CStr;

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

pub fn c_array_to_rust_string(array: Vec<u8>) -> String {
    let safe_array = array.to_owned()
        .iter_mut()
        .take_while(|i| i != &&0u8)
        .chain([0u8].iter_mut())
        .map(|i| i.to_owned())
        .collect::<Vec<u8>>();

    CStr::from_bytes_with_nul(&safe_array)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

