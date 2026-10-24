use crate::user_interface::mouse::{MouseEvent, MouseEventKind, MouseTarget};

#[derive(Default)]
struct PromptOverlay<T> {
    original: Option<Vec<T>>,
}

impl<T> PromptOverlay<T> {
    fn capture_if_missing(&mut self, current_row: Vec<T>) {
        if self.original.is_none() {
            self.original = Some(current_row);
        }
    }

    fn clear(&mut self) -> Option<Vec<T>> {
        self.original.take()
    }
}

fn hover_prompt(
    event: MouseEvent,
    target: Option<MouseTarget>,
    describe: impl FnOnce(i32, i32) -> Option<String>,
) -> Option<String> {
    if event.kind != MouseEventKind::Motion {
        return None;
    }

    Some(match target {
        Some(MouseTarget::Map { y, x }) => describe(y, x).unwrap_or_default(),
        _ => String::new(),
    })
}

fn apply_hover(
    event: MouseEvent,
    target: Option<MouseTarget>,
    describe: impl FnOnce(i32, i32) -> Option<String>,
    mut draw: impl FnMut(&str),
    mut clear: impl FnMut(),
    mut refresh: impl FnMut(),
) {
    if let Some(description) = hover_prompt(event, target, describe) {
        if description.is_empty() {
            clear();
        } else {
            draw(&description);
        }
        refresh();
    }
}

#[cfg(not(test))]
lazy_static! {
    static ref PROMPT_OVERLAY: std::sync::Mutex<PromptOverlay<pancurses::chtype>> =
        std::sync::Mutex::new(PromptOverlay::default());
}

#[cfg(not(test))]
pub(super) fn show_prompt_overlay(description: &str) {
    let mut overlay = PROMPT_OVERLAY.lock().expect("hover prompt lock poisoned");
    overlay.capture_if_missing(crate::ncurses::read_row(0));
    crate::ncurses::mov(0, 0);
    crate::ncurses::clrtoeol();
    crate::ncurses::mvaddstr(0, 0, description);
}

#[cfg(not(test))]
pub(super) fn clear_prompt_overlay() {
    let mut overlay = PROMPT_OVERLAY.lock().expect("hover prompt lock poisoned");
    if let Some(original) = overlay.clear() {
        crate::ncurses::restore_row(0, &original);
    }
}

#[cfg(test)]
pub(super) fn clear_prompt_overlay() {}

#[cfg(not(test))]
fn handle_hover(event: MouseEvent, target: Option<MouseTarget>) {
    apply_hover(
        event,
        target,
        |y, x| crate::player_action::describe_at_global(y.into(), x.into()),
        show_prompt_overlay,
        clear_prompt_overlay,
        crate::ncurses::refresh,
    );
}

#[cfg(not(test))]
pub(super) fn register() {
    super::register_handler(handle_hover);
}

#[cfg(test)]
pub(super) fn register() {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user_interface::mouse::{MouseEvent, MouseEventKind, MouseTarget};

    #[test]
    fn prompt_overlay_restores_the_original_row_after_repeated_hovers() {
        let original = vec![1_u8, 2, 3];
        let mut overlay = PromptOverlay::default();

        overlay.capture_if_missing(original.clone());
        overlay.capture_if_missing(vec![4, 5, 6]);

        assert_eq!(overlay.clear(), Some(original));
    }

    #[test]
    fn map_motion_produces_prompt_description() {
        let event = MouseEvent {
            row: 8,
            col: 14,
            kind: MouseEventKind::Motion,
        };

        assert_eq!(
            hover_prompt(event, Some(MouseTarget::Map { y: 3, x: 4 }), |y, x| {
                assert_eq!((y, x), (3, 4));
                Some("You see an orc.".to_owned())
            }),
            Some("You see an orc.".to_owned())
        );
    }

    #[test]
    fn map_motion_draws_the_description_and_refreshes() {
        let event = MouseEvent {
            row: 8,
            col: 14,
            kind: MouseEventKind::Motion,
        };
        let mut displayed = None;
        let mut refreshes = 0;

        apply_hover(
            event,
            Some(MouseTarget::Map { y: 3, x: 4 }),
            |_, _| Some("You see an orc.".to_owned()),
            |description| displayed = Some(description.to_owned()),
            || {},
            || refreshes += 1,
        );

        assert_eq!(displayed.as_deref(), Some("You see an orc."));
        assert_eq!(refreshes, 1);
    }

    #[test]
    fn empty_map_motion_clears_the_previous_description() {
        let event = MouseEvent {
            row: 8,
            col: 14,
            kind: MouseEventKind::Motion,
        };

        assert_eq!(
            hover_prompt(event, Some(MouseTarget::Map { y: 3, x: 4 }), |_, _| None),
            Some(String::new())
        );
    }

    #[test]
    fn empty_map_motion_clears_the_prompt_and_refreshes() {
        let event = MouseEvent {
            row: 8,
            col: 14,
            kind: MouseEventKind::Motion,
        };
        let mut cleared = false;
        let mut refreshes = 0;

        apply_hover(
            event,
            Some(MouseTarget::Map { y: 3, x: 4 }),
            |_, _| None,
            |_| panic!("empty descriptions must not be drawn"),
            || cleared = true,
            || refreshes += 1,
        );

        assert!(cleared);
        assert_eq!(refreshes, 1);
    }

    #[test]
    fn clicks_do_not_replace_the_prompt() {
        let event = MouseEvent {
            row: 8,
            col: 14,
            kind: MouseEventKind::Click(crate::user_interface::mouse::MouseButton::Left),
        };

        assert_eq!(
            hover_prompt(event, Some(MouseTarget::Map { y: 3, x: 4 }), |_, _| {
                panic!("clicks must not describe map cells")
            }),
            None
        );
    }
}
