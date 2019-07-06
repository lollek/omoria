use std::sync::RwLock;
use types::{ Creature, Monster };
use player;

const MAX_CREATURES: usize = 415;

extern "C" {
    static mut c_list: [Creature; MAX_CREATURES + 1]; // List of monster templates
    fn distance(y1: libc::c_long, x1: libc::c_long, y2: libc::c_long, x2: libc::c_long) -> libc::c_long;
}

lazy_static! {
    static ref MONSTERS: RwLock<Vec<Monster>> = RwLock::new(Vec::new());
}

// Places a monster at given location -RAK-
pub fn create_monster(y: u8, x: u8, z: usize, is_asleep: bool) {
    let template = unsafe { c_list[z] };
    let mut monster = Monster::new();
    monster.fy = y;
    monster.fx = x;
    monster.mptr = z as u16;
    monster.nptr = 0; // unused

    monster.hp = template.calculate_hp();

    monster.cdis = unsafe { distance(player::current_row(), player::current_column(), y as i64, x as i64) } as i16;
    monster.cspeed = template.calculate_speed();
    monster.stunned = 0;

    if is_asleep {
        monster.csleep = template.calculate_rounds_asleep();
    }

    //popm(&cur_pos);
    //muptr = cur_pos;
    MONSTERS.write().unwrap().push(monster);
    //cave[y][x].cptr = cur_pos;
}
