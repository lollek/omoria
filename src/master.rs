use crate::data;
use crate::debug;
use crate::error::Error;
use crate::persistence;
use crate::player;
use crate::random;

#[derive(Serialize, Deserialize)]
pub struct MasterRecord {
    pub uid: i64,
    pub user_name: String,
    pub character_name: String,
    pub points: i64,
    pub alive: bool,
    pub level: u8,
    pub race: String,
    pub class: String,
}

pub fn init_master() -> Result<(), Error> {
    persistence::init_masters()
}

pub fn read_master() -> Result<Vec<MasterRecord>, Error> {
    persistence::load_masters()
}

pub fn update_character(uid: i64) -> Result<(), Error> {
    persistence::save_master(
        MasterRecord {
            uid,
            user_name: "-".to_string(),
            character_name: player::name(),
            points: player::calc_total_points(),
            alive: !player::is_dead(),
            level: player::level(),
            race: data::race::name(&player::race()).to_string(),
            class: data::class::name(&player::class()).to_string(),
        },
        false,
    )?;

    Ok(())
}

pub fn add_character() -> Result<i64, Error> {
    let mut new_uid;
    loop {
        new_uid = random::randint(<i64>::MAX - 1);
        if new_uid != 0 {
            break;
        }
    }

    persistence::save_master(
        MasterRecord {
            uid: new_uid,
            user_name: "-".to_string(),
            character_name: player::name(),
            points: player::calc_total_points(),
            alive: true,
            level: player::level(),
            race: data::race::name(&player::race()).to_string(),
            class: data::class::name(&player::class()).to_string(),
        },
        true,
    )?;

    Ok(new_uid)
}

pub fn character_exists(uid: i64) -> bool {
    let records = persistence::load_masters();
    if let Err(e) = records {
        debug::error(format!("{:?}", e));
        return false;
    }

    match records.unwrap().iter().position(|ref i| i.uid == uid) {
        Some(_) => true,
        None => {
            debug::warn("Master file did not contain the player");
            false
        }
    }
}
