use crate::{
    dungeon::door::{self, JamTarget},
    model::{Cave, Item},
};

#[cfg(not(test))]
mod globals;
#[cfg(not(test))]
mod interop;

trait JamContext {
    fn read(&mut self, y: i64, x: i64) -> Option<(Cave, Option<Item>)>;
    fn write(&mut self, y: i64, x: i64, cell: Cave, item: Item);
    fn find_spike(&mut self) -> Option<u16>;
    fn set_spike_count(&mut self, count: u16);
    fn destroy_spike(&mut self);
    fn monster_name(&mut self, index: u8) -> String;
    fn message(&mut self, message: &str);
    fn redraw_stats(&mut self);
}

fn jam(context: &mut impl JamContext, y: i64, x: i64) {
    let Some((cell, Some(mut item))) = context.read(y, x) else {
        context.message("That isn't a door!");
        return;
    };
    match door::jam_target(&cell, &item) {
        JamTarget::Closed => {
            let Some(count) = context.find_spike() else {
                context.message("But you have no spikes...");
                return;
            };
            context.message("You jam the door with a spike.");
            if count > 1 {
                context.set_spike_count(count - 1);
            } else {
                context.destroy_spike();
            }
            context.redraw_stats();
            item.p1 = -item.p1.abs() - 20;
            context.write(y, x, cell, item);
        }
        JamTarget::Monster(index) => {
            let name = context.monster_name(index);
            context.message(&format!("{name} is in your way!"));
        }
        JamTarget::Open => context.message("The door must be closed first."),
        JamTarget::NotDoor => context.message("That isn't a door!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::cell::test_support::TestMap;

    struct Context {
        map: TestMap,
        spikes: Option<u16>,
        messages: Vec<String>,
        events: Vec<&'static str>,
        named: Vec<u8>,
    }

    impl Context {
        fn new() -> Self {
            let mut map = TestMap::reset();
            map.put(3, 4, 1, 105, b"a closed door");
            Self {
                map,
                spikes: Some(2),
                messages: Vec::new(),
                events: Vec::new(),
                named: Vec::new(),
            }
        }

        fn assert_refused(&self, message: &str) {
            assert_eq!(self.messages, [message]);
            assert!(!self.events.contains(&"write"));
            assert!(!self.events.contains(&"redraw"));
            assert!(!self.events.contains(&"decrement"));
            assert!(!self.events.contains(&"destroy"));
        }
    }

    impl JamContext for Context {
        fn read(&mut self, y: i64, x: i64) -> Option<(Cave, Option<Item>)> {
            self.map.cells().read(y, x)
        }
        fn write(&mut self, y: i64, x: i64, cell: Cave, item: Item) {
            self.events.push("write");
            self.map.cells().write(y, x, cell, item);
        }
        fn find_spike(&mut self) -> Option<u16> {
            self.events.push("find");
            self.spikes
        }
        fn set_spike_count(&mut self, count: u16) {
            self.events.push("decrement");
            self.spikes = Some(count);
        }
        fn destroy_spike(&mut self) {
            self.events.push("destroy");
            self.spikes = None;
        }
        fn monster_name(&mut self, index: u8) -> String {
            self.named.push(index);
            "The orc".to_owned()
        }
        fn message(&mut self, message: &str) {
            self.events.push("message");
            self.messages.push(message.to_owned());
        }
        fn redraw_stats(&mut self) {
            self.events.push("redraw");
        }
    }

    #[test]
    fn jams_closed_door_consuming_one_spike_in_legacy_order() {
        let mut context = Context::new();
        context.map.items[1].p1 = 7;
        jam(&mut context, 3, 4);
        assert_eq!(context.map.items[1].p1, -27);
        assert_eq!(context.map.items[1].tval, 105);
        assert_eq!(context.map.cave[3][4].fopen, 0);
        assert_eq!(context.spikes, Some(1));
        assert_eq!(context.messages, ["You jam the door with a spike."]);
        assert_eq!(
            context.events,
            ["find", "message", "decrement", "redraw", "write"]
        );
        assert!(context.named.is_empty());
    }

    #[test]
    fn destroys_the_final_spike_and_increases_an_existing_jam() {
        let mut context = Context::new();
        context.spikes = Some(1);
        context.map.items[1].p1 = -40;
        jam(&mut context, 3, 4);
        assert_eq!(context.map.items[1].p1, -60);
        assert_eq!(context.spikes, None);
        assert_eq!(
            context.events,
            ["find", "message", "destroy", "redraw", "write"]
        );
    }

    #[test]
    fn no_spikes_leaves_closed_door_unchanged() {
        let mut context = Context::new();
        context.spikes = None;
        context.map.items[1].p1 = 7;
        jam(&mut context, 3, 4);
        context.assert_refused("But you have no spikes...");
        assert_eq!(context.map.items[1].p1, 7);
        assert_eq!(context.spikes, None);
        assert_eq!(context.events, ["find", "message"]);
    }

    #[test]
    fn monster_blocks_a_closed_door_before_inventory_lookup() {
        let mut context = Context::new();
        context.map.cave[3][4].cptr = 2;
        jam(&mut context, 3, 4);
        context.assert_refused("The orc is in your way!");
        assert_eq!(context.named, [2]);
        assert_eq!(context.events, ["message"]);
        assert_eq!(context.spikes, Some(2));
        assert_eq!(context.map.items[1].p1, 0);
    }

    #[test]
    fn open_door_refusal_precedes_monster_and_inventory_checks() {
        let mut context = Context::new();
        context.map.items[1].tval = 104;
        context.map.cave[3][4].cptr = 2;
        jam(&mut context, 3, 4);
        context.assert_refused("The door must be closed first.");
        assert_eq!(context.events, ["message"]);
        assert!(context.named.is_empty());
        assert_eq!(context.spikes, Some(2));
    }

    #[test]
    fn non_door_and_empty_targets_do_not_consume_spikes() {
        let mut context = Context::new();
        context.map.items[1].tval = 80;
        jam(&mut context, 3, 4);
        context.assert_refused("That isn't a door!");
        assert_eq!(context.events, ["message"]);
        context.messages.clear();
        context.events.clear();
        context.map.cave[3][4].tptr = 0;
        jam(&mut context, 3, 4);
        context.assert_refused("That isn't a door!");
        assert_eq!(context.events, ["message"]);
        assert_eq!(context.spikes, Some(2));
    }

    #[test]
    fn invalid_targets_do_not_access_inventory_or_change_door() {
        let mut context = Context::new();
        for (row, col) in [(1, 4), (3, 6), (-1, 4), (i64::MAX, 4)] {
            context.messages.clear();
            context.events.clear();
            jam(&mut context, row, col);
            context.assert_refused("That isn't a door!");
            assert_eq!(context.events, ["message"]);
        }
        context.messages.clear();
        context.events.clear();
        context.map.cave[3][4].tptr = u8::MAX;
        jam(&mut context, 3, 4);
        context.assert_refused("That isn't a door!");
        assert_eq!(context.spikes, Some(2));
        assert_eq!(context.map.items[1].p1, 0);
    }
}
