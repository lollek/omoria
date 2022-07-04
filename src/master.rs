use crate::data;
use crate::debug;
use crate::error::Error;
use crate::player;
use crate::persistence;
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

pub fn read_master() -> Result<Vec<MasterRecord>, Error> {
    persistence::load_masters()
}

pub fn update_character(uid: i64) -> Result<(), Error> {
    debug::enter("master_update_character");

    persistence::save_master(MasterRecord {
        uid: uid,
        user_name: "-".to_string(),
        character_name: player::name(),
        points: player::calc_total_points(),
        alive: !player::is_dead(),
        level: player::level(),
        race: data::race::name(&player::race()).to_string(),
        class: data::class::name(&player::class()).to_string(),
    }, false)?;

    debug::leave("master_update_character");
    Ok(())
}

pub fn add_character() -> Result<i64, Error> {
    debug::enter("master_add_character");

    let mut new_uid;
    loop {
        new_uid = random::randint(<i64>::max_value() -1);
        if new_uid != 0 {
            break;
        }
    }

    persistence::save_master(MasterRecord{
        uid: new_uid,
        user_name: "-".to_string(),
        character_name: player::name(),
        points: player::calc_total_points(),
        alive: true,
        level: player::level(),
        race: data::race::name(&player::race()).to_string(),
        class: data::class::name(&player::class()).to_string(),
    }, true)?;

    debug::leave("master_add_character");
    Ok(new_uid)
}

pub fn character_exists(uid: i64) -> bool {
    debug::enter("master_character_exists");

    let records = persistence::load_masters();
    if let Err(e) = records {
        debug::error(&format!("{:?}", e));
        return false;
    }

    let result = match records.unwrap().iter()
        .position(|ref i| i.uid == uid) {
            Some(_) => true,
            None => {
                debug::warn("Master file did not contain the player");
                false
            },
        };

    debug::leave("master_character_exists");
    result
}
