#ifndef PLAYER_H
#define PLAYER_H

#include "imoria.h"

extern p_flags player_flags;

/* P_STATS */
/* array[stat_set] of {permanent} */
extern uint8_t player_stats_perm[STAT_SET_MAX + 1];
/* array[stat_set] of {current=p-l+m*10} */
extern uint8_t player_stats_curr[STAT_SET_MAX + 1];
/* array[stat_set] of {net magical adj} */
extern int8_t  player_stats_mod[STAT_SET_MAX + 1];
/* array[stat_set] of {amt lost} */
extern uint8_t player_stats_lost[STAT_SET_MAX + 1];

/* P_MISC */
extern int64_t player_xtr_wgt;	  /* { Extra weight limit	} */
extern int64_t player_account;		  /* { Money in the bank	} */
extern money_type player_money;	 /* { Money on person	} */
extern game_time_type player_birth;     /* {Date of char's birth} */
extern game_time_type player_cur_age;   /* {Current game date	} */
extern time_type player_play_tm;	/* { Time spent in game	} */
extern uint8_t player_diffic;     /* { Difficulty of game	} */
extern char player_name[82];		  /* { Name of character	} */
extern char player_race[82];		  /* { Race of character	} */
extern char player_sex[82];		  /* { Sex of character	} */
extern char player_title[82];		  /* { Character's title	} */
extern char player_tclass[82];		  /* { Character's class	} */
extern int64_t player_max_exp;		  /* { Max experience} */
extern int64_t player_exp;		  /* { Cur experienc	} */
extern int64_t player_rep;		  /* { XP from good creatures } */
extern int64_t player_deaths;		  /* {Number of insured restores} */
extern int64_t player_premium;		  /* {Base cost to restore } */
extern uint16_t player_age;       /* { Characters age} */
extern uint16_t player_ht;	/* { Height	} */
extern uint16_t player_wt;	/* { Weight	} */
extern uint16_t player_lev;       /* { Level		} */
extern uint16_t player_max_lev;   /* { Max level explored} */
extern int16_t player_srh;		  /* { Chance in search} */
extern int16_t player_fos;		  /* { Frenq of search} */
extern int16_t player_bth;		  /* { Base to hit	} */
extern int16_t player_bthb;		  /* { BTH with bows	} */
extern int16_t player_mana;		  /* { Mana points	} */
extern int16_t player_mhp;		  /* { Max hit pts	} */
extern int16_t player_ptohit;		  /* { Pluses to hit	} */
extern int16_t player_ptodam;		  /* { Pluses to dam	} */
extern int16_t player_pac;		  /* { Total AC	} */
extern int16_t player_ptoac;		  /* { Magical AC	} */
extern int16_t player_dis_th;		  /* { Display +ToHit} */
extern int16_t player_dis_td;		  /* { Display +ToDam} */
extern int16_t player_dis_ac;		  /* { Display +ToAC } */
extern int16_t player_dis_tac;		  /* { Display +ToTAC} */
extern int16_t player_disarm;		  /* { % to Disarm	} */
extern int16_t player_save;		  /* { Saving throw	} */
extern int16_t player_sc;		  /* { Social Class	} */
extern enum class_t player_pclass;      /* { # of class	} */
extern uint8_t player_prace;      /* { # of race	} */
extern uint8_t player_hitdie;     /* { Char hit die	} */
extern uint8_t player_stl;	/* { Stealth factor} */
extern float player_expfact;		  /* { Experience factor} */
extern float player_cmana;		  /* { Cur mana pts  } */
extern float player_chp;		  /* { Cur hit pts	} */
extern char player_history[5][82];	 /* ;{ History record} */
extern boolean player_cheated;	  /*{ gone into wizard or god mode} */
extern int64_t  player_mr;		  /* { mag.res.lev.delta } */
extern uint8_t player_quests;     /* { # completed } {FUBAR} */
extern uint16_t player_cur_quest; /* { creature # of quest } {FUBAR} */
extern time_t player_creation_time;     /* used as key in master file */
extern int64_t player_save_count;	  /* compared to master file value */
extern int64_t player_claim_check;	 /* used to track trading post */

#endif /* PLAYER_H */