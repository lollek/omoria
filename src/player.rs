use libc::{time_t};
use std::ffi::CStr;
use types::{StatBlock, stats_iter, Wallet, currencies_iter};

#[repr(C)]
pub struct p_flags {
    pub insured: u8 ,      /* { Character insured   } */
    pub dead: u8 ,	 /* { Currently restored  } */
    pub status: u64, /* { Status of player    } */
    pub rest: i64 ,	    /* { Rest counter	 } */
    pub blind: i64 ,	   /* { Blindness counter   } */
    pub paralysis: i64 ,       /* { Paralysis counter   } */
    pub confused: i64 ,	/* { Confusion counter   } */
    pub foodc: i64 ,	   /* { Food counter        } (was just food) */
    pub food_digested: i64 ,   /* { Food per round      } */
    pub protection: i64 ,      /* { Protection fr. evil } */
    pub speed: i64 ,	   /* { Cur speed adjust    } */
    pub speed_paral: i64 ,     /* { Slow speed adjust   } */
    pub speed_flag: u8 ,   /* { On if reset speed   } */
    pub paral_init: i64 ,      /* { Init val for slow   } */
    pub move_rate: i64 ,       /* { move_rate	         } */
    pub swim: i64 ,	    /* { Cur swim adjust     } */
    pub fast: i64 ,	    /* { Temp speed change   } */
    pub slow: i64 ,	    /* { Temp speed change   } */
    pub petrification: i64 ,   /* { Amount Petrified    } */
    pub afraid: i64 ,	  /* { Fear                } */
    pub poisoned: i64 ,	/* { Poisoned            } */
    pub image: i64 ,	   /* { Hallucinate         } */
    pub protevil: i64 ,	/* { Protect VS evil     } */
    pub invuln: i64 ,	  /* { Increases AC        } */
    pub hero: i64 ,	    /* { Heroism	         } */
    pub shero: i64 ,	   /* { Super Heroism	 } */
    pub blessed: i64 ,	 /* { Blessed	         } */
    pub resist_heat: i64 ,     /* { Timed heat resist   } */
    pub resist_cold: i64 ,     /* { Timed cold resist   } */
    pub detect_inv: i64 ,      /* { Timed see invisible } */
    pub word_recall: i64 ,     /* { Timed teleport level} */
    pub see_infra: i64 ,       /* { See warm creatures  } */
    pub tim_infra: i64 ,       /* { Timed infra vision  } */
    pub see_inv: u8 ,      /* { Can see invisible   } */
    pub teleport: u8 ,     /* { Random teleportation} */
    pub free_act: u8 ,     /* { Never paralyzed     } */
    pub slow_digest: u8 ,  /* { Lower food needs    } */
    pub aggravate: u8 ,    /* { Agravate monsters   } */
    pub fire_resist: u8 ,  /* { Resistance to fire  } */
    pub cold_resist: u8 ,  /* { Resistance to cold  } */
    pub acid_resist: u8 ,  /* { Resistance to acid  } */
    pub hunger_item: u8 ,  /* { Resets food counter } */
    pub regenerate: u8 ,   /* { Regenerate hit pts  } */
    pub lght_resist: u8 ,  /* { Resistance to light } */
    pub ffall: u8 ,	/* { No damage falling   } */
    pub sustain: [u8; 6], /* { keep characteristic } */
    pub confuse_monster: u8 ,	   /* { Glowing hands...    } */
    pub resist_lght: i64 ,		   /* { Timed lighting rst  } */
    pub free_time: i64 ,			   /* { Timed free action   } */
    pub ring_fire: i64 ,			   /* { Timed fire spell    } */
    pub protmon: i64 ,			   /* { Timed monst prot    } */
    pub hoarse: i64 ,			   /* { Timed no-bard spells} */
    pub magic_prot: i64 ,		   /* { Timed magic prot    } */
    pub ring_ice: i64 ,			   /* { Timed cold spell    } */
    pub temp_stealth: i64 ,		   /* { Timed stealth       } */
    pub resist_petri: i64 ,		   /* { Timed resist petrify} */
    pub blade_ring: i64 ,		   /* { Timed blade spell   } */
    pub petri_resist: u8 ,		   /* { Resist Petrification} */
    pub quested: u8 ,		   /* { Performing a Quest  } {FUBAR} */
    pub light_on: u8 ,		   /* { Light source is active } */
    pub resting_till_full: u8 ,
}

extern "C" {
    pub static mut player_flags: p_flags;
    static mut player_name: [u8; 82];
    static mut player_race: [u8; 82];
    static mut player_sex: [u8; 82];
    static mut player_tclass: [u8; 82];
    pub static mut player_prace: u8;
    pub static mut player_stl: u8;
    pub static mut player_sc: i16;
    pub static mut player_age: u16;
    pub static mut player_ht: u16;
    pub static mut player_wt: u16;
    pub static mut player_lev: u16;
    pub static mut player_mhp: i16;
    pub static mut player_fos: i16;
    pub static mut player_bth: i16;
    pub static mut player_bthb: i16;
    pub static mut player_srh: i16;
    pub static mut player_chp: f32;
    pub static mut player_mana: i16;
    pub static mut player_pac: i16;
    pub static mut player_ptoac: i16;
    pub static mut player_ptohit: i16;
    pub static mut player_ptodam: i16;
    pub static mut player_save: i16;
    pub static mut player_cmana: f32;
    pub static mut player_diffic: u8;
    pub static mut player_hitdie: u8;
    pub static mut player_exp: i64;
    pub static mut player_expfact: f32;
    pub static mut player_account: i64;
    pub static mut player_rep: i64;
    pub static mut player_dis_ac: i16;
    pub static mut player_dis_th: i16;
    pub static mut player_dis_td: i16;
    pub static mut player_dis_tac: i16;
    pub static mut player_claim_check: i64;
    pub static mut player_save_count: i64;
    pub static mut player_creation_time: time_t;
    static mut player_stats_perm: [u8; 6];
    static mut player_stats_curr: [u8; 6];
    static mut player_money: [i64; 7];
    static mut bank: [i64; 7];
}

fn c_array_to_rust_string(array: [u8; 82]) -> String {
    let safe_array = array.to_owned()
        .iter_mut()
        .take_while(|i| i != &&0u8)
        .chain([0u8].iter_mut())
        .map(|i| i.to_owned())
        .collect::<Vec<u8>>();

    CStr::from_bytes_with_nul(&safe_array)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

pub fn name() -> String {
    c_array_to_rust_string(unsafe { player_name })
}

pub fn race_string() -> String {
    c_array_to_rust_string(unsafe { player_race })
}

pub fn sex_string() -> String {
    c_array_to_rust_string(unsafe { player_sex })
}

pub fn class_string() -> String {
    c_array_to_rust_string(unsafe { player_tclass })
}

pub fn perm_stats() -> StatBlock {
    StatBlock::from(unsafe { player_stats_perm })
}

pub fn set_perm_stats(block: &StatBlock) {
    for stat in stats_iter() {
        unsafe { player_stats_perm[stat] = block.get_pos(stat) };
    }
}

pub fn curr_stats() -> StatBlock {
    StatBlock::from(unsafe { player_stats_curr })
}

pub fn set_curr_stats(block: &StatBlock) {
    for stat in stats_iter() {
        unsafe { player_stats_curr[stat] = block.get_pos(stat) };
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

