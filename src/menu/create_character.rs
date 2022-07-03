use std::cmp::{max, min};
use std::ffi::CString;
use std::ptr::null;
use std::str;

use libc::{strcpy, time, time_t};

use conversion;
use data;
use debug;
use io;
use item_template;
use logic::{generate_item, menu};
use misc;
use player;
use random;
use screen;
use term;

use model::{Currency, GameTime, InventoryItem, Item, Race, Sex, Stat, StatBlock};

extern "C" {
    fn add_money(amount: i64);
    fn add_inven_item(item: Item) -> *mut InventoryItem;
}

// returns [3, 18]
fn random_stat() -> i16 {
    let mut stats = vec![
        random::randint(6),
        random::randint(6),
        random::randint(6),
        random::randint(6),
    ];
    stats.sort();
    stats.iter().skip(1).fold(0, |sum, i| sum + i) as i16
}

fn put_character(show_values: bool) {
    debug::enter("create_character::put_character");

    term::prt("Name      : ", 2, 2);
    term::prt("Race      : ", 3, 2);
    term::prt("Sex       : ", 4, 2);
    term::prt("Class     : ", 5, 2);

    if show_values {
        term::prt(player::name(), 2, 14);
        term::prt(data::race::name(&player::race()), 3, 14);
        term::prt(data::sex::name(&player::sex()), 4, 14);
        term::prt(data::class::name(&player::class()), 5, 14);
    }

    debug::leave("create_character::put_character");
}

fn get_money() {
    debug::enter("create_character::get_money");
    let player_stats = player::curr_stats();
    let mut amount: i64 = 325 + random::randint(25)
        // Social Class adj
        + unsafe { player::player_sc } as i64 * 6
        // Stat adj
        - Stat::iter().fold(0, |sum: i64, tstat|
            sum + player_stats.get(tstat) as i64)
        // Charisma adj
        + player_stats.get(Stat::Charisma) as i64;

    // Minimum
    amount = max(amount, 80);

    let gold_value = data::currency::value(&Currency::Gold);
    unsafe { add_money((amount * gold_value) + random::randint(gold_value)) };
    debug::leave("create_character::get_money");
}

fn get_stats() {
    debug::enter("create_character::get_stats");

    let race = player::race();
    let race_stats = data::race::stat_block(&race);

    let new_stats = StatBlock {
        strength: random_stat() + race_stats.get(Stat::Strength),
        intelligence: random_stat() + race_stats.get(Stat::Intelligence),
        wisdom: random_stat() + race_stats.get(Stat::Wisdom),
        dexterity: random_stat() + race_stats.get(Stat::Dexterity),
        constitution: random_stat() + race_stats.get(Stat::Constitution),
        charisma: random_stat() + race_stats.get(Stat::Charisma),
    };

    player::set_perm_stats(new_stats);
    player::recalc_curr_stats();

    unsafe {
        player::player_rep = 0;
        player::player_bth = data::race::melee_bonus(&race) as i16;
        player::player_bthb = data::race::ranged_bonus(&race) as i16;
        player::player_fos = data::race::search_freq(&race) as i16;
        player::player_stl = data::race::stealth_mod(&race) as i16;
        player::player_save = data::race::save_mod(&race) as i16;
        player::player_lev = 1;
        player::player_ptodam = player::dmg_from_str() as i16;
        player::player_ptohit = player::tohit_from_stats() as i16;
        player::player_ptoac = 0;
        player::player_pac = player::ac_from_dex() as i16;
        player::player_expfact = data::race::expfactor(&race);
    }
    player::set_infravision(data::race::infravision(&race) as i64);
    player::set_swim_speed(data::race::swim_speed(&race) as i64);

    debug::leave("create_character::get_stats");
}

fn put_stats() {
    debug::enter("create_character::put_stats");

    screen::print_stats(2, 64);

    term::prt(
        format!("+ To Hit   : {}", unsafe { player::player_dis_th }),
        9,
        3,
    );
    term::prt(
        format!("+ To Damage: {}", unsafe { player::player_dis_td }),
        10,
        3,
    );
    term::prt(
        format!("+ To AC    : {}", unsafe { player::player_dis_tac }),
        11,
        3,
    );
    term::prt(
        format!("  Total AC : {}", unsafe { player::player_dis_ac }),
        12,
        3,
    );

    debug::leave("create_character::put_stats");
}

fn put_misc1() {
    debug::enter("create_character::put_misc1");

    term::prt(
        format!("Age          : {}", unsafe { player::player_age }),
        2,
        39,
    );
    term::prt(
        format!("Height       : {}", unsafe { player::player_ht }),
        3,
        39,
    );
    term::prt(
        format!("Weight       : {}", unsafe { player::player_wt }),
        4,
        39,
    );
    term::prt(
        format!("Social Class : {}", unsafe { player::player_sc }),
        5,
        39,
    );

    debug::leave("create_character::put_misc1");
}

fn put_misc2() {
    debug::enter("create_character::put_misc2");

    term::prt(
        format!("Level      : {}", unsafe { player::player_lev }),
        9,
        30,
    );
    term::prt(
        format!("Experience : {}", unsafe { player::player_exp }),
        10,
        30,
    );
    term::prt(format!("Gold       : {}", player::wallet().total), 11, 30);
    term::prt(
        format!("Account    : {}", unsafe { player::player_account }),
        12,
        30,
    );
    term::prt(format!("Max Hit Points : {}", player::max_hp()), 9, 53);
    term::prt(format!("Cur Hit Points : {}", player::current_hp()), 10, 53);
    term::prt(
        format!("Max Mana       : {}", unsafe { player::player_mana }),
        11,
        53,
    );
    term::prt(
        format!("Cur Mana       : {}", unsafe { player::player_cmana }),
        12,
        53,
    );

    debug::leave("create_character::put_misc2");
}

fn put_misc3() {
    debug::enter("create_character::put_misc3");

    term::clear_from(14);

    let xbth: i64 = player::melee_tohit().into();
    let xbthb: i64 = player::ranged_tohit().into();

    let xfos: i64 = max(27 - unsafe { player::player_fos }, 0).into();
    let xsrh: i64 = player::curr_search_skill().into();
    let xstl: i64 = unsafe { player::player_stl }.into();
    let xdis: i64 = unsafe { player::player_disarm } as i64
        + unsafe { player::player_lev } as i64
        + (2 * player::disarm_from_dex()) as i64
        + player::modifier_from_stat(Stat::Intelligence) as i64;
    let xsave: i64 = unsafe { player::player_save } as i64
        + unsafe { player::player_lev } as i64
        + player::modifier_from_stat(Stat::Wisdom) as i64;
    let xdev: i64 = unsafe { player::player_save } as i64
        + unsafe { player::player_lev } as i64
        + player::modifier_from_stat(Stat::Intelligence) as i64;
    let xswm: i64 = player::swim_speed() + 4;
    let xrep: i64 = 6 + (unsafe { player::player_rep } / 25);
    let xinf: i64 = player::infravision() * 10;

    term::prt("(Miscellaneous Abilities)", 15, 23);
    term::put_buffer(
        format!("Fighting    : {}", misc::mod_to_string(xbth, 12)),
        16,
        1,
    );
    term::put_buffer(
        format!("Bows/Throw  : {}", misc::mod_to_string(xbthb, 12)),
        17,
        1,
    );
    term::put_buffer(
        format!("Saving Throw: {}", misc::mod_to_string(xsave, 6)),
        18,
        1,
    );
    term::put_buffer(
        format!("Stealth     : {}", misc::mod_to_string(xstl, 1)),
        16,
        26,
    );
    term::put_buffer(
        format!("Disarming   : {}", misc::mod_to_string(xdis, 8)),
        17,
        26,
    );
    term::put_buffer(
        format!("Magic Device: {}", misc::mod_to_string(xdev, 7)),
        18,
        26,
    );
    term::put_buffer(
        format!("Perception  : {}", misc::mod_to_string(xfos, 3)),
        16,
        51,
    );
    term::put_buffer(
        format!("Searching   : {}", misc::mod_to_string(xsrh, 6)),
        17,
        51,
    );
    term::put_buffer(format!("Infra-Vision: {} feet", xinf), 18, 51);
    term::put_buffer(
        format!("Swimming    : {}", misc::mod_to_string(xswm, 1)),
        19,
        51,
    );
    term::put_buffer(
        format!("Reputation  : {}", misc::mod_to_string(xrep, 1)),
        19,
        1,
    );

    debug::leave("create_character::put_misc3");
}

fn print_history() {
    let history: String = (0..5)
        .map(|i| {
            let c_hist: Vec<u8> = unsafe { player::player_history[i] }
                .iter()
                .map(|&i| i as u8)
                .collect();
            misc::c_array_to_rust_string(c_hist)
        })
        .collect::<Vec<String>>()
        .join(" ");

    menu::draw_help("Your background", &history);
}

fn apply_stats_from_class() {
    debug::enter("create_character::apply_stats_from_class");

    player::modify_max_hp(player::hitdie() as i16);
    player::reset_current_hp();
    unsafe {
        player::player_bth += ((data::class::melee_bonus(&player::class()) * 5) + 20) as i16;
        player::player_bthb += ((data::class::ranged_bonus(&player::class()) * 5) + 20) as i16;
        player::player_disarm += data::class::disarm_mod(&player::class()) as i16;
        player::player_fos += data::class::search_freq(&player::class()) as i16;
        player::player_stl += data::class::stealth_mod(&player::class()) as i16;
        player::player_save += data::class::save_mod(&player::class()) as i16;
        player::player_expfact += data::class::expfactor(&player::class());
        player::player_mr = data::class::magic_resist(&player::class()).into();

        // Real values
        player::player_ptodam = player::dmg_from_str() as i16;
        player::player_ptohit = player::tohit_from_stats() as i16;
        player::player_ptoac = player::ac_from_dex() as i16;
        player::player_pac = 0;

        // Displayed values
        player::player_dis_td = player::player_ptodam;
        player::player_dis_th = player::player_ptohit;
        player::player_dis_tac = player::player_ptoac;
        player::player_dis_ac = player::player_pac;
    }

    debug::leave("create_character::apply_stats_from_class");
}

fn generate_player_age(player_race: Race) -> u16 {
    match player_race {
        Race::Human => 12 + random::randint(6) as u16,
        Race::HalfElf => 24 + random::randint(16) as u16,
        Race::Elf => 75 + random::randint(75) as u16,
        Race::Halfling => 21 + random::randint(12) as u16,
        Race::Gnome => 50 + random::randint(40) as u16,
        Race::Dwarf => 35 + random::randint(15) as u16,
        Race::HalfOrc => 11 + random::randint(4) as u16,
        Race::HalfTroll => 20 + random::randint(10) as u16,
        Race::Phraint => 15 + random::randint(10) as u16,
        Race::Dryad => 75 + random::randint(75) as u16,
    }
}

fn generate_player_height(player_race: Race, player_sex: Sex) -> u16 {
    match player_race {
        Race::Human => match player_sex {
            Sex::Male => random::randnor(72, 6) as u16,
            Sex::Female => random::randnor(66, 4) as u16,
        },
        Race::HalfElf => match player_sex {
            Sex::Male => random::randnor(66, 6) as u16,
            Sex::Female => random::randnor(62, 6) as u16,
        },
        Race::Elf => match player_sex {
            Sex::Male => random::randnor(60, 4) as u16,
            Sex::Female => random::randnor(54, 4) as u16,
        },
        Race::Halfling => match player_sex {
            Sex::Male => random::randnor(36, 3) as u16,
            Sex::Female => random::randnor(33, 3) as u16,
        },
        Race::Gnome => match player_sex {
            Sex::Male => random::randnor(42, 3) as u16,
            Sex::Female => random::randnor(39, 3) as u16,
        },
        Race::Dwarf => match player_sex {
            Sex::Male => random::randnor(48, 3) as u16,
            Sex::Female => random::randnor(46, 3) as u16,
        },
        Race::HalfOrc => match player_sex {
            Sex::Male => random::randnor(66, 1) as u16,
            Sex::Female => random::randnor(62, 3) as u16,
        },
        Race::HalfTroll => match player_sex {
            Sex::Male => random::randnor(96, 10) as u16,
            Sex::Female => random::randnor(84, 12) as u16,
        },
        Race::Phraint => match player_sex {
            Sex::Male => random::randnor(96, 24) as u16,
            Sex::Female => random::randnor(84, 12) as u16,
        },
        Race::Dryad => match player_sex {
            Sex::Male => random::randnor(60, 4) as u16,
            Sex::Female => random::randnor(40, 4) as u16,
        },
    }
}

fn generate_player_weight(race: Race, sex: Sex) -> u16 {
    random::randnor(
        data::race::weight_base(&race, sex) as i64,
        data::race::weight_modifier(&race, sex) as i64,
    ) as u16
}

// Computes character's age, height, and weight -JWT-
fn generate_ahw() {
    debug::enter("create_character::generate_ahw");

    let player_race = player::race();
    let player_sex = player::sex();

    unsafe {
        player::player_age = generate_player_age(player_race);
    }

    let mut player_birthdate: GameTime = GameTime::new();
    player_birthdate.year = 500 + random::randint(50);
    player_birthdate.month = random::randint(13) as u8;
    player_birthdate.day = random::randint(28) as u8;
    player_birthdate.hour = (random::randint(24) - 1) as u8;
    player_birthdate.secs = (random::randint(400) - 1) as u16;
    player::set_birthdate(player_birthdate);

    let mut player_age: GameTime = GameTime::new();
    player_age.year = unsafe { player::player_age } as i64 + player_birthdate.year;
    player_age.month = player_birthdate.month as u8;
    player_age.day = (player_birthdate.day + 1) as u8;
    player_age.hour = 7 as u8;
    player_age.secs = (300 + random::randint(99)) as u16;
    player::set_age(player_age);

    unsafe {
        player::player_ht = generate_player_height(player_race, player_sex);
        player::player_wt = generate_player_weight(player_race, player_sex);

        player::player_disarm =
            (data::race::disarm_mod(&player_race) as i16 + player::disarm_from_dex()).into();
    }

    debug::leave("create_character::generate_ahw");
}

// Get the racial history, and determines social class
fn generate_history() {
    let human_history: Vec<Vec<(&'static str, u8, i16)>> = vec![
        vec![
            (
                "You are the illegitimate and unacknowledged child ",
                10,
                -25,
            ),
            ("You are the illegitimate but acknowledged child ", 20, -15),
            ("You are one of several children ", 95, -5),
            ("You are the 1st child ", 100, 0),
        ],
        vec![
            ("of a serf.  ", 40, 15),
            ("of a yeoman.  ", 65, 30),
            ("of a townsman.  ", 80, 40),
            ("of a guildsman.  ", 90, 55),
            ("of a landed knight.  ", 96, 70),
            ("of a titled noble.  ", 99, 80),
            ("of a royal blood line.  ", 100, 90),
        ],
        vec![
            ("You are the black sheep of the family.  ", 20, -30),
            ("You are a credit to the family.  ", 80, 5),
            ("You are a well liked child.  ", 100, 10),
        ],
        vec![
            ("You have dark brown eyes, ", 20, 0),
            ("You have brown eyes, ", 60, 0),
            ("You have hazel eyes, ", 70, 0),
            ("You have green eyes, ", 80, 0),
            ("You have blue eyes, ", 90, 0),
            ("You have blue-gray eyes, ", 100, 0),
        ],
        vec![("straight ", 70, 0), ("wavey ", 90, 0), ("curly ", 100, 0)],
        vec![
            ("black hair, ", 30, 0),
            ("brown hair, ", 70, 0),
            ("auburn hair, ", 80, 0),
            ("red hair, ", 90, 0),
            ("blonde hair, ", 100, 0),
        ],
        vec![
            ("and a very dark complexion.", 10, 0),
            ("and a dark complexion.", 30, 0),
            ("and an average complexion.", 80, 0),
            ("and a fair complexion.", 90, 0),
            ("and a very fair complexion.", 100, 0),
        ],
    ];
    let halfelf_history: Vec<Vec<(&'static str, u8, i16)>> = vec![vec![
        ("Your mother was a Green-Elf.  ", 40, 0),
        ("Your father was a Green-Elf.  ", 75, 5),
        ("Your mother was a Grey-Elf.  ", 90, 5),
        ("Your father was a Grey-Elf.  ", 95, 10),
        ("Your mother was a High-Elf.  ", 98, 15),
        ("Your father was a High-Elf.  ", 100, 20),
    ]];
    let elf_history: Vec<Vec<(&'static str, u8, i16)>> = vec![
        vec![
            ("You are one of several children ", 60, 0),
            ("You are the only child ", 100, 5),
        ],
        vec![
            ("of a Green-Elf ", 75, 0),
            ("of a Grey-Elf ", 95, 5),
            ("of a High-Elf ", 100, 10),
        ],
        vec![
            ("ranger.  ", 40, 30),
            ("archer.  ", 70, 40),
            ("warrior.  ", 87, 60),
            ("mage.  ", 95, 75),
            ("prince.  ", 99, 90),
            ("king.  ", 100, 95),
        ],
        vec![
            ("You have light grey eyes, ", 85, 0),
            ("You have light violet eyes, ", 90, 0),
            ("You have light blue eyes, ", 95, 0),
            ("You have light green eyes, ", 98, 2),
            ("You have light golden colored eyes, ", 100, 5),
        ],
        vec![("straight ", 75, 0), ("wavey ", 100, 0)],
        vec![
            ("jet black hair, and a fair complexion.", 75, 0),
            ("light brown hair, and a fair complexion.", 85, 0),
            ("blonde hair, and a fair complexion.", 95, 0),
            ("silver hair, and a fair complexion.", 98, 1),
            ("hair the color of spun gold and pale skin.", 100, 5),
        ],
    ];
    let halfling_history: Vec<Vec<(&'static str, u8, i16)>> = vec![
        vec![
            ("You are one of several children of a Halfling ", 85, -5),
            ("You are the only child of a Halfling ", 100, 5),
        ],
        vec![
            ("bum.  ", 20, 5),
            ("tavern owner.  ", 30, 30),
            ("miller.  ", 40, 40),
            ("home owner.  ", 50, 50),
            ("burglar.  ", 80, 60),
            ("monk.  ", 95, 65),
            ("clan elder.  ", 100, 90),
        ],
        vec![
            ("You are the black sheep of the family.  ", 20, -30),
            ("You are a credit to the family.  ", 80, 5),
            ("You are a well liked child.  ", 100, 10),
        ],
        vec![
            ("You have dark brown eyes, ", 20, 0),
            ("You have brown eyes, ", 60, 0),
            ("You have hazel eyes, ", 70, 0),
            ("You have green eyes, ", 80, 0),
            ("You have blue eyes, ", 90, 0),
            ("You have blue-gray eyes, ", 100, 0),
        ],
        vec![("straight ", 70, 0), ("wavey ", 90, 0), ("curly ", 100, 0)],
        vec![
            ("black hair, ", 30, 0),
            ("brown hair, ", 70, 0),
            ("auburn hair, ", 80, 0),
            ("red hair, ", 90, 0),
            ("blonde hair, ", 100, 0),
        ],
        vec![
            ("and a very dark complexion.", 10, 0),
            ("and a dark complexion.", 30, 0),
            ("and an average complexion.", 80, 0),
            ("and a fair complexion.", 90, 0),
            ("and a very fair complexion.", 100, 0),
        ],
    ];
    let gnome_history: Vec<Vec<(&'static str, u8, i16)>> = vec![
        vec![
            ("You are one of several children of a Gnome ", 85, -5),
            ("You are the only child of a Gnome ", 100, 5),
        ],
        vec![
            ("beggar.  ", 20, 5),
            ("bragart.  ", 50, 20),
            ("prankster.  ", 75, 35),
            ("druid.  ", 95, 50),
            ("mage.  ", 100, 75),
        ],
        vec![
            ("You are the black sheep of the family.  ", 20, -30),
            ("You are a credit to the family.  ", 80, 5),
            ("You are a well liked child.  ", 100, 10),
        ],
        vec![
            ("You have dark brown eyes, ", 20, 0),
            ("You have brown eyes, ", 60, 0),
            ("You have hazel eyes, ", 70, 0),
            ("You have green eyes, ", 80, 0),
            ("You have blue eyes, ", 90, 0),
            ("You have blue-gray eyes, ", 100, 0),
        ],
        vec![("straight ", 70, 0), ("wavey ", 90, 0), ("curly ", 100, 0)],
        vec![
            ("black hair, ", 30, 0),
            ("brown hair, ", 70, 0),
            ("auburn hair, ", 80, 0),
            ("red hair, ", 90, 0),
            ("blonde hair, ", 100, 0),
        ],
        vec![
            ("and a very dark complexion.", 10, 0),
            ("and a dark complexion.", 30, 0),
            ("and an average complexion.", 80, 0),
            ("and a fair complexion.", 90, 0),
            ("and a very fair complexion.", 100, 0),
        ],
    ];
    let dwarf_history: Vec<Vec<(&'static str, u8, i16)>> = vec![
        vec![
            ("You are one of two children of a Dwarven ", 25, -10),
            ("You are the only child of a Dwarven ", 100, 0),
        ],
        vec![
            ("thief.  ", 10, 10),
            ("prison guard.  ", 25, 25),
            ("miner.  ", 75, 40),
            ("warrior.  ", 90, 60),
            ("priest.  ", 99, 80),
            ("king.  ", 100, 100),
        ],
        vec![
            ("You are the black sheep of the family.  ", 15, -40),
            ("You are a credit to the family.  ", 85, 0),
            ("You are a well liked child.  ", 100, 5),
        ],
        vec![
            ("You have dark brown eyes, ", 99, 0),
            ("You have glowing red eyes, ", 100, 10),
        ],
        vec![("straight ", 90, 0), ("wavey ", 100, 0)],
        vec![("black hair, ", 75, 0), ("brown hair, ", 100, 0)],
        vec![
            ("a one foot beard, ", 25, 0),
            ("a two foot beard, ", 60, 1),
            ("a three foot beard, ", 90, 3),
            ("a four foot beard, ", 100, 5),
        ],
        vec![("and a dark complexion.", 100, 0)],
    ];
    let halforc_history: Vec<Vec<(&'static str, u8, i16)>> = vec![
        vec![
            (
                "Your mother was an Orc, but it is unacknowledged.  ",
                25,
                -25,
            ),
            (
                "Your father was an Orc, but it is unacknowledged.  ",
                100,
                -25,
            ),
        ],
        vec![("You are the adopted child ", 100, 0)],
        vec![
            ("of a serf.  ", 40, 15),
            ("of a yeoman.  ", 65, 30),
            ("of a townsman.  ", 80, 40),
            ("of a guildsman.  ", 90, 55),
            ("of a landed knight.  ", 96, 70),
            ("of a titled noble.  ", 99, 80),
            ("of a royal blood line.  ", 100, 90),
        ],
        vec![
            ("You are the black sheep of the family.  ", 20, -30),
            ("You are a credit to the family.  ", 80, 5),
            ("You are a well liked child.  ", 100, 10),
        ],
        vec![
            ("You have dark brown eyes, ", 20, 0),
            ("You have brown eyes, ", 60, 0),
            ("You have hazel eyes, ", 70, 0),
            ("You have green eyes, ", 80, 0),
            ("You have blue eyes, ", 90, 0),
            ("You have blue-gray eyes, ", 100, 0),
        ],
        vec![("straight ", 70, 0), ("wavey ", 90, 0), ("curly ", 100, 0)],
        vec![
            ("black hair, ", 30, 0),
            ("brown hair, ", 70, 0),
            ("auburn hair, ", 80, 0),
            ("red hair, ", 90, 0),
            ("blonde hair, ", 100, 0),
        ],
        vec![
            ("and a very dark complexion.", 10, 0),
            ("and a dark complexion.", 30, 0),
            ("and an average complexion.", 80, 0),
            ("and a fair complexion.", 90, 0),
            ("and a very fair complexion.", 100, 0),
        ],
    ];
    let halftroll_history: Vec<Vec<(&'static str, u8, i16)>> = vec![
        vec![
            ("Your mother was a Cave-Troll ", 30, -30),
            ("Your father was a Cave-Troll ", 60, -25),
            ("Your mother was a Hill-Troll ", 75, -20),
            ("Your father was a Hill-Troll ", 90, -15),
            ("Your mother was a Water-Troll ", 95, -10),
            ("Your father was a Water-Troll ", 100, -5),
        ],
        vec![
            ("cook.  ", 5, 10),
            ("warrior.  ", 95, 5),
            ("shaman.  ", 99, 15),
            ("clan chief.  ", 100, 30),
        ],
        vec![
            ("You have slime green eyes, ", 60, 0),
            ("You have puke yellow eyes, ", 85, 0),
            ("You have blue-bloodshot eyes, ", 99, 0),
            ("You have glowing red eyes, ", 100, 5),
        ],
        vec![("dirty ", 33, 0), ("mangy ", 66, 0), ("oily ", 100, 0)],
        vec![
            ("sea-weed green hair, ", 33, 0),
            ("bright red hair, ", 66, 0),
            ("dark purple hair, ", 100, 0),
        ],
        vec![
            ("and green ", 25, 0),
            ("and blue ", 50, 0),
            ("and white ", 75, 0),
            ("and black ", 100, 0),
        ],
        vec![
            ("ulcerous skin.", 33, 0),
            ("scabby skin.", 66, 0),
            ("leprous skin.", 100, 0),
        ],
    ];
    let phraint_history: Vec<Vec<(&'static str, u8, i16)>> = vec![
        vec![
            ("You are one of many illegitimate children ", 5, -30),
            ("You are one of several illegitimate children ", 10, -25),
            ("You are one of many children ", 50, -10),
            ("You are one of several children ", 75, -5),
            ("You are the 2nd child ", 95, 0),
            ("You are the only child ", 100, 5),
        ],
        vec![
            ("of a worker.  ", 50, 15),
            ("of a warrior.  ", 75, 30),
            ("of an elite warrior.  ", 90, 50),
            ("of the hive mother.  ", 100, 100),
        ],
        vec![
            ("You are the outcast of the hive.  ", 5, -50),
            ("You are the black sheep of the hive.  ", 20, -30),
            ("You are a credit to the hive.  ", 80, 5),
            ("You are a well liked child.  ", 100, 10),
        ],
        vec![
            ("You have small ", 40, 0),
            ("You have large ", 70, 0),
            ("You have very large ", 100, 0),
        ],
        vec![
            ("buggy green eyes, ", 10, 0),
            ("buggy silver eyes, ", 30, 0),
            ("iridescent eyes, ", 70, 0),
            ("glowing eyes, ", 100, 0),
        ],
        vec![
            ("straight feelers, ", 10, 0),
            ("curved feelers, ", 30, 0),
            ("bent feelers, ", 80, 0),
            ("very long feelers, ", 100, 0),
        ],
        vec![
            ("and dull brown chiton. ", 10, 0),
            ("and shiny brown chiton. ", 60, 0),
            ("and shiny black chiton. ", 90, 0),
            ("and polished silver chiton. ", 100, 0),
        ],
    ];
    let dryad_history: Vec<Vec<(&'static str, u8, i16)>> = vec![
        vec![
            ("You are the Dryad of a sickly ", 15, -20),
            ("You are the Dryad of a large ", 40, 0),
            ("You are the Dryad of a rich, green ", 60, 0),
            ("You are the Dryad of a magnificent ", 90, 10),
        ],
        vec![
            ("pine tree", 30, 5),
            ("birch tree", 40, 15),
            ("ash tree", 50, 30),
            ("cedar tree", 70, 50),
            ("willow tree", 90, 70),
            ("oak tree", 100, 90),
        ],
        vec![
            (", but the elven community avoids your forest.  ", 10, -30),
            (" in a small glade.  ", 40, -5),
            (" and you are a fine upholder of the woodlands.  ", 60, 5),
            (
                " and Humans and Half-Elves hold your tree sacred.  ",
                88,
                20,
            ),
            (" where the Elves hold yearly ceremonies.  ", 90, 25),
            (" that all races hold in reverence.  ", 100, 30),
        ],
        vec![
            ("You have light grey eyes, ", 85, 0),
            ("You have light violet eyes, ", 90, 0),
            ("You have light blue eyes, ", 95, 0),
            ("You have light green eyes, ", 98, 2),
            ("You have light golden colored eyes, ", 100, 5),
        ],
        vec![("straight ", 75, 0), ("wavey ", 100, 0)],
        vec![
            ("jet black hair, and a fair complexion.", 75, 0),
            ("light brown hair, and a fair complexion.", 85, 0),
            ("blonde hair, and a fair complexion.", 95, 0),
            ("silver hair, and a fair complexion.", 98, 1),
            ("hair the color of spun gold and pale skin.", 100, 5),
        ],
    ];
    fn generate_history(
        history: &mut String,
        social_class: &mut i16,
        history_list: Vec<Vec<(&'static str, u8, i16)>>,
    ) {
        for list in history_list.iter() {
            let roll = random::randint(100) as u8;
            for triple in list {
                if triple.1 >= roll {
                    *history += triple.0;
                    *social_class += triple.2;
                    break;
                }
            }
        }
    }

    let mut history: String = String::new();
    let mut social_class: i16 = 0;

    match player::race() {
        Race::Human => {
            generate_history(&mut history, &mut social_class, human_history);
        }
        Race::HalfElf => {
            generate_history(&mut history, &mut social_class, halfelf_history);
            generate_history(&mut history, &mut social_class, human_history);
        }
        Race::Elf => {
            generate_history(&mut history, &mut social_class, elf_history);
        }
        Race::Halfling => {
            generate_history(&mut history, &mut social_class, halfling_history);
        }
        Race::Gnome => {
            generate_history(&mut history, &mut social_class, gnome_history);
        }
        Race::Dwarf => {
            generate_history(&mut history, &mut social_class, dwarf_history);
        }
        Race::HalfOrc => {
            generate_history(&mut history, &mut social_class, halforc_history);
        }
        Race::HalfTroll => {
            generate_history(&mut history, &mut social_class, halftroll_history);
        }
        Race::Phraint => {
            generate_history(&mut history, &mut social_class, phraint_history);
        }
        Race::Dryad => {
            generate_history(&mut history, &mut social_class, dryad_history);
        }
    };

    // Process block of history text for pretty output
    for i in 0..5 {
        unsafe { player::player_history[i][0] = '\0' as i8 };
    }

    let mut i: usize = 0;
    let mut tmp_str: String = String::new();
    let mut history_words_iter = history.split_whitespace();
    while let Some(word) = history_words_iter.next() {
        let tmp_str_len = tmp_str.len();
        let word_len = word.len();

        if tmp_str_len + word_len > 70 {
            let cstr = CString::new(tmp_str).unwrap();
            unsafe {
                strcpy(player::player_history[i].as_mut_ptr(), cstr.as_ptr());
            }
            i += 1;
            tmp_str = String::new();
        }
        tmp_str += word;
        tmp_str += " ";
    }

    let cstr = CString::new(tmp_str).unwrap();
    unsafe {
        strcpy(player::player_history[i].as_mut_ptr(), cstr.as_ptr());
    }

    social_class = min(max(social_class, 1), 100);

    unsafe {
        player::player_rep = (50 - social_class).into();
        player::player_sc = social_class;
    }
}

// Display character on screen -RAK-
fn display_char() {
    debug::enter("create_character::display_char");

    put_character(true);
    put_misc1();
    put_stats();
    put_misc2();
    put_misc3();

    debug::leave("create_character::display_char");
}

pub fn change_name() {
    debug::enter("create_character::change_name");

    term::clear_screen();
    display_char();
    loop {
        term::prt("<c>hange character name.     <ESCAPE> to continue.", 21, 2);
        match io::inkey_flush() {
            99 => choose_name(),
            0 | 3 | 25 | 26 | 27 => break,
            _ => (),
        }
    }

    debug::leave("create_character::change_name");
}

/* Gets a name for the character    -JWT- */
fn choose_name() {
    debug::enter("create_character::choose_name");

    term::prt(
        "Enter your player's name  [press <RETURN> when finished]",
        21,
        2,
    );
    loop {
        let new_name = term::get_string(3, 15, 24);
        if !new_name.is_empty() {
            player::set_name(&new_name);
            break;
        }
    }
    term::clear_from(21);

    debug::leave("create_character::choose_name");
}

fn choose_class() {
    debug::enter("create_character::choose_class");

    let classes = data::race::available_classes(&player::race());
    let class_strings = classes.iter().map(data::class::name).collect::<Vec<&str>>();
    let mut index = 0;

    loop {
        menu::draw_menu(
            "Choose your class",
            &class_strings,
            "j=up, k=down, enter=select, ?=info, r=restrictions",
            index,
        );
        match io::inkey_flush() as char {
            'k' => index = if index == 0 { 0 } else { index - 1 },
            'j' => index = min(classes.len() as u8 - 1, index + 1),
            '\r' => {
                player::set_class(classes[index as usize]);
                debug::leave("create_character::choose_class");
                return;
            }
            '?' => menu::draw_help(
                data::class::name(&classes[index as usize]),
                data::class::info(&classes[index as usize]),
            ),
            'r' => menu::draw_help(
                data::class::name(&classes[index as usize]),
                data::class::restriction_info(&classes[index as usize]),
            ),
            _ => {}
        }
    }
}

pub fn stats_info(race: &Race) -> Vec<String> {
    let stats = data::race::stat_block(race);
    vec![
        format!("Melee bonus:       {}", data::race::melee_bonus(race)),
        format!("Ranged bonus:      {}", data::race::ranged_bonus(race)),
        format!("Experience factor: {}", data::race::expfactor(race)),
        format!("Search frequence:  {}", data::race::search_freq(race)),
        format!("Search modifier:   {}", data::race::search_mod(race)),
        format!("Stealth modifier:  {}", data::race::stealth_mod(race)),
        format!("Save modifier:     {}", data::race::save_mod(race)),
        format!("Disarm modifier:   {}", data::race::disarm_mod(race)),
        format!("Infravision:       {}", data::race::infravision(race)),
        format!("Swim speed:        {}", data::race::swim_speed(race)),
        "ATTRIBUTES:".to_owned(),
        format!("Strength:          {}", stats.strength),
        format!("Dexterity:         {}", stats.dexterity),
        format!("Constituton:       {}", stats.constitution),
        format!("Intelligence:      {}", stats.intelligence),
        format!("Wisdom:            {}", stats.wisdom),
        format!("Charisma:          {}", stats.charisma),
    ]
}

fn choose_race() {
    debug::enter("create_character::choose_race");

    let races = Race::iter()
        .map(|i| data::race::name(&Race::from(i)))
        .collect::<Vec<&str>>();
    let mut index = 0;

    loop {
        menu::draw_menu(
            "Choose your race",
            &races,
            "j=up, k=down, enter=select, ?=info, s=stats, c=classes",
            index,
        );

        match io::inkey_flush() as char {
            'k' => index = if index == 0 { 0 } else { index - 1 },
            'j' => index = min(races.len() as u8 - 1, index + 1),
            '\r' => {
                player::set_race(conversion::race::from_usize(index as usize).unwrap());
                debug::leave("create_character::choose_race");
                return;
            }
            '?' => menu::draw_help(
                races[index as usize],
                data::race::info(&conversion::race::from_usize(index as usize).unwrap()),
            ),
            's' => menu::draw_help_vec(
                races[index as usize],
                &stats_info(&conversion::race::from_usize(index as usize).unwrap())
                    .iter()
                    .map(|it| it.as_ref())
                    .collect::<Vec<&str>>(),
            ),
            'c' => menu::draw_help_vec(
                races[index as usize],
                &data::race::available_classes(
                    &conversion::race::from_usize(index as usize).unwrap(),
                )
                .iter()
                .map(data::class::name)
                .collect::<Vec<&str>>(),
            ),
            _ => {}
        }
    }
}

fn choose_sex() {
    debug::enter("create_character::choose_sex");

    if player::race() == Race::Dryad {
        player::set_sex(Sex::Female);
        debug::leave("create_character::choose_sex");
        return;
    }

    let mut index = 0;
    loop {
        menu::draw_menu(
            "Choose your sex",
            &vec!["Male", "Female"],
            "j=up, k=down, enter=select",
            index,
        );

        match io::inkey_flush() as char {
            'k' => index = 0,
            'j' => index = 1,
            '\r' => {
                player::set_sex(if index == 0 { Sex::Male } else { Sex::Female });
                debug::leave("create_character::choose_sex");
                return;
            }
            _ => {}
        }
    }
}

fn choose_stats() {
    debug::enter("create_character::choose_stats");

    fn roll_stats() {
        get_stats();
        generate_history();
        generate_ahw();

        unsafe {
            player::player_dis_th = player::tohit_from_stats().into();
            player::player_dis_td = player::dmg_from_str().into();
            player::player_dis_tac = player::ac_from_dex().into();
        }
    }

    roll_stats();
    loop {
        let stats = player::curr_stats();

        menu::draw_menu(
            "Roll up your stats",
            &vec![
                format!("Age:           {}", unsafe { player::player_age }),
                format!("Height:        {}", unsafe { player::player_ht }),
                format!("Weight:        {}", unsafe { player::player_wt }),
                format!("Social Class:  {}", unsafe { player::player_sc }),
                "".to_string(),
                "(Attributes):".to_owned(),
                format!("Strength:      {}", stats.strength),
                format!("Dexterity:     {}", stats.dexterity),
                format!("Constitution:  {}", stats.constitution),
                format!("Intelligence:  {}", stats.intelligence),
                format!("Wisdom:        {}", stats.wisdom),
                format!("Charisma:      {}", stats.charisma),
            ]
            .iter()
            .map(|it| it.as_ref())
            .collect::<Vec<&str>>(),
            "r=reroll stats, enter=confirm",
            255,
        );
        match io::inkey_flush() as char {
            'r' => roll_stats(),
            '\r' => return,
            _ => {}
        }
        put_misc1();
    }
}

fn confirm_character() {
    debug::enter("create_character::confirm_character");

    let stats = player::curr_stats();

    menu::draw_menu(
        "Confirm your character",
        &vec![
            "Name: ".to_string(),
            format!("Race:          {}", data::race::name(&player::race())),
            format!("Sex:           {}", data::sex::name(&player::sex())),
            format!("Class:         {}", data::class::name(&player::class())),
            "".to_string(),
            format!("Hit Points     {}", player::max_hp()),
            format!("Mana           {}", unsafe { player::player_mana }),
            "".to_string(),
            "(Attributes):".to_string(),
            format!("Strength:      {}", stats.strength),
            format!("Dexterity:     {}", stats.dexterity),
            format!("Constitution:  {}", stats.constitution),
            format!("Intelligence:  {}", stats.intelligence),
            format!("Wisdom:        {}", stats.wisdom),
            format!("Charisma:      {}", stats.charisma),
        ]
        .iter()
        .map(|it| it.as_ref())
        .collect::<Vec<&str>>(),
        "Enter your name, finish with enter",
        255,
    );

    loop {
        let new_name = term::get_string(4, 18, 24);
        if !new_name.is_empty() {
            player::set_name(&new_name);
            break;
        }
    }

    debug::leave("create_character::confirm_character");
}

fn add_equipment() {
    debug::enter("create_character::add_equipment");

    // General starting items
    let mut general_starting_items = Vec::new();

    let mut ration_of_food =
        generate_item::create_item(Box::new(item_template::FoodTemplate::RationOfFood), 0);
    ration_of_food.number = 5;
    general_starting_items.push(ration_of_food);

    let torch =
        generate_item::create_item(Box::new(item_template::LightSourceTemplate::WoodenTorch), 0);
    general_starting_items.push(torch);
    let light_cloak =
        generate_item::create_item(Box::new(item_template::CloakTemplate::LightCloak), 0);
    general_starting_items.push(light_cloak);
    let soft_leather_armor =
        generate_item::create_item(Box::new(item_template::ArmorTemplate::SoftLeatherArmor), 0);
    general_starting_items.push(soft_leather_armor);

    for item in general_starting_items {
        unsafe {
            add_inven_item(item);
        }
    }

    // Class specific starting items
    for item in data::class::starting_items(&player::class()) {
        unsafe {
            add_inven_item(item);
        }
    }

    debug::leave("create_character::add_equipment");
}

pub fn create_character() {
    debug::enter("create_character::create_character");

    choose_race();
    choose_sex();
    choose_stats();
    print_history();
    choose_class();
    apply_stats_from_class();

    unsafe {
        player::player_creation_time = time(null::<time_t>() as *mut i64);
        player::player_claim_check = 0;
    }

    get_money();

    confirm_character(); // And choose name
    add_equipment();

    debug::leave("create_character::create_character");
}
