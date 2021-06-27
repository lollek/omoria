use libc;

use debug;
use types::Item;

use save::types::*;

pub const MAX_TALLOC: usize = 225;

extern "C" {
    #[link_name="tlink"] fn C_tlink();
    #[link_name="popt"] fn C_popt(index: *mut libc::c_long);

    static mut cur_height: libc::c_long;
    static mut cur_width: libc::c_long;
    static mut max_panel_rows: libc::c_long;
    static mut max_panel_cols: libc::c_long;
    static mut cave: [[Cave; MAX_WIDTH + 1]; MAX_HEIGHT + 1];
    static mut t_list: [Item; MAX_TALLOC + 1];
    static mut dun_level: libc::c_long;
    static mut mon_tot_mult: libc::c_long;
    static mut turn: libc::c_long;
    static mut randes_seed: libc::c_long;
}

fn save_cave() -> Vec<Cave> {
    let height = unsafe { cur_height } + 1;
    let width = unsafe { cur_width } + 1;
    let mut vec = Vec::new();
    for y in 1..height {
        for x in 1..width {
            vec.push(unsafe { cave[y as usize][x as usize] }.to_owned());
        }
    }
    vec
}

fn load_cave(data: Vec<Cave>) {
    let height = unsafe { cur_height } + 1;
    let width = unsafe { cur_width } + 1;

    let mut i = 0;
    for y in 1..height {
        for x in 1..width {
            unsafe { cave[y as usize][x as usize] = data[i] };
            i += 1;
        }
    }

    if data.len() != i {
        debug::fatal(&format!("load_cave: leftover data: {}", data.len() - i))
    }
}

fn save_treasure() -> Vec<TreasureAndCoordinate> {
    let height = unsafe { cur_height } + 1;
    let width = unsafe { cur_width } + 1;
    let mut vec = Vec::new();
    for y in 1..height {
        for x in 1..width {
            let tile = unsafe { cave[y as usize][x as usize] };
            if tile.tptr > 0 {
                vec.push(TreasureAndCoordinate {
                    treasure: unsafe { t_list[tile.tptr as usize] }.to_owned(),
                    y: y,
                    x: x,
                });
            }
        }
    }
    vec
}

fn load_treasure(data: Vec<TreasureAndCoordinate>) {
    unsafe { C_tlink() };

    for item in data.into_iter() {
        let mut index: libc::c_long = 0;
        unsafe {
            C_popt(&mut index);
            cave[item.y as usize][item.x as usize].tptr = index as u8;
            t_list[index as usize] = item.treasure;
        }
    }
}

pub fn record() -> DungeonRecord {
    DungeonRecord {
        cur_height: unsafe { cur_height }.to_owned(),
        cur_width: unsafe { cur_width }.to_owned(),
        max_panel_rows: unsafe { max_panel_rows }.to_owned(),
        max_panel_cols: unsafe { max_panel_cols }.to_owned(),
        cave: save_cave(),
        treasure: save_treasure(),
        dun_level: unsafe { dun_level },
        mon_tot_mult: unsafe { mon_tot_mult },
        turn: unsafe { turn },
        randes_seed: unsafe { randes_seed },
    }
}

pub fn set_record(record: DungeonRecord) {
    unsafe {
        cur_height = record.cur_height;
        cur_width = record.cur_width;
        max_panel_rows = record.max_panel_rows;
        max_panel_cols = record.max_panel_cols;
    }
    load_cave(record.cave);
    load_treasure(record.treasure);
    unsafe {
        dun_level = record.dun_level;
        mon_tot_mult = record.mon_tot_mult;
        turn = record.turn;
        randes_seed = record.randes_seed;
    }
}
