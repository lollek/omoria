use std::fs;
use std::cmp::min;

use constants;
use debug;
use io;
use logic::menu;
use player;
use term;

#[derive(Clone, Debug)]
struct Character {
    pub name: String,
    pub uid: String,
}

#[no_mangle]
pub extern fn pregame__menu_rs() {
    debug::enter("pregame__menu_rs");

    if let Some(character) = main_menu() {
        player::set_name(&character.name);
        player::set_uid(character.uid.parse::<i64>().unwrap());
    }

    debug::leave("pregame__menu_rs");
}

/**
 * main_menu() - Select character to load, or create new
 *
 * @returns
 * - Some(Character) to load a character
 * - None to create a new character
 */
fn main_menu() -> Option<Character> {
    debug::enter("pregame::main_menu");

    print_banner();

    let characters = load_characters();
    let char_names: Vec<&str> = characters.iter().map(|it| it.name.as_str()).collect();
    let mut index = 0;
    let mut retval = None;

    loop {
        menu::draw_menu(
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

    debug::leave("pregame::main_menu");
    retval
}

fn print_banner() {
    debug::enter("pregame::print_banner");

    menu::draw_menu(
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
        "Updates by Olle Kvarnstrom, 2018-2022.",
        ],
        "Press any key to continue",
        255);

    io::inkey_flush();
    term::clear_screen();

    debug::leave("pregame::print_banner");
}

fn load_characters() -> Vec<Character> {
    debug::enter("pregame::load_characters");

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

    debug::leave("pregame::load_characters");
    res
}

