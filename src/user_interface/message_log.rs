use super::layout::Rect;
use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LogLine {
    pub text: String,
    pub dim: bool,
}

#[derive(Default)]
pub(crate) struct MessageLog {
    lines: VecDeque<LogLine>,
    group_sizes: VecDeque<usize>,
    last: Option<(String, usize)>,
    last_unseen: usize,
    unseen: usize,
    turn: Option<libc::c_long>,
}

#[derive(Default)]
pub(crate) struct Scrollback {
    offset: usize,
}

impl Scrollback {
    pub(crate) fn offset(&self) -> usize {
        self.offset
    }

    pub(crate) fn should_close(key: u8) -> bool {
        key == crate::io::ESCAPE || key == (b'P' & 0x1f)
    }

    pub(crate) fn handle_key(&mut self, key: u8, line_count: usize, page_height: usize) {
        let maximum = line_count.saturating_sub(page_height);
        self.offset = self.offset.min(maximum);
        match key {
            b'8' => self.offset = self.offset.saturating_add(1).min(maximum),
            b'2' => self.offset = self.offset.saturating_sub(1),
            crate::key_translation::KEY_PAGE_UP => {
                self.offset = self.offset.saturating_add(page_height).min(maximum);
            }
            crate::key_translation::KEY_PAGE_DOWN => {
                self.offset = self.offset.saturating_sub(page_height);
            }
            crate::key_translation::KEY_HOME => self.offset = maximum,
            crate::key_translation::KEY_END => self.offset = 0,
            _ => {}
        }
    }

    pub(crate) fn run(
        page_height: usize,
        mut render: impl FnMut(usize) -> (usize, Vec<LogLine>),
        mut read_key: impl FnMut() -> u8,
        mut draw: impl FnMut(&[LogLine]),
    ) {
        let mut scrollback = Self::default();
        loop {
            let (line_count, lines) = render(scrollback.offset());
            draw(&lines);
            let key = read_key();
            if Self::should_close(key) {
                return;
            }
            scrollback.handle_key(key, line_count, page_height);
        }
    }
}

impl MessageLog {
    pub(crate) fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn command_read(&mut self) {
        self.input_read();
        for line in &mut self.lines {
            line.dim = true;
        }
    }

    pub fn input_read(&mut self) {
        self.unseen = 0;
        self.last_unseen = 0;
    }

    pub fn render(&self, rect: Rect) -> Vec<LogLine> {
        self.render_scrollback(rect, 0)
    }

    pub(crate) fn render_scrollback(&self, rect: Rect, offset: usize) -> Vec<LogLine> {
        assert!(rect.width > 0 && rect.height > 0, "empty log rectangle");
        let height = rect.height as usize;
        let width = rect.width as usize;
        let max_offset = self.lines.len().saturating_sub(height);
        let end = self.lines.len().saturating_sub(offset.min(max_offset));
        let start = end.saturating_sub(height);
        let visible_count = end - start;
        let blanks = height - visible_count;
        std::iter::repeat_n(
            LogLine {
                text: " ".repeat(width),
                dim: false,
            },
            blanks,
        )
        .chain(
            self.lines
                .iter()
                .skip(start)
                .take(visible_count)
                .map(|line| {
                    let mut text: String = line.text.chars().take(width).collect();
                    text.extend(std::iter::repeat_n(' ', width - text.chars().count()));
                    LogLine {
                        text,
                        dim: line.dim,
                    }
                }),
        )
        .collect()
    }

    pub fn print(
        &mut self,
        text: &str,
        turn: libc::c_long,
        rect: Rect,
        mut draw: impl FnMut(&[LogLine]),
        mut more: impl FnMut() -> bool,
    ) -> bool {
        assert!(rect.width > 0 && rect.height > 0, "empty log rectangle");
        if self.turn != Some(turn) {
            self.input_read();
            self.turn = Some(turn);
        }
        let repeats = match &self.last {
            Some((previous, count)) if previous == text => count + 1,
            _ => 1,
        };
        let lines = if repeats > 1 {
            for _ in 0..self.group_sizes.pop_back().expect("missing repeat group") {
                self.lines.pop_back();
            }
            self.unseen = self.unseen.saturating_sub(self.last_unseen);
            let suffix = format!(" (x{repeats})");
            let single_line = text.split_whitespace().collect::<Vec<_>>().join(" ");
            let prefix: String = single_line
                .chars()
                .take((rect.width as usize).saturating_sub(suffix.chars().count()))
                .collect();
            vec![format!("{prefix}{suffix}")]
        } else {
            if self.group_sizes.len() == crate::message::MAX_MESSAGES {
                for _ in 0..self.group_sizes.pop_front().expect("missing oldest group") {
                    self.lines.pop_front();
                }
            }
            wrap(text, rect.width as usize)
        };
        self.last = Some((text.to_owned(), repeats));
        self.group_sizes.push_back(lines.len());
        self.last_unseen = 0;
        let mut aborted = false;
        for line in lines {
            if self.unseen >= rect.height as usize && !aborted {
                draw(&self.render(rect));
                aborted = more();
                self.input_read();
            }
            self.lines.push_back(LogLine {
                text: line,
                dim: false,
            });
            self.unseen += 1;
            self.last_unseen += 1;
        }
        draw(&self.render(rect));
        aborted
    }
}

fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        let mut line = String::new();
        for word in paragraph.split_whitespace() {
            let mut chars = word.chars().peekable();
            if !line.is_empty() {
                if line.chars().count() + 1 + word.chars().count() > width {
                    lines.push(std::mem::take(&mut line));
                } else {
                    line.push(' ');
                }
            }
            while chars.peek().is_some() {
                line.extend(chars.by_ref().take(width - line.chars().count()));
                if line.chars().count() == width && chars.peek().is_some() {
                    lines.push(std::mem::take(&mut line));
                }
            }
        }
        lines.push(line);
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print(log: &mut MessageLog, text: &str, turn: libc::c_long, rect: Rect) {
        assert!(!log.print(text, turn, rect, |_| {}, || panic!("Unexpected -more-")));
    }

    fn texts(log: &MessageLog, rect: Rect) -> Vec<String> {
        log.render(rect).into_iter().map(|line| line.text).collect()
    }

    #[test]
    fn wraps_words_and_long_words_with_newest_lines_at_bottom() {
        let rect = Rect::new(25, 0, 6, 3);
        let mut log = MessageLog::default();
        print(&mut log, "Old", 1, rect);
        print(&mut log, "two abcdefgh", 2, rect);
        assert_eq!(texts(&log, rect), ["two   ", "abcdef", "gh    "]);
    }

    #[test]
    fn pads_blank_rows_and_dims_only_earlier_commands() {
        let rect = Rect::new(25, 0, 8, 3);
        let mut log = MessageLog::default();
        print(&mut log, "Old", 1, rect);
        log.command_read();
        print(&mut log, "New", 2, rect);
        let lines = log.render(rect);
        assert_eq!(texts(&log, rect), ["        ", "Old     ", "New     "]);
        assert!(!lines[0].dim);
        assert!(lines[1].dim);
        assert!(!lines[2].dim);
    }

    #[test]
    fn current_command_messages_stay_bright_across_input_and_redraws() {
        let rect = Rect::new(25, 0, 8, 3);
        let mut log = MessageLog::default();
        print(&mut log, "Old", 1, rect);
        log.command_read();
        print(&mut log, "New", 2, rect);
        print(&mut log, "Also new", 2, rect);
        for _ in 0..3 {
            log.input_read();
            let lines = log.render(rect);
            assert!(lines[0].dim);
            assert!(
                !lines[1].dim,
                "Latest messages dimmed without a newer message"
            );
            assert!(
                !lines[2].dim,
                "Latest messages dimmed without a newer message"
            );
        }
    }

    #[test]
    fn next_command_dims_previous_messages_even_within_the_same_turn() {
        let rect = Rect::new(25, 0, 8, 3);
        let mut log = MessageLog::default();
        print(&mut log, "First", 1, rect);
        assert!(!log.render(rect)[2].dim);
        log.command_read();
        assert!(log.render(rect)[2].dim);
        print(&mut log, "Next", 1, rect);
        let lines = log.render(rect);
        assert!(lines[1].dim);
        assert!(!lines[2].dim);
    }

    #[test]
    fn player_monster_and_status_messages_stay_bright_across_turn_numbers() {
        let rect = Rect::new(25, 0, 14, 4);
        let mut log = MessageLog::default();
        print(&mut log, "Previous", 1, rect);
        log.command_read();
        print(&mut log, "Player action", 2, rect);
        print(&mut log, "Monster action", 2, rect);
        print(&mut log, "Status update", 3, rect);
        let lines = log.render(rect);
        assert!(lines[0].dim);
        assert!(
            lines[1..].iter().all(|line| !line.dim),
            "All messages since the last command must remain bright"
        );
    }

    #[test]
    fn collapsed_repeat_stays_bright_after_prompt_input() {
        let rect = Rect::new(25, 0, 12, 2);
        let mut log = MessageLog::default();
        print(&mut log, "Miss", 1, rect);
        print(&mut log, "Miss", 2, rect);
        log.input_read();
        let lines = log.render(rect);
        assert_eq!(lines[1].text, "Miss (x2)   ");
        assert!(!lines[1].dim);
    }

    #[test]
    fn repeating_a_dimmed_message_in_a_new_command_makes_it_bright() {
        let rect = Rect::new(25, 0, 12, 2);
        let mut log = MessageLog::default();
        print(&mut log, "Miss", 1, rect);
        log.command_read();
        assert!(log.render(rect)[1].dim);
        print(&mut log, "Miss", 1, rect);
        let lines = log.render(rect);
        assert_eq!(lines[1].text, "Miss (x2)   ");
        assert!(!lines[1].dim);
    }

    #[test]
    fn repeats_collapse_to_one_truncated_line_with_count() {
        let rect = Rect::new(25, 0, 12, 2);
        let mut log = MessageLog::default();
        for _ in 0..3 {
            print(&mut log, "You miss the orc.", 1, rect);
        }
        assert_eq!(texts(&log, rect), ["            ", "You mis (x3)"]);
    }

    #[test]
    fn collapsing_a_wrapped_message_reveals_older_history() {
        let rect = Rect::new(25, 0, 12, 2);
        let mut log = MessageLog::default();
        print(&mut log, "Older", 1, rect);
        print(&mut log, "You miss the orc.", 2, rect);
        print(&mut log, "You miss the orc.", 2, rect);
        assert_eq!(texts(&log, rect), ["Older       ", "You mis (x2)"]);
    }

    #[test]
    fn collapsed_multiline_messages_stay_on_one_terminal_row() {
        let rect = Rect::new(25, 0, 12, 2);
        let mut log = MessageLog::default();
        print(&mut log, "one\ntwo", 1, rect);
        print(&mut log, "one\ntwo", 1, rect);
        assert_eq!(texts(&log, rect), ["            ", "one two (x2)"]);
    }

    #[test]
    fn display_retains_a_bounded_history_of_message_groups() {
        let rect = Rect::new(25, 0, 3, 60);
        let mut log = MessageLog::default();
        for index in 0..60 {
            print(&mut log, &index.to_string(), index, rect);
        }
        let lines = texts(&log, rect);
        assert!(lines[..10].iter().all(|line| line == "   "));
        assert_eq!(lines[10], "10 ");
        assert_eq!(lines[59], "59 ");
    }

    #[test]
    fn scrollback_renders_the_requested_window_and_clamps_offset() {
        let rect = Rect::new(1, 0, 3, 3);
        let mut log = MessageLog::default();
        for index in 0..6 {
            print(&mut log, &index.to_string(), index, rect);
        }

        assert_eq!(
            texts(&log, rect),
            ["3  ", "4  ", "5  "],
            "ordinary log rendering stays anchored to the newest lines"
        );
        assert_eq!(
            log.render_scrollback(rect, 1)
                .into_iter()
                .map(|line| line.text)
                .collect::<Vec<_>>(),
            ["2  ", "3  ", "4  "]
        );
        assert_eq!(
            log.render_scrollback(rect, usize::MAX)
                .into_iter()
                .map(|line| line.text)
                .collect::<Vec<_>>(),
            ["0  ", "1  ", "2  "]
        );
    }

    #[test]
    fn scrollback_navigation_moves_by_lines_pages_and_endpoints() {
        let mut scrollback = Scrollback::default();
        let line_count = 100;
        let page_height = 10;

        scrollback.handle_key(crate::key_translation::KEY_PAGE_UP, line_count, page_height);
        assert_eq!(scrollback.offset(), 10);
        scrollback.handle_key(b'8', line_count, page_height);
        assert_eq!(scrollback.offset(), 11);
        scrollback.handle_key(crate::key_translation::KEY_HOME, line_count, page_height);
        assert_eq!(scrollback.offset(), 90);
        scrollback.handle_key(
            crate::key_translation::KEY_PAGE_DOWN,
            line_count,
            page_height,
        );
        assert_eq!(scrollback.offset(), 80);
        scrollback.handle_key(crate::key_translation::KEY_END, line_count, page_height);
        assert_eq!(scrollback.offset(), 0);
        scrollback.handle_key(b'2', line_count, page_height);
        assert_eq!(scrollback.offset(), 0);
    }

    #[test]
    fn scrollback_navigation_clamps_when_history_fits_in_view() {
        let mut scrollback = Scrollback::default();
        scrollback.handle_key(crate::key_translation::KEY_HOME, 2, 10);
        assert_eq!(scrollback.offset(), 0);
    }

    #[test]
    fn scrollback_closes_on_escape_or_control_p() {
        assert!(Scrollback::should_close(crate::io::ESCAPE));
        assert!(Scrollback::should_close(b'P' & 0x1f));
        assert!(!Scrollback::should_close(crate::key_translation::KEY_END));
    }

    #[test]
    fn scrollback_runs_scripted_navigation_and_closes_on_escape() {
        let rect = Rect::new(1, 0, 16, 3);
        let mut log = MessageLog::default();
        print(&mut log, "Old", 1, rect);
        print(&mut log, "Repeated", 1, rect);
        log.command_read();
        print(&mut log, "Repeated", 2, rect);
        for index in 2..7 {
            print(&mut log, &index.to_string(), index, rect);
        }

        let keys = [
            crate::key_translation::KEY_PAGE_UP,
            b'8',
            crate::key_translation::KEY_PAGE_DOWN,
            crate::key_translation::KEY_END,
            crate::key_translation::KEY_HOME,
            crate::io::ESCAPE,
            b'2',
        ];
        let mut consumed = 0;
        let mut frames = Vec::new();
        Scrollback::run(
            rect.height as usize,
            |offset| (log.line_count(), log.render_scrollback(rect, offset)),
            || {
                let key = keys[consumed];
                consumed += 1;
                key
            },
            |lines| frames.push(lines.to_vec()),
        );

        assert_eq!(consumed, 6, "keys after Escape must remain for the game");
        assert_eq!(frames.len(), 6);
        assert_eq!(frames[0][0].text.trim(), "4");
        assert_eq!(frames[0][2].text.trim(), "6");
        assert_eq!(frames[1][0].text.trim(), "Repeated (x2)");
        assert_eq!(frames[1][2].text.trim(), "3");
        assert_eq!(frames[2][0].text.trim(), "Old");
        assert!(frames[2][0].dim);
        assert_eq!(frames[2][1].text.trim(), "Repeated (x2)");
        assert!(!frames[2][1].dim);
        assert_eq!(frames[3][0].text.trim(), "3");
        assert_eq!(frames[3][2].text.trim(), "5");
        assert_eq!(frames[4][0].text.trim(), "4");
        assert_eq!(frames[4][2].text.trim(), "6");
        assert_eq!(frames[5][0].text.trim(), "Old");
    }

    #[test]
    fn only_consecutive_repeats_collapse_and_current_command_is_bright() {
        let rect = Rect::new(25, 0, 10, 4);
        let mut log = MessageLog::default();
        print(&mut log, "A", 1, rect);
        print(&mut log, "A", 2, rect);
        print(&mut log, "B", 2, rect);
        print(&mut log, "A", 2, rect);
        assert_eq!(
            texts(&log, rect),
            ["          ", "A (x2)    ", "B         ", "A         "]
        );
        assert!(!log.render(rect)[1].dim);
    }

    #[test]
    fn more_occurs_exactly_when_unseen_lines_exceed_height() {
        let rect = Rect::new(25, 0, 10, 2);
        let mut log = MessageLog::default();
        print(&mut log, "A", 1, rect);
        print(&mut log, "B", 1, rect);
        let mut prompts = 0;
        assert!(log.print(
            "C",
            1,
            rect,
            |_| {},
            || {
                prompts += 1;
                true
            }
        ));
        assert_eq!(prompts, 1);
        assert_eq!(texts(&log, rect), ["B         ", "C         "]);
    }

    #[test]
    fn command_input_and_new_turn_reset_unseen_lines() {
        let rect = Rect::new(25, 0, 10, 1);
        let mut log = MessageLog::default();
        print(&mut log, "A", 1, rect);
        log.input_read();
        print(&mut log, "B", 1, rect);
        print(&mut log, "C", 2, rect);
    }

    #[test]
    fn collapsed_repeat_counts_as_one_unseen_line_even_after_input() {
        let rect = Rect::new(25, 0, 12, 1);
        let mut log = MessageLog::default();
        for _ in 0..12 {
            print(&mut log, "Miss", 1, rect);
        }
        log.input_read();
        print(&mut log, "Miss", 1, rect);
        assert_eq!(texts(&log, rect), ["Miss (x13)  "]);
        let mut prompts = 0;
        log.print(
            "Hit",
            1,
            rect,
            |_| {},
            || {
                prompts += 1;
                false
            },
        );
        assert_eq!(prompts, 1);
    }

    #[test]
    fn long_message_is_paged_before_unseen_lines_are_lost() {
        let rect = Rect::new(25, 0, 4, 2);
        let mut log = MessageLog::default();
        let mut frames = Vec::new();
        let mut prompts = 0;
        assert!(!log.print(
            "abcdefghijklmnopqrst",
            1,
            rect,
            |lines| frames.push(lines.to_vec()),
            || {
                prompts += 1;
                false
            }
        ));
        assert_eq!(prompts, 2);
        assert!(frames.iter().flatten().all(|line| !line.dim));
        assert_eq!(
            frames[0]
                .iter()
                .map(|line| line.text.as_str())
                .collect::<Vec<_>>(),
            ["abcd", "efgh"]
        );
        assert_eq!(texts(&log, rect), ["mnop", "qrst"]);
    }

    #[test]
    fn escape_skips_remaining_pages_but_keeps_newest_text() {
        let rect = Rect::new(25, 0, 2, 1);
        let mut log = MessageLog::default();
        let mut prompts = 0;
        assert!(log.print(
            "abcdefgh",
            1,
            rect,
            |_| {},
            || {
                prompts += 1;
                true
            }
        ));
        assert_eq!(prompts, 1);
        assert_eq!(texts(&log, rect), ["gh"]);
    }

    #[test]
    fn explicit_newlines_empty_messages_and_unicode_wrap_safely() {
        let rect = Rect::new(25, 0, 2, 4);
        let mut log = MessageLog::default();
        print(&mut log, "a\n\n\u{e9}\u{e9}\u{e9}", 1, rect);
        assert_eq!(texts(&log, rect), ["a ", "  ", "\u{e9}\u{e9}", "\u{e9} "]);
        log.input_read();
        print(&mut log, "", 1, rect);
        assert_eq!(texts(&log, rect)[3], "  ");
    }
}
