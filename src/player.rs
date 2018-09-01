use libc;
use std::mem;

use std::ffi::CString;
use std::sync::RwLock;

use types::{
    Class, StatBlock, stats_iter, Wallet, currencies_iter, Race, Sex, Stat,
    Magic
};

use misc;
use debug;

#[derive(Serialize, Deserialize, Clone, Copy)]
#[repr(C)]
pub struct GameTime {
    pub year: libc::int64_t,
    pub month: libc::uint8_t,
    pub day: libc::uint8_t,
    pub hour: libc::uint8_t,
    pub secs: libc::uint16_t,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[repr(C)]
pub struct Time {
    pub years: libc::uint16_t,
    pub months: libc::uint16_t,
    pub days: libc::uint16_t,
    pub hours: libc::uint16_t,
    pub minutes: libc::uint16_t,
    pub seconds: libc::uint16_t ,
    pub hundredths: libc::uint16_t,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[repr(C)]
pub struct PlayerFlags {
    pub insured: libc::uint8_t,      /* { Character insured   } */
    pub dead: libc::uint8_t,  /* { Currently restored  } */
    pub status: libc::uint64_t, /* { Status of player    } */
    pub rest: libc::int64_t,     /* { Rest counter  } */
    pub blind: libc::int64_t,    /* { Blindness counter   } */
    pub paralysis: libc::int64_t,       /* { Paralysis counter   } */
    pub confused: libc::int64_t, /* { Confusion counter   } */
    pub foodc: libc::int64_t,    /* { Food counter        } (was just food) */
    pub food_digested: libc::int64_t,   /* { Food per round      } */
    pub protection: libc::int64_t,      /* { Protection fr. evil } */
    pub speed: libc::int64_t,    /* { Cur speed adjust    } */
    pub speed_paral: libc::int64_t,     /* { Slow speed adjust   } */
    pub speed_flag: libc::uint8_t,   /* { On if reset speed   } */
    pub paral_init: libc::int64_t,      /* { Init val for slow   } */
    pub move_rate: libc::int64_t,       /* { move_rate          } */
    pub swim: libc::int64_t,     /* { Cur swim adjust     } */
    pub fast: libc::int64_t,     /* { Temp speed change   } */
    pub slow: libc::int64_t,     /* { Temp speed change   } */
    pub petrification: libc::int64_t,   /* { Amount Petrified    } */
    pub afraid: libc::int64_t,   /* { Fear                } */
    pub poisoned: libc::int64_t, /* { Poisoned            } */
    pub image: libc::int64_t,    /* { Hallucinate         } */
    pub protevil: libc::int64_t, /* { Protect VS evil     } */
    pub invuln: libc::int64_t,   /* { Increases AC        } */
    pub hero: libc::int64_t,     /* { Heroism          } */
    pub shero: libc::int64_t,    /* { Super Heroism  } */
    pub blessed: libc::int64_t,  /* { Blessed          } */
    pub resist_heat: libc::int64_t,     /* { Timed heat resist   } */
    pub resist_cold: libc::int64_t,     /* { Timed cold resist   } */
    pub detect_inv: libc::int64_t,      /* { Timed see invisible } */
    pub word_recall: libc::int64_t,     /* { Timed teleport level} */
    pub see_infra: libc::int64_t,       /* { See warm creatures  } */
    pub tim_infra: libc::int64_t,       /* { Timed infra vision  } */
    pub see_inv: libc::uint8_t,      /* { Can see invisible   } */
    pub teleport: libc::uint8_t,     /* { Random teleportation} */
    pub free_act: libc::uint8_t,     /* { Never paralyzed     } */
    pub slow_digest: libc::uint8_t,  /* { Lower food needs    } */
    pub aggravate: libc::uint8_t,    /* { Agravate monsters   } */
    pub fire_resist: libc::uint8_t,  /* { Resistance to fire  } */
    pub cold_resist: libc::uint8_t,  /* { Resistance to cold  } */
    pub acid_resist: libc::uint8_t,  /* { Resistance to acid  } */
    pub hunger_item: libc::uint8_t,  /* { Resets food counter } */
    pub regenerate: libc::uint8_t,   /* { Regenerate hit pts  } */
    pub lght_resist: libc::uint8_t,  /* { Resistance to light } */
    pub ffall: libc::uint8_t, /* { No damage falling   } */
    pub sustain: [libc::uint8_t; 6], /* { keep characteristic } */
    pub confuse_monster: libc::uint8_t,    /* { Glowing hands...    } */
    pub resist_lght: libc::int64_t,     /* { Timed lighting rst  } */
    pub free_time: libc::int64_t,      /* { Timed free action   } */
    pub ring_fire: libc::int64_t,      /* { Timed fire spell    } */
    pub protmon: libc::int64_t,      /* { Timed monst prot    } */
    pub hoarse: libc::int64_t,      /* { Timed no-bard spells} */
    pub magic_prot: libc::int64_t,     /* { Timed magic prot    } */
    pub ring_ice: libc::int64_t,      /* { Timed cold spell    } */
    pub temp_stealth: libc::int64_t,     /* { Timed stealth       } */
    pub resist_petri: libc::int64_t,     /* { Timed resist petrify} */
    pub blade_ring: libc::int64_t,     /* { Timed blade spell   } */
    pub petri_resist: libc::uint8_t,     /* { Resist Petrification} */
    pub quested: libc::uint8_t,     /* { Performing a Quest  } {FUBAR} */
    pub light_on: libc::uint8_t,     /* { Light source is active } */
    pub resting_till_full: libc::uint8_t,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub spells_known: Vec<bool>
}

impl Player {
    pub fn new() -> Player {
        Player {
            spells_known: [false; 40].to_vec(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[repr(C)]
pub struct PlayerRecord {
    pub uid: libc::int64_t,
    pub save_count: libc::int64_t,
    pub deaths: libc::int64_t,
    pub xtr_wgt: libc::int64_t,
    pub account: libc::int64_t,
    pub money: Wallet,
    pub diffic: libc::uint8_t,
    pub birth: GameTime,
    pub cur_age: GameTime,
    pub cur_quest: libc::uint16_t,
    pub quests: libc::uint8_t,
    pub claim_check: libc::int64_t,
    pub play_tm: Time,
    pub name: String,
    pub race: Race,
    pub sex: Sex,
    pub class: Class,
    pub title: String,
    pub history: Vec<String>,
    pub cheated: libc::uint8_t,
    pub age: libc::uint16_t,
    pub ht: libc::uint16_t,
    pub wt: libc::uint16_t,
    pub sc: libc::int16_t,
    pub max_exp: libc::int64_t,
    pub exp: libc::int64_t,
    pub rep: libc::int64_t,
    pub premium: libc::int64_t,
    pub lev: libc::uint16_t,
    pub max_lev: libc::uint16_t,
    pub expfact: libc::c_float,
    pub srh: libc::int16_t,
    pub fos: libc::int16_t,
    pub stl: libc::uint8_t,
    pub bth: libc::int16_t,
    pub bthb: libc::int16_t,
    pub mana: libc::int16_t,
    pub cmana: libc::c_float,
    pub mhp: libc::int16_t,
    pub chp: libc::c_float,
    pub ptohit: libc::int16_t,
    pub ptodam: libc::int16_t,
    pub pac: libc::int16_t,
    pub ptoac: libc::int16_t,
    pub dis_th: libc::int16_t,
    pub dis_td: libc::int16_t,
    pub dis_ac: libc::int16_t,
    pub dis_tac: libc::int16_t,
    pub disarm: libc::int16_t,
    pub save: libc::int16_t,
    pub hitdie: libc::uint8_t,
    pub perm_stats: StatBlock,
    pub curr_stats: StatBlock,
    pub mod_stats: StatBlock,
    pub lost_stats: StatBlock,
    pub flags: PlayerFlags,
    pub player: Player,
}

extern "C" {
    pub static mut player_xtr_wgt: libc::int64_t ;	  /* { Extra weight limit	} */
    pub static mut player_money: [libc::int64_t; 7] ;	 /* { Money on person	} */
    pub static mut player_play_tm: Time ;	/* { Time spent in game	} */
    pub static mut player_max_exp: libc::int64_t ;		  /* { Max experience} */
    pub static mut player_deaths: libc::int64_t ;		  /* {Number of insured restores} */
    pub static mut player_premium: libc::int64_t ;		  /* {Base cost to restore } */
    pub static mut player_max_lev: libc::uint16_t ;   /* { Max level explored} */
    pub static mut player_srh: libc::int16_t ;		  /* { Chance in search} */
    pub static mut player_chp: libc::c_float ;		  /* { Cur hit pts	} */
    pub static mut player_cheated: libc::uint8_t ;	  /*{ gone into wizard or god mode} */
    pub static mut player_quests: libc::uint8_t ;     /* { # completed } {FUBAR} */
    pub static mut player_cur_quest: libc::uint16_t ; /* { creature # of quest } {FUBAR} */
    pub static mut player_birth: GameTime;     /* {Date of char's birth} */
    pub static mut player_cur_age: GameTime;   /* {Current game date	} */
    pub static mut player_flags: PlayerFlags;
    pub static mut player_history: [[libc::c_char; 82]; 5];
    static mut player_title: [libc::c_char; 82];
    static mut player_name: [libc::c_char; 82];
    static mut player_race: [libc::c_char; 82];
    static mut player_prace: libc::uint8_t;
    static mut player_sex: [libc::c_char; 82];
    static mut player_tclass: [libc::c_char; 82];
    static mut player_pclass: libc::c_int;
    pub static mut player_stl: libc::uint8_t;
    pub static mut player_sc: libc::int16_t ;		  /* { Social Class	} */
    pub static mut player_age: libc::uint16_t ;       /* { Characters age} */
    pub static mut player_ht: libc::uint16_t ;	/* { Height	} */
    pub static mut player_wt: libc::uint16_t ;	/* { Weight	} */
    pub static mut player_lev: libc::uint16_t ;       /* { Level		} */
    pub static mut player_mhp: libc::int16_t ;		  /* { Max hit pts	} */
    pub static mut player_fos: libc::int16_t ;		  /* { Frenq of search} */
    pub static mut player_bth: libc::int16_t ;		  /* { Base to hit	} */
    pub static mut player_bthb: libc::int16_t ;		  /* { BTH with bows	} */
    pub static mut player_mana: libc::int16_t ;		  /* { Mana points	} */
    pub static mut player_ptohit: libc::int16_t ;		  /* { Pluses to hit	} */
    pub static mut player_ptodam: libc::int16_t ;		  /* { Pluses to dam	} */
    pub static mut player_pac: libc::int16_t ;		  /* { Total AC	} */
    pub static mut player_ptoac: libc::int16_t ;		  /* { Magical AC	} */
    pub static mut player_dis_th: libc::int16_t ;		  /* { Display +ToHit} */
    pub static mut player_dis_td: libc::int16_t ;		  /* { Display +ToDam} */
    pub static mut player_dis_ac: libc::int16_t ;		  /* { Display +ToAC } */
    pub static mut player_dis_tac: libc::int16_t ;		  /* { Display +ToTAC} */
    pub static mut player_disarm: libc::int16_t ;		  /* { % to Disarm	} */
    pub static mut player_save: libc::int16_t ;		  /* { Saving throw	} */
    pub static mut player_hitdie: libc::uint8_t ;     /* { Char hit die	} */
    pub static mut player_expfact: libc::c_float ;		  /* { Experience factor} */
    pub static mut player_cmana: libc::c_float ;		  /* { Cur mana pts  } */
    pub static mut player_diffic: libc::uint8_t ;     /* { Difficulty of game	} */
    pub static mut player_exp: libc::int64_t ;		  /* { Cur experienc	} */
    pub static mut player_account: libc::int64_t ;		  /* { Money in the bank	} */
    pub static mut player_mr: libc::int64_t  ;		  /* { mag.res.lev.delta } */
    pub static mut player_rep: libc::int64_t ;		  /* { XP from good creatures } */
    pub static mut player_claim_check: libc::int64_t ;	 /* used to track trading post */
    pub static mut player_save_count: libc::int64_t ;	  /* compared to master file value */
    pub static mut player_creation_time: libc::time_t ;     /* used as key in master file */
    static mut player_uid: libc::int64_t;	/* Used in master file */

    static mut player_stats_perm: [libc::uint8_t; 6];
    static mut player_stats_curr: [libc::uint8_t; 6];
    pub static mut player_stats_mod: [libc::int8_t; 6];
    pub static mut player_stats_lost: [libc::uint8_t; 6];

    static mut bank: [libc::int64_t; 7];

    #[link_name = "total_points"]
    fn C_total_points() -> libc::int64_t;
}

lazy_static! {
    static ref PLAYER: RwLock<Player> = RwLock::new(Player::new());
}

pub fn name() -> String {
    let string: Vec<u8> = unsafe { player_name }.iter().map(|&i| i as u8).collect();
    misc::c_array_to_rust_string(string)
}

pub fn set_name(new_name: &str) {
    let cstr = CString::new(new_name).unwrap();
    unsafe { libc::strcpy(player_name.as_mut_ptr() as *mut i8, cstr.as_ptr()); }
}

pub fn race() -> Race {
    debug::enter("player::race");
    let result = Race::from(unsafe { player_prace } as usize);
    debug::leave("player::race");
    result
}

pub fn set_race(race: Race) {
    debug::enter("player::set_race");

    let cstr = CString::new(race.name()).unwrap();
    unsafe {
        player_prace = race as u8;
        libc::strcpy(player_race.as_mut_ptr(), cstr.as_ptr());
    }
    debug::leave("player::set_race");
}

pub fn sex() -> Sex {
    Sex::from(unsafe { player_sex[0] as u8 as char })
}

pub fn set_sex(sex: Sex) {
    let cstr = CString::new(sex.to_string()).unwrap();
    unsafe { libc::strcpy(player_sex.as_mut_ptr(), cstr.as_ptr()); }
}

pub fn class() -> Class {
    debug::enter("player::class");
    let result = Class::from(unsafe { player_pclass } as usize);
    debug::leave("player::class");
    result
}

pub fn set_class(class: Class) {
    debug::enter("player::set_class");
    let cstr = CString::new(class.name()).unwrap();
    unsafe {
        player_pclass = class as i32;
        libc::strcpy(player_tclass.as_mut_ptr(), cstr.as_ptr());
    }
    debug::leave("player::set_class");
}

pub fn perm_stats() -> StatBlock {
    StatBlock::from(unsafe { player_stats_perm })
}

pub fn set_perm_stats(block: &StatBlock) {
    for stat in stats_iter() {
        unsafe { player_stats_perm[stat] = block.get_pos(stat) as u8 };
    }
}

pub fn curr_stats() -> StatBlock {
    StatBlock::from(unsafe { player_stats_curr })
}

pub fn set_curr_stats(block: &StatBlock) {
    for stat in stats_iter() {
        unsafe { player_stats_curr[stat] = block.get_pos(stat) as u8 };
    }
}

pub fn mod_stats() -> StatBlock {
    StatBlock::from(unsafe { player_stats_mod })
}

pub fn set_mod_stats(block: &StatBlock) {
    for stat in stats_iter() {
        unsafe { player_stats_mod[stat] = block.get_pos(stat) as i8 };
    }
}

pub fn lost_stats() -> StatBlock {
    StatBlock::from(unsafe { player_stats_lost })
}

pub fn set_lost_stats(block: &StatBlock) {
    for stat in stats_iter() {
        unsafe { player_stats_lost[stat] = block.get_pos(stat) as u8 };
    }
}

pub fn wallet() -> Wallet {
    Wallet::from(unsafe { player_money })
}

pub fn set_wallet(wallet: &Wallet) {
    for currency in currencies_iter() {
        unsafe { player_money[currency] = wallet.get_pos(currency) };
    }
    unsafe { player_money[0] = wallet.total };
}

pub fn bank_wallet() -> Wallet {
    Wallet::from(unsafe { bank })
}

pub fn set_bank_wallet(wallet: &Wallet) {
    for currency in currencies_iter() {
        unsafe { bank[currency] = wallet.get_pos(currency) };
    }
    unsafe { bank[0] = wallet.total };
}

pub fn refresh_title() {
    let new_title = class().title(level());
    let cstr = CString::new(new_title).unwrap();
    unsafe {
        libc::strcpy(player_title.as_mut_ptr(), cstr.as_ptr());
    }
}

pub fn title() -> String {
    let string: Vec<u8> = unsafe { player_title }.iter().map(|&i| i as u8).collect();
    misc::c_array_to_rust_string(string)
}

pub fn ac_from_dex() -> i8 {
    match curr_stats().get(Stat::Dexterity) {
        dex if dex < 10 => -4,
        dex if dex < 20 => -3,
        dex if dex < 30 => -2,
        dex if dex < 40 => -1,
        dex if dex < 120 => 0,
        dex if dex < 150 => 1,
        dex if dex < 191 => 2,
        dex if dex < 226 => 3,
        dex if dex < 249 => 4,
        _ => 5,
    }
}

pub fn tohit_from_stats() -> i8 {
    fn tohit_from_dex() -> i8 {
        match curr_stats().get(Stat::Dexterity) {
            dex if dex < 10 => -3,
            dex if dex < 30 => -2,
            dex if dex < 50 => -1,
            dex if dex < 130 => 0,
            dex if dex < 140 => 1,
            dex if dex < 150 => 2,
            dex if dex < 201 => 3,
            dex if dex < 250 => 4,
            _ => 5,
        }
    }

    fn tohit_from_str() -> i8 {
        match curr_stats().get(Stat::Strength) {
            str if str < 10 => -3,
            str if str < 20 => -2,
            str if str < 40 => -1,
            str if str < 150 => 0,
            str if str < 226 => 1,
            str if str < 241 => 2,
            str if str < 249 => 3,
            _ => 4,
        }
    }

    tohit_from_dex() + tohit_from_str()
}

pub fn dmg_from_str() -> i8 {
    match curr_stats().get(Stat::Strength) {
        str if str < 10 => -2,
        str if str < 20 => -1,
        str if str < 130 => 0,
        str if str < 140 => 1,
        str if str < 150 => 2,
        str if str < 226 => 3,
        str if str < 241 => 4,
        str if str < 249 => 5,
        _ => 6,
    }
}

pub fn hp_from_con() -> i8 {
    match curr_stats().get(Stat::Constitution) {
        con if con < 10 => -4,
        con if con < 20 => -3,
        con if con < 30 => -2,
        con if con < 40 => -1,
        con if con < 140 => 0,
        con if con < 150 => 1,
        con if con < 226 => 2,
        con if con < 299 => 3,
        _ => 4,
    }
}

pub fn disarm_from_dex() -> i8 {
    match curr_stats().get(Stat::Dexterity) {
        dex if dex < 10 => -8,
        dex if dex < 20 => -6,
        dex if dex < 30 => -4,
        dex if dex < 40 => -2,
        dex if dex < 50 => -1,
        dex if dex < 100 => 0,
        dex if dex < 130 => 1,
        dex if dex < 150 => 2,
        dex if dex < 191 => 4,
        dex if dex < 226 => 5,
        dex if dex < 249 => 6,
        _ => 8,
    }
}

// Max amount of health to gain each level up
pub fn hitdie() -> u8 {
    class().health_bonus() + race().health_bonus()
}

pub fn melee_tohit() -> i16 {
    unsafe {
        player_bth +
            (player_lev as i16 * misc::BTH_LEV_ADJ) +
            (player_ptohit * misc::BTH_PLUS_ADJ)
    }
}

pub fn ranged_tohit() -> i16 {
    unsafe {
        player_bthb +
            (player_lev as i16 * misc::BTH_LEV_ADJ) +
            (player_ptohit * misc::BTH_PLUS_ADJ)
    }
}

pub fn calc_total_points() -> i64 {
    unsafe { C_total_points() }
}

pub fn set_level(level: u8) {
    unsafe { player_lev = level.into() }
}

pub fn level() -> u8 {
    (unsafe { player_lev }) as u8
}

pub fn uid() -> i64 {
    unsafe { player_uid }
}

pub fn is_dead() -> bool {
    unsafe { player_flags.dead != 0 }
}

pub fn increase_save_counter() {
    unsafe { player_save_count += 1 };
}

pub fn player_record() -> PlayerRecord {
    PlayerRecord {
        uid: uid(),
        save_count: unsafe { player_save_count },
        deaths: unsafe { player_deaths },
        xtr_wgt: unsafe { player_xtr_wgt },
        account: unsafe { player_account },
        money: wallet(),
        diffic: unsafe { player_diffic },
        birth: unsafe { player_birth },
        cur_age: unsafe { player_cur_age },
        cur_quest: unsafe { player_cur_quest },
        quests: unsafe { player_quests },
        claim_check: unsafe { player_claim_check },
        play_tm: unsafe { player_play_tm },
        name: name(),
        race: race(),
        sex: sex(),
        class: class(),
        title: title(),
        history: unsafe { player_history }.iter()
            .map(|i| misc::c_i8_array_to_rust_string(i.to_vec()))
            .collect(),
        cheated: unsafe { player_cheated },
        age: unsafe { player_age },
        ht: unsafe { player_ht },
        wt: unsafe { player_wt },
        sc: unsafe { player_sc },
        max_exp: unsafe { player_max_exp },
        exp: unsafe { player_exp },
        rep: unsafe { player_rep },
        premium: unsafe { player_premium },
        lev: level().into(),
        max_lev: unsafe { player_max_lev },
        expfact: unsafe { player_expfact },
        srh: unsafe { player_srh },
        fos: unsafe { player_fos },
        stl: unsafe { player_stl },
        bth: unsafe { player_bth },
        bthb: unsafe { player_bthb },
        mana: unsafe { player_mana },
        cmana: unsafe { player_cmana },
        mhp: unsafe { player_mhp },
        chp: unsafe { player_chp },
        ptohit: unsafe { player_ptohit },
        ptodam: unsafe { player_ptodam },
        pac: unsafe { player_pac },
        ptoac: unsafe { player_ptoac },
        dis_th: unsafe { player_dis_th },
        dis_td: unsafe { player_dis_td },
        dis_ac: unsafe { player_dis_ac },
        dis_tac: unsafe { player_dis_tac },
        disarm: unsafe { player_disarm },
        save: unsafe { player_save },
        hitdie: unsafe { player_hitdie },
        perm_stats: perm_stats(),
        curr_stats: curr_stats(),
        mod_stats: mod_stats(),
        lost_stats: lost_stats(),
        flags: unsafe { player_flags }.to_owned(),
        player: PLAYER.read().unwrap().clone(),
    }
}

pub fn knows_spell(slot: usize) -> bool {
    PLAYER.read().unwrap().spells_known[slot].clone()
}

pub fn set_knows_spell(slot: usize, yn: bool) {
    PLAYER.write().unwrap().spells_known[slot] = yn;
}


pub fn set_player_record(record: PlayerRecord) {
    unsafe {
        player_save_count = record.save_count;
        player_deaths = record.deaths;
        player_xtr_wgt = record.xtr_wgt;
        player_account = record.account;
    }

    set_wallet(&record.money);

    unsafe {
        player_diffic = record.diffic;
        player_birth = record.birth;
        player_cur_age = record.cur_age;
        player_cur_quest = record.cur_quest;
        player_quests = record.quests;
        player_claim_check = record.claim_check;
        player_play_tm = record.play_tm;
    }

    set_name(&record.name);
    set_race(record.race);
    set_sex(record.sex);
    set_class(record.class);
    refresh_title();

    for (i, line) in record.history.iter().enumerate() {
        let cstr = CString::new(line.to_string()).unwrap();
        unsafe { libc::strcpy(player_history[i].as_mut_ptr(), cstr.as_ptr()); }
    }

    unsafe {
        player_cheated = record.cheated;
        player_age = record.age;
        player_ht = record.ht;
        player_wt = record.wt;
        player_sc = record.sc;
        player_max_exp = record.max_exp;
        player_exp = record.exp;
        player_rep = record.rep;
        player_premium = record.premium;
    }

    set_level(record.lev as u8);

    unsafe {
        player_max_lev = record.max_lev;
        player_expfact = record.expfact;
        player_srh = record.srh;
        player_fos = record.fos;
        player_stl = record.stl;
        player_bth = record.bth;
        player_bthb = record.bthb;
        player_mana = record.mana;
        player_cmana = record.cmana;
        player_mhp = record.mhp;
        player_chp = record.chp;
        player_ptohit = record.ptohit;
        player_ptodam = record.ptodam;
        player_pac = record.pac;
        player_ptoac = record.ptoac;
        player_dis_th = record.dis_th;
        player_dis_td = record.dis_td;
        player_dis_ac = record.dis_ac;
        player_dis_tac = record.dis_tac;
        player_disarm = record.disarm;
        player_save = record.save;
        player_hitdie = record.hitdie;
    }

    set_perm_stats(&record.perm_stats);
    set_curr_stats(&record.curr_stats);
    set_mod_stats(&record.mod_stats);
    set_lost_stats(&record.lost_stats);

    unsafe {
        player_flags = record.flags;
    }

    mem::replace(&mut *PLAYER.write().unwrap(), record.player);
}


#[no_mangle]
pub extern fn C_player_knows_spell(slot: libc::int32_t) -> libc::uint8_t {
    match knows_spell(slot as usize) {
        true => 255,
        false => 0,
    }
}

#[no_mangle]
pub extern fn C_player_set_knows_spell(slot: libc::int32_t, yn: libc::uint8_t) {
    set_knows_spell(slot as usize, yn != 0);
}

pub fn uses_magic(magic: Magic) -> bool {
    match class() {
        Class::Mage | Class::Adventurer => magic == Magic::Arcane,
        Class::Priest | Class::Paladin => magic == Magic::Divine,
        Class::Druid | Class::Ranger => magic == Magic::Nature,
        Class::Bard | Class::Rogue => magic == Magic::Song,
        Class::Monk => magic == Magic::Chakra,
        Class::Warrior => false,
    }
}

#[no_mangle]
pub extern fn C_player_uses_magic(magic_type: libc::int32_t) -> libc::uint8_t {
    match uses_magic(Magic::from(magic_type)) {
        true => 255,
        false => 0
    }
}

