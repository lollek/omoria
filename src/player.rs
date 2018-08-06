use libc;
use std::ffi::CString;

use types::{
    Class, StatBlock, stats_iter, Wallet, currencies_iter, Race, Sex, Stat
};

use debug;
use misc;

#[repr(C)]
pub struct GameTime {
    pub year: libc::int64_t,
    pub month: libc::uint8_t,
    pub day: libc::uint8_t,
    pub hour: libc::uint8_t,
    pub secs: libc::uint16_t,
}

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

#[repr(C)]
pub struct p_flags {
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
    pub static mut player_flags: p_flags;
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
    static mut player_stats_perm: [libc::uint8_t; 6];
    static mut player_stats_curr: [libc::uint8_t; 6];
    pub static mut player_stats_mod: [libc::int8_t; 6];
    pub static mut player_stats_lost: [libc::uint8_t; 6];

    static mut bank: [libc::int64_t; 7];
}

pub fn name() -> String {
    let string: Vec<u8> = unsafe { player_name }.iter().map(|&i| i as u8).collect();
    misc::c_array_to_rust_string(string)
}

pub fn set_name(new_name: &str) {
    unsafe {
        libc::strcpy(player_name.as_mut_ptr() as *mut i8, CString::new(new_name).unwrap().as_ptr());
    }
}

pub fn race() -> Race {
    debug::enter("player::race");
    let result = Race::from(unsafe { player_prace } as usize);
    debug::leave("player::race");
    result
}

pub fn set_race(race: Race) {
    debug::enter("player::set_race");
    unsafe {
        player_prace = race as u8;
        libc::strcpy(player_race.as_mut_ptr(), CString::new(race.name()).unwrap().as_ptr());
    }
    debug::leave("player::set_race");
}

pub fn sex() -> Sex {
    Sex::from(unsafe { player_sex[0] as u8 as char })
}

pub fn set_sex(sex: Sex) {
    unsafe {
        libc::strcpy(player_sex.as_mut_ptr(), CString::new(sex.to_string()).unwrap().as_ptr());
    }
}

pub fn class() -> Class {
    debug::enter("player::class");
    let result = Class::from(unsafe { player_pclass } as usize);
    debug::enter("player::class");
    result
}

pub fn set_class(class: Class) {
    debug::enter("player::set_class");
    unsafe {
        player_pclass = class as i32;
        libc::strcpy(player_tclass.as_mut_ptr(), CString::new(class.name()).unwrap().as_ptr());
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

pub fn wallet() -> Wallet {
    Wallet::from(unsafe { player_money })
}

pub fn set_wallet(wallet: &Wallet) {
    for currency in currencies_iter() {
        unsafe { player_money[currency] = wallet.get_pos(currency) };
    }
}

pub fn bank_wallet() -> Wallet {
    Wallet::from(unsafe { bank })
}

pub fn set_bank_wallet(wallet: &Wallet) {
    for currency in currencies_iter() {
        unsafe { bank[currency] = wallet.get_pos(currency) };
    }
}

pub fn refresh_title() {
    let new_title = class().title(unsafe { player_lev } as u8);
    unsafe {
        libc::strcpy(player_title.as_mut_ptr(), CString::new(new_title).unwrap().as_ptr());
    }
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
