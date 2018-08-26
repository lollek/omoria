use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Write, Seek};

use libc;
use serde_json;

use debug;
use player;


#[derive(Serialize, Deserialize)]
struct SaveRecord {
    player_record: player::PlayerRecord,
    /* Player record */

    /* sc__write_inventory */
    /* sc__write_equipment */
    /* sc__write_stats_and_flags */
    /* sc__write_magic */
    /* sc__write_dungeon */
    /* sc__write_identified */
    /* sc__write_monsters */
    /* sc__write_town */
    /* sc__write_version */
    /* sc__write_seeds */

    /*
		sc__write_seeds(f1, &cf_state, out_rec);
		sc__display_status(quick, out_rec);

		sc__write_version(f1, &cf_state, out_rec);
		sc__write_player_record(f1, &cf_state, out_rec);
		sc__write_inventory(f1, &cf_state, out_rec);
		sc__write_equipment(f1, &cf_state, out_rec);
		sc__write_stats_and_flags(f1, &cf_state, out_rec);
		sc__write_magic(f1, &cf_state, out_rec);
		sc__write_dungeon(f1, &cf_state, out_rec);
		sc__write_identified(f1, &cf_state, out_rec);
		sc__write_monsters(f1, &cf_state, out_rec);
		sc__write_town(f1, &cf_state, out_rec);
    */
}

fn savefile_name() -> String {
    format!("save/{}-{}.json", player::name(), player::uid())
}

fn open_savefile() -> Option<File> {
    match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .append(false)
        .open(savefile_name()) {
            Ok(file) => Some(file),
            Err(e) => {
                debug::error(&format!("failed to open master: {}", e));
                None
            },
        }
}

fn read_save(mut f: &File) -> Option<SaveRecord> {
    let mut buffer = String::new();

    if let Err(e) = f.read_to_string(&mut buffer) {
        debug::warn(&format!("Failed to load save @read_to_string, (err: {})", e));
        return None;
    }

    match serde_json::from_str(&buffer) {
        Ok(json) => Some(json),
        Err(e) => {
            debug::warn(&format!("Failed to load save @from_str, (err: {})", e));
            return None;
        },
    }
}

fn write_save(mut f: &File, data: &SaveRecord) -> Option<()> {
    if let Err(e) = f.seek(io::SeekFrom::Start(0)) {
        debug::error(&format!("Failed during seek: {}", e));
        return None;
    }
    if let Err(e) = f.write_all(&serde_json::to_string(data).unwrap().into_bytes()) {
        debug::error(&format!("Failed to write file: {}", e));
        return None;
    }
    Some(())
}

pub fn load_character() -> Option<()> {
    debug::enter("load_character");

    let mut file = open_savefile()?;
    let records = read_save(&file)?;

    debug::leave("load_character");
    Some(())
}

pub fn save_character() -> Option<()> {
    debug::enter("save_character");

    let mut file = open_savefile()?;
    let records = write_save(&file, &SaveRecord{
        player_record: player::player_record(),
    })?;

    debug::leave("save_character");
    Some(())
}

#[no_mangle]
pub extern fn C_save_character() -> libc::uint8_t {
    match save_character() {
        Some(_) => 255,
        None => 0,
    }
}
