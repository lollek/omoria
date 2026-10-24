use std::collections::VecDeque;
use std::sync::Mutex;

use crate::layout::{Layout, Rect};
use crate::user_interface::viewport::Viewport;

mod hover;
mod item_click;
mod monster_list;
mod travel;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Fourth,
    Fifth,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseEventKind {
    Click(MouseButton),
    Motion,
}

pub const MOUSE_INPUT: libc::c_int = -2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TerminalInput {
    Key(u8),
    Mouse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MouseEvent {
    pub row: i32,
    pub col: i32,
    pub kind: MouseEventKind,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CMouseEvent {
    pub row: libc::c_int,
    pub col: libc::c_int,
    pub kind: libc::c_int,
    pub button: libc::c_int,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SidebarPanel {
    Equipment,
    Inventory,
    Monsters,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseTarget {
    Map { y: i32, x: i32 },
    Sidebar { panel: SidebarPanel, index: i32 },
}

pub type MouseHandler = fn(MouseEvent, Option<MouseTarget>);

const MAX_QUEUED_MOUSE_EVENTS: usize = 64;

lazy_static! {
    static ref MOUSE_EVENTS: Mutex<VecDeque<MouseEvent>> = Mutex::new(VecDeque::new());
    static ref MOUSE_HANDLERS: Mutex<Vec<MouseHandler>> = Mutex::new(Vec::new());
}

pub fn hit_test(layout: Layout, viewport: Viewport, row: i32, col: i32) -> Option<MouseTarget> {
    if let Some(map) = layout.map {
        if contains(map, row, col) {
            let y = row + viewport.panel_row_prt + 1;
            let x = col + viewport.panel_col_prt + 1;
            if (viewport.world_top..=viewport.world_bottom).contains(&y)
                && (viewport.world_left..=viewport.world_right).contains(&x)
            {
                return Some(MouseTarget::Map { y, x });
            }
        }
    }

    [
        (layout.equipment, SidebarPanel::Equipment),
        (layout.inventory, SidebarPanel::Inventory),
        (layout.monsters, SidebarPanel::Monsters),
    ]
    .iter()
    .find_map(|(rect, panel)| {
        rect.filter(|rect| contains(*rect, row, col))
            .map(|rect| MouseTarget::Sidebar {
                panel: *panel,
                index: row - rect.row,
            })
    })
}

pub fn translate_terminal_input(code: libc::c_int) -> Option<TerminalInput> {
    if code == ncurses::KEY_MOUSE {
        Some(TerminalInput::Mouse)
    } else {
        crate::key_translation::translate_key(code).map(TerminalInput::Key)
    }
}

fn contains(rect: Rect, row: i32, col: i32) -> bool {
    row >= rect.row
        && row < rect.row + rect.height
        && col >= rect.col
        && col < rect.col + rect.width
}

fn decode_mouse_event(row: i32, col: i32, state: libc::c_ulong) -> Option<MouseEvent> {
    let buttons = [
        (ncurses::BUTTON1_CLICKED as libc::c_ulong, MouseButton::Left),
        (
            ncurses::BUTTON1_DOUBLE_CLICKED as libc::c_ulong,
            MouseButton::Left,
        ),
        (
            ncurses::BUTTON1_TRIPLE_CLICKED as libc::c_ulong,
            MouseButton::Left,
        ),
        (
            ncurses::BUTTON2_CLICKED as libc::c_ulong,
            MouseButton::Middle,
        ),
        (
            ncurses::BUTTON2_DOUBLE_CLICKED as libc::c_ulong,
            MouseButton::Middle,
        ),
        (
            ncurses::BUTTON2_TRIPLE_CLICKED as libc::c_ulong,
            MouseButton::Middle,
        ),
        (
            ncurses::BUTTON3_CLICKED as libc::c_ulong,
            MouseButton::Right,
        ),
        (
            ncurses::BUTTON3_DOUBLE_CLICKED as libc::c_ulong,
            MouseButton::Right,
        ),
        (
            ncurses::BUTTON3_TRIPLE_CLICKED as libc::c_ulong,
            MouseButton::Right,
        ),
        (
            ncurses::BUTTON4_CLICKED as libc::c_ulong,
            MouseButton::Fourth,
        ),
        (
            ncurses::BUTTON5_CLICKED as libc::c_ulong,
            MouseButton::Fifth,
        ),
    ];
    let kind = buttons
        .iter()
        .find(|(mask, _)| state & mask != 0)
        .map(|(_, button)| MouseEventKind::Click(*button))
        .or_else(|| {
            (state & ncurses::REPORT_MOUSE_POSITION as libc::c_ulong != 0)
                .then_some(MouseEventKind::Motion)
        })?;

    Some(MouseEvent { row, col, kind })
}

pub fn pop_mouse_event() -> Option<MouseEvent> {
    MOUSE_EVENTS
        .lock()
        .expect("mouse event queue lock poisoned")
        .pop_front()
}

fn enqueue_mouse_event(event: MouseEvent) {
    let mut events = MOUSE_EVENTS
        .lock()
        .expect("mouse event queue lock poisoned");
    if event.kind == MouseEventKind::Motion
        && events
            .back()
            .is_some_and(|previous| previous.kind == MouseEventKind::Motion)
    {
        *events.back_mut().expect("last mouse event disappeared") = event;
        return;
    }
    if events.len() == MAX_QUEUED_MOUSE_EVENTS {
        events.pop_front();
    }
    events.push_back(event);
}

pub fn register_handler(handler: MouseHandler) {
    MOUSE_HANDLERS
        .lock()
        .expect("mouse handler lock poisoned")
        .push(handler);
}

pub fn register_hover_handler() {
    hover::register();
}

pub fn register_monster_list_handler() {
    monster_list::register();
}

pub fn register_item_click_handler() {
    item_click::register();
}

pub fn register_travel_handler() {
    travel::register();
}

pub fn clear_hover_prompt() {
    hover::clear_prompt_overlay();
}

pub fn dispatch_pending_events(layout: Layout, viewport: Viewport) -> usize {
    let handlers = MOUSE_HANDLERS
        .lock()
        .expect("mouse handler lock poisoned")
        .clone();
    if handlers.is_empty() {
        return 0;
    }
    let events: Vec<_> = MOUSE_EVENTS
        .lock()
        .expect("mouse event queue lock poisoned")
        .drain(..)
        .collect();

    for event in &events {
        let target = hit_test(layout, viewport, event.row, event.col);
        for handler in &handlers {
            handler(*event, target);
        }
    }
    events.len()
}

#[no_mangle]
pub extern "C" fn C_mouse_event_push(row: libc::c_int, col: libc::c_int, state: libc::c_ulong) {
    if let Some(event) = decode_mouse_event(row, col, state) {
        enqueue_mouse_event(event);
    }
}

#[no_mangle]
pub extern "C" fn C_mouse_event_pop() -> CMouseEvent {
    match pop_mouse_event() {
        Some(MouseEvent { row, col, kind }) => {
            let (kind, button) = match kind {
                MouseEventKind::Click(button) => (1, button as libc::c_int + 1),
                MouseEventKind::Motion => (2, 0),
            };
            CMouseEvent {
                row,
                col,
                kind,
                button,
            }
        }
        None => CMouseEvent {
            row: -1,
            col: -1,
            kind: 0,
            button: 0,
        },
    }
}

#[no_mangle]
pub extern "C" fn C_translate_terminal_input_code(code: libc::c_int) -> libc::c_int {
    let input = translate_terminal_input(code);
    if matches!(input, Some(TerminalInput::Key(_))) {
        clear_hover_prompt();
    }
    match input {
        Some(TerminalInput::Key(byte)) => byte as libc::c_int,
        Some(TerminalInput::Mouse) => MOUSE_INPUT,
        None => -1,
    }
}

#[no_mangle]
pub extern "C" fn C_mouse_dispatch_pending_events(
    panel_row: libc::c_long,
    panel_col: libc::c_long,
    world_height: libc::c_long,
    world_width: libc::c_long,
) {
    let layout = crate::layout::current_layout();
    let map = layout.map.unwrap_or(Rect::new(0, 0, 0, 0));
    let viewport = crate::user_interface::viewport::calculate_viewport(
        map,
        panel_row as i32,
        panel_col as i32,
        world_height as i32,
        world_width as i32,
    );
    dispatch_pending_events(layout, viewport);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::compute_layout;
    use crate::user_interface::viewport::calculate_viewport;

    struct ResetMouseState;

    impl Drop for ResetMouseState {
        fn drop(&mut self) {
            MOUSE_EVENTS.lock().unwrap().clear();
            MOUSE_HANDLERS.lock().unwrap().clear();
        }
    }

    fn reset_mouse_state() -> ResetMouseState {
        let reset = ResetMouseState;
        MOUSE_EVENTS.lock().unwrap().clear();
        MOUSE_HANDLERS.lock().unwrap().clear();
        reset
    }

    #[test]
    fn classic_map_cells_map_to_world_coordinates() {
        let layout = compute_layout(25, 150);
        let viewport = calculate_viewport(layout.map.unwrap(), 0, 0, 66, 198);

        assert_eq!(
            hit_test(layout, viewport, 1, 14),
            Some(MouseTarget::Map { y: 1, x: 1 })
        );
        assert_eq!(
            hit_test(layout, viewport, 22, 79),
            Some(MouseTarget::Map { y: 22, x: 66 })
        );
    }

    #[test]
    fn split_sidebar_cells_report_panel_and_row_index() {
        let layout = compute_layout(35, 150);
        let viewport = calculate_viewport(layout.map.unwrap(), 0, 0, 66, 198);

        assert_eq!(
            hit_test(layout, viewport, 14, 81),
            Some(MouseTarget::Sidebar {
                panel: SidebarPanel::Equipment,
                index: 13,
            })
        );
        assert_eq!(
            hit_test(layout, viewport, 15, 81),
            Some(MouseTarget::Sidebar {
                panel: SidebarPanel::Inventory,
                index: 0,
            })
        );
        assert_eq!(
            hit_test(layout, viewport, 25, 81),
            Some(MouseTarget::Sidebar {
                panel: SidebarPanel::Monsters,
                index: 0,
            })
        );
    }

    #[test]
    fn noninteractive_cells_and_panel_edges_are_not_misidentified() {
        let layout = compute_layout(35, 150);
        let viewport = calculate_viewport(layout.map.unwrap(), 0, 0, 66, 198);

        assert_eq!(hit_test(layout, viewport, 0, 0), None);
        assert_eq!(
            hit_test(layout, viewport, 1, 81),
            Some(MouseTarget::Sidebar {
                panel: SidebarPanel::Equipment,
                index: 0,
            })
        );
        assert_eq!(
            hit_test(layout, viewport, 1, 80),
            Some(MouseTarget::Map { y: 1, x: 81 })
        );
        assert_eq!(
            hit_test(layout, viewport, 34, 81),
            Some(MouseTarget::Sidebar {
                panel: SidebarPanel::Monsters,
                index: 9,
            })
        );
        assert_eq!(hit_test(layout, viewport, 35, 81), None);
    }

    #[test]
    fn scrolled_and_centered_viewports_map_only_visible_world_cells() {
        let classic = compute_layout(25, 150);
        let classic_viewport = calculate_viewport(classic.map.unwrap(), 2, 3, 66, 198);
        assert_eq!(
            hit_test(classic, classic_viewport, 1, 14),
            Some(MouseTarget::Map { y: 23, x: 100 })
        );

        let split = compute_layout(50, 150);
        let split_viewport = calculate_viewport(split.map.unwrap(), 0, 0, 20, 40);
        assert_eq!(hit_test(split, split_viewport, 1, 0), None);
        assert_eq!(
            hit_test(split, split_viewport, 9, 20),
            Some(MouseTarget::Map { y: 1, x: 1 })
        );
        assert_eq!(
            hit_test(split, split_viewport, 28, 59),
            Some(MouseTarget::Map { y: 20, x: 40 })
        );
    }

    #[test]
    #[serial_test::serial]
    fn decoded_mouse_clicks_are_available_from_the_event_queue() {
        let _reset = reset_mouse_state();
        C_mouse_event_push(4, 9, ncurses::BUTTON1_CLICKED as libc::c_ulong);

        assert_eq!(
            pop_mouse_event(),
            Some(MouseEvent {
                row: 4,
                col: 9,
                kind: MouseEventKind::Click(MouseButton::Left),
            })
        );
        assert_eq!(pop_mouse_event(), None);
    }

    #[test]
    #[serial_test::serial]
    fn motion_events_are_queued_and_unrecognized_states_are_ignored() {
        let _reset = reset_mouse_state();
        C_mouse_event_push(7, 12, ncurses::REPORT_MOUSE_POSITION as libc::c_ulong);
        C_mouse_event_push(8, 13, ncurses::REPORT_MOUSE_POSITION as libc::c_ulong);
        C_mouse_event_push(8, 13, 0);

        assert_eq!(
            pop_mouse_event(),
            Some(MouseEvent {
                row: 8,
                col: 13,
                kind: MouseEventKind::Motion,
            })
        );
        assert_eq!(pop_mouse_event(), None);
    }

    #[test]
    #[serial_test::serial]
    fn dispatch_without_handlers_preserves_events_for_later_consumers() {
        let _reset = reset_mouse_state();
        C_mouse_event_push(4, 9, ncurses::BUTTON1_CLICKED as libc::c_ulong);

        let layout = compute_layout(25, 150);
        let viewport = calculate_viewport(layout.map.unwrap(), 0, 0, 66, 198);
        assert_eq!(dispatch_pending_events(layout, viewport), 0);
        assert_eq!(
            pop_mouse_event(),
            Some(MouseEvent {
                row: 4,
                col: 9,
                kind: MouseEventKind::Click(MouseButton::Left),
            })
        );
    }

    #[test]
    #[serial_test::serial]
    fn unconsumed_event_queue_keeps_only_the_newest_bounded_events() {
        let _reset = reset_mouse_state();
        for row in 0..=64 {
            C_mouse_event_push(row, 0, ncurses::BUTTON1_CLICKED as libc::c_ulong);
        }

        assert_eq!(
            pop_mouse_event().map(|event| event.row),
            Some(1),
            "the oldest click should be dropped once the queue reaches capacity"
        );
    }

    #[test]
    fn scripted_mouse_input_does_not_change_the_command_key_stream() {
        let input = [
            27,
            b'3' as libc::c_int,
            ncurses::KEY_MOUSE,
            b'a' as libc::c_int,
            ncurses::KEY_RESIZE,
            b'Z' as libc::c_int,
        ];
        let output: Vec<_> = input
            .iter()
            .filter_map(|code| match translate_terminal_input(*code) {
                Some(TerminalInput::Key(byte)) => Some(byte),
                Some(TerminalInput::Mouse) | None => None,
            })
            .collect();

        assert_eq!(output, [27, b'3', b'a', b'Z']);
    }

    #[test]
    #[serial_test::serial]
    fn registered_handlers_receive_queued_events_with_hit_targets() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        let _reset = reset_mouse_state();
        static HANDLED: AtomicUsize = AtomicUsize::new(0);
        fn handler(event: MouseEvent, target: Option<MouseTarget>) {
            assert_eq!(event.kind, MouseEventKind::Click(MouseButton::Left));
            assert_eq!(target, Some(MouseTarget::Map { y: 1, x: 1 }));
            HANDLED.fetch_add(1, Ordering::SeqCst);
        }

        HANDLED.store(0, Ordering::SeqCst);
        register_handler(handler);
        C_mouse_event_push(1, 14, ncurses::BUTTON1_CLICKED as libc::c_ulong);

        let layout = compute_layout(25, 150);
        let viewport = calculate_viewport(layout.map.unwrap(), 0, 0, 66, 198);
        assert_eq!(dispatch_pending_events(layout, viewport), 1);
        assert_eq!(HANDLED.load(Ordering::SeqCst), 1);
        assert_eq!(dispatch_pending_events(layout, viewport), 0);
    }
}
