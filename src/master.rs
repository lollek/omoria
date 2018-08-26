use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Write, Seek};

use libc;
use serde_json;

use debug;
use player;
use random;

#[derive(Serialize, Deserialize)]
struct MasterRecord {
    uid: i64,
    user_name: String,
    character_name: String,
    points: i64,
    title: String,
    alive: bool,
    level: u8,
    race: String,
    class: String,
}

const MASTER_FILE: &'static str = "save/moria_master.json";

// TODO: Consider flocking (https://stackoverflow.com/a/32743299)
// Will probably never to this since I'm the only intended user for this program
fn open_master() -> Option<File> {
    match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .append(false)
        .open(MASTER_FILE) {
            Ok(file) => Some(file),
            Err(e) => {
                debug::error(&format!("failed to open master: {}", e));
                None
            },
        }
}

fn read_master(mut f: &File) -> Option<Vec<MasterRecord>> {
    let mut buffer = String::new();

    if let Err(e) = f.read_to_string(&mut buffer) {
        debug::warn(&format!("Either master was empty, or corrupt, (err: {})", e));
        return Some(vec![]);
    }

    match serde_json::from_str(&buffer) {
        Ok(json) => Some(json),
        Err(e) => {
            debug::warn(&format!("Either master was empty, or corrupt, (err: {})", e));
            Some(vec![])
        },
    }
}

fn write_master(mut f: &File, data: &Vec<MasterRecord>) -> Option<()> {
    if data.len() > 1 {
        if let Err(e) = f.seek(io::SeekFrom::Start(0)) {
            debug::error(&format!("Failed during seek: {}", e));
            return None;
        }
    }
    if let Err(e) = f.write_all(&serde_json::to_string(data).unwrap().into_bytes()) {
        debug::error(&format!("Failed to write file: {}", e));
        return None;
    }
    Some(())
}

pub fn master_update_character(uid: i64) -> Option<()> {
    debug::enter("master_record_death");

    let mut file = open_master()?;
    let mut records = read_master(&file)?;

    let pos = match records.iter()
        .position(|ref i| i.uid == uid) {
            Some(pos) => pos,
            None => {
                debug::warn("Master file did not contain the player");
                return None;
            },
        };

    {
        let record = records.get_mut(pos).unwrap();
        record.character_name = player::name();
        record.points = player::calc_total_points();
        record.title = player::title();
        record.alive = player::is_dead();
        record.level = player::level();
    }

    let result = write_master(&mut file, &records);

    debug::leave("master_record_death");
    result
}

pub fn master_add_character() -> Option<i64> {
    debug::enter("master_add_character");

    let mut file = open_master()?;
    let mut records = read_master(&file)?;

    let mut new_uid;
    loop {
        new_uid = random::randint(<i64>::max_value() -1);
        if new_uid != 0 {
            break;
        }
    }

    records.push(MasterRecord{
        uid: new_uid,
        user_name: "-".to_string(),
        character_name: player::name(),
        points: player::calc_total_points(),
        title: player::title(),
        alive: true,
        level: player::level(),
        race: player::race().name().to_string(),
        class: player::class().name().to_string(),
    });

    write_master(&mut file, &records)?;

    debug::leave("master_add_character");
    Some(new_uid)
}

#[no_mangle]
pub extern fn C_master_update_character(uid: libc::int64_t) -> libc::uint8_t {
    match master_update_character(uid) {
        Some(_) => 255,
        None => 0,
    }
}

#[no_mangle]
pub extern fn C_master_add_character() -> libc::int64_t {
    match master_add_character() {
        Some(uid) => uid,
        None => 0,
    }
}
