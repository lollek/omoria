use crate::error::Error;
use crate::master::MasterRecord;
use std::collections::BTreeMap;

use super::main::upsert_master;
use super::{json, CharacterStorageError, PersistenceEngine, SaveKey};
use crate::save::SaveRecord;

#[derive(Default)]
pub(crate) struct InMemoryEngine {
    pub masters: Option<String>,
    pub init_calls: usize,
    pub load_calls: usize,
    pub save_calls: Vec<(i64, bool)>,
    pub fail_init: bool,
    pub fail_load: bool,
    pub fail_save: bool,
    pub saves: BTreeMap<SaveKey, String>,
    pub character_calls: Vec<(&'static str, Option<SaveKey>)>,
    pub fail_read: bool,
    pub fail_write: bool,
    pub fail_delete: bool,
    pub fail_list: bool,
}

impl PersistenceEngine for InMemoryEngine {
    fn init_masters(&mut self) -> Result<(), Error> {
        self.init_calls += 1;
        if self.fail_init {
            return Err("injected init failure".into());
        }
        if self.masters.as_ref().is_none_or(|json| json.is_empty()) {
            self.masters = Some(json::encode(&Vec::<MasterRecord>::new())?);
        }
        Ok(())
    }

    fn load_masters(&mut self) -> Result<Vec<MasterRecord>, Error> {
        self.load_calls += 1;
        if self.fail_load {
            return Err("injected load failure".into());
        }
        json::decode(self.masters.as_deref().ok_or("masters missing")?)
    }

    fn save_master(&mut self, record: MasterRecord, allow_new: bool) -> Result<(), Error> {
        self.save_calls.push((record.uid, allow_new));
        let mut records = self.load_masters()?;
        upsert_master(&mut records, record, allow_new)?;
        let encoded = json::encode(&records)?;
        if self.fail_save {
            return Err("injected save failure".into());
        }
        self.masters = Some(encoded);
        Ok(())
    }

    fn load_save(&mut self, name: &str, uid: i64) -> Result<SaveRecord, CharacterStorageError> {
        let key = SaveKey {
            name: name.into(),
            uid,
        };
        self.character_calls.push(("read", Some(key.clone())));
        if self.fail_read {
            return Err(CharacterStorageError::Io("injected read failure".into()));
        }
        let encoded = self
            .saves
            .get(&key)
            .ok_or(CharacterStorageError::NotFound)?;
        json::decode(encoded).map_err(|err| CharacterStorageError::Codec(err.to_string()))
    }

    fn write_save(
        &mut self,
        name: &str,
        uid: i64,
        record: &SaveRecord,
    ) -> Result<(), CharacterStorageError> {
        let key = SaveKey {
            name: name.into(),
            uid,
        };
        self.character_calls.push(("write", Some(key.clone())));
        let encoded =
            json::encode(record).map_err(|err| CharacterStorageError::Codec(err.to_string()))?;
        if self.fail_write {
            return Err(CharacterStorageError::Io("injected write failure".into()));
        }
        self.saves.insert(key, encoded);
        Ok(())
    }

    fn delete_save(&mut self, name: &str, uid: i64) -> Result<(), CharacterStorageError> {
        let key = SaveKey {
            name: name.into(),
            uid,
        };
        self.character_calls.push(("delete", Some(key.clone())));
        if self.fail_delete {
            return Err(CharacterStorageError::Io("injected delete failure".into()));
        }
        self.saves
            .remove(&key)
            .ok_or(CharacterStorageError::NotFound)?;
        Ok(())
    }

    fn list_saves(&mut self) -> Result<Vec<SaveKey>, CharacterStorageError> {
        self.character_calls.push(("list", None));
        if self.fail_list {
            return Err(CharacterStorageError::Io("injected list failure".into()));
        }
        Ok(self.saves.keys().cloned().collect())
    }
}

#[cfg(test)]
pub(crate) fn record(uid: i64, points: i64) -> MasterRecord {
    MasterRecord {
        uid,
        user_name: "-".into(),
        character_name: "Fixture".into(),
        points,
        alive: true,
        level: 2,
        race: "Elf".into(),
        class: "Wizard".into(),
    }
}

#[cfg(test)]
pub(crate) fn engine_with_records(records: &[MasterRecord]) -> InMemoryEngine {
    InMemoryEngine {
        masters: Some(json::encode(records).unwrap()),
        ..Default::default()
    }
}

#[cfg(feature = "save-test-support")]
#[no_mangle]
pub extern "C" fn C_save_test_memory_begin() -> bool {
    let player = crate::player::record();
    let master = MasterRecord {
        uid: player.uid,
        user_name: "-".into(),
        character_name: player.name,
        points: crate::player::calc_total_points(),
        alive: !crate::player::is_dead(),
        level: crate::player::level(),
        race: crate::data::race::name(&crate::player::race()).into(),
        class: crate::data::class::name(&crate::player::class()).into(),
    };
    let engine = InMemoryEngine {
        masters: Some(json::encode(&vec![master]).expect("test master should serialize")),
        ..Default::default()
    };
    crate::persistence::replace_engine_for_test(Box::new(engine)).is_ok()
}

#[cfg(feature = "save-test-support")]
#[no_mangle]
pub extern "C" fn C_save_test_memory_save_character() -> bool {
    crate::save::save_character_for_test()
}

#[cfg(feature = "save-test-support")]
#[no_mangle]
pub extern "C" fn C_save_test_memory_load_character() -> bool {
    let player = crate::player::record();
    let expected = match crate::persistence::load_save(&player.name, player.uid) {
        Ok(record) => record.player,
        Err(_) => return false,
    };
    if !crate::save::load_character_for_test(&player.name, player.uid) {
        return false;
    }
    match (
        serde_json::to_value(expected),
        serde_json::to_value(crate::player::record()),
    ) {
        (Ok(expected), Ok(actual)) => actual == expected,
        _ => false,
    }
}

#[cfg(feature = "save-test-support")]
#[no_mangle]
pub extern "C" fn C_save_test_memory_end() -> bool {
    crate::persistence::restore_file_engine_for_test().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::{
        delete_save_with_engine, list_saves_with_engine, load_save_with_engine,
        write_save_with_engine, CharacterStorageError,
    };
    use crate::save::test_support::{fixture_record, normalize, with_large_stack, FIXTURE};

    fn key() -> SaveKey {
        SaveKey {
            name: "Fixture-With-Hyphens".into(),
            uid: -42,
        }
    }

    fn fixture_engine() -> InMemoryEngine {
        InMemoryEngine {
            saves: BTreeMap::from([(key(), FIXTURE.into())]),
            ..Default::default()
        }
    }

    #[test]
    fn character_fixture_round_trip_uses_injected_operations() {
        with_large_stack(|| {
            let mut engine = InMemoryEngine::default();
            let key = key();
            write_save_with_engine(&mut engine, &key.name, key.uid, &fixture_record()).unwrap();
            let loaded = load_save_with_engine(&mut engine, &key.name, key.uid).unwrap();
            let actual = normalize(serde_json::from_str(&json::encode(&loaded).unwrap()).unwrap());
            let expected = normalize(serde_json::from_str(FIXTURE).unwrap());
            for (section, value) in expected.as_object().unwrap() {
                assert!(
                    value == &actual[section],
                    "section `{}` changed after storage round trip",
                    section
                );
            }
            assert_eq!(
                list_saves_with_engine(&mut engine).unwrap(),
                vec![key.clone()]
            );
            delete_save_with_engine(&mut engine, &key.name, key.uid).unwrap();
            assert!(engine.saves.is_empty());
            assert_eq!(
                engine.character_calls,
                vec![
                    ("write", Some(key.clone())),
                    ("read", Some(key.clone())),
                    ("list", None),
                    ("delete", Some(key))
                ]
            );
        });
    }

    #[test]
    fn character_missing_read_and_delete_return_not_found() {
        with_large_stack(|| {
            let mut engine = InMemoryEngine::default();
            assert_eq!(
                load_save_with_engine(&mut engine, "Missing", 1).unwrap_err(),
                CharacterStorageError::NotFound
            );
            assert_eq!(
                delete_save_with_engine(&mut engine, "Missing", 1).unwrap_err(),
                CharacterStorageError::NotFound
            );
            let missing = SaveKey {
                name: "Missing".into(),
                uid: 1,
            };
            assert_eq!(
                engine.character_calls,
                vec![("read", Some(missing.clone())), ("delete", Some(missing))]
            );
        });
    }

    #[test]
    fn character_read_failure_propagates_and_logs_operation() {
        with_large_stack(|| {
            let mut engine = fixture_engine();
            engine.fail_read = true;
            assert_eq!(
                load_save_with_engine(&mut engine, &key().name, key().uid).unwrap_err(),
                CharacterStorageError::Io("injected read failure".into())
            );
            assert_eq!(engine.character_calls, vec![("read", Some(key()))]);
        });
    }

    #[test]
    fn character_failed_write_preserves_existing_data() {
        with_large_stack(|| {
            let mut engine = fixture_engine();
            let before = engine.saves.clone();
            engine.fail_write = true;
            let mut replacement = fixture_record();
            replacement.player.name = "Replacement".into();
            assert_eq!(
                write_save_with_engine(&mut engine, &key().name, key().uid, &replacement)
                    .unwrap_err(),
                CharacterStorageError::Io("injected write failure".into())
            );
            assert_eq!(engine.saves, before);
            assert_eq!(engine.character_calls, vec![("write", Some(key()))]);
        });
    }

    #[test]
    fn character_delete_failure_preserves_existing_data() {
        let mut engine = fixture_engine();
        let before = engine.saves.clone();
        engine.fail_delete = true;
        assert_eq!(
            delete_save_with_engine(&mut engine, &key().name, key().uid).unwrap_err(),
            CharacterStorageError::Io("injected delete failure".into())
        );
        assert_eq!(engine.saves, before);
        assert_eq!(engine.character_calls, vec![("delete", Some(key()))]);
    }

    #[test]
    fn character_list_failure_propagates_and_logs_operation() {
        let mut engine = fixture_engine();
        engine.fail_list = true;
        assert_eq!(
            list_saves_with_engine(&mut engine).unwrap_err(),
            CharacterStorageError::Io("injected list failure".into())
        );
        assert_eq!(engine.character_calls, vec![("list", None)]);
    }

    #[test]
    fn character_corrupt_json_returns_codec_error() {
        with_large_stack(|| {
            let mut engine = fixture_engine();
            engine.saves.insert(key(), "not json".into());
            let err = load_save_with_engine(&mut engine, &key().name, key().uid).unwrap_err();
            assert!(
                matches!(err, CharacterStorageError::Codec(_)),
                "unexpected error: {}",
                err
            );
            assert_eq!(engine.character_calls, vec![("read", Some(key()))]);
        });
    }
}
