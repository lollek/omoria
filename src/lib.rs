#![crate_type = "staticlib"]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

extern crate libc;
extern crate serde;
extern crate serde_json;
extern crate pancurses;
extern crate enum_iterator;

mod pregame;

pub mod conversion;
pub mod commands;
pub mod constants;
pub mod data;
pub mod debug;
pub mod dungeon;
pub mod equipment;
pub mod error;
pub mod flow;
pub mod highscore;
pub mod io;
pub mod logic;
pub mod magic;
pub mod master;
pub mod master_interop;
pub mod menu;
pub mod misc;
pub mod model;
pub mod ncurses;
pub mod persistence;
pub mod player;
pub mod random;
pub mod save;
pub mod screen;
pub mod item_template;
pub mod term;
pub mod thirdparty;

pub mod bank_extern;
pub mod commands_extern;
pub mod constants_extern;
pub mod highscore_extern;
pub mod misc_extern;
pub mod ncurses_extern;
pub mod random_extern;
pub mod screen_extern;
pub mod term_extern;
