#ifndef VARIABLES_H
#define VARIABLES_H
/* Ever feel the need for more global vars? */

extern treas_ptr cur_inven;       /* { Current inven page  } */
extern boolean is_magii;	  /* { True if has mana    } */
extern time_t start_time;	 /* { Time started playing} */
extern boolean is_from_file;      /* { True if restored    } */
extern money_type bank;		  /* { Bank's money	 } */
extern money_type coin_value;     /* { Copy of money values} */
extern long player_max_exp;       /* { Max exp possible    } */
extern unsigned long seed;	/* { Contains seed #     } */
extern unsigned long randes_seed; /* { For encoding colors } */
extern unsigned long town_seed;   /* { Seed for town genera} */
extern long cur_height;		  /* { Cur dungeon size    } */
extern long cur_width;
extern long dun_level;   /* { Cur dungeon level   } */
extern long missle_ctr;  /* { Counter for missles } */
extern long msg_line;    /* { Contains message txt} */
extern boolean msg_flag; /* { Set with first msg  } */
extern char msg_prev[MAX_MESSAGES + 1][82];
extern long quest[NUM_QUESTS + 1]; /* {quest data} */
extern char old_msg[82];		   /* { Last message	      } */
extern boolean want_trap;	  /* { True = trap messages} */
extern boolean want_warn;	  /* { True = water warning} */
extern message_ptr old_message;    /* { Past messages	      } */
extern long max_mess_keep;	 /* { Max old to keep     } */
extern long max_score;		   /*	{ # of scores to list } */
extern boolean generate;	   /*	{ Generate next level } */
extern boolean death;		   /*	{ True if died	      } */
extern char died_from[82];		   /*	{ What killed him     } */
extern long turn_counter;	  /*	{ Turns ellapsed      } */
extern boolean find_flag;	  /*	{ Used in MORIA	      } */
extern boolean cave_flag;	  /*	{ Used in GET_PANEL   } */
extern boolean redraw;		   /*	{ For redraw screen   } */
extern unsigned long print_stat;   /*	{ Flag for stats      } */
extern long turn;		   /*	{ Cur trun of game    } */
extern boolean wizard1;		   /*	{ Wizard flag	      } */
extern boolean wizard2;		   /*	{ Wizard flag	      } */
extern boolean used_line[24];      /* array [2..23] of boolean; */
extern char password1[13];
extern char password2[13];
extern boolean became_wizard;
extern unsigned long wdata[2][13]; /*  array [1..2,0..12] of unsigned; */
extern char days[7][30];
extern long closing_flag; /* { Used for closing   } */
/*{neatness arrays} */
extern unsigned char key_of[9];  /*  array [0..8] of unsigned char; */
extern unsigned char oct_of[10]; /*  array [1..9] of unsigned char; */
extern signed char dx_of[10];    /*  array [1..9] of signed char; */
extern signed char dy_of[10];    /*  array [1..9] of signed char; */

/*	{ Bit testing array						} */
extern unsigned long bit_array[33]; /*  array [1..32] of unsigned; */

/*	{ External file names; are all located in directory with image	} */
extern char MORIA_HOU[192];
extern char MORIA_MOR[192];
extern char MORIA_MAS[192];
extern char MORIA_TOP[192];
extern char MORIA_TRD[192];
extern char MORIA_HLP[192];
extern char MORIA_LCK[192];
extern char MORIA_DTH[192];
extern char MORIA_MON[192];
extern char MORIA_CST[192];
extern char MORIA_GCST[192];

/*	{  following are calculated from max dungeon sizes		} */
extern long max_panel_rows, max_panel_cols;
extern long quart_height, quart_width;
extern long panel_row, panel_col;
extern long panel_row_min, panel_row_max;
extern long panel_col_min, panel_col_max;
extern long panel_col_prt, panel_row_prt;

/*	{  Following are all floor definitions				} */
extern row_floor cave[MAX_HEIGHT + 1];
extern cave_type blank_floor;
extern floor_type dopen_floor;
extern floor_type lopen_floor;
extern floor_type corr_floor1;
extern floor_type corr_floor2;
extern floor_type corr_floor3;
extern floor_type corr_floor4;
extern floor_type rock_wall1;
extern floor_type rock_wall2;
extern floor_type rock_wall3;
extern floor_type water1;
extern floor_type water2;
extern floor_type water3;
extern floor_type boundry_wall;

/*	{  Following are set definitions				} */
extern obj_set floor_set;
extern obj_set open_dry_floors;
extern obj_set wall_set;
extern obj_set pwall_set;
extern obj_set corr_set;
extern obj_set trap_set;
extern obj_set light_set;
extern obj_set water_set;
extern obj_set earth_set;
extern obj_set float_set;
extern obj_set slow_set;
extern obj_set stable_set;

/*	{ Following are player variables				} */
extern player_type py;

/*	{ Class titles for different levels				} */

extern char const *player_titles[MAX_CLASS][MAX_PLAYER_LEVEL + 1];
/*				  array [1..max_class] of */
/*				  array [1..max_player_level] of char const *;
 */

extern long exp_per_level[MAX_PLAYER_LEVEL + 1];
extern float acc_exp; /*{ Accumulator for fractional exp} */
extern char bare_hands[7];
extern boolean msg_terse;
extern unsigned char record_ctr;
extern long char_row;
extern long char_col;
extern long com_val;
extern long pclass;
extern char sex_type[82];
extern background_type background[MAX_BACKGROUND];
extern float rgold_adj[MAX_RACES][MAX_RACES];
extern treasure_type yums[NUM_YUM + 1];
extern treasure_type monk_book;
extern unsigned char player_init[MAX_CLASS][5];
extern boolean total_winner;

/*	{ Following are store definitions				} */
extern owner_type owners[MAX_OWNERS];
extern store_type stores[MAX_STORES + 1];
extern treasure_type store_door[MAX_STORES + MAX_UNNAMED + 5 + 1];
extern long store_choice[MAX_STORES][STORE_CHOICES];
extern obj_set store_buy[MAX_STORES];
/*	  array [1..max_stores] of obj_set; */
extern char store_hours[MAX_STORES + MAX_UNNAMED][7][14];
extern long store_bribe[MAX_STORES + MAX_UNNAMED];
extern long mugging_chance; /* { Chance page gets mugged} */

/*	{ Following are treasure arrays	and variables			} */
extern treasure_type object_list[MAX_OBJECTS + 1];
extern boolean object_ident[MAX_OBJECTS + 1];
extern long t_level[MAX_OBJ_LEVEL + 1];
extern treasure_type gold_list[MAX_GOLD];
extern treasure_type t_list[MAX_TALLOC + 1];
extern treasure_type equipment[EQUIP_MAX];
extern treas_ptr inventory_list;
extern treas_ptr inven_temp;
extern treasure_type inventory_init[INVEN_INIT_MAX + 1];
extern treasure_type blank_treasure;
extern long inven_ctr;    /* { Total different obj's	} */
extern long inven_weight; /* { Cur carried weight	} */
extern long equip_ctr;    /* { Cur equipment ctr	} */
extern long tcptr;	/* { Cur treasure heap ptr	} */

/*	{ Following are variables that change with level of difficulty	} */
/*	{ 1/x chance of treasure per magma		} */
extern long dun_str_mc;
/*	{ 1/x chance of treasure per quartz		} */
extern long dun_str_qc;
/*	{ Level/x chance of unusual room		} */
extern long dun_unusual;
/*	{ Amount of objects for rooms			} */
extern long treas_room_alloc;
/*	{ Amount of objects for corridors		} */
extern long treas_any_alloc;
/*	{ Amount of gold (and gems)			} */
extern long treas_gold_alloc;
/*	{ 1/n Chance of item being a Great Item 	} */
extern long obj_great;
/*	{ Adjust STD per level				} */
extern float obj_std_adj;
/*	{ Minimum STD					} */
extern long obj_std_min;
/*	{ Town object generation level			} */
extern long obj_town_level;
/*	{ Base amount of magic				} */
extern long obj_base_magic;
/*	{ Max amount of magic				} */
extern long obj_base_max;
/*	{ magic_chance/# = special magic		} */
extern long obj_div_special;
/*	{ magic_chance/# = cursed items			} */
extern float obj_div_cursed;
/*	{ High value slows multiplication		} */
extern long mon_mult_adj;
/*	{ Dun_level/x chance of high level creature	} */
extern long mon_nasty;

/*	{ Following are feature objects defined for dungeon		} */
extern treasure_type trap_lista[MAX_TRAPA + 1];
extern treasure_type trap_listb[MAX_TRAPB + 1];
extern treasure_type scare_monster; /* { Special trap	} */
extern treasure_type some_rubble;
extern treasure_type door_list[3];
extern treasure_type up_stair;
extern treasure_type down_stair;
extern treasure_type up_steep;
extern treasure_type down_steep;

/*	{ Following are creature arrays and variables			} */
extern creature_type c_list[MAX_CREATURES + 1];
extern monster_type m_list[MAX_MALLOC + 1];
extern long m_level[MAX_MONS_LEVEL + 1];
extern monster_type blank_monster; /* { Blank monster values	} */
extern long muptr;		   /* { Cur used monster ptr	} */
extern long mfptr;		   /* { Cur free monster ptr	} */
extern long mon_tot_mult;	  /* { # of repro's of creature	} */

/*	{ Following are arrays for descriptive pieces			} */
extern char const *colors[MAX_COLORS];
extern char const *mushrooms[MAX_MUSH];
extern char const *woods[MAX_WOODS];
extern char const *metals[MAX_METALS];
extern char const *horns[MAX_HORNS];
extern char const *rocks[MAX_ROCKS];
extern char const *amulets[MAX_AMULETS];
extern char const *cloths[MAX_CLOTHS];
extern char const *syllables[MAX_SYLLABLES];
/*	vowel_set		: */
/*				  char_set; */

/* new stuff */
extern long malloc_calls;
extern long malloc_bytes;
extern long free_calls;
extern long free_bytes;
extern char coin_name[MITHRIL + 1][82];
extern obj_set blank_floor_set;

extern obj_set null_obj_set;
extern obj_set destroyed_by_lightning;
extern obj_set destroyed_by_acid;
extern obj_set destroyed_by_cold;
extern obj_set destroyed_by_fire;
extern obj_set destroyed_by_petrify;
extern obj_set destroyed_by_sunray;

extern gid_t games_gid;
extern boolean scoresAreEncrypted;
extern boolean saveFilesAreEncrypted;

extern unsigned char highScoreKey[8];
extern unsigned char saveFileKey[8];

extern int game_state;
extern boolean curses_is_running;

#include "player.h"

/* END FILE  variables.h */
#endif /* VARIABLES_H */
