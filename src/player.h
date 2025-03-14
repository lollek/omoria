#pragma once

#include <time.h>

#include "generate_monster/monster_template.h"
#include "magic.h"
#include "types.h"

typedef struct p_flags {
  bool insured;              /* { Character insured   } */
  bool dead;                 /* { Currently restored  } */
  uint64_t status;              /* { Status of player    } */
  int64_t rest;                 /* { Rest counter	 } */
  int64_t blind;                /* { Blindness counter   } */
  int64_t paralysis;            /* { Paralysis counter   } */
  int64_t confused;             /* { Confusion counter   } */
  int64_t foodc;                /* { Food counter        } (was just food) */
  int64_t food_digested;        /* { Food per round      } */
  int64_t protection;           /* { Protection fr. evil } */
  int64_t speed;                /* { Cur speed adjust    } */
  int64_t speed_paral;          /* { Slow speed adjust   } */
  bool speed_flag;           /* { On if reset speed   } */
  int64_t paral_init;           /* { Init val for slow   } */
  int64_t move_rate;            /* { move_rate	         } */
  int64_t swim;                 /* { Cur swim adjust     } */
  int64_t fast;                 /* { Temp speed change   } */
  int64_t slow;                 /* { Temp speed change   } */
  int64_t petrification;        /* { Amount Petrified    } */
  int64_t afraid;               /* { Fear                } */
  int64_t poisoned;             /* { Poisoned            } */
  int64_t image;                /* { Hallucinate         } */
  int64_t protevil;             /* { Protect VS evil     } */
  int64_t invuln;               /* { Increases AC        } */
  int64_t hero;                 /* { Heroism	         } */
  int64_t shero;                /* { Super Heroism	 } */
  int64_t blessed;              /* { Blessed	         } */
  int64_t resist_heat;          /* { Timed heat resist   } */
  int64_t resist_cold;          /* { Timed cold resist   } */
  int64_t detect_inv;           /* { Timed see invisible } */
  int64_t word_recall;          /* { Timed teleport level} */
  int64_t see_infra;            /* { See warm creatures  } */
  int64_t tim_infra;            /* { Timed infra vision  } */
  bool see_inv;              /* { Can see invisible   } */
  bool teleport;             /* { Random teleportation} */
  bool free_act;             /* { Never paralyzed     } */
  bool slow_digest;          /* { Lower food needs    } */
  bool aggravate;            /* { Agravate monsters   } */
  bool fire_resist;          /* { Resistance to fire  } */
  bool cold_resist;          /* { Resistance to cold  } */
  bool acid_resist;          /* { Resistance to acid  } */
  bool hunger_item;          /* { Resets food counter } */
  bool regenerate;           /* { Regenerate hit pts  } */
  bool lght_resist;          /* { Resistance to light } */
  bool ffall;                /* { No damage falling   } */
  bool sustain[STAT_T_SIZE]; /* { keep characteristic } */
  bool confuse_monster;      /* { Glowing hands...    } */
  int64_t resist_lght;          /* { Timed lighting rst  } */
  int64_t free_time;            /* { Timed free action   } */
  int64_t ring_fire;            /* { Timed fire spell    } */
  int64_t protmon;              /* { Timed monst prot    } */
  int64_t hoarse;               /* { Timed no-bard spells} */
  int64_t magic_prot;           /* { Timed magic prot    } */
  int64_t ring_ice;             /* { Timed cold spell    } */
  int64_t temp_stealth;         /* { Timed stealth       } */
  int64_t resist_petri;         /* { Timed resist petrify} */
  int64_t blade_ring;           /* { Timed blade spell   } */
  bool petri_resist;         /* { Resist Petrification} */
  bool quested;              /* { Performing a Quest  } {FUBAR} */
  bool light_on;             /* { Light source is active } */
  bool resting_till_full;
} p_flags;

extern p_flags player_flags;

/* P_MISC */
extern int64_t player_xtr_wgt;        /* { Extra weight limit	} */
extern int64_t player_account;        /* { Money in the bank	} */
extern money_type player_money;       /* { Money on person	} */
extern game_time_type player_birth;   /* {Date of char's birth} */
extern game_time_type player_cur_age; /* {Current game date	} */
extern time_type player_play_tm;      /* { Time spent in game	} */
extern char player_name[82];          /* { Name of character	} */
extern char player_race[82];          /* { Race of character	} */
extern char player_sex[82];           /* { Sex of character	} */
extern char player_tclass[82];        /* { Character's class	} */
extern int64_t player_max_exp;        /* { Max experience} */
extern int64_t player_exp;            /* { Cur experienc	} */
extern int64_t player_rep;            /* { XP from good creatures } */
extern uint16_t player_age;           /* { Characters age} */
extern uint16_t player_ht;            /* { Height	} */
extern uint16_t player_wt;            /* { Weight	} */
extern uint16_t player_lev;           /* { Level		} */
extern uint16_t player_max_lev;       /* { Max level explored} */
extern int16_t player_fos;            /* { Frenq of search} */
extern int16_t player_bth;            /* { Base to hit	} */
extern int16_t player_bthb;           /* { BTH with bows	} */
extern int16_t player_mana;           /* { Mana points	} */
extern int16_t player_ptohit;         /* { Pluses to hit	} */
extern int16_t player_ptodam;         /* { Pluses to dam	} */
extern int16_t player_pac;            /* { Total AC	} */
extern int16_t player_ptoac;          /* { Magical AC	} */
extern int16_t player_dis_th;         /* { Display +ToHit} */
extern int16_t player_dis_td;         /* { Display +ToDam} */
extern int16_t player_dis_ac;         /* { Display +ToAC } */
extern int16_t player_dis_tac;        /* { Display +ToTAC} */
extern int16_t player_disarm;         /* { % to Disarm	} */
extern int16_t player_save;           /* { Saving throw	} */
extern int16_t player_sc;             /* { Social Class	} */
extern enum class_t player_pclass;    /* { # of class	} */
extern uint8_t player_prace;          /* { # of race	} */
extern int16_t player_stl;            /* { Stealth factor} */
extern float player_expfact;          /* { Experience factor} */
extern float player_cmana;            /* { Cur mana pts  } */
extern char player_history[5][82];    /* ;{ History record} */
extern bool player_cheated;        /*{ gone into wizard or god mode} */
extern int64_t player_mr;             /* { mag.res.lev.delta } */
extern uint8_t player_quests;         /* { # completed } {FUBAR} */
extern uint16_t player_cur_quest;     /* { creature # of quest } {FUBAR} */
extern time_t player_creation_time;   /* used as key in master file */
extern int64_t player_claim_check;    /* used to track trading post */
extern int64_t player_uid;            /* Used in master file */
extern bool player_light;          /* { Player carrying light } */

uint16_t C_player_max_bulk(void);
int16_t C_player_dmg_from_str(void);
int16_t C_player_disarm_from_dex(void);
int16_t C_player_hp_from_con(void);
bool C_player_knows_spell(int32_t slot);
void C_player_set_knows_spell(int32_t slot, bool yn);
bool C_player_uses_magic(enum magic_t magic_type);
void C_player_add_exp(long num);
int16_t C_player_roll_hp_for_levelup(void);
void C_player_set_extra_bulk_carry(uint16_t new_value);
void C_player_mod_search_skill(int16_t modifier);
int16_t C_player_curr_search_skill(void);

int16_t C_player_current_hp(void);
int16_t C_player_max_hp(void);
void C_player_reset_current_hp(void);
void C_player_modify_max_hp(int16_t modifier);
void C_player_modify_current_hp(float modifier);
void C_player_regen_hp(float percent);

void C_player_recalc_stats(void);
int16_t C_player_get_stat(enum stat_t attr);
void C_player_modify_lost_stat(enum stat_t attr, int16_t amount);
void C_player_reset_lost_stat(enum stat_t attr);
bool C_player_has_lost_stat(enum stat_t attr);
int16_t C_player_mod_from_stat(enum stat_t attr);
int16_t C_player_tohit_from_stats(void);
int16_t C_player_ac_from_dex(void);
float C_player_cost_modifier_from_charisma(void);

void C_player_mod_stat(enum stat_t attr, int16_t modifier);
void C_player_mod_perm_stat(enum stat_t attr, int16_t modifier);

void search_off(void);
void search_on(void);
void rest_off(void);
void take_hit(long damage, char hit_from[82]);
void regenmana(float percent);

/*
  { Changes speed of monsters relative to player		-RAK-
  }
  { Note: When the player is sped up or slowed down, I simply     }
  {       change the speed of all the monsters.  This greatly     }
  {       simplified the logic...                                 }
*/
void change_speed(long num);

/*
      { Player bonuses					-RAK-	}
      { When an item is worn or taken off, this re-adjusts the player }
      { bonuses.  Factor=1 : wear; Factor=-1 : removed                }
*/
void py_bonuses(const treasure_type *tobj, long factor);

/**
 * @return True if player has no light
 */
bool player_has_no_light(void);

void change_rep(long amt);

bool player_test_hit(long base_to_hit, long level, long plus_to_hit, long enemy_ac,
                        bool was_fired);

long tot_dam(const treasure_type *item, long tdam, monster_template const *monster);
void py_bonuses(const treasure_type *tobj, long factor);
