use super::layout::{current_layout, draw_lines, render_lines, Layout, LayoutKind, Rect};
use crate::{data::item_name, equipment, inventory::not_safe::inventory_clone, ncurses};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EquipmentSlot {
    pub name: &'static str,
    pub item_name: String,
}

#[derive(Clone, Debug)]
struct SeenMonster {
    name: String,
    symbol: char,
    distance: i16,
    y: i64,
    x: i64,
}

fn monster_snapshot(monsters: &[crate::model::Monster]) -> Vec<SeenMonster> {
    monsters
        .iter()
        .map(|monster| {
            let template = crate::generate_monster::monster_template::MONSTER_TEMPLATES
                .get(monster.mptr as usize)
                .expect("seen monster has an invalid template index");
            SeenMonster {
                name: template.name.to_string(),
                symbol: template.symbol,
                distance: monster.cdis,
                y: monster.fy.into(),
                x: monster.fx.into(),
            }
        })
        .collect()
}

fn grouped_monsters(monsters: &[SeenMonster]) -> Vec<(&SeenMonster, usize)> {
    let mut groups = BTreeMap::new();
    for monster in monsters {
        let (nearest, count) = groups.entry(monster.name.as_str()).or_insert((monster, 0));
        *count += 1;
        if monster.distance < nearest.distance {
            *nearest = monster;
        }
    }
    let mut groups: Vec<_> = groups.into_values().collect();
    groups.sort_by_key(|(monster, _)| (monster.distance, monster.name.as_str()));
    groups
}

fn monster_position_for_panel_row(
    monsters: &[SeenMonster],
    panel_row: i32,
    blind: bool,
    hallucinating: bool,
) -> Option<(i64, i64)> {
    if blind || hallucinating || panel_row < 2 {
        return None;
    }
    grouped_monsters(monsters)
        .get((panel_row - 2) as usize)
        .map(|(monster, _)| (monster.y, monster.x))
}

#[cfg(not(test))]
pub(crate) fn current_monster_position_for_panel_row(panel_row: i32) -> Option<(i64, i64)> {
    let layout = current_layout();
    let rect = layout.monsters?;
    if panel_row >= rect.height {
        return None;
    }
    extern "C" {
        static player_flags: crate::model::PlayerFlags;
    }
    let flags = unsafe { player_flags };
    let monsters = monster_snapshot(&crate::model::seen_monsters());
    monster_position_for_panel_row(&monsters, panel_row, flags.blind > 0, flags.image > 0)
}

fn render_monsters_panel(
    rect: Rect,
    monsters: &[SeenMonster],
    blind: bool,
    hallucinating: bool,
) -> Vec<String> {
    let mut lines = vec![String::new(), "Visible monsters".to_string()];
    if !blind && !hallucinating {
        lines.extend(
            grouped_monsters(monsters)
                .into_iter()
                .map(|(monster, count)| format!("{} {} ({})", monster.symbol, monster.name, count)),
        );
    }
    render_lines(rect, &lines)
}

fn draw_monsters_panel(
    layout: Layout,
    monsters: &[SeenMonster],
    blind: bool,
    hallucinating: bool,
    draw: impl FnMut(i32, i32, &str),
) {
    if let Some(rect) = layout.monsters {
        let lines = render_monsters_panel(rect, monsters, blind, hallucinating);
        draw_lines(rect, &lines, draw);
    }
}

pub(crate) fn render_equipment_panel(rect: Rect, slots: &[EquipmentSlot]) -> Vec<String> {
    let lines: Vec<_> = slots
        .iter()
        .enumerate()
        .map(|(index, slot)| {
            let index_char = (b'a' + index as u8) as char;
            format!("{}) {:<13}: {}", index_char, slot.name, slot.item_name)
        })
        .collect();
    render_lines(rect, &lines)
}

fn render_inventory_panel(rect: Rect, names: &[String]) -> Vec<String> {
    let mut lines = vec![String::new(), "Inventory".to_string()];
    let item_height = (rect.height as usize).saturating_sub(lines.len());
    let visible = if names.len() > item_height {
        item_height.saturating_sub(1)
    } else {
        names.len()
    };
    lines.extend(
        names
            .iter()
            .take(visible)
            .enumerate()
            .map(|(index, name)| format!("{}) {}", (b'a' + index as u8) as char, name)),
    );
    if names.len() > visible {
        lines.push(format!("...{} more", names.len() - visible));
    }
    render_lines(rect, &lines)
}

fn inventory_snapshot() -> Vec<String> {
    inventory_clone()
        .iter()
        .map(|item| item_name::generate(&item.data))
        .collect()
}

fn full_item_description(item: &crate::model::Item) -> Option<String> {
    item.item_type()?;
    Some(item_name::generate(item))
}

pub(crate) fn inventory_item_description_for_panel_row(
    items: &[crate::model::InventoryItem],
    panel_height: i32,
    panel_row: i32,
) -> Option<String> {
    if panel_row < 2 {
        return None;
    }
    let item_rows = panel_height.saturating_sub(2).max(0) as usize;
    let visible_items = if items.len() > item_rows {
        item_rows.saturating_sub(1)
    } else {
        items.len()
    };
    let item_index = (panel_row - 2) as usize;
    if item_index >= visible_items {
        return None;
    }
    full_item_description(&items.get(item_index)?.data)
}

pub(crate) fn equipment_item_description_for_panel_row(
    items: &[crate::model::Item],
    panel_height: i32,
    panel_row: i32,
) -> Option<String> {
    if panel_row < 0 || panel_row >= panel_height {
        return None;
    }
    full_item_description(items.get(panel_row as usize)?)
}

#[cfg(not(test))]
pub(crate) fn current_equipment_item_description_for_panel_row(panel_row: i32) -> Option<String> {
    let rect = current_layout().equipment?;
    let items: Vec<_> = equipment::slots_iter()
        .map(|slot_index| unsafe { *equipment::get_item(equipment::Slot::from(slot_index)) })
        .collect();
    equipment_item_description_for_panel_row(&items, rect.height, panel_row)
}

#[cfg(not(test))]
pub(crate) fn current_inventory_item_description_for_panel_row(panel_row: i32) -> Option<String> {
    let rect = current_layout().inventory?;
    let items = inventory_clone();
    inventory_item_description_for_panel_row(&items, rect.height, panel_row)
}

fn draw_inventory_panel(layout: Layout, draw: impl FnMut(i32, i32, &str)) {
    if let Some(rect) = layout.inventory {
        let lines = render_inventory_panel(rect, &inventory_snapshot());
        draw_lines(rect, &lines, draw);
    }
}

fn draw_equipment_panel(
    layout: Layout,
    slots: &[EquipmentSlot],
    mut draw: impl FnMut(i32, i32, &str),
) {
    if let Some(rect) = layout.equipment {
        if layout.kind == LayoutKind::Split {
            // The sidebar's row zero is beside, not inside, the prompt line.
            let heading = Rect::new(rect.row - 1, rect.col, rect.width, 1);
            draw_lines(heading, &["Equipment"], &mut draw);
        }
        draw_lines(rect, &render_equipment_panel(rect, slots), draw);
    }
}

#[no_mangle]
pub extern "C" fn C_print_equipment_block() {
    let layout = current_layout();
    let slots: Vec<_> = equipment::slots_iter()
        .map(|slot_index| {
            let slot = equipment::Slot::from(slot_index);
            EquipmentSlot {
                name: slot.name(),
                item_name: equipment::get_name(slot),
            }
        })
        .collect();
    draw_equipment_panel(layout, &slots, |row, col, line| {
        ncurses::mvaddstr(row, col, line);
    });
    draw_inventory_panel(layout, |row, col, line| {
        ncurses::mvaddstr(row, col, line);
    });
    if layout.monsters.is_some() {
        extern "C" {
            static player_flags: crate::model::PlayerFlags;
        }
        let flags = unsafe { player_flags };
        let monsters = monster_snapshot(&crate::model::seen_monsters());
        draw_monsters_panel(
            layout,
            &monsters,
            flags.blind > 0,
            flags.image > 0,
            |row, col, line| {
                ncurses::mvaddstr(row, col, line);
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{InventoryItem, Item, ItemType};
    use crate::user_interface::layout::{
        compute_layout, CLASSIC_MIN_COLS, EQUIPMENT_HEIGHT, SPLIT_MIN_ROWS,
    };
    use std::ptr;

    fn seen(name: &str, symbol: char, distance: i16) -> SeenMonster {
        SeenMonster {
            name: name.to_string(),
            symbol,
            distance,
            y: 0,
            x: 0,
        }
    }

    fn seen_at(name: &str, symbol: char, distance: i16, y: i64, x: i64) -> SeenMonster {
        SeenMonster {
            name: name.to_string(),
            symbol,
            distance,
            y,
            x,
        }
    }

    #[test]
    fn monster_panel_entry_rows_resolve_to_nearest_visible_group_member() {
        let monsters = [
            seen_at("Orc", 'o', 8, 8, 9),
            seen_at("Troll", 'T', 4, 5, 6),
            seen_at("Orc", 'o', 1, 2, 3),
            seen_at("Bat", 'b', 6, 7, 8),
        ];

        assert_eq!(
            monster_position_for_panel_row(&monsters, 0, false, false),
            None
        );
        assert_eq!(
            monster_position_for_panel_row(&monsters, 1, false, false),
            None
        );
        assert_eq!(
            monster_position_for_panel_row(&monsters, 2, false, false),
            Some((2, 3))
        );
        assert_eq!(
            monster_position_for_panel_row(&monsters, 3, false, false),
            Some((5, 6))
        );
        assert_eq!(
            monster_position_for_panel_row(&monsters, 4, false, false),
            Some((7, 8))
        );
        assert_eq!(
            monster_position_for_panel_row(&monsters, 5, false, false),
            None
        );
        assert_eq!(
            monster_position_for_panel_row(&monsters, 2, true, false),
            None
        );
        assert_eq!(
            monster_position_for_panel_row(&monsters, 2, false, true),
            None
        );
    }

    #[test]
    fn monsters_panel_groups_names_and_orders_by_the_nearest_member() {
        let monsters = [
            seen("Orc", 'o', 9),
            seen("Troll", 'T', 4),
            seen("Orc", 'o', 1),
            seen("Bat", 'b', 6),
        ];
        let lines = render_monsters_panel(Rect::new(25, 81, 20, 5), &monsters, false, false);
        assert_eq!(
            lines.iter().map(|s| s.trim_end()).collect::<Vec<_>>(),
            [
                "",
                "Visible monsters",
                "o Orc (2)",
                "T Troll (1)",
                "b Bat (1)",
            ]
        );
    }

    #[test]
    fn monsters_panel_orders_equal_distances_by_name() {
        let monsters = [seen("Troll", 'T', 2), seen("Bat", 'b', 2)];
        let lines = render_monsters_panel(Rect::new(25, 81, 20, 4), &monsters, false, false);
        assert_eq!(lines[2].trim_end(), "b Bat (1)");
        assert_eq!(lines[3].trim_end(), "T Troll (1)");
    }

    #[test]
    fn monsters_panel_fits_names_and_rows_to_the_rectangle() {
        let monsters = [
            seen("\u{e9}normous bat", 'b', 1),
            seen("Orc", 'o', 2),
            seen("Troll", 'T', 3),
        ];
        assert_eq!(
            render_monsters_panel(Rect::new(25, 81, 8, 3), &monsters, false, false),
            ["        ", "Visible ", "b \u{e9}normo"]
        );
        assert!(render_monsters_panel(Rect::new(25, 81, 8, 0), &monsters, false, false).is_empty());
        assert_eq!(
            render_monsters_panel(Rect::new(25, 81, 0, 2), &monsters, false, false),
            ["", ""]
        );
    }

    #[test]
    fn monsters_panel_hides_all_entries_when_blind_or_hallucinating() {
        let monsters = [seen("Orc", 'o', 1)];
        for (blind, hallucinating) in [(true, false), (false, true), (true, true)] {
            assert_eq!(
                render_monsters_panel(Rect::new(25, 81, 16, 3), &monsters, blind, hallucinating),
                ["                ", "Visible monsters", "                "]
            );
        }
        assert_eq!(
            render_monsters_panel(Rect::new(25, 81, 16, 3), &[], false, false),
            ["                ", "Visible monsters", "                "]
        );
    }

    #[test]
    fn monster_snapshot_uses_template_symbols_names_and_instance_distances() {
        let monster = crate::model::Monster {
            mptr: 0,
            cdis: 7,
            is_seen: 1,
            ..Default::default()
        };
        let snapshot = monster_snapshot(&[monster]);
        assert_eq!(snapshot.len(), 1);
        assert_eq!(snapshot[0].name, "<<Placeholder>>");
        assert_eq!(snapshot[0].symbol, 'p');
        assert_eq!(snapshot[0].distance, 7);
    }

    #[test]
    #[should_panic(expected = "seen monster has an invalid template index")]
    fn monster_snapshot_rejects_an_invalid_template_index() {
        monster_snapshot(&[crate::model::Monster {
            mptr: u16::MAX,
            ..Default::default()
        }]);
    }

    #[test]
    fn monsters_panel_redraws_its_rect_and_clears_disappearing_entries() {
        let layout = compute_layout(SPLIT_MIN_ROWS, CLASSIC_MIN_COLS);
        let rect = layout.monsters.unwrap();
        let monsters = [seen("Orc", 'o', 1)];
        for (entries, blind, hallucinating) in [
            (&monsters[..], false, false),
            (&[][..], false, false),
            (&monsters[..], true, false),
            (&monsters[..], false, true),
        ] {
            let mut draws = Vec::new();
            draw_monsters_panel(layout, entries, blind, hallucinating, |row, col, line| {
                draws.push((row, col, line.to_string()));
            });
            assert_eq!(draws.len(), rect.height as usize);
            assert!(draws[0].2.trim().is_empty());
            assert_eq!(draws[1].2.trim_end(), "Visible monsters");
            assert!(draws.iter().enumerate().all(|(index, (row, col, line))| {
                *row == rect.row + index as i32
                    && *col == rect.col
                    && line.len() == rect.width as usize
            }));
            if entries.is_empty() || blind || hallucinating {
                assert!(draws[2..].iter().all(|(_, _, line)| line.trim().is_empty()));
            } else {
                assert_eq!(draws[2].2.trim_end(), "o Orc (1)");
            }
        }
    }

    #[test]
    fn monsters_panel_does_not_draw_in_classic_or_too_small_layouts() {
        for rows in [24, 25] {
            draw_monsters_panel(
                compute_layout(rows, CLASSIC_MIN_COLS),
                &[seen("Orc", 'o', 1)],
                false,
                false,
                |_, _, _| panic!("monsters must only draw in Split"),
            );
        }
    }

    #[export_name = "inventory_list"]
    static mut INVENTORY_LIST: *mut InventoryItem = ptr::null_mut();

    struct InventoryGuard(*mut InventoryItem);

    impl InventoryGuard {
        fn new(head: *mut InventoryItem) -> Self {
            let previous = unsafe { INVENTORY_LIST };
            unsafe { INVENTORY_LIST = head };
            Self(previous)
        }
    }

    impl Drop for InventoryGuard {
        fn drop(&mut self) {
            unsafe { INVENTORY_LIST = self.0 };
        }
    }

    fn inventory_item(subval: i64) -> InventoryItem {
        InventoryItem {
            data: Item {
                name: [0; 70],
                tval: ItemType::MiscObject.into(),
                flags2: 0,
                flags: 0,
                p1: 0,
                cost: 0,
                subval,
                weight: 5,
                number: 1,
                tohit: 0,
                todam: 0,
                ac: 0,
                toac: 0,
                damage: [0; 7],
                level: 0,
                identified: 0,
            },
            ok: 0,
            insides: 3,
            is_in: 1,
            next: ptr::null_mut(),
        }
    }

    fn identified_weapon() -> Item {
        let mut item = crate::generate_item::generate(
            Box::new(crate::generate_item::template::AxeTemplate::Balestarius),
            0,
            crate::generate_item::ItemQuality::Normal,
        );
        item.set_identified(true);
        item.tohit = 1;
        item.todam = 2;
        item
    }

    #[test]
    #[serial_test::serial]
    fn equipment_panel_rows_resolve_to_full_item_details() {
        let items = [identified_weapon(), inventory_item(4).data];

        assert_eq!(
            equipment_item_description_for_panel_row(&items, 14, 0).as_deref(),
            Some("balestarius (2d8) (+1,+2)")
        );
        assert_eq!(
            equipment_item_description_for_panel_row(&items, 14, 1).as_deref(),
            Some("empty bottle")
        );
        assert_eq!(
            equipment_item_description_for_panel_row(&items, 14, -1),
            None
        );
        assert_eq!(
            equipment_item_description_for_panel_row(&items, 14, 14),
            None
        );
    }

    #[test]
    #[serial_test::serial]
    fn inventory_panel_rows_resolve_items_but_not_the_overflow_line() {
        let items = [
            inventory_item(1),
            InventoryItem {
                data: identified_weapon(),
                ..inventory_item(4)
            },
            inventory_item(4),
            inventory_item(1),
        ];

        assert_eq!(inventory_item_description_for_panel_row(&items, 5, 0), None);
        assert_eq!(inventory_item_description_for_panel_row(&items, 5, 1), None);
        assert_eq!(
            inventory_item_description_for_panel_row(&items, 5, 3).as_deref(),
            Some("balestarius (2d8) (+1,+2)")
        );
        assert_eq!(inventory_item_description_for_panel_row(&items, 5, 4), None);
        assert_eq!(inventory_item_description_for_panel_row(&items, 5, 5), None);
    }

    #[test]
    fn inventory_panel_labels_items_in_list_order() {
        let names = vec!["Long Sword".to_string(), "5 rations of food".to_string()];
        let lines = render_inventory_panel(Rect::new(15, 81, 24, 5), &names);

        assert_eq!(
            lines,
            [
                "                        ",
                "Inventory               ",
                "a) Long Sword           ",
                "b) 5 rations of food    ",
                "                        ",
            ]
        );
    }

    #[test]
    fn inventory_panel_truncates_names_to_character_width() {
        let names = vec![
            "A very long item name".to_string(),
            "\u{e9}p\u{e9}e".to_string(),
        ];
        let lines = render_inventory_panel(Rect::new(15, 81, 6, 4), &names);

        assert_eq!(lines, ["      ", "Invent", "a) A v", "b) \u{e9}p\u{e9}"]);
    }

    #[test]
    fn inventory_panel_reserves_the_last_row_for_all_hidden_items() {
        let names = vec!["Sword".to_string(); 5];
        let lines = render_inventory_panel(Rect::new(15, 81, 12, 5), &names);

        assert_eq!(
            lines,
            [
                "            ",
                "Inventory   ",
                "a) Sword    ",
                "b) Sword    ",
                "...3 more   ",
            ]
        );
    }

    #[test]
    fn inventory_panel_uses_every_row_when_items_fit_exactly() {
        let names = vec!["Sword".to_string(); 3];
        let lines = render_inventory_panel(Rect::new(15, 81, 12, 5), &names);

        assert_eq!(
            lines,
            [
                "            ",
                "Inventory   ",
                "a) Sword    ",
                "b) Sword    ",
                "c) Sword    ",
            ]
        );
    }

    #[test]
    fn inventory_panel_with_one_item_row_reports_all_items_on_overflow() {
        let names = vec!["Sword".to_string(); 2];

        assert_eq!(
            render_inventory_panel(Rect::new(15, 81, 12, 3), &names),
            ["            ", "Inventory   ", "...2 more   "]
        );
    }

    #[test]
    fn inventory_panel_with_no_item_rows_fits_just_the_separator_and_heading() {
        let names = vec!["Sword".to_string(); 2];
        assert_eq!(
            render_inventory_panel(Rect::new(15, 81, 12, 1), &names),
            ["            "]
        );
        assert_eq!(
            render_inventory_panel(Rect::new(15, 81, 12, 2), &names),
            ["            ", "Inventory   "]
        );
    }

    #[test]
    fn inventory_panel_handles_empty_and_zero_sized_rectangles() {
        let rect = Rect::new(15, 81, 12, 2);
        assert_eq!(
            render_inventory_panel(rect, &[]),
            ["            ", "Inventory   "]
        );
        let names = vec!["Sword".to_string(); 2];
        assert!(render_inventory_panel(Rect { height: 0, ..rect }, &names).is_empty());
        assert_eq!(
            render_inventory_panel(Rect { width: 0, ..rect }, &names),
            ["", ""]
        );
    }

    #[test]
    #[serial_test::serial]
    fn inventory_snapshot_reads_the_c_list_without_mutating_nodes() {
        let mut tail = inventory_item(4);
        tail.ok = 1;
        tail.is_in = 0;
        let mut head = inventory_item(1);
        head.next = &mut tail;
        let before = serde_json::to_value([head, tail]).unwrap();
        let head_next = head.next;
        let tail_next = tail.next;
        let head_ptr = &mut head as *mut InventoryItem;
        let _guard = InventoryGuard::new(head_ptr);

        let names = inventory_snapshot();

        assert_eq!(names, ["rat skeleton", "empty bottle"]);
        assert_eq!(unsafe { INVENTORY_LIST }, head_ptr);
        assert_eq!(head.next, head_next);
        assert_eq!(tail.next, tail_next);
        assert_eq!(serde_json::to_value([head, tail]).unwrap(), before);
    }

    #[test]
    #[serial_test::serial]
    fn inventory_snapshot_of_an_empty_c_list_is_empty() {
        let _guard = InventoryGuard::new(ptr::null_mut());

        assert!(inventory_snapshot().is_empty());
    }

    #[test]
    #[serial_test::serial]
    fn inventory_panel_redraw_reads_current_items_and_clears_stale_rows() {
        let layout = compute_layout(SPLIT_MIN_ROWS, CLASSIC_MIN_COLS);
        let rect = layout.inventory.unwrap();
        let mut item = inventory_item(1);
        let _guard = InventoryGuard::new(&mut item);
        let mut draws = Vec::new();

        draw_inventory_panel(layout, |row, col, line| {
            draws.push((row, col, line.to_string()));
        });

        assert_eq!(draws.len(), rect.height as usize);
        assert_eq!((draws[0].0, draws[0].1), (rect.row, rect.col));
        assert!(draws[0].2.trim().is_empty());
        assert_eq!(draws[1].2.trim_end(), "Inventory");
        assert_eq!(draws[2].2.trim_end(), "a) rat skeleton");
        assert!(draws.iter().enumerate().all(|(index, (row, col, line))| {
            *row == rect.row + index as i32 && *col == rect.col && line.len() == rect.width as usize
        }));

        let _empty_guard = InventoryGuard::new(ptr::null_mut());
        draws.clear();
        draw_inventory_panel(layout, |row, col, line| {
            draws.push((row, col, line.to_string()));
        });
        assert_eq!(draws.len(), rect.height as usize);
        assert!(draws[0].2.trim().is_empty());
        assert_eq!(draws[1].2.trim_end(), "Inventory");
        assert!(draws[2..].iter().all(|(_, _, line)| line.trim().is_empty()));
    }

    #[test]
    #[serial_test::serial]
    fn inventory_panel_does_not_draw_in_classic_or_too_small_layouts() {
        let _guard = InventoryGuard::new(ptr::null_mut());
        for rows in [24, 25] {
            draw_inventory_panel(compute_layout(rows, CLASSIC_MIN_COLS), |_, _, _| {
                panic!("inventory must only draw in Split");
            });
        }
    }

    #[test]
    fn split_equipment_heading_preserves_all_slots_and_stays_beside_the_prompt() {
        let layout = compute_layout(SPLIT_MIN_ROWS, CLASSIC_MIN_COLS);
        let rect = layout.equipment.unwrap();
        let prompt = layout.prompt.unwrap();
        let slots: Vec<_> = equipment::slots_iter()
            .map(|index| EquipmentSlot {
                name: equipment::Slot::from(index).name(),
                item_name: String::new(),
            })
            .collect();
        let mut draws = Vec::new();

        draw_equipment_panel(layout, &slots, |row, col, line| {
            draws.push((row, col, line.to_string()));
        });

        assert_eq!(draws.len(), slots.len() + 1);
        assert_eq!((draws[0].0, draws[0].1), (0, rect.col));
        assert_eq!(draws[0].2.trim_end(), "Equipment");
        assert!(draws[0].1 >= prompt.col + prompt.width);
        assert_eq!(draws[0].2.len(), rect.width as usize);
        for (index, line) in render_equipment_panel(rect, &slots).iter().enumerate() {
            assert_eq!(
                draws[index + 1],
                (rect.row + index as i32, rect.col, line.clone())
            );
        }
        assert!(draws.last().unwrap().2.starts_with("n) Backup weapon: "));
    }

    #[test]
    fn classic_equipment_drawing_has_no_heading_and_matches_existing_lines() {
        let layout = compute_layout(25, CLASSIC_MIN_COLS);
        let rect = layout.equipment.unwrap();
        let slots = [EquipmentSlot {
            name: "Main hand",
            item_name: "Long Sword".to_string(),
        }];
        let mut draws = Vec::new();

        draw_equipment_panel(layout, &slots, |row, col, line| {
            draws.push((row, col, line.to_string()));
        });

        let expected: Vec<_> = render_equipment_panel(rect, &slots)
            .into_iter()
            .enumerate()
            .map(|(index, line)| (rect.row + index as i32, rect.col, line))
            .collect();
        assert_eq!(draws, expected);
    }

    #[test]
    fn split_equipment_heading_is_fitted_to_sidebar_width() {
        let mut layout = compute_layout(SPLIT_MIN_ROWS, CLASSIC_MIN_COLS);
        layout.equipment.as_mut().unwrap().width = 5;
        let mut draws = Vec::new();

        draw_equipment_panel(layout, &[], |_, _, line| {
            draws.push(line.to_string());
        });

        assert_eq!(draws.first().map(String::as_str), Some("Equip"));
        assert!(draws.iter().all(|line| line.len() == 5));
    }

    #[test]
    fn equipment_panel_does_not_draw_without_a_drawable_layout() {
        draw_equipment_panel(compute_layout(24, CLASSIC_MIN_COLS), &[], |_, _, _| {
            panic!("equipment must not draw without a panel");
        });
    }

    #[test]
    fn equipment_panel_renders_classic_lines_and_keeps_empty_slots() {
        let rect = Rect::new(1, 81, CLASSIC_MIN_COLS - 81, EQUIPMENT_HEIGHT);
        let slot_names = [
            "Main hand",
            "Helm",
            "Amulet",
            "Armor",
            "Belt",
            "Shield",
            "Gloves",
            "Bracers",
            "Right ring",
            "Left ring",
            "Boots",
            "Cloak",
            "Light source",
            "Backup weapon",
        ];
        let slots: Vec<_> = slot_names
            .iter()
            .enumerate()
            .map(|(index, name)| EquipmentSlot {
                name,
                item_name: if index == 0 {
                    "Long Sword".to_string()
                } else {
                    String::new()
                },
            })
            .collect();

        let lines = render_equipment_panel(rect, &slots);

        assert_eq!(lines.len(), EQUIPMENT_HEIGHT as usize);
        assert_eq!(lines[0].trim_end(), "a) Main hand    : Long Sword");
        assert_eq!(lines[1].trim_end(), "b) Helm         :");
        assert!(lines[13].starts_with("n) Backup weapon: "));
        assert!(lines
            .iter()
            .all(|line| line.chars().count() == rect.width as usize));
    }

    #[test]
    fn equipment_panel_truncates_names_to_the_sidebar_width() {
        let rect = Rect::new(1, 81, 23, 1);
        let slots = [EquipmentSlot {
            name: "Main hand",
            item_name: "A very long item name".to_string(),
        }];

        let lines = render_equipment_panel(rect, &slots);

        assert_eq!(lines, ["a) Main hand    : A ver"]);
    }
}
