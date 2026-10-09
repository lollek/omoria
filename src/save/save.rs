use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Seek, Write};

use serde_json;

use crate::debug;
use crate::master;
use crate::ncurses;
use crate::player;
use crate::save;
use crate::save::save_record::SaveRecord;
use crate::term;
use crate::{constants, identification};

fn savefile_name(player_name: &str, player_uid: i64) -> String {
    format!(
        "{}/{}-{}.json",
        constants::SAVE_FOLDER,
        player_name,
        player_uid
    )
}

fn open_savefile(player_name: &str, player_uid: i64, to_write: bool) -> Option<File> {
    match OpenOptions::new()
        .read(!to_write)
        .write(to_write)
        .create(to_write)
        .truncate(to_write)
        .append(false)
        .open(savefile_name(player_name, player_uid))
    {
        Ok(file) => Some(file),
        Err(e) => {
            debug::error(format!("failed to open save file: {}", e));
            None
        }
    }
}

fn parse_save(json: &str) -> serde_json::Result<SaveRecord> {
    serde_json::from_str(json)
}

fn serialize_save(data: &SaveRecord) -> serde_json::Result<String> {
    serde_json::to_string(data)
}

fn read_save(mut f: &File) -> Option<SaveRecord> {
    let mut buffer = String::new();

    if let Err(e) = f.read_to_string(&mut buffer) {
        debug::error(format!("Failed to load save @read_to_string, (err: {})", e));
        return None;
    }

    match parse_save(&buffer) {
        Ok(json) => Some(json),
        Err(e) => {
            debug::error(format!("Failed to load save @from_str, (err: {})", e));
            None
        }
    }
}

fn write_save(mut f: &File, data: &SaveRecord) -> Option<()> {
    let serialized_data = match serialize_save(data) {
        Ok(serialized_data) => serialized_data,
        Err(e) => {
            debug::error(format!("Failed to serialize data: {e}"));
            debug_serialize_save_record(data);
            return None;
        }
    };
    if let Err(e) = f.seek(io::SeekFrom::Start(0)) {
        debug::error(format!("Failed during seek: {}", e));
        return None;
    }
    if let Err(e) = f.write_all(&serialized_data.into_bytes()) {
        debug::error(format!("Failed to write file: {}", e));
        return None;
    }
    Some(())
}

fn debug_serialize_save_record(save_record: &SaveRecord) {
    fn serialize_status<'a>(value: impl serde::Serialize) -> &'a str {
        match serde_json::to_string(&value) {
            Ok(_) => "OK",
            Err(_) => "ERROR",
        }
    }
    debug::error("### DEBUG SERIALIZE SAVE RECORD START ###");
    debug::error(format!(
        "Player: {}, Inventory: {}, Equipment: {}, Town: {}, Dungeon: {}, Identified: {}, Monsters: {}",
        serialize_status(&save_record.player),
        serialize_status(&save_record.inventory),
        serialize_status(&save_record.equipment),
        serialize_status(&save_record.town),
        serialize_status(&save_record.dungeon),
        serialize_status(&save_record.identified),
        serialize_status(&save_record.monsters)
    ));
    debug::error("### DEBUG SERIALIZE SAVE RECORD END ###");
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
        }
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
    identification::set_record(records.identified);
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
    write_save(
        &file,
        &SaveRecord {
            player: player::record(),
            inventory: save::inventory::record(),
            equipment: save::equipment::record(),
            town: save::town::record(),
            dungeon: save::dungeon::record(),
            identified: identification::record(),
            monsters: save::monsters::record(),
        },
    )?;
    Some(())
}

pub fn delete_character() -> Option<()> {
    match fs::remove_file(savefile_name(&player::name(), player::uid())) {
        Ok(_) => Some(()),
        Err(e) => {
            debug::error(format!("Failed to delete save (err: {})", e));
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::error::Category;
    use serde_json::Value;

    use super::*;
    use crate::model::item_subtype::{FoodSubType, ItemSubType};
    use crate::model::{Class, Race, Sex};

    const FIXTURE: &str = include_str!("../../tests/fixtures/save_record_v1.json");

    // Debug-build SaveRecord deserialization overflows the 2 MiB default test-thread stack.
    fn with_large_stack<F: FnOnce() + Send + 'static>(test: F) {
        let handle = std::thread::Builder::new()
            .name(std::thread::current().name().unwrap_or("test").into())
            .stack_size(8 * 1024 * 1024)
            .spawn(test)
            .expect("failed to spawn test thread");
        if let Err(panic) = handle.join() {
            std::panic::resume_unwind(panic);
        }
    }

    fn fixture_record() -> SaveRecord {
        parse_save(FIXTURE).expect("fixture should deserialize")
    }

    // Identified entries come from a HashMap, so their serialized order is unspecified.
    fn normalize(mut value: Value) -> Value {
        if let Some(Value::Array(entries)) = value.get_mut("identified") {
            entries.sort_by_key(|entry| entry.to_string());
        }
        value
    }

    #[test]
    fn fixture_player_fields_are_loaded() {
        with_large_stack(|| {
            let player = fixture_record().player;

            assert_eq!(player.name, "Fixture");
            assert_eq!(player.uid, 1);
            assert_eq!(player.race, Race::Elf);
            assert_eq!(player.sex, Sex::Male);
            assert_eq!(player.class, Class::Wizard);
            assert_eq!(player.lev, 2);
            assert_eq!(player.money.total, 513);
            assert_eq!(player.money.mithril, 9);
            assert_eq!((player.char_row, player.char_col), (39, 140));
        });
    }

    #[test]
    fn fixture_inventory_and_equipment_are_loaded() {
        with_large_stack(|| {
            let record = fixture_record();

            assert_eq!(record.inventory.len(), 7);
            let first = &record.inventory[0].data;
            let name: Vec<u8> = first
                .name
                .iter()
                .take_while(|&&c| c != 0)
                .map(|&c| c as u8)
                .collect();
            assert_eq!(name, b"& Book of Magic Spells [Beginners-Magik]".to_vec());
            assert_eq!(
                (first.tval, first.subval, first.cost),
                (90, 257, 6000),
                "first inventory item should be the starting spell book"
            );

            assert_eq!(record.equipment.len(), 15);
            assert_eq!(
                (record.equipment[0].tval, record.equipment[0].subval),
                (23, 3),
                "wielded weapon should be loaded into the first equipment slot"
            );
        });
    }

    #[test]
    fn fixture_dungeon_state_is_loaded() {
        with_large_stack(|| {
            let dungeon = fixture_record().dungeon;

            assert_eq!((dungeon.cur_height, dungeon.cur_width), (66, 198));
            assert_eq!(dungeon.cave.len(), 66 * 198);
            assert_eq!(dungeon.treasure.len(), 64);
            assert_eq!(dungeon.dun_level, 1);
            assert_eq!(dungeon.turn, 1973);
        });
    }

    #[test]
    fn fixture_town_identification_and_monsters_are_loaded() {
        with_large_stack(|| {
            let record = fixture_record();

            assert_eq!(record.town.stores.len(), 13);
            let identified = serde_json::to_value(&record.identified).unwrap();
            assert_eq!(identified.as_array().map(Vec::len), Some(136));
            let ration = ItemSubType::Food(FoodSubType::RationOfFood);
            assert_eq!(record.identified.get(ration), Some(true));
            let mushroom = ItemSubType::Food(FoodSubType::Mushroom2);
            assert_eq!(record.identified.get(mushroom), None);

            assert_eq!(record.monsters.monsters.len(), 18);
            let monster = &record.monsters.monsters[0];
            assert_eq!(
                (monster.hp, monster.fy, monster.fx),
                (2, 39, 142),
                "first monster should keep its hit points and position"
            );
        });
    }

    #[test]
    fn fixture_round_trips_without_semantic_changes() {
        with_large_stack(|| {
            let serialized = serialize_save(&fixture_record()).expect("record should serialize");
            parse_save(&serialized).expect("serialized record should parse again");

            let original = normalize(serde_json::from_str(FIXTURE).unwrap());
            let round_tripped = normalize(serde_json::from_str(&serialized).unwrap());
            let (original, round_tripped) = (
                original.as_object().unwrap(),
                round_tripped.as_object().unwrap(),
            );
            assert_eq!(
                original.keys().collect::<Vec<_>>(),
                round_tripped.keys().collect::<Vec<_>>()
            );
            // Compare per section so a failure doesn't dump the whole save.
            for (section, value) in original {
                assert!(
                    value == &round_tripped[section],
                    "section `{}` changed after round trip",
                    section
                );
            }
        });
    }

    #[test]
    fn truncated_save_is_an_eof_error() {
        with_large_stack(|| {
            let truncated = &FIXTURE[..FIXTURE.len() / 2];

            let err = parse_save(truncated).expect_err("truncated save should fail");
            assert_eq!(err.classify(), Category::Eof);
        });
    }

    #[test]
    fn malformed_save_is_a_syntax_error() {
        let err = parse_save("{\"player\": nope}").expect_err("malformed save should fail");
        assert_eq!(err.classify(), Category::Syntax);
    }

    #[test]
    fn save_missing_required_section_is_a_data_error() {
        with_large_stack(|| {
            let mut value: Value = serde_json::from_str(FIXTURE).unwrap();
            value.as_object_mut().unwrap().remove("dungeon");

            let err = parse_save(&value.to_string()).expect_err("missing dungeon should fail");
            assert_eq!(err.classify(), Category::Data);
            assert!(
                err.to_string().contains("missing field `dungeon`"),
                "unexpected error: {}",
                err
            );
        });
    }
}
