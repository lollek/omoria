use std::cmp::{max, min};
use std::convert::TryInto;
use std::ffi::CString;
use std::ptr::addr_of_mut;
use std::sync::RwLock;

use crate::conversion::{class, currency, race, sex};
use crate::data;
use crate::data::class::calculate_tohit_bonus_for_weapon_type;
use crate::misc;
use crate::model::{Ability, Class, Currency, GameTime, Item, Player, PlayerFlags, PlayerRecord, Race, Sex, Stat, Time, Wallet};
use crate::player;
use crate::player::ac_from_dex;
use crate::player_action::attack::{AttackType, MeleeAttackType};
use crate::rng;
use crate::{constants, equipment};

extern "C" {
    pub(super) static mut player_money: [i64; 7]; /* { Money on person	} */
    pub(super) static mut player_play_tm: Time; /* { Time spent in game	} */
    pub(super) static mut player_max_exp: i64; /* { Max experience} */
    pub(super) static mut player_max_lev: u16; /* { Max level explored} */
    pub(super) static mut player_cheated: u8; /*{ gone into wizard or god mode} */
    pub(super) static mut player_quests: u8; /* { # completed } {FUBAR} */
    pub(super) static mut player_cur_quest: u16; /* { creature # of quest } {FUBAR} */
    pub(super) static mut player_birth: GameTime; /* {Date of char's birth} */
    pub(super) static mut player_cur_age: GameTime; /* {Current game date	} */
    pub(super) static mut player_flags: PlayerFlags;
    pub static mut player_history: [[libc::c_char; 82]; 5];
    static mut player_name: [libc::c_char; 82];
    static mut player_race: [libc::c_char; 82];
    static mut player_prace: u8;
    static mut player_sex: [libc::c_char; 82];
    static mut player_tclass: [libc::c_char; 82];
    static mut player_pclass: libc::c_int;
    pub static mut player_stl: i16;
    pub static mut player_sc: i16; /* { Social Class	} */
    pub static mut player_age: u16; /* { Characters age} */
    pub static mut player_ht: u16; /* { Height	} */
    pub static mut player_wt: u16; /* { Weight	} */
    pub static mut player_lev: u16; /* { Level		} */
    pub static mut player_fos: i16; /* { Frenq of search} */
    pub static mut player_bthb: i16; /* { BTH with bows	} */
    pub static mut player_mana: i16; /* { Mana points	} */
    pub static mut player_disarm: i16; /* { % to Disarm	} */
    pub static mut player_save: i16; /* { Saving throw	} */
    pub static mut player_expfact: libc::c_float; /* { Experience factor} */
    pub static mut player_cmana: libc::c_float; /* { Cur mana pts  } */
    pub static mut player_exp: i64; /* { Cur experienc	} */
    pub static mut player_account: i64; /* { Money in the bank	} */
    pub static mut player_mr: i64; /* { mag.res.lev.delta } */
    pub static mut player_rep: i64; /* { XP from good creatures } */
    pub static mut player_claim_check: i64; /* used to track trading post */
    pub static mut inven_weight: libc::c_long; /* Inventory carry weight */
    pub static mut player_creation_time: libc::time_t; /* used as key in master file */
    static mut player_uid: i64; /* Used in master file */

    pub static mut char_row: libc::c_long;
    pub static mut char_col: libc::c_long;

    static mut bank: [i64; 7];
    static exp_per_level: [libc::c_long; constants::MAX_PLAYER_LEVEL + 1];
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
    unsafe {
        libc::strcpy(addr_of_mut!(player_name) as *mut i8, cstr.as_ptr());
    }
}

pub fn race() -> Race {
    race::from_usize(unsafe { player_prace } as usize).unwrap()
}

pub fn set_race(race: Race) {
    let cstr = CString::new(data::race::name(&race)).unwrap();
    unsafe {
        player_prace = race as u8;
        libc::strcpy(addr_of_mut!(player_race) as *mut i8, cstr.as_ptr());
    }
}

pub fn sex() -> Sex {
    sex::from_char(unsafe { player_sex[0] as u8 as char }).unwrap()
}

pub fn set_sex(sex: Sex) {
    let cstr = CString::new(data::sex::name(&sex)).unwrap();
    unsafe {
        libc::strcpy(addr_of_mut!(player_sex) as *mut i8, cstr.as_ptr());
    }
}

pub fn class() -> Class {
    class::from_usize(unsafe { player_pclass }.try_into().unwrap()).unwrap()
}

pub fn set_class(class: Class) {
    let cstr = CString::new(data::class::name(&class)).unwrap();
    unsafe {
        player_pclass = class as i32;
        libc::strcpy(addr_of_mut!(player_tclass) as *mut i8, cstr.as_ptr());
    }
}

pub fn roll_hp_for_levelup() -> i16 {
    rng::randint(hitdie() as i64) as i16
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
    unsafe { player_money }.into()
}

pub fn set_wallet(wallet: &Wallet) {
    for currency in Currency::iter() {
        let position = currency::to_usize(currency);
        unsafe { player_money[position] = wallet.get_pos(currency) };
    }
    unsafe { player_money[0] = wallet.total };
}

pub fn bank_wallet() -> Wallet {
    unsafe { bank }.into()
}

pub fn set_bank_wallet(wallet: &Wallet) {
    for currency in Currency::iter() {
        let position = currency::to_usize(currency);
        unsafe { bank[position] = wallet.get_pos(currency) };
    }
    unsafe { bank[0] = wallet.total };
}

fn rage_rounds_from_level() -> i16 {
    ((level() - 1) * 2) as i16
}

// Max amount of health to gain each level up
pub fn hitdie() -> u8 {
    data::class::health_bonus(&class())
}

#[no_mangle]
fn player_bth() -> i16 {
    base_to_hit()
}

pub fn base_to_hit() -> i16 {
    let mut bth: i16 = level() as i16;
    bth += data::race::melee_bonus(&race()) as i16;
    bth += ((data::class::melee_bonus(&class()) * 5) + 20) as i16;
    unsafe {
        if player_flags.shero > 0 {
            bth += 24;
        }
        if player_flags.hero > 0 {
            bth += 12;
        }
        if player_flags.blessed > 0 {
            bth += 5;
        }
    }
    bth
}

pub fn player_main_weapon<'a>() -> &'a Item {
    unsafe { &*equipment::get_item(equipment::Slot::Primary) }
}

#[no_mangle]
pub fn player_ptohit() -> i16 {
    plus_to_hit(AttackType::Melee(MeleeAttackType::Standard), player_main_weapon())
}

pub fn plus_to_hit(attack_type: AttackType, weapon: &Item) -> i16 {
    let mut plus_to_hit: i16 = player::tohit_from_stats();
    equipment::items_iter().for_each(|item| {
        plus_to_hit += item.tohit;
    });
    if max_bulk() < weapon.weight {
        plus_to_hit -= (max_bulk() as i16 - weapon.weight as i16) / 10
    };
    if attack_type == AttackType::Melee(MeleeAttackType::Backstab) {
        plus_to_hit += level() as i16 / 4
    };
    plus_to_hit +=
        calculate_tohit_bonus_for_weapon_type(&player::class(), weapon.item_type())
            as i16;
    plus_to_hit
}

#[no_mangle]
pub fn player_ptodam() -> i16 {
    let mut plus_to_dam: i16 = player::dmg_from_str();
    equipment::items_iter().for_each(|item| {
        plus_to_dam += item.todam;
    });
    plus_to_dam
}

pub fn plus_to_damage() -> i16 {
    player_ptodam()
}

#[no_mangle]
pub fn player_pac() -> i16 {
    base_ac()
}

pub fn base_ac() -> i16 {
    let mut ac = ac_from_dex();
    equipment::items_iter().for_each(|item| {
        ac += item.ac;
    });
    unsafe {
        if player_flags.invuln > 0 {
            ac += 100;
        }
        if player_flags.blessed > 0 {
            ac += 5;
        }
    }
    ac
}

#[no_mangle]
pub fn player_ptoac() -> i16 {
    plus_to_ac()
}

pub fn plus_to_ac() -> i16 {
    let mut ac = 0;
    equipment::items_iter().for_each(|item| {
        ac += item.toac;
    });
    ac
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
        history: unsafe { player_history }
            .iter()
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
        bthb: unsafe { player_bthb },
        mana: unsafe { player_mana },
        cmana: unsafe { player_cmana },
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
    PLAYER
        .try_read()
        .unwrap()
        .spells_known
        .iter()
        .any(|&known| known)
}

pub fn knows_spell(slot: usize) -> bool {
    PLAYER.try_read().unwrap().spells_known[slot].clone()
}

pub fn set_knows_spell(slot: usize, yn: bool) {
    PLAYER.try_write().unwrap().spells_known[slot] = yn;
}

pub fn max_rage_rounds() -> u8 {
    max(
        0,
        4 + rage_rounds_from_level() + player::rage_rounds_from_con(),
    ) as u8
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
    data::class::abilities(&class())
        .into_iter()
        .chain(data::race::abilities(&race()).into_iter())
        .collect()
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
        unsafe {
            libc::strcpy(player_history[i].as_mut_ptr(), cstr.as_ptr());
        }
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
        player_bthb = record.bthb;
        player_mana = record.mana;
        player_cmana = record.cmana;
        player_disarm = record.disarm;
        player_save = record.save;
        inven_weight = record.inven_weight;
    }

    unsafe {
        player_flags = record.flags;
    }

    *PLAYER.try_write().unwrap() = record.player;

    unsafe {
        char_row = record.char_row;
        char_col = record.char_col;
    }
}

pub fn current_hp() -> i16 {
    PLAYER.try_read().unwrap().current_hp as i16
}

pub fn reset_current_hp() {
    let max_hp = PLAYER.try_read().unwrap().max_hp;
    PLAYER.try_write().unwrap().current_hp = max_hp.into();
}

pub fn modify_current_hp(amount: f32) {
    PLAYER.try_write().unwrap().current_hp += amount;
}

fn set_max_hp(new_value: i16) {
    let hp_modifier = new_value - PLAYER.try_read().unwrap().max_hp_last_calc;
    {
        let mut player = PLAYER.try_write().unwrap();
        player.max_hp_last_calc = new_value;
        player.max_hp = new_value;
    }
    modify_current_hp(hp_modifier as f32);
}

pub fn max_hp() -> i16 {
    let max_hp = PLAYER.try_read().unwrap().max_hp;
    let new_max_hp = max_hp + (player::hp_from_con() * level() as i16);

    let hp_modifier = new_max_hp - PLAYER.try_read().unwrap().max_hp_last_calc;
    PLAYER.try_write().unwrap().max_hp_last_calc = new_max_hp;
    modify_current_hp(hp_modifier as f32);

    new_max_hp
}

pub fn modify_max_hp(amount: i16) {
    let max_hp = PLAYER.try_read().unwrap().max_hp;
    set_max_hp(max(1, max_hp + amount));
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
        <i64>::MAX
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

pub fn modify_extra_bulk_carry(modifier: i16) {
    PLAYER.try_write().unwrap().extra_bulk_carry += modifier;
}

pub fn extra_bulk_carry() -> i16 {
    PLAYER.try_read().unwrap().extra_bulk_carry
}

pub fn current_bulk() -> u16 {
    (unsafe { inven_weight }) as u16 / 100
}

pub fn max_bulk() -> u16 {
    let min_base_bulk = 3000;
    let player_weight_modifier = 13;
    // [60-280]
    let player_carry_base_amount = (30 + (player::curr_stats().strength * 10)) as u16;
    let base_bulk = min(
        (player_carry_base_amount * player_weight_modifier) + current_weight(),
        min_base_bulk);
    let max_bulk: i32 = base_bulk as i32 + extra_bulk_carry() as i32;
    max_bulk.clamp(0, u16::MAX.into()) as u16
}

pub fn set_birthdate(new_value: GameTime) {
    unsafe {
        player_birth = new_value;
    }
}

pub fn birthdate() -> GameTime {
    unsafe { player_birth }
}

pub fn set_age(new_value: GameTime) {
    unsafe {
        player_cur_age = new_value;
    }
}

pub fn age() -> GameTime {
    unsafe { player_cur_age }
}

pub fn regen_hp(percent: f32) {
    let player_regen_hpbase = 0.022;
    modify_current_hp(player_regen_hpbase + max_hp() as f32 * percent);
}
