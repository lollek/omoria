use std::fs;
use std::cmp::min;

use constants;
use debug;
use term;
use io;
use menu::helpers;

#[derive(Clone, Debug)]
pub struct Character {
    pub name: String,
    pub uid: String,
}

fn print_banner() {
    debug::enter("main_menu::print_banner");

    helpers::draw_menu(
        format!("Omoria {}", constants::OMORIA_VERSION),
        &vec![
        "",
        "COPYRIGHT (c) Robert Alan Koeneke",
        "",
        "Programers : Robert Alan Koeneke / University of Oklahoma",
        "             Jimmey Wayne Todd   / University of Oklahoma",
        "",
        "Based on University of Washington version 4.8",
        "",
        "UW Modifications by : Kenneth Case, Mary Conner,",
        "                      Robert DeLoura, Dan Flye,",
        "                      Todd Gardiner, Dave Jungck,",
        "                      Andy Walker, Dean Yasuda.",
        "",
        "Linux port by Stephen Kertes, 1997-2000.",
        "",
        "Updates by Olle Kvarnstrom, 2018.",
        ],
        "Press any key to continue",
        255);

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

pub fn main_menu() -> Option<Character> {
    debug::enter("main_menu::main_menu");

    print_banner();

    let characters = load_characters();
    let char_names: Vec<&str> = characters.iter().map(|it| it.name.as_str()).collect();
    let mut index = 0;
    let mut retval = None;

    loop {
        helpers::draw_menu(
            "Select your adventurer",
            &char_names,
            "j=down, k=up, enter=select, n=new",
            index);

        match io::inkey_flush() as char {
            'k' => index = if index == 0 { 0 } else { index - 1 },
            'j' => index = min(characters.len() as u8 -1, index + 1),
            'n' | 'N' => break,
            '\r' => {
                if characters.is_empty() {
                    continue;
                }
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
