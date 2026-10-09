use self::filestorage::FileStorageEngine;
pub use self::main::init_masters;
pub use self::main::load_masters;
pub use self::main::save_master;
pub(crate) use self::main::with_engine;
pub(crate) use self::main::{delete_save, list_saves, load_save, write_save};
#[cfg(test)]
pub(crate) use self::main::{
    delete_save_with_engine, list_saves_with_engine, load_save_with_engine, write_save_with_engine,
};
pub(crate) use self::main::{CharacterStorageError, PersistenceEngine, SaveKey};

mod main;

mod filestorage;
pub(crate) mod json;
#[cfg(test)]
pub(crate) mod memory;
