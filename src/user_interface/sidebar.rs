use super::layout::{current_layout, draw_lines, render_lines, Layout, Rect};
use crate::{data::item_name, equipment, inventory::not_safe::inventory_clone, ncurses};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EquipmentSlot {
    pub name: &'static str,
    pub item_name: String,
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

fn draw_inventory_panel(layout: Layout, draw: impl FnMut(i32, i32, &str)) {
    if let Some(rect) = layout.inventory {
        let lines = render_inventory_panel(rect, &inventory_snapshot());
        draw_lines(rect, &lines, draw);
    }
}

#[no_mangle]
pub extern "C" fn C_print_equipment_block() {
    let layout = current_layout();
    let rect = layout.equipment.expect("equipment panel unavailable");
    let slots: Vec<_> = equipment::slots_iter()
        .map(|slot_index| {
            let slot = equipment::Slot::from(slot_index);
            EquipmentSlot {
                name: slot.name(),
                item_name: equipment::get_name(slot),
            }
        })
        .collect();
    let lines = render_equipment_panel(rect, &slots);
    draw_lines(rect, &lines, |row, col, line| {
        ncurses::mvaddstr(row, col, line);
    });
    draw_inventory_panel(layout, |row, col, line| {
        ncurses::mvaddstr(row, col, line);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{InventoryItem, Item, ItemType};
    use crate::user_interface::layout::{
        compute_layout, CLASSIC_MIN_COLS, EQUIPMENT_HEIGHT, SPLIT_MIN_ROWS,
    };
    use std::ptr;

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
