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
    inventory: Vec<TreasureRecJson>,

    /*
		sc__write_equipment(f1, &cf_state, out_rec);
		sc__write_dungeon(f1, &cf_state, out_rec);
		sc__write_identified(f1, &cf_state, out_rec);
		sc__write_monsters(f1, &cf_state, out_rec);
		sc__write_town(f1, &cf_state, out_rec);
    */
    /*
     * missile_ctr (player_record -> inventory/equipment)
     * equip_ctr (player_record -> equipment)
     * dun_level (player_record -> dungeon)
     * mon_tot_mult (player_record -> ???)
     * turn (player_record -> ???)
     * randes_seed (player_record -> ???)
     */
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


    /*
	if (paniced) {
		exit_game();
	} else {
		gc__read_seeds(f1, &cf_state, in_rec, &paniced);
		if (!paniced)
			gc__display_status();
		if (!paniced)
			gc__read_version(f1, &cf_state, in_rec, &paniced,
					 &save_version);
		if (!paniced)
			gc__read_player_record(f1, &cf_state, in_rec, &paniced,
					       prop, &was_dead);
		if (!paniced)
			gc__read_inventory(f1, &cf_state, in_rec, &paniced,
					   &was_dead);
		if (!paniced)
			gc__read_equipment(f1, &cf_state, in_rec, &paniced,
					   &was_dead);

		if (was_dead) {
			/* nuke claim_check entry, they are lucky to be alive */
			player_claim_check = 0;
			msg_print(" ");
		}

		if (!paniced)
			gc__read_stats_and_flags(f1, &cf_state, in_rec,
						 &paniced);
		if (!paniced)
			gc__read_magic(f1, &cf_state, in_rec, &paniced);
		if (!paniced)
			gc__read_dungeon(f1, &cf_state, in_rec, &paniced);
		if (!paniced)
			gc__read_identified(f1, &cf_state, in_rec, &paniced);
		if (!paniced)
			gc__read_monsters(f1, &cf_state, in_rec, &paniced);
		if (!paniced)
			gc__read_town(f1, &cf_state, in_rec, &paniced);

		/* odds are we would have paniced by this time if an
		   encrypted file has been tampered with, but just in case... */
		if (!paniced) {
			read_decrypt(f1, &cf_state, in_rec, &paniced);
			sscanf(in_rec, "%ld", &check_time);
			if (player_creation_time != check_time) {
				paniced = true;
			}
		}

		fclose(f1);

	} /* endif !paniced */

	if (!paniced)
		seed = 0;
	if (!paniced)
		paniced = !sc__open_master(&f2);
	if (!paniced) {
		gc__read_master(f2, &paniced);
		master_file_close(&f2);
	}
    */

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
