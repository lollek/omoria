
/* This file doesn't need to know variables, since it defines them */
#define VARIABLES_H

#include "imoria.h"

treas_ptr cur_inven;    /* { Current inven page  } */
boolean is_from_file;   /* { True if restored    } */
money_type bank;	/* { Bank's money	 } */
money_type coin_value = /* { Copy of money values} */
    {0, 1, 4, 20, 240, 960, 12480};
long player_max_exp;       /* { Max exp possible    } */
unsigned long seed;	/* { Contains seed #     } */
unsigned long randes_seed; /* { For encoding colors } */
unsigned long town_seed;   /* { Seed for town genera} */
long cur_height;	   /* { Cur dungeon size    } */
long cur_width;
long dun_level;      /* { Cur dungeon level   } */
long missle_ctr = 0; /* { Counter for missles } */
long msg_line;       /* { Contains message txt} */
boolean msg_flag;    /* { Set with first msg  } */
char msg_prev[MAX_MESSAGES + 1][82];
long quest[NUM_QUESTS + 1];   /* {quest data} */
char old_msg[82] = "bogus msg";  /* { Last message	      } */
boolean want_trap;	    /* { True = trap messages} */
boolean want_warn;	    /* { True = water warning} */
message_ptr old_message;      /* { Past messages	      } */
long max_mess_keep;	   /* { Max old to keep     } */
long max_score;		      /*	{ # of scores to list } */
boolean generate;	     /*	{ Generate next level } */
boolean death = false;	/*	{ True if died	      } */
char died_from[82];	      /*	{ What killed him     } */
long turn_counter;	    /*	{ Turns ellapsed      } */
boolean find_flag;	    /*	{ Used in MORIA	      } */
boolean cave_flag;	    /*	{ Used in GET_PANEL   } */
boolean redraw;		      /*	{ For redraw screen   } */
unsigned long print_stat = 0; /*	{ Flag for stats      } */
long turn = 0;		      /*	{ Cur trun of game    } */
boolean wizard1 = false;      /*	{ Wizard flag	      } */
boolean wizard2 = false;      /*	{ Wizard flag	      } */
boolean used_line[24] =       /* 22 of false */
    {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
char password1[13];
char password2[13];
boolean became_wizard = false;
unsigned long wdata[2][13] = /*  array [1..2,0..12] of unsigned; */
    {{31415, 81, 58, 35, 193, 3, 41, 49, 228, 2, 85, 9, 125},
     {92653, 24, 166, 38, 92, 31, 137, 155, 177, 239, 79, 236, 112}};

/*	{{31415,'n','a','f','f',0,'w','z','r','i','n','b'}, */
/*	{92653,'m','o','s','k','a',0,'j','a','u','m','z'}}; */

/**
 * -RAK-
 *  Operating hours for Moria
 *  X = Open
 *  . = Closed
 */
char days[7][30] = {
    "SUN:XXXXXXXXXXXXXXXXXXXXXXXX|", "MON:XXXXXXXXXXXXXXXXXXXXXXXX|",
    "TUE:XXXXXXXXXXXXXXXXXXXXXXXX|", "WED:XXXXXXXXXXXXXXXXXXXXXXXX|",
    "THU:XXXXXXXXXXXXXXXXXXXXXXXX|", "FRI:XXXXXXXXXXXXXXXXXXXXXXXX|",
    "SAT:XXXXXXXXXXXXXXXXXXXXXXXX|"};
long closing_flag = 0;    /* { Used for closing   } */
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
    {0,		 0x00000001, 0x00000002, 0x00000004, 0x00000008, 0x00000010,
     0x00000020, 0x00000040, 0x00000080, 0x00000100, 0x00000200, 0x00000400,
     0x00000800, 0x00001000, 0x00002000, 0x00004000, 0x00008000, 0x00010000,
     0x00020000, 0x00040000, 0x00080000, 0x00100000, 0x00200000, 0x00400000,
     0x00800000, 0x01000000, 0x02000000, 0x04000000, 0x08000000, 0x10000000,
     0x20000000, 0x40000000, 0x80000000};

/*	{ External file names; paths are set in io.c get_paths } */
char MORIA_HOU[192];
char MORIA_TRD[192];
/*	vtype		MORIA_HLP; */
char MORIA_LCK[192];
char MORIA_DTH[192];
char MORIA_MON[192];
char MORIA_CST[192];
char MORIA_GCST[192];

/*	{  following are calculated from max dungeon sizes		} */
long max_panel_rows, max_panel_cols;
long quart_height, quart_width;
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
floor_type water1 = {16, true};	/*{ Water on floor	} */
floor_type water2 = {17, true};	/*{ Water on room floor} */
floor_type water3 = {18, true};	/*{ Lit water on floor	} */
floor_type boundry_wall = {15, false}; /*{ Indestructable wall} */

/*	{  Following are set definitions				} */
obj_set floor_set = {1, 2, 4, 5, 6, 7, 16, 17, 18, 0, 0, 0, 0, 0, 0, 0};
obj_set open_dry_floors = {1, 2, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
obj_set wall_set = {10, 11, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
obj_set pwall_set = {10, 11, 12, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
obj_set corr_set = {4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
obj_set trap_set = {
    unseen_trap, seen_trap, secret_door, entrance_to_store, 0, 0, 0, 0,
    0,		 0,	 0,		 0,		    0, 0, 0, 0};
obj_set light_set = {
    seen_trap,		rubble,		      open_door,   closed_door,
    up_staircase,       down_staircase,       secret_door, entrance_to_store,
    up_steep_staircase, down_steep_staircase, 0,	   0,
    0,			0,		      0,	   0};
obj_set water_set = {16, 17, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
obj_set earth_set = {1, 2, 4, 5, 6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
obj_set float_set = {
    arrow,		  lamp_or_torch, bow_crossbow_or_sling, boots,
    gloves_and_gauntlets, cloak,	 soft_armor,		scroll1,
    scroll2,		  potion1,       potion2,		flask_of_oil,
    Food,		  magic_book,    prayer_book,		song_book};
obj_set slow_set = {hafted_weapon,  pole_arm,       dagger,   sword,
		    pick_or_shovel, maul,	   gem_helm, helm,
		    shield,	 valuable_metal, 0,	0,
		    0,		    0,		    0,	0};
obj_set stable_set = {chest, spike, hard_armor, 0, 0, 0, 0, 0,
		      0,     0,     0,		0, 0, 0, 0, 0};

/*	{ Following are player variables				} */
p_flags player_flags = {
    false,
     false,
     0,
     0,
     0,
     0,
     0,
     7500,
     2,
     0,
     0,
     0,
     false,
     0,
     0,
     0,
     0,
     0,
     0,
     0,
     0,
     0,
     0,
     0,
     0,
     0,
     0,
     0,
     0,
     0,
     0,
     0,
     0,
     false,
     false,
     false,
     false,
     false,
     false,
     false,
     false,
     false,
     false,
     false,
     false,
     {false, false, false, false, false, false}, /* sustain */
     false,
     0,
     0,
     0,
     0,
     0,
     0,
     0,
     0,
     0,
     0,
     false,
     false,
     false,
     false};

/*	{ Base experience levels, may be adjusted up for race and/or class} */
long exp_per_level[MAX_PLAYER_LEVEL + 1] = {
    0,      10,      25,      45,      70,     100,    140,    200,    280,
    380,    500,     650,     850,     1100,   1400,   1800,   2300,   2900,
    3600,   4400,    5400,    6800,    8400,   10200,  12500,  17500,  25000,
    35000,  50000,   75000,   100000,  150000, 200000, 300000, 400000, 500000,
    750000, 1500000, 2500000, 5000000, 9999999};
float acc_exp = 0.0; /*{ Accumulator for fractional exp} */
char bare_hands[7] = "1d1";
boolean msg_terse;
unsigned char record_ctr = 0;
long char_row = 0;
long char_col = 0;
long com_val;
char sex_type[82] = "FemaleMale  ";

/*	{ Background information					} */

/*	{ Buying and selling adjustments for character race VS store	} */
/*	{ owner race							} */
float rgold_adj[MAX_RACES][MAX_RACES] = {
    /*             {  Hum,  HfE,  Elf,  Hal,  Gno,  Dwa,  HfO,  HfT,  Phr,
       Dry */
    /*             } */
    /*Human	     */ {0.00, 0.05, 0.05, 0.10, 0.13, 0.15, 0.20, 0.25, 0.20,
			 0.05},
    /*Half-Elf   */ {0.10, 0.00, 0.00, 0.05, 0.10, 0.20, 0.25, 0.30, 0.25,
		     0.05},
    /*Elf	     */ {0.10, 0.05, 0.00, 0.05, 0.10, 0.20, 0.25, 0.30, 0.30,
			 0.00},
    /*Halfling   */ {0.15, 0.10, 0.05, -0.05, 0.05, 0.10, 0.15, 0.30, 0.25,
		     0.05},
    /*Gnome	     */ {0.15, 0.15, 0.10, 0.05, -0.05, 0.10, 0.15, 0.30, 0.20,
			 0.15},
    /*Dwarf	     */ {0.15, 0.20, 0.20, 0.10, 0.10, -0.05, 0.25, 0.35, 0.15,
			 0.30},
    /*Half-Orc   */ {0.15, 0.20, 0.25, 0.15, 0.15, 0.30, 0.10, 0.15, 0.15,
		     0.25},
    /*Half-Troll */ {0.10, 0.15, 0.15, 0.10, 0.10, 0.30, 0.10, 0.10, 0.15,
		     0.25},
    /*Phraint    */ {0.20, 0.25, 0.30, 0.25, 0.20, 0.15, 0.15, 0.15, -0.10,
		     0.20},
    /*Dryad	     */ {0.10, 0.05, 0.05, 0.05, 0.15, 0.30, 0.30, 0.25, 0.20,
			 -0.05}

};

/*	{ Yums -- A numbered array for eatin' purposes! } */
/* */
/*       some of the food is not good for eating, look in */
/*       treasure.c:mt__armor_and_shields for where they are */
/*       generated.  (also mt__food in the same file) */

treasure_type yums[NUM_YUM + 1] = {
    {"& Bogus Hard Biscuit~", Food, 0x00000000, Nothing_flag, 500, 1, 309,
     2, 1, 0, 0, 0, 0, "0d0", -1, 0}, /*{0} */
    {"& Hard Biscuit~", Food, 0x00000000, Nothing_flag, 500, 1, 309, 2, 1,
     0, 0, 0, 0, "0d0", -1, 0}, /*{1} */
    {"& Pint of Fine Wine", Food, 0x00000000, Nothing_flag, 400, 2, 312,
     10, 1, 0, 0, 0, 0, "0d0", -1, 0}, /*{2} */
    {"& Strip~ of Beef Jerky", Food, 0x00000000, Nothing_flag, 1750, 2,
     310, 2, 1, 0, 0, 0, 0, "0d0", -1, 0}, /*{3} */
    {"& Piece~ of Elvish Waybread", Food, 0x00000000, 0x21800020, 3500, 10,
     313, 3, 1, 0, 0, 0, 0, "0d0", -1, 0}, /*{4} */
    {"& Stew~", Food, 0x00000000,
     0x330001C0, 2000, 0, 314, 3, 1, 0, 0, 0, 0, "0d0", -1, 0}, /*{5} */
    {"& Green Jelly~", Food, 0x00000000, 0x22400060,
     4000, 50, 315, 3, 1, 0, 0, 0, 0, "0d0", -1, 0}, /*{6} */
    {"& pint~ of fine grade mush", Food, 0x00000000, 0x00000000, 1500, 0,
     306, 252, 1, 0, 0, 0, 0, "0d0", -1, 0}, /*{7} */
    {"& Mushroom~", Food, 0x00000000, Nothing_flag, 3000, 2, 308, 5, 1,
     0, 0, 0, 0, "0d0", -1, 0}, /*{8} */
    {"& Pint of Fine Ale", Food, 0x00000000, Nothing_flag, 500, 1, 311, 10,
     1, 0, 0, 0, 0, "0d0", -1, 0}, /*{9} */
    {"& Handful~ of Berries| (Smurfberries)", Food, 0x00000000, 0x30400000,
     1000, 0, 317, 3, 1, 0, 0, 0, 0, "0d0", -1, 0}, /*{10} */
    {"& Handful~ of Berries| (Goodberries)", Food, 0x00000000, 0x30C00080,
     1000, 0, 318, 3, 1, 0, 0, 0, 0, "0d0", -1, 0}, /*{11} */
    {"& Cool Set of Threads^ [%P6,%P4]", soft_armor, 0x00000000,
     Nothing_flag, 0, 45, 11, 75, 1, -1, 0, 3, 0, "0d0", -1, 0}, /*{12} */
    {"Filthy Naga Hide Armor^ [%P6,%P4]", soft_armor, 0x00000000,
     Nothing_flag, 0, 45, 12, 300, 1, -1, 0, 9, 0, "0d0", -1, 0}, /*{13} */
    {"Stone Plate Armor^ [%P6,%P4]", hard_armor, 0x00000000, Nothing_flag,
     0, 45, 14, 600, 1, -6, 0, 10, 0, "2d4", -1, 0}, /*{14} */
    {"Elven Chain Mail^ [%P6,%P4]", soft_armor, 0x00000000, Nothing_flag,
     0, 900, 13, 160, 1, -1, 0, 17, 0, "1d2", -1, 0}, /*{15} */
    {"Mithril Chain Mail^ [%P6,%P4]", hard_armor, 0x00000000, Nothing_flag,
     0, 1800, 15, 240, 1, -1, 0, 24, 0, "1d4", -1, 0}, /*{16} */
    {"Mithril Plate Armor^ [%P6,%P4]", hard_armor, 0x00000000,
     Nothing_flag, 0, 3600, 16, 400, 1, -1, 0, 32, 0, "2d4", -1, 0}, /*{17} */
    {"& Eyeball~| of Drong", junk_food, 0x00000000, 0x00000000, 300, 1000,
     270, 2, 2, 0, 0, 0, 0, "10d12", 20, 0}, /*{18} */
};

/*	{ Create fake book for using monk disciplines		} */
treasure_type monk_book = {
    "& Book of Mental Disciplines", miscellaneous_object, 0x00000000,
    0x00003FFF,			    0,			  0,     1,
    10,				    1,			  0,     0,
    0,				    0,			  "0d0", 1, 0};

boolean total_winner = false;

/*	{ Following are store definitions				} */
store_type stores[MAX_STORES + 1];

/*	{ Store owners have different characteristics for pricing and haggling}
 */
/*	{ Note: Store owners should be added in groups, one for each store    }
 */
owner_type owners[MAX_OWNERS] = {
    /* {set number one} */
    {"Erick the Honest       (Human)      General Store", 2500, 0.75, 0.08,
     0.04, 0, 12},
    {"Mauglin the Grumpy     (Dwarf)      Armory", 32000, 1.00, 0.12, 0.04, 5,
     5},
    {"Arndal Beast-Slayer    (Half-Elf)   Weaponsmith", 10000, 0.85, 0.10, 0.05,
     1, 8},
    {"Hardblow the Humble    (Human)      Temple", 3500, 0.75, 0.09, 0.06, 0,
     15},
    {"Ga-nat the Greedy      (Gnome)      Alchemist", 12000, 1.20, 0.15, 0.04,
     4, 9},
    {"Valeria Starshine      (Elf)        Magic Shop", 32000, 0.75, 0.10, 0.05,
     2, 11},
    {"Tika Majere            (Human)      Inn", 32000, 1.00, 0.08, 0.05, 0, 7},
    {"Socrates the Philosopher  (Human)  Library", 5000, 1.00, 0.10, 0.05, 0,
     10},
    {"Dysella of Oakglade    (Dryad)      Music Shop", 10000, 1.00, 0.10, 0.05,
     9, 10},
    {"The Dragon Master      (Human)       Gem Store", 15000, 0.95, 0.10, 0.05,
     0, 5},
    {"Grond the Grotesque   (Half-Orc)     All-Nite Deli", 3000, 1.00, 0.10,
     0.05, 6, 5},
    {"Ugluk the Ugly        (Orc)          Black Market", 500000, 2.50, 1.5,
     0.01, 6, 6},

    /*{set number two} */
    {"Andy the Friendly      (Halfling)   General Store", 2000, 0.70, 0.08,
     0.05, 3, 15},
    {"Darg-Low the Grim      (Human)      Armory", 10000, 0.90, 0.11, 0.04, 0,
     9},
    {"Oglign Dragon-Slayer   (Dwarf)      Weaponsmith", 32000, 0.95, 0.12, 0.04,
     5, 8},
    {"Gunnar the Paladin     (Human)      Temple", 5000, 0.85, 0.10, 0.05, 0,
     23},
    {"Mauser the Chemist     (Half-Elf)   Alchemist", 10000, 0.90, 0.11, 0.05,
     1, 8},
    {"K'rek Kwith the Quick  (Phraint)    Magic Shop", 32000, 0.90, 0.10, 0.05,
     8, 5},
    {"Samwise                (Halfling)   Inn", 32000, 0.70, 0.10, 0.05, 3, 12},
    {"Elrond the Wise        (Elf)        Library", 5000, 1.00, 0.10, 0.05, 2,
     10},
    {"Shaun the Bard         (Human)    Music Shop", 10000, 1.00, 0.10, 0.05, 0,
     10},
    {"Ari-San                (Elf)       Gem Store", 15000, 1.00, 0.10, 0.05, 2,
     10},
    {"Gerald Ciceau		 (Human)     All-Nite Deli", 3000, 1.00, 0.07,
     0.05, 0, 5},
    {"Gloin the Fierce       (Dwarf)     Black Market", 500000, 2.5, 1.5, 0.01,
     5, 4},

    /*{set number three} */
    {"Lyar-el the Comely     (Elf)        General Store", 3000, 0.65, 0.07,
     0.06, 2, 18},
    {"Mauglim the Horrible   (Half-Orc)   Armory", 3000, 1.00, 0.13, 0.05, 6,
     9},
    {"Ithyl-Mak the Beastly	 (Half-Troll) Weaponsmith", 3000, 1.10, 0.15,
     0.06, 7, 8},
    {"Delihla the Pure       (Half-Elf)   Temple", 25000, 0.80, 0.07, 0.06, 1,
     20},
    {"Wizzle the Chaotic     (Halfling)   Alchemist", 10000, 0.90, 0.10, 0.06,
     3, 8},
    {"Inglorian the Mage     (Human?)     Magic Shop", 32000, 1.00, 0.10, 0.07,
     0, 10},
    {"Lucas the Portly       (Human)      Inn", 32000, 0.75, 0.10, 0.03, 0, 3},
    {"Dyxel the Beautiful    (Dryad)      Library", 5000, 1.00, 0.10, 0.05, 9,
     15},
    {"Roland the Melodic     (Halfling)   Music Shop", 10000, 1.00, 0.10, 0.05,
     3, 10},
    {"Galton the turrible    (Half-Orc)   Gem Store", 15000, 0.95, 0.20, 0.05,
     6, 4},
    {"Joseph Tansli		 (Human)      All-Nite Deli", 3000, 1.00, 0.10,
     0.05, 0, 10},
    {"Grima Wormtongue       (Human?)     Black Market", 500000, 2.5, 1.5, 0.01,
     0, 5},

    /*{set number four} */
    {"Felimid mac Fal        (Half-Elf)   General Store", 3500, 1.10, 0.15,
     0.10, 1, 5},
    {"Andre the Dull         (Half-Troll) Armory", 10000, 1.00, 0.08, 0.04, 6,
     8},
    {"Vlad Taltos            (Human)      Weaponsmith", 25000, 0.80, 0.10, 0.03,
     0, 15},
    {"Brother Maynard        (Human)      Temple", 15000, 1.00, 0.15, 0.08, 0,
     5},
    {"Questor Thews	         (Gnome)      Alchemist", 20000, 0.70, 0.10,
     0.02, 4, 10},
    {"Gopher the Great!      (Gnome)      Magic Shop", 20000, 1.15, 0.13, 0.06,
     4, 10},
    {"Mike the *Very* large  (Phraint)    Inn", 32000, 0.90, 0.10, 0.05, 8, 1},
    {"Kelstor the Sage       (Human)      Library", 5000, 1.00, 0.10, 0.05, 0,
     10},
    {"K'phelt the Drone     (Phraint)    Music Shop", 10000, 1.00, 0.10, 0.05,
     8, 10},
    {"Daphnea the Tender     (Dryad)      Gem Store", 15000, 0.80, 0.10, 0.05,
     9, 3},
    {"Clarion the Exotic 	 (Dryad)      All-Nite Deli", 3000, 1.00, 0.10,
     0.05, 9, 5},
    {"Netta Winsome          (Dryad)      Black Market", 500000, 2.5, 1.4, 0.02,
     9, 9}

};

/*	{ Stores are just special traps			} */
treasure_type store_door[MAX_STORES + MAX_UNNAMED + 5 + 1] = {
    {"the entrance to the General Store", entrance_to_store,
     0x00000000, 0x00000000, 0, 0, 101, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Armory", entrance_to_store, 0x00000000,
     0x00000000, 0, 0, 102, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Weapon Smiths", entrance_to_store,
     0x00000000, 0x00000000, 0, 0, 103, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Temple", entrance_to_store, 0x00000000,
     0x00000000, 0, 0, 104, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Alchemy Shop", entrance_to_store,
     0x00000000, 0x00000000, 0, 0, 105, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Magic Shop", entrance_to_store,
     0x00000000, 0x00000000, 0, 0, 106, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Inn", entrance_to_store, 0x00000000,
     0x00000000, 0, 0, 107, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Library", entrance_to_store, 0x00000000,
     0x00000000, 0, 0, 109, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Music Shop", entrance_to_store,
     0x00000000, 0x00000000, 0, 0, 110, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Gem Store", entrance_to_store,
     0x00000000, 0x00000000, 0, 0, 113, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the All-Nite Deli", entrance_to_store,
     0x00000000, 0x00000000, 0, 0, 116, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    /* this is the black market, it looks like a normal door */
    {"the entrance to a building", entrance_to_store, 0x00000000,
     0x00000000, 0, 0, 118, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Trading Post", entrance_to_store,
     0x00000000, 0x00000000, 0, 0, 108, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Insurance Shop", entrance_to_store,
     0x00000000, 0x00000000, 0, 0, 111, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Bank", entrance_to_store, 0x00000000,
     0x00000000, 0, 0, 112, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Money Exchange", entrance_to_store,
     0x00000000, 0x00000000, 0, 0, 114, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to the Casino", entrance_to_store, 0x00000000,
     0x00000000, 0, 0, 115, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},

    {"the entrance to a strange building", entrance_to_store,
     0x00000000, 0x00000000, 0, 0, 117, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to a building", entrance_to_store, 0x00000000,
     0x00000000, 0, 0, 120, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to a building", entrance_to_store, 0x00000000,
     0x00000000, 0, 0, 121, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to a building", entrance_to_store, 0x00000000,
     0x00000000, 0, 0, 122, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to a building", entrance_to_store, 0x00000000,
     0x00000000, 0, 0, 123, 0, 0, 0, 0, 0, 0, "0d0", 0, 0},
    {"the entrance to a building", entrance_to_store, 0x00000000,
     0x00000000, 0, 0, 124, 0, 0, 0, 0, 0, 0, "0d0", 0, 0}

};

/*{ Note : Raised from 26 to 50 possible choices		-DMF-	} */
long store_choice[MAX_STORES][STORE_CHOICES] = {
    /*	{ General Store } */
    {105, 104, 103, 102, 105, 104, 42, 105, 27, 26, 5, 4, 3, 3, 2, 102, 103,
     104, 105, 1, 1, 1, 2, 2, 3, 3, 1, 1, 1, 1, 4, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
     1, 1, 1, 27, 26, 4, 103, 104, 105},
    /*	{ Armory	} */
    {30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 43, 44, 45, 46, 47, 30, 33,
     34, 43, 44, 28, 29, 30, 31, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
     43, 44, 45, 46, 47, 28, 29, 30, 31, 30, 32, 35},
    /*	{ Weaponsmith	} */
    {6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
     6, 7, 23, 25, 23, 25, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
     20, 21, 22, 23, 24, 25, 6, 7, 156, 156},
    /*	{ Temple	} */
    {59, 59, 77, 79, 80, 81, 84, 85, 13, 14, 15, 96, 97, 98, 100, 79, 79, 80,
     80, 81, 98, 59, 77, 80, 81, 84, 59, 59, 77, 79, 80, 81, 84, 85, 13, 14, 15,
     96, 97, 98, 59, 77, 80, 81, 84, 85, 13, 156, 156, 96},
    /*	{ Alchemy shop	} */
    {55, 56, 57, 58, 58, 60, 61, 62, 63, 64, 65, 66, 75, 76, 78, 82, 83, 60, 61,
     62, 63, 64, 98, 99, 98, 98, 62, 63, 64, 65, 66, 75, 76, 78, 82, 83, 60, 61,
     55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66},
    /*	{ Magic shop	} */
    {49, 50, 51, 52, 53, 54, 48, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 86,
     101, 88, 49, 50, 51, 52, 53, 54, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95,
     86, 101, 49, 50, 51, 52, 53, 54, 48, 86, 87, 88, 89, 90},
    /*	{ Inn		} */
    {106, 106, 106, 106, 106, 106, 106, 106, 106, 106, 106, 106, 106, 106, 106,
     106, 106, 106, 106, 106, 106, 106, 106, 106, 106, 115, 115, 115, 115, 115,
     115, 115, 115, 116, 116, 116, 116, 116, 116, 116, 116, 157, 157, 157, 157,
     157, 157, 157, 157, 157},
    /*	{ Library	} */
    {67, 68, 69, 70, 71, 72, 73, 74, 67, 68, 69, 70, 71, 72, 58, 62, 63, 64, 65,
     68, 67, 71, 68, 72, 67, 68, 69, 70, 71, 72, 73, 74, 68, 58, 62, 63, 64, 65,
     73, 68, 69, 70, 67, 68, 71, 58, 62, 63, 64, 65},
    /*	{ Music Shop	} */
    {107, 108, 109, 110, 111, 112, 113, 114, 114, 107, 108, 109, 110, 111, 112,
     113, 113, 114, 107, 108, 109, 110, 111, 112, 113, 114, 112, 107, 107, 107,
     108, 108, 108, 109, 109, 109, 110, 110, 111, 112, 117, 118, 119, 120, 121,
     117, 118, 119, 120, 121},
    /*	{ Gem Shop	} */
    {122, 137, 142, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 122,
     124, 125, 126, 126, 127, 127, 134, 129, 130, 133, 131, 135, 133, 122, 134,
     122, 122, 130, 136, 140, 124, 122, 125, 123, 137, 124, 125, 126, 141, 138,
     129, 130, 131, 132, 139},
    /*	{ All-Nite Deli } */
    {144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 143, 144, 145,
     146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 143, 144, 145, 146, 147,
     148, 149, 150, 151, 152, 153, 154, 155, 143, 144, 145, 146, 148, 149, 151,
     152, 153, 154, 155, 143},
    /*	{ Black Market } */
    {1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
     1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1}

};

/*	{ Each store will buy only certain items, based on TVAL } */
obj_set store_buy[MAX_STORES] = {
    /* {General Store	} */
    {1, 2, 3, 13, 15, 25, 30, 32, 38, 77, 80, 85, 86, 0, 0, 0},
    /* {Armory		} */
    {29, 30, 31, 33, 34, 35, 36, 37, 0, 0, 0, 0, 0, 0, 0, 0},
    /* {Weaponsmith	} */
    {10, 11, 12, 20, 21, 22, 23, 24, 26, 0, 0, 0, 0, 0, 0, 0},
    /* {Temple		} */
    {21, 70, 71, 75, 76, 91, 94, 0, 0, 0, 0, 0, 0, 0, 0, 0},
    /* {Alchemy shop	} */
    {70, 71, 75, 76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
    /* {Magic Shop	} */
    {1, 3, 6, 40, 45, 55, 60, 65, 90, 0, 0, 0, 0, 0, 0, 0},
    /* {Inn	 	} */
    {106, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
    /* {Library	} */
    {71, 70, 90, 91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
    /* {Music Shop	} */
    {85, 86, 92, 93, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
    /* {Jewelry Shop	} */
    {4, 5, 7, 29, 37, 38, 40, 45, 0, 0, 0, 0, 0, 0, 0, 0},
    /* {All-Night Deli } */
    {81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
    /* {Black Market } */
    {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}};

/*	{ Hours that a store is open, in two hour increments. */
/*		  = Open */
/*		N = 'Closed for the night' */
/*		W = 'Closed for the weekend' */
/*		D = 'Closed for the day' */
/*		B = Bribeable */
/*					} */
char store_hours[MAX_STORES + MAX_UNNAMED][7][14] =
    /* Mon, Tue, Wed, Thu, Fri, Sat, Sun */
    {
     /*{General Store	} */
     {"NNB       BN", "NNB       BN", "NNB       BN", "NNB       BN",
      "NNB       BN", "NNB       BN", "NNB       BN"},
     /*{Armory		} */
     {"NNNB     NNN", "NNNB     NNN", "WWWB     NNN", "NNNB     NNN",
      "NNNB     NNN", "NNNB     WWW", "WWWWWWWWWWWW"},
     /*{Weapon Smiths	} */
     {"NNNB     NNN", "NNNB     NNN", "WWWB     NNN", "NNNB     NNN",
      "NNNB     NNN", "NNNB     WWW", "WWWWWWWWWWWW"},
     /*{Temple		} */
     {"            ", "            ", "            ", "            ",
      "            ", "            ", "            "},
     /*{Alchemy Shop	} */
     {"NNNN    NNNN", "NNNN    NNNN", "NNNN    NNNN", "NNNN    NNNN",
      "NNNN    NNNN", "NNNN    NNNN", "NNNN    NNNN"},
     /*{Magic Shop	} */
     {"NNNN    NNNN", "NNNN    NNNN", "NNNN    NNNN", "NNNN    NNNN",
      "NNNN    NNNN", "NNNN    NNNN", "NNNN    NNNN"},
     /*{Inn		} */
     {"            ", "            ", "            ", "            ",
      "            ", "            ", "            "},
     /*{Library	} */
     {"NNN        N", "NNN        N", "NNN        N", "NNN        N",
      "NNN        N", "NNN        N", "NNN        N"},
     /*{Music Shop 	} */
     {"NNNB    BNNN", "NNNB   BNNNN", "NNNB    BNNN", "NNNB    BNNN",
      "NNNB    BNNN", "NNNB    BNNN", "NNNB    BNNN"},
     /*{Gem Store	} */
     {"NNNB     BNN", "NNNB   BNNNN", "NNNB     BNN", "NNNB     BNN",
      "NNNB     BNN", "NNNB     BNN", "NNNB     BNN"},
     /*{All-Night Deli } */
     {"            ", "            ", "            ", "            ",
      "            ", "            ", "            "},
     /*{Black Market } */
     {"BBBBBBBBBBBB", "BBBBBBBBBBBB", "BBBBBBBBBBBB", "BBBBBBBBBBBB",
      "BBBBBBBBBBBB", "BBBBBBBBBBBB", "BBBBBBBBBBBB"},
     /*{Trading Post	} */
     {"NNNB      BN", "NNNB      BN", "WWWB      BN", "NNNB      BN",
      "NNNB      BN", "NNNB      BW", "WWWWWWWWWWWW"},
     /*{Insurance Shop	} */
     {"BBBB     BBB", "BBBB     BBB", "BBBB     BBB", "BBBB     BBB",
      "BBBB     BBB", "BBBB     BBB", "BBBB     BBB"},
     /*{Bank		} */
     {"NNNN     NNN", "NNNN     NNN", "WWWW     NNN", "NNNN     NNN",
      "NNNN     NNN", "NNNN    WWWW", "WWWWWWWWWWWW"},
     /*{Money Exchange	} */
     {"NNNB     BNN", "NNNB     BNN", "WWWB     BNN", "NNNB     BNN",
      "NNNB     BNN", "NNNB     BWW", "WWWWWWWWWWWW"},
     /*{Casino		} */
     {"            ", "            ", "   DDDDDD   ", "   DDDDDD   ",
      "   DDDDDD   ", "   DDDDDD   ", "   DDDDDD   "},
     /*{Quest Store    } */
     {"            ", "            ", "            ", "            ",
      "            ", "            ", "            "}};

/*	{ Store owners can be bribed to open up their shop during */
/*	  certain hours (so that you can always have the opportunity to */
/*	  buy insurance, and suchlike.)					} */
long store_bribe[MAX_STORES + MAX_UNNAMED] = {
    50,  /*  0 general */
    150, /*  1 armory */
    150, /*  2 weapons */
    75,  /*  3 temple */
    75,  /*  4 alchemy */
    300, /*  5 magic */
    25,  /*  6 inn */
    100, /*  7 library */
    50,  /*  8 music */
    75,  /*  9 gem */
    0,   /* 10 deli */
    100, /* 11 black market */
    50,  /* 12 trade post */
    100, /* 13 insurance */
    100, /* 14 bank */
    25,  /* 15 money changer */
    100, /* 16 casino */
    0    /* 17 fortress */
};

long mugging_chance; /* { Chance page gets mugged} */

/*	{ Following are treasure arrays	and variables			} */
/*      Search for MAX_OBJECTS to get to the end of the list */

/*	{ Object list[1..max_objects] (All objects must be defined here)} */
treasure_type object_list[MAX_OBJECTS + 1] = {
    {"& %M Bogus Object~| of Doom", Food, 0x00000000, 0x00000001, 500, 0,
     257, 1, 1, 0, 0, 0, 0, "0d0", 7, 0},
    {"& %M Mushroom~| of Poison", Food, 0x00000000, 0x00000001, 500, 0,
     257, 1, 1, 0, 0, 0, 0, "0d0", 7, 0},
    {"& %M Mushroom~| of Blindness", Food, 0x00000000, 0x00000002, 500, 0,
     258, 1, 1, 0, 0, 0, 0, "0d0", 9, 0},
    {"& %M Mushroom~| of Paranoia", Food, 0x00000000, 0x00000004, 500, 0,
     259, 1, 1, 0, 0, 0, 0, "0d0", 9, 0},
    {"& %M Mushroom~| of Confusion", Food, 0x00000000, 0x00000008, 500, 0,
     260, 1, 1, 0, 0, 0, 0, "0d0", 7, 0},
    {"& %M Mushroom~| of Hallucination", Food, 0x00000000, 0x00000010, 500,
     0, 261, 1, 1, 0, 0, 0, 0, "0d0", 13, 0},
    {"& %M Mushroom~| of Cure Poison", Food, 0x00000000, 0x00000020, 500,
     60, 262, 1, 1, 0, 0, 0, 0, "0d0", 8, 0},
    {"& %M Mushroom~| of Cure Blindness", Food, 0x00000000, 0x00000040,
     500, 50, 263, 1, 1, 0, 0, 0, 0, "0d0", 10, 0},
    {"& %M Mushroom~| of Cure Paranoia", Food, 0x00000000, 0x00000080, 500,
     25, 264, 1, 1, 0, 0, 0, 0, "0d0", 12, 0},
    {"& %M Mushroom~| of Cure Confusion", Food, 0x00000000, 0x00000100,
     500, 50, 265, 1, 1, 0, 0, 0, 0, "0d0", 6, 0},
    {"& %M Mushroom~| of Weakness", Food, 0x00000000, 0x04000200, 500, 0,
     266, 1, 1, 0, 0, 0, 0, "0d0", 7, 0},
    {"& %M Mushroom~| of Unhealth", Food, 0x00000000, 0x04000400, 500, 50,
     267, 1, 1, 0, 0, 0, 0, "10d10", 15, 0},
    {"& %M Mushroom~| of Restore Constitution", Food, 0x00000000,
     0x00010000, 500, 350, 268, 1, 1, 0, 0, 0, 0, "0d0", 20, 0},
    {"& %M Mushroom~| of First-Aid", Food, 0x00000000, 0x00200000, 500, 5,
     269, 1, 1, 0, 0, 0, 0, "0d0", 6, 0},
    {"& %M Mushroom~| of Minor Cures", Food, 0x00000000, 0x00400000, 500,
     20, 270, 1, 1, 0, 0, 0, 0, "0d0", 7, 0},
    {"& %M Mushroom~| of Light Cures", Food, 0x00000000, 0x00800000, 500,
     30, 271, 1, 1, 0, 0, 0, 0, "0d0", 10, 0},
    {"& %M Mushroom~| of Restoring", Food, 0x00000000, 0x001F8040, 500,
     1000, 272, 1, 1, 0, 0, 0, 0, "0d0", 30, 0},
    {"& %M Mushroom~| of Poison", Food, 0x00000000, 0x00000001, 1200, 0,
     273, 1, 1, 0, 0, 0, 0, "0d0", 15, 0},
    {"& %M Mushroom~| of Hallucinations", Food, 0x00000000, 0x00000010,
     1200, 0, 274, 1, 1, 0, 0, 0, 0, "0d0", 18, 0},
    {"& %M Mushroom~| of Cure Poison", Food, 0x00000000, 0x00000020,
     1200, 75, 275, 1, 1, 0, 0, 0, 0, "0d0", 19, 0},
    {"& %M Mushroom~| of Unhealth", Food, 0x00000000, 0x00000400, 1200,
     25, 276, 1, 1, 0, 0, 0, 0, "6d8", 28, 0},
    {"& %M Mushroom~| of Cure Serious Wounds", Food, 0x00000000,
     0x01800000, 1200, 75, 277, 2, 1, 0, 0, 0, 0, "0d0", 16, 0},
    {"& Ration~ of Food^", Food, 0x00000000, Nothing_flag, 5000, 3, 307,
     10, 1, 0, 0, 0, 0, "0d0", 0, 0},
    {"& Ration~ of Food^", Food, 0x00000000, Nothing_flag, 5000, 3, 307,
     10, 1, 0, 0, 0, 0, "0d0", 5, 0},
    {"& Ration~ of Food^", Food, 0x00000000, Nothing_flag, 5000, 3, 307,
     10, 1, 0, 0, 0, 0, "0d0", 10, 0},
    {"& Mushroom~^", Food, 0x00000000, Nothing_flag, 3000, 2, 256, 5, 1,
     0, 0, 0, 0, "0d0", 1, 0},
    {"& Piece~ of Elvish Waybread^", Food, 0x00000000, 0x21800020, 3500,
     10, 313, 3, 1, 0, 0, 0, 0, "0d0", 6, 0},
    {"& Handful~ of Berries| (Poisonous)", Food, 0x00000000, 0x0C0000000,
     1000, 0, 316, 3, 1, 0, 0, 0, 0, "0d0", 8, 0},
    {"& Piece~ of Elvish Waybread^", Food, 0x00000000, 0x21800020, 3500,
     10, 313, 3, 1, 0, 0, 0, 0, "0d0", 12, 0},
    {"& Handful~ of Berries| (Smurfberries)", Food, 0x00000000, 0x10400000,
     1000, 0, 317, 3, 1, 0, 0, 0, 0, "0d0", 20, 0},
    {"& Piece~ of Elvish Waybread^", Food, 0x00000000, 0x21800020, 3500,
     10, 313, 3, 1, 0, 0, 0, 0, "0d0", 30, 0},
    {"& Handful~ of Berries| (Goodberries)", Food, 0x00000000, 0x10C00080,
     1000, 0, 318, 3, 1, 0, 0, 0, 0, "0d0", 40, 0},
    {"& Jolly Green Jelly~| (Ho Ho Ho!)", Food, 0x00000000, 0x224001C0,
     4000, 0, 315, 3, 1, 0, 0, 0, 0, "0d0", 50, 0},
    {"& Eyeball~| of Ned", Food, 0x00000000, 0x00000053, 200, 50, 319, 2,
     2, 0, 0, 0, 0, "6d5", 20, 0},
    {"& Main Gauche (%P0)^ (%P2,%P3)", dagger, 0x10000000, Nothing_flag, 0,
     25, 1, 30, 1, 0, 0, 0, 0, "1d5", 2, 0},
    {"& Misercorde (%P0)^ (%P2,%P3)", dagger, 0x10000000, Nothing_flag, 0,
     10, 2, 15, 1, 0, 0, 0, 0, "1d4", 0, 0},
    {"& Stiletto (%P0) (%P0)^ (%P2,%P3)", dagger, 0x10000000, Nothing_flag,
     0, 10, 3, 12, 1, 0, 0, 0, 0, "1d4", 0, 0},
    {"& Bodkin (%P0)^ (%P2,%P3)", dagger, 0x10000000, Nothing_flag, 0, 10,
     4, 20, 1, 0, 0, 0, 0, "1d4", 1, 0},
    {"& Broken dagger (%P0)^ (%P2,%P3)", dagger, 0x10000000, Nothing_flag,
     0, 0, 5, 15, 1, -2, -2, 0, 0, "1d1", 0, 0},
    {"& Backsword (%P0)^ (%P2,%P3)", sword, 0x10000000, Nothing_flag, 0,
     60, 6, 95, 1, 0, 0, 0, 0, "1d9", 7, 0},
    {"& Bastard Sword (%P0)^ (%P2,%P3)", sword, 0x10000000, Nothing_flag,
     0, 350, 7, 140, 1, 0, 0, 0, 0, "3d4", 14, 0},
    {"& Bilbo (%P0)^ (%P2,%P3)", dagger, 0x10000000, Nothing_flag, 0, 60,
     8, 80, 1, 0, 0, 0, 0, "1d6", 4, 0},
    {"& Baselard (%P0)^ (%P2,%P3)", dagger, 0x10000000, Nothing_flag, 0,
     80, 9, 100, 1, 0, 0, 0, 0, "1d7", 5, 0},
    {"& Broadsword (%P0)^ (%P2,%P3)", sword, 0x10000000, Nothing_flag, 0,
     255, 10, 150, 1, 0, 0, 0, 0, "2d5", 9, 0},
    {"& Claymore (%P0)^ (%P2,%P3)", sword, 0x10000000, Nothing_flag, 0,
     775, 11, 200, 1, 0, 0, 0, 0, "3d6", 30, 0},
    {"& Cutlass (%P0)^ (%P2,%P3)", sword, 0x10000000, Nothing_flag, 0, 85,
     12, 110, 1, 0, 0, 0, 0, "1d7", 7, 0},
    {"& Espadon (%P0)^ (%P2,%P3)", sword, 0x10000000, Nothing_flag, 0, 655,
     13, 180, 1, 0, 0, 0, 0, "3d6", 35, 0},
    {"& Executioner's Sword (%P0)^ (%P2,%P3)", sword, 0x10000000,
     Nothing_flag, 0, 850, 14, 260, 1, 0, 0, 0, 0, "4d5", 40, 0},
    {"& Flamberge (%P0)^ (%P2,%P3)", sword, 0x10000000, Nothing_flag, 0,
     1000, 15, 240, 1, 0, 0, 0, 0, "4d5", 45, 0},
    {"& Foil (%P0)^ (%P2,%P3)", dagger, 0x10000000, Nothing_flag, 0, 35,
     16, 30, 1, 0, 0, 0, 0, "1d5", 2, 0},
    {"& Katana (%P0)^ (%P2,%P3)", sword, 0x10000000, Nothing_flag, 0, 400,
     17, 120, 1, 0, 0, 0, 0, "3d4", 18, 0},
    {"& Longsword (%P0)^ (%P2,%P3)", sword, 0x10000000, Nothing_flag, 0,
     300, 18, 130, 1, 0, 0, 0, 0, "1d10", 12, 0},
    {"& No-Dachi (%P0)^ (%P2,%P3)", sword, 0x10000000, Nothing_flag, 0,
     675, 19, 200, 1, 0, 0, 0, 0, "4d4", 45, 0},
    {"& Rapier (%P0)^ (%P2,%P3)", dagger, 0x10000000, Nothing_flag, 0, 42,
     20, 40, 1, 0, 0, 0, 0, "1d6", 4, 0},
    {"& Sabre (%P0)^ (%P2,%P3)", sword, 0x10000000, Nothing_flag, 0, 50,
     21, 50, 1, 0, 0, 0, 0, "1d7", 5, 0},
    {"& Small Sword (%P0)^ (%P2,%P3)", dagger, 0x10000000, Nothing_flag, 0,
     48, 22, 75, 1, 0, 0, 0, 0, "1d6", 5, 0},
    {"& Zweihander (%P0)^ (%P2,%P3)", sword, 0x10000000, Nothing_flag, 0,
     1000, 23, 280, 1, 0, 0, 0, 0, "4d6", 50, 0},
    {"& Broken sword (%P0)^ (%P2,%P3)", sword, 0x10000000, Nothing_flag, 0,
     0, 24, 75, 1, -2, -2, 0, 0, "1d1", 0, 0},
    {"& Balestarius (%P0)^ (%P2,%P3)", hafted_weapon, 0x1000000,
     Nothing_flag, 0, 500, 1, 180, 1, 0, 0, 0, 0, "2d8", 30, 0},
    {"& Ball and Chain (%P0)^ (%P2,%P3)", maul, 0x00000000, Nothing_flag,
     0, 200, 2, 150, 1, 0, 0, 0, 0, "2d4", 20, 0},
    {"& Battle Axe (%P0)^ (%P2,%P3)", hafted_weapon, 0x10000000,
     Nothing_flag, 0, 334, 3, 170, 1, 0, 0, 0, 0, "3d4", 13, 0},
    {"& Broad Axe (%P0)^ (%P2,%P3)", hafted_weapon, 0x10000000,
     Nothing_flag, 0, 304, 4, 160, 1, 0, 0, 0, 0, "2d6", 17, 0},
    {"& Cat-O-Nine Tails (%P0)^ (%P2,%P3)", dagger, 0x00000000,
     Nothing_flag, 0, 14, 5, 40, 1, 0, 0, 0, 0, "1d4", 3, 0},
    {"& Wooden Club (%P0)^ (%P2,%P3)", maul, 0x00000000, Nothing_flag, 0,
     1, 6, 100, 1, 0, 0, 0, 0, "1d3", 0, 0},
    {"& Flail (%P0)^ (%P2,%P3)", maul, 0x00000000, Nothing_flag, 0, 353,
     7, 150, 1, 0, 0, 0, 0, "2d6", 12, 0},
    {"& Two Handed Great Flail (%P0)^ (%P2,%P3)", maul, 0x00000000,
     Nothing_flag, 0, 590, 8, 280, 1, 0, 0, 0, 0, "3d6", 45, 0},
    {"& Morningstar (%P0)^ (%P2,%P3)", maul, 0x00000000, Nothing_flag, 0,
     396, 9, 150, 1, 0, 0, 0, 0, "2d6", 10, 0},
    {"& Mace (%P0)^ (%P2,%P3)", maul, 0x00000000, Nothing_flag, 0, 130,
     10, 120, 1, 0, 0, 0, 0, "2d4", 6, 0},
    {"& War Hammer (%P0)^ (%P2,%P3)", maul, 0x00000000, Nothing_flag, 0,
     225, 11, 120, 1, 0, 0, 0, 0, "3d3", 5, 0},
    {"& Lead Filled Mace (%P0)^ (%P2,%P3)", maul, 0x00000000,
     Nothing_flag, 0, 502, 12, 180, 1, 0, 0, 0, 0, "3d4", 15, 0},
    {"& Iron Shod Quarterstaff (%P0)^ (%P2,%P3)", maul, 0x00000000,
     Nothing_flag, 0, 25, 13, 100, 1, 0, 0, 0, 0, "1d5", 2, 0},
    {"& Awl-Pike (%P0)^ (%P2,%P3)", pole_arm, 0x10000000, Nothing_flag, 0,
     340, 1, 160, 1, 0, 0, 0, 0, "1d8", 8, 0},
    {"& Beaked Axe (%P0)^ (%P2,%P3)", pole_arm, 0x10000000, Nothing_flag,
     0, 408, 2, 180, 1, 0, 0, 0, 0, "2d6", 15, 0},
    {"& Fauchard (%P0)^ (%P2,%P3)", pole_arm, 0x10000000, Nothing_flag, 0,
     376, 3, 170, 1, 0, 0, 0, 0, "1d10", 17, 0},
    {"& Glaive (%P0)^ (%P2,%P3)", pole_arm, 0x10000000, Nothing_flag, 0,
     363, 4, 190, 1, 0, 0, 0, 0, "2d6", 20, 0},
    {"& Halberd (%P0)^ (%P2,%P3)", pole_arm, 0x10000000, Nothing_flag, 0,
     430, 5, 190, 1, 0, 0, 0, 0, "3d4", 22, 0},
    {"& Lucerne Hammer (%P0)^ (%P2,%P3)", pole_arm, 0x00000000,
     Nothing_flag, 0, 376, 6, 120, 1, 0, 0, 0, 0, "2d5", 11, 0},
    {"& Pike (%P0)^ (%P2,%P3)", pole_arm, 0x10000000, Nothing_flag, 0, 358,
     7, 160, 1, 0, 0, 0, 0, "2d5", 15, 0},
    {"& Spear (%P0)^ (%P2,%P3)", pole_arm, 0x10000000, Nothing_flag, 0, 36,
     8, 50, 1, 0, 0, 0, 0, "1d6", 5, 0},
    {"& Lance (%P0)^ (%P2,%P3)", pole_arm, 0x10000000, Nothing_flag, 0,
     230, 9, 300, 1, 0, 0, 0, 0, "2d8", 10, 0},
    {"& Javelin (%P0)^ (%P2,%P3)", pole_arm, 0x10000000, Nothing_flag, 0,
     18, 10, 30, 1, 0, 0, 0, 0, "1d4", 4, 0},
    {"& Short Bow^ (%P2)", bow_crossbow_or_sling, 0x00000000, Nothing_flag,
     2, 50, 1, 30, 1, 0, 0, 0, 0, "0d0", 3, 0},
    {"& Long Bow^ (%P2)", bow_crossbow_or_sling, 0x00000000, Nothing_flag,
     3, 120, 2, 40, 1, 0, 0, 0, 0, "0d0", 10, 0},
    {"& Composite Bow^ (%P2)", bow_crossbow_or_sling, 0x00000000,
     Nothing_flag, 4, 240, 3, 40, 1, 0, 0, 0, 0, "0d0", 40, 0},
    {"& Light Crossbow^ (%P2)", bow_crossbow_or_sling, 0x00000000,
     Nothing_flag, 5, 140, 10, 110, 1, 0, 0, 0, 0, "0d0", 15, 0},
    {"& Heavy Crossbow^ (%P2)", bow_crossbow_or_sling, 0x00000000,
     Nothing_flag, 6, 300, 11, 200, 1, 0, 0, 0, 0, "1d1", 30, 0},
    {"& Sling^ (%P2)", bow_crossbow_or_sling, 0x00000000, Nothing_flag, 1,
     5, 20, 5, 1, 0, 0, 0, 0, "0d0", 1, 0},
    {"& Arrow~^ (%P2,%P3)", arrow, 0x10000000, Nothing_flag, 0, 1, 1, 2, 1,
     0, 0, 0, 0, "3d4", 2, 0},
    {"& Bolt~^ (%P2,%P3)", bolt, 0x10000000, Nothing_flag, 0, 2, 1, 3, 1,
     0, 0, 0, 0, "3d5", 2, 0},
    {"& Rounded Pebble~^ (%P2,%P3)", sling_ammo, 0x00000000, Nothing_flag,
     0, 1, 1, 4, 1, 0, 0, 0, 0, "3d2", 0, 0},
    {"& Iron Shot~^ (%P2,%P3)", sling_ammo, 0x00000000, Nothing_flag, 0, 2,
     1, 5, 1, 0, 0, 0, 0, "3d3", 3, 0},
    {"& Iron Spike~^", spike, 0x10000000, Nothing_flag, 0, 1, 1, 10, 1, 0,
     0, 0, 0, "1d1", 1, 0},
    {"& Magic Lantern^ (%P5 turns left)", lamp_or_torch, 0x00000000,
     Nothing_flag, 20000, 200, 17, 45, 1, 0, 0, 0, 0, "1d8", 20, 0},
    {"& Magic Torch^ (%P5 turns left)", lamp_or_torch, 0x00000000,
     Nothing_flag, 9000, 30, 15, 1, 1, 0, 0, 0, 0, "2d6", 10, 0},
    {"& Brass Lantern~ with %P5 turns of light", lamp_or_torch, 0x00000000,
     Nothing_flag, 7500, 35, 1, 50, 1, 0, 0, 0, 0, "1d1", 1, 0},
    {"& Wooden Torch~ with %P5 turns of light", lamp_or_torch, 0x00000000,
     Nothing_flag, 4000, 2, 13, 30, 1, 0, 0, 0, 0, "1d1", 1, 0},
    {"& Orcish Pick^ (%P1) (%P2,%P3)", pick_or_shovel, 0x10000000,
     Tunneling_worn_bit, 2, 500, 2, 180, 1, 0, 0, 0, 0, "1d3", 20, 0},
    {"& Dwarven Pick^ (%P1) (%P2,%P3)", pick_or_shovel, 0x10000000,
     Tunneling_worn_bit, 3, 1200, 3, 200, 1, 0, 0, 0, 0, "1d4", 50, 0},
    {"& Gnomish Shovel^ (%P1) (%P2,%P3)", pick_or_shovel, 0x00000000,
     Tunneling_worn_bit, 1, 100, 5, 50, 1, 0, 0, 0, 0, "1d2", 20, 0},
    {"& Dwarven Shovel^ (%P1) (%P2,%P3)", pick_or_shovel, 0x00000000,
     Tunneling_worn_bit, 2, 250, 6, 120, 1, 0, 0, 0, 0, "1d3", 40, 0},
    {"& Orcish Pick^ (%P1) (%P2,%P3)", pick_or_shovel, 0x10000000, 0x20100084, 3,
     1500, 7, 180, 1, 0, 0, 0, 0, "2d3", 40, 0},
    {"& Pair of Soft Leather Shoes^ [%P6,%P4]", boots, 0x00000000,
     Nothing_flag, 0, 4, 1, 5, 1, 0, 0, 1, 0, "0d0", 1, 0},
    {"& Pair of Soft Leather Boots^ [%P6,%P4]", boots, 0x00000000,
     Nothing_flag, 0, 7, 2, 20, 1, 0, 0, 2, 0, "1d1", 4, 0},
    {"& Pair of Hard Leather Boots^ [%P6,%P4]", boots, 0x00000000,
     Nothing_flag, 0, 12, 3, 40, 1, 0, 0, 3, 0, "1d1", 6, 0},
    {"& Pair of Sandals^ [%P6,%P4]", boots, Nothing_flag, Nothing_flag, 0,
     1, 4, 1, 1, 0, 0, 0, 0, "0d0", 1, 0},
    {"& Soft Leather Cap^ [%P6,%P4]", helm, 0x00000000, Nothing_flag, 0, 4,
     1, 10, 1, 0, 0, 1, 0, "0d0", 2, 0},
    {"& Hard Leather Cap^ [%P6,%P4]", helm, 0x00000000, Nothing_flag, 0,
     12, 2, 15, 1, 0, 0, 2, 0, "0d0", 4, 0},
    {"& Metal Cap^ [%P6,%P4]", helm, 0x00000000, Nothing_flag, 0, 30, 3,
     20, 1, 0, 0, 3, 0, "1d1", 7, 0},
    {"& Iron Helm^ [%P6,%P4]", helm, 0x00000000, Nothing_flag, 0, 75, 4,
     75, 1, 0, 0, 5, 0, "1d3", 20, 0},
    {"& Steel Helm^ [%P6,%P4]", helm, 0x00000000, Nothing_flag, 0, 200, 5,
     60, 1, 0, 0, 6, 0, "1d3", 40, 0},
    {"& Silver Crown^ [%P6,%P4] (%P1)", helm, 0x00000000, Nothing_flag, 0,
     250, 6, 20, 1, 0, 0, 0, 0, "1d1", 44, 0},
    {"& Golden Crown^ [%P6,%P4] (%P1)", helm, 0x00000000, Nothing_flag, 0,
     500, 7, 30, 1, 0, 0, 0, 0, "1d1", 47, 0},
    {"& Jewel Encrusted Crown^ [%P6,%P4] (%P1)", helm, 0x00000000,
     Nothing_flag, 0, 1000, 8, 40, 1, 0, 0, 0, 0, "1d1", 50, 0},
    {"& Iron Helm^ of Gems [%P6,%P4]", gem_helm, 0x00000000, 0x00000000, 2,
     300, 9, 75, 1, 0, 0, 2, 0, "1d3", 50, 0},
    {"& Steel Helm^ of Gems [%P6,%P4]", gem_helm, 0x00000000, 0x00000000,
     5, 700, 10, 60, 1, 0, 0, 3, 0, "1d3", 75, 0},
    {"& Cloth Hat^ [%P6,%P4]", helm, Nothing_flag, Nothing_flag, 0, 5, 11,
     5, 1, 0, 0, 0, 0, "0d0", 1, 0},
    {"& Cloth Hat^ [%P6,%P4]", helm, Nothing_flag, Nothing_flag, 0, 5, 11,
     5, 1, 0, 0, 0, 0, "0d0", 1, 0},
    {"& Finely cut %R| of Teleportation^", valuable_gems_wear, 0x00000000,
     0x00000400, 0, 300, 1, 5, 1, 0, 0, 0, 0, "0d0", 5, 0},
    {"& Finely cut %R| of Resist Cold^", valuable_gems_wear, 0x00000000,
     0x00200000, 0, 250, 2, 5, 1, 0, 0, 0, 0, "0d0", 14, 0},
    {"& Finely cut %R| of Resist Acid^", valuable_gems_wear, 0x00000000,
     0x00100000, 0, 250, 3, 5, 1, 0, 0, 0, 0, "0d0", 14, 0},
    {"& Finely cut %R| of See Invisible^", valuable_gems_wear, 0x00000000,
     0x01000000, 0, 350, 4, 5, 1, 0, 0, 0, 0, "0d0", 40, 0},
    {"& Finely cut %R| of Stealth^", valuable_gems_wear, 0x00000000,
     0x00000100, 0, 300, 5, 5, 1, 0, 0, 0, 0, "0d0", 35, 0},
    {"& Finely cut %R| of Slow Digestion^", valuable_gems_wear, 0x00000000,
     0x00000800, 0, 200, 6, 5, 1, 0, 0, 0, 0, "0d0", 14, 0},
    {"& Finely cut %R| of Lordly Protection (FIRE)^", valuable_gems_wear, 
     0x00000000, 0x00000800, 0, 1200, 7, 5, 1, 0, 0, 0, 5, "0d0", 40, 0},
    {"& Robe^ [%P6,%P4]", soft_armor, 0x00000000, Nothing_flag, 0, 4, 1,
     20, 1, 0, 0, 2, 0, "0d0", 1, 0},
    {"Soft Leather Armor^ [%P6,%P4]", soft_armor, 0x00000000, Nothing_flag,
     0, 18, 2, 80, 1, 0, 0, 4, 0, "0d0", 2, 0},
    {"Soft Studded Leather^ [%P6,%P4]", soft_armor, 0x00000000,
     Nothing_flag, 0, 35, 3, 90, 1, 0, 0, 5, 0, "1d1", 3, 0},
    {"Hard Leather Armor^ [%P6,%P4]", soft_armor, 0x00000000, Nothing_flag,
     0, 55, 4, 100, 1, -1, 0, 6, 0, "1d1", 5, 0},
    {"Hard Studded Leather^ [%P6,%P4]", soft_armor, 0x00000000,
     Nothing_flag, 0, 100, 5, 110, 1, -1, 0, 7, 0, "1d2", 7, 0},
    {"Woven Cord Armor^ [%P6,%P4]", soft_armor, 0x00000000, Nothing_flag,
     0, 45, 6, 150, 1, -1, 0, 6, 0, "0d0", 7, 0},
    {"Soft Leather Ring Mail^ [%P6,%P4]", soft_armor, 0x00000000,
     Nothing_flag, 0, 160, 7, 130, 1, -1, 0, 6, 0, "1d2", 10, 0},
    {"Hard Leather Ring Mail^ [%P6,%P4]", soft_armor, 0x00000000,
     Nothing_flag, 0, 230, 8, 150, 1, -2, 0, 8, 0, "1d3", 12, 0},
    {"Leather Scale Mail^ [%P6,%P4]", soft_armor, 0x00000000, Nothing_flag,
     0, 330, 9, 140, 1, -1, 0, 11, 0, "1d1", 16, 0},
    {"Leather Brigantine Armor^ [%P6,%P4]", soft_armor, 0x00000000,
     Nothing_flag, 0, 380, 10, 190, 1, -1, 0, 12, 0, "1d2", 20, 0},
    {"Metal Scale Mail^ [%P6,%P4]", hard_armor, 0x00000000, Nothing_flag,
     0, 430, 1, 250, 1, -2, 0, 13, 0, "1d4", 24, 0},
    {"Chain Mail^ [%P6,%P4]", hard_armor, 0x00000000, Nothing_flag, 0, 530,
     2, 220, 1, -2, 0, 14, 0, "1d4", 26, 0},
    {"Rusty Chain Mail^ [%P6,%P4]", hard_armor, 0x00000000, Nothing_flag,
     0, 0, 3, 200, 1, -5, 0, 14, -8, "1d4", 26, 0},
    {"Double Chain Mail^ [%P6,%P4]", hard_armor, 0x00000000, Nothing_flag,
     0, 630, 4, 260, 1, -2, 0, 15, 0, "1d4", 28, 0},
    {"Augmented Chain Mail^ [%P6,%P4]", hard_armor, 0x00000000,
     Nothing_flag, 0, 675, 5, 270, 1, -2, 0, 16, 0, "1d4", 30, 0},
    {"Bronze Plate Mail^ [%P6,%P4]", hard_armor, 0x00000000, Nothing_flag,
     0, 700, 13, 380, 1, -4, 0, 21, 0, "1d6", 32, 0},
    {"Bar Chain Mail^ [%P6,%P4]", hard_armor, 0x00000000, Nothing_flag, 0,
     720, 6, 280, 1, -2, 0, 18, 0, "1d4", 34, 0},
    {"Metal Brigandine Armor^ [%P6,%P4]", hard_armor, 0x00000000,
     Nothing_flag, 0, 775, 7, 290, 1, -3, 0, 19, 0, "1d4", 36, 0},
    {"Laminated Armor^ [%P6,%P4]", hard_armor, 0x00000000, Nothing_flag, 0,
     825, 8, 300, 1, -3, 0, 20, 0, "1d4", 38, 0},
    {"Partial Plate Armor^ [%P6,%P4]", hard_armor, 0x00000000,
     Nothing_flag, 0, 900, 9, 260, 1, -3, 0, 22, 0, "1d6", 42, 0},
    {"Metal Lamellar Armor^ [%P6,%P4]", hard_armor, 0x00000000,
     Nothing_flag, 0, 950, 10, 340, 1, -3, 0, 23, 0, "1d6", 44, 0},
    {"Full Plate Armor^ [%P6,%P4]", hard_armor, 0x00000000, Nothing_flag,
     0, 1050, 11, 380, 1, -3, 0, 25, 0, "2d4", 48, 0},
    {"Ribbed Plate Armor^ [%P6,%P4]", hard_armor, 0x00000000, Nothing_flag,
     0, 1200, 12, 380, 1, -3, 0, 28, 0, "2d4", 50, 0},
    {"& Cloak^ [%P6,%P4]", cloak, 0x00000000, Nothing_flag, 0, 3, 1, 10, 1,
     0, 0, 1, 0, "0d0", 1, 0},
    {"& Cloak^ [%P6,%P4]", cloak, 0x00000000, Nothing_flag, 0, 3, 1, 10, 1,
     0, 0, 1, 0, "0d0", 1, 0},
    {"& Set of Leather Gloves^ [%P6,%P4]", gloves_and_gauntlets, 
     0x00000000, Nothing_flag, 0, 3, 1, 5, 1, 0, 0, 1, 0, "0d0", 1, 0},
    {"& Set of Gauntlets^ [%P6,%P4]", gloves_and_gauntlets, 0x00000000,
     Nothing_flag, 0, 35, 2, 25, 1, 0, 0, 2, 0, "1d1", 12, 0},
    {"& Set of Cloth Gloves^ [%P6,%P4]", gloves_and_gauntlets, 
     Nothing_flag, Nothing_flag, 0, 1, 5, 1, 1, 0, 0, 0, 0, "0d0", 1, 0},
    {"& Silver Belt Buckle^", belt, Nothing_flag, Nothing_flag, 0, 30, 10,
     50, 1, 0, 0, 0, 0, "0d0", 5, 0},
    {"& Gold Belt Buckle^", belt, Nothing_flag, Nothing_flag, 0, 120, 11,
     50, 1, 0, 0, 0, 0, "0d0", 10, 0},
    {"& Leather Belt^", belt, Nothing_flag, Nothing_flag, 0, 10, 13, 75, 1,
     0, 0, 0, 0, "0d0", 1, 0},
    {"& Girdle^ [%P6,%P4]", belt, Nothing_flag, Nothing_flag, 0, 1, 1, 100,
     1, 0, 0, 1, 0, "0d0", 10, 0},
    {"& Set of Bracers^ of Protection", bracers, 0x00000000, 0x00000000, 0,
     1200, 1, 125, 1, 0, 0, 6, 0, "0d0", 35, 0},
    {"& Set of Bracers^ of Defense", bracers, 0x00000000, 0x00000000, 0,
     2400, 2, 125, 1, 0, 0, 7, 0, "0d0", 40, 0},
    {"& Set of Bracers^ of Shielding", bracers, 0x00000000, 0x00000000, 0,
     3600, 3, 125, 1, 0, 0, 8, 0, "0d0", 45, 0},
    {"& Set of Mithril Bracers^", bracers, 0x00000000, 0x00000000, 0, 4800,
     4, 125, 1, 0, 0, 9, 0, "0d0", 50, 0},
    {"& Set of Adamantite Bracers^", bracers, 0x00000000, 0x00000000, 0,
     6000, 5, 125, 1, 0, 0, 10, 0, "0d0", 55, 0},
    {"& Set of Bracers^ of Weapon Attraction", bracers, 0x00000000,
     0x80000000, 0, -1200, 6, 125, 1, -6, 0, -6, 0, "0d0", 30, 0},
    {"& Small silver Bracelet^ of Warding (R)", bracers, 0x00000010,
     0x02380000, 1, 10000, 31, 5, 1, 0, 0, 5, 0, "0d0", 50, 0},
    {"& Small Leather Shield^ [%P6,%P4]", shield, 0x00000000, Nothing_flag,
     0, 30, 1, 50, 1, 0, 0, 2, 0, "1d1", 3, 0},
    {"& Medium Leather Shield^ [%P6,%P4]", shield, 0x00000000,
     Nothing_flag, 0, 60, 2, 75, 1, 0, 0, 3, 0, "1d2", 8, 0},
    {"& Large Leather Shield^ [%P6,%P4]", shield, 0x00000000, Nothing_flag,
     0, 120, 3, 100, 1, 0, 0, 4, 0, "1d2", 15, 0},
    {"& Small Metal Shield^ [%P6,%P4]", shield, 0x00000000, Nothing_flag,
     0, 50, 4, 65, 1, 0, 0, 3, 0, "1d2", 10, 0},
    {"& Medium Metal Shield^ [%P6,%P4]", shield, 0x00000000, Nothing_flag,
     0, 125, 5, 90, 1, 0, 0, 4, 0, "1d3", 20, 0},
    {"& Large Metal Shield^ [%P6,%P4]", shield, 0x00000000, Nothing_flag,
     0, 200, 6, 120, 1, 0, 0, 5, 0, "1d3", 30, 0},
    {"& %R Ring| of Gain Strength^ (%P1)", ring, 0, Strength_worn_bit, 0,
     400, 1, 2, 1, 0, 0, 0, 0, "0d0", 30, 0},
    {"& %R Ring| of Gain Dexterity^ (%P1)", ring, 0, Dexterity_worn_bit, 0,
     400, 2, 2, 1, 0, 0, 0, 0, "0d0", 30, 0},
    {"& %R Ring| of Gain Constitution^ (%P1)", ring, 0,
     Constitution_worn_bit, 0, 400, 3, 2, 1, 0, 0, 0, 0, "0d0", 30, 0},
    {"& %R Ring| of Gain Intelligence^ (%P1)", ring, 0,
     Intelligence_worn_bit, 0, 350, 4, 2, 1, 0, 0, 0, 0, "0d0", 30, 0},
    {"& %R Ring| of Speed^ (%P1)", ring, 0, Speed_worn_bit, 0, 8000, 7, 2,
     1, 0, 0, 0, 0, "0d0", 50, 0},
    {"& %R Ring| of Speed^ (%P1)", ring, 0, Speed_worn_bit, 0, 1, 35, 2, 1,
     0, 0, 0, 0, "0d0", 5, 0},
    {"& %R Ring| of Searching^ (%P1)", ring, 0, Searching_worn_bit, 0, 250,
     8, 2, 1, 0, 0, 0, 0, "0d0", 7, 0},
    {"& %R Ring| of Teleportation^", ring, 0,
     Cursed_worn_bit + Teleportation_worn_bit, 0, 0, 9, 2, 1, 0, 0, 0, 0, "0d0",
     7, 0},
    {"& %R Ring| of Slow Digestion^", ring, 0, Slow_Digestion_worn_bit, 0,
     250, 10, 2, 1, 0, 0, 0, 0, "0d0", 7, 0},
    {"& %R Ring| of Resist Fire^", ring, 0, Resist_Fire_worn_bit, 0, 250,
     11, 2, 1, 0, 0, 0, 0, "0d0", 14, 0},
    {"& %R Ring| of Resist Cold^", ring, 0, Resist_Cold_worn_bit, 0, 250,
     12, 2, 1, 0, 0, 0, 0, "0d0", 14, 0},
    {"& %R Ring| of Feather Falling^", ring, 0, Feather_Fall_worn_bit, 0,
     200, 13, 2, 1, 0, 0, 0, 0, "0d0", 7, 0},
    {"& %R Ring| of Adornment^", ring, 0x00000000, Nothing_flag, 0, 20, 14,
     2, 1, 0, 0, 0, 0, "0d0", 7, 0},
    {"& %R Ring| of Adornment^", ring, 0x00000000, Nothing_flag, 0, 30, 15,
     2, 1, 0, 0, 0, 0, "0d0", 7, 0},
    {"& %R Ring| of Weakness^", ring, 0,
     Cursed_worn_bit + Strength_worn_bit, -5, 0, 16, 2, 1, 0, 0, 0, 0, "0d0",
     7, 0},
    {"& %R Ring| of Lordly Protection (FIRE)^", ring, 0,
     Resist_Fire_worn_bit, 0, 1200, 17, 2, 1, 0, 0, 0, 5, "0d0", 50, 0},
    {"& %R Ring| of Lordly Protection (ACID)^", ring, 0,
     Resist_Acid_worn_bit, 0, 1200, 18, 2, 1, 0, 0, 0, 5, "0d0", 50, 0},
    {"& %R Ring| of Lordly Protection (COLD)^", ring, 0,
     Resist_Cold_worn_bit, 0, 1200, 19, 2, 1, 0, 0, 0, 5, "0d0", 50, 0},
    {"& %R Ring| of WOE^", ring, 0,
     Cursed_worn_bit + Aggravation_worn_bit + Teleportation_worn_bit, -5, 0, 20,
     2, 1, 0, 0, 0, -3, "0d0", 50, 0},
    {"& %R Ring| of Stupidity^", ring, 0,
     Cursed_worn_bit + Intelligence_worn_bit, -5, 0, 21, 2, 1, 0, 0, 0, 0,
     "0d0", 20, 0},
    {"& %R Ring| of Increase Damage^ (%P3)", ring, 0x00000000,
     Nothing_flag, 0, 100, 22, 2, 1, 0, 0, 0, 0, "0d0", 20, 0},
    {"& %R Ring| of Increase To-Hit^ (%P2)", ring, 0x00000000,
     Nothing_flag, 0, 100, 23, 2, 1, 0, 0, 0, 0, "0d0", 20, 0},
    {"& %R Ring| of Protection^ [%P4]", ring, 0x00000000, Nothing_flag, 0,
     100, 24, 2, 1, 0, 0, 0, 0, "0d0", 7, 0},
    {"& %R Ring| of Aggravate Monster^", ring, 0,
     Cursed_worn_bit + Aggravation_worn_bit, 0, 0, 25, 2, 1, 0, 0, 0, 0, "0d0",
     7, 0},
    {"& %R Ring| of See Invisible^", ring, 0, See_Invisible_worn_bit, 0,
     340, 26, 2, 1, 0, 0, 0, 0, "0d0", 40, 0},
    {"& %R Ring| of Sustain Strength^", ring, 0, Sustain_Stat_worn_bit, 1,
     750, 27, 2, 1, 0, 0, 0, 0, "0d0", 44, 0},
    {"& %R Ring| of Sustain Intelligence^", ring, 0, Sustain_Stat_worn_bit,
     2, 600, 28, 2, 1, 0, 0, 0, 0, "0d0", 44, 0},
    {"& %R Ring| of Sustain Wisdom^", ring, 0, Sustain_Stat_worn_bit, 3,
     600, 29, 2, 1, 0, 0, 0, 0, "0d0", 44, 0},
    {"& %R Ring| of Sustain Constitution^", ring, 0, Sustain_Stat_worn_bit,
     4, 750, 30, 2, 1, 0, 0, 0, 0, "0d0", 44, 0},
    {"& %R Ring| of Sustain Dexterity^", ring, 0, Sustain_Stat_worn_bit, 5,
     750, 31, 2, 1, 0, 0, 0, 0, "0d0", 44, 0},
    {"& %R Ring| of Sustain Charisma^", ring, 0, Sustain_Stat_worn_bit, 6,
     500, 32, 2, 1, 0, 0, 0, 0, "0d0", 7, 0},
    {"& %R Ring| of Slaying^", ring, 0x00000000, Nothing_flag, 6, 1000, 33,
     2, 1, 0, 0, 0, 0, "0d0", 50, 0},
    {"& %R Ring| of Gnomekind^", ring, Nothing_flag, 0x00400088, 2, 2000,
     34, 2, 1, 0, 0, 0, 0, "0d0", 40, 0},
    {"& %A Amulet| of Adornment^", amulet, 0x00000000, Nothing_flag, 0, 20,
     11, 3, 1, 0, 0, 0, 0, "0d0", 16, 0},
    {"& %A Amulet| of Adornment^", amulet, 0x00000000, Nothing_flag, 0, 30,
     12, 3, 1, 0, 0, 0, 0, "0d0", 16, 0},
    {"& %A Amulet| of Wisdom^ (%P1)", amulet, 0, Wisdom_worn_bit, 0, 300,
     5, 3, 1, 0, 0, 0, 0, "0d0", 20, 0},
    {"& %A Amulet| of Charisma^ (%P1)", amulet, 0, Charisma_worn_bit, 0,
     250, 6, 3, 1, 0, 0, 0, 0, "0d0", 20, 0},
    {"& %A Amulet| of Searching^ (%P1)", amulet, 0, Searching_worn_bit, 0,
     250, 7, 3, 1, 0, 0, 0, 0, "0d0", 14, 0},
    {"& %A Amulet| of Teleportation^", amulet, 0,
     Cursed_worn_bit + Teleportation_worn_bit, 0, 0, 8, 3, 1, 0, 0, 0, 0, "0d0",
     14, 0},
    {"& %A Amulet| of Slow Digestion^", amulet, 0, Slow_Digestion_worn_bit,
     0, 200, 9, 3, 1, 0, 0, 0, 0, "0d0", 14, 0},
    {"& %A Amulet| of Resist Acid^", amulet, 0, Resist_Acid_worn_bit, 0,
     300, 10, 3, 1, 0, 0, 0, 0, "0d0", 24, 0},
    {"& %A Amulet| of the Magi^", amulet, 0,
     See_Invisible_worn_bit + Free_Action_worn_bit + Searching_worn_bit, 0,
     5000, 13, 3, 1, 0, 0, 0, 3, "0d0", 50, 0},
    {"& %A Amulet| of DOOM^", amulet, 0,
     Cursed_worn_bit + Strength_worn_bit + Dexterity_worn_bit +
	 Constitution_worn_bit + Intelligence_worn_bit + Wisdom_worn_bit +
	 Charisma_worn_bit + Searching_worn_bit,
     -5, 0, 14, 3, 1, 0, 0, 0, 0, "0d0", 50, 0},
    {"& Scroll~ %T| of Enchant Weapon To-Hit", scroll1, 0x00000000,
     0x00000001, 0, 125, 257, 5, 1, 0, 0, 0, 0, "0d0", 12, 0},
    {"& Scroll~ %T| of Enchant Weapon To-Dam", scroll1, 0x00000000,
     0x00000002, 0, 125, 258, 5, 1, 0, 0, 0, 0, "0d0", 12, 0},
    {"& Scroll~ %T| of Enchant Armor", scroll1, 0x00000000, 0x00000004, 0,
     125, 259, 5, 1, 0, 0, 0, 0, "0d0", 12, 0},
    {"& Scroll~ %T| of Identify", scroll1, 0x00000000, 0x00000008, 0, 50,
     260, 5, 1, 0, 0, 0, 0, "0d0", 1, 0},
    {"& Scroll~ %T| of Identify", scroll1, 0x00000000, 0x00000008, 0, 50,
     260, 5, 1, 0, 0, 0, 0, "0d0", 5, 0},
    {"& Scroll~ %T| of Identify", scroll1, 0x00000000, 0x00000008, 0, 50,
     260, 5, 1, 0, 0, 0, 0, "0d0", 10, 0},
    {"& Scroll~ %T| of Remove Curse", scroll1, 0x00000000, 0x00000010, 0,
     100, 261, 5, 1, 0, 0, 0, 0, "0d0", 7, 0},
    {"& Scroll~ %T| of Light", scroll1, 0x00000000, 0x00000020, 0, 15, 262,
     5, 1, 0, 0, 0, 0, "0d0", 0, 0},
    {"& Scroll~ %T| of Light", scroll1, 0x00000000, 0x00000020, 0, 15, 262,
     5, 1, 0, 0, 0, 0, "0d0", 3, 0},
    {"& Scroll~ %T| of Light", scroll1, 0x00000000, 0x00000020, 0, 15, 262,
     5, 1, 0, 0, 0, 0, "0d0", 7, 0},
    {"& Scroll~ %T| of Summon Monster", scroll1, 0x00000000, 0x00000040, 0,
     0, 263, 5, 1, 0, 0, 0, 0, "0d0", 1, 0},
    {"& Scroll~ %T| of Phase Door", scroll1, 0x00000000, 0x00000080, 0, 15,
     264, 5, 1, 0, 0, 0, 0, "0d0", 1, 0},
    {"& Scroll~ %T| of Teleport", scroll1, 0x00000000, 0x00000100, 0, 40,
     265, 5, 1, 0, 0, 0, 0, "0d0", 10, 0},
    {"& Scroll~ %T| of Teleport Level", scroll1, 0x00000000, 0x00000200, 0,
     50, 266, 5, 1, 0, 0, 0, 0, "0d0", 20, 0},
    {"& Scroll~ %T| of Monster Confusion", scroll1, 0x00000000, 0x00000400,
     0, 30, 267, 5, 1, 0, 0, 0, 0, "0d0", 5, 0},
    {"& Scroll~ %T| of Magic Mapping", scroll1, 0x00000000, 0x00000800, 0,
     40, 268, 5, 1, 0, 0, 0, 0, "0d0", 5, 0},
    {"& Scroll~ %T| of Sleep Monster", scroll1, 0x00000000, 0x00001000, 0,
     35, 269, 5, 1, 0, 0, 0, 0, "0d0", 5, 0},
    {"& Scroll~ %T| of Rune of Protection", scroll1, 0x00000000,
     0x00002000, 0, 500, 270, 5, 1, 0, 0, 0, 0, "0d0", 50, 0},
    {"& Scroll~ %T| of Treasure Detection", scroll1, 0x00000000,
     0x00004000, 0, 15, 271, 5, 1, 0, 0, 0, 0, "0d0", 0, 0},
    {"& Scroll~ %T| of Object Detection", scroll1, 0x00000000, 0x00008000,
     0, 15, 272, 5, 1, 0, 0, 0, 0, "0d0", 0, 0},
    {"& Scroll~ %T| of Trap Detection", scroll1, 0x00000000, 0x00010000, 0,
     35, 273, 5, 1, 0, 0, 0, 0, "0d0", 5, 0},
    {"& Scroll~ %T| of Trap Detection", scroll1, 0x00000000, 0x00010000, 0,
     35, 273, 5, 1, 0, 0, 0, 0, "0d0", 8, 0},
    {"& Scroll~ %T| of Trap Detection", scroll1, 0x00000000, 0x00010000, 0,
     35, 273, 5, 1, 0, 0, 0, 0, "0d0", 12, 0},
    {"& Scroll~ %T| of Door/Stair Location", scroll1, 0x00000000,
     0x00020000, 0, 35, 274, 5, 1, 0, 0, 0, 0, "0d0", 5, 0},
    {"& Scroll~ %T| of Door/Stair Location", scroll1, 0x00000000,
     0x00020000, 0, 35, 274, 5, 1, 0, 0, 0, 0, "0d0", 10, 0},
    {"& Scroll~ %T| of Door/Stair Location", scroll1, 0x00000000,
     0x00020000, 0, 35, 274, 5, 1, 0, 0, 0, 0, "0d0", 15, 0},
    {"& Scroll~ %T| of Mass Genocide", scroll1, 0x00000000, 0x00040000, 0,
     1000, 275, 5, 1, 0, 0, 0, 0, "0d0", 50, 0},
    {"& Scroll~ %T| of Detect Invisible", scroll1, 0x00000000, 0x00080000,
     0, 15, 276, 5, 1, 0, 0, 0, 0, "0d0", 1, 0},
    {"& Scroll~ %T| of Aggravate Monster", scroll1, 0x00000000, 0x00100000,
     0, 0, 277, 5, 1, 0, 0, 0, 0, "0d0", 5, 0},
    {"& Scroll~ %T| of Trap Creation", scroll1, 0x00000000, 0x00200000, 0,
     0, 278, 5, 1, 0, 0, 0, 0, "0d0", 12, 0},
    {"& Scroll~ %T| of Trap/Door Destruction", scroll1, 0x00000000,
     0x00400000, 0, 50, 279, 5, 1, 0, 0, 0, 0, "0d0", 12, 0},
    {"& Scroll~ %T| of Door Creation", scroll1, 0x00000000, 0x00800000, 0,
     100, 280, 5, 1, 0, 0, 0, 0, "0d0", 12, 0},
    {"& Scroll~ %T| of Recharging", scroll1, 0x00000000, 0x01000000, 0,
     200, 281, 5, 1, 0, 0, 0, 0, "0d0", 40, 0},
    {"& Scroll~ %T| of Genocide", scroll1, 0x00000000, 0x02000000, 0, 750,
     282, 5, 1, 0, 0, 0, 0, "0d0", 35, 0},
    {"& Scroll~ %T| of Darkness", scroll1, 0x00000000, 0x04000000, 0, 0,
     283, 5, 1, 0, 0, 0, 0, "0d0", 1, 0},
    {"& Scroll~ %T| of Protection from Evil", scroll1, 0x00000000,
     0x08000000, 0, 50, 284, 5, 1, 0, 0, 0, 0, "0d0", 30, 0},
    {"& Scroll~ %T| of Create Food", scroll1, 0x00000000, 0x10000000, 0,
     15, 285, 5, 1, 0, 0, 0, 0, "0d0", 10, 0},
    {"& Scroll~ %T| of Dispel Undead", scroll1, 0x00000000, 0x20000000, 0,
     200, 286, 5, 1, 0, 0, 0, 0, "0d0", 40, 0},
    {"& Scroll~ %T| of *Enchant Weapon*", scroll1, 0x00001000, 0x00000000,
     0, 500, 257, 5, 1, 0, 0, 0, 0, "0d0", 50, 0},
    {"& Scroll~ %T| of Curse Weapon", scroll1, 0x00000001, 0x00000000, 0,
     0, 258, 5, 1, 0, 0, 0, 0, "0d0", 50, 0},
    {"& Scroll~ %T| of *Enchant Armor*", scroll1, 0x00000002, 0x00000000,
     0, 500, 259, 5, 1, 0, 0, 0, 0, "0d0", 50, 0},
    {"& Scroll~ %T| of Curse Armor", scroll1, 0x00000004, 0x00000000, 0, 0,
     260, 5, 1, 0, 0, 0, 0, "0d0", 50, 0},
    {"& Scroll~ %T| of Summon Undead", scroll1, 0x00000008, 0x00000000, 0,
     0, 261, 5, 1, 0, 0, 0, 0, "0d0", 15, 0},
    {"& Scroll~ %T| of Blessing", scroll1, 0x00000010, 0x00000000, 0, 15,
     262, 5, 1, 0, 0, 0, 0, "0d0", 1, 0},
    {"& Scroll~ %T| of Holy Chant", scroll1, 0x00000020, 0x00000000, 0, 40,
     263, 5, 1, 0, 0, 0, 0, "0d0", 12, 0},
    {"& Scroll~ %T| of Holy Prayer", scroll1, 0x00000040, 0x00000000, 0,
     80, 264, 5, 1, 0, 0, 0, 0, "0d0", 24, 0},
    {"& Scroll~ %T| of Word-of-Recall", scroll1, 0x00000080, 0x00000000, 0,
     150, 265, 5, 1, 0, 0, 0, 0, "0d0", 5, 0},
    {"& Scroll~ %T| of *Destruction*", scroll1, 0x00000100, 0x00000000, 0,
     250, 266, 5, 1, 0, 0, 0, 0, "0d0", 40, 0},
    {"& Scroll~ %T| of Wishing", scroll1, 0x00000200, 0x00000000, 0, 15000,
     267, 5, 1, 0, 0, 0, 0, "0d0", 50, 0},
    {"& Scroll~ %T| of Feign Death", scroll1, 0x00000400, 0x00000000, 0, 0,
     268, 5, 1, 0, 0, 0, 0, "0d0", 10, 0},
    {"& Scroll~ %T| of Make Munchies", scroll1, 0x00000800, 0x00000000, 0,
     150, 269, 5, 1, 0, 0, 0, 0, "0d0", 25, 0},
    {"& %C Potion~| of Gain Strength", potion1, 0x00000000, 0x00000001, 0,
     6500, 257, 4, 1, 0, 0, 0, 0, "1d1", 25, 0},
    {"& %C Potion~| of Poison", potion1, 0x00000000, 0x00000002, 0, 0, 258,
     4, 1, 0, 0, 0, 0, "1d1", 3, 0},
    {"& %C Potion~| of Restore Strength", potion1, 0x00000000, 0x00000004,
     0, 300, 259, 4, 1, 0, 0, 0, 0, "1d1", 40, 0},
    {"& %C Potion~| of Gain Intelligence", potion1, 0x00000000, 0x00000008,
     0, 6500, 260, 4, 1, 0, 0, 0, 0, "1d1", 25, 0},
    {"& %C Potion~| of Lose Intelligence", potion1, 0x00000000, 0x00000010,
     0, 0, 261, 4, 1, 0, 0, 0, 0, "1d1", 25, 0},
    {"& %C Potion~| of Restore Intelligence", potion1, 0x00000000,
     0x00000020, 0, 300, 262, 4, 1, 0, 0, 0, 0, "1d1", 40, 0},
    {"& %C Potion~| of Gain Wisdom", potion1, 0x00000000, 0x00000040, 0,
     6500, 263, 4, 1, 0, 0, 0, 0, "1d1", 25, 0},
    {"& %C Potion~| of Lose Wisdom", potion1, 0x00000000, 0x00000080, 0, 0,
     264, 4, 1, 0, 0, 0, 0, "1d1", 25, 0},
    {"& %C Potion~| of Restore Wisdom", potion1, 0x00000000, 0x00000100, 0,
     300, 265, 4, 1, 0, 0, 0, 0, "1d1", 40, 0},
    {"& %C Potion~| of Charisma", potion1, 0x00000000, 0x00000200, 0, 6500,
     266, 4, 1, 0, 0, 0, 0, "1d1", 25, 0},
    {"& %C Potion~| of Ugliness", potion1, 0x00000000, 0x00000400, 0, 0,
     267, 4, 1, 0, 0, 0, 0, "1d1", 25, 0},
    {"& %C Potion~| of Restore Charisma", potion1, 0x00000000, 0x00000800,
     0, 300, 268, 4, 1, 0, 0, 0, 0, "1d1", 40, 0},
    {"& %C Potion~| of Cure Light Wounds", potion1, 0x00000000, 0x10001000,
     50, 15, 269, 4, 1, 0, 0, 0, 0, "1d1", 0, 0},
    {"& %C Potion~| of Cure Light Wounds", potion1, 0x00000000, 0x10001000,
     50, 15, 269, 4, 1, 0, 0, 0, 0, "1d1", 1, 0},
    {"& %C Potion~| of Cure Light Wounds", potion1, 0x00000000, 0x10001000,
     50, 15, 269, 4, 1, 0, 0, 0, 0, "1d1", 2, 0},
    {"& %C Potion~| of Cure Serious Wounds", potion1, 0x00000000,
     0x30002000, 100, 40, 270, 4, 1, 0, 0, 0, 0, "1d1", 3, 0},
    {"& %C Potion~| of Cure Critical Wounds", potion1, 0x00000000,
     0x70004000, 100, 100, 271, 4, 1, 0, 0, 0, 0, "1d1", 5, 0},
    {"& %C Potion~| of Healing", potion1, 0x00000000, 0x70008000, 200,
     2500, 272, 4, 1, 0, 0, 0, 0, "1d1", 12, 0},
    {"& %C Potion~| of Gain Constitution", potion1, 0x00000000, 0x00010000,
     0, 6500, 273, 4, 1, 0, 0, 0, 0, "1d1", 25, 0},
    {"& %C Potion~| of Gain Experience", potion1, 0x00000000, 0x00020000,
     0, 15000, 274, 4, 1, 0, 0, 0, 0, "1d1", 50, 0},
    {"& %C Potion~| of Sleep", potion1, 0x00000000, 0x10040000, 100, 0,
     275, 4, 1, 0, 0, 0, 0, "1d1", 0, 0},
    {"& %C Potion~| of Blindness", potion1, 0x00000000, 0x00080000, 0, 0,
     276, 4, 1, 0, 0, 0, 0, "1d1", 0, 0},
    {"& %C Potion~| of Confusion", potion1, 0x00000000, 0x00100000, 50, 0,
     277, 4, 1, 0, 0, 0, 0, "1d1", 0, 0},
    {"& %C Potion~| of Poison", potion1, 0x00000000, 0x00200000, 0, 0, 278,
     4, 1, 0, 0, 0, 0, "1d1", 3, 0},
    {"& %C Potion~| of Haste Self", potion1, 0x00000000, 0x00400000, 0,
     155, 279, 4, 1, 0, 0, 0, 0, "1d1", 1, 0},
    {"& %C Potion~| of Slowness", potion1, 0x00000000, 0x00800000, 50, 0,
     280, 4, 1, 0, 0, 0, 0, "1d1", 1, 0},
    {"& Icky Green Potion~| of Slime Mold Juice", potion1, 0x00000000,
     0x30000000, 400, 2, 281, 4, 1, 0, 0, 0, 0, "1d1", 0, 0},
    {"& Light Brown Potion~| of Apple Juice", potion1, 0x00000000,
     Nothing_flag, 250, 1, 282, 4, 1, 0, 0, 0, 0, "1d1", 0, 0},
    {"& Clear Potion~| of Water", potion1, 0x00000000, Nothing_flag, 200,
     0, 283, 4, 1, 0, 0, 0, 0, "1d1", 0, 0},
    {"& %C Potion~| of Gain Dexterity", potion1, 0x00000000, 0x02000000, 0,
     6500, 284, 4, 1, 0, 0, 0, 0, "1d1", 25, 0},
    {"& %C Potion~| of Restore Dexterity", potion1, 0x00010000, 0x04000000,
     0, 300, 285, 4, 1, 0, 0, 0, 0, "1d1", 40, 0},
    {"& %C Potion~| of Restore Constitution", potion1, 0x00000000,
     0x68000000, 0, 300, 286, 4, 1, 0, 0, 0, 0, "1d1", 40, 0},
    {"& %C Potion~| of Learning", potion1, 0x00008000, 0x00000000, 0, 200,
     287, 4, 1, 0, 0, 0, 0, "1d1", 45, 0},
    {"& %C Potion~| of Lose Memories", potion1, 0x00000001, 0x00000000, 0,
     0, 288, 4, 1, 0, 0, 0, 0, "1d1", 10, 0},
    {"& %C Potion~| of Salt Water", potion1, 0x00000002, 0x00000000, 0, 0,
     289, 4, 1, 0, 0, 0, 0, "1d1", 0, 0},
    {"& %C Potion~| of Invulnerability", potion1, 0x00000004, 0x00000000,
     0, 12500, 290, 4, 1, 0, 0, 0, 0, "1d1", 40, 0},
    {"& %C Potion~| of Heroism", potion1, 0x00000008, 0x00000000, 0, 35,
     291, 4, 1, 0, 0, 0, 0, "1d1", 1, 0},
    {"& %C Potion~| of Super Heroism", potion1, 0x00000010, 0x00000000, 0,
     100, 292, 4, 1, 0, 0, 0, 0, "1d1", 3, 0},
    {"& %C Potion~| of Boldliness", potion1, 0x00000020, 0x00000000, 0, 10,
     293, 4, 1, 0, 0, 0, 0, "1d1", 1, 0},
    {"& %C Potion~| of Restore Life Levels", potion1, 0x00000040,
     0x00000000, 0, 400, 294, 4, 1, 0, 0, 0, 0, "1d1", 40, 0},
    {"& %C Potion~| of Resist Heat", potion1, 0x00000080, 0x00000000, 0,
     30, 295, 4, 1, 0, 0, 0, 0, "1d1", 1, 0},
    {"& %C Potion~| of Resist Cold", potion1, 0x00000100, 0x00000000, 0,
     30, 296, 4, 1, 0, 0, 0, 0, "1d1", 1, 0},
    {"& %C Potion~| of Detect Invisible", potion1, 0x00000200, 0x00000000,
     0, 50, 297, 4, 1, 0, 0, 0, 0, "1d1", 3, 0},
    {"& %C Potion~| of Slow Poison", potion1, 0x00000400, 0x00000000, 0,
     25, 298, 4, 1, 0, 0, 0, 0, "1d1", 1, 0},
    {"& %C Potion~| of Neutralize Poison", potion1, 0x00000800, 0x00000000,
     0, 75, 299, 4, 1, 0, 0, 0, 0, "1d1", 5, 0},
    {"& %C Potion~| of Restore Mana", potion1, 0x00001000, 0x00000000, 0,
     3000, 300, 4, 1, 0, 0, 0, 0, "1d1", 25, 0},
    {"& %C Potion~| of Infra-Vision", potion1, 0x00002000, 0x00000000, 0,
     20, 301, 4, 1, 0, 0, 0, 0, "1d1", 3, 0},
    {"& %C Potion~| of Flea Bile", potion1, 0x00004000, 0x00000000, 0, 50,
     302, 4, 1, 0, 0, 0, 0, "1d1", 0, 0},
    {"& Flask~ of oil", flask_of_oil, 0x00000000, 0x00040000, 7500, 3, 257,
     10, 1, 0, 0, 0, 0, "2d6", 1, 0},
    {"& %M Wand| of Probing^ (%P1 charges)", wand, 0x00000000, 0x01000000,
     0, 1500, 25, 10, 1, 0, 0, 0, 0, "1d1", 30, 0},
    {"& %M Wand| of Light^ (%P1 charges)", wand, 0x00000000, 0x00000001, 0,
     200, 1, 10, 1, 0, 0, 0, 0, "1d1", 2, 0},
    {"& %M Wand| of Lightning Bolts^ (%P1 charges)", wand, 0x00000000,
     0x00000002, 0, 600, 2, 10, 1, 0, 0, 0, 0, "1d1", 15, 0},
    {"& %M Wand| of Frost Bolts^ (%P1 charges)", wand, 0x00000000,
     0x00000004, 0, 800, 3, 10, 1, 0, 0, 0, 0, "1d1", 20, 0},
    {"& %M Wand| of Fire Bolts^ (%P1 charges)", wand, 0x00000000,
     0x00000008, 0, 1000, 4, 10, 1, 0, 0, 0, 0, "1d1", 30, 0},
    {"& %M Wand| of Stone-to-Mud^ (%P1 charges)", wand, 0x00000000,
     0x00000010, 0, 300, 5, 10, 1, 0, 0, 0, 0, "1d1", 12, 0},
    {"& %M Wand| of Polymorph^ (%P1 charges)", wand, 0x00000000,
     0x00000020, 0, 400, 6, 10, 1, 0, 0, 0, 0, "1d1", 20, 0},
    {"& %M Wand| of Heal Monster^ (%P1 charges)", wand, 0x00000000,
     0x00000040, 0, 0, 7, 10, 1, 0, 0, 0, 0, "1d1", 2, 0},
    {"& %M Wand| of Haste Monster^ (%P1 charges)", wand, 0x00000000,
     0x00000080, 0, 0, 8, 10, 1, 0, 0, 0, 0, "1d1", 2, 0},
    {"& %M Wand| of Slow Monster^ (%P1 charges)", wand, 0x00000000,
     0x00000100, 0, 500, 9, 10, 1, 0, 0, 0, 0, "1d1", 2, 0},
    {"& %M Wand| of Confuse Monster^ (%P1 charges)", wand, 0x00000000,
     0x00000200, 0, 400, 10, 10, 1, 0, 0, 0, 0, "1d1", 2, 0},
    {"& %M Wand| of Sleep Monster^ (%P1 charges)", wand, 0x00000000,
     0x00000400, 0, 500, 11, 10, 1, 0, 0, 0, 0, "1d1", 7, 0},
    {"& %M Wand| of Drain Life^ (%P1 charges)", wand, 0x00000000,
     0x00000800, 0, 2500, 12, 10, 1, 0, 0, 0, 0, "1d1", 50, 0},
    {"& %M Wand| of Trap/Door destruction^ (%P1 charges)", wand, 0x00000000,
     0x00001000, 0, 100, 13, 10, 1, 0, 0, 0, 0, "1d1", 12, 0},
    {"& %M Wand| of Magic Missile^ (%P1 charges)", wand, 0x00000000,
     0x00002000, 0, 200, 14, 10, 1, 0, 0, 0, 0, "1d1", 2, 0},
    {"& %M Wand| of Wall Building^ (%P1 charges)", wand, 0x00000000,
     0x00004000, 0, 1400, 15, 10, 1, 0, 0, 0, 0, "1d1", 25, 0},
    {"& %M Wand| of Clone Monster^ (%P1 charges)", wand, 0x00000000,
     0x00008000, 0, 0, 16, 10, 1, 0, 0, 0, 0, "1d1", 2, 0},
    {"& %M Wand| of Teleport Away^ (%P1 charges)", wand, 0x00000000,
     0x00010000, 0, 350, 17, 10, 1, 0, 0, 0, 0, "1d1", 20, 0},
    {"& %M Wand| of Disarming^ (%P1 charges)", wand, 0x00000000,
     0x00020000, 0, 700, 18, 10, 1, 0, 0, 0, 0, "1d1", 20, 0},
    {"& %M Wand| of Lightning Balls^ (%P1 charges)", wand, 0x00000000,
     0x00040000, 0, 1200, 19, 10, 1, 0, 0, 0, 0, "1d1", 35, 0},
    {"& %M Wand| of Cold Balls^ (%P1 charges)", wand, 0x00000000,
     0x00080000, 0, 1500, 20, 10, 1, 0, 0, 0, 0, "1d1", 40, 0},
    {"& %M Wand| of Fire Balls^ (%P1 charges)", wand, 0x00000000,
     0x00100000, 0, 1800, 21, 10, 1, 0, 0, 0, 0, "1d1", 50, 0},
    {"& %M Wand| of Stinking Cloud^ (%P1 charges)", wand, 0x00000000,
     0x00200000, 0, 400, 22, 10, 1, 0, 0, 0, 0, "1d1", 5, 0},
    {"& %M Wand| of Acid Balls^ (%P1 charges)", wand, 0x00000000,
     0x00400000, 0, 2650, 23, 10, 1, 0, 0, 0, 0, "1d1", 48, 0},
    {"& %M Wand| of Wonder^ (%P1 charges)", wand, 0x00000000, 0x00800000,
     0, 250, 24, 10, 1, 0, 0, 0, 0, "1d1", 2, 0},
    {"& %W Staff| of Light^ (%P1 charges)", staff, 0x00000000, 0x00000001,
     0, 250, 1, 50, 1, 0, 0, 0, 0, "1d2", 5, 0},
    {"& %W Staff| of Door/Stair Location^ (%P1 charges)", staff, 
     0x00000000, 0x00000002, 0, 350, 2, 50, 1, 0, 0, 0, 0, "1d2", 10, 0},
    {"& %W Staff| of Trap Location^ (%P1 charges)", staff, 0x00000000,
     0x00000004, 0, 350, 3, 50, 1, 0, 0, 0, 0, "1d2", 10, 0},
    {"& %W Staff| of Treasure Location^ (%P1 charges)", staff, 0x00000000,
     0x00000008, 0, 200, 4, 50, 1, 0, 0, 0, 0, "1d2", 5, 0},
    {"& %W Staff| of Object Location^ (%P1 charges)", staff, 0x00000000,
     0x00000010, 0, 200, 5, 50, 1, 0, 0, 0, 0, "1d2", 5, 0},
    {"& %W Staff| of Teleportation^ (%P1 charges)", staff, 0x00000000,
     0x00000020, 0, 400, 6, 50, 1, 0, 0, 0, 0, "1d2", 20, 0},
    {"& %W Staff| of Earthquakes^ (%P1 charges)", staff, 0x00000000,
     0x00000040, 0, 350, 7, 50, 1, 0, 0, 0, 0, "1d2", 40, 0},
    {"& %W Staff| of Summoning^ (%P1 charges)", staff, 0x00000000,
     0x00000080, 0, 0, 8, 50, 1, 0, 0, 0, 0, "1d2", 10, 0},
    {"& %W Staff| of Summoning^ (%P1 charges)", staff, 0x00000000,
     0x00000080, 0, 0, 8, 50, 1, 0, 0, 0, 0, "1d2", 50, 0},
    {"& %W Staff| of *Destruction*^ (%P1 charges)", staff, 0x00000000,
     0x00000200, 0, 2500, 10, 50, 1, 0, 0, 0, 0, "1d2", 50, 0},
    {"& %W Staff| of Starlite^ (%P1 charges)", staff, 0x00000000,
     0x00000400, 0, 800, 11, 50, 1, 0, 0, 0, 0, "1d2", 20, 0},
    {"& %W Staff| of Haste Monsters^ (%P1 charges)", staff, 0x00000000,
     0x00000800, 0, 0, 12, 50, 1, 0, 0, 0, 0, "1d2", 10, 0},
    {"& %W Staff| of Slow Monsters^ (%P1 charges)", staff, 0x00000000,
     0x00001000, 0, 800, 13, 50, 1, 0, 0, 0, 0, "1d2", 10, 0},
    {"& %W Staff| of Sleep Monsters^ (%P1 charges)", staff, 0x00000000,
     0x00002000, 0, 700, 14, 50, 1, 0, 0, 0, 0, "1d2", 10, 0},
    {"& %W Staff| of Cure Light Wounds^ (%P1 charges)", staff, 0x00000000,
     0x00004000, 0, 350, 15, 50, 1, 0, 0, 0, 0, "1d2", 5, 0},
    {"& %W Staff| of Detect Invisible^ (%P1 charges)", staff, 0x00000000,
     0x00008000, 0, 200, 16, 50, 1, 0, 0, 0, 0, "1d2", 5, 0},
    {"& %W Staff| of Speed^ (%P1 charges)", staff, 0x00000000, 0x00010000,
     0, 800, 17, 50, 1, 0, 0, 0, 0, "1d2", 40, 0},
    {"& %W Staff| of Slowness^ (%P1 charges)", staff, 0x00000000,
     0x00020000, 0, 0, 18, 50, 1, 0, 0, 0, 0, "1d2", 40, 0},
    {"& %W Staff| of Mass Polymorph^ (%P1 charges)", staff, 0x00000000,
     0x00040000, 0, 1750, 19, 50, 1, 0, 0, 0, 0, "1d2", 46, 0},
    {"& %W Staff| of Remove Curse^ (%P1 charges)", staff, 0x00000000,
     0x00080000, 0, 500, 20, 50, 1, 0, 0, 0, 0, "1d2", 47, 0},
    {"& %W Staff| of Detect Evil^ (%P1 charges)", staff, 0x00000000,
     0x00100000, 0, 350, 21, 50, 1, 0, 0, 0, 0, "1d2", 20, 0},
    {"& %W Staff| of Curing^ (%P1 charges)", staff, 0x00000000, 0x00200000,
     0, 1000, 22, 50, 1, 0, 0, 0, 0, "1d2", 25, 0},
    {"& %W Staff| of Dispel Evil^ (%P1 charges)", staff, 0x00000000,
     0x00400000, 0, 1200, 23, 50, 1, 0, 0, 0, 0, "1d2", 49, 0},
    {"& %W Staff| of Darkness^ (%P1 charges)", staff, 0x00000000,
     0x01000000, 0, 0, 25, 50, 1, 0, 0, 0, 0, "1d2", 50, 0},
    {"& %W Staff| of Darkness^ (%P1 charges)", staff, 0x00000000,
     0x01000000, 0, 0, 25, 50, 1, 0, 0, 0, 0, "1d2", 5, 0},
    {"& %W Staff| of Identify^ (%P1 charges)", staff, 0x00000000,
     0x02000000, 0, 1500, 26, 50, 1, 0, 0, 0, 0, "1d2", 20, 0},
    {"& Book of Magic Spells [Beginners-Magik]", magic_book, 0x00000000,
     0x0000007F, 0, 25, 257, 60, 1, -100, 0, 0, 0, "1d1", 40, 0},
    {"& Book of Magic Spells [Magik I]", magic_book, 0x00000000,
     0x0007FF80, 0, 100, 258, 60, 1, -100, 0, 0, 0, "1d1", 40, 0},
    {"& Book of Magic Spells [Magik II]", magic_book, 0x00000000,
     0x7FF80000, 0, 400, 259, 60, 1, -100, 0, 0, 0, "1d1", 40, 0},
    {"& Book of Magic Spells [The Mages Guide to Power]", magic_book,
     0x000001FF, 0x00000000, 0, 800, 261, 60, 1, -100, 0, 0, 0, "1d1", 40, 0},
    {"& Holy Book of Prayers [Beginners Handbook]", prayer_book,
     0x00000000, 0x000000FF, 0, 25, 258, 60, 1, -100, 0, 0, 0, "1d1", 40, 0},
    {"& Holy Book of Prayers [Words of Wisdom]", prayer_book, 0x00000000,
     0x0007FF00, 0, 100, 259, 60, 1, -100, 0, 0, 0, "1d1", 40, 0},
    {"& Holy Book of Prayers [Chants and Blessings]", prayer_book,
     0x00000001, 0x7FF80000, 0, 300, 260, 60, 1, -100, 0, 0, 0, "1d1", 40, 0},
    {"& Holy Book of Prayers [Exorcism and Dispelling]", prayer_book,
     0x000001FE, 0x00000000, 0, 900, 261, 60, 1, -100, 0, 0, 0, "1d1", 40, 0},
    {"& Small wooden chest", chest, 0x00000000, 0x0F000000, 0, 300, 1, 250,
     1, 0, 0, 0, 0, "2d3", 7, 0},
    {"& Large wooden chest", chest, 0x00000000, 0x15000000, 0, 320, 4, 500,
     1, 0, 0, 0, 0, "2d5", 15, 0},
    {"& Small iron chest", chest, 0x00000000, 0x0F000000, 0, 420, 7, 300,
     1, 0, 0, 0, 0, "2d4", 25, 0},
    {"& Large iron chest", chest, 0x00000000, 0x1F000000, 0, 520, 10, 1000,
     1, 0, 0, 0, 0, "2d6", 35, 0},
    {"& Small steel chest", chest, 0x00000000, 0x0F000000, 0, 520, 13, 500,
     1, 0, 0, 0, 0, "2d4", 45, 0},
    {"& Large steel chest", chest, 0x00000000, 0x23000000, 0, 620, 16,
     1000, 1, 0, 0, 0, 0, "2d6", 50, 0},
    {"& %N Bag| of Holding (250)", bag_or_sack, 0x04000000, 0x00000000,
     25000, 1000, 1, 50, 1, 0, 0, 0, 0, "0d0", 35, 0},
    {"& %N Bag| of Holding (500)", bag_or_sack, 0x04000000, 0x00000000,
     50000, 2000, 2, 100, 1, 0, 0, 0, 0, "0d0", 45, 0},
    {"& %N Bag| of Holding (1000)", bag_or_sack, 0x04000000, 0x00000000,
     100000, 5000, 3, 200, 1, 0, 0, 0, 0, "0d0", 50, 0},
    {"& %N Bag| of Holding (1500)", bag_or_sack, 0x04000000, 0x00000000,
     150000, 7000, 3, 300, 1, 0, 0, 0, 0, "0d0", 50, 0},
    {"& %N Bag| of Devouring", bag_or_sack, 0x0C000000, 0x00000000, 150000,
     0, 4, 100, 1, 0, 0, 0, 0, "0d0", 20, 0},
    {"& Rat Skeleton", miscellaneous_object, 0x00000000, Nothing_flag, 0,
     0, 1, 10, 1, 0, 0, 0, 0, "1d1", 1, 0},
    {"& Giant Centipede Skeleton", miscellaneous_object, 0x00000000,
     Nothing_flag, 0, 0, 2, 25, 1, 0, 0, 0, 0, "1d1", 1, 0},
    {"Some filthy rags^ [%P6,%P4]", soft_armor, 0x00000000, Nothing_flag,
     0, 0, 99, 20, 1, 0, 0, 1, 0, "0d0", 0, 0},
    {"& empty bottle^", miscellaneous_object, 0x00000000, Nothing_flag, 0,
     0, 4, 2, 1, 0, 0, 0, 0, "1d1", 0, 0},
    {"Some shards of pottery^", miscellaneous_object, 0x00000000,
     Nothing_flag, 0, 0, 5, 5, 1, 0, 0, 0, 0, "1d1", 0, 0},
    {"& Human Skeleton", miscellaneous_object, 0x00000000, Nothing_flag, 0,
     0, 7, 50, 1, 0, 0, 0, 0, "1d1", 1, 0},
    {"& Dwarf Skeleton", miscellaneous_object, 0x00000000, Nothing_flag, 0,
     0, 8, 60, 1, 0, 0, 0, 0, "1d1", 1, 0},
    {"& Elf Skeleton", miscellaneous_object, 0x00000000, Nothing_flag, 0,
     0, 9, 40, 1, 0, 0, 0, 0, "1d1", 1, 0},
    {"& Gnome Skeleton", miscellaneous_object, 0x00000000, Nothing_flag, 0,
     0, 10, 25, 1, 0, 0, 0, 0, "1d1", 1, 0},
    {"& broken set of teeth^", miscellaneous_object, 0x00000000,
     Nothing_flag, 0, 0, 11, 3, 1, 0, 0, 0, 0, "1d1", 0, 0},
    {"& large broken bone^", miscellaneous_object, 0x00000000,
     Nothing_flag, 0, 0, 12, 2, 1, 0, 0, 0, 0, "1d1", 0, 0},
    {"& dead human body", chest, 0x00000000, 0x0F000000, 0, 0, 5, 50, 1, 0,
     0, 0, 0, "1d1", 7, 0},
    {"& broken stick^", miscellaneous_object, 0x00000000, Nothing_flag, 0,
     0, 13, 3, 1, 0, 0, 0, 0, "1d1", 0, 0},
    {"& broken set of teeth^", misc_usable, nothing_flag, Nothing_flag, 0,
     0, 15, 3, 1, 0, 0, 0, 0, "1d1", 0, 0},
    {"& %A Statue^", misc_usable, 0x00000000, nothing_flag, 0, 20, 14, 10,
     1, 0, 0, 0, 0, "1d2", 5, 0},
    {"& %A Statue^", misc_usable, 0x00000000, nothing_flag, 0, 20, 14, 10,
     1, 0, 0, 0, 0, "1d2", 10, 0},
    {"& %A Statue^", misc_usable, 0x00000000, nothing_flag, 0, 20, 14, 10,
     1, 0, 0, 0, 0, "1d2", 25, 0},
    {"& Finely cut %R| of Detect Monsters^ (%P1 charges)", valuable_gems, 
     nothing_flag, 0x00040000, 0, 350, 1, 5, 1, 0, 0, 0, 0, "0d0", 14, 0},
    {"& Finely cut %R| of Dispel Evil^ (%P1 charges)", valuable_gems, 
     nothing_flag, 0x00080000, 0, 1200, 2, 5, 1, 0, 0, 0, 0, "0d0", 49, 0},
    {"& Finely cut %R| of Darkness^ (%P1 charges)", valuable_gems, 
     nothing_flag, 0x00100000, 0, 0, 3, 5, 1, 0, 0, 0, 0, "0d0", 7, 0},
    {"& Finely cut %R| of Acid Balls^ (%P1 charges)", valuable_gems, 
     nothing_flag, 0x00200000, 0, 1800, 4, 5, 1, 0, 0, 0, 0, "0d0", 50, 0},
    {"& Finely cut %R| of Detect Invisible^ (%P1 charges)", valuable_gems, 
     nothing_flag, 0x00400000, 0, 225, 5, 5, 1, 0, 0, 0, 0, "0d0", 47, 0},
    {"& Finely cut %R| of Identify^ (%P1 charges)", valuable_gems, 
     nothing_flag, 0x00800000, 0, 400, 6, 5, 1, 0, 0, 0, 0, "0d0", 40, 0},
    {"& Finely cut %R| of Light^ (%P1 charges)", valuable_gems, 
     nothing_flag, 0x01000000, 0, 300, 7, 5, 1, 0, 0, 0, 0, "0d0", 2, 0},
    {"& Finely cut %R| of Summoning^ (%P1 charges)", valuable_gems, 
     nothing_flag, 0x02000000, 0, 0, 8, 5, 1, 0, 0, 0, 0, "0d0", 3, 0},
    {"& Finely cut %R| of Remove Curse^ (%P1 charges)", valuable_gems, 
     nothing_flag, 0x04000000, 0, 700, 9, 5, 1, 0, 0, 0, 0, "0d0", 25, 0},
    {"& Finely cut %R| of Annihilation^ (%P1 charges)", valuable_gems, 
     nothing_flag, 0x08000000, 0, 1000, 10, 5, 1, 0, 0, 0, 0, "0d0", 40, 0},
    {"& Finely cut %R| of Recall^ (%P1 charges)", valuable_gems, 
     nothing_flag, 0x10000000, 0, 1200, 11, 5, 1, 0, 0, 0, 0, "0d0", 22, 0},
    {"& Finely cut Agate~^", valuable_gems, 
     0x00000000, 0x00000000, 0, 50, 257, 5, 1, 0, 0, 0, 0, "0d0", 5, 0},
    {"& Finely cut Diamond~^", valuable_gems, 
     0x00000000, 0x00000000, 0, 500, 258, 5, 1, 0, 0, 0, 0, "0d0", 35, 0},
    {"& Rough cut Diamond~^", valuable_gems, 
     0x00000000, 0x00000000, 0, 100, 259, 5, 1, 0, 0, 0, 0, "0d0", 15, 0},
    {"& Rough cut Sapphire~^", valuable_gems, 
     0x00000000, 0x00000000, 0, 40, 260, 5, 1, 0, 0, 0, 0, "0d0", 7, 0},
    {"& Finely cut Sapphire~^", valuable_gems, 
     0x00000000, 0x00000000, 0, 250, 261, 5, 1, 0, 0, 0, 0, "0d0", 12, 0},
    {"& Small bag~ of Opals^", valuable_gems, 
     0x00000000, 0x00000000, 0, 250, 262, 5, 1, 0, 0, 0, 0, "0d0", 10, 0},
    {"& Small bag~ of Sapphires^", valuable_gems, 
     0x00000000, 0x00000000, 0, 450, 263, 5, 1, 0, 0, 0, 0, "0d0", 15, 0},
    {"& Small pouch~ of Diamonds^", valuable_gems, 
     0x00000000, 0x00000000, 0, 1000, 264, 5, 1, 0, 0, 0, 0, "0d0", 45, 0},
    {"& Large sack~ of Pearls^", valuable_gems, 
     0x00000000, 0x00000000, 0, 650, 265, 35, 1, 0, 0, 0, 0, "0d0", 25, 0},
    {"& Large sack~ of Sapphires^", valuable_gems, 
     0x00000000, 0x00000000, 0, 600, 266, 5, 1, 0, 0, 0, 0, "0d0", 30, 0},
    {"& Large pouch~ of Diamonds^", valuable_gems, 
     0x00000000, 0x00000000, 0, 2000, 267, 5, 1, 0, 0, 0, 0, "0d0", 65, 0},
    {"& Finely wrought silver necklace~^", amulet, 0x00000000, 0x00000000,
     0, 50, 30, 5, 1, 0, 0, 0, 0, "0d0", 1, 0},
    {"& Finely wrought gold necklace~^", amulet, 0x00000000, 0x00000000, 0,
     100, 40, 5, 1, 0, 0, 0, 0, "0d0", 7, 0},
    {"& Finely wrought mithril necklace~^", amulet, 0x00000000, 0x00000000,
     0, 400, 60, 5, 1, 0, 0, 0, 0, "0d0", 9, 0},
    {"& Small silver Bracelet~^", bracers, 0x00000000, 0x00000000, 0, 25,
     30, 5, 1, 0, 0, 0, 0, "0d0", 2, 0},
    {"& Small gold Bracelet~^", bracers, 0x00000000, 0x00000000, 0, 50,
     40, 5, 1, 0, 0, 0, 0, "0d0", 5, 0},
    {"& Small platinum bracelet~^", bracers, 0x00000000, 0x00000000, 0,
     100, 50, 5, 1, 0, 0, 0, 0, "0d0", 8, 0},
    {"& Small gold pendant~^", valuable_gems, 0x00000000, 0x00000000, 0,
     75, 274, 5, 1, 0, 0, 0, 0, "0d0", 5, 0},
    {"& Small mithril pendant~^", valuable_gems, 0x00000000, 0x00000000, 0,
     350, 275, 5, 1, 0, 0, 0, 0, "0d0", 10, 0},
    {"& Large mithril garter-belt~^", valuable_gems, 0x00000000, 0x00000000,
     0, 1500, 276, 5, 1, 0, 0, 0, 0, "0d0", 45, 0},
    {"& Silver Cross^", misc_usable, 0x00000000, 0x00000000, 0, 250, 16, 3,
     1, 0, 0, 0, 0, "1d1", 15, 0},
    {"& Gold Cross^", misc_usable, 0x00000000, 0x00000000, 0, 500, 17, 5,
     1, 0, 0, 0, 0, "1d1", 25, 0},
    {"& Mithril Cross^", misc_usable, 0x00000000, 0x00000000, 0, 750, 18,
     10, 1, 0, 0, 0, 0, "1d1", 45, 0},
    {"& %M Cross^", misc_usable, 0x00000000, 0x00000000, 0, 20, 19, 5, 1,
     0, 0, 0, 0, "1d1", 20, 0},
    {"& %M Cross^", misc_usable, 0x00000000, 0x00000000, 0, 20, 20, 5, 1,
     0, 0, 0, 0, "1d1", 35, 0},
    {"& Corked Bottle^", misc_usable, 0x00000000, 0x00000000, 0, 0, 21, 1,
     1, 0, 0, 0, 0, "1d1", 10, 0},
    {"& Holy Hand Grenade of Antioch^", misc_usable, 0x00000000,
     0x00020000, 1, 350, 24, 25, 1, 99, 0, 0, 0, "1d1", 25, 0},
    {"& %M Chime| of Light^ (%P1 charges)", chime, 0x00000000, 0x00000001, 0,
     275, 1, 1, 1, 0, 0, 0, 0, "0d0", 10, 0},
    {"& %M Chime| of Detect Doors/Stairs^ (%P1 charges)", chime, 0x00000000,
     0x00000002, 0, 375, 2, 1, 1, 0, 0, 0, 0, "0d0", 15, 0},
    {"& %M Chime| of Detect Traps^ (%P1 charges)", chime, 0x00000000,
     0x00000004, 0, 375, 3, 1, 1, 0, 0, 0, 0, "0d0", 15, 0},
    {"& %M Chime| of Teleportation^ (%P1 charges)", chime, 0x00000000,
     0x00000008, 0, 450, 4, 1, 1, 0, 0, 0, 0, "0d0", 23, 0},
    {"& %M Chime| of Thunderblasts^ (%P1 charges)", chime, 0x00000000,
     0x00000010, 0, 400, 5, 1, 1, 0, 0, 0, 0, "0d0", 42, 0},
    {"& %M Chime| of Summon Monster^ (%P1 charges)", chime, 0x00000000,
     0x00000020, 0, 0, 6, 1, 1, 0, 0, 0, 0, "0d0", 10, 0},
    {"& %M Chime| of Disarming^ (%P1 charges)", chime, 0x00000000, 0x00000040,
     0, 400, 7, 1, 1, 0, 0, 0, 0, "0d0", 30, 0},
    {"& %M Chime| of Aggravation^ (%P1 charges)", chime, 0x00000000,
     0x00000080, 0, 0, 8, 1, 1, 0, 0, 0, 0, "0d0", 15, 0},
    {"& %M Chime| of Slow Monster^ (%P1 charges)", chime, 0x00000000,
     0x00000100, 0, 850, 9, 1, 1, 0, 0, 0, 0, "0d0", 15, 0},
    {"& %M Chime| of Soothe Monster^ (%P1 charges)", chime, 0x00000000,
     0x00000200, 0, 800, 10, 1, 1, 0, 0, 0, 0, "0d0", 15, 0},
    {"& %M Chime| of Cure Light Wound^ (%P1 charges)", chime, 0x00000000,
     0x00000400, 0, 400, 11, 1, 1, 0, 0, 0, 0, "0d0", 10, 0},
    {"& %M Chime| of Changing^ (%P1 charges)", chime, 0x00000000, 0x00000800,
     0, 800, 12, 1, 1, 0, 0, 0, 0, "0d0", 46, 0},
    {"& %M Chime| of Remove Curse^ (%P1 charges)", chime, 0x00000000,
     0x00001000, 0, 675, 13, 1, 1, 0, 0, 0, 0, "0d0", 47, 0},
    {"& %M Chime| of Curing^ (%P1 charges)", chime, 0x00000000, 0x00002000, 0,
     1100, 14, 1, 1, 0, 0, 0, 0, "0d0", 27, 0},
    {"& %M Chime| of Dispel Evil^ (%P1 charges)", chime, 0x00000000,
     0x00004000, 0, 1300, 15, 1, 1, 0, 0, 0, 0, "0d0", 49, 0},
    {"& %M Chime| of Darkness^ (%P1 charges)", chime, 0x00000000, 0x00008000,
     0, 0, 16, 1, 1, 0, 0, 0, 0, "0d0", 20, 0},
    {"& %H| of Bubbles^ (%P1 charges)", horn, 0x00000000, 0x00010000, 0, 0,
     1, 20, 1, 0, 0, 0, 0, "0d0", 15, 0},
    {"& %H| of Calling^ (%P1 charges)", horn, 0x00000000, 0x00020000, 0, 0,
     2, 20, 1, 0, 0, 0, 0, "0d0", 10, 0},
    {"& %H| of Soft Sounds^ (%P1 charges)", horn, 0x00000000, 0x00040000, 0,
     600, 3, 20, 1, 0, 0, 0, 0, "0d0", 8, 0},
    {"& %H| of *Blasting*^ (%P1 charges)", horn, 0x00000000, 0x00080000, 0,
     2600, 4, 20, 1, 0, 0, 0, 0, "0d0", 49, 0},
    {"& %H| of Cold^ (%P1 charges)", horn, 0x00000000, 0x00100000, 0, 1000,
     5, 20, 1, 0, 0, 0, 0, "0d0", 40, 0},
    {"& %H| of Heat^ (%P1 charges)", horn, 0x00000000, 0x00200000, 0, 1000,
     6, 20, 1, 0, 0, 0, 0, "0d0", 40, 0},
    {"& %H| of Gas^ (%P1 charges)", horn, 0x00000000, 0x00400000, 0, 900, 7,
     20, 1, 0, 0, 0, 0, "0d0", 35, 0},
    {"& %H| of Recall^ (%P1 charges)", horn, 0x00000000, 0x00800000, 0, 1200,
     8, 20, 1, 0, 0, 0, 0, "0d0", 30, 0},
    {"& %H| of *Chaos*^ (%P1 charges)", horn, 0x00000000, 0x01000000, 0, 800,
     9, 20, 1, 0, 0, 0, 0, "0d0", 43, 0},
    {"& %H| of Glue^ (%P1 charges)", horn, 0x00000000, 0x02000000, 0, 0, 10,
     20, 1, 0, 0, 0, 0, "0d0", 20, 0},
    {"& %H| of Valhalla^ (%P1 charges)", horn, 0x00000000, 0x04000000, 0,
     2700, 11, 20, 1, 0, 0, 0, 0, "0d0", 50, 0},
    {"& %H| of Tritons^ (%P1 charges)", horn, 0x00000000, 0x08000000, 0, 200,
     12, 20, 1, 0, 0, 0, 0, "0d0", 15, 0},
    {"& %H| of Fog^ (%P1 charges)", horn, 0x00000000, 0x10000000, 0, 500, 13,
     20, 1, 0, 0, 0, 0, "0d0", 25, 0},
    /* Instruments are not in use */
    {"& Pipes of Peace", instrument, 0x00000000, 0x000003FF,
     0, 30, 258, 40, 1, -100, 0, 0, 0, "1d1", 40, 0},
    {"& Lyre of Nature", instrument, 0x00000000, 0x000FFC00, 0, 105,
     259, 40, 1, -100, 0, 0, 0, "0d0", 40, 0},
    {"& Lute of the Woods", instrument, 0x00000000, 0x7FF00000, 0,
     320, 260, 40, 1, -100, 0, 0, 0, "0d0", 40, 0},
    {"& Harp of the Druids", instrument, 0x000001FF,
     0x00000000, 0, 850, 261, 40, 1, -100, 0, 0, 0, "2d1", 40, 0},

    {"& Book of Bard Lyrics [Beginners Handbook]", song_book, 0x00000000,
     0x000007FF, 0, 30, 262, 50, 1, -100, 0, 0, 0, "0d0", 40, 0},
    {"& Songs of Charming [Song Book I]", song_book, 0x00000000, 0x000FF800, 0,
     105, 263, 60, 1, -100, 0, 0, 0, "1d1", 40, 0},
    {"& Ballads of Knowledge [Song Book II]", song_book, 0x00000000, 0x7FF00000,
     0, 305, 264, 60, 1, -100, 0, 0, 0, "1d1", 40, 0},
    {"& Epics of the Bards [Greater Song Book]", song_book, 0x000001FF,
     0x00000000, 0, 950, 265, 60, 1, -100, 0, 0, 0, "0d0", 40, 0}

};

boolean object_ident[MAX_OBJECTS + 1]; /*(max_objects of false) */
long t_level[MAX_OBJ_LEVEL + 1];

/*	{ Gold list (All types of gold and gems are defined here)	} */
treasure_type gold_list[MAX_GOLD] = {
    {"& copper piece~", valuable_metal, 0, 0, 0, 0, 2, 5, 420, 0, 0, 0, 0,
     " ", 2, 0},
    {"& iron piece~", valuable_metal, 0, 0, 0, 0, 1, 5, 2400, 0, 0, 0, 0,
     " ", 1, 0},
    {"& copper piece~", valuable_metal, 0, 0, 0, 0, 4, 5, 720, 0, 0, 0, 0,
     " ", 2, 0},
    {"& silver piece~", valuable_metal, 0, 0, 0, 0, 5, 5, 180, 0, 0, 0, 0,
     " ", 3, 0},
    {"& iron piece~", valuable_metal, 0, 0, 0, 0, 3, 5, 4800, 0, 0, 0, 0,
     " ", 1, 0},
    {"& copper piece~", valuable_metal, 0, 0, 0, 0, 6, 5, 1200, 0, 0, 0, 0,
     " ", 2, 0},
    {"& silver piece~", valuable_metal, 0, 0, 0, 0, 7, 5, 300, 0, 0, 0, 0,
     " ", 3, 0},
    {"& gold piece~", valuable_metal, 0, 0, 0, 0, 12, 5, 30, 0, 0, 0, 0,
     " ", 4, 0},
    {"& iron piece~", valuable_metal, 0, 0, 0, 0, 3, 5, 9000, 0, 0, 0, 0,
     " ", 1, 0},
    {"& copper piece~", valuable_metal, 0, 0, 0, 0, 6, 5, 2400, 0, 0, 0, 0,
     " ", 2, 0},
    {"& silver piece~", valuable_metal, 0, 0, 0, 0, 7, 5, 600, 0, 0, 0, 0,
     " ", 3, 0},
    {"& gold piece~", valuable_metal, 0, 0, 0, 0, 12, 5, 60, 0, 0, 0, 0,
     " ", 4, 0},
    {"& copper piece~", valuable_metal, 0, 0, 0, 0, 6, 5, 6000, 0, 0, 0, 0,
     " ", 2, 0},
    {"& silver piece~", valuable_metal, 0, 0, 0, 0, 7, 5, 1200, 0, 0, 0, 0,
     " ", 3, 0},
    {"& gold piece~", valuable_metal, 0, 0, 0, 0, 12, 5, 120, 0, 0, 0, 0,
     " ", 4, 0},
    {"& silver piece~", valuable_metal, 0, 0, 0, 0, 7, 5, 1500, 0, 0, 0, 0,
     " ", 3, 0},
    {"& gold piece~", valuable_metal, 0, 0, 0, 0, 18, 5, 150, 0, 0, 0, 0,
     " ", 4, 0},
    {"& platinum piece~", valuable_metal, 0, 0, 0, 0, 20, 5, 50, 0, 0, 0,
     0, " ", 5, 0},
    {"& silver piece~", valuable_metal, 0, 0, 0, 0, 16, 5, 3000, 0, 0, 0,
     0, " ", 3, 0},
    {"& gold piece~", valuable_metal, 0, 0, 0, 0, 24, 5, 250, 0, 0, 0, 0,
     " ", 4, 0},
    {"& platinum piece~", valuable_metal, 0, 0, 0, 0, 28, 5, 75, 0, 0, 0,
     0, " ", 5, 0},
    {"& mithril piece~", valuable_metal, 0, 0, 0, 0, 32, 5, 8, 0, 0, 0, 0,
     " ", 6, 0},
    {"& gold piece~", valuable_metal, 0, 0, 0, 0, 50, 5, 600, 0, 0, 0, 0,
     " ", 4, 0},
    {"& platinum piece~", valuable_metal, 0, 0, 0, 0, 55, 5, 200, 0, 0, 0,
     0, " ", 5, 0},
    {"& mithril piece~", valuable_metal, 0, 0, 0, 0, 60, 5, 20, 0, 0, 0, 0,
     " ", 6, 0}

};

treasure_type t_list[MAX_TALLOC + 1];
treasure_type equipment[EQUIP_MAX];
treas_rec* inventory_list = NULL;
treas_ptr inven_temp;

/*	{ Items which are sold in the stores are different from dungeon } */
/*	{ items so that identify works properly.  Note that the players } */
/*	{ receive their initial items from this list, so position is	} */
/*	{ very important...						} */

treasure_type inventory_init[INVEN_INIT_MAX + 1] = {
    {"& Bogus Ration~ of Food", Food, 0x00000000, Nothing_flag, 5000, 3,
     307, 10, 5, 0, 0, 0, 0, "0d0", 0, 1}, /*{  0} */
    {"& Ration~ of Food", Food, 0x00000000, Nothing_flag, 5000, 3, 307, 10,
     5, 0, 0, 0, 0, "0d0", 0, 1}, /*{  1} */
    {"& Hard Biscuit~", Food, 0x00000000, Nothing_flag, 500, 1, 309, 2, 5,
     0, 0, 0, 0, "0d0", 0, 1}, /*{  2} */
    {"& Strip~ of Beef Jerky", Food, 0x00000000, Nothing_flag, 1750, 2,
     310, 2, 5, 0, 0, 0, 0, "0d0", 0, 1}, /*{  3} */
    {"& Pint of Fine Ale", Food, 0x00000000, Nothing_flag, 500, 1, 311, 10,
     3, 0, 0, 0, 0, "0d0", 0, 1}, /*{  4} */
    {"& Pint of Fine Wine", Food, 0x00000000, Nothing_flag, 400, 2, 312,
     10, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{  5} */
    {"& Misercorde (%P0) (%P2,%P3)", dagger, 0x10000000, Nothing_flag, 0,
     10, 2, 15, 1, 0, 0, 0, 0, "1d4", 0, 1}, /*{  6} */
    {"& Stiletto (%P0) (%P2,%P3)", dagger, 0x10000000, Nothing_flag, 0, 10,
     3, 12, 1, 0, 0, 0, 0, "1d4", 0, 1}, /*{  7} */
    {"& Bastard Sword (%P0) (%P2,%P3)", sword, 0x10000000, Nothing_flag, 0,
     350, 7, 140, 1, 0, 0, 0, 0, "3d4", 0, 1}, /*{  8} */
    {"& Broadsword (%P0) (%P2,%P3)", sword, 0x10000000, Nothing_flag, 0,
     255, 10, 150, 1, 0, 0, 0, 0, "2d5", 0, 1}, /*{  9} */
    {"& Longsword (%P0) (%P2,%P3)", sword, 0x10000000, Nothing_flag, 0,
     300, 18, 130, 1, 0, 0, 0, 0, "1d10", 0, 1}, /*{ 10} */
    {"& Small Sword (%P0) (%P2,%P3)", dagger, 0x10000000, Nothing_flag, 0,
     48, 22, 75, 1, 0, 0, 0, 0, "1d6", 0, 1}, /*{ 11} */
    {"& Broad Axe (%P0) (%P2,%P3)", hafted_weapon, 0x10000000,
     Nothing_flag, 0, 304, 4, 160, 1, 0, 0, 0, 0, "2d5", 0, 1}, /*{ 12} */
    {"& Morningstar (%P0) (%P2,%P3)", maul, 0x00000000, Nothing_flag, 0,
     396, 9, 150, 1, 0, 0, 0, 0, "2d6", 0, 1}, /*{ 13} */
    {"& Mace (%P0) (%P2,%P3)", maul, 0x00000000, Nothing_flag, 0, 130, 10,
     120, 1, 0, 0, 0, 0, "2d4", 0, 1}, /*{ 14} */
    {"& War Hammer (%P0) (%P2,%P3)", maul, 0x00000000, Nothing_flag, 0,
     225, 11, 120, 1, 0, 0, 0, 0, "3d3", 0, 1}, /*{ 15} */
    {"& Halberd (%P0) (%P2,%P3)", pole_arm, 0x10000000, Nothing_flag, 0,
     430, 5, 190, 1, 0, 0, 0, 0, "3d4", 0, 1}, /*{ 16} */
    {"& Pike (%P0) (%P2,%P3)", pole_arm, 0x10000000, Nothing_flag, 0, 358,
     7, 160, 1, 0, 0, 0, 0, "2d5", 0, 1}, /*{ 17} */
    {"& Spear (%P0) (%P2,%P3)", pole_arm, 0x10000000, Nothing_flag, 0, 36,
     8, 50, 1, 0, 0, 0, 0, "1d6", 0, 1}, /*{ 18} */
    {"& Short Bow (%P2)", bow_crossbow_or_sling, 0x00000000, Nothing_flag,
     2, 50, 1, 30, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{ 19} */
    {"& Long Bow (%P2)", bow_crossbow_or_sling, 0x00000000, Nothing_flag,
     3, 120, 2, 40, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{ 20} */
    {"& Light Crossbow (%P2)", bow_crossbow_or_sling, 0x00000000,
     Nothing_flag, 5, 160, 10, 110, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{ 21} */
    {"& Sling (%P2)", bow_crossbow_or_sling, 0x00000000, Nothing_flag, 1,
     5, 20, 5, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{ 22} */
    {"& Arrow~ (%P2,%P3)", arrow, 0x10000000, Nothing_flag, 0, 1, 1, 2, 1,
     0, 0, 0, 0, "3d4", 0, 1}, /*{ 23} */
    {"& Bolt~ (%P2,%P3)", bolt, 0x10000000, Nothing_flag, 0, 2, 1, 3, 1, 0,
     0, 0, 0, "3d5", 0, 1}, /*{ 24} */
    {"& Iron Shot~ (%P2,%P3)", sling_ammo, 0x00000000, Nothing_flag, 0, 2,
     1, 5, 1, 0, 0, 0, 0, "3d3", 0, 1}, /*{ 25} */
    {"& Pick (%P1) (%P2,%P3)", pick_or_shovel, 0x10000000, Tunneling_worn_bit, 1, 50,
     1, 150, 1, 0, 0, 0, 0, "1d3", 0, 1}, /*{ 26} */
    {"& Shovel (%P1) (%P2,%P3)", pick_or_shovel, 0x00000000, Tunneling_worn_bit, 0,
     15, 2, 60, 1, 0, 0, 0, 0, "1d2", 0, 1}, /*{ 27} */
    {"& Pair of Soft Leather Boots [%P6,%P4]", boots, 0x00000000,
     Nothing_flag, 0, 7, 2, 30, 1, 0, 0, 2, 0, "1d1", 0, 1}, /*{ 28} */
    {"& Pair of Hard Leather Boots [%P6,%P4]", boots, 0x00000000,
     Nothing_flag, 0, 12, 3, 40, 1, 0, 0, 3, 0, "1d1", 0, 1}, /*{ 29} */
    {"& Hard Leather Cap [%P6,%P4]", helm, 0x00000000, Nothing_flag, 0, 12,
     2, 15, 1, 0, 0, 2, 0, "0d0", 0, 1}, /*{ 30} */
    {"& Metal Cap [%P6,%P4]", helm, 0x00000000, Nothing_flag, 0, 30, 3, 20,
     1, 0, 0, 3, 0, "1d1", 0, 1}, /*{ 31} */
    {"& Iron Helm [%P6,%P4]", helm, 0x00000000, Nothing_flag, 0, 75, 4, 75,
     1, 0, 0, 5, 0, "1d3", 0, 1}, /*{ 32} */
    {"Soft Leather Armor [%P6,%P4]", soft_armor, 0x00000000, Nothing_flag,
     0, 18, 2, 80, 1, 0, 0, 4, 0, "0d0", 0, 1}, /*{ 33} */
    {"Soft Studded Leather [%P6,%P4]", soft_armor, 0x00000000,
     Nothing_flag, 0, 35, 3, 90, 1, 0, 0, 5, 0, "1d1", 0, 1}, /*{ 34} */
    {"Hard Leather Armor [%P6,%P4]", soft_armor, 0x00000000, Nothing_flag,
     0, 55, 4, 100, 1, -1, 0, 6, 0, "1d1", 0, 1}, /*{ 35} */
    {"Hard Studded Leather [%P6,%P4]", soft_armor, 0x00000000,
     Nothing_flag, 0, 100, 5, 110, 1, -1, 0, 7, 0, "1d2", 0, 1}, /*{ 36} */
    {"Leather Scale Mail [%P6,%P4]", soft_armor, 0x00000000, Nothing_flag,
     0, 330, 9, 140, 1, -1, 0, 11, 0, "1d1", 0, 1}, /*{ 37} */
    {"Metal Scale Mail [%P6,%P4]", hard_armor, 0x00000000, Nothing_flag, 0,
     430, 1, 250, 1, -2, 0, 13, 0, "1d4", 0, 1}, /*{ 38} */
    {"Chain Mail [%P6,%P4]", hard_armor, 0x00000000, Nothing_flag, 0, 530,
     2, 220, 1, -2, 0, 14, 0, "1d4", 0, 1}, /*{ 39} */
    {"Partial Plate Armor [%P6,%P4]", hard_armor, 0x00000000, Nothing_flag,
     0, 900, 9, 260, 1, -3, 0, 22, 0, "1d6", 0, 1}, /*{ 40} */
    {"Full Plate Armor [%P6,%P4]", hard_armor, 0x00000000, Nothing_flag, 0,
     1050, 11, 380, 1, -3, 0, 25, 0, "2d4", 0, 1}, /*{ 41} */
    {"& Cloak [%P6,%P4]", cloak, 0x00000000, Nothing_flag, 0, 3, 1, 10, 1,
     0, 0, 1, 0, "0d0", 0, 1}, /*{ 42} */
    {"& Set of Leather Gloves [%P6,%P4]", gloves_and_gauntlets, 0x00000000,
     Nothing_flag, 0, 3, 1, 5, 1, 0, 0, 1, 0, "0d0", 0, 1}, /*{ 43} */
    {"& Set of Gauntlets [%P6,%P4]", gloves_and_gauntlets, 0x00000000,
     Nothing_flag, 0, 35, 2, 25, 1, 0, 0, 2, 0, "1d1", 0, 1}, /*{ 44} */
    {"& Small Leather Shield [%P6,%P4]", shield, 0x00000000, Nothing_flag,
     0, 30, 1, 50, 1, 0, 0, 2, 0, "1d1", 0, 1}, /*{ 45} */
    {"& Medium Leather Shield [%P6,%P4]", shield, 0x00000000, Nothing_flag,
     0, 60, 2, 75, 1, 0, 0, 3, 0, "1d2", 0, 1}, /*{ 46} */
    {"& Small Metal Shield [%P6,%P4]", shield, 0x00000000, Nothing_flag, 0,
     50, 4, 65, 1, 0, 0, 3, 0, "1d3", 0, 1}, /*{ 47} */
    {"& Ring of Resist Fire", ring, 0x00000000, 0x00080000, 0, 250, 11, 2,
     1, 0, 0, 0, 0, "0d0", 0, 1}, /*{ 48} */
    {"& Ring of Resist Cold", ring, 0x00000000, 0x00200000, 0, 250, 12, 2,
     1, 0, 0, 0, 0, "0d0", 0, 1}, /*{ 49} */
    {"& Ring of Feather Falling", ring, 0x00000000, 0x04000000, 0, 250, 13,
     2, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{ 50} */
    {"& Ring of Protection [%P4]", ring, 0x00000000, Nothing_flag, 0, 100,
     24, 2, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{ 51} */
    {"& Amulet of Charisma (%P1)", amulet, 0x00000000, 0x00000020, 0, 250,
     6, 3, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{ 52} */
    {"& Amulet of Slow Digestion", amulet, 0x00000000, 0x00000080, 0, 200,
     9, 3, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{ 53} */
    {"& Amulet of Resist Acid", amulet, 0x00000000, 0x00100000, 0, 300, 10,
     3, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{ 54} */
    {"& Scroll~ of Enchant Weapon To-Hit", scroll1, 0x00000000, 0x00000001,
     0, 125, 300, 5, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{ 55} */
    {"& Scroll~ of Enchant Weapon To-Dam", scroll1, 0x00000000, 0x00000002,
     0, 125, 301, 5, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{ 56} */
    {"& Scroll~ of Enchant Armor", scroll1, 0x00000000, 0x00000004, 0, 125,
     302, 5, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{ 57} */
    {"& Scroll~ of Identify", scroll1, 0x00000000, 0x00000008, 0, 50, 303,
     5, 2, 0, 0, 0, 0, "0d0", 0, 1}, /*{ 58} */
    {"& Scroll~ of Remove Curse", scroll1, 0x00000000, 0x00000010, 0, 100,
     304, 5, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{ 59} */
    {"& Scroll~ of Light", scroll1, 0x00000000, 0x00000020, 0, 15, 305, 5,
     3, 0, 0, 0, 0, "0d0", 0, 1}, /*{ 60} */
    {"& Scroll~ of Phase Door", scroll1, 0x00000000, 0x00000080, 0, 15,
     306, 5, 2, 0, 0, 0, 0, "0d0", 0, 1}, /*{ 61} */
    {"& Scroll~ of Magic Mapping", scroll1, 0x00000000, 0x00000800, 0, 40,
     307, 5, 2, 0, 0, 0, 0, "0d0", 0, 1}, /*{ 62} */
    {"& Scroll~ of Treasure Detection", scroll1, 0x00000000, 0x00004000, 0,
     15, 308, 5, 2, 0, 0, 0, 0, "0d0", 0, 1}, /*{ 63} */
    {"& Scroll~ of Object Detection", scroll1, 0x00000000, 0x00008000, 0,
     15, 309, 5, 2, 0, 0, 0, 0, "0d0", 0, 1}, /*{ 64} */
    {"& Scroll~ of Detect Invisible", scroll1, 0x00000000, 0x00080000, 0,
     15, 310, 5, 2, 0, 0, 0, 0, "0d0", 0, 1}, /*{ 65} */
    {"& Scroll~ of Recharging", scroll1, 0x00000000, 0x01000000, 0, 200,
     311, 5, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{ 66} */
    {"& Book of Magic Spells [Beginners-Magik]", magic_book, 0x00000000,
     0x0000007F, 0, 25, 257, 60, 1, -100, 0, 0, 0, "1d1", 0, 1}, /*{ 67} */
    {"& Book of Magic Spells [Magik I]", magic_book, 0x00000000,
     0x0007FF80, 0, 100, 258, 60, 1, -100, 0, 0, 0, "1d1", 0, 1}, /*{ 68} */
    {"& Book of Magic Spells [Magik II]", magic_book, 0x00000000,
     0x7FF80000, 0, 400, 259, 60, 1, -100, 0, 0, 0, "1d1", 0, 1}, /*{ 69} */
    {"& Book of Magic Spells [Mages Guide to Power]", magic_book, 
     0x000001FF, 0x00000000, 0, 800, 260, 60, 1, -100, 0, 0, 0, "1d1",
     0, 1}, /*{ 70} */
    {"& Holy Book of Prayers [Beginners Handbook]", prayer_book, 
     0x00000000, 0x000000FF, 0, 25, 258, 60, 1, -100, 0, 0, 0, "1d1",
     0, 1}, /*{ 71} */
    {"& Holy Book of Prayers [Words of Wisdom]", prayer_book, 0x00000000,
     0x0007FF00, 0, 100, 259, 60, 1, -100, 0, 0, 0, "1d1", 0, 1}, /*{ 72} */
    {"& Holy Book of Prayers [Chants and Blessings]", prayer_book, 
     0x00000001, 0x7FF80000, 0, 300, 260, 60, 1, -100, 0, 0, 0, "1d1",
     0, 1}, /*{ 73} */
    {"& Holy Book of Prayers [Exorcism and Dispelling]", prayer_book, 
     0x000001FE, 0x00000000, 0, 900, 261, 60, 1, -100, 0, 0, 0, "1d1",
     0, 1}, /*{ 74} */
    {"& Potion~ of Restore Strength", potion1, 0x00000000, 0x00000004, 0,
     300, 310, 4, 1, 0, 0, 0, 0, "1d1", 0, 1}, /*{ 75} */
    {"& Potion~ of Restore Intelligence", potion1, 0x00000000, 0x00000020,
     0, 300, 311, 4, 1, 0, 0, 0, 0, "1d1", 0, 1}, /*{ 76} */
    {"& Potion~ of Restore Wisdom", potion1, 0x00000000, 0x00000100, 0,
     300, 312, 4, 1, 0, 0, 0, 0, "1d1", 0, 1}, /*{ 77} */
    {"& Potion~ of Restore Charisma", potion1, 0x00000000, 0x00000800, 0,
     300, 313, 4, 1, 0, 0, 0, 0, "1d1", 0, 1}, /*{ 78} */
    {"& Potion~ of Cure Light Wounds", potion1, 0x00000000, 0x10001000, 50,
     15, 314, 4, 2, 0, 0, 0, 0, "1d1", 0, 1}, /*{ 79} */
    {"& Potion~ of Cure Serious Wounds", potion1, 0x00000000, 0x30002000,
     100, 40, 315, 4, 1, 0, 0, 0, 0, "1d1", 0, 1}, /*{ 80} */
    {"& Potion~ of Cure Critical Wounds", potion1, 0x00000000, 0x70004000,
     100, 100, 316, 4, 1, 0, 0, 0, 0, "1d1", 0, 1}, /*{ 81} */
    {"& Potion~ of Restore Dexterity", potion1, 0x00010000, 0x04000000, 0,
     300, 317, 4, 1, 0, 0, 0, 0, "1d1", 0, 1}, /*{ 82} */
    {"& Potion~ of Restore Constitution", potion1, 0x00000000, 0x68000000,
     0, 300, 318, 4, 1, 0, 0, 0, 0, "1d1", 0, 1}, /*{ 83} */
    {"& Potion~ of Heroism", potion1, 0x00000008, 0x00000000, 0, 35, 319,
     4, 2, 0, 0, 0, 0, "1d1", 0, 1}, /*{ 84} */
    {"& Potion~ of Boldliness", potion1, 0x00000020, 0x00000000, 0, 10,
     320, 4, 2, 0, 0, 0, 0, "1d1", 0, 1}, /*{ 85} */
    {"& Wand of Light (%P1 charges)", wand, 0x00000000, 0x00000001, 0, 200,
     1, 10, 1, 0, 0, 0, 0, "1d1", 2, 1}, /*{ 86} */
    {"& Wand of Lightning Bolts (%P1 charges)", wand, 0x00000000,
     0x00000002, 0, 600, 2, 10, 1, 0, 0, 0, 0, "1d1", 6, 1}, /*{ 87} */
    {"& Wand of Magic Missile (%P1 charges)", wand, 0x00000000, 0x00002000,
     0, 200, 14, 10, 1, 0, 0, 0, 0, "1d1", 2, 1}, /*{ 88} */
    {"& Wand of Disarming (%P1 charges)", wand, 0x00000000, 0x00020000, 0,
     700, 18, 10, 1, 0, 0, 0, 0, "1d1", 12, 1}, /*{ 89} */
    {"& Wand of Lightning Balls (%P1 charges)", wand, 0x00000000,
     0x00040000, 0, 1200, 19, 10, 1, 0, 0, 0, 0, "1d1", 20, 1}, /*{ 90} */
    {"& Wand of Wonder (%P1 charges)", wand, 0x00000000, 0x00800000, 0,
     250, 24, 10, 1, 0, 0, 0, 0, "1d1", 10, 1}, /*{ 91} */
    {"& Staff of Light (%P1 charges)", staff, 0x00000000, 0x00000001, 0,
     250, 1, 50, 1, 0, 0, 0, 0, "1d2", 3, 1}, /*{ 92} */
    {"& Staff of Door/Stair Location (%P1 charges)", staff, 0x00000000,
     0x00000002, 0, 350, 2, 50, 1, 0, 0, 0, 0, "1d2", 7, 1}, /*{ 93} */
    {"& Staff of Trap Location (%P1 charges)", staff, 0x00000000,
     0x00000004, 0, 350, 3, 50, 1, 0, 0, 0, 0, "1d2", 7, 1}, /*{ 94} */
    {"& Staff of Detect Invisible (%P1 charges)", staff, 0x00000000,
     0x00008000, 0, 200, 16, 50, 1, 0, 0, 0, 0, "1d2", 3, 1}, /*{ 95} */
    {"& Potion~ of Restore Life Levels", potion1, 0x00000040, 0x00000000,
     0, 400, 321, 4, 1, 0, 0, 0, 0, "1d1", 0, 1}, /*{ 96} */
    {"& Scroll~ of Blessing", scroll1, 0x00000010, 0x00000000, 0, 15, 312,
     5, 2, 0, 0, 0, 0, "0d0", 0, 1}, /*{ 97} */
    {"& Scroll~ of Word-of-Recall", scroll1, 0x00000080, 0x00000000, 0,
     150, 313, 5, 3, 0, 0, 0, 0, "0d0", 0, 1}, /*{ 98} */
    {"& Potion~ of Slow Poison", potion1, 0x00000400, 0x00000000, 0, 25,
     322, 4, 2, 0, 0, 0, 0, "1d1", 0, 1}, /*{ 99} */
    {"& Potion~ of Neutralize Poison", potion1, 0x00000800, 0x00000000, 0,
     75, 323, 4, 1, 0, 0, 0, 0, "1d1", 0, 1}, /*{100} */
    {"& Wand of Stinking Cloud (%P1 charges)", wand, 0x00000000,
     0x00200000, 0, 400, 22, 10, 1, 0, 0, 0, 0, "1d1", 5, 1}, /*{101} */
    {"& Iron Spike~", spike, 0x10000000, Nothing_flag, 0, 1, 1, 10, 1, 0,
     0, 0, 0, "1d1", 1, 1}, /*{102} */
    {"& Brass Lantern~ with %P5 turns of light", lamp_or_torch, 0x00000000,
     Nothing_flag, 7500, 35, 2, 50, 1, 0, 0, 0, 0, "1d1", 1, 1}, /*{103} */
    {"& Wooden Torch~ with %P5 turns of light", lamp_or_torch, 0x00000000,
     Nothing_flag, 4000, 2, 14, 30, 1, 0, 0, 0, 0, "1d1", 1, 1}, /*{104} */
    {"& Flask~ of oil", flask_of_oil, 0x00000000, 0x00040000, 7500, 3, 257,
     10, 5, 0, 0, 0, 0, "2d6", 1, 1}, /*{105} */
    {"Lodging for one day", lodging_at_inn, 0x00000000, Nothing_flag, 1,
     50, 300, 3000, 14, 0, 0, 0, 0, "0d0", 0, 1}, /*{106} */
    /* Instruments are not in use */
    {"& Pipes of Peace", instrument, 0x00000000,
     0x000003FF, 0, 30, 258, 40, 1, -100, 0, 0, 0, "1d1", 40, 1}, /*{107} */
    {"& Lyre of Nature", instrument, 0x00000000, 0x0007FC00,
     0, 105, 259, 40, 1, -100, 0, 0, 0, "0d0", 40, 1}, /*{108} */
    {"& Lute of the Woods", instrument, 0x00000000,
     0x7FF80000, 0, 320, 260, 40, 1, -100, 0, 0, 0, "0d0", 40, 1}, /*{109} */
    {"& Harp of the Druids", instrument, 0x000001FF,
     0x00000000, 0, 850, 261, 40, 1, -100, 0, 0, 0, "2d1", 40, 1}, /*{110} */

    {"& Book of Bard Lyrics [Beginners Song book]", song_book, 0x00000000,
     0x000007FF, 0, 30, 262, 50, 1, -100, 0, 0, 0, "0d0", 40, 1}, /*{111} */
    {"& Songs of Charming [Song Book I]", song_book, 0x00000000,
     0x000FF800, 0, 105, 263, 60, 1, -100, 0, 0, 0, "1d1", 40, 1}, /*{112} */
    {"& Ballads of Knowledge [Song Book II]", song_book, 0x00000000,
     0x7FF00000, 0, 305, 264, 60, 1, -100, 0, 0, 0, "1d1", 40, 1}, /*{113} */
    {"& Epics of the Bards [Greater Song Book]", song_book, 0x000001FF,
     0x00000000, 0, 950, 265, 60, 1, -100, 0, 0, 0, "0d0", 40, 1}, /*{114} */
    {"Lodging for the week", lodging_at_inn, 0, 0, 7, 200, 301, 3000, 1, 0,
     0, 0, 0, "0d0", 0, 1}, /*{115} */
    {"Lodging for three days", lodging_at_inn, 0, 0, 3, 120, 302, 3000, 1,
     0, 0, 0, 0, "0d0", 0, 1}, /*{116} */
    {"Chime of Light (%P1 charges)", chime, 0x00000000, 0x00000001, 0, 275,
     1, 1, 1, 0, 0, 0, 0, "0d0", 10, 1}, /*{117} */
    {"Chime of Detect Doors/Stairs (%P1 charges)", chime, 0x00000000,
     0x00000002, 0, 375, 2, 1, 1, 0, 0, 0, 0, "0d0", 15, 1}, /*{118} */
    {"Chime of Remove Curse (%P1 charges)", chime, 0x00000000, 0x00001000,
     0, 675, 13, 1, 1, 0, 0, 0, 0, "0d0", 47, 1}, /*{119} */
    {"Horn of Soft Sounds (%P1 charges)", horn, 0x00000000, 0x00040000, 0,
     600, 3, 20, 1, 0, 0, 0, 0, "0d0", 8, 1}, /*{120} */
    {"Horn of Tritons (%P1 charges)", horn, 0x00000000, 0x08000000, 0, 200,
     12, 20, 1, 0, 0, 0, 0, "0d0", 15, 1}, /*{121} */
    {"& Finely cut Agate~", valuable_gems, 0x00000000, 0x00000000, 0, 50,
     257, 5, 1, 0, 0, 0, 0, "0d0", 5, 1}, /*{122} */
    {"& Finely cut Diamond~", valuable_gems, 0x00000000, 0x00000000, 0,
     500, 258, 5, 1, 0, 0, 0, 0, "0d0", 10, 1}, /*{123} */
    {"& Rough cut Diamond~", valuable_gems, 0x00000000, 0x00000000, 0, 100,
     259, 5, 1, 0, 0, 0, 0, "0d0", 10, 1}, /*{124} */
    {"& Rough cut Sapphire~", valuable_gems, 0x00000000, 0x00000000, 0, 40,
     260, 5, 1, 0, 0, 0, 0, "0d0", 5, 1}, /*{125} */
    {"& Finely cut Sapphire~", valuable_gems, 0x00000000, 0x00000000, 0,
     250, 261, 5, 1, 0, 0, 0, 0, "0d0", 10, 1}, /*{126} */
    {"& Small pouch of Diamonds~", valuable_gems, 0x00000000, 0x00000000,
     0, 1000, 262, 5, 1, 0, 0, 0, 0, "0d0", 10, 1}, /*{127} */
    {"& Finely wrought gold necklace~", valuable_jewelry, 0x00000000,
     0x00000000, 0, 100, 40, 5, 1, 0, 0, 0, 0, "0d0", 10, 1}, /*{128} */
    {"& Small silver bracelet~", bracers, 0x00000000, 0x00000000, 0, 80,
     30, 5, 1, 0, 0, 0, 0, "0d0", 10, 1}, /*{129} */
    {"& Large mithril garter-belt~", valuable_jewelry, 0x00000000,
     0x00000000, 0, 1500, 265, 5, 1, 0, 0, 0, 0, "0d0", 20, 1}, /*{130} */
    {"& Small silver pendant~", valuable_jewelry, 0x00000000, 0x00000000,
     0, 60, 266, 5, 1, 0, 0, 0, 0, "0d0", 5, 1}, /*{131} */
    {"& Small gold pendant~", valuable_jewelry, 0x00000000, 0x00000000, 0,
     90, 267, 5, 1, 0, 0, 0, 0, "0d0", 10, 1}, /*{132} */
    {"& Small mithril pendant~", valuable_jewelry, 0x00000000, 0x00000000,
     0, 450, 268, 5, 1, 0, 0, 0, 0, "0d0", 15, 1}, /*{133} */
    {"& Finely cut Gem of Detect Monsters (%P1 charges)", valuable_gems, 
     nothing_flag, 0x00040000, 0, 350, 1, 5, 1, 0, 0, 0, 0, "0d0",
     10, 1}, /*{134} */
    {"& Finely cut Gem of Dispel Evil (%P1 charges)", valuable_gems, 
     nothing_flag, 0x00080000, 0, 1200, 2, 5, 1, 0, 0, 0, 0, "0d0",
     10, 1}, /*{135} */
    {"& Finely cut Gem of Acid Balls (%P1 charges)", valuable_gems, 
     nothing_flag, 0x00200000, 0, 1000, 4, 5, 1, 0, 0, 0, 0, "0d0",
     10, 1}, /*{136} */
    {"& Finely cut Gem of Detect Invisible (%P1 charges)", valuable_gems, 
     nothing_flag, 0x00400000, 0, 200, 5, 5, 1, 0, 0, 0, 0, "0d0",
     10, 1}, /*{137} */
    {"& Finely cut Gem of Identify (%P1 charges)", valuable_gems, 
     nothing_flag, 0x00800000, 0, 600, 6, 5, 1, 0, 0, 0, 0, "0d0",
     10, 1}, /*{138} */
    {"& Finely cut Gem of Light (%P1 charges)", valuable_gems, 
     nothing_flag, 0x01000000, 0, 100, 7, 5, 1, 0, 0, 0, 0, "0d0",
     10, 1}, /*{139} */
    {"& Finely cut Gem of Remove Curse (%P1 charges)", valuable_gems, 
     nothing_flag, 0x04000000, 0, 250, 8, 5, 1, 0, 0, 0, 0, "0d0",
     10, 1}, /*{140} */
    {"& Finely cut Gem of Annihilation (%P1 charges)", valuable_gems, 
     nothing_flag, 0x08000000, 0, 350, 7, 5, 1, 0, 0, 0, 0, "0d0",
     10, 1}, /*{141} */
    {"& Finely cut Gem of Recall (%P1 charges)", valuable_gems, 
     nothing_flag, 0x10000000, 0, 1200, 7, 5, 1, 0, 0, 0, 0, "0d0",
     10, 1}, /*{142} */
    {"& Box~ of Piranha Crackers", junk_food, 0x00000001, 0x40000000, 1500,
     4, 257, 2, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{143} */
    {"& Can~ of Orca-Cola", junk_food, 0x00000002, 0x40000000, 500, 4, 258,
     2, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{144} */
    {"& Twelve-Pound Troll Burger~", junk_food, 0x00000001, 0x40000000,
     7500, 15, 259, 2, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{145} */
    {"& Bag~ of Brontosaurus Chips", junk_food, 0x00000001, 0x40000000,
     3000, 12, 260, 2, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{146} */
    {"& Slice~ of Purple Mushroom Pizza", junk_food, 0x00000001,
     0x40000400, 1500, 8, 261, 2, 1, 0, 0, 0, 0, "2d6", 0, 1}, /*{147} */
    /*      { This ought to surprise them, Pizza = Oil now } */
    {"& Peanut Butter and Grape Jelly Sandwich~", junk_food, 0x00000001,
     0x40000000, 1000, 5, 262, 2, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{148} */
    {"& Dragon Steak~", junk_food, 0x00000001, 0x50000000, 5000, 15, 263,
     2, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{149} */
    {"& Vorpal Bunny Throat Lozenge~", junk_food, 0x00000001, 0x40000000,
     50, 2, 264, 2, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{150} */
    {"& Deep-Fried Giant Centipede~", junk_food, 0x00000001, 0x40000000,
     750, 5, 265, 2, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{151} */
    {"& Pint~ of Beetle Juice", junk_food, 0x00000002, 0x40000000, 1000, 4,
     266, 2, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{152} */
    {"& Bowl~ of Bat Stew", junk_food, 0x00000001, 0x40000000, 2000, 6,
     267, 2, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{153} */
    {"& Jar~ of Pickled Leeches", junk_food, 0x00000001, 0x40000000, 1500,
     5, 268, 2, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{154} */
    {"& Pack~ of Kitten McNuggets", junk_food, 0x00000001, 0x40000000,
     1500, 8, 269, 2, 1, 0, 0, 0, 0, "0d0", 0, 1}, /*{155} */
    {"& Iron Shod Quarterstaff^ (%P2,%P3)", maul, 0x00000000,
     Nothing_flag, 0, 25, 13, 100, 1, 0, 0, 0, 0, "1d5", 0, 1}, /*{156} */
    {"Room and board for one day", lodging_at_inn, 0x00000000,
     Nothing_flag, 1, 70, 303, 3000, 14, 0, 0, 0, 0, "0d0", 0, 1} /*{157} */

};

treasure_type blank_treasure = {" ", 0, 0, 0, 0, 0,   0,
				0,   0, 0,   0, 0, 0, " ", 0, 0};
long inven_ctr = 0;    /* { Total different obj's} */
long inven_weight = 0; /* { Cur carried weight	} */
long equip_ctr = 0;    /* { Cur equipment ctr	} */
long tcptr;	    /* { Cur treasure heap ptr} */

/*	{ Following are variables that changed with level of difficulty	} */
/*	{ 1/x chance of treasure per magma		} */
const long dun_str_mc = 95;
/*	{ 1/x chance of treasure per quartz		} */
const long dun_str_qc = 55;
/*	{ Level/x chance of unusual room		} */
const long dun_unusual = 100;
/*	{ Amount of objects for rooms			} */
const long treas_room_alloc = 7;
/*	{ Amount of objects for corridors		} */
const long treas_any_alloc = 2;
/*	{ Amount of gold (and gems)			} */
const long treas_gold_alloc = 2;
/*	{ 1/n Chance of item being a Great Item 	} */
const long obj_great = 30;
/*	{ Adjust STD per level				} */
const float obj_std_adj = 1.25;
/*	{ Minimum STD					} */
const long obj_std_min = 7;
/*	{ Town object generation level			} */
const long obj_town_level = 7;
/*	{ Base amount of magic				} */
const long obj_base_magic = 12;
/*	{ Max amount of magic				} */
const long obj_base_max = 100;
/*	{ magic_chance/# = special magic		} */
const long obj_div_special = 11;
/*	{ magic_chance/# = cursed items			} */
const float obj_div_cursed = 1.2;
/*	{ High value slows multiplication		} */
const long mon_mult_adj = 7;
/*	{ Dun_level/x chance of high level creature	} */
const long mon_nasty = 50;
/* */
/*	{ Following are feature objects defined for dungeon		} */
/* */

/*	{ Traps are just Nasty treasures...				} */
treasure_type trap_lista[MAX_TRAPA + 1] = {
    {"bogus trap a", seen_trap, 0x00000000, 0x00000000, 0, 0, 1, 0, 0, 0,
     0, 0, 0, "2d6", -50, 0},
    {"an open pit", seen_trap, 0x00000000, 0x00000000, 0, 0, 1, 0, 0, 0, 0,
     0, 0, "2d6", -50, 0},
    {"an arrow trap", unseen_trap, 0x00000000, 0x00000000, 0, 0, 2, 0, 0,
     0, 0, 0, 0, "1d8", 0, 0},
    {"a covered pit", unseen_trap, 0x00000000, 0x00000000, 0, 0, 3, 0, 0,
     0, 0, 0, 0, "2d6", 0, 0},
    {"a trap door", unseen_trap, 0x00000000, 0x00000000, 0, 0, 4, 0, 0, 0,
     0, 0, 0, "2d8", 0, 0},
    {"a gas trap", unseen_trap, 0x00000000, 0x00000000, 0, 0, 5, 0, 0, 0,
     0, 0, 0, "1d4", 0, 0},
    {"a loose rock", unseen_trap, 0x00000000, 0x00000000, 0, 0, 6, 0, 0, 0,
     0, 0, 0, "0d0", 0, 0},
    {"a dart trap", unseen_trap, 0x00000000, 0x00000000, 0, 0, 7, 0, 0, 0,
     0, 0, 0, "1d4", 0, 0},
    {"a strange rune", unseen_trap, 0x00000000, 0x00000000, 0, 0, 8, 0, 0,
     0, 0, 0, 0, "0d0", 0, 0},
    {"some loose rock", unseen_trap, 0x00000000, 0x00000000, 0, 0, 9, 0, 0,
     0, 0, 0, 0, "2d6", 0, 0},
    {"a gas trap", unseen_trap, 0x00000000, 0x00000000, 0, 0, 10, 0, 0, 0,
     0, 0, 0, "1d4", 0, 0},
    {"a strange rune", unseen_trap, 0x00000000, 0x00000000, 0, 0, 11, 0, 0,
     0, 0, 0, 0, "0d0", 0, 0},
    {"a blackened spot", unseen_trap, 0x00000000, 0x00000000, 0, 0, 12, 0,
     0, 0, 0, 0, 0, "4d6", 0, 0},
    {"some corroded rock", unseen_trap, 0x00000000, 0x00000000, 0, 0, 13,
     0, 0, 0, 0, 0, 0, "4d6", 0, 0},
    {"a gas trap", unseen_trap, 0x00000000, 0x00000000, 0, 0, 14, 0, 0, 0,
     0, 0, 0, "2d6", 0, 0},
    {"a gas trap", unseen_trap, 0x00000000, 0x00000000, 5, 0, 15, 0, 0, 0,
     0, 0, 0, "1d4", 10, 0},
    {"a gas trap", unseen_trap, 0x00000000, 0x00000000, 5, 0, 16, 0, 0, 0,
     0, 0, 0, "1d8", 5, 0},
    {"a dart trap", unseen_trap, 0x00000000, 0x00000000, 5, 0, 17, 0, 0, 0,
     0, 0, 0, "1d8", 10, 0},
    {"a dart trap", unseen_trap, 0x00000000, 0x00000000, 5, 0, 18, 0, 0, 0,
     0, 0, 0, "1d8", 10, 0},
    {"a chute", unseen_trap, 0x00000000, 0x00000000, 5, 0, 20, 0, 0, 0, 0,
     0, 0, "4d8", 20, 0}};

/*	{ Traps: Level represents the difficulty of disarming;	} */
/*	{ and P1 represents the experienced gained when disarmed} */
treasure_type trap_listb[MAX_TRAPB + 1] = {
    {"bogus trap b", seen_trap, 0x00000000, 0x00000000, 0, 0, 1, 0, 0, 0,
     0, 0, 0, "2d6", -50, 0},
    {"an open pit", seen_trap, 0x00000000, 0x00000000, 1, 0, 1, 0, 0, 0, 0,
     0, 0, "2d6", -50, 0},
    {"an arrow trap", seen_trap, 0x00000000, 0x00000000, 3, 0, 2, 0, 0, 0,
     0, 0, 0, "1d8", -10, 0},
    {"a covered pit", seen_trap, 0x00000000, 0x00000000, 2, 0, 3, 0, 0, 0,
     0, 0, 0, "2d6", -40, 0},
    {"a trap door", seen_trap, 0x00000000, 0x00000000, 5, 0, 4, 0, 0, 0, 0,
     0, 0, "2d8", -25, 0},
    {"a gas trap", seen_trap, 0x00000000, 0x00000000, 3, 0, 5, 0, 0, 0, 0,
     0, 0, "1d4", 5, 0},
    {"a loose rock", seen_trap, 0x00000000, 0x00000000, 0, 0, 6, 0, 0, 0,
     0, 0, 0, "0d0", -90, 0},
    {"a dart trap", seen_trap, 0x00000000, 0x00000000, 5, 0, 7, 0, 0, 0, 0,
     0, 0, "1d4", 10, 0},
    {"a strange rune", seen_trap, 0x00000000, 0x00000000, 5, 0, 8, 0, 0, 0,
     0, 0, 0, "0d0", -10, 0},
    {"some loose rock", seen_trap, 0x00000000, 0x00000000, 5, 0, 9, 0, 0,
     0, 0, 0, 0, "2d6", -10, 0},
    {"a gas trap", seen_trap, 0x00000000, 0x00000000, 10, 0, 10, 0, 0, 0,
     0, 0, 0, "1d4", 5, 0},
    {"a strange rune", seen_trap, 0x00000000, 0x00000000, 5, 0, 11, 0, 0,
     0, 0, 0, 0, "0d0", -10, 0},
    {"a blackened spot", seen_trap, 0x00000000, 0x00000000, 10, 0, 12, 0,
     0, 0, 0, 0, 0, "4d6", 10, 0},
    {"some corroded rock", seen_trap, 0x00000000, 0x00000000, 10, 0, 13, 0,
     0, 0, 0, 0, 0, "4d6", 10, 0},
    {"a gas trap", seen_trap, 0x00000000, 0x00000000, 5, 0, 14, 0, 0, 0, 0,
     0, 0, "2d6", 5, 0},
    {"a gas trap", seen_trap, 0x00000000, 0x00000000, 5, 0, 15, 0, 0, 0, 0,
     0, 0, "1d4", 10, 0},
    {"a gas trap", seen_trap, 0x00000000, 0x00000000, 5, 0, 16, 0, 0, 0, 0,
     0, 0, "1d8", 5, 0},
    {"a dart trap", seen_trap, 0x00000000, 0x00000000, 5, 0, 17, 0, 0, 0,
     0, 0, 0, "1d8", 10, 0},
    {"a dart trap", seen_trap, 0x00000000, 0x00000000, 5, 0, 18, 0, 0, 0,
     0, 0, 0, "1d8", 10, 0},
    /*	{ Special case, see DOOR_LIST below (subvals must agree)	} */
    {"a closed door", closed_door, 0x00000000, 0x00000000, 0, 0, 19, 0, 0,
     0, 0, 0, 0, "1d1", 0, 0},
    {"a chute", seen_trap, 0x00000000, 0x00000000, 5, 0, 20, 0, 0, 0, 0, 0,
     0, "4d8", 20, 0}

};

treasure_type scare_monster = /* { Special trap	} */
    {"a strange rune", seen_trap, 0x00000000, 0x00000000, 0, 0,     99,
     0,		       0,	 0,   0,	  0,	  0, "0d0", -90, 0};

treasure_type some_rubble = {
    "some rubble", rubble, 0x00000000, 0x00000000, 0, 0,     1,
    0,		   0,      0,   0,	  0,		0, "0d0", 0, 0};

/*	{ Secret door must have same subval as closed door in	} */
/*	{ TRAP_LISTB.  See CHANGE_TRAP				} */
treasure_type door_list[3] = {
    {"an open door", open_door, 0x00000000, 0x00000000, 0, 0, 1, 0, 0, 0,
     0, 0, 0, "1d1", 0, 0},
    {"a closed door", closed_door, 0x00000000, 0x00000000, 0, 0, 19, 0, 0,
     0, 0, 0, 0, "1d1", 0, 0},
    {"a secret door", secret_door, 0x00000000, 0x00000000, 0, 0, 19, 0, 0,
     0, 0, 0, 0, "1d1", 0, 0}};

treasure_type up_stair = {
    "an up staircase", up_staircase, 0x00000000, 0x00000000, 0, 0,     1,
    0,		       0,	    0,   0,	  0,	  0, "1d1", 0, 0};

treasure_type down_stair = {
    "a down staircase", down_staircase, 0x00000000, 0x00000000, 0,
    0,			1,		0,     0,	  0,	  0,
    0,			0,		"1d1", 0, 0};

treasure_type up_steep = {
    "a steep staircase", up_steep_staircase, 0x00000000, 0x00000000, 0,
    0,			 1,		     0,     0,		0,	  0,
    0,			 0,		     "1d1", 0, 0};

treasure_type down_steep = {
    "a steep staircase", down_steep_staircase, 0x00000000, 0x00000000, 0,
    0,			 1,		       0,     0,	  0,	  0,
    0,			 0,		       "1d1", 0, 0};

/*	{ Following are creature arrays and variables			} */
creature_type c_list[MAX_CREATURES + 1];
monster_type m_list[MAX_MALLOC + 1];
long m_level[MAX_MONS_LEVEL + 1];

monster_type blank_monster = /* { Blank monster values	} */
    {0, 0, 0, 0, 0, 0, 0, 0, 0, false, false, false};

long muptr;	/* { Cur used monster ptr	} */
long mfptr;	/* { Cur free monster ptr	} */
long mon_tot_mult; /* { # of repro's of creature	} */

/*	{ Following are arrays for descriptive pieces			} */
char const *colors[MAX_COLORS] = {
    "Amber",	   "Azure",		  "Blue",	    "Blue Speckled",
    "Blue Spotted",    "Black",		  "Black Speckled",  "Black Spotted",
    "Brown",	   "Brown Speckled",  "Brown Spotted",   "Bubbling",
    "Chartreuse",      "Clear",		  "Cloudy",	  "Copper",
    "Copper Spotted",  "Crimson",	 "Cyan",	    "Dark Blue",
    "Dark Green",      "Dark Red",	"Ecru",	    "Gold",
    "Gold Spotted",    "Green",		  "Green Speckled",  "Green Spotted",
    "Grey",	    "Grey Spotted",    "Hazy",	    "Indigo",
    "Light Blue",      "Light Green",     "Magenta",	 "Metallic Blue",
    "Metallic Red",    "Metallic Green",  "Metallic Purple", "Misty",
    "Orange",	  "Orange Speckled", "Orange Spotted",  "Pink",
    "Pink Speckled",   "Plaid",		  "Puce",	    "Purple",
    "Purple Speckled", "Purple Spotted",  "Red",	     "Red Speckled",
    "Red Spotted",     "Silver",	  "Silver Speckled", "Silver Spotted",
    "Smokey",	  "Tan",		  "Tangerine",       "Topaz",
    "Turquoise",       "Violet",	  "Vermilion",       "White",
    "White Speckled",  "White Spotted",   "Yellow",	  "Daggy"};

char const *mushrooms[MAX_MUSH] = {
    "Blue",	"Black",     "Brown",    "Copper",  "Crimson", "Dark blue",
    "Dark green",  "Dark red",  "Gold",     "Green",   "Grey",    "Light Blue",
    "Light Green", "Orange",    "Pink",     "Plaid",   "Purple",  "Red",
    "Tan",	 "Turquoise", "Violet",   "White",   "Yellow",  "Wrinkled",
    "Wooden",      "Slimey",    "Speckled", "Spotted", "Furry"};

char const *woods[MAX_WOODS] = {
    "Applewood",  "Ashen",      "Aspen",     "Avocado wood", "Balsa",
    "Banyan",     "Birch",      "Cedar",     "Cherrywood",   "Cinnibar",
    "Cottonwood", "Cypress",    "Dogwood",   "Driftwood",    "Ebony",
    "Elm wood",   "Eucalyptus", "Grapevine", "Hawthorn",     "Hemlock",
    "Hickory",    "Iron wood",  "Juniper",   "Locust",       "Mahogany",
    "Magnolia",   "Manzanita",  "Maple",     "Mulberry",     "Oak",
    "Pecan",      "Persimmon",  "Pine",      "Redwood",      "Rosewood",
    "Spruce",     "Sumac",      "Sycamore",  "Teak",	 "Walnut",
    "Zebra wood"};

char const *metals[MAX_METALS] = {
    "Aluminium",     "Bone",	"Brass",		"Bronze",
    "Cast Iron",     "Chromium",    "Copper",		"Gold",
    "Iron",	  "Lead",	"Magnesium",	"Molybdenum",
    "Nickel",	"Pewter",      "Rusty",		"Silver",
    "Steel",	 "Tin",	 "Titanium",		"Tungsten",
    "Zirconium",     "Zinc",	"Aluminium Plated", "Brass Plated",
    "Copper Plated", "Gold Plated", "Nickel Plated",    "Silver Plated",
    "Steel Plated",  "Tin Plated",  "Zinc Plated",      "Uranium"};

char const *horns[MAX_HORNS] = {"Bag Pipes", "Bugle", "Conch Shell", "Fife",
				"Harmonica", "Horn",  "Picolo",      "Pipes",
				"Recorder",  "Reed",  "Trumpet",     "Tuba",
				"Whistle"};

char const *rocks[MAX_ROCKS] = {
    "Amber",      "Agate",     "Alexandrite", "Amethyst",     "Antlerite",
    "Aquamarine", "Argentite", "Azurite",     "Beryl",	"Bloodstone",
    "Calcite",    "Carnelian", "Coral",       "Corundum",     "Cryolite",
    "Diamond",    "Diorite",   "Emerald",     "Flint",	"Fluorite",
    "Gabbro",     "Garnet",    "Granite",     "Gypsum",       "Hematite",
    "Jade",       "Jasper",    "Kryptonite",  "Lapus lazuli", "Limestone",
    "Malachite",  "Manganite", "Marble",      "Mica",	 "Moonstone",
    "Neptunite",  "Obsidian",  "Onyx",	"Opal",	 "Pearl",
    "Pyrite",     "Quartz",    "Quartzite",   "Rhodonite",    "Rhyolite",
    "Ruby",       "Sapphire",  "Sphalerite",  "Staurolite",   "Tiger eye",
    "Topaz",      "Turquoise", "Zircon"};

char const *amulets[MAX_AMULETS] = {
    "Birch",     "Cedar",    "Dogwood",   "Driftwood", "Elm wood", "Hemlock",
    "Hickory",   "Mahogany", "Maple",     "Oak",       "Pine",     "Redwood",
    "Rosewood",  "Walnut",   "Aluminium", "Bone",      "Brass",    "Bronze",
    "Copper",    "Iron",     "Lead",      "Nickel",    "Agate",    "Amethyst",
    "Diamond",   "Emerald",  "Flint",     "Garnet",    "Jade",     "Obsidian",
    "Onyx",      "Opal",     "Pearl",     "Quartz",    "Ruby",     "Sapphire",
    "Tiger eye", "Topaz",    "Turquoise"};

char const *cloths[MAX_CLOTHS] = {"Burlap",     "Cotton",     "Wool",
				  "Sack-cloth", "Rabbit-fur", "Lizard-skin",
				  "Goat-skin"};

char const *syllables[MAX_SYLLABLES] = {
    "a",    "ab",   "ag",   "aks",  "ala",  "an",   "ankh", "app",  "arg",
    "arze", "ash",  "aus",  "ban",  "bar",  "bat",  "bek",  "bie",  "bin",
    "bit",  "bjor", "blu",  "brd",  "bu",   "byt",  "comp", "con",  "cos",
    "cre",  "dalf", "dan",  "den",  "doe",  "dok",  "eep",  "el",   "eng",
    "er",   "ere",  "erk",  "esh",  "evs",  "fa",   "fid",  "for",  "fri",
    "fu",   "gan",  "gar",  "glen", "gop",  "gre",  "ha",   "he",   "hyd",
    "i",    "ing",  "ion",  "ip",   "ish",  "it",   "ite",  "iv",   "jo",
    "kho",  "kli",  "klis", "la",   "lech", "man",  "mar",  "me",   "mi",
    "mic",  "mik",  "mon",  "mung", "mur",  "naed", "neg",  "nep",  "ner",
    "nes",  "nis",  "nih",  "nin",  "o",    "od",   "ood",  "ook",  "oook",
    "org",  "orn",  "ox",   "oxy",  "pay",  "pet",  "ple",  "plu",  "po",
    "pot",  "prok", "re",   "rea",  "rhov", "ri",   "ro",   "rog",  "rok",
    "rol",  "sa",   "san",  "sat",  "see",  "sef",  "seh",  "shu",  "si",
    "snd",  "sne",  "snik", "sno",  "so",   "sol",  "spam", "sri",  "sta",
    "sun",  "ta",   "taf",  "tem",  "ther", "ti",   "tox",  "trol", "tue",
    "turs", "u",    "ulk",  "um",   "un",   "uni",  "ur",   "val",  "viv",
    "vly",  "vom",  "wah",  "wed",  "werg", "wex",  "whon", "wlf",  "x",
    "yerg", "yp",   "zun"};

/*	vowel_set		: */
/*				  char_set; */

/* new stuff */
long malloc_calls = 0;
long malloc_bytes = 0;
long free_calls = 0;
long free_bytes = 0;

char coin_name[MITHRIL + 1][82] = {"total", "iron",     "copper", "silver",
				"gold",  "platinum", "mithril"};

/* used in gc__fill_cave, not really objects but I don't care */
obj_set blank_floor_set = {0, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};

/* used in get_flags, and other places if I needed them */
obj_set null_obj_set = {0, 0};

obj_set destroyed_by_lightning = {ring, rod, wand, 0};

obj_set destroyed_by_acid = {
    arrow,      bow_crossbow_or_sling, hafted_weapon, pole_arm, gem_helm,
    boots,      gloves_and_gauntlets,  cloak,	 helm,     shield,
    hard_armor, soft_armor,	    staff,	 scroll1,  scroll2,
    Food,       open_door,	     closed_door,   0};

obj_set destroyed_by_cold = {potion1, potion2, 0};

obj_set destroyed_by_fire = {
    arrow,   bow_crossbow_or_sling, hafted_weapon, pole_arm,
    boots,   gloves_and_gauntlets,  cloak,	 soft_armor,
    staff,   scroll1,		    scroll2,       potion1,
    potion2, Food,		    open_door,     closed_door,
    0};

obj_set destroyed_by_petrify = {boots, soft_armor, potion1, potion2, Food, 0};

obj_set destroyed_by_sunray = {cloak, scroll1, scroll2, potion1, potion2, 0};

gid_t games_gid;

boolean scoresAreEncrypted = true;
boolean saveFilesAreEncrypted = true;

unsigned char highScoreKey[8] = {1, 2, 3, 4, 5, 6, 7, 8};
unsigned char saveFileKey[8] = {8, 7, 6, 5, 4, 3, 2, 1};

int game_state;
boolean curses_is_running = false;
