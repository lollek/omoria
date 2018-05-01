#ifndef TYPES_H
#define TYPES_H
/* The global types for imoria */

typedef uint8_t boolean;

typedef struct quad_type
{
	uint16_t l0;
	uint16_t l1;
} quad_type;

typedef char ctype[26 + 2];
typedef char dtype[5 + 2];
typedef char etype[34 + 2];
typedef char htype[12 + 2];
typedef char string[132 + 2];
typedef char mtype[190 + 2];
typedef char ntype[1024 + 2];
typedef char ttype[68 + 2];
typedef char vtype[80 + 2];
typedef char account_type[10 + 2];
typedef uint8_t obj_set[MAX_OBJ_SET]; /* = set of 0..255; */
typedef char stat_set;			    /* = 0..5; */
typedef uint8_t
    stat_s_type[STAT_SET_MAX + 1];    /* = array [stat_set] of unsigned char; */
typedef int64_t money_type[MITHRIL + 1]; /* = array[total$..mithril] of long; */

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
	C_MONK = 9
};

typedef struct game_time_type
{
	int64_t year;
	uint8_t month;
	uint8_t day;
	uint8_t hour;
	uint16_t secs;
} game_time_type;

typedef struct time_type
{
	uint16_t years;
	uint16_t months;
	uint16_t days;
	uint16_t hours;
	uint16_t minutes;
	uint16_t seconds;
	uint16_t hundredths;
} time_type;

typedef struct creature_type
{
	uint8_t aaf;    /*: [bit(7),pos(0)] 0..127; Area affect radius */
	uint8_t ac;     /*: [bit(7),pos(8)] 0..127;	 AC */
	ctype name;	   /* Descrip of creature */
	uint64_t cmove;  /*: unsigned;	Bit field */
	uint64_t spells; /*: unsigned;	Creature spells	 */
	uint64_t cdefense; /*: unsigned;	Bit field */
	int16_t sleep;		/* Inactive counter */
	int64_t mexp;		/* Exp value for kill */
	int8_t speed;      /* Movement speed */
	char cchar;		/* Character rep. */
	dtype hd;		/* Creatures hit die */
	etype damage;		/* Type attack and damage */
	int8_t level;      /* Level of creature */
	uint8_t mr;       /* Magic Resistance */
} creature_type;

typedef struct monster_type
{
	int16_t hp;	    /* Hit points		*/
	int16_t csleep;	/* Inactive counter	*/
	int16_t cdis;	  /* Cur dis from player	*/
	uint16_t mptr; /* Pointer into creature	*/
	uint16_t nptr; /* Pointer to next block	*/
	int8_t cspeed;  /* Movement speed	*/

	/* Note: FY and FX constrain dungeon size to 255 */
	uint8_t fy; /* Y Pointer into map	 */
	uint8_t fx; /* X Pointer into map	 */

	int8_t stunned; /* [bit(6),pos(104)] -32..31; Rounds stunned */
	boolean ml;	  /* [bit(1),pos(110)] boolean; On if shown   */
	boolean confused;    /* [bit(1),pos(111)] boolean; On if confused */
	boolean moved;       /* [bit(1),pos(112)] boolean; On if water-moved */
} monster_type;

typedef struct treasure_type
{
	ttype name;	    /* Object name		*/
	uint8_t tval;    /* Catagory number	*/
	chtype tchar;	  /* Character representation */
	uint64_t flags2;  /*: unsigned;	 { MORE Special flags	} */
	uint64_t flags;   /*: unsigned;	 { Special flags	} */
	int64_t p1;	       /* { Misc. use variable	} */
	int64_t cost;	     /* { Cost of item		} */
	int64_t subval;	   /* { Sub-catagory number	} */
	uint16_t weight; /* { Weight in gp's	} */
	uint16_t number; /* { Number of items	} */
	int16_t tohit;	   /* { Pluses to hit		} */
	int16_t todam;	   /* { Pluses to damage	} */
	int16_t ac;	      /* { Normal AC		} */
	int16_t toac;	    /* { Pluses to AC		} */
	dtype damage;	  /* { Damage when hits	}*/
	int8_t level;     /* { Level item found	} */
} treasure_type;

typedef struct treas_rec
{
	treasure_type data;
	boolean ok;
	uint16_t insides;
	boolean is_in;
	struct treas_rec *next;
} treas_rec;

typedef treas_rec *treas_ptr;

/* various player fields */

typedef struct p_misc
{
	int64_t xtr_wgt;	  /* { Extra weight limit	} */
	int64_t account;		  /* { Money in the bank	} */
	money_type money;	 /* { Money on person	} */
	game_time_type birth;     /* {Date of char's birth} */
	game_time_type cur_age;   /* {Current game date	} */
	time_type play_tm;	/* { Time spent in game	} */
	uint8_t diffic;     /* { Difficulty of game	} */
	vtype name;		  /* { Name of character	} */
	vtype race;		  /* { Race of character	} */
	vtype sex;		  /* { Sex of character	} */
	vtype title;		  /* { Character's title	} */
	vtype tclass;		  /* { Character's class	} */
	int64_t max_exp;		  /* { Max experience} */
	int64_t exp;		  /* { Cur experienc	} */
	int64_t rep;		  /* { XP from good creatures } */
	int64_t deaths;		  /* {Number of insured restores} */
	int64_t premium;		  /* {Base cost to restore } */
	uint16_t age;       /* { Characters age} */
	uint16_t ht;	/* { Height	} */
	uint16_t wt;	/* { Weight	} */
	uint16_t lev;       /* { Level		} */
	uint16_t max_lev;   /* { Max level explored} */
	int16_t srh;		  /* { Chance in search} */
	int16_t fos;		  /* { Frenq of search} */
	int16_t bth;		  /* { Base to hit	} */
	int16_t bthb;		  /* { BTH with bows	} */
	int16_t mana;		  /* { Mana points	} */
	int16_t mhp;		  /* { Max hit pts	} */
	int16_t ptohit;		  /* { Pluses to hit	} */
	int16_t ptodam;		  /* { Pluses to dam	} */
	int16_t pac;		  /* { Total AC	} */
	int16_t ptoac;		  /* { Magical AC	} */
	int16_t dis_th;		  /* { Display +ToHit} */
	int16_t dis_td;		  /* { Display +ToDam} */
	int16_t dis_ac;		  /* { Display +ToAC } */
	int16_t dis_tac;		  /* { Display +ToTAC} */
	int16_t disarm;		  /* { % to Disarm	} */
	int16_t save;		  /* { Saving throw	} */
	int16_t sc;		  /* { Social Class	} */
	enum class_t pclass;      /* { # of class	} */
	uint8_t prace;      /* { # of race	} */
	uint8_t hitdie;     /* { Char hit die	} */
	uint8_t stl;	/* { Stealth factor} */
	float expfact;		  /* { Experience factor} */
	float cmana;		  /* { Cur mana pts  } */
	float chp;		  /* { Cur hit pts	} */
	vtype history[5];	 /* array [1..5] of vtype;{ History record} */
	boolean cheated;	  /*{ gone into wizard or god mode} */
	int64_t  mr;		  /* { mag.res.lev.delta } */
	uint8_t quests;     /* { # completed } {FUBAR} */
	uint16_t cur_quest; /* { creature # of quest } {FUBAR} */
	time_t creation_time;     /* used as key in master file */
	int64_t save_count;	  /* compared to master file value */
	int64_t claim_check;	 /* used to track trading post */
} p_misc;

typedef struct p_flags
{
	boolean insured;      /* { Character insured   } */
	boolean dead;	 /* { Currently restored  } */
	uint64_t status; /* { Status of player    } */
	int64_t rest;	    /* { Rest counter	 } */
	int64_t blind;	   /* { Blindness counter   } */
	int64_t paralysis;       /* { Paralysis counter   } */
	int64_t confused;	/* { Confusion counter   } */
	int64_t foodc;	   /* { Food counter        } (was just food) */
	int64_t food_digested;   /* { Food per round      } */
	int64_t protection;      /* { Protection fr. evil } */
	int64_t speed;	   /* { Cur speed adjust    } */
	int64_t speed_paral;     /* { Slow speed adjust   } */
	boolean speed_flag;   /* { On if reset speed   } */
	int64_t paral_init;      /* { Init val for slow   } */
	int64_t move_rate;       /* { move_rate	         } */
	int64_t swim;	    /* { Cur swim adjust     } */
	int64_t fast;	    /* { Temp speed change   } */
	int64_t slow;	    /* { Temp speed change   } */
	int64_t petrification;   /* { Amount Petrified    } */
	int64_t afraid;	  /* { Fear                } */
	int64_t poisoned;	/* { Poisoned            } */
	int64_t image;	   /* { Hallucinate         } */
	int64_t protevil;	/* { Protect VS evil     } */
	int64_t invuln;	  /* { Increases AC        } */
	int64_t hero;	    /* { Heroism	         } */
	int64_t shero;	   /* { Super Heroism	 } */
	int64_t blessed;	 /* { Blessed	         } */
	int64_t resist_heat;     /* { Timed heat resist   } */
	int64_t resist_cold;     /* { Timed cold resist   } */
	int64_t detect_inv;      /* { Timed see invisible } */
	int64_t word_recall;     /* { Timed teleport level} */
	int64_t see_infra;       /* { See warm creatures  } */
	int64_t tim_infra;       /* { Timed infra vision  } */
	boolean see_inv;      /* { Can see invisible   } */
	boolean teleport;     /* { Random teleportation} */
	boolean free_act;     /* { Never paralyzed     } */
	boolean slow_digest;  /* { Lower food needs    } */
	boolean aggravate;    /* { Agravate monsters   } */
	boolean fire_resist;  /* { Resistance to fire  } */
	boolean cold_resist;  /* { Resistance to cold  } */
	boolean acid_resist;  /* { Resistance to acid  } */
	boolean hunger_item;  /* { Resets food counter } */
	boolean regenerate;   /* { Regenerate hit pts  } */
	boolean lght_resist;  /* { Resistance to light } */
	boolean ffall;	/* { No damage falling   } */
	boolean sustain[STAT_SET_MAX + 1]; /* { keep characteristic } */
	boolean confuse_monster;	   /* { Glowing hands...    } */
	int64_t resist_lght;		   /* { Timed lighting rst  } */
	int64_t free_time;			   /* { Timed free action   } */
	int64_t ring_fire;			   /* { Timed fire spell    } */
	int64_t protmon;			   /* { Timed monst prot    } */
	int64_t hoarse;			   /* { Timed no-bard spells} */
	int64_t magic_prot;		   /* { Timed magic prot    } */
	int64_t ring_ice;			   /* { Timed cold spell    } */
	int64_t temp_stealth;		   /* { Timed stealth       } */
	int64_t resist_petri;		   /* { Timed resist petrify} */
	int64_t blade_ring;		   /* { Timed blade spell   } */
	boolean petri_resist;		   /* { Resist Petrification} */
	boolean quested;		   /* { Performing a Quest  } {FUBAR} */
	boolean light_on;		   /* { Light source is active } */
	boolean resting_till_full;
} p_flags;

typedef struct player_type
{
	p_misc misc;
	p_flags flags;
} player_type;

typedef struct spell_t
{
	ctype sname;
	uint8_t slevel;
	uint8_t smana;
	uint16_t sexp;
	uint8_t sfail;
	boolean learned;
} spell_t;

typedef struct spl_rec
{
	int64_t splnum;
	int64_t splchn;
} spl_rec;

typedef spl_rec spl_type[MAX_SPELLS]; /* array [1..max_spells] of spl_rec; */

typedef struct background_type
{
	vtype info;	  /* { History information	} */
	uint8_t roll;  /* { Die roll needed for history} */
	uint8_t chart; /* { Table number		} */
	int8_t next;    /* { Pointer to next table	} */
	uint8_t bonus;   /* { Bonus to the Social Class	} */
} background_type;

typedef struct floor_type
{
	uint8_t ftval; /*: [bit(7),pos(0)] 0..127; */
	boolean ftopen;      /*: [bit(1),pos(7)] boolean; */
} floor_type;

typedef struct cave_type
{
	uint8_t cptr; /*	: unsigned char; */
	uint8_t tptr; /*	: unsigned char; */
	uint8_t fval; /*	: [bit(7),pos(16)] 0..127; */
	boolean fopen;      /*	: [bit(1),pos(23)] boolean; */
	boolean fm;	 /*	: [bit(1),pos(24)] boolean; */
	boolean pl;	 /*	: [bit(1),pos(25)] boolean; */
	boolean tl;	 /*	: [bit(1),pos(26)] boolean; */
	boolean moved;      /*	: [bit(1),pos(27)] boolean; */
	uint8_t oct;  /*	: [bit(3),pos(28)] 0..7; { octant direction } */
	uint8_t h2o;  /*	: [bit(4),pos(31)] 0..15; */
} cave_type;

typedef cave_type
    row_floor[MAX_WIDTH + 1]; /*= array [1..max_width] of cave_type; */

typedef struct owner_type
{
	vtype owner_name;
	int64_t max_cost;
	float max_inflate;
	float min_inflate;
	float haggle_per;
	uint8_t owner_race;
	uint8_t insult_max;
} owner_type;

typedef struct inven_record
{
	int64_t scost;
	treasure_type sitem;
} inven_record;

typedef struct store_type
{
	game_time_type store_open;
	uint8_t owner;
	int8_t insult_cur;
	uint8_t store_ctr;
	inven_record store_inven[STORE_INVEN_MAX + 1];
	/* : array [1..store_inven_max] of inven_record; */
} store_type;

typedef struct message_record
{
	string data;
	struct message_record *next;
} message_record;

typedef message_record *message_ptr;

typedef struct list_elem
{
	treasure_type data;
	struct list_elem *next;
} list_elem;

typedef list_elem *list_elem_ptr;

/* needs to be a multiple of 8 */
#define ENCRYPT_STAT_BUF_SIZE 1024
typedef struct encrypt_state
{
	uint8_t des_key[8];
	uint8_t des_ivec[8];
	boolean doit;

	boolean got_eof; /* out of bytes and hit eof         */
	int32_t buf_pos;     /* for read/write, current position */
	int32_t buf_size;    /* for reading, bytes in buffer     */
	char data_buf[ENCRYPT_STAT_BUF_SIZE + 8];
} encrypt_state;

typedef struct master_key
{
	time_t creation_time;
} master_key;

typedef struct master_entry
{
	int64_t save_count;
	int64_t deaths;
	time_t updated;
} master_entry;

#endif /* TYPES_H */
