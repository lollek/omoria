use std::fs;
use std::cmp::{min, max};

use constants;
use ncurses;
use debug;
use term;
use io;

#[derive(Clone, Debug)]
pub struct Character {
    pub name: String,
    pub uid: String,
}

fn print_banner() {
    debug::enter("main_menu::print_banner");

    term::put_buffer("*************************************************************", 0, 0);
    term::put_buffer("* Omoria                                                    *", 1, 0);
    term::put_buffer(constants::OMORIA_VERSION, 1, 9);
    term::put_buffer("*************************************************************", 2, 0);
    term::put_buffer("*                                                           *", 3, 0);
    term::put_buffer("*           COPYRIGHT (c) Robert Alan Koeneke               *", 4, 0);
    term::put_buffer("*                                                           *", 5, 0);
    term::put_buffer("* Programers : Robert Alan Koeneke / University of Oklahoma *", 6, 0);
    term::put_buffer("*              Jimmey Wayne Todd   / University of Oklahoma *", 7, 0);
    term::put_buffer("*                                                           *", 8, 0);
    term::put_buffer("* Based on University of Washington version 4.8             *", 9, 0);
    term::put_buffer("*                                                           *", 10, 0);
    term::put_buffer("* UW Modifications by : Kenneth Case, Mary Conner,          *", 11, 0);
    term::put_buffer("*                       Robert DeLoura, Dan Flye,           *", 12, 0);
    term::put_buffer("*                       Todd Gardiner, Dave Jungck,         *", 13, 0);
    term::put_buffer("*                       Andy Walker, Dean Yasuda.           *", 14, 0);
    term::put_buffer("*                                                           *", 15, 0);
    term::put_buffer("* Linux port by Stephen Kertes, 1997-2000.                  *", 16, 0);
    term::put_buffer("*                                                           *", 17, 0);
    term::put_buffer("* Updates by Olle Kvarnstrom, 2018.                         *", 18, 0);
    term::put_buffer("*                                                           *", 19, 0);
    term::put_buffer("*                                                           *", 20, 0);
    term::put_buffer("*************************************************************", 21, 0);
    term::put_buffer("*                                 Press any key to continue *", 22, 0);
    term::put_buffer("*************************************************************", 23, 0);
    io::inkey_flush();
    term::clear_screen();

    debug::leave("main_menu::print_banner");
}

fn load_characters() -> Vec<Character> {
    debug::enter("main_menu::load_characters");

    let res = fs::read_dir(constants::SAVE_FOLDER)
        .unwrap()
        .map(|it| it.unwrap()
             .file_name()
             .into_string()
             .unwrap()
             .to_owned())
        .filter(|it| it.find(".json") != None)
        .map(|it| it.replace(".json", ""))
        .map(|it| {
            let (name, uid) = it.split_at(it.rfind("-").unwrap());
            Character {
                name: name.to_string(),
                uid: uid[1..uid.len()].to_string(),
            }
        })
    .collect();

    debug::leave("main_menu::load_characters");
    res
}

fn redraw(characters: &Vec<Character>, selected: i8) {
    debug::enter("main_menu::redraw");

    term::put_buffer("*************************************************************", 0, 0);
    term::put_buffer("* Select your adventurer                                    *", 1, 0);
    term::put_buffer("*************************************************************", 2, 0);
    (3..21i32).for_each(|y| term::put_buffer("*                                                           *", y, 0));

    for (index, c) in characters.iter().enumerate() {
        let reverse = selected == index as i8;
        if reverse {
            ncurses::chattr(ncurses::CursesAttr::Reverse, true);
        }
        term::put_buffer(&format!("* {:>57} *", c.name), (index + 3) as i32, 0);

        if reverse {
            ncurses::chattr(ncurses::CursesAttr::Reverse, false);
        }
    }
    term::put_buffer("*************************************************************",21, 0);
    term::put_buffer("*                         j=down, k=up, enter=select, n=new *",22, 0);
    term::put_buffer("*************************************************************",23, 0);

    debug::leave("main_menu::redraw");
}

pub fn main_menu() -> Option<Character> {
    debug::enter("main_menu::main_menu");

    print_banner();

    let characters = load_characters();
    let mut index = 0;
    let mut retval = None;

    loop {
        redraw(&characters, index);
        match io::inkey_flush() as char {
            'k' => index = max(0, index - 1),
            'j' => index = min(characters.len() as i8 -1, index + 1),
            'n' | 'N' => break,
            '\r' => {
                retval = Some(characters[index as usize].to_owned());
                break;
            },
            _ => {},
        }
    }

    term::clear_screen();

    debug::leave("main_menu::main_menu");
    retval
}
