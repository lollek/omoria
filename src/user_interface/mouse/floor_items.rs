use crate::user_interface::mouse::{
    MouseButton, MouseEvent, MouseEventKind, MouseTarget, SidebarPanel,
};

fn click_description(
    event: MouseEvent,
    target: Option<MouseTarget>,
    floor_item_position: impl FnOnce(i32) -> Option<(i64, i64)>,
    describe_at: impl FnOnce(i64, i64) -> Option<String>,
) -> Option<String> {
    if event.kind != MouseEventKind::Click(MouseButton::Left) {
        return None;
    }
    let Some(MouseTarget::Sidebar {
        panel: SidebarPanel::FloorItems,
        index,
    }) = target
    else {
        return None;
    };
    let (y, x) = floor_item_position(index)?;
    describe_at(y, x)
}

#[cfg(not(test))]
fn handle_click(event: MouseEvent, target: Option<MouseTarget>) {
    if let Some(description) = click_description(
        event,
        target,
        crate::user_interface::sidebar::current_floor_item_position_for_panel_row,
        crate::player_action::describe_at_global,
    ) {
        super::hover::show_prompt_overlay(&description);
        crate::ncurses::refresh();
    }
}

#[cfg(not(test))]
pub(super) fn register() {
    super::register_handler(handle_click);
}

#[cfg(test)]
pub(super) fn register() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clicking_a_floor_item_entry_shows_its_full_look_description() {
        let event = MouseEvent {
            row: 32,
            col: 90,
            kind: MouseEventKind::Click(MouseButton::Left),
        };
        let description = click_description(
            event,
            Some(MouseTarget::Sidebar {
                panel: SidebarPanel::FloorItems,
                index: 2,
            }),
            |row| {
                assert_eq!(row, 2);
                Some((14, 23))
            },
            |y, x| {
                assert_eq!((y, x), (14, 23));
                Some("You see a long-sword (2d5) (+3,+4).".to_owned())
            },
        );

        assert_eq!(
            description.as_deref(),
            Some("You see a long-sword (2d5) (+3,+4).")
        );
    }

    #[test]
    fn separators_overflow_non_left_clicks_and_other_targets_are_ignored() {
        let event = MouseEvent {
            row: 30,
            col: 90,
            kind: MouseEventKind::Click(MouseButton::Left),
        };
        let target = |index| {
            Some(MouseTarget::Sidebar {
                panel: SidebarPanel::FloorItems,
                index,
            })
        };
        for index in [0, 1, 4] {
            assert_eq!(
                click_description(
                    event,
                    target(index),
                    |row| match row {
                        2 => Some((14, 23)),
                        _ => None,
                    },
                    |_, _| panic!("non-entry rows must not show item descriptions"),
                ),
                None
            );
        }
        assert_eq!(
            click_description(
                MouseEvent {
                    kind: MouseEventKind::Click(MouseButton::Right),
                    ..event
                },
                target(2),
                |_| panic!("non-left clicks must not resolve a row"),
                |_, _| panic!("non-left clicks must not describe an item"),
            ),
            None
        );
        assert_eq!(
            click_description(
                event,
                Some(MouseTarget::Sidebar {
                    panel: SidebarPanel::Inventory,
                    index: 2,
                }),
                |_| panic!("other panel clicks must not resolve a floor item"),
                |_, _| panic!("other panel clicks must not describe a floor item"),
            ),
            None
        );
    }
}
