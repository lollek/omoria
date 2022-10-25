use crate::{data::item_name, io, logic::menu};

use super::not_safe::inventory_clone;

const MAX_DISPLAYED_ITEMS: usize = 20;

pub fn display_inventory() {
    let inventory = inventory_clone();
    let mut starting_index = 0;

    'outer: loop {
        let items_to_display = inventory
            .iter()
            .skip(starting_index)
            .take(MAX_DISPLAYED_ITEMS)
            .map(|item| item_name::generate(&item.data))
            .collect::<Vec<String>>();

        menu::draw_menu(
            "Inventory",
            &items_to_display
                .iter()
                .map(|item_name| item_name.as_ref())
                .collect::<Vec<&str>>(),
            "ESC / SPACE / e / t / w / M / c / p / r / I",
            255,
        );
        loop {
            match io::inkey_flush() {
                io::ESCAPE => return,
                b' ' => {
                    starting_index += MAX_DISPLAYED_ITEMS;
                    if starting_index >= inventory.len() {
                        starting_index = 0;
                    }
                    continue 'outer;
                }
                _ => {}
            }
        }
    }
}
