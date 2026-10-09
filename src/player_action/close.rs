use crate::{
    dungeon::door::{self, CloseResult},
    model::{Cave, Item},
};

#[cfg(not(test))]
mod globals;
#[cfg(not(test))]
mod interop;

/// Dungeon access, door templates, and display effects needed to close a door.
///
/// The game implementation bridges C globals and UI calls; tests use an
/// in-memory map. Changes to cell and item snapshots take effect only on write.
trait CloseContext {
    /// Returns copies of the cell at row `y`, column `x` and its item.
    ///
    /// The outer `None` means the cell is outside the accessible map. The inner
    /// `None` means there is no item or its index is invalid.
    fn read(&mut self, y: i64, x: i64) -> Option<(Cave, Option<Item>)>;
    /// Stores the cell and replaces the item referenced by `cell.tptr`.
    ///
    /// Does nothing if the coordinates or item index are invalid; does not redraw.
    fn write(&mut self, y: i64, x: i64, cell: Cave, item: Item);
    /// Returns a copy of the closed-door template used to replace an open door.
    fn closed_door(&mut self) -> Item;
    /// Formats a monster instance's name for the start of a sentence.
    fn monster_name(&mut self, index: u8) -> String;
    /// Sends a player-facing message to the game's message display.
    fn message(&mut self, message: &str);
    /// Refreshes the displayed tile at row `y`, column `x` after a successful write.
    fn redraw(&mut self, y: i64, x: i64);
}

fn close(context: &mut impl CloseContext, y: i64, x: i64) {
    let result = match context.read(y, x) {
        Some((mut cell, Some(mut item))) => {
            let result = door::close(&mut cell, &mut item, context.closed_door());
            if result == CloseResult::Closed {
                context.write(y, x, cell, item);
            }
            result
        }
        _ => CloseResult::NotDoor,
    };
    match result {
        CloseResult::Closed => context.redraw(y, x),
        CloseResult::Monster(index) => {
            let name = context.monster_name(index);
            context.message(&format!("{name} is in your way!"));
        }
        CloseResult::Broken => context.message("The door appears to be broken."),
        CloseResult::NotDoor => context.message("I do not see anything you can close there."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::cell::test_support::{blank_item, TestMap};

    struct Context {
        map: TestMap,
        template: Item,
        messages: Vec<String>,
        redraws: Vec<(i64, i64)>,
        writes: usize,
        named: Vec<u8>,
    }

    impl Context {
        fn new() -> Self {
            let mut map = TestMap::reset();
            map.put(3, 4, 1, 104, b"an open door");
            map.cave[3][4].fopen = 1;
            map.cave[3][4].fval = 5;
            let mut template = blank_item();
            template.tval = 105;
            template.name[0] = b'+' as libc::c_char;
            template.flags = 42;
            template.cost = 17;
            Self {
                map,
                template,
                messages: Vec::new(),
                redraws: Vec::new(),
                writes: 0,
                named: Vec::new(),
            }
        }

        fn assert_refused(&self, message: &str) {
            assert_eq!(self.messages, [message]);
            assert!(self.redraws.is_empty());
            assert_eq!(self.writes, 0);
            assert_eq!(self.map.cave[3][4].fopen, 1);
        }
    }

    impl CloseContext for Context {
        fn read(&mut self, y: i64, x: i64) -> Option<(Cave, Option<Item>)> {
            self.map.cells().read(y, x)
        }
        fn write(&mut self, y: i64, x: i64, cell: Cave, item: Item) {
            self.writes += 1;
            self.map.cells().write(y, x, cell, item);
        }
        fn closed_door(&mut self) -> Item {
            self.template
        }
        fn monster_name(&mut self, index: u8) -> String {
            self.named.push(index);
            "The orc".to_owned()
        }
        fn message(&mut self, message: &str) {
            self.messages.push(message.to_owned());
        }
        fn redraw(&mut self, y: i64, x: i64) {
            assert_eq!(self.map.cave[y as usize][x as usize].fopen, 0);
            assert_eq!(self.map.items[1].tval, 105);
            self.redraws.push((y, x));
        }
    }

    #[test]
    fn closes_open_door_with_template_and_redraws_after_writing() {
        let mut context = Context::new();
        close(&mut context, 3, 4);
        let cell = context.map.cave[3][4];
        let item = context.map.items[1];
        assert_eq!(cell.fopen, 0);
        assert_eq!(cell.tptr, 1);
        assert_eq!(cell.fval, 5);
        assert_eq!(item.tval, 105);
        assert_eq!(item.name, context.template.name);
        assert_eq!(item.flags, 42);
        assert_eq!(item.cost, 17);
        assert_eq!(item.subval, context.template.subval);
        assert_eq!(context.writes, 1);
        assert_eq!(context.redraws, [(3, 4)]);
        assert!(context.messages.is_empty());
        assert!(context.named.is_empty());
    }

    #[test]
    fn monster_blocks_even_a_broken_door_with_legacy_message() {
        let mut context = Context::new();
        context.map.cave[3][4].cptr = 2;
        context.map.items[1].p1 = 1;
        close(&mut context, 3, 4);
        context.assert_refused("The orc is in your way!");
        assert_eq!(context.named, [2]);
        assert_eq!(context.map.items[1].p1, 1);
        assert_eq!(context.map.items[1].tval, 104);
    }

    #[test]
    fn broken_door_is_unchanged_with_legacy_message() {
        for broken in [-1, 1] {
            let mut context = Context::new();
            context.map.items[1].p1 = broken;
            close(&mut context, 3, 4);
            context.assert_refused("The door appears to be broken.");
            assert_eq!(context.map.items[1].p1, broken);
            assert_eq!(context.map.items[1].tval, 104);
            assert!(context.named.is_empty());
        }
    }

    #[test]
    fn non_door_target_is_unchanged_even_with_a_monster() {
        let mut context = Context::new();
        context.map.items[1].tval = 105;
        context.map.cave[3][4].cptr = 2;
        close(&mut context, 3, 4);
        context.assert_refused("I do not see anything you can close there.");
        assert_eq!(context.map.items[1].tval, 105);
        assert!(context.named.is_empty());
    }

    #[test]
    fn empty_target_is_unchanged_with_legacy_message() {
        let mut context = Context::new();
        context.map.cave[3][4].tptr = 0;
        close(&mut context, 3, 4);
        context.assert_refused("I do not see anything you can close there.");
        assert_eq!(context.map.cave[3][4].tptr, 0);
    }

    #[test]
    fn invalid_targets_do_not_write_or_redraw() {
        let mut context = Context::new();
        for (y, x) in [(1, 4), (3, 6), (-1, 4), (i64::MAX, 4)] {
            context.messages.clear();
            close(&mut context, y, x);
            context.assert_refused("I do not see anything you can close there.");
        }
        context.map.cave[3][4].tptr = u8::MAX;
        context.messages.clear();
        close(&mut context, 3, 4);
        context.assert_refused("I do not see anything you can close there.");
    }
}
