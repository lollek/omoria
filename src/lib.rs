#![crate_type = "staticlib"]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde_derive;

extern crate libc;
extern crate serde;
extern crate serde_json;

pub mod bank;
pub mod classes;
pub mod create;
pub mod debug;
pub mod io;
pub mod master;
pub mod magic;
pub mod misc;
pub mod ncurses;
pub mod player;
pub mod save;
pub mod screen;
pub mod random;
pub mod term;
pub mod types;
