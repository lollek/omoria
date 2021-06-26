#ifndef TYPES_H
#define TYPES_H

#include <stdint.h>

#include "boolean.h"
#include "constants.h"

typedef uint8_t obj_set[MAX_OBJ_SET];    /* = set of 0..255; */
typedef int64_t money_type[MITHRIL + 1]; /* = array[total$..mithril] of long; */

#define STAT_T_SIZE 6 /* type stat_set is supposed to be 0..5 */
enum stat_t { STR = 0, INT = 1, WIS = 2, DEX = 3, CON = 4, CHR = 5 };

enum spell_class_t {
  SC_NULL = 0,
  SC_HOLD = 1,
  SC_MENTAL = 2 // SE_CONFUSE, SE_SLEEP, SE_JOKE
};

enum spell_effect_t {
  SE_NULL = 0,      // Nothing, or "normal" type
  SE_LIGHTNING = 1, // Lightning damage
  SE_GAS = 2,       // Poisonous gas damage
  SE_ACID = 3,      // Acid damage
  SE_COLD = 4,      // Cold damage
  SE_FIRE = 5,      // Fire damage
  SE_GOOD = 6,      // Holy damage
  SE_EVIL = 7,      // Marker for evil creatures, also xp loss damage type
  SE_PETRIFY = 8,
  SE_SUNRAY = 9,
  SE_ILLUSION = 10, // Illusion damage
  SE_PROBE = 11,    // Some kind of ray?
  SE_SLEEP = 12,    // Sleep monster
  SE_CONFUSE = 13,  // Confuses monster
  SE_HP = 14,       // Pure HP damage/healing?
  SE_DRAIN = 15,    // Drain life
  SE_SPEED = 16,    // Slow / speed up monster (and player?)
  SE_HOLD = 17,     // Hold monster
  SE_TURN = 18,     // Turn undead
  SE_POLY = 19,
  SE_JOKE = 20,      // Joke damage (really)
  SE_MONSTER = 21,   // Marker for monster creatures
  SE_INVISIBLE = 22, // Marker for invisible creatures
  SE_CREATURE = 23,  // Marker for enemies?
  SE_OBJECT = 24,    // Marker for object
  SE_TREASURE = 25,  // Marker for treasure
  SE_HP_UNDEAD = 26,
  SE_THUNDER = 27, // Causes confusion?
  SE_HOLY_WORD = 28
};

enum class_t {
  C_WARRIOR = 0,
  C_MAGE = 1,
  C_PRIEST = 2,
  C_ROGUE = 3,
  C_RANGER = 4,
  C_PALADIN = 5,
  C_DRUID = 6,
  C_BARD = 7,
  C_ADVENTURER = 8,
  C_MONK = 9,
  C_BARBARIAN = 10
};

enum store_t {
  S_GENERAL = 0, /* 'the entrance to the General Store' */
  S_ARMORY = 1,  /* 'the entrance to the Armory' */
  S_WEAPONS = 2, /* 'the entrance to the Weapon Smiths' */
  S_TEMPLE = 3,  /* 'the entrance to the Temple' */
  S_ALCHEMY = 4, /* 'the entrance to the Alchemy Shop' */
  S_MAGIC = 5,   /* 'the entrance to the Magic Shop' */
  S_INN = 6,     /*,'the entrance to the Inn' */
  S_LIBRARY = 7, /* 'the entrance to the Library' */
  S_MUSIC = 8,   /* 'the entrance to the Music Shop' */
  S_GEM = 9,     /*,'the entrance to the Gem Store' */
  S_DELI = 10,   /* 'the entrance to the All-Nite Deli' */
  S_BLACK_MARKET = 11,
  S_TRADE_POST = 12, /* 'the entrance to the Trading Post' */
  S_INSURANCE = 13,  /* 'the entrance to the Insurance Shop' */
  S_BANK = 14,       /* 'the entrance to the Bank' */
  S_CHANGER = 15,    /* 'the entrance to the Money Exchange' */
  S_CASINO = 16,     /* 'the entrance to the Casino' */
  S_FORTRESS = 17,
  S_GENERIC_1 = 18, /* 'the entrance to a building' */
  S_GENERIC_2 = 19, /* 'the entrance to a building' */
  S_GENERIC_3 = 20, /* 'the entrance to a building' */
  S_GENERIC_4 = 21, /* 'the entrance to a building' */
  S_GENERIC_5 = 22, /* 'the entrance to a building' */
};

typedef struct game_time_type {
  int64_t year;
  uint8_t month;
  uint8_t day;
  uint8_t hour;
  uint16_t secs;
} game_time_type;

typedef struct time_type {
  uint16_t years;
  uint16_t months;
  uint16_t days;
  uint16_t hours;
  uint16_t minutes;
  uint16_t seconds;
  uint16_t hundredths;
} time_type;

typedef struct creature_type {
  uint8_t aaf;       /*: [bit(7),pos(0)] 0..127; Area affect radius */
  uint8_t ac;        /*: [bit(7),pos(8)] 0..127;	 AC */
  char name[28];     /* Descrip of creature */
  uint64_t cmove;    /*: unsigned;	Bit field */
  uint64_t spells;   /*: unsigned;	Creature spells	 */
  uint64_t cdefense; /*: unsigned;	Bit field */
  int16_t sleep;     /* Inactive counter */
  int64_t mexp;      /* Exp value for kill */
  int8_t speed;      /* Movement speed */
  char cchar;        /* Character rep. */
  char hd[7];        /* Creatures hit die */
  char damage[36];   /* Type attack and damage */
  int8_t level;      /* Level of creature */
  uint8_t mr;        /* Magic Resistance */
} creature_type;

typedef struct monster_type {
  int16_t hp;     /* Hit points		*/
  int16_t csleep; /* Inactive counter	*/
  int16_t cdis;   /* Cur dis from player	*/
  uint16_t mptr;  /* Pointer into creature	*/
  uint16_t nptr;  /* Pointer to next block	*/
  int8_t cspeed;  /* Movement speed	*/

  /* Note: FY and FX constrain dungeon size to 255 */
  uint8_t fy; /* Y Pointer into map	 */
  uint8_t fx; /* X Pointer into map	 */

  int8_t stunned;   /* [bit(6),pos(104)] -32..31; Rounds stunned */
  boolean ml;       /* [bit(1),pos(110)] boolean; On if shown   */
  boolean confused; /* [bit(1),pos(111)] boolean; On if confused */
  boolean moved;    /* [bit(1),pos(112)] boolean; On if water-moved */
} monster_type;

typedef struct treasure_type {
  char name[70];   /* Object name		*/
  uint8_t tval;    /* Catagory number	*/
  uint64_t flags2; /*: unsigned;	 { MORE Special flags	} */
  uint64_t flags;  /*: unsigned;	 { Special flags	} */
  int64_t p1;      /* { Misc. use variable	} */
  int64_t cost;    /* { Cost of item		} */
  int64_t subval;  /* { Sub-catagory number	} */
  uint16_t weight; /* { Weight in gp's	} */
  uint16_t number; /* { Number of items	} */
  int16_t tohit;   /* { Pluses to hit		} */
  int16_t todam;   /* { Pluses to damage	} */
  int16_t ac;      /* { Normal AC		} */
  int16_t toac;    /* { Pluses to AC		} */
  char damage[7];  /* { Damage when hits	}*/
  int8_t level;    /* { Level item found	} */
  uint8_t identified;
} treasure_type;

typedef struct treas_rec {
  treasure_type data;
  boolean ok;
  uint16_t insides;
  boolean is_in;
  struct treas_rec *next;
} treas_rec;

typedef treas_rec *treas_ptr;

typedef struct spell_t {
  char sname[28];
  uint8_t slevel;
  uint8_t smana;
  uint8_t sfail;
  boolean learned;
} spell_t;

typedef struct spl_rec {
  int64_t splnum;
  int64_t splchn;
} spl_rec;

typedef spl_rec spl_type[MAX_SPELLS]; /* array [1..max_spells] of spl_rec; */

typedef struct floor_type {
  uint8_t ftval;  /*: [bit(7),pos(0)] 0..127; */
  boolean ftopen; /*: [bit(1),pos(7)] boolean; */
} floor_type;

typedef struct cave_type {
  uint8_t cptr;  /*	: unsigned char; */
  uint8_t tptr;  /*	: unsigned char; */
  uint8_t fval;  /*	: [bit(7),pos(16)] 0..127; */
  boolean fopen; /*	: [bit(1),pos(23)] boolean; */
  boolean fm;    /*	: [bit(1),pos(24)] boolean; */
  boolean pl;    /*	: [bit(1),pos(25)] boolean; */
  boolean tl;    /*	: [bit(1),pos(26)] boolean; */
  boolean moved; /*	: [bit(1),pos(27)] boolean; */
  uint8_t oct;   /*	: [bit(3),pos(28)] 0..7; { octant direction } */
  uint8_t h2o;   /*	: [bit(4),pos(31)] 0..15; */
} cave_type;

typedef cave_type
    row_floor[MAX_WIDTH + 1]; /*= array [1..max_width] of cave_type; */

typedef struct owner_type {
  char owner_name[82];
  int64_t max_cost;
  float max_inflate;
  float min_inflate;
  float haggle_per;
  uint8_t owner_race;
  uint8_t insult_max;
} owner_type;

typedef struct inven_record {
  int64_t scost;
  treasure_type sitem;
} inven_record;

typedef struct store_type {
  game_time_type store_open;
  uint8_t owner;
  int8_t insult_cur;
  uint8_t store_ctr;
  inven_record store_inven[STORE_INVEN_MAX + 1];
  /* : array [1..store_inven_max] of inven_record; */
} store_type;

typedef struct message_record {
  char data[134];
  struct message_record *next;
} message_record;

typedef message_record *message_ptr;


/* needs to be a multiple of 8 */
#define ENCRYPT_STAT_BUF_SIZE 1024
typedef struct encrypt_state {
  uint8_t des_key[8];
  uint8_t des_ivec[8];
  boolean doit;

  boolean got_eof;  /* out of bytes and hit eof         */
  int32_t buf_pos;  /* for read/write, current position */
  int32_t buf_size; /* for reading, bytes in buffer     */
  char data_buf[ENCRYPT_STAT_BUF_SIZE + 8];
} encrypt_state;

#endif /* TYPES_H */
