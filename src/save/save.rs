use std::fs::{File, OpenOptions};
use std::fs;
use std::io;
use std::io::{Read, Write, Seek};

use serde_json;

use crate::constants;
use crate::debug;
use crate::master;
use crate::ncurses;
use crate::save;
use crate::term;
use crate::model::{
    DungeonRecord, IdentifiedRecord, InventoryItem, Item, MonsterRecord,
    PlayerRecord, TownRecord
};
use crate::player;


#[derive(Serialize, Deserialize)]
struct SaveRecord {
    player: PlayerRecord,
    inventory: Vec<InventoryItem>,
    equipment: Vec<Item>,
    town: TownRecord,
    dungeon: DungeonRecord,
    identified: IdentifiedRecord,
    monsters: MonsterRecord,
}

fn savefile_name(player_name: &str, player_uid: i64) -> String {
    format!("{}/{}-{}.json", constants::SAVE_FOLDER, player_name, player_uid)
}

fn open_savefile(player_name: &str, player_uid: i64, to_write: bool) -> Option<File> {
    match OpenOptions::new()
        .read(!to_write)
        .write(to_write)
        .create(to_write)
        .truncate(to_write)
        .append(false)
        .open(savefile_name(player_name, player_uid)) {
            Ok(file) => Some(file),
            Err(e) => {
                debug::error(&format!("failed to open save file: {}", e));
                None
            },
        }
}

fn read_save(mut f: &File) -> Option<SaveRecord> {
    let mut buffer = String::new();

    if let Err(e) = f.read_to_string(&mut buffer) {
        debug::warn(format!("Failed to load save @read_to_string, (err: {})", e));
        return None;
    }

    match serde_json::from_str(&buffer) {
        Ok(json) => Some(json),
        Err(e) => {
            debug::warn(format!("Failed to load save @from_str, (err: {})", e));
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

pub fn load_character_with_feedback(player_name: &str, player_uid: i64) -> Option<()> {
    term::prt("Restoring Character...", 1, 1);
    ncurses::refresh();

    let result = match load_character(player_name, player_uid) {
        Some(_) => Some(()),
        None => {
            debug::error("Failed to load character!");
            term::prt("Data Corruption Error", 0, 0);
            None
        },
    };

    ncurses::clear();
    result
}

fn load_character(player_name: &str, player_uid: i64) -> Option<()> {
    if !master::character_exists(player_uid) {
        debug::error("Character does not exist in master!");
        return None;
    }

    let file = open_savefile(player_name, player_uid, false)?;
    let records = read_save(&file)?;
    player::set_record(records.player);
    save::inventory::set_record(records.inventory);
    save::equipment::set_record(records.equipment);
    save::town::set_record(records.town);
    save::dungeon::set_record(records.dungeon);
    save::identified::set_record(records.identified);
    save::monsters::set_record(records.monsters);
    Some(())
}

pub fn save_character_with_feedback() -> Option<()> {
    if !player::is_dead() {
        term::clear_from(0);
        term::prt("Saving character...", 0, 0);
        ncurses::refresh();
    }

    save_character()
}

fn save_character() -> Option<()> {
    if let Err(e) = master::update_character(player::uid()) {
        debug::error(format!("Failed to update character in master: {}", e).as_str());
        return None;
    }
    player::increase_save_counter();

    let file = open_savefile(&player::name(), player::uid(), true)?;
    write_save(&file, &SaveRecord{
        player: player::record(),
        inventory: save::inventory::record(),
        equipment: save::equipment::record(),
        town: save::town::record(),
        dungeon: save::dungeon::record(),
        identified: save::identified::record(),
        monsters: save::monsters::record(),
    })?;
    Some(())
}

pub fn delete_character() -> Option<()> {
    match fs::remove_file(savefile_name(&player::name(), player::uid())) {
        Ok(_) => Some(()),
        Err(e) => {
            debug::error(&format!("Failed to delete save (err: {})", e));
            None
        },
    }
}
