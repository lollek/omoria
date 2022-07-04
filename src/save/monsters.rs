use libc;

use crate::constants;
use crate::debug;
use crate::model::{ Cave, Monster, MonsterRecord };

extern "C" {
    #[link_name = "mlink"] fn C_mlink();
    #[link_name = "popm"] fn C_popm(index: *mut libc::c_long);

    static mut m_list: [Monster; constants::MAX_MALLOC + 1];
    static mut muptr: libc::c_long;
    static mut cave: [[Cave; constants::MAX_WIDTH + 1]; constants::MAX_HEIGHT + 1];
}

fn save_monsters() -> Vec<Monster> {
    let mut vec = Vec::new();
    let mut index: libc::c_long = unsafe { muptr };
    while index > 0 {
        let copy = unsafe { m_list[index as usize] }.to_owned();
        index = copy.nptr as libc::c_long;
        vec.push(copy);
    }
    vec
}

fn load_monsters(data: Vec<Monster>) {
    unsafe {
        muptr = 0;
        C_mlink();
    }

    let mut prev: libc::c_long = 0;
    for item in data.into_iter() {
        let mut index: libc::c_long = 0;
        unsafe {
            C_popm(&mut index);

            m_list[index as usize] = item;

            cave[item.fy as usize][item.fx as usize].cptr = index as u8;
            if muptr == 0 {
                muptr = index;
            } else {
                m_list[prev as usize].nptr = index as u16;
            }
            m_list[index as usize].nptr = 0;
        }
        prev = index;
    }
}

pub fn record() -> MonsterRecord {
    MonsterRecord {
        monsters: save_monsters(),
    }
}

pub fn set_record(record: MonsterRecord) {
    debug::enter("monsters::set_record");
    load_monsters(record.monsters);
    debug::leave("monsters::set_record");
}
