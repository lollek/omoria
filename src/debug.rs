use std::fs;
use std::io::Write;

// todo: Make this toggle-able
static DEBUG_ENABLED: bool = true;
const FILENAME: &str = "debug_rust.out";
static mut DEPTH: i32 = 0;

#[derive(Debug)]
enum DebugLevel {
    Info,
    Warning,
    Error,
    Fatal,
}

fn debug<S>(level: DebugLevel, msg: S)
where
    S: AsRef<str>,
{
    fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(FILENAME)
        .expect("Failed to open debug file for writing")
        .write_all(&format!("{:?}: {}\n", level, msg.as_ref()).into_bytes())
        .expect("Failed to write to debug file");
}

pub fn info<S>(msg: S)
where
    S: AsRef<str>,
{
    if DEBUG_ENABLED {
        debug(DebugLevel::Info, msg.as_ref());
    }
}

pub fn warn<S>(msg: S)
where
    S: AsRef<str>,
{
    if DEBUG_ENABLED {
        debug(DebugLevel::Warning, msg.as_ref());
    }
}

pub fn error<S>(msg: S)
where
    S: AsRef<str>,
{
    if DEBUG_ENABLED {
        debug(DebugLevel::Error, msg.as_ref());
    }
}

pub fn fatal<S>(msg: S) -> !
where
    S: AsRef<str>,
{
    if DEBUG_ENABLED {
        debug(DebugLevel::Fatal, msg.as_ref());
    }
    panic!("{}", msg.as_ref());
}

pub fn enter<S>(function: S)
where
    S: AsRef<str>,
{
    unsafe {
        DEPTH += 1;
        info(format!("{}: ENTER {}", DEPTH, function.as_ref()));
    }
}

pub fn leave<S>(function: S)
where
    S: AsRef<str>,
{
    unsafe {
        DEPTH -= 1;
        info(format!("{}: LEAVE {}", DEPTH, function.as_ref()));
    }
}
