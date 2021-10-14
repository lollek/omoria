use libc;

#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy)]
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

