use super::layout::{current_layout, draw_lines, render_lines, Rect};
use crate::{equipment, ncurses};

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

#[no_mangle]
pub extern "C" fn C_print_equipment_block() {
    let rect = current_layout()
        .equipment
        .expect("equipment panel unavailable");
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user_interface::layout::{CLASSIC_MIN_COLS, EQUIPMENT_HEIGHT};

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
