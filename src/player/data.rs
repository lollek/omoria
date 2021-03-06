use libc;
use std::mem;
use std::cmp::{ min, max };

use std::ffi::CString;
use std::sync::RwLock;

use types::{
    Class, StatBlock, Wallet, currencies_iter, Race, Sex, Stat,
    Magic, GameTime, Ability
};

use player;
use random;
use misc;
use debug;

const MAX_PLAYER_LEVEL: usize = 40;

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
    pub spells_known: Vec<bool>,
    pub rage_rounds_spent: u8,
    pub is_raging: bool,
    pub rage_exhaustion_rounds_left: u8,
    pub curr_stats: StatBlock,
    pub lost_stats: StatBlock,
    pub mod_stats: StatBlock,
    pub perm_stats: StatBlock,
    pub save_counter: u64,
    pub extra_bulk_carry: u16,
    pub search_modifier: i16,
    pub max_hp_last_calc: i16, // Last time we checked, what was max hp?
    pub max_hp: i16,
    pub current_hp: f32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            spells_known: [false; 40].to_vec(),
            rage_rounds_spent: 0,
            is_raging: false,
            rage_exhaustion_rounds_left: 0,
            curr_stats: StatBlock::new(0),
            lost_stats: StatBlock::new(0),
            mod_stats: StatBlock::new(0),
            perm_stats: StatBlock::new(0),
            save_counter: 0,
            extra_bulk_carry: 0,
            search_modifier: 0,
            max_hp_last_calc: 0,
            max_hp: 0,
            current_hp: 0.0,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[repr(C)]
pub struct PlayerRecord {
    pub uid: libc::int64_t,
    pub account: libc::int64_t,
    pub money: Wallet,
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
    pub history: Vec<String>,
    pub cheated: libc::uint8_t,
    pub age: libc::uint16_t,
    pub ht: libc::uint16_t,
    pub wt: libc::uint16_t,
    pub sc: libc::int16_t,
    pub max_exp: libc::int64_t,
    pub exp: libc::int64_t,
    pub rep: libc::int64_t,
    pub lev: libc::uint16_t,
    pub max_lev: libc::uint16_t,
    pub expfact: libc::c_float,
    pub fos: libc::int16_t,
    pub stl: libc::int16_t,
    pub bth: libc::int16_t,
    pub bthb: libc::int16_t,
    pub mana: libc::int16_t,
    pub cmana: libc::c_float,
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
    pub inven_weight: libc::c_long,
    pub flags: PlayerFlags,
    pub player: Player,
    pub char_row: libc::c_long,
    pub char_col: libc::c_long,
}

extern "C" {
    pub(super) static mut player_money: [libc::int64_t; 7] ; /* { Money on person	} */
    pub(super) static mut player_play_tm: Time ;/* { Time spent in game	} */
    pub(super) static mut player_max_exp: libc::int64_t ;  /* { Max experience} */
    pub(super) static mut player_max_lev: libc::uint16_t ;   /* { Max level explored} */
    pub(super) static mut player_cheated: libc::uint8_t ;  /*{ gone into wizard or god mode} */
    pub(super) static mut player_quests: libc::uint8_t ;     /* { # completed } {FUBAR} */
    pub(super) static mut player_cur_quest: libc::uint16_t ; /* { creature # of quest } {FUBAR} */
    pub(super) static mut player_birth: GameTime;     /* {Date of char's birth} */
    pub(super) static mut player_cur_age: GameTime;   /* {Current game date	} */
    pub(super) static mut player_flags: PlayerFlags;
    pub static mut player_history: [[libc::c_char; 82]; 5];
    static mut player_name: [libc::c_char; 82];
    static mut player_race: [libc::c_char; 82];
    static mut player_prace: libc::uint8_t;
    static mut player_sex: [libc::c_char; 82];
    static mut player_tclass: [libc::c_char; 82];
    static mut player_pclass: libc::c_int;
    pub static mut player_stl: libc::int16_t;
    pub static mut player_sc: libc::int16_t ;		  /* { Social Class	} */
    pub static mut player_age: libc::uint16_t ;       /* { Characters age} */
    pub static mut player_ht: libc::uint16_t ;	/* { Height	} */
    pub static mut player_wt: libc::uint16_t ;	/* { Weight	} */
    pub static mut player_lev: libc::uint16_t ;       /* { Level		} */
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
    pub static mut player_expfact: libc::c_float ;		  /* { Experience factor} */
    pub static mut player_cmana: libc::c_float ;		  /* { Cur mana pts  } */
    pub static mut player_exp: libc::int64_t ;		  /* { Cur experienc	} */
    pub static mut player_account: libc::int64_t ;		  /* { Money in the bank	} */
    pub static mut player_mr: libc::int64_t  ;		  /* { mag.res.lev.delta } */
    pub static mut player_rep: libc::int64_t ;		  /* { XP from good creatures } */
    pub static mut player_claim_check: libc::int64_t ;	 /* used to track trading post */
    pub static mut inven_weight: libc::c_long; /* Inventory carry weight */
    pub static mut player_creation_time: libc::time_t ;     /* used as key in master file */
    static mut player_uid: libc::int64_t;	/* Used in master file */

    pub static mut char_row: libc::c_long;
    pub static mut char_col: libc::c_long;

    static mut bank: [libc::int64_t; 7];
    static exp_per_level: [libc::c_long; MAX_PLAYER_LEVEL + 1];
    #[link_name = "gain_level"]
    fn C_gain_level();
}

lazy_static! {
    pub(super) static ref PLAYER: RwLock<Player> = RwLock::new(Player::new());
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


pub fn roll_hp_for_levelup() -> i16 {
    random::randint(hitdie() as i64) as i16
}

pub fn mod_stat(stat: Stat, modifier: i16) {
    let mut stats = PLAYER.try_write().unwrap().mod_stats;
    let old_stat = stats.get(stat);
    stats.set(stat, old_stat + modifier);
}

pub fn mod_perm_stat(stat: Stat, modifier: i16) {
    let mut stats = PLAYER.try_write().unwrap().perm_stats;
    let old_stat = stats.get(stat);
    stats.set(stat, old_stat + modifier);
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

fn rage_rounds_from_level() -> i16 {
    ((level() - 1) * 2) as i16
}

// Max amount of health to gain each level up
pub fn hitdie() -> u8 {
    class().health_bonus()
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
    (1000 * deepest_level() as i64) + exp()
}

pub fn set_level(level: u8) {
    unsafe { player_lev = level.into() }
}

pub fn level() -> u8 {
    (unsafe { player_lev }) as u8
}

pub fn set_uid(val: i64) {
    unsafe { player_uid = val };
}

pub fn uid() -> i64 {
    unsafe { player_uid }
}

pub fn increase_save_counter() {
    PLAYER.try_write().unwrap().save_counter += 1;
}

pub fn max_exp() -> i64 {
    unsafe { player_max_exp }
}

pub fn exp() -> i64 {
    unsafe { player_exp }
}

pub fn deepest_level() -> u8 {
    (unsafe { player_max_lev }) as u8
}

pub fn record() -> PlayerRecord {
    PlayerRecord {
        uid: uid(),
        account: unsafe { player_account },
        money: wallet(),
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
        lev: level().into(),
        max_lev: unsafe { player_max_lev },
        expfact: unsafe { player_expfact },
        fos: unsafe { player_fos },
        stl: unsafe { player_stl },
        bth: unsafe { player_bth },
        bthb: unsafe { player_bthb },
        mana: unsafe { player_mana },
        cmana: unsafe { player_cmana },
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
        inven_weight: unsafe { inven_weight },
        flags: unsafe { player_flags }.to_owned(),
        player: PLAYER.try_read().unwrap().clone(),
        char_row: unsafe { char_row },
        char_col: unsafe { char_col },
    }
}

pub fn knows_any_spell() -> bool {
    PLAYER.try_read().unwrap().spells_known.iter().any(|&known| known)
}

pub fn knows_spell(slot: usize) -> bool {
    PLAYER.try_read().unwrap().spells_known[slot].clone()
}

pub fn set_knows_spell(slot: usize, yn: bool) {
    PLAYER.try_write().unwrap().spells_known[slot] = yn;
}

pub fn max_rage_rounds() -> u8 {
    max(0, 4 + rage_rounds_from_level() + player::rage_rounds_from_con()) as u8
}

pub fn rage_rounds_spent() -> u8 {
    PLAYER.try_read().unwrap().rage_rounds_spent
}

pub fn set_rage_rounds_spent(new_value: u8) {
    PLAYER.try_write().unwrap().rage_rounds_spent = new_value;
}

pub fn rage_exhaustion_rounds_left() -> u8 {
    PLAYER.try_read().unwrap().rage_exhaustion_rounds_left
}

pub fn set_rage_exhaustion_rounds_left(new_value: u8) {
    PLAYER.try_write().unwrap().rage_exhaustion_rounds_left = new_value;
}

pub fn abilities() -> Vec<Ability> {
    class().abilities().into_iter().chain(race().abilities().into_iter()).collect()
}

pub fn set_record(record: PlayerRecord) {
    unsafe {
        player_account = record.account;
    }

    set_wallet(&record.money);

    unsafe {
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
    }

    set_level(record.lev as u8);

    unsafe {
        player_max_lev = record.max_lev;
        player_expfact = record.expfact;
        player_fos = record.fos;
        player_stl = record.stl;
        player_bth = record.bth;
        player_bthb = record.bthb;
        player_mana = record.mana;
        player_cmana = record.cmana;
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
        inven_weight = record.inven_weight;
    }

    unsafe {
        player_flags = record.flags;
    }

    mem::replace(&mut *PLAYER.try_write().unwrap(), record.player);

    unsafe {
        char_row = record.char_row;
        char_col = record.char_col;
    }
}


pub fn uses_magic(magic: Magic) -> bool {
    match class() {
        Class::Wizard | Class::Adventurer => magic == Magic::Arcane,
        Class::Cleric | Class::Paladin => magic == Magic::Divine,
        Class::Druid | Class::Ranger => magic == Magic::Nature,
        Class::Bard | Class::Rogue => magic == Magic::Song,
        Class::Monk => magic == Magic::Chakra,
        Class::Fighter => false,
        Class::Barbarian => false,
    }
}

pub fn current_hp() -> i16 {
    PLAYER.try_read().unwrap().current_hp as i16
}

pub fn reset_current_hp() {
    debug::enter("player::reset_current_hp");
    let max_hp = PLAYER.try_read().unwrap().max_hp;
    PLAYER.try_write().unwrap().current_hp = max_hp.into();
    debug::leave("player::reset_current_hp");
}

pub fn modify_current_hp(amount: f32) {
    debug::enter("player::modify_current_hp");
    PLAYER.try_write().unwrap().current_hp += amount;
    debug::leave("player::modify_current_hp");
}

fn set_max_hp(new_value: i16) {
    debug::enter("player::set_max_hp");
    let hp_modifier = new_value - PLAYER.try_read().unwrap().max_hp_last_calc;
    {
        let mut player = PLAYER.try_write().unwrap();
        player.max_hp_last_calc = new_value;
        player.max_hp = new_value;
    }
    modify_current_hp(hp_modifier as f32);
    debug::leave("player::set_max_hp");
}

pub fn max_hp() -> i16 {
    debug::enter("player::max_hp");
    let max_hp = PLAYER.try_read().unwrap().max_hp;
    let new_max_hp = max_hp + (player::hp_from_con() * level() as i16);

    let hp_modifier = new_max_hp - PLAYER.try_read().unwrap().max_hp_last_calc;
    PLAYER.try_write().unwrap().max_hp_last_calc = new_max_hp;
    modify_current_hp(hp_modifier as f32);

    debug::leave("player::max_hp");
    return new_max_hp
}

pub fn modify_max_hp(amount: i16) {
    debug::enter("player::modify_max_hp");
    let max_hp = PLAYER.try_read().unwrap().max_hp;
    set_max_hp(max(1, max_hp + amount));
    debug::leave("player::modify_max_hp");
}

pub fn current_mp() -> i16 {
    (unsafe { player_cmana }) as i16
}

pub fn modify_current_mp(amount: f32) {
    unsafe { player_cmana += amount };
}

pub fn max_mp() -> i16 {
    unsafe { player_mana }
}

pub fn modify_max_mp(amount: i16) {
    unsafe { player_mana += amount };
    modify_current_mp(amount as f32);
}

pub fn uses_mana() -> bool {
    class() != Class::Fighter
}

pub fn expfact() -> f32 {
    unsafe { player_expfact }
}

pub fn exp_to_next_level() -> i64 {
    if exp() >= max_exp() {
        <i64>::max_value()
    } else {
        (unsafe { exp_per_level[level() as usize] } as f64 * expfact() as f64) as i64 - exp()
    }
}

pub fn add_experience(num: i64) {
    unsafe { player_exp += num };
    if exp_to_next_level() <= 0 {
        unsafe { C_gain_level() };
    }
}

pub fn quests() -> u8 {
    unsafe { player_quests }
}

pub fn current_weight() -> u16 {
    unsafe { player_wt }
}

pub fn set_extra_bulk_carry(new_value: u16) {
    PLAYER.try_write().unwrap().extra_bulk_carry = new_value;
}

pub fn extra_bulk_carry() -> u16 {
    PLAYER.try_read().unwrap().extra_bulk_carry
}

pub fn current_bulk() -> u16 {
    (unsafe { inven_weight }) as u16 / 100
}

pub fn max_bulk() -> u16 {
    let player_weight_modifier = 13;
    min((30 + (player::curr_stats().strength * 10)) as u16 * player_weight_modifier
        + current_weight(), 3000)
        + extra_bulk_carry()
}

pub fn set_birthdate(new_value: GameTime) {
    unsafe { player_birth = new_value; }
}

pub fn birthdate() -> GameTime {
    unsafe { player_birth }
}

pub fn set_age(new_value: GameTime) {
    unsafe { player_cur_age = new_value; }
}

pub fn age() -> GameTime {
    unsafe { player_cur_age }
}

pub fn regen_hp(percent: f32) {
    let player_regen_hpbase = 0.022;
    modify_current_hp(player_regen_hpbase + max_hp() as f32 * percent);
}
