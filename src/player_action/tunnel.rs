use crate::model::{Cave, FloorType, Item, ItemType, WornFlag1};

#[cfg(not(test))]
mod globals;
#[cfg(not(test))]
mod interop;

/// Snapshot access and ordered C effects for tunneling on the game thread.
trait TunnelContext {
    /// Returns a bounded target snapshot and its optional item.
    fn read(&mut self, y: i64, x: i64) -> Option<(Cave, Option<Item>)>;
    /// Calls C wall removal, preserving its terrain and lighting effects.
    fn wall(&mut self, y: i64, x: i64, ability: i64, difficulty: i64) -> bool;
    /// Frees the rubble index before clearing the tile's item, mark, and obstruction.
    fn remove_rubble(&mut self, y: i64, x: i64, index: u8);
    /// Places a random dungeon item using the C helper.
    fn loot(&mut self, y: i64, x: i64);
    /// Checks target visibility after loot placement.
    fn lit(&mut self, y: i64, x: i64) -> bool;
    /// Refreshes the target after rubble and loot effects are committed.
    fn redraw(&mut self, y: i64, x: i64);
    /// Searches around the player using their current skill and condition.
    fn search(&mut self);
    /// Delivers a player-facing message in action order.
    fn message(&mut self, message: &str);
}

/// Tunnel directions can target permanent map borders, unlike movement destinations.
fn read_target(
    cells: &crate::dungeon::cell::Cells<'_>,
    y: i64,
    x: i64,
) -> Option<(Cave, Option<Item>)> {
    if y < 1 || y > cells.height || x < 1 || x > cells.width {
        return None;
    }
    let cell = *cells.cave.get(y as usize)?.get(x as usize)?;
    let item = (cell.tptr != 0)
        .then(|| cells.items.get(cell.tptr as usize).copied())
        .flatten();
    Some((cell, item))
}

fn tunnel(
    context: &mut impl TunnelContext,
    roll: &mut impl FnMut(i64) -> i64,
    y: i64,
    x: i64,
    strength: i64,
    weapon: Item,
) {
    let Some((cell, item)) = context.read(y, x) else {
        return;
    };
    let mut ability = (strength * 10 + 20) / 5;
    let has_equipped_weapon = weapon.tval != 0;
    if has_equipped_weapon && weapon.has_wornflag1(WornFlag1::ImprovedTunneling) {
        ability += 25 + weapon.p1 * 50;
    }
    match cell.floor_type() {
        Some(terrain @ (FloorType::WallGranite | FloorType::WallMagma | FloorType::WallQuartz)) => {
            let (max, offset, message) = match terrain {
                FloorType::WallGranite => (1200, 80, "You tunnel into the granite wall."),
                FloorType::WallMagma => (600, 10, "You tunnel into the magma intrusion."),
                _ => (400, 10, "You tunnel into the quartz vein."),
            };
            if context.wall(y, x, ability, roll(max) + offset) {
                context.message("You have finished the tunnel.");
            } else {
                context.message(message);
            }
        }
        Some(FloorType::BoundaryWall) => context.message("This seems to be permanent rock."),
        Some(FloorType::WaterOnFloor) => context.message("You can't tunnel through water!"),
        _ => match item.map(|item| item.tval) {
            Some(tval) if tval == u8::from(ItemType::Rubble) => {
                if ability > roll(180) {
                    context.remove_rubble(y, x, cell.tptr);
                    context.message("You have removed the rubble.");
                    if roll(10) == 1 {
                        context.loot(y, x);
                        if context.lit(y, x) {
                            context.message("You have found something!");
                        }
                    }
                    context.redraw(y, x);
                } else {
                    context.message("You dig in the rubble...");
                }
            }
            Some(tval) if tval == u8::from(ItemType::SecretDoor) => {
                context.message("You tunnel into the granite wall.");
                context.search();
            }
            Some(_) => context.message("You can't tunnel through that."),
            None => context.message("Tunnel through what?  Empty air???"),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::cell::test_support::{blank_item, TestMap};
    use rand::{SeedableRng, StdRng};

    struct Context {
        map: TestMap,
        messages: Vec<String>,
        events: Vec<&'static str>,
        walls: Vec<(i64, i64)>,
        finished: bool,
        light: bool,
    }

    impl Context {
        fn new() -> Self {
            Self {
                map: TestMap::reset(),
                messages: Vec::new(),
                events: Vec::new(),
                walls: Vec::new(),
                finished: false,
                light: true,
            }
        }

        fn tunnel(&mut self, strength: i64, weapon: Item) {
            let mut rng = StdRng::from_seed(&[1, 2, 3, 4][..]);
            tunnel(
                self,
                &mut |max| crate::rng::randint_with_rng(&mut rng, max),
                3,
                4,
                strength,
                weapon,
            );
        }
    }

    impl TunnelContext for Context {
        fn read(&mut self, y: i64, x: i64) -> Option<(Cave, Option<Item>)> {
            read_target(&self.map.cells(), y, x)
        }
        fn wall(&mut self, _y: i64, _x: i64, ability: i64, difficulty: i64) -> bool {
            self.walls.push((ability, difficulty));
            self.finished
        }
        fn remove_rubble(&mut self, y: i64, x: i64, index: u8) {
            assert_eq!(self.map.cave[y as usize][x as usize].tptr, index);
            let cell = &mut self.map.cave[y as usize][x as usize];
            cell.tptr = 0;
            cell.fm = 0;
            cell.fopen = 1;
            self.events.push("remove");
        }
        fn loot(&mut self, _y: i64, _x: i64) {
            self.events.push("loot");
        }
        fn lit(&mut self, _y: i64, _x: i64) -> bool {
            self.events.push("light");
            self.light
        }
        fn redraw(&mut self, _y: i64, _x: i64) {
            self.events.push("redraw");
        }
        fn search(&mut self) {
            self.events.push("search");
        }
        fn message(&mut self, message: &str) {
            self.events.push("message");
            self.messages.push(message.to_owned());
        }
    }

    #[test]
    fn seeded_wall_difficulties_and_tool_bonus_match_legacy() {
        let mut context = Context::new();
        let mut weapon = blank_item();
        weapon.tval = 1;
        weapon.apply_wornflag1(WornFlag1::ImprovedTunneling);
        weapon.p1 = 2;
        for (terrain, max, offset, message) in [
            (10, 1200, 80, "You tunnel into the granite wall."),
            (11, 600, 10, "You tunnel into the magma intrusion."),
            (12, 400, 10, "You tunnel into the quartz vein."),
        ] {
            context.map.cave[3][4].fval = terrain;
            context.messages.clear();
            context.walls.clear();
            context.tunnel(18, weapon);
            let mut rng = StdRng::from_seed(&[1, 2, 3, 4][..]);
            assert_eq!(
                context.walls,
                [(165, crate::rng::randint_with_rng(&mut rng, max) + offset)]
            );
            assert_eq!(context.messages, [message]);
        }
    }

    #[test]
    fn completed_wall_and_empty_weapon_match_legacy() {
        let mut context = Context::new();
        context.map.cave[3][4].fval = 10;
        context.finished = true;
        let mut weapon = blank_item();
        weapon.apply_wornflag1(WornFlag1::ImprovedTunneling);
        weapon.p1 = 10;
        context.tunnel(18, weapon);
        assert_eq!(context.walls[0].0, 40);
        assert_eq!(context.messages, ["You have finished the tunnel."]);
    }

    #[test]
    fn ordinary_weapon_does_not_add_a_tunneling_bonus() {
        let mut context = Context::new();
        context.map.cave[3][4].fval = 10;
        let mut weapon = blank_item();
        weapon.tval = 1;
        weapon.p1 = 10;
        context.tunnel(18, weapon);
        assert_eq!(context.walls[0].0, 40);
    }

    #[test]
    fn invalid_targets_have_no_effects_or_rng_draws() {
        let mut context = Context::new();
        for (row, col) in [(-1, 4), (0, 4), (7, 4), (3, i64::MAX)] {
            tunnel(
                &mut context,
                &mut |_| panic!("invalid targets do not roll"),
                row,
                col,
                18,
                blank_item(),
            );
        }
        assert!(context.events.is_empty());
        assert!(context.walls.is_empty());
    }

    #[test]
    fn permanent_border_targets_keep_their_message_without_rng_draws() {
        let mut context = Context::new();
        for (row, col) in [(1, 3), (6, 3), (3, 1), (3, 6)] {
            context.map.cave[row as usize][col as usize].fval = 15;
            let original = context.map.cave[row as usize][col as usize];
            context.messages.clear();
            tunnel(
                &mut context,
                &mut |_| panic!("permanent borders do not roll"),
                row,
                col,
                18,
                blank_item(),
            );
            assert_eq!(context.messages, ["This seems to be permanent rock."]);
            assert_eq!(
                context.map.cave[row as usize][col as usize].fval,
                original.fval
            );
        }
        assert!(context.walls.is_empty());
    }

    #[test]
    fn rubble_requires_ability_strictly_above_roll() {
        let mut context = Context::new();
        context
            .map
            .put(3, 4, 1, u8::from(ItemType::Rubble), b"rubble");
        tunnel(
            &mut context,
            &mut |max| {
                assert_eq!(max, 180);
                40
            },
            3,
            4,
            18,
            blank_item(),
        );
        assert_eq!(context.messages, ["You dig in the rubble..."]);
        assert_eq!(context.map.cave[3][4].tptr, 1);
    }

    #[test]
    fn rubble_loot_and_visibility_precede_redraw() {
        let mut context = Context::new();
        for light in [true, false] {
            context
                .map
                .put(3, 4, 1, u8::from(ItemType::Rubble), b"rubble");
            context.map.cave[3][4].fm = 1;
            context.light = light;
            context.events.clear();
            context.messages.clear();
            tunnel(&mut context, &mut |_| 1, 3, 4, 18, blank_item());
            assert_eq!(context.map.cave[3][4].tptr, 0);
            assert_eq!(context.map.cave[3][4].fm, 0);
            assert_eq!(context.map.cave[3][4].fopen, 1);
            if light {
                assert_eq!(
                    context.events,
                    ["remove", "message", "loot", "light", "message", "redraw"]
                );
                assert_eq!(
                    context.messages,
                    ["You have removed the rubble.", "You have found something!"]
                );
            } else {
                assert_eq!(
                    context.events,
                    ["remove", "message", "loot", "light", "redraw"]
                );
                assert_eq!(context.messages, ["You have removed the rubble."]);
            }
        }
    }

    #[test]
    fn seeded_rubble_removal_consumes_loot_roll() {
        let mut context = Context::new();
        context
            .map
            .put(3, 4, 1, u8::from(ItemType::Rubble), b"rubble");
        context.tunnel(100, blank_item());
        let mut rng = StdRng::from_seed(&[1, 2, 3, 4][..]);
        crate::rng::randint_with_rng(&mut rng, 180);
        let loot = crate::rng::randint_with_rng(&mut rng, 10) == 1;
        assert_eq!(context.map.cave[3][4].tptr, 0);
        assert_eq!(context.events.contains(&"loot"), loot);
        assert_eq!(context.events.last(), Some(&"redraw"));
    }

    #[test]
    fn secret_door_search_follows_granite_message_without_tunnel_roll() {
        let mut context = Context::new();
        context
            .map
            .put(3, 4, 1, u8::from(ItemType::SecretDoor), b"secret door");
        tunnel(
            &mut context,
            &mut |_| panic!("secret doors do not roll"),
            3,
            4,
            18,
            blank_item(),
        );
        assert_eq!(context.messages, ["You tunnel into the granite wall."]);
        assert_eq!(context.events, ["message", "search"]);
    }

    #[test]
    fn permanent_water_door_monster_and_empty_targets_match_legacy_messages() {
        let mut context = Context::new();
        for (terrain, tval, monster, message) in [
            (15, 0, 0, "This seems to be permanent rock."),
            (16, 0, 0, "You can't tunnel through water!"),
            (5, 105, 0, "You can't tunnel through that."),
            (5, 105, 2, "You can't tunnel through that."),
            (1, 0, 2, "Tunnel through what?  Empty air???"),
            (1, 0, 0, "Tunnel through what?  Empty air???"),
        ] {
            context.map.put(3, 4, 1, tval, b"target");
            context.map.cave[3][4].fval = terrain;
            context.map.cave[3][4].cptr = monster;
            if tval == 0 {
                context.map.cave[3][4].tptr = 0;
            }
            context.messages.clear();
            context.events.clear();
            tunnel(
                &mut context,
                &mut |_| panic!("target does not roll"),
                3,
                4,
                18,
                blank_item(),
            );
            assert_eq!(context.messages, [message]);
            assert_eq!(context.events, ["message"]);
        }
    }
}
