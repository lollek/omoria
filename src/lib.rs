#![crate_type = "staticlib"]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

extern crate enum_iterator;
extern crate libc;
extern crate pancurses;
extern crate serde;
extern crate serde_json;

mod town_level;
mod pregame;

pub mod commands;
pub mod constants;
pub mod conversion;
pub mod data;
pub mod debug;
pub mod equipment;
pub mod error;
pub mod flow;
pub mod highscore;
pub mod inventory;
pub mod io;
pub mod logic;
pub mod master;
pub mod master_interop;
pub mod message;
pub mod misc;
pub mod model;
pub mod ncurses;
pub mod persistence;
pub mod player;
pub mod random;
pub mod save;
pub mod screen;
pub mod term;
pub mod thirdparty;

pub mod commands_extern;
pub mod constants_extern;
pub mod generate_item;
pub mod highscore_extern;
pub mod identification;
pub mod misc_extern;
pub mod ncurses_extern;
pub mod random_extern;
pub mod screen_extern;
pub mod term_extern;
