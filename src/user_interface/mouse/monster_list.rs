use crate::user_interface::mouse::{
    MouseButton, MouseEvent, MouseEventKind, MouseTarget, SidebarPanel,
};

fn click_description(
    event: MouseEvent,
    target: Option<MouseTarget>,
    monster_position: impl FnOnce(i32) -> Option<(i64, i64)>,
    describe_at: impl FnOnce(i64, i64) -> Option<String>,
) -> Option<String> {
    if event.kind != MouseEventKind::Click(MouseButton::Left) {
        return None;
    }
    let Some(MouseTarget::Sidebar {
        panel: SidebarPanel::Monsters,
        index,
    }) = target
    else {
        return None;
    };
    let (y, x) = monster_position(index)?;
    describe_at(y, x)
}

#[cfg(not(test))]
fn handle_click(event: MouseEvent, target: Option<MouseTarget>) {
    if let Some(description) = click_description(
        event,
        target,
        crate::user_interface::sidebar::current_monster_position_for_panel_row,
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
    fn clicking_a_monster_entry_shows_its_look_description() {
        let event = MouseEvent {
            row: 27,
            col: 90,
            kind: MouseEventKind::Click(MouseButton::Left),
        };

        let description = click_description(
            event,
            Some(MouseTarget::Sidebar {
                panel: SidebarPanel::Monsters,
                index: 2,
            }),
            |row| {
                assert_eq!(row, 2);
                Some((14, 23))
            },
            |y, x| {
                assert_eq!((y, x), (14, 23));
                Some("You see an orc.".to_owned())
            },
        );

        assert_eq!(description.as_deref(), Some("You see an orc."));
    }

    #[test]
    fn other_clicks_and_targets_do_not_describe_monsters() {
        let describe = |_, _| panic!("non-monster clicks must not describe a monster");
        let position = |_| panic!("non-monster clicks must not resolve a row");
        let target = Some(MouseTarget::Sidebar {
            panel: SidebarPanel::Monsters,
            index: 2,
        });

        assert_eq!(
            click_description(
                MouseEvent {
                    row: 27,
                    col: 90,
                    kind: MouseEventKind::Click(MouseButton::Right),
                },
                target,
                position,
                describe,
            ),
            None
        );
        assert_eq!(
            click_description(
                MouseEvent {
                    row: 27,
                    col: 90,
                    kind: MouseEventKind::Click(MouseButton::Left),
                },
                Some(MouseTarget::Sidebar {
                    panel: SidebarPanel::Inventory,
                    index: 2,
                }),
                position,
                describe,
            ),
            None
        );
    }
}
