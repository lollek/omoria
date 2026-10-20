use super::{EatContext, StatEffect, Status};
use crate::inventory::selected::SelectedItem;
use crate::magic::spells;
use crate::model::{InventoryItem, Item, PlayerFlags, Stat};
use std::ffi::CString;
use std::ptr::{addr_of_mut, NonNull};

extern "C" {
    static mut player_flags: PlayerFlags;
    static player_lev: u16;
    fn C_player_add_exp(amount: libc::c_long);
    fn player_hunger_eat(amount: libc::c_long);
    fn player_hunger_set_status(status: libc::c_int);
    fn draw_cave();
    fn prt_stat_block();
}

struct Context {
    selected: Option<SelectedItem>,
}

impl EatContext for Context {
    fn item(&mut self) -> Item {
        self.selected
            .as_ref()
            .expect("selected food is live")
            .item()
    }

    fn add_status_duration(&mut self, status: Status, turns: i64) {
        unsafe { *status.counter(&mut *addr_of_mut!(player_flags)) += turns };
    }

    fn set_paralysis(&mut self, turns: i64) {
        unsafe { player_flags.paralysis = turns };
    }

    fn clear_secondary_flag(&mut self, flag: u64) {
        self.selected
            .as_mut()
            .expect("selected food is live")
            .clear_secondary_flag(flag);
    }

    fn cure(&mut self, status: Status) -> bool {
        unsafe {
            let flags = &mut *addr_of_mut!(player_flags);
            spells::cure_player_status_effect(status.counter(flags))
        }
    }

    fn stat_effect(
        &mut self,
        effect: StatEffect,
        stat: Stat,
        message: &'static str,
        second_message: &'static str,
    ) -> bool {
        let message = CString::new(message).expect("static C message has no NUL byte");
        let second_message =
            CString::new(second_message).expect("static C message has no NUL byte");
        unsafe {
            match effect {
                StatEffect::Gain => spells::gain_stat(u8::from(stat).into(), message.as_ptr()),
                StatEffect::Lose => spells::lose_stat(
                    u8::from(stat).into(),
                    message.as_ptr(),
                    second_message.as_ptr(),
                ),
                StatEffect::Restore => {
                    spells::restore_stat(u8::from(stat).into(), message.as_ptr())
                }
            }
        }
    }

    fn hp_player(&mut self, amount: i64, cause: &str) -> bool {
        let cause = CString::new(cause).expect("food damage causes contain no NUL bytes");
        unsafe { spells::hp_player(amount, cause.as_ptr()) }
    }

    fn take_hit(&mut self, amount: i64, cause: &str) {
        let cause = CString::new(cause).expect("food damage causes contain no NUL bytes");
        unsafe { crate::player::damage::take_hit(amount, cause.as_ptr()) };
    }

    fn bless(&mut self, amount: i64) {
        unsafe { spells::bless(amount) };
    }

    fn aggravate_monsters(&mut self, distance: i64) {
        unsafe { spells::aggravate_monster(distance) };
    }

    fn set_full(&mut self) {
        unsafe { player_hunger_set_status(3) };
    }

    fn player_level(&self) -> u16 {
        unsafe { player_lev }
    }

    fn add_experience(&mut self, amount: i64) {
        unsafe { C_player_add_exp(amount) };
    }

    fn eat(&mut self, amount: i64) {
        unsafe { player_hunger_eat(amount) };
    }

    fn message(&mut self, message: &'static str) {
        crate::term::msg_print(message);
    }

    fn draw_cave(&mut self) {
        unsafe { draw_cave() };
    }

    fn redraw_stats(&mut self) {
        unsafe { prt_stat_block() };
    }

    fn identify(&mut self) {
        self.selected
            .as_mut()
            .expect("selected food is live")
            .identify();
    }

    fn remaining(&mut self) {
        self.selected
            .as_ref()
            .expect("selected food is live")
            .remaining();
    }

    fn destroy(&mut self) {
        self.selected
            .take()
            .expect("selected food is live")
            .consume();
    }
}

/// # Safety
/// `item` must be the live C-selected inventory node. The game loop is
/// single-threaded; no other caller may access inventory or player globals here.
pub(super) unsafe fn eat_global(item: *mut InventoryItem) {
    let Some(node) = NonNull::new(item) else {
        crate::debug::error("Cannot eat a null selected inventory item");
        return;
    };
    let mut context = Context {
        selected: Some(unsafe { SelectedItem::new(node) }),
    };
    // Release the active RNG borrow after each draw because C effects draw
    // from the same scoped seeded RNG during this action.
    super::eat_selected_with_roll(&mut context, &mut crate::rng::randint);
}
