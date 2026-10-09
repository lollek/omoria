use crate::data;
use crate::debug;
use crate::error::Error;
use crate::persistence;
use crate::player;
use crate::rng;

#[derive(Serialize, Deserialize)]
pub struct MasterRecord {
    pub uid: i64,
    pub user_name: String,
    pub character_name: String,
    pub points: i64,
    pub alive: bool,
    pub level: u8,
    pub race: String,
    pub class: String,
}

pub fn init_master() -> Result<(), Error> {
    persistence::init_masters()
}

pub fn read_master() -> Result<Vec<MasterRecord>, Error> {
    persistence::load_masters()
}

pub(crate) fn read_master_with_engine(
    engine: &mut dyn persistence::PersistenceEngine,
) -> Result<Vec<MasterRecord>, Error> {
    engine.load_masters()
}

pub(crate) fn character_exists_with_engine(
    engine: &mut dyn persistence::PersistenceEngine,
    uid: i64,
) -> Result<bool, Error> {
    Ok(read_master_with_engine(engine)?
        .iter()
        .any(|record| record.uid == uid))
}

pub fn update_character(uid: i64) -> Result<(), Error> {
    persistence::save_master(
        MasterRecord {
            uid,
            user_name: "-".to_string(),
            character_name: player::name(),
            points: player::calc_total_points(),
            alive: !player::is_dead(),
            level: player::level(),
            race: data::race::name(&player::race()).to_string(),
            class: data::class::name(&player::class()).to_string(),
        },
        false,
    )?;

    Ok(())
}

pub fn add_character() -> Result<i64, Error> {
    let mut new_uid;
    loop {
        new_uid = rng::randint(<i64>::MAX - 1);
        if new_uid != 0 {
            break;
        }
    }

    persistence::save_master(
        MasterRecord {
            uid: new_uid,
            user_name: "-".to_string(),
            character_name: player::name(),
            points: player::calc_total_points(),
            alive: true,
            level: player::level(),
            race: data::race::name(&player::race()).to_string(),
            class: data::class::name(&player::class()).to_string(),
        },
        true,
    )?;

    Ok(new_uid)
}

pub fn character_exists(uid: i64) -> bool {
    match persistence::with_engine(|engine| character_exists_with_engine(engine, uid)) {
        Ok(true) => true,
        Ok(false) => {
            debug::warn("Master file did not contain the player");
            false
        }
        Err(err) => {
            debug::error(format!("{:?}", err));
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::memory::{engine_with_records, record, InMemoryEngine};

    #[test]
    fn injected_reader_returns_master_records() {
        let mut engine = engine_with_records(&[record(1, 10), record(2, 20)]);
        let records = read_master_with_engine(&mut engine).unwrap();
        assert_eq!(records.len(), 2);
        assert_eq!((records[0].uid, records[0].points), (1, 10));
        assert_eq!((records[1].uid, records[1].points), (2, 20));
        assert_eq!(engine.load_calls, 1);
    }

    #[test]
    fn injected_reader_propagates_storage_errors() {
        let mut engine = InMemoryEngine {
            fail_load: true,
            ..Default::default()
        };
        let err = read_master_with_engine(&mut engine).err().unwrap();
        assert_eq!(err.to_string(), "injected load failure");
        assert_eq!(engine.load_calls, 1);
    }

    #[test]
    fn injected_query_finds_matching_uid() {
        let mut engine = engine_with_records(&[record(1, 10), record(2, 20)]);
        assert!(character_exists_with_engine(&mut engine, 2).unwrap());
        assert_eq!(engine.load_calls, 1);
    }

    #[test]
    fn injected_query_rejects_missing_uid() {
        let mut engine = engine_with_records(&[record(1, 10)]);
        assert!(!character_exists_with_engine(&mut engine, 2).unwrap());
        assert_eq!(engine.load_calls, 1);
    }

    #[test]
    fn injected_query_rejects_empty_masters() {
        let mut engine = engine_with_records(&[]);
        assert!(!character_exists_with_engine(&mut engine, 1).unwrap());
    }

    #[test]
    fn injected_query_propagates_storage_errors() {
        let mut engine = InMemoryEngine {
            fail_load: true,
            ..Default::default()
        };
        let err = character_exists_with_engine(&mut engine, 1).unwrap_err();
        assert_eq!(err.to_string(), "injected load failure");
        assert_eq!(engine.load_calls, 1);
    }

    #[test]
    fn injected_reader_rejects_corrupt_json() {
        let mut engine = InMemoryEngine {
            masters: Some("not json".into()),
            ..Default::default()
        };
        let err = read_master_with_engine(&mut engine).err().unwrap();
        assert!(
            err.to_string().contains("expected"),
            "unexpected error: {}",
            err
        );
    }

    #[test]
    fn injected_engines_do_not_share_state() {
        let mut first = engine_with_records(&[record(1, 10)]);
        let mut second = engine_with_records(&[]);
        assert!(character_exists_with_engine(&mut first, 1).unwrap());
        assert!(!character_exists_with_engine(&mut second, 1).unwrap());
        assert_eq!(first.load_calls, 1);
        assert_eq!(second.load_calls, 1);
    }
}
