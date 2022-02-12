
/* This file doesn't need to know variables, since it defines them */
#define VARIABLES_H

#include <curses.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "configure.h"
#include "constants.h"
#include "debug.h"
#include "magic.h"
#include "pascal.h"
#include "routines.h"
#include "term.h"
#include "types.h"
#include "variables.h"

treas_rec *cur_inven;   /* { Current inven page  } */
money_type bank;        /* { Bank's money	 } */
money_type coin_value = /* { Copy of money values} */
    {0, 1, 4, 20, 240, 960, 12480};
long player_max_exp;       /* { Max exp possible    } */
unsigned long randes_seed; /* { For encoding colors } */
unsigned long town_seed;   /* { Seed for town genera} */
long cur_height;           /* { Cur dungeon size    } */
long cur_width;
long dun_level = 0;             /* { Cur dungeon level   } */
long missle_ctr = 0;            /* { Counter for missles } */
long msg_line = 1;              /* { Contains message txt} */
boolean msg_flag;               /* { Set with first msg  } */
long quest[NUM_QUESTS + 1];     /* {quest data} */
char old_msg[82] = "bogus msg"; /* { Last message	      } */
boolean want_trap;              /* { True = trap messages} */
boolean want_warn;              /* { True = water warning} */
boolean death = false;          /*	{ True if died	      } */
char died_from[82];             /*	{ What killed him     } */
long turn_counter = 100000;     /*	{ Turns ellapsed      } */
boolean find_flag;              /*	{ Used in MORIA	      } */
boolean redraw;                 /*	{ For redraw screen   } */
unsigned long print_stat = 0;   /*	{ Flag for stats      } */
long turn = 0;                  /*	{ Cur trun of game    } */
boolean wizard1 = false;        /*	{ Wizard flag	      } */
boolean wizard2 = false;        /*	{ Wizard flag	      } */
boolean used_line[24] =         /* 22 of false */
    {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};

/*{neatness arrays} */
unsigned char key_of[9] = /*  array [0..8] of unsigned char; */
    {6, 9, 8, 7, 4, 1, 2, 3, 5};
unsigned char oct_of[10] = /*  array [1..9] of unsigned char; */
    {0, 5, 6, 7, 4, 8, 0, 3, 2, 1};
signed char dx_of[10] = /*  array [1..9] of signed char; */
    {0, -1, 0, 1, -1, 0, 1, -1, 0, 1};
signed char dy_of[10] = /*  array [1..9] of signed char; */
    {0, 1, 1, 1, 0, 0, 0, -1, -1, -1};
/*	{ Bit testing array						} */
unsigned long bit_array[33] = /*  array [1..32] of unsigned; */
    {0,          0x00000001, 0x00000002, 0x00000004, 0x00000008, 0x00000010,
     0x00000020, 0x00000040, 0x00000080, 0x00000100, 0x00000200, 0x00000400,
     0x00000800, 0x00001000, 0x00002000, 0x00004000, 0x00008000, 0x00010000,
     0x00020000, 0x00040000, 0x00080000, 0x00100000, 0x00200000, 0x00400000,
     0x00800000, 0x01000000, 0x02000000, 0x04000000, 0x08000000, 0x10000000,
     0x20000000, 0x40000000, 0x80000000};

/*	{  following are calculated from max dungeon sizes		} */
long max_panel_rows, max_panel_cols;
long quart_height = SCREEN_HEIGHT / 4;
long quart_width = SCREEN_WIDTH / 4;
long panel_row, panel_col;
long panel_row_min, panel_row_max;
long panel_col_min, panel_col_max;
long panel_col_prt, panel_row_prt;

/*	{  Following are all floor definitions				} */
row_floor cave[MAX_HEIGHT + 1];
cave_type blank_floor = {0, 0, 0, false, false, false, false, false, 0, 0};
floor_type dopen_floor = {1, true};  /*{ Dark open floor	} */
floor_type lopen_floor = {2, true};  /*{ Light open floor	} */
floor_type corr_floor1 = {4, true};  /*{ Corridor open floor	} */
floor_type corr_floor2 = {5, true};  /*{ Room junction marker} */
floor_type corr_floor3 = {6, true};  /*{ Door type floor	} */
floor_type corr_floor4 = {7, false}; /*{ Secret door type floor} */
/*{ Floor values 8 and 9 are used in generate		} */
floor_type rock_wall1 = {10, false};   /*{ Granite rock wall	} */
floor_type rock_wall2 = {11, false};   /*{ Magma rock wall	} */
floor_type rock_wall3 = {12, false};   /*{ Quartz rock wall	} */
floor_type water1 = {16, true};        /*{ Water on floor	} */
floor_type water2 = {17, true};        /*{ Water on room floor} */
floor_type water3 = {18, true};        /*{ Lit water on floor	} */
floor_type boundry_wall = {15, false}; /*{ Indestructable wall} */

/*	{  Following are set definitions				} */
obj_set floor_set = {1, 2, 4, 5, 6, 7, 16, 17, 18, 0, 0, 0, 0, 0, 0, 0};
obj_set open_dry_floors = {1, 2, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
obj_set wall_set = {10, 11, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
obj_set pwall_set = {10, 11, 12, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
obj_set corr_set = {4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
obj_set trap_set = {unseen_trap, seen_trap, secret_door, entrance_to_store,
                    0,           0,         0,           0,
                    0,           0,         0,           0,
                    0,           0,         0,           0};
obj_set light_set = {seen_trap,
                     rubble,
                     open_door,
                     closed_door,
                     up_staircase,
                     down_staircase,
                     secret_door,
                     entrance_to_store,
                     up_steep_staircase,
                     down_steep_staircase,
                     0,
                     0,
                     0,
                     0,
                     0,
                     0};
obj_set water_set = {16, 17, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
obj_set earth_set = {1, 2, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
obj_set float_set = {arrow,
                     lamp_or_torch,
                     bow_crossbow_or_sling,
                     boots,
                     gloves_and_gauntlets,
                     cloak,
                     soft_armor,
                     scroll1,
                     scroll2,
                     potion1,
                     potion2,
                     flask_of_oil,
                     Food,
                     magic_book,
                     prayer_book,
                     song_book};
obj_set slow_set = {hafted_weapon,
                    pole_arm,
                    dagger,
                    sword,
                    pick_or_shovel,
                    maul,
                    gem_helm,
                    helm,
                    shield,
                    valuable_metal,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0};
obj_set stable_set = {chest, spike, hard_armor, 0, 0, 0, 0, 0,
                      0,     0,     0,          0, 0, 0, 0, 0};

/*	{ Base experience levels, may be adjusted up for race and/or class} */
long exp_per_level[MAX_PLAYER_LEVEL + 1] = {
    0,      10,      25,      45,      70,     100,    140,    200,    280,
    380,    500,     650,     850,     1100,   1400,   1800,   2300,   2900,
    3600,   4400,    5400,    6800,    8400,   10200,  12500,  17500,  25000,
    35000,  50000,   75000,   100000,  150000, 200000, 300000, 400000, 500000,
    750000, 1500000, 2500000, 5000000, 9999999};
char bare_hands[7] = "1d1";
boolean msg_terse;
long char_row = 0;
long char_col = 0;
long com_val;

/*	{ Yums -- A numbered array for eatin' purposes! } */
/* */
/*       some of the food is not good for eating, look in */
/*       treasure.c:mt__armor_and_shields for where they are */
/*       generated.  (also mt__food in the same file) */

treasure_type yums[NUM_YUM + 1] = {
    {"& Bogus Hard Biscuit~", Food, 0x00000000, Nothing_flag, 500, 1, 309, 2, 1,
     0, 0, 0, 0, "0d0", -1, 0},
    {"& Hard Biscuit~", Food, 0x00000000, Nothing_flag, 500, 1, 309, 2, 1, 0, 0,
     0, 0, "0d0", -1, 0},
    {"& Pint of Fine Wine", Food, 0x00000000, Nothing_flag, 400, 2, 312, 10, 1,
     0, 0, 0, 0, "0d0", -1, 0},
    {"& Strip~ of Beef Jerky", Food, 0x00000000, Nothing_flag, 1750, 2, 310, 2,
     1, 0, 0, 0, 0, "0d0", -1, 0},
    {"& Piece~ of Elvish Waybread", Food, 0x00000000, 0x21800020, 3500, 10, 313,
     3, 1, 0, 0, 0, 0, "0d0", -1, 0},
    {"& Stew~", Food, 0x00000000, 0x330001C0, 2000, 0, 314, 3, 1, 0, 0, 0, 0,
     "0d0", -1, 0}, 
    {"& Green Jelly~", Food, 0x00000000, 0x22400060, 4000, 50, 315, 3, 1, 0, 0,
     0, 0, "0d0", -1, 0},
    {"& pint~ of fine grade mush", Food, 0x00000000, 0x00000000, 1500, 0, 306,
     252, 1, 0, 0, 0, 0, "0d0", -1, 0},
    {"& Mushroom~", Food, 0x00000000, Nothing_flag, 3000, 2, 308, 5, 1, 0, 0, 0,
     0, "0d0", -1, 0}, 
    {"& Pint of Fine Ale", Food, 0x00000000, Nothing_flag, 500, 1, 311, 10, 1,
     0, 0, 0, 0, "0d0", -1, 0},
    {"& Handful~ of Berries| (Smurfberries)", Food, 0x00000000, 0x30400000,
     1000, 0, 317, 3, 1, 0, 0, 0, 0, "0d0", -1, 0}, 
    {"& Handful~ of Berries| (Goodberries)", Food, 0x00000000, 0x30C00080, 1000,
     0, 318, 3, 1, 0, 0, 0, 0, "0d0", -1, 0},
    {"& Cool Set of Threads^ [%P6,%P4]", soft_armor, 0x00000000, Nothing_flag,
     0, 45, 11, 75, 1, -1, 0, 3, 0, "0d0", -1, 0},
    {"Filthy Naga Hide Armor^ [%P6,%P4]", soft_armor, 0x00000000, Nothing_flag,
     0, 45, 12, 300, 1, -1, 0, 9, 0, "0d0", -1, 0},
    {"Stone Plate Armor^ [%P6,%P4]", hard_armor, 0x00000000, Nothing_flag, 0,
     45, 14, 600, 1, -6, 0, 10, 0, "2d4", -1, 0},
    {"Elven Chain Mail^ [%P6,%P4]", soft_armor, 0x00000000, Nothing_flag, 0,
     900, 13, 160, 1, -1, 0, 17, 0, "1d2", -1, 0},
    {"Mithril Chain Mail^ [%P6,%P4]", hard_armor, 0x00000000, Nothing_flag, 0,
     1800, 15, 240, 1, -1, 0, 24, 0, "1d4", -1, 0},
    {"Mithril Plate Armor^ [%P6,%P4]", hard_armor, 0x00000000, Nothing_flag, 0,
     3600, 16, 400, 1, -1, 0, 32, 0, "2d4", -1, 0},
    {"& Eyeball~| of Drong", junk_food, 0x00000000, 0x00000000, 300, 1000, 270,
     2, 2, 0, 0, 0, 0, "10d12", 20, 0}, /*{18} */
};

boolean total_winner = false;

/*	{ Following are store definitions				} */
store_type stores[MAX_STORES + 1];

/*	{ Stores are just special traps			} */
treasure_type store_door[MAX_STORES + MAX_UNNAMED + 5 + 1] = {
    {"the entrance to the General Store", entrance_to_store, 0x00000000,
     0x00000000, 0, 0, 101, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Armory", entrance_to_store, 0x00000000, 0x00000000, 0,
     0, 102, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Weapon Smiths", entrance_to_store, 0x00000000,
     0x00000000, 0, 0, 103, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Temple", entrance_to_store, 0x00000000, 0x00000000, 0,
     0, 104, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Alchemy Shop", entrance_to_store, 0x00000000,
     0x00000000, 0, 0, 105, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Magic Shop", entrance_to_store, 0x00000000,
     0x00000000, 0, 0, 106, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Inn", entrance_to_store, 0x00000000, 0x00000000, 0, 0,
     107, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Library", entrance_to_store, 0x00000000, 0x00000000,
     0, 0, 109, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Music Shop", entrance_to_store, 0x00000000,
     0x00000000, 0, 0, 110, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Gem Store", entrance_to_store, 0x00000000, 0x00000000,
     0, 0, 113, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the All-Nite Deli", entrance_to_store, 0x00000000,
     0x00000000, 0, 0, 116, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    /* this is the black market, it looks like a normal door */
    {"the entrance to a building", entrance_to_store, 0x00000000, 0x00000000, 0,
     0, 118, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Trading Post", entrance_to_store, 0x00000000,
     0x00000000, 0, 0, 108, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Insurance Shop", entrance_to_store, 0x00000000,
     0x00000000, 0, 0, 111, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Bank", entrance_to_store, 0x00000000, 0x00000000, 0,
     0, 112, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Money Exchange", entrance_to_store, 0x00000000,
     0x00000000, 0, 0, 114, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Casino", entrance_to_store, 0x00000000, 0x00000000, 0,
     0, 115, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},

    {"the entrance to a strange building", entrance_to_store, 0x00000000,
     0x00000000, 0, 0, 117, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to a building", entrance_to_store, 0x00000000, 0x00000000, 0,
     0, 120, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to a building", entrance_to_store, 0x00000000, 0x00000000, 0,
     0, 121, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to a building", entrance_to_store, 0x00000000, 0x00000000, 0,
     0, 122, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to a building", entrance_to_store, 0x00000000, 0x00000000, 0,
     0, 123, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to a building", entrance_to_store, 0x00000000, 0x00000000, 0,
     0, 124, 0, 0, 0, 0, 0, 0, "0d0", 0, 0}

};

long mugging_chance; /* { Chance page gets mugged} */

boolean object_ident[MAX_OBJECTS + 1]; /*(max_objects of false) */

/*	{ Gold list (All types of gold and gems are defined here)	} */
treasure_type gold_list[MAX_GOLD] = {{"& copper piece~", valuable_metal, 0, 0,
                                      0, 0, 2, 5, 420, 0, 0, 0, 0, " ", 2, 0},
                                     {"& iron piece~", valuable_metal, 0, 0, 0,
                                      0, 1, 5, 2400, 0, 0, 0, 0, " ", 1, 0},
                                     {"& copper piece~", valuable_metal, 0, 0,
                                      0, 0, 4, 5, 720, 0, 0, 0, 0, " ", 2, 0},
                                     {"& silver piece~", valuable_metal, 0, 0,
                                      0, 0, 5, 5, 180, 0, 0, 0, 0, " ", 3, 0},
                                     {"& iron piece~", valuable_metal, 0, 0, 0,
                                      0, 3, 5, 4800, 0, 0, 0, 0, " ", 1, 0},
                                     {"& copper piece~", valuable_metal, 0, 0,
                                      0, 0, 6, 5, 1200, 0, 0, 0, 0, " ", 2, 0},
                                     {"& silver piece~", valuable_metal, 0, 0,
                                      0, 0, 7, 5, 300, 0, 0, 0, 0, " ", 3, 0},
                                     {"& gold piece~", valuable_metal, 0, 0, 0,
                                      0, 12, 5, 30, 0, 0, 0, 0, " ", 4, 0},
                                     {"& iron piece~", valuable_metal, 0, 0, 0,
                                      0, 3, 5, 9000, 0, 0, 0, 0, " ", 1, 0},
                                     {"& copper piece~", valuable_metal, 0, 0,
                                      0, 0, 6, 5, 2400, 0, 0, 0, 0, " ", 2, 0},
                                     {"& silver piece~", valuable_metal, 0, 0,
                                      0, 0, 7, 5, 600, 0, 0, 0, 0, " ", 3, 0},
                                     {"& gold piece~", valuable_metal, 0, 0, 0,
                                      0, 12, 5, 60, 0, 0, 0, 0, " ", 4, 0},
                                     {"& copper piece~", valuable_metal, 0, 0,
                                      0, 0, 6, 5, 6000, 0, 0, 0, 0, " ", 2, 0},
                                     {"& silver piece~", valuable_metal, 0, 0,
                                      0, 0, 7, 5, 1200, 0, 0, 0, 0, " ", 3, 0},
                                     {"& gold piece~", valuable_metal, 0, 0, 0,
                                      0, 12, 5, 120, 0, 0, 0, 0, " ", 4, 0},
                                     {"& silver piece~", valuable_metal, 0, 0,
                                      0, 0, 7, 5, 1500, 0, 0, 0, 0, " ", 3, 0},
                                     {"& gold piece~", valuable_metal, 0, 0, 0,
                                      0, 18, 5, 150, 0, 0, 0, 0, " ", 4, 0},
                                     {"& platinum piece~", valuable_metal, 0, 0,
                                      0, 0, 20, 5, 50, 0, 0, 0, 0, " ", 5, 0},
                                     {"& silver piece~", valuable_metal, 0, 0,
                                      0, 0, 16, 5, 3000, 0, 0, 0, 0, " ", 3, 0},
                                     {"& gold piece~", valuable_metal, 0, 0, 0,
                                      0, 24, 5, 250, 0, 0, 0, 0, " ", 4, 0},
                                     {"& platinum piece~", valuable_metal, 0, 0,
                                      0, 0, 28, 5, 75, 0, 0, 0, 0, " ", 5, 0},
                                     {"& mithril piece~", valuable_metal, 0, 0,
                                      0, 0, 32, 5, 8, 0, 0, 0, 0, " ", 6, 0},
                                     {"& gold piece~", valuable_metal, 0, 0, 0,
                                      0, 50, 5, 600, 0, 0, 0, 0, " ", 4, 0},
                                     {"& platinum piece~", valuable_metal, 0, 0,
                                      0, 0, 55, 5, 200, 0, 0, 0, 0, " ", 5, 0},
                                     {"& mithril piece~", valuable_metal, 0, 0,
                                      0, 0, 60, 5, 20, 0, 0, 0, 0, " ", 6, 0}

};

treasure_type t_list[MAX_TALLOC + 1];
treasure_type equipment[EQUIP_MAX];
treas_rec *inventory_list = NULL;
treas_rec inven_temp = {
    .data = {" ", 0, 0, 0, 0, 0,   0, 0,
        0,   0, 0, 0, 0, " ", 0, 0},
    .ok = false,
    .insides = 0,
    .next = NULL,
    .is_in = false,
};

treasure_type blank_treasure = {" ", 0, 0, 0, 0, 0,   0, 0,
                                0,   0, 0, 0, 0, " ", 0, 0};
long inven_ctr = 0;    /* { Total different obj's} */
long inven_weight = 0; /* { Cur carried weight	} */
long equip_ctr = 0;    /* { Cur equipment ctr	} */

/*	{ Following are feature objects defined for dungeon		} */
/* */

/*	{ Secret door must have same subval as closed door in	} */
/*	{ TRAP_LISTB.  See CHANGE_TRAP				} */
treasure_type door_list[3] = {
    {"an open door", open_door, 0x00000000, 0x00000000, 0, 0, 1, 0, 0, 0, 0, 0,
     0, "1d1", 0, 0},
    {"a closed door", closed_door, 0x00000000, 0x00000000, 0, 0, 19, 0, 0, 0, 0,
     0, 0, "1d1", 0, 0},
    {"a secret door", secret_door, 0x00000000, 0x00000000, 0, 0, 19, 0, 0, 0, 0,
     0, 0, "1d1", 0, 0}};

/*	{ Following are creature arrays and variables			} */
creature_type c_list[MAX_CREATURES + 1];
monster_type m_list[MAX_MALLOC + 1];
long m_level[MAX_MONS_LEVEL + 1];

long muptr;        /* { Cur used monster ptr	} */
long mon_tot_mult; /* { # of repro's of creature	} */

/* new stuff */
char coin_name[MITHRIL + 1][82] = {"total", "iron",     "copper", "silver",
                                   "gold",  "platinum", "mithril"};

unsigned char highScoreKey[8] = {1, 2, 3, 4, 5, 6, 7, 8};
unsigned char saveFileKey[8] = {8, 7, 6, 5, 4, 3, 2, 1};

int game_state;
