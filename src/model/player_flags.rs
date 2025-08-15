#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct PlayerFlags {
    pub insured: u8,         /* { Character insured   } */
    pub dead: u8,            /* { Currently restored  } */
    pub status: u64,         /* { Status of player    } */
    pub rest: i64,           /* { Rest counter  } */
    pub blind: i64,          /* { Blindness counter   } */
    pub paralysis: i64,      /* { Paralysis counter   } */
    pub confused: i64,       /* { Confusion counter   } */
    pub foodc: i64,          /* { Food counter        } (was just food) */
    pub food_digested: i64,  /* { Food per round      } */
    pub protection: i64,     /* { Protection fr. evil } */
    pub speed: i64,          /* { Cur speed adjust    } */
    pub speed_paral: i64,    /* { Slow speed adjust   } */
    pub speed_flag: u8,      /* { On if reset speed   } */
    pub paral_init: i64,     /* { Init val for slow   } */
    pub move_rate: i64,      /* { move_rate          } */
    pub swim: i64,           /* { Cur swim adjust     } */
    pub fast: i64,           /* { Temp speed change   } */
    pub slow: i64,           /* { Temp speed change   } */
    pub petrification: i64,  /* { Amount Petrified    } */
    pub afraid: i64,         /* { Fear                } */
    pub poisoned: i64,       /* { Poisoned            } */
    pub image: i64,          /* { Hallucinate         } */
    pub protevil: i64,       /* { Protect VS evil     } */
    pub invuln: i64,         /* { Increases AC        } */
    pub hero: i64,           /* { Heroism          } */
    pub shero: i64,          /* { Super Heroism  } */
    pub blessed: i64,        /* { Blessed          } */
    pub resist_heat: i64,    /* { Timed heat resist   } */
    pub resist_cold: i64,    /* { Timed see invisible } */
    pub word_recall: i64,    /* { Timed teleport level} */
    pub see_infra: i64,      /* { Timed cold resist   } */
    pub detect_inv: i64,     /* { See warm creatures  } */
    pub tim_infra: i64,      /* { Timed infra vision  } */
    pub see_inv: u8,         /* { Can see invisible   } */
    pub teleport: u8,        /* { Random teleportation} */
    pub free_act: u8,        /* { Never paralyzed     } */
    pub slow_digest: u8,     /* { Lower food needs    } */
    pub aggravate: u8,       /* { Agravate monsters   } */
    pub fire_resist: u8,     /* { Resistance to fire  } */
    pub cold_resist: u8,     /* { Resistance to cold  } */
    pub acid_resist: u8,     /* { Resistance to acid  } */
    pub hunger_item: u8,     /* { Resets food counter } */
    pub regenerate: u8,      /* { Regenerate hit pts  } */
    pub lght_resist: u8,     /* { Resistance to light } */
    pub ffall: u8,           /* { No damage falling   } */
    pub sustain: [u8; 6],    /* { keep characteristic } */
    pub confuse_monster: u8, /* { Glowing hands...    } */
    pub resist_lght: i64,    /* { Timed lighting rst  } */
    pub free_time: i64,      /* { Timed free action   } */
    pub ring_fire: i64,      /* { Timed fire spell    } */
    pub protmon: i64,        /* { Timed monst prot    } */
    pub hoarse: i64,         /* { Timed no-bard spells} */
    pub magic_prot: i64,     /* { Timed magic prot    } */
    pub ring_ice: i64,       /* { Timed cold spell    } */
    pub temp_stealth: i64,   /* { Timed stealth       } */
    pub resist_petri: i64,   /* { Timed resist petrify} */
    pub blade_ring: i64,     /* { Timed blade spell   } */
    pub petri_resist: u8,    /* { Resist Petrification} */
    pub quested: u8,         /* { Performing a Quest  } {FUBAR} */
    pub light_on: u8,        /* { Light source is active } */
    pub resting_till_full: u8,
}

impl PlayerFlags {
    pub fn new() -> Self {
        PlayerFlags {
            insured: 0,
            dead: 0,
            status: 0,
            rest: 0,
            blind: 0,
            paralysis: 0,
            confused: 0,
            foodc: 0,
            food_digested: 0,
            protection: 0,
            speed: 0,
            speed_paral: 0,
            speed_flag: 0,
            paral_init: 0,
            move_rate: 0,
            swim: 0,
            fast: 0,
            slow: 0,
            petrification: 0,
            afraid: 0,
            poisoned: 0,
            image: 0,
            protevil: 0,
            invuln: 0,
            hero: 0,
            shero: 0,
            blessed: 0,
            resist_heat: 0,
            resist_cold: 0,
            word_recall: 0,
            see_infra: 0,
            detect_inv: 0,
            tim_infra: 0,
            see_inv: 0,
            teleport: 0,
            free_act: 0,
            slow_digest: 0,
            aggravate: 0,
            fire_resist: 0,
            cold_resist: 0,
            acid_resist: 0,
            hunger_item: 0,
            regenerate: 0,
            lght_resist: 0,
            ffall: 0,
            sustain: [0; 6],
            confuse_monster: 0,
            resist_lght: 0,
            free_time: 0,
            ring_fire: 0,
            protmon: 0,
            hoarse: 0,
            magic_prot: 0,
            ring_ice: 0,
            temp_stealth: 0,
            resist_petri: 0,
            blade_ring: 0,
            petri_resist: 0,
            quested: 0,
            light_on: 0,
            resting_till_full: 0,
        }
    }

    pub fn leading_ones(self) -> u32 {
        self.detect_inv.leading_ones()
    }
}

impl Default for PlayerFlags {
    fn default() -> Self {
        Self::new()
    }
}
