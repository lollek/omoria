use super::{
    player_flags, prt_hunger, recalculate, report_death, RecalculationContext, C_PLAYER_WT,
};

extern "C" {
    static mut msg_flag: bool;
    static mut find_flag: bool;
    fn rest_off();
    fn player_action_move(direction: libc::c_long);
}

struct GlobalHunger;

impl RecalculationContext for GlobalHunger {
    fn has_hunger_item(&self) -> bool {
        unsafe { player_flags.hunger_item != 0 }
    }

    fn food_level(&self) -> i64 {
        unsafe { player_flags.foodc }
    }

    fn is_hungry(&self) -> bool {
        unsafe { player_flags.status & super::IS_HUNGERY != 0 }
    }

    fn is_weak(&self) -> bool {
        unsafe { player_flags.status & super::IS_WEAK != 0 }
    }

    fn set_food_level(&mut self, food_level: i64) {
        unsafe { player_flags.foodc = food_level };
    }

    fn mark_hungry(&mut self) {
        unsafe { player_flags.status |= super::IS_HUNGERY };
    }

    fn mark_weak(&mut self) {
        unsafe { player_flags.status |= super::IS_WEAK };
    }

    fn weight(&self) -> u16 {
        unsafe { C_PLAYER_WT }
    }

    fn set_weight(&mut self, weight: u16) {
        unsafe { C_PLAYER_WT = weight };
    }

    fn min_weight(&self) -> u16 {
        crate::misc::min_allowable_weight()
    }

    fn add_paralysis(&mut self, turns: i64) {
        unsafe { player_flags.paralysis += turns };
    }

    fn message(&mut self, message: &'static str) {
        crate::term::msg_print(message);
    }

    fn interrupt(&mut self) {
        unsafe {
            msg_flag = false;
            rest_off();
            if find_flag {
                player_action_move(5);
            }
        }
    }

    fn redraw(&mut self) {
        unsafe { prt_hunger() };
    }

    fn starve(&mut self) {
        report_death("starvation");
    }
}

pub(super) fn recalculate_global() {
    // C movement can draw RNG too; never hold an active RNG borrow across callbacks.
    recalculate(&mut GlobalHunger, &mut crate::rng::randint);
}
