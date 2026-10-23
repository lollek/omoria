use super::layout::Rect;
use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LogLine {
    pub text: String,
    pub dim: bool,
}

#[derive(Default)]
pub(crate) struct MessageLog {
    lines: VecDeque<(String, libc::c_long)>,
    group_sizes: VecDeque<usize>,
    last: Option<(String, usize)>,
    last_unseen: usize,
    unseen: usize,
    turn: Option<libc::c_long>,
}

impl MessageLog {
    pub fn input_read(&mut self) {
        self.unseen = 0;
        self.last_unseen = 0;
    }

    pub fn render(&self, rect: Rect, turn: libc::c_long) -> Vec<LogLine> {
        assert!(rect.width > 0 && rect.height > 0, "empty log rectangle");
        let height = rect.height as usize;
        let width = rect.width as usize;
        let skip = self.lines.len().saturating_sub(height);
        let blanks = height.saturating_sub(self.lines.len());
        std::iter::repeat_n(
            LogLine {
                text: " ".repeat(width),
                dim: false,
            },
            blanks,
        )
        .chain(self.lines.iter().skip(skip).map(|(text, printed_turn)| {
            let mut text: String = text.chars().take(width).collect();
            text.extend(std::iter::repeat_n(' ', width - text.chars().count()));
            LogLine {
                text,
                dim: *printed_turn < turn,
            }
        }))
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
                draw(&self.render(rect, turn));
                aborted = more();
                self.input_read();
            }
            self.lines.push_back((line, turn));
            self.unseen += 1;
            self.last_unseen += 1;
        }
        draw(&self.render(rect, turn));
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

    fn texts(log: &MessageLog, rect: Rect, turn: libc::c_long) -> Vec<String> {
        log.render(rect, turn)
            .into_iter()
            .map(|line| line.text)
            .collect()
    }

    #[test]
    fn wraps_words_and_long_words_with_newest_lines_at_bottom() {
        let rect = Rect::new(25, 0, 6, 3);
        let mut log = MessageLog::default();
        print(&mut log, "Old", 1, rect);
        print(&mut log, "two abcdefgh", 2, rect);
        assert_eq!(texts(&log, rect, 2), ["two   ", "abcdef", "gh    "]);
    }

    #[test]
    fn pads_blank_rows_and_dims_only_earlier_turns() {
        let rect = Rect::new(25, 0, 8, 3);
        let mut log = MessageLog::default();
        print(&mut log, "Old", 1, rect);
        print(&mut log, "New", 2, rect);
        let lines = log.render(rect, 2);
        assert_eq!(texts(&log, rect, 2), ["        ", "Old     ", "New     "]);
        assert!(!lines[0].dim);
        assert!(lines[1].dim);
        assert!(!lines[2].dim);
        assert!(log.render(rect, 3)[2].dim);
    }

    #[test]
    fn repeats_collapse_to_one_truncated_line_with_count() {
        let rect = Rect::new(25, 0, 12, 2);
        let mut log = MessageLog::default();
        for _ in 0..3 {
            print(&mut log, "You miss the orc.", 1, rect);
        }
        assert_eq!(texts(&log, rect, 1), ["            ", "You mis (x3)"]);
    }

    #[test]
    fn collapsing_a_wrapped_message_reveals_older_history() {
        let rect = Rect::new(25, 0, 12, 2);
        let mut log = MessageLog::default();
        print(&mut log, "Older", 1, rect);
        print(&mut log, "You miss the orc.", 2, rect);
        print(&mut log, "You miss the orc.", 2, rect);
        assert_eq!(texts(&log, rect, 2), ["Older       ", "You mis (x2)"]);
    }

    #[test]
    fn collapsed_multiline_messages_stay_on_one_terminal_row() {
        let rect = Rect::new(25, 0, 12, 2);
        let mut log = MessageLog::default();
        print(&mut log, "one\ntwo", 1, rect);
        print(&mut log, "one\ntwo", 1, rect);
        assert_eq!(texts(&log, rect, 1), ["            ", "one two (x2)"]);
    }

    #[test]
    fn display_retains_a_bounded_history_of_message_groups() {
        let rect = Rect::new(25, 0, 3, 60);
        let mut log = MessageLog::default();
        for index in 0..60 {
            print(&mut log, &index.to_string(), index, rect);
        }
        let lines = texts(&log, rect, 60);
        assert!(lines[..10].iter().all(|line| line == "   "));
        assert_eq!(lines[10], "10 ");
        assert_eq!(lines[59], "59 ");
    }

    #[test]
    fn only_consecutive_repeats_collapse_and_latest_turn_is_bright() {
        let rect = Rect::new(25, 0, 10, 4);
        let mut log = MessageLog::default();
        print(&mut log, "A", 1, rect);
        print(&mut log, "A", 2, rect);
        print(&mut log, "B", 2, rect);
        print(&mut log, "A", 2, rect);
        assert_eq!(
            texts(&log, rect, 2),
            ["          ", "A (x2)    ", "B         ", "A         "]
        );
        assert!(!log.render(rect, 2)[1].dim);
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
        assert_eq!(texts(&log, rect, 1), ["B         ", "C         "]);
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
        assert_eq!(texts(&log, rect, 1), ["Miss (x13)  "]);
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
        assert_eq!(
            frames[0]
                .iter()
                .map(|line| line.text.as_str())
                .collect::<Vec<_>>(),
            ["abcd", "efgh"]
        );
        assert_eq!(texts(&log, rect, 1), ["mnop", "qrst"]);
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
        assert_eq!(texts(&log, rect, 1), ["gh"]);
    }

    #[test]
    fn explicit_newlines_empty_messages_and_unicode_wrap_safely() {
        let rect = Rect::new(25, 0, 2, 4);
        let mut log = MessageLog::default();
        print(&mut log, "a\n\n\u{e9}\u{e9}\u{e9}", 1, rect);
        assert_eq!(
            texts(&log, rect, 1),
            ["a ", "  ", "\u{e9}\u{e9}", "\u{e9} "]
        );
        log.input_read();
        print(&mut log, "", 1, rect);
        assert_eq!(texts(&log, rect, 1)[3], "  ");
    }
}
