use crate::error::Error;
use std::sync::RwLock;

use crate::master::MasterRecord;
use crate::persistence::FileStorageEngine;
use crate::save::SaveRecord;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SaveKey {
    pub name: String,
    pub uid: i64,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum CharacterStorageError {
    NotFound,
    Io(String),
    Codec(String),
}

impl std::fmt::Display for CharacterStorageError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => formatter.write_str("Character save not found"),
            Self::Io(message) | Self::Codec(message) => formatter.write_str(message),
        }
    }
}

impl From<&'static str> for CharacterStorageError {
    fn from(message: &'static str) -> Self {
        Self::Io(message.into())
    }
}

pub(crate) trait PersistenceEngine
where
    Self: Sync + Send,
{
    fn init_masters(&mut self) -> Result<(), Error>;
    fn load_masters(&mut self) -> Result<Vec<MasterRecord>, Error>;
    fn save_master(&mut self, record: MasterRecord, allow_new: bool) -> Result<(), Error>;
    fn load_save(&mut self, name: &str, uid: i64) -> Result<SaveRecord, CharacterStorageError>;
    fn write_save(
        &mut self,
        name: &str,
        uid: i64,
        record: &SaveRecord,
    ) -> Result<(), CharacterStorageError>;
    fn delete_save(&mut self, name: &str, uid: i64) -> Result<(), CharacterStorageError>;
    fn list_saves(&mut self) -> Result<Vec<SaveKey>, CharacterStorageError>;
}

pub(crate) fn load_save_with_engine(
    engine: &mut dyn PersistenceEngine,
    name: &str,
    uid: i64,
) -> Result<SaveRecord, CharacterStorageError> {
    engine.load_save(name, uid)
}

pub(crate) fn write_save_with_engine(
    engine: &mut dyn PersistenceEngine,
    name: &str,
    uid: i64,
    record: &SaveRecord,
) -> Result<(), CharacterStorageError> {
    engine.write_save(name, uid, record)
}

pub(crate) fn delete_save_with_engine(
    engine: &mut dyn PersistenceEngine,
    name: &str,
    uid: i64,
) -> Result<(), CharacterStorageError> {
    engine.delete_save(name, uid)
}

pub(crate) fn list_saves_with_engine(
    engine: &mut dyn PersistenceEngine,
) -> Result<Vec<SaveKey>, CharacterStorageError> {
    engine.list_saves()
}

pub(super) fn upsert_master(
    records: &mut Vec<MasterRecord>,
    record: MasterRecord,
    allow_new: bool,
) -> Result<(), Error> {
    match records
        .iter()
        .position(|existing| existing.uid == record.uid)
    {
        Some(position) => records[position] = record,
        None if allow_new => records.push(record),
        None => return Err("Master file did not contain the player".into()),
    }
    Ok(())
}

pub(crate) fn init_masters_with_engine(engine: &mut dyn PersistenceEngine) -> Result<(), Error> {
    engine.init_masters()
}

pub(crate) fn load_masters_with_engine(
    engine: &mut dyn PersistenceEngine,
) -> Result<Vec<MasterRecord>, Error> {
    engine.load_masters()
}

pub(crate) fn save_master_with_engine(
    engine: &mut dyn PersistenceEngine,
    record: MasterRecord,
    allow_new: bool,
) -> Result<(), Error> {
    engine.save_master(record, allow_new)
}

lazy_static! {
    // It would be nice if we could make a setter function and set the engine from that instead.
    // Since that would remove the two-way dependency
    static ref ENGINE: RwLock<Option<Box<dyn PersistenceEngine>>> = RwLock::new(Some(Box::new(FileStorageEngine)));
}

pub(crate) fn with_engine<T, E: From<&'static str>>(
    fun: impl FnOnce(&mut dyn PersistenceEngine) -> Result<T, E>,
) -> Result<T, E> {
    let mut lock = ENGINE
        .try_write()
        .map_err(|_| E::from("Error in persistence engine"))?;
    let engine = lock
        .as_deref_mut()
        .ok_or_else(|| E::from("No persistence engine assigned!"))?;
    fun(engine)
}

pub(crate) fn load_save(name: &str, uid: i64) -> Result<SaveRecord, CharacterStorageError> {
    with_engine(|engine| load_save_with_engine(engine, name, uid))
}

pub(crate) fn write_save(
    name: &str,
    uid: i64,
    record: &SaveRecord,
) -> Result<(), CharacterStorageError> {
    with_engine(|engine| write_save_with_engine(engine, name, uid, record))
}

pub(crate) fn delete_save(name: &str, uid: i64) -> Result<(), CharacterStorageError> {
    with_engine(|engine| delete_save_with_engine(engine, name, uid))
}

pub(crate) fn list_saves() -> Result<Vec<SaveKey>, CharacterStorageError> {
    with_engine(list_saves_with_engine)
}

/**
 * init_masters() - Init masters for use
 */
pub fn init_masters() -> Result<(), Error> {
    with_engine(init_masters_with_engine)
}

/**
 * load_masters() - Load all characters from the master list
 */
pub fn load_masters() -> Result<Vec<MasterRecord>, Error> {
    with_engine(load_masters_with_engine)
}

/**
 * save_master() - Save a master record
 * @record:     The record to save
 * @allow_new:  If we should allow the save if the character doesn't currently already exist
 */
pub fn save_master(record: MasterRecord, allow_new: bool) -> Result<(), Error> {
    with_engine(|engine| save_master_with_engine(engine, record, allow_new))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::memory::{engine_with_records, record, InMemoryEngine};

    #[test]
    fn character_engine_access_errors_are_io_errors() {
        for message in [
            "Error in persistence engine",
            "No persistence engine assigned!",
        ] {
            assert_eq!(
                CharacterStorageError::from(message),
                CharacterStorageError::Io(message.into())
            );
        }
    }

    #[test]
    fn character_storage_errors_display_their_diagnostics() {
        assert_eq!(
            CharacterStorageError::NotFound.to_string(),
            "Character save not found"
        );
        assert_eq!(
            CharacterStorageError::Io("denied".into()).to_string(),
            "denied"
        );
        assert_eq!(
            CharacterStorageError::Codec("invalid JSON".into()).to_string(),
            "invalid JSON"
        );
    }

    #[test]
    fn upsert_replaces_existing_uid_without_reordering_other_records() {
        let mut records = vec![record(1, 10), record(2, 20)];
        upsert_master(&mut records, record(1, 30), false).unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!((records[0].uid, records[0].points), (1, 30));
        assert_eq!((records[1].uid, records[1].points), (2, 20));
    }

    #[test]
    fn upsert_appends_new_uid_when_allowed() {
        let mut records = vec![record(1, 10)];
        upsert_master(&mut records, record(2, 20), true).unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!((records[1].uid, records[1].points), (2, 20));
    }

    #[test]
    fn upsert_rejects_new_uid_without_changing_records() {
        let mut records = vec![record(1, 10)];
        let before = crate::persistence::json::encode(&records).unwrap();
        let err = upsert_master(&mut records, record(2, 20), false).unwrap_err();
        assert_eq!(err.to_string(), "Master file did not contain the player");
        assert_eq!(crate::persistence::json::encode(&records).unwrap(), before);
    }

    #[test]
    fn injected_init_creates_empty_masters() {
        let mut engine = InMemoryEngine::default();
        init_masters_with_engine(&mut engine).unwrap();
        assert_eq!(engine.init_calls, 1);
        assert!(load_masters_with_engine(&mut engine).unwrap().is_empty());
        assert_eq!(engine.load_calls, 1);
    }

    #[test]
    fn injected_init_preserves_existing_data() {
        let mut engine = engine_with_records(&[record(1, 10)]);
        let before = engine.masters.clone();
        init_masters_with_engine(&mut engine).unwrap();
        assert_eq!(engine.masters, before);
        assert_eq!(engine.init_calls, 1);
    }

    #[test]
    fn injected_init_propagates_storage_errors() {
        let mut engine = InMemoryEngine {
            fail_init: true,
            ..Default::default()
        };
        assert_eq!(
            init_masters_with_engine(&mut engine)
                .unwrap_err()
                .to_string(),
            "injected init failure"
        );
        assert_eq!(engine.init_calls, 1);
    }

    #[test]
    fn injected_save_round_trips_json_and_forwards_allow_new() {
        let mut engine = engine_with_records(&[]);
        save_master_with_engine(&mut engine, record(1, 10), true).unwrap();
        save_master_with_engine(&mut engine, record(1, 30), false).unwrap();
        let records = load_masters_with_engine(&mut engine).unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!((records[0].uid, records[0].points), (1, 30));
        assert_eq!(engine.save_calls, vec![(1, true), (1, false)]);
        assert_eq!(engine.load_calls, 3);
    }

    #[test]
    fn injected_save_rejects_new_uid_when_not_allowed() {
        let mut engine = engine_with_records(&[]);
        let before = engine.masters.clone();
        let err = save_master_with_engine(&mut engine, record(1, 10), false).unwrap_err();
        assert_eq!(err.to_string(), "Master file did not contain the player");
        assert_eq!(engine.masters, before);
        assert_eq!(engine.save_calls, vec![(1, false)]);
    }

    #[test]
    fn injected_save_propagates_load_errors() {
        let mut engine = engine_with_records(&[]);
        engine.fail_load = true;
        let err = save_master_with_engine(&mut engine, record(1, 10), true).unwrap_err();
        assert_eq!(err.to_string(), "injected load failure");
        assert_eq!(engine.save_calls, vec![(1, true)]);
        assert_eq!(engine.load_calls, 1);
    }

    #[test]
    fn injected_save_propagates_write_errors_without_changing_data() {
        let mut engine = engine_with_records(&[record(1, 10)]);
        let before = engine.masters.clone();
        engine.fail_save = true;
        let err = save_master_with_engine(&mut engine, record(1, 30), false).unwrap_err();
        assert_eq!(err.to_string(), "injected save failure");
        assert_eq!(engine.masters, before);
        assert_eq!(engine.save_calls, vec![(1, false)]);
    }

    #[test]
    fn injected_load_propagates_storage_errors() {
        let mut engine = InMemoryEngine {
            fail_load: true,
            ..Default::default()
        };
        let err = load_masters_with_engine(&mut engine).err().unwrap();
        assert_eq!(err.to_string(), "injected load failure");
        assert_eq!(engine.load_calls, 1);
    }

    #[test]
    fn injected_load_rejects_missing_data() {
        let mut engine = InMemoryEngine::default();
        let err = load_masters_with_engine(&mut engine).err().unwrap();
        assert_eq!(err.to_string(), "masters missing");
    }

    #[test]
    fn injected_load_rejects_corrupt_json() {
        let mut engine = InMemoryEngine {
            masters: Some("not json".into()),
            ..Default::default()
        };
        let err = load_masters_with_engine(&mut engine).err().unwrap();
        assert!(
            err.to_string().contains("expected"),
            "unexpected error: {}",
            err
        );
    }
}
