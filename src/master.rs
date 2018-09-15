use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

use serde_json;

use constants;
use debug;
use player;
use random;

#[derive(Serialize, Deserialize)]
pub struct MasterRecord {
    pub uid: i64,
    pub user_name: String,
    pub character_name: String,
    pub points: i64,
    pub title: String,
    pub alive: bool,
    pub level: u8,
    pub race: String,
    pub class: String,
}

// TODO: Consider flocking (https://stackoverflow.com/a/32743299)
// Will probably never to this since I'm the only intended user for this program
fn open_master(to_write: bool) -> Option<File> {
    match OpenOptions::new()
        .read(!to_write)
        .write(to_write)
        .create(to_write)
        .truncate(to_write)
        .append(false)
        .open(format!("{}/moria_master.json", constants::DATA_FOLDER)) {
            Ok(file) => Some(file),
            Err(e) => {
                debug::error(&format!("failed to open master: {}", e));
                None
            },
        }
}

pub fn read_master() -> Option<Vec<MasterRecord>> {
    let mut buffer = String::new();

    let mut file = open_master(false)?;
    if let Err(e) = file.read_to_string(&mut buffer) {
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

fn write_master(data: &Vec<MasterRecord>) -> Option<()> {
    let mut file = open_master(true)?;
    if let Err(e) = file.write_all(&serde_json::to_string(data).unwrap().into_bytes()) {
        debug::error(&format!("Failed to write file: {}", e));
        return None;
    }
    Some(())
}

pub fn update_character(uid: i64) -> Option<()> {
    debug::enter("master_update_character");

    let mut records = read_master()?;

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
        record.alive = !player::is_dead();
        record.level = player::level();
    }

    let result = write_master(&records);

    debug::leave("master_update_character");
    result
}

pub fn add_character() -> Option<i64> {
    debug::enter("master_add_character");

    let mut records = read_master()?;

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

    write_master(&records)?;

    debug::leave("master_add_character");
    Some(new_uid)
}

pub fn character_exists(uid: i64) -> Option<()> {
    debug::enter("master_character_exists");

    let records = read_master()?;

    let result = match records.iter()
        .position(|ref i| i.uid == uid) {
            Some(_) => Some(()),
            None => {
                debug::warn("Master file did not contain the player");
                None
            },
        };

    debug::leave("master_character_exists");
    result
}
