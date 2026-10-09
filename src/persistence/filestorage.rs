use std::fs;
use std::io::{Read, Seek, SeekFrom, Write};

use crate::constants;
use crate::debug;
use crate::error::Error;
use crate::master::MasterRecord;
use crate::persistence;
use crate::persistence::{json, main::upsert_master};
use crate::persistence::{CharacterStorageError, SaveKey};
use crate::save::SaveRecord;

fn parse_save_filename(filename: &str) -> Option<SaveKey> {
    let stem = filename.strip_suffix(".json")?;
    let (name, uid) = stem.rsplit_once('-')?;
    // Preserve legacy names ending in '-' rather than reinterpret them as negative UIDs.
    Some(SaveKey {
        name: name.into(),
        uid: uid.parse().ok()?,
    })
}

fn collect_save_keys(
    entries: impl IntoIterator<Item = std::io::Result<std::ffi::OsString>>,
    mut log: impl FnMut(String),
) -> Result<Vec<SaveKey>, CharacterStorageError> {
    let mut keys = Vec::new();
    for entry in entries {
        let filename = entry.map_err(character_io_error)?;
        match filename.to_str().and_then(parse_save_filename) {
            Some(key) => keys.push(key),
            None => log(format!("Ignoring malformed save filename: {:?}", filename)),
        }
    }
    Ok(keys)
}

fn character_io_error(error: std::io::Error) -> CharacterStorageError {
    if error.kind() == std::io::ErrorKind::NotFound {
        CharacterStorageError::NotFound
    } else {
        CharacterStorageError::Io(error.to_string())
    }
}

pub struct FileStorageEngine;

// TODO: Consider flocking (https://stackoverflow.com/a/32743299)
// Will probably never to this since I'm the only intended user for this program
impl persistence::PersistenceEngine for FileStorageEngine {
    fn init_masters(&mut self) -> Result<(), Error> {
        let empty_masters = json::encode(&Vec::<MasterRecord>::new())?;
        let mut file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(master_file_path())
            .map_err(|e| Error::from(format!("Failed to create masters file: {}", e).as_str()))?;
        let file_bytes = file
            .seek(SeekFrom::End(0))
            .map_err(|e| Error::from(format!("Failed to seek in masters file: {}", e)))?;

        // Create empty masters file
        if file_bytes == 0 {
            return file
                .write_all(empty_masters.as_bytes())
                .map_err(|e| Error::from(format!("Failed to write file: {}", e).as_str()));
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
        json::decode(&buffer).map_err(|e| {
            Error::from(format!("Either master was empty, or corrupt: {}", e).as_str())
        })
    }

    fn save_master(&mut self, record: MasterRecord, allow_new: bool) -> Result<(), Error> {
        let mut records = self.load_masters()?;
        upsert_master(&mut records, record, allow_new)?;
        let encoded = json::encode(&records)?;

        let mut file = fs::OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .truncate(true)
            .open(master_file_path())
            .map_err(|e| Error::from(format!("failed to open master: {}", e).as_str()))?;
        file.write_all(encoded.as_bytes())
            .map_err(|e| Error::from(format!("Failed to write file: {}", e).as_str()))
    }

    fn load_save(&mut self, name: &str, uid: i64) -> Result<SaveRecord, CharacterStorageError> {
        let encoded = fs::read_to_string(save_file_path(name, uid)).map_err(character_io_error)?;
        json::decode(&encoded).map_err(|err| CharacterStorageError::Codec(err.to_string()))
    }

    fn write_save(
        &mut self,
        name: &str,
        uid: i64,
        record: &SaveRecord,
    ) -> Result<(), CharacterStorageError> {
        let (encoded, file) = encode_before_open(record, || {
            fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(save_file_path(name, uid))
        })
        .map_err(|err| CharacterStorageError::Codec(err.to_string()))?;
        let file = file.map_err(character_io_error)?;
        write_save_bytes(file, &encoded).map_err(character_io_error)
    }

    fn delete_save(&mut self, name: &str, uid: i64) -> Result<(), CharacterStorageError> {
        fs::remove_file(save_file_path(name, uid)).map_err(character_io_error)
    }

    fn list_saves(&mut self) -> Result<Vec<SaveKey>, CharacterStorageError> {
        let entries = fs::read_dir(constants::SAVE_FOLDER).map_err(character_io_error)?;
        collect_save_keys(
            entries.map(|entry| entry.map(|entry| entry.file_name())),
            debug::error,
        )
    }
}

fn save_file_path(name: &str, uid: i64) -> String {
    format!("{}/{}-{}.json", constants::SAVE_FOLDER, name, uid)
}

fn write_save_bytes(mut writer: impl Write, json: &str) -> std::io::Result<()> {
    writer.write_all(json.as_bytes())
}

fn encode_before_open<T: serde::Serialize + ?Sized, Output>(
    record: &T,
    open: impl FnOnce() -> Output,
) -> Result<(String, Output), Error> {
    let encoded = json::encode(record)?;
    Ok((encoded, open()))
}

fn master_file_path() -> String {
    format!("{}/moria_master.json", constants::DATA_FOLDER)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::save::test_support::FIXTURE;
    use std::ffi::OsString;
    use std::io;

    #[test]
    fn failed_serialization_does_not_open_save_storage() {
        struct FailingRecord;

        impl serde::Serialize for FailingRecord {
            fn serialize<S: serde::Serializer>(&self, _serializer: S) -> Result<S::Ok, S::Error> {
                Err(serde::ser::Error::custom("injected serialization failure"))
            }
        }

        let mut opened = false;
        let result = encode_before_open(&FailingRecord, || opened = true);
        assert!(
            !opened,
            "serialization failure must not open or truncate storage"
        );
        assert_eq!(
            result.unwrap_err().to_string(),
            "injected serialization failure"
        );
    }

    #[test]
    fn successful_serialization_opens_storage_once() {
        let mut opens = 0;
        let (encoded, writer) = encode_before_open(&vec![1, 2, 3], || {
            opens += 1;
            Vec::<u8>::new()
        })
        .unwrap();
        assert_eq!(opens, 1);
        assert_eq!(encoded, "[1,2,3]");
        assert!(writer.is_empty());
    }

    #[test]
    fn storage_open_failure_is_preserved() {
        let (encoded, writer) = encode_before_open(&vec![1], || None::<Vec<u8>>).unwrap();
        assert_eq!(encoded, "[1]");
        assert!(writer.is_none());
    }

    #[test]
    fn save_writer_writes_all_encoded_bytes() {
        struct ShortWriter(Vec<u8>);

        impl Write for ShortWriter {
            fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
                let count = bytes.len().min(3);
                self.0.extend_from_slice(&bytes[..count]);
                Ok(count)
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        let mut writer = ShortWriter(Vec::new());
        write_save_bytes(&mut writer, FIXTURE).unwrap();
        assert_eq!(writer.0, FIXTURE.as_bytes());
    }

    #[test]
    fn save_writer_propagates_write_failure() {
        struct FailingWriter;

        impl Write for FailingWriter {
            fn write(&mut self, _bytes: &[u8]) -> io::Result<usize> {
                Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "injected write failure",
                ))
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        let err = write_save_bytes(FailingWriter, FIXTURE).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::PermissionDenied);
        assert_eq!(err.to_string(), "injected write failure");
    }

    #[test]
    fn character_filename_preserves_legacy_trailing_hyphen_name() {
        assert_eq!(
            parse_save_filename("Fixture--42.json"),
            Some(SaveKey {
                name: "Fixture-".into(),
                uid: 42,
            })
        );
    }

    #[test]
    fn character_filename_preserves_hyphenated_names_and_uid_boundaries() {
        for (filename, name, uid) in [
            ("Fixture-With-Hyphens-42.json", "Fixture-With-Hyphens", 42),
            ("Fixture-9223372036854775807.json", "Fixture", i64::MAX),
            ("-0.json", "", 0),
            ("Fixture---42.json", "Fixture--", 42),
        ] {
            assert_eq!(
                parse_save_filename(filename),
                Some(SaveKey {
                    name: name.into(),
                    uid
                }),
                "filename: {}",
                filename
            );
            let path = save_file_path(name, uid);
            assert_eq!(
                std::path::Path::new(&path).file_name().unwrap().to_str(),
                Some(filename)
            );
        }
    }

    #[test]
    fn character_filename_rejects_bad_uid_and_inexact_suffix() {
        for filename in [
            "Fixture.json",
            "Fixture-nope.json",
            "Fixture-1.json.bak",
            "Fixture-1.json.json",
            "Fixture-1json",
            "Fixture-9223372036854775808.json",
            "Fixture--9223372036854775809.json",
        ] {
            assert_eq!(
                parse_save_filename(filename),
                None,
                "filename: {}",
                filename
            );
        }
    }

    #[test]
    fn character_list_ignores_and_logs_malformed_names() {
        let entries = [
            "Fixture-1.json",
            "bad.json",
            "Fixture-2.json.bak",
            "Other--3.json",
        ]
        .map(|name| Ok(OsString::from(name)));
        let mut messages = Vec::new();
        let keys = collect_save_keys(entries, |message| messages.push(message)).unwrap();
        assert_eq!(
            keys,
            vec![
                SaveKey {
                    name: "Fixture".into(),
                    uid: 1
                },
                SaveKey {
                    name: "Other-".into(),
                    uid: 3
                }
            ]
        );
        assert_eq!(messages.len(), 2);
        assert!(messages[0].contains("bad.json"));
        assert!(messages[1].contains("Fixture-2.json.bak"));
    }

    #[test]
    fn character_list_entry_errors_propagate() {
        let entries = [Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "injected entry failure",
        ))];
        assert_eq!(
            collect_save_keys(entries, |_| {}).unwrap_err(),
            CharacterStorageError::Io("injected entry failure".into())
        );
    }

    #[cfg(unix)]
    #[test]
    fn character_list_non_utf8_names_are_logged_and_ignored() {
        use std::os::unix::ffi::OsStringExt;
        let entries = [Ok(OsString::from_vec(vec![0xff]))];
        let mut messages = Vec::new();
        assert!(collect_save_keys(entries, |message| messages.push(message))
            .unwrap()
            .is_empty());
        assert_eq!(messages.len(), 1);
    }

    #[test]
    fn character_io_errors_distinguish_missing_from_other_failures() {
        assert_eq!(
            character_io_error(io::Error::from(io::ErrorKind::NotFound)),
            CharacterStorageError::NotFound
        );
        assert_eq!(
            character_io_error(io::Error::new(io::ErrorKind::PermissionDenied, "denied")),
            CharacterStorageError::Io("denied".into())
        );
    }
}
