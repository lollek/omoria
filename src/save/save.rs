use std::fs::{File, OpenOptions};
use std::fs;
use std::io;
use std::io::{Read, Write, Seek};

use libc;
use serde_json;

use debug;
use player;
use save;

use save::types::*;


#[derive(Serialize, Deserialize)]
struct SaveRecord {
    player: player::PlayerRecord,
    inventory: Vec<TreasureRec>,
    equipment: Vec<Item>,
    town: TownRecord,
    dungeon: DungeonRecord,
    identified: IdentifiedRecord,
    monsters: MonsterRecord,
}

fn savefile_name() -> String {
    format!("save/{}-{}.json", player::name(), player::uid())
}

fn open_savefile(to_write: bool) -> Option<File> {
    match OpenOptions::new()
        .read(!to_write)
        .write(to_write)
        .create(to_write)
        .truncate(to_write)
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

    let file = open_savefile(false)?;
    let records = read_save(&file)?;
    player::set_record(records.player);
    save::inventory::set_record(records.inventory);
    save::equipment::set_record(records.equipment);
    save::town::set_record(records.town);
    save::dungeon::set_record(records.dungeon);
    save::identified::set_record(records.identified);
    save::monsters::set_record(records.monsters);

    debug::leave("load_character");
    Some(())
}

pub fn save_character() -> Option<()> {
    debug::enter("save_character");

    player::increase_save_counter();

    let file = open_savefile(true)?;
    write_save(&file, &SaveRecord{
        player: player::record(),
        inventory: save::inventory::record(),
        equipment: save::equipment::record(),
        town: save::town::record(),
        dungeon: save::dungeon::record(),
        identified: save::identified::record(),
        monsters: save::monsters::record(),
    })?;

    debug::leave("save_character");
    Some(())
}

pub fn delete_character() -> Option<()> {
    match fs::remove_file(savefile_name()) {
        Ok(_) => Some(()),
        Err(e) => {
            debug::error(&format!("Failed to delete save (err: {})", e));
            None
        },
    }
}

#[no_mangle]
pub extern fn C_delete_character() {
    delete_character();
}

#[no_mangle]
pub extern fn C_save_character() -> libc::uint8_t {
    match save_character() {
        Some(_) => 255,
        None => 0,
    }
}

#[no_mangle]
pub extern fn C_load_character() -> libc::uint8_t {
    match load_character() {
        Some(_) => 255,
        None => 0,
    }
}
