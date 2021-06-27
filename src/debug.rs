use std::fs;
use std::io::Write;
use std::error;

// todo: Make this toggle-able
static DEBUG_ENABLED: bool = true;
const FILENAME: &'static str = "debug2.out";
static mut DEPTH: i32 = 0;

#[derive(Debug)]
enum DebugLevel {
    INFO,
    WARN,
    ERR,
    FATAL,
}

fn debug(level: DebugLevel, msg: &str) {
    fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(FILENAME)
        .expect("Failed to open debug file for writing")
        .write_all(&format!("{:?}: {}\n", level, msg).into_bytes())
        .expect("Failed to write to debug file");
}

pub fn info(msg: &str) {
    if DEBUG_ENABLED {
        debug(DebugLevel::INFO, msg);
    }
}

pub fn warn(msg: &str) {
    if DEBUG_ENABLED {
        debug(DebugLevel::WARN, msg);
    }
}

pub fn error(msg: &str) {
    if DEBUG_ENABLED {
        debug(DebugLevel::ERR, msg);
    }
}

pub fn fatal(msg: &str) -> ! {
    if DEBUG_ENABLED {
        debug(DebugLevel::FATAL, msg);
    }
    panic!("{}", msg);
}

fn log_error(e: &dyn error::Error) {
    debug(DebugLevel::FATAL, &format!("{}", e));
    match e.source() {
        Some(cause) => log_error(cause),
        None => ()
    }
}

pub fn fatal2(msg: &str, e: &dyn error::Error) -> ! {
    if DEBUG_ENABLED {
        debug(DebugLevel::FATAL, msg);
        debug(DebugLevel::FATAL, "!!BEGIN STACKTRACE!!");
        log_error(e);
        debug(DebugLevel::FATAL, "!!END STACKTRACE!!");
    }
    panic!("{}: {}", msg, e);
}

pub fn enter(function: &str) {
    unsafe {
        DEPTH += 1;
        info(&format!("{}: ENTER {}", DEPTH, function));
    }
}

pub fn leave(function: &str) {
    unsafe {
        DEPTH -= 1;
        info(&format!("{}: LEAVE {}", DEPTH, function));
    }
}
