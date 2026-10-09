use crate::model::{Cave, Item, ItemType};

#[cfg(not(test))]
mod globals;
#[cfg(not(test))]
mod interop;

const MAX_SIGHT: usize = 20;

trait LookContext {
    fn blind(&mut self) -> bool;
    fn step(&mut self, direction: i64, y: &mut i64, x: &mut i64);
    fn read(&mut self, y: i64, x: i64) -> Option<(Cave, Option<Item>)>;
    fn monster_name(&mut self, index: u8) -> Option<String>;
    fn item_name(&mut self, item: Item) -> String;
    fn message(&mut self, message: &str);
}

fn look(context: &mut impl LookContext, direction: i64, mut y: i64, mut x: i64) {
    if context.blind() {
        context.message("You can't see a damn thing!");
        return;
    }

    let mut seen = false;
    // Legacy look scans MAX_SIGHT + 1 tiles, including the first blocking tile.
    for _ in 0..=MAX_SIGHT {
        context.step(direction, &mut y, &mut x);
        let Some((cell, item)) = context.read(y, x) else {
            break;
        };
        if cell.cptr > 1 {
            if let Some(name) = context.monster_name(cell.cptr) {
                let article = article(&name);
                context.message(&format!("You see {article} {name}."));
                seen = true;
            }
        }

        if cell.tl != 0 || cell.pl != 0 || cell.fm != 0 {
            if let Some(mut item) = item {
                match item.item_type() {
                    Some(ItemType::SecretDoor) => context.message("You see a granite wall."),
                    Some(ItemType::UnseenTrap) => {}
                    _ => {
                        item.number = 1;
                        let name = context.item_name(item);
                        let article = article(&name);
                        context.message(&format!("You see {article} {name}."));
                        seen = true;
                    }
                }
            }

            if cell.fopen == 0 {
                seen = true;
                match cell.fval {
                    10 | 15 => context.message("You see a granite wall."),
                    11 => context.message("You see some dark rock."),
                    12 => context.message("You see a quartz vein."),
                    _ => {}
                }
            } else if matches!(cell.fval, 16 | 17) {
                seen = true;
                context.message("You see some water.");
            }
        }
        if cell.fopen == 0 {
            break;
        }
    }
    if !seen {
        context.message("You see nothing of interest in that direction.");
    }
}

fn article(name: &str) -> &'static str {
    if name
        .chars()
        .next()
        .is_some_and(crate::pascal::is_vowel_char)
    {
        "an"
    } else {
        "a"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dungeon::cell::test_support::TestMap;

    struct Context {
        map: TestMap,
        blind: bool,
        messages: Vec<String>,
        steps: usize,
    }

    impl Context {
        fn new() -> Self {
            let mut map = TestMap::reset();
            map.cave[3][4].fopen = 1;
            Self {
                map,
                blind: false,
                messages: Vec::new(),
                steps: 0,
            }
        }
    }

    impl LookContext for Context {
        fn blind(&mut self) -> bool {
            self.blind
        }
        fn step(&mut self, direction: i64, y: &mut i64, x: &mut i64) {
            self.steps += 1;
            *y += match direction {
                1..=3 => 1,
                7..=9 => -1,
                _ => 0,
            };
            *x += match direction {
                1 | 4 | 7 => -1,
                3 | 6 | 9 => 1,
                _ => 0,
            };
        }
        fn read(&mut self, y: i64, x: i64) -> Option<(Cave, Option<Item>)> {
            let height = self.map.cave.len() as i64 - 1;
            crate::dungeon::cell::Cells {
                cave: &mut self.map.cave,
                items: &mut self.map.items,
                height,
                width: crate::constants::MAX_WIDTH as i64,
            }
            .read(y, x)
        }
        fn monster_name(&mut self, index: u8) -> Option<String> {
            match index {
                2 => Some("orc".to_owned()),
                4 => Some("bat".to_owned()),
                _ => None,
            }
        }
        fn item_name(&mut self, item: Item) -> String {
            assert_eq!(item.number, 1);
            if item.tval == 80 {
                "apple".to_owned()
            } else {
                "sword".to_owned()
            }
        }
        fn message(&mut self, message: &str) {
            self.messages.push(message.to_owned());
        }
    }

    #[test]
    fn reports_monster_then_object_on_the_ray() {
        let mut context = Context::new();
        context.map.put(3, 4, 1, 80, b"apple");
        context.map.items[1].number = 5;
        context.map.cave[3][4].cptr = 2;
        context.map.cave[3][4].pl = 1;
        look(&mut context, 6, 3, 3);
        assert_eq!(context.messages, ["You see an orc.", "You see an apple."]);
        assert_eq!(context.map.items[1].number, 5);
    }

    #[test]
    fn empty_ray_reports_nothing_of_interest() {
        let mut context = Context::new();
        look(&mut context, 6, 3, 3);
        assert_eq!(
            context.messages,
            ["You see nothing of interest in that direction."]
        );
    }

    #[test]
    fn blindness_stops_before_reading_the_ray() {
        let mut context = Context::new();
        context.blind = true;
        look(&mut context, 6, 3, 3);
        assert_eq!(context.messages, ["You can't see a damn thing!"]);
        assert_eq!(context.steps, 0);
    }

    #[test]
    fn each_lighting_flag_reveals_objects() {
        for lighting in [0, 1, 2] {
            let mut context = Context::new();
            context.map.put(3, 4, 1, 23, b"sword");
            let cell = &mut context.map.cave[3][4];
            match lighting {
                0 => cell.pl = 1,
                1 => cell.tl = 1,
                _ => cell.fm = 1,
            }
            look(&mut context, 6, 3, 3);
            assert_eq!(context.messages, ["You see a sword."]);
        }
    }

    #[test]
    fn unseen_monsters_and_unlit_objects_are_not_reported() {
        let mut context = Context::new();
        context.map.put(3, 4, 1, 80, b"apple");
        context.map.cave[3][4].cptr = 3;
        look(&mut context, 6, 3, 3);
        assert_eq!(
            context.messages,
            ["You see nothing of interest in that direction."]
        );
    }

    #[test]
    fn seen_monster_is_reported_without_tile_lighting() {
        let mut context = Context::new();
        context.map.cave[3][4].cptr = 4;
        look(&mut context, 6, 3, 3);
        assert_eq!(context.messages, ["You see a bat."]);
    }

    #[test]
    fn lit_unseen_trap_is_not_reported() {
        let mut context = Context::new();
        context.map.put(3, 4, 1, 101, b"trap");
        context.map.cave[3][4].pl = 1;
        look(&mut context, 6, 3, 3);
        assert_eq!(
            context.messages,
            ["You see nothing of interest in that direction."]
        );
    }

    #[test]
    fn secret_door_keeps_both_legacy_wall_messages() {
        let mut context = Context::new();
        context.map.put(3, 4, 1, 109, b"door");
        context.map.cave[3][4].pl = 1;
        context.map.cave[3][4].fopen = 0;
        context.map.cave[3][4].fval = 10;
        look(&mut context, 6, 3, 3);
        assert_eq!(
            context.messages,
            ["You see a granite wall.", "You see a granite wall."]
        );
        assert_eq!(context.steps, 1);
    }

    #[test]
    fn secret_door_on_open_tile_keeps_legacy_empty_summary() {
        let mut context = Context::new();
        context.map.put(3, 4, 1, 109, b"door");
        context.map.cave[3][4].pl = 1;
        look(&mut context, 6, 3, 3);
        assert_eq!(
            context.messages,
            [
                "You see a granite wall.",
                "You see nothing of interest in that direction."
            ]
        );
    }

    #[test]
    fn walls_report_the_legacy_material_and_stop_the_ray() {
        for (material, message) in [
            (10, "You see a granite wall."),
            (11, "You see some dark rock."),
            (12, "You see a quartz vein."),
            (15, "You see a granite wall."),
        ] {
            let mut context = Context::new();
            context.map.cave[3][4].fopen = 0;
            context.map.cave[3][4].fval = material;
            context.map.cave[3][4].pl = 1;
            context.map.cave[3][5].cptr = 2;
            look(&mut context, 6, 3, 3);
            assert_eq!(context.messages, [message]);
            assert_eq!(context.steps, 1);
        }
    }

    #[test]
    fn both_water_floors_are_reported_after_objects() {
        for floor in [16, 17] {
            let mut context = Context::new();
            context.map.put(3, 4, 1, 80, b"apple");
            context.map.cave[3][4].fval = floor;
            context.map.cave[3][4].tl = 1;
            look(&mut context, 6, 3, 3);
            assert_eq!(
                context.messages,
                ["You see an apple.", "You see some water."]
            );
        }
    }

    #[test]
    fn ray_reports_distant_monsters_and_uses_legacy_sight_limit() {
        let mut context = Context::new();
        for col in 4..=25 {
            context.map.cave[3][col].fopen = 1;
        }
        context.map.cave[3][24].cptr = 2;
        context.map.cave[3][25].cptr = 4;
        look(&mut context, 6, 3, 3);
        assert_eq!(context.messages, ["You see an orc."]);
        assert_eq!(context.steps, 21);
    }

    #[test]
    fn map_edge_stops_without_indexing_outside_the_map() {
        let mut context = Context::new();
        look(&mut context, 4, 3, 2);
        assert_eq!(
            context.messages,
            ["You see nothing of interest in that direction."]
        );
        assert_eq!(context.steps, 1);
    }
}
