use std::fs;
use std::io::{Read, Seek, SeekFrom, Write};

use serde_json;

use crate::constants;
use crate::error::Error;
use crate::master::MasterRecord;
use crate::persistence;

pub struct FileStorageEngine;

// TODO: Consider flocking (https://stackoverflow.com/a/32743299)
// Will probably never to this since I'm the only intended user for this program
impl persistence::PersistenceEngine for FileStorageEngine {
    fn init_masters(&mut self) -> Result<(), Error> {
        let mut file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(master_file_path())
            .map_err(|e| Error::from(format!("Failed to create masters file: {}", e).as_str()))?;
        let file_bytes = file.seek(SeekFrom::End(0))
            .map_err(|e| Error::from(format!("Failed to seek in masters file: {}", e.to_string())))?;

        // Create empty masters file
        if file_bytes == 0 {
            let records = Vec::<MasterRecord>::new();
            return file.write_all(&serde_json::to_string(&records).unwrap().into_bytes())
                .map_err(|e| Error::from(format!("Failed to write file: {}", e).as_str()))
        }
        Ok(())
    }

    fn load_masters(&mut self) -> Result<Vec<MasterRecord>, Error> {
        let mut file = fs::OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .truncate(false)
            .open(master_file_path())
            .map_err(|e| Error::from(format!("failed to open master: {}", e).as_str()))?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).map_err(|e| {
            Error::from(format!("Either master was empty, or corrupt: {}", e).as_str())
        })?;
        serde_json::from_str(&buffer).map_err(|e| {
            Error::from(format!("Either master was empty, or corrupt: {}", e).as_str())
        })
    }

    fn save_master(&mut self, record: MasterRecord, allow_new: bool) -> Result<(), Error> {
        let mut records = self.load_masters()?;

        match records.iter().position(|ref i| i.uid == record.uid) {
            Some(pos) => {
                records[pos] = record;
            }
            None => {
                if !allow_new {
                    return Err(Error::from("Master file did not contain the player"));
                }

                records.push(record);
            }
        }

        let mut file = fs::OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .truncate(true)
            .open(master_file_path())
            .map_err(|e| Error::from(format!("failed to open master: {}", e).as_str()))?;
        file.write_all(&serde_json::to_string(&records).unwrap().into_bytes())
            .map_err(|e| Error::from(format!("Failed to write file: {}", e).as_str()))
    }
}

fn master_file_path() -> String {
    format!("{}/moria_master.json", constants::DATA_FOLDER)
}
