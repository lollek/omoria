use term;
use ncurses;
use io;

const WIDTH: usize = 80;
const MAX_X: usize = WIDTH - 5;
const HEIGHT: usize = 24;
const MAX_Y: usize = HEIGHT - 6;

pub fn draw_menu(title: &str, items: &Vec<&str>, commands: &str, selected: u8) {
    // Title
    term::put_buffer(&format!("{:*<width$}", "", width=WIDTH - 1), 0, 0);
    term::put_buffer(&format!("* {}{:<width$} *", title, "", width=MAX_X - title.len()) , 1, 0);
    term::put_buffer(&format!("{:*<width$}", "", width=WIDTH - 1), 2, 0);

    // Side markers
    for y in 3..(HEIGHT - 3) {
        term::put_buffer(&format!("* {:<width$} *", "", width = MAX_X), y as i32, 0);
    }

    // Items
    for (index, item) in items.iter().enumerate() {
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
    term::put_buffer(&format!("{:*<width$}", "", width=WIDTH - 1), HEIGHT as i32 - 3, 0);
    term::put_buffer(&format!("* {:<width$}{} *", "", commands, width=MAX_X - commands.len()) , HEIGHT as i32 - 2, 0);
    term::put_buffer(&format!("{:*<width$}", "", width=WIDTH - 1), HEIGHT as i32 - 1, 0);
}

pub fn draw_help_vec(title: &str, lines: &Vec<&str>) {
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
            .map(|it| it.to_owned())
            .collect();

        draw_menu(
            title,
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

pub fn draw_help(title: &str, msg: &str) {
    let mut lines = vec![String::new()];
    let mut msg_iter = msg.split_whitespace();
    while let Some(word) = msg_iter.next() {
        if lines.last().unwrap().len() + word.len() >= MAX_X {
            lines.push(String::new());
        }
        let mut last = lines.last_mut().unwrap();
        if last.len() > 0 {
            last.push_str(" ");
        }
        last.push_str(word);
    }

    draw_help_vec(title, &lines.iter().map(|it| it.as_str()).collect());
}
