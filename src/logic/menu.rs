use crate::term;
use crate::ncurses;
use crate::io;

const WIDTH: usize = 80;
const MAX_X: usize = WIDTH - 5;
const HEIGHT: usize = 24;
const MAX_Y: usize = HEIGHT - 6;

pub fn draw_quick_menu<S1>(title: S1, items: &[&str])
    where S1: AsRef<str>,
{
    // Title
    let title_s = title.as_ref();
    term::put_buffer(title_s, 0, 0);

    // Items
    for (index, item) in items.as_ref().iter().enumerate() {
        let character = ((index as u8) + ('a' as u8)) as char;
        term::put_buffer(&format!(" {}) {:<width$}", character, item, width=20),
            index as i32 + 1, 0);
    }
    term::put_buffer(&format!("    {:<width$}", "", width=20), items.len() as i32 + 1, 0);
}

pub fn draw_menu<S1, S2>(title: S1, items: &[&str], commands: S2, selected: u8)
    where S1: AsRef<str>,
          S2: AsRef<str>,
{
    // Title
    let title_s = title.as_ref();
    term::put_buffer(&format!("{:*<width$}", "", width=WIDTH - 1), 0, 0);
    term::put_buffer(&format!("* {}{:<width$} *", title_s, "", width=MAX_X - title_s.len()) , 1, 0);
    term::put_buffer(&format!("{:*<width$}", "", width=WIDTH - 1), 2, 0);

    // Side markers
    for y in 3..(HEIGHT - 3) {
        term::put_buffer(&format!("* {:<width$} *", "", width = MAX_X), y as i32, 0);
    }

    // Items
    for (index, item) in items.as_ref().iter().enumerate() {
        let reverse = selected == index as u8;
        if reverse {
            ncurses::attron(ncurses::A_REVERSE);
        }
        term::put_buffer(item, index as i32 + 3, 2);

        if reverse {
            ncurses::attroff(ncurses::A_REVERSE);
        }
    }

    // Bottom
    let commands_s = commands.as_ref();
    term::put_buffer(&format!("{:*<width$}", "", width=WIDTH - 1), HEIGHT as i32 - 3, 0);
    term::put_buffer(&format!("* {:<width$}{} *", "", commands_s, width=MAX_X - commands_s.len()) , HEIGHT as i32 - 2, 0);
    term::put_buffer(&format!("{:*<width$}", "", width=WIDTH - 1), HEIGHT as i32 - 1, 0);
}

pub fn draw_help_vec<S1>(title: S1, lines: &[&str])
    where S1: AsRef<str>,
{
    let commands =
        if lines.len() <= MAX_Y {
            "ESC=close"
        } else {
            "ESC=close, SPACE=next page"
        };

    let mut page = 0;

    loop {
        let visible_lines: Vec<&str> = lines.iter()
            .skip(page * MAX_Y)
            .take(MAX_Y)
            .map(|it| it.as_ref())
            .collect();

        draw_menu(
            title.as_ref(),
            &visible_lines,
            commands,
            255);

        match io::inkey_flush() {
            /* ESC */
            27 => return,
            /* SPACE */
            32 => {
                page =
                    if page == lines.len() / MAX_Y {
                        0
                    } else {
                        page + 1
                    }
            },
            _ => {}
        }
    }
}

pub fn draw_help<S1, S2>(title: S1, msg: S2)
    where S1: AsRef<str>,
          S2: AsRef<str>,
{
    let mut lines = vec![String::new()];
    let mut msg_iter = msg.as_ref().split_whitespace();
    while let Some(word) = msg_iter.next() {
        if lines.last().unwrap().len() + word.len() >= MAX_X {
            lines.push(String::new());
        }
        let last = lines.last_mut().unwrap();
        if last.len() > 0 {
            last.push_str(" ");
        }
        last.push_str(word);
    }

    draw_help_vec(title, &lines.iter().map(|it| it.as_ref()).collect::<Vec<&str>>());
}
