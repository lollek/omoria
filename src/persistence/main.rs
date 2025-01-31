use crate::error::Error;
use std::sync::RwLock;

use crate::master::MasterRecord;
use crate::persistence::FileStorageEngine;

pub trait PersistenceEngine
where
    Self: Sync + Send,
{
    fn init_masters(&mut self) -> Result<(), Error>;
    fn load_masters(&mut self) -> Result<Vec<MasterRecord>, Error>;
    fn save_master(&mut self, record: MasterRecord, allow_new: bool) -> Result<(), Error>;
}

lazy_static! {
    // It would be nice if we could make a setter function and set the engine from that instead.
    // Since that would remove the two-way dependency
    static ref ENGINE: RwLock<Option<Box<dyn PersistenceEngine>>> = RwLock::new(Some(Box::new(FileStorageEngine)));
}

fn with_engine<T>(
    fun: impl FnOnce(&mut dyn PersistenceEngine) -> Result<T, Error>,
) -> Result<T, Error> {
    let mut lock = ENGINE
        .try_write()
        .map_err(|_| "Error in persistence engine")?;
    let engine = lock
        .as_deref_mut()
        .ok_or("No persistence engine assigned!")?;
    fun(engine)
}

/**
 * init_masters() - Init masters for use
 */
pub fn init_masters() -> Result<(), Error> {
    with_engine(|engine| engine.init_masters())
}

/**
 * load_masters() - Load all characters from the master list
 */
pub fn load_masters() -> Result<Vec<MasterRecord>, Error> {
    with_engine(|engine| engine.load_masters())
}

/**
 * save_master() - Save a master record
 * @record:     The record to save
 * @allow_new:  If we should allow the save if the character doesn't currently already exist
 */
pub fn save_master(record: MasterRecord, allow_new: bool) -> Result<(), Error> {
    with_engine(|engine| engine.save_master(record, allow_new))
}
