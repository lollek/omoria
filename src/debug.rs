use std::{fs, panic};
use std::io::Write;
use backtrace::Backtrace;

// todo: Make this toggle-able
static DEBUG_ENABLED: bool = true;
const FILENAME: &str = "debug_rust.out";

#[derive(Debug)]
enum DebugLevel {
    Info,
    Warning,
    Error,
    Fatal,
}

#[no_mangle]
extern "C" fn debug_init() {
    init();
}

pub fn init() {
    panic::set_hook(Box::new(|panic_info| {
        let (filename, line) =
            panic_info.location().map(|loc| (loc.file(), loc.line()))
            .unwrap_or(("<unknown>", 0));

        let cause = panic_info.payload().downcast_ref::<String>().map(String::to_owned)
            .unwrap_or_else(||
                            panic_info.payload().downcast_ref::<&str>().map(|it| it.to_string())
                            .unwrap_or_else(|| "<cause unknown>".to_owned()));

        error(format!("A panic occurred at {filename}:{line}: {cause}"));
        error(format!("{:?}", Backtrace::new()));
    }));
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
