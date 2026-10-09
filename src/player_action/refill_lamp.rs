use crate::model::Item;

#[cfg(not(test))]
mod interop {
    use super::*;
    use crate::{
        equipment,
        model::{InventoryItem, ItemType},
        term,
    };
    use std::ptr;

    extern "C" {
        static mut equipment: [Item; equipment::EQUIP_MAX];
        fn inventory_find_range(
            item_val: *const u8,
            inner: bool,
            first: *mut *mut InventoryItem,
            count: *mut libc::c_long,
        ) -> bool;
        fn msg_remaining_of_item(item: *const InventoryItem);
        fn inven_destroy(item: *mut InventoryItem);
        fn prt_stat_block();
    }

    struct Context {
        oil: *mut InventoryItem,
    }

    impl RefillContext for Context {
        fn find_oil(&mut self) -> Option<i64> {
            let mut oil_types = [0_u8; 25];
            oil_types[0] = ItemType::FlaskOfOil.into();
            let mut count = 0;
            if unsafe { inventory_find_range(oil_types.as_ptr(), false, &mut self.oil, &mut count) }
            {
                Some(unsafe { (*self.oil).data.p1 })
            } else {
                None
            }
        }
        fn message(&mut self, message: &str) {
            term::msg_print(message);
        }
        fn remaining(&mut self) {
            unsafe { msg_remaining_of_item(self.oil) };
        }
        fn destroy(&mut self) {
            unsafe { inven_destroy(self.oil) };
            self.oil = ptr::null_mut();
        }
        fn redraw(&mut self) {
            unsafe { prt_stat_block() };
        }
    }

    /// Runs on the single-threaded game loop with initialized equipment and inventory.
    /// The selected inventory node stays valid until `inven_destroy` consumes it.
    fn refill_global() {
        let lamp = unsafe { &mut (*ptr::addr_of_mut!(equipment))[equipment::Slot::Light as usize] };
        refill(
            lamp,
            &mut Context {
                oil: ptr::null_mut(),
            },
        );
    }

    #[no_mangle]
    pub extern "C" fn player_action_refill_lamp() {
        refill_global();
    }
}

trait RefillContext {
    fn find_oil(&mut self) -> Option<i64>;
    fn message(&mut self, message: &str);
    fn remaining(&mut self);
    fn destroy(&mut self);
    fn redraw(&mut self);
}

fn refill(lamp: &mut Item, context: &mut impl RefillContext) {
    if !(1..10).contains(&lamp.subval) {
        context.message("But you are not using a lamp.");
        return;
    }
    let Some(oil) = context.find_oil() else {
        context.message("You have no oil.");
        return;
    };
    context.message("Your lamp is full.");
    lamp.p1 = (lamp.p1 + oil).min(15000);
    context.remaining();
    context.destroy();
    context.redraw();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::cell::test_support::blank_item;

    struct Context {
        oil: Option<i64>,
        count: usize,
        searches: usize,
        events: Vec<String>,
    }

    impl Context {
        fn new(oil: Option<i64>) -> Self {
            Self {
                oil,
                count: 2,
                searches: 0,
                events: Vec::new(),
            }
        }
    }

    impl RefillContext for Context {
        fn find_oil(&mut self) -> Option<i64> {
            self.searches += 1;
            self.oil
        }
        fn message(&mut self, message: &str) {
            self.events.push(message.to_owned());
        }
        fn remaining(&mut self) {
            assert_eq!(self.count, 2);
            self.events.push("remaining".to_owned());
        }
        fn destroy(&mut self) {
            self.count -= 1;
            self.events.push("destroy".to_owned());
        }
        fn redraw(&mut self) {
            self.events.push("redraw".to_owned());
        }
    }

    #[test]
    fn refill_adds_fuel_up_to_cap_and_consumes_one_flask() {
        for (fuel, oil, expected) in [
            (100, 7500, 7600),
            (14000, 7500, 15000),
            (15000, 7500, 15000),
        ] {
            let mut lamp = blank_item();
            lamp.subval = 1;
            lamp.p1 = fuel;
            let mut context = Context::new(Some(oil));
            refill(&mut lamp, &mut context);
            assert_eq!(lamp.p1, expected);
            assert_eq!(context.count, 1);
            assert_eq!(context.searches, 1);
            assert_eq!(
                context.events,
                ["Your lamp is full.", "remaining", "destroy", "redraw"]
            );
        }
    }

    #[test]
    fn no_lamp_leaves_fuel_and_inventory_unchanged() {
        for subval in [0, 10, 11] {
            let mut lamp = blank_item();
            lamp.subval = subval;
            lamp.p1 = 100;
            let mut context = Context::new(Some(7500));
            refill(&mut lamp, &mut context);
            assert_eq!(lamp.p1, 100);
            assert_eq!(context.count, 2);
            assert_eq!(context.searches, 0);
            assert_eq!(context.events, ["But you are not using a lamp."]);
        }
    }

    #[test]
    fn no_flask_leaves_lamp_and_inventory_unchanged() {
        let mut lamp = blank_item();
        lamp.subval = 9;
        lamp.p1 = 100;
        let mut context = Context::new(None);
        refill(&mut lamp, &mut context);
        assert_eq!(lamp.p1, 100);
        assert_eq!(context.count, 2);
        assert_eq!(context.searches, 1);
        assert_eq!(context.events, ["You have no oil."]);
    }
}
