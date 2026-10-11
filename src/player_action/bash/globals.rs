use super::{bash_with_rng, BashContext, BashRandom};
use crate::{
    dungeon::cell::with_global_cells,
    equipment::{Slot, EQUIP_MAX},
    model::{Cave, Item, PlayerFlags, Stat},
};

struct GlobalBash;
struct GlobalRandom;

impl BashRandom for GlobalRandom {
    fn roll(&mut self, max: i64) -> i64 {
        crate::rng::randint(max)
    }

    fn hit(&mut self, base: i64, armor: i64) -> bool {
        crate::combat::fighting::managed_to_hit(base, 0, 0, armor)
    }
}

impl BashContext for GlobalBash {
    fn read(&mut self, y: i64, x: i64) -> Option<(Cave, Option<Item>)> {
        unsafe { with_global_cells(|cells| cells.read(y, x)) }
    }

    fn write(&mut self, y: i64, x: i64, cell: Cave, item: Item) {
        unsafe { with_global_cells(|cells| cells.write(y, x, cell, item)) }
    }

    fn open_door(&mut self) -> Item {
        extern "C" {
            static mut door_list: [Item; 3];
        }
        unsafe { door_list[0] }
    }

    fn strength(&mut self) -> i64 {
        i64::from(crate::player::get_stat(Stat::Strength))
    }

    fn weight(&mut self) -> i64 {
        unsafe { i64::from(crate::player::player_wt) }
    }

    fn message(&mut self, message: &str) {
        crate::term::msg_print(message);
    }

    fn redraw(&mut self, y: i64, x: i64) {
        extern "C" {
            fn lite_spot(y: libc::c_long, x: libc::c_long);
        }
        unsafe { lite_spot(y, x) }
    }

    fn paralyze(&mut self, turns: i64) {
        extern "C" {
            static mut player_flags: PlayerFlags;
        }
        unsafe { player_flags.paralysis = turns }
    }

    fn afraid(&mut self) -> bool {
        extern "C" {
            static mut player_flags: PlayerFlags;
        }
        unsafe { player_flags.afraid > 0 }
    }

    fn dexterity(&mut self) -> i64 {
        i64::from(crate::player::get_stat(Stat::Dexterity))
    }

    fn primary(&mut self) -> Item {
        unsafe { *crate::equipment::get_item(Slot::Primary) }
    }

    fn set_primary(&mut self, item: Item) {
        extern "C" {
            static mut equipment: [Item; EQUIP_MAX];
        }
        unsafe { equipment[Slot::Primary as usize] = item }
    }

    fn blank_item(&mut self) -> Item {
        extern "C" {
            static blank_treasure: Item;
        }
        unsafe { blank_treasure }
    }

    fn shield(&mut self) -> Item {
        unsafe { *crate::equipment::get_item(Slot::Shield) }
    }

    fn attack(&mut self, y: i64, x: i64) -> bool {
        extern "C" {
            fn player_action_attack(y: libc::c_long, x: libc::c_long) -> bool;
        }
        unsafe { player_action_attack(y, x) }
    }

    fn stun(&mut self, monster: u8) {
        extern "C" {
            fn do_stun(
                monster: libc::c_uchar,
                save_bonus: libc::c_long,
                time: libc::c_long,
            ) -> bool;
        }
        unsafe { do_stun(monster, -10, 2) };
    }
}

pub(super) fn bash_global(y: i64, x: i64) {
    bash_with_rng(&mut GlobalBash, &mut GlobalRandom, y, x);
}
