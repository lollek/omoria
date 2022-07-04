use std::fs;
use std::io::{Read, Write};

use serde_json;

use crate::constants;
use crate::error::Error;
use crate::master::MasterRecord;
use crate::persistence;

pub struct FileStorageEngine;

// TODO: Consider flocking (https://stackoverflow.com/a/32743299)
// Will probably never to this since I'm the only intended user for this program
impl persistence::PersistenceEngine for FileStorageEngine {
    fn load_masters(&mut self) -> Result<Vec<MasterRecord>, Error> {
        let mut file = open_master(false)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)
            .map_err(|e| Error::from(format!("Either master was empty, or corrupt: {}", e).as_str()))?;
        serde_json::from_str(&buffer)
            .map_err(|e| Error::from(format!("Either master was empty, or corrupt: {}", e).as_str()))
    }

    fn save_master(&mut self, record: MasterRecord, allow_new: bool) -> Result<(), Error> {

        let mut records = self.load_masters()?;

        match records.iter().position(|ref i| i.uid == record.uid) {
            Some(pos) => {
                records[pos] = record;
            },
            None => {
                if !allow_new {
                    return Err(Error::from("Master file did not contain the player"));
                }

                records.push(record);
            }
        }

        let mut file = open_master(true)?;
        file.write_all(&serde_json::to_string(&records).unwrap().into_bytes())
            .map_err(|e| Error::from(format!("Failed to write file: {}", e).as_str()))
    }
}

fn open_master(to_write: bool) -> Result<fs::File, Error> {
    fs::OpenOptions::new()
        .read(!to_write)
        .write(to_write)
        .create(to_write)
        .truncate(to_write)
        .append(false)
        .open(format!("{}/moria_master.json", constants::DATA_FOLDER))
        .map_err(|e| Error::from(format!("failed to open master: {}", e).as_str()))
}

