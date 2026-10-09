use crate::error::Error;
use crate::master::MasterRecord;

use super::main::upsert_master;
use super::{json, PersistenceEngine};

#[derive(Default)]
pub(crate) struct InMemoryEngine {
    pub masters: Option<String>,
    pub init_calls: usize,
    pub load_calls: usize,
    pub save_calls: Vec<(i64, bool)>,
    pub fail_init: bool,
    pub fail_load: bool,
    pub fail_save: bool,
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
}

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

pub(crate) fn engine_with_records(records: &[MasterRecord]) -> InMemoryEngine {
    InMemoryEngine {
        masters: Some(json::encode(records).unwrap()),
        ..Default::default()
    }
}
