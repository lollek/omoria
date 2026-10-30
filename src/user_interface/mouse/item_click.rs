use crate::user_interface::layout::LayoutKind;
use crate::user_interface::mouse::SidebarPanel;
use crate::user_interface::mouse::{MouseButton, MouseEvent, MouseEventKind, MouseTarget};

fn click_description(
    layout_kind: LayoutKind,
    event: MouseEvent,
    target: Option<MouseTarget>,
    resolve: impl FnOnce(SidebarPanel, i32) -> Option<String>,
) -> Option<String> {
    if layout_kind != LayoutKind::Split || event.kind != MouseEventKind::Click(MouseButton::Left) {
        return None;
    }
    let Some(MouseTarget::Sidebar { panel, index }) = target else {
        return None;
    };
    if !matches!(panel, SidebarPanel::Equipment | SidebarPanel::Inventory) {
        return None;
    }
    resolve(panel, index)
}

#[cfg(not(test))]
fn handle_click(event: MouseEvent, target: Option<MouseTarget>) {
    let description = click_description(
        crate::layout::current_layout().kind,
        event,
        target,
        |panel, index| match panel {
            SidebarPanel::Equipment => {
                crate::user_interface::sidebar::current_equipment_item_description_for_panel_row(
                    index,
                )
            }
            SidebarPanel::Inventory => {
                crate::user_interface::sidebar::current_inventory_item_description_for_panel_row(
                    index,
                )
            }
            SidebarPanel::Monsters | SidebarPanel::FloorItems => None,
        },
    );
    if let Some(description) = description {
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
    fn left_clicks_resolve_equipment_and_inventory_rows() {
        let event = MouseEvent {
            row: 27,
            col: 90,
            kind: MouseEventKind::Click(MouseButton::Left),
        };

        for (panel, index) in [(SidebarPanel::Equipment, 2), (SidebarPanel::Inventory, 3)] {
            let description = click_description(
                LayoutKind::Split,
                event,
                Some(MouseTarget::Sidebar { panel, index }),
                |resolved_panel, resolved_index| {
                    assert_eq!(resolved_panel, panel);
                    assert_eq!(resolved_index, index);
                    Some("balestarius (2d8) (+1,+2)".to_owned())
                },
            );
            assert_eq!(description.as_deref(), Some("balestarius (2d8) (+1,+2)"));
        }
    }

    #[test]
    fn non_left_clicks_and_non_item_targets_are_ignored() {
        let resolve = |_, _| panic!("non-item clicks must not resolve a sidebar row");
        let inventory = Some(MouseTarget::Sidebar {
            panel: SidebarPanel::Inventory,
            index: 3,
        });

        assert_eq!(
            click_description(
                LayoutKind::Split,
                MouseEvent {
                    row: 27,
                    col: 90,
                    kind: MouseEventKind::Click(MouseButton::Right),
                },
                inventory,
                resolve,
            ),
            None
        );
        assert_eq!(
            click_description(
                LayoutKind::Split,
                MouseEvent {
                    row: 8,
                    col: 14,
                    kind: MouseEventKind::Click(MouseButton::Left),
                },
                Some(MouseTarget::Map { y: 3, x: 4 }),
                resolve,
            ),
            None
        );
        assert_eq!(
            click_description(
                LayoutKind::Split,
                MouseEvent {
                    row: 27,
                    col: 90,
                    kind: MouseEventKind::Click(MouseButton::Left),
                },
                Some(MouseTarget::Sidebar {
                    panel: SidebarPanel::Monsters,
                    index: 3,
                }),
                resolve,
            ),
            None
        );
    }

    #[test]
    fn classic_equipment_clicks_do_not_show_split_item_descriptions() {
        let description = click_description(
            LayoutKind::Classic,
            MouseEvent {
                row: 3,
                col: 90,
                kind: MouseEventKind::Click(MouseButton::Left),
            },
            Some(MouseTarget::Sidebar {
                panel: SidebarPanel::Equipment,
                index: 2,
            }),
            |_, _| panic!("Classic clicks must not resolve Split item rows"),
        );

        assert_eq!(description, None);
    }
}
