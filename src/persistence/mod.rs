use self::filestorage::FileStorageEngine;
pub use self::main::init_masters;
pub use self::main::load_masters;
pub use self::main::save_master;
pub(crate) use self::main::with_engine;
pub use self::main::PersistenceEngine;

mod main;

mod filestorage;
pub(crate) mod json;
#[cfg(test)]
pub(crate) mod memory;
