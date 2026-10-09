use crate::debug;
use crate::identification;
use crate::master;
use crate::ncurses;
use crate::persistence::{self, json, CharacterStorageError};
use crate::player;
use crate::save;
use crate::save::save_record::SaveRecord;
use crate::term;

fn debug_serialize_save_record(save_record: &SaveRecord) {
    fn serialize_status<'a>(value: impl serde::Serialize) -> &'a str {
        match json::encode(&value) {
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

    let records = match persistence::load_save(player_name, player_uid) {
        Ok(records) => records,
        Err(err) => {
            debug::error(format!("Failed to load save: {}", err));
            return None;
        }
    };
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

    let record = SaveRecord {
        player: player::record(),
        inventory: save::inventory::record(),
        equipment: save::equipment::record(),
        town: save::town::record(),
        dungeon: save::dungeon::record(),
        identified: identification::record(),
        monsters: save::monsters::record(),
    };
    match persistence::write_save(&player::name(), player::uid(), &record) {
        Ok(()) => Some(()),
        Err(err @ CharacterStorageError::Codec(_)) => {
            debug::error(format!("Failed to serialize data: {}", err));
            debug_serialize_save_record(&record);
            None
        }
        Err(err) => {
            debug::error(format!("Failed to write file: {}", err));
            None
        }
    }
}

pub fn delete_character() -> Option<()> {
    match persistence::delete_save(&player::name(), player::uid()) {
        Ok(_) => Some(()),
        Err(e) => {
            debug::error(format!("Failed to delete save (err: {})", e));
            None
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use serde_json::Value;

    use super::*;
    use crate::model::item_subtype::{FoodSubType, ItemSubType};
    use crate::model::{Class, Race, Sex};

    pub(crate) const FIXTURE: &str = include_str!("../../tests/fixtures/save_record_v1.json");

    // Debug-build SaveRecord deserialization overflows the 2 MiB default test-thread stack.
    pub(crate) fn with_large_stack<F: FnOnce() + Send + 'static>(test: F) {
        let handle = std::thread::Builder::new()
            .name(std::thread::current().name().unwrap_or("test").into())
            .stack_size(8 * 1024 * 1024)
            .spawn(test)
            .expect("failed to spawn test thread");
        if let Err(panic) = handle.join() {
            std::panic::resume_unwind(panic);
        }
    }

    pub(crate) fn fixture_record() -> SaveRecord {
        json::decode(FIXTURE).expect("fixture should deserialize")
    }

    // Identified entries come from a HashMap, so their serialized order is unspecified.
    pub(crate) fn normalize(mut value: Value) -> Value {
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
            let serialized = json::encode(&fixture_record()).expect("record should serialize");
            json::decode::<SaveRecord>(&serialized).expect("serialized record should parse again");

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

            let err =
                json::decode::<SaveRecord>(truncated).expect_err("truncated save should fail");
            assert!(err.to_string().contains("EOF"), "unexpected error: {}", err);
        });
    }

    #[test]
    fn malformed_save_is_a_syntax_error() {
        let err = json::decode::<SaveRecord>("{\"player\": nope}")
            .expect_err("malformed save should fail");
        assert!(
            err.to_string().contains("expected"),
            "unexpected error: {}",
            err
        );
    }

    #[test]
    fn save_missing_required_section_is_a_data_error() {
        with_large_stack(|| {
            let mut value: Value = serde_json::from_str(FIXTURE).unwrap();
            value.as_object_mut().unwrap().remove("dungeon");

            let err = json::decode::<SaveRecord>(&value.to_string())
                .expect_err("missing dungeon should fail");
            assert!(
                err.to_string().contains("missing field `dungeon`"),
                "unexpected error: {}",
                err
            );
        });
    }

    #[test]
    fn save_with_invalid_identified_entry_is_a_data_error() {
        with_large_stack(|| {
            let mut value: Value = serde_json::from_str(FIXTURE).unwrap();
            value["identified"]
                .as_array_mut()
                .expect("identified should be an array")
                .push(serde_json::json!([255, 0, true]));

            let err = json::decode::<SaveRecord>(&value.to_string())
                .expect_err("invalid identified entry should fail save decoding");
            assert!(
                err.to_string().contains("item type") && err.to_string().contains("255"),
                "unexpected error: {}",
                err
            );
        });
    }
}
