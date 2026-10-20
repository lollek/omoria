use super::{QuaffContext, Spell};
use crate::inventory::selected::SelectedItem;
use crate::magic::spells;
use crate::model::{InventoryItem, Item, PlayerFlags};
use std::ptr::{addr_of_mut, NonNull};

extern "C" {
    static mut player_flags: PlayerFlags;
    static player_exp: i64;
    static player_lev: u16;
    static player_mana: i16;
    static mut player_cmana: libc::c_float;
    fn C_player_add_exp(amount: libc::c_long);
    fn player_hunger_eat(amount: libc::c_long);
    fn prt_stat_block();
    fn prt_poisoned();
    fn learn_magic(redraw: bool);
}

struct Context {
    selected: Option<SelectedItem>,
}

impl QuaffContext for Context {
    fn item(&mut self) -> Item {
        self.selected
            .as_ref()
            .expect("selected potion is live")
            .item()
    }

    fn flags(&mut self) -> &mut PlayerFlags {
        unsafe { &mut *addr_of_mut!(player_flags) }
    }

    fn experience(&mut self) -> i64 {
        unsafe { player_exp }
    }

    fn level(&mut self) -> u16 {
        unsafe { player_lev }
    }

    fn restore_mana(&mut self) {
        unsafe {
            if player_cmana < f32::from(player_mana) {
                player_cmana = f32::from(player_mana);
            }
        }
    }

    fn spell(&mut self, spell: Spell) -> bool {
        let placeholder = b"X\0".as_ptr().cast();
        unsafe {
            match spell {
                Spell::Gain(stat) => spells::gain_stat(i32::from(u8::from(stat)), placeholder),
                Spell::Lose(stat) => {
                    spells::lose_stat(i32::from(u8::from(stat)), placeholder, placeholder)
                }
                Spell::Restore(stat) => {
                    spells::restore_stat(i32::from(u8::from(stat)), placeholder)
                }
                Spell::Heal(amount) => spells::hp_player(amount, b"a potion.\0".as_ptr().cast()),
                Spell::DetectCreatures => spells::detect_creatures(spells::SE_MONSTER),
                Spell::Cure(status) => {
                    spells::cure_player_status_effect(status.counter(self.flags()))
                }
                Spell::Learn => {
                    learn_magic(true);
                    false
                }
                Spell::LoseExperience(amount) => {
                    spells::lose_exp(amount);
                    false
                }
                Spell::RestoreLevels => spells::restore_player_drained_levels(),
                Spell::DetectInvisible(amount) => spells::detect_inv2(amount),
                Spell::SlowPoison => spells::slow_poison(),
            }
        }
    }

    fn add_experience(&mut self, amount: i64) {
        unsafe { C_player_add_exp(amount) };
    }

    fn eat(&mut self, amount: i64) {
        unsafe { player_hunger_eat(amount) };
    }

    fn message(&mut self, message: &str) {
        crate::term::msg_print(message);
    }

    fn redraw(&mut self) {
        unsafe { prt_stat_block() };
    }

    fn redraw_poisoned(&mut self) {
        unsafe { prt_poisoned() };
    }

    fn identify(&mut self) {
        self.selected
            .as_mut()
            .expect("selected potion is live")
            .identify();
    }

    fn remaining(&mut self) {
        self.selected
            .as_ref()
            .expect("selected potion is live")
            .remaining();
    }

    fn destroy(&mut self) {
        self.selected
            .take()
            .expect("selected potion is live")
            .consume();
    }
}

/// # Safety
/// `item` must be a live C-selected inventory node. The game loop is
/// single-threaded; no other caller may access inventory or player globals here.
pub(super) unsafe fn quaff_global(item: *mut InventoryItem) {
    let Some(node) = NonNull::new(item) else {
        crate::debug::error("Cannot quaff a null selected inventory item");
        return;
    };
    let mut context = Context {
        selected: Some(unsafe { SelectedItem::new(node) }),
    };
    // Release the active RNG borrow after each draw: C spell effects may draw
    // from the same scoped seeded RNG during the action.
    super::quaff_with_roll(&mut context, &mut crate::rng::randint);
}
