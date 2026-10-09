use std::cmp::min;

use crate::constants;
use crate::debug;
use crate::io;
use crate::logic::menu;
use crate::master;
use crate::persistence::{self, CharacterStorageError, SaveKey};
use crate::player;
use crate::term;

#[derive(Clone, Debug)]
struct Character {
    pub name: String,
    pub uid: i64,
}

#[no_mangle]
pub extern "C" fn pregame__menu_rs() {
    if let Some(character) = main_menu() {
        player::set_name(&character.name);
        player::set_uid(character.uid);
    }
}

///
/// Select character to load, or create new
///
/// Returns
/// - `Some(Character)` to load a character
/// - `None` to create a new character
///
fn main_menu() -> Option<Character> {
    print_banner();
    show_highscore();

    let characters = load_characters();
    let char_names: Vec<&str> = characters.iter().map(|it| it.name.as_str()).collect();
    let mut index = 0;
    let mut retval = None;

    loop {
        menu::draw_menu(
            "Select your adventurer",
            &char_names,
            "j=down, k=up, enter=select, n=new",
            index,
        );

        match io::inkey_flush() as char {
            'k' => index = previous_index(index),
            'j' => index = next_index(index, characters.len()),
            'n' | 'N' => break,
            '\r' => {
                if characters.is_empty() {
                    continue;
                }
                retval = Some(characters[index].to_owned());
                break;
            }
            _ => {}
        }
    }

    term::clear_screen();
    retval
}

fn print_banner() {
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
        255,
    );

    io::inkey_flush();
    term::clear_screen();
}

fn show_highscore() {
    let mut master = master::read_master().unwrap();
    master.sort_unstable_by_key(|item| std::cmp::Reverse(item.points));
    // println!("Username     Points   Alive    Character name    Level  Race         Class");
    // println!("____________ ________ _____ ________________________ __ __________ ________________");

    let lines = master
        .iter()
        .map(|item| {
            format!(
                "{:<24}  Level {}  {:>10}  {:>9}  {:>5}  {:>9}",
                item.character_name,
                item.level,
                item.race,
                item.class,
                if item.alive { "alive" } else { "dead" },
                item.points,
            )
        })
        .collect::<Vec<String>>();
    menu::draw_help_vec(
        "Highscore",
        &lines.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
    );

    term::clear_screen();
}

fn load_characters() -> Vec<Character> {
    characters_from_saves(persistence::list_saves(), debug::error)
}

fn characters_from_saves(
    result: Result<Vec<SaveKey>, CharacterStorageError>,
    mut log: impl FnMut(String),
) -> Vec<Character> {
    match result {
        Ok(keys) => keys
            .into_iter()
            .map(|key| Character {
                name: key.name,
                uid: key.uid,
            })
            .collect(),
        Err(err) => {
            log(format!("Failed to list saves: {}", err));
            Vec::new()
        }
    }
}

fn next_index(index: usize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    min(index + 1, len - 1)
}

fn previous_index(index: usize) -> usize {
    index.saturating_sub(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::{list_saves_with_engine, memory::InMemoryEngine};

    #[test]
    fn character_menu_list_error_logs_and_returns_empty_list() {
        let mut engine = InMemoryEngine {
            fail_list: true,
            ..Default::default()
        };
        let mut messages = Vec::new();
        let characters = characters_from_saves(list_saves_with_engine(&mut engine), |message| {
            messages.push(message)
        });
        assert!(characters.is_empty());
        assert_eq!(
            messages,
            vec!["Failed to list saves: injected list failure"]
        );
        assert_eq!(engine.character_calls, vec![("list", None)]);
    }

    #[test]
    fn character_menu_uses_typed_uid_without_parsing() {
        let mut engine = InMemoryEngine::default();
        engine.saves.insert(
            SaveKey {
                name: "Fixture-With-Hyphens".into(),
                uid: -42,
            },
            String::new(),
        );
        let mut messages = Vec::new();
        let characters = characters_from_saves(list_saves_with_engine(&mut engine), |message| {
            messages.push(message)
        });
        assert_eq!(characters.len(), 1);
        assert_eq!(characters[0].name, "Fixture-With-Hyphens");
        assert_eq!(characters[0].uid, -42);
        assert!(messages.is_empty());
        assert_eq!(engine.character_calls, vec![("list", None)]);
    }

    #[test]
    fn character_menu_down_on_empty_list_keeps_index_zero() {
        assert_eq!(next_index(0, 0), 0);
    }

    #[test]
    fn character_menu_down_on_single_entry_keeps_index_zero() {
        assert_eq!(next_index(0, 1), 0);
    }

    #[test]
    fn character_menu_down_moves_to_next_entry() {
        assert_eq!(next_index(0, 3), 1);
    }

    #[test]
    fn character_menu_down_stops_at_last_entry() {
        assert_eq!(next_index(2, 3), 2);
    }

    #[test]
    fn character_menu_up_stops_at_first_entry() {
        assert_eq!(previous_index(0), 0);
    }

    #[test]
    fn character_menu_up_moves_to_previous_entry() {
        assert_eq!(previous_index(2), 1);
    }

    #[test]
    fn character_menu_down_moves_past_index_255_in_large_list() {
        assert_eq!(next_index(255, 300), 256);
    }

    #[test]
    fn character_menu_down_stops_at_last_entry_in_large_list() {
        assert_eq!(next_index(299, 300), 299);
    }
}
