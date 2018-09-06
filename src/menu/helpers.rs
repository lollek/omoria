use term;
use io;
use ncurses;

const WIDTH: usize = 60;
const HEIGHT: usize = 24;

pub fn draw_menu(title: &str, items: &Vec<&str>, commands: &str, selected: u8) {
    // Title
    term::put_buffer(&format!("{:*<width$}", "", width=WIDTH), 0, 0);
    term::put_buffer(&format!("* {}{:<width$} *", title, "", width=WIDTH - title.len() - 4) , 1, 0);
    term::put_buffer(&format!("{:*<width$}", "", width=WIDTH), 2, 0);

    // Side markers
    for y in 3..(HEIGHT - 3) {
        term::put_buffer(&format!("* {:<width$} *", "", width = WIDTH - 4), y as i32, 0);
    }

    // Items
    for (index, item) in items.iter().enumerate() {
        let reverse = selected == index as u8;
        if reverse {
            ncurses::chattr(ncurses::CursesAttr::Reverse, true);
        }
        term::put_buffer(item, index as i32 + 3, 2);

        if reverse {
            ncurses::chattr(ncurses::CursesAttr::Reverse, false);
        }
    }

    // Bottom
    term::put_buffer(&format!("{:*<width$}", "", width=WIDTH), HEIGHT as i32 - 3, 0);
    term::put_buffer(&format!("* {:<width$}{} *", "", commands, width=WIDTH - commands.len() - 4) , HEIGHT as i32 - 2, 0);
    term::put_buffer(&format!("{:*<width$}", "", width=WIDTH), HEIGHT as i32 - 1, 0);
}
