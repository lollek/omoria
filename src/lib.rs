#![crate_type = "staticlib"]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

extern crate libc;
extern crate serde;
extern crate serde_json;

pub mod constants;
pub mod create;
pub mod debug;
pub mod highscore;
pub mod io;
pub mod magic;
pub mod master;
pub mod misc;
pub mod ncurses;
pub mod player;
pub mod random;
pub mod save;
pub mod screen;
pub mod term;
pub mod thirdparty;
pub mod types;

pub mod bank_extern;
pub mod classes_extern;
pub mod create_extern;
pub mod highscore_extern;
pub mod master_extern;
pub mod misc_extern;
pub mod player_extern;
pub mod random_extern;
pub mod screen_extern;
pub mod term_extern;
