#pragma once

/*	{ Dungeon size parameters					} */
#define MAX_HEIGHT 66 /* { Multiple of 11; >= 22 } */
#define MAX_WIDTH 198 /* { Multiple of 33; >= 66 } */

/*	{ Store constants						} */
#define MAX_OWNER_KIND 4 /*{ Number of different keepers for a shop} */
#define MAX_STORES 12    /*{ Number of different stores		} */
#define MAX_OWNERS (MAX_STORES * MAX_OWNER_KIND)
/*{ Number of owners to choose from	} */
#define MAX_UNNAMED 6       /*{ # of 'odd' shops (Post, etc)	} */
#define STORE_INVEN_MAX 24  /*{ Max number of discrete objs in inven	} */
#define STORE_CHOICES 50    /*{ Number of items to choice stock from	} */
#define STORE_MAX_INVEN 20  /*{ Max diff objs in stock before auto sell} */
#define STORE_MIN_INVEN 14  /*{ Min diff objs in stock before auto buy} */
#define STORE_TURN_AROUND 3 /*{ Amount of buying and selling normally } */
#define INVEN_INIT_MAX 157  /*{ Size of store init array		} */
#define COST_ADJ 240.00     /*{ FLOAT(Gold$value)--adjusts store costs } */
#define WEIGHT_ADJ 100      /*{ Adjust weights of items 		} */
#define QUEST_DELAY 800     /*{ # of turns to wait before char can get quest} */

/*	{ Treasure constants						} */
#define EQUIP_MAX 15      /*{ Size of equipment array		} */
#define MAX_OBJ_LEVEL 256 /*{ Maximum level of magic in dungeon	} */
#define MAX_OBJECTS 473   /*{ Number of objects for universe       } */
/* if MAX_OBJECTS goes over 1024 then save will break */
#define MAX_GOLD 25    /*{ Number of different types of gold	} */
#define MAX_TALLOC 225 /*{ Max objects per level		} */

/*	{ Money constants, for those of us who can't remember '12480'   } */
#define MITHRIL_VALUE 1248 /*{ Iron Pieces				} */
#define PLATINUM_VALUE 960
#define GOLD_VALUE 240
#define SILVER_VALUE 20
#define COPPER_VALUE 4
#define IRON_VALUE 1

/*	{ Creature contants						} */
#define MAX_MALLOC (100 + 1)  /*{ Max that can be allocated	        } */
#define MAX_MALLOC_CHANCE 160 /*{ 1/x chance of new monster each round	} */
#define MAX_SIGHT 20          /*{ Maximum dis a creature can be seen	} */
#define MAX_SPELL_DIS 18      /*{ Maximum dis creat. spell can be cast	} */
#define MAX_MON_MULT 75       /*{ Maximum reproductions on a level	} */
#define MIN_MALLOC_LEVEL 14   /*{ Minimum number of monsters/level	} */
#define MIN_MALLOC_TD 8       /*{ Number of people on town level (day)	} */
#define MIN_MALLOC_TN 12      /*{ Number of people on town level (night)} */
#define WIN_MON_TOT 2         /*{ Total number of "win" creatures	} */
#define WIN_MON_APPEAR 100    /*{ Level where winning creatures begin	} */
#define OUT_OF_ENV_DAM 32     /*{ max HP lost/rnd/2^move in wrong env	} */
#define MON_SUMMON_ADJ 2      /*{ Adjust level of summoned creatures	} */
#define MON_DRAIN_LIFE 5      /*{ Percent of player exp drained per hit } */

/*	{ Trap constants						} */
#define MAX_TRAPA 19 /*{ Number of defined traps		} */
#define MAX_TRAPB 20 /*{ Includes secret doors		} */

/*	{ Player constants						} */
#define MAX_SPELLS 40          /*{ Maximum number of spells per class	} */
#define MAX_PLAYER_LEVEL 40    /*{ Maximum possible character level	} */
#define USE_DEVICE 3           /*{ x> Harder devices x< Easier devices	} */
#define NUM_QUESTS 5           /*{ # of possible quests to select from	} */
#define BASE_FOOD_DIGESTED 2   /* food digested per turn */
#define PLAYER_WEIGHT_CAP 13       /*{ "#"*(1/10 pounds) per strength point	} */
#define PLAYER_EXIT_PAUSE 0        /*{ Pause time before player can re-roll	} */
#define MAX_HIGH_SCORES 100 /*{ Maximum number of high scores to keep } */
#define FEMALE 1
#define MALE 2

/*	{ Base to hit constants						} */
#define BTH_LEV_ADJ 3  /*{ Adjust BTH per level			} */
#define BTH_PLUS_ADJ 3 /*{ Adjust BTH per plus-to-hit		} */
#define BTH_HIT 12     /*{ Automatic hit; 1/bth_hit		} */

/*	{ Misc								} */
#define Nothing_flag 0x00000000
#define nothing_flag 0x00000000
#define Everything_flag 0xFFFFFFFF
#define everything_flag 0xFFFFFFFF

/*	{ Flags for items that can be worn or wielded		-DCJ-	} */
#define Strength_worn_bit 0x00000001
#define Dexterity_worn_bit 0x00000002
#define Constitution_worn_bit 0x00000004
#define Intelligence_worn_bit 0x00000008
#define Wisdom_worn_bit 0x00000010
#define Charisma_worn_bit 0x00000020
#define Searching_worn_bit 0x00000040
#define Slow_Digestion_worn_bit 0x00000080
#define Stealth_worn_bit 0x00000100
#define Aggravation_worn_bit 0x00000200
#define Teleportation_worn_bit 0x00000400
#define Regeneration_worn_bit 0x00000800
#define Speed_worn_bit 0x00001000
#define Slay_Dragon_worn_bit 0x00002000
#define Slay_Monster_worn_bit 0x00004000
#define Slay_Evil_worn_bit 0x00008000
#define Slay_Undead_worn_bit 0x00010000
#define Cold_Brand_worn_bit 0x00020000
#define Flame_Brand_worn_bit 0x00040000
#define Resist_Fire_worn_bit 0x00080000
#define Resist_Acid_worn_bit 0x00100000
#define Resist_Cold_worn_bit 0x00200000
#define Sustain_Stat_worn_bit 0x00400000
#define Free_Action_worn_bit 0x00800000
#define See_Invisible_worn_bit 0x01000000
#define Resist_Lightning_worn_bit 0x02000000
#define Feather_Fall_worn_bit 0x04000000
#define Blindness_worn_bit 0x08000000
#define Timidness_worn_bit 0x10000000
#define Tunneling_worn_bit 0x20000000
#define Infra_Vision_worn_bit 0x40000000
#define Cursed_worn_bit 0x80000000

/*	{ Flags for items that can be worn or wielded (flags2)		} */
#define Slay_demon_worn_bit 0x00000001
#define Soul_Sword_worn_bit 0x00000002
#define Slay_regen_worn_bit 0x00000004
#define Swimming_worn_bit 0x00000008
#define Magic_proof_worn_bit 0x00000010
#define Disarm_worn_bit 0x00000020
#define Sharp_worn_bit 0x00000040
/*{	Unused_bit				0x00000080		} */
/*{	Unused_bit_set				0x000FFF00		} */
/*{	Unused_bit				0x00100000		} */
#define Bad_repute_worn_bit 0x00200000
#define Hunger_worn_bit 0x00400000
#define Negative_gem_bit 0x00800000
#define Increase_carry_worn_bit 0x01000000
/*{	Unused_bit				0x02000000		} */
#define Holding_bit 0x04000000
#define Swallowing_bit 0x08000000
#define Sharp_bit 0x10000000
#define Blackmarket_bit 0x20000000
#define Insured_bit 0x40000000
#define Known_cursed_bit 0x80000000

/*	{ Constants for equipment inventory items		-KRC-	} */

#define Equipment_none -1
#define Equipment_min 0
#define Equipment_primary (Equipment_min)              /*  0 */
#define Equipment_helm (Equipment_primary + 1)         /*  1 */
#define Equipment_amulet (Equipment_helm + 1)          /*  2 */
#define Equipment_armor (Equipment_amulet + 1)         /*  3 */
#define Equipment_belt (Equipment_armor + 1)           /*  4 */
#define Equipment_shield (Equipment_belt + 1)          /*  5 */
#define Equipment_gloves (Equipment_shield + 1)        /*  6 */
#define Equipment_bracers (Equipment_gloves + 1)       /*  7 */
#define Equipment_right_ring (Equipment_bracers + 1)   /*  8 */
#define Equipment_left_ring (Equipment_right_ring + 1) /*  9 */
#define Equipment_boots (Equipment_left_ring + 1)      /* 10 */
#define Equipment_cloak (Equipment_boots + 1)          /* 11 */
#define Equipment_light (Equipment_cloak + 1)          /* 12 */
#define Equipment_secondary (Equipment_light + 1)      /* 13 */

/* .tval constants -DCJ- */
#define miscellaneous_object 1
#define chest 2
#define misc_usable 3
#define valuable_jewelry 4
#define valuable_gems 5
#define bag_or_sack 6
#define valuable_gems_wear 7
#define sling_ammo 10
#define bolt 11
#define arrow 12
#define spike 13
#define lamp_or_torch 15
#define bow_crossbow_or_sling 20
#define hafted_weapon 21
#define pole_arm 22
#define dagger 23
#define sword 24
#define pick_or_shovel 25
#define maul 26
#define gem_helm 29
#define boots 30
#define gloves_and_gauntlets 31
#define cloak 32
#define helm 33
#define shield 34
#define hard_armor 35
#define soft_armor 36
#define bracers 37
#define belt 38
#define amulet 40
#define ring 45
#define staff 55
#define rod 60
#define wand 65
#define scroll1 70
#define scroll2 71
#define potion1 75
#define potion2 76
#define flask_of_oil 77
#define Food 80
#define junk_food 81
#define chime 85
#define horn 86
#define magic_book 90
#define prayer_book 91
#define instrument 92 /* Not in use */
#define song_book 93
#define lodging_at_inn 95
#define valuable_metal 100 /* look in detect_item for limit */
#define unseen_trap 101
#define seen_trap 102
#define rubble 103
#define open_door 104
#define closed_door 105
#define up_staircase 107
#define down_staircase 108
#define secret_door 109
#define entrance_to_store 110
#define up_steep_staircase 111
#define down_steep_staircase 112
#define whirlpool 113

/* fields in py.flags.status */
#define IS_HUNGERY 0x00000001
#define IS_WEAK 0x00000002
#define IS_BLIND 0x00000004
#define IS_CONFUSED 0x00000008
#define IS_AFRAID 0x00000010
#define IS_POISONED 0x00000020
#define IS_HASTED 0x00000040
#define IS_SLOW 0x00000080
#define IS_SEARCHING 0x00000100
#define IS_RESTING 0x00000200
#define IS_aaa 0x00000400
#define IS_bbb 0x00000800
#define IS_INVULNERABLE 0x00001000
#define IS_HERO 0x00002000
#define IS_SUPER_HERO 0x00004000
#define IS_BLESSED 0x00008000
#define IS_ABLE_TO_SEE_INVIS 0x00010000
#define IS_ABLE_TO_SEE_HEAT 0x00020000
#define IS_ccc 0x00040000
#define IS_ddd 0x00080000
#define IS_eee 0x00100000
#define IS_fff 0x00200000
#define IS_ggg 0x00400000
#define IS_CHARM_PROOF 0x00800000
#define IS_hhh 0x01000000
#define IS_iii 0x02000000
#define IS_jjj 0x04000000
#define IS_kkk 0x08000000
#define IS_lll 0x10000000
#define IS_STEALTHY 0x20000000
#define IS_MAGIC_PROTECED 0x40000000
#define IS_mmm 0x80000000

#define TOTAL_ 0
#define IRON 1
#define COPPER 2
#define SILVER 3
#define GOLD 4
#define PLATINUM 5
#define MITHRIL 6

#define COIN_WEIGHT 5

#define CTRL_A 1
#define CTRL_B 2
#define CTRL_C 3
#define CTRL_D 4
#define CTRL_E 5
#define CTRL_F 6
#define CTRL_G 7
#define CTRL_H 8
#define CTRL_I 9
#define CTRL_J 10
#define CTRL_K 11
#define CTRL_L 12
#define CTRL_M 13
#define CTRL_N 14
#define CTRL_O 15
#define CTRL_P 16
#define CTRL_Q 17
#define CTRL_R 18
#define CTRL_S 19
#define CTRL_T 20
#define CTRL_U 21
#define CTRL_V 22
#define CTRL_W 23
#define CTRL_X 24
#define CTRL_Y 25
#define CTRL_Z 26

#define OBJ_SET_MAX 255 /* type obj_set is supposed to be 0..255 */
#define MAX_OBJ_SET 25  /* max items allowed in an obj_set[] */

#define PLACE_OBJECT_TRIES 3 /* pick best object out of this many */

/* return values for master_file_verify */
#define MF_CHAR_OK 0
#define MF_CHAR_NOT_FOUND 1
#define MF_CHAR_MISMATCH 2

/* values for game_state */
#define GS_IGNORE_CTRL_C 0
#define GS_ALLOW_CTRL_C 1
#define GS_LOADING 2
#define GS_GET_COMMAND 3
#define GS_HELP 4

const char *imoria_version(void);
const char *omoria_version(void);
