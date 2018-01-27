#ifndef TYPES_H
#define TYPES_H
/* The global types for imoria */

typedef unsigned char boolean;

typedef struct quad_type
{
	unsigned short l0;
	unsigned short l1;
} quad_type;

typedef char btype[14 + 2];
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
typedef unsigned char obj_set[MAX_OBJ_SET]; /* = set of 0..255; */
typedef char stat_set;			    /* = 0..5; */
typedef unsigned char
    stat_s_type[STAT_SET_MAX + 1];    /* = array [stat_set] of unsigned char; */
typedef long money_type[MITHRIL + 1]; /* = array[total$..mithril] of long; */

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
	long year;
	unsigned char month;
	unsigned char day;
	unsigned char hour;
	unsigned short secs;
} game_time_type;

typedef struct time_type
{
	unsigned short years;
	unsigned short months;
	unsigned short days;
	unsigned short hours;
	unsigned short minutes;
	unsigned short seconds;
	unsigned short hundredths;
} time_type;

typedef struct creature_type
{
	unsigned char aaf;    /*: [bit(7),pos(0)] 0..127; Area affect radius */
	unsigned char ac;     /*: [bit(7),pos(8)] 0..127;	 AC */
	ctype name;	   /* Descrip of creature */
	unsigned long cmove;  /*: unsigned;	Bit field */
	unsigned long spells; /*: unsigned;	Creature spells	 */
	unsigned long cdefense; /*: unsigned;	Bit field */
	short sleep;		/* Inactive counter */
	long mexp;		/* Exp value for kill */
	signed char speed;      /* Movement speed */
	char cchar;		/* Character rep. */
	dtype hd;		/* Creatures hit die */
	etype damage;		/* Type attack and damage */
	signed char level;      /* Level of creature */
	unsigned char mr;       /* Magic Resistance */
} creature_type;

typedef struct monster_type
{
	short hp;	    /* Hit points		*/
	short csleep;	/* Inactive counter	*/
	short cdis;	  /* Cur dis from player	*/
	unsigned short mptr; /* Pointer into creature	*/
	unsigned short nptr; /* Pointer to next block	*/
	signed char cspeed;  /* Movement speed	*/

	/* Note: FY and FX constrain dungeon size to 255 */
	unsigned char fy; /* Y Pointer into map	 */
	unsigned char fx; /* X Pointer into map	 */

	signed char stunned; /* [bit(6),pos(104)] -32..31; Rounds stunned */
	boolean ml;	  /* [bit(1),pos(110)] boolean; On if shown   */
	boolean confused;    /* [bit(1),pos(111)] boolean; On if confused */
	boolean moved;       /* [bit(1),pos(112)] boolean; On if water-moved */
} monster_type;

typedef struct treasure_type
{
	ttype name;	    /* Object name		*/
	unsigned char tval;    /* Catagory number	*/
	chtype tchar;	  /* Character representation */
	unsigned long flags2;  /*: unsigned;	 { MORE Special flags	} */
	unsigned long flags;   /*: unsigned;	 { Special flags	} */
	long p1;	       /* { Misc. use variable	} */
	long cost;	     /* { Cost of item		} */
	long subval;	   /* { Sub-catagory number	} */
	unsigned short weight; /* { Weight in gp's	} */
	unsigned short number; /* { Number of items	} */
	short tohit;	   /* { Pluses to hit		} */
	short todam;	   /* { Pluses to damage	} */
	short ac;	      /* { Normal AC		} */
	short toac;	    /* { Pluses to AC		} */
	dtype damage;	  /* { Damage when hits	}*/
	signed char level;     /* { Level item found	} */
} treasure_type;

typedef struct treas_rec
{
	treasure_type data;
	boolean ok;
	unsigned short insides;
	boolean is_in;
	struct treas_rec *next;
} treas_rec;

typedef treas_rec *treas_ptr;

/* various player fields */

typedef struct p_misc
{
	long xtr_wgt;		  /* { Extra weight limit	} */
	long account;		  /* { Money in the bank	} */
	money_type money;	 /* { Money on person	} */
	game_time_type birth;     /* {Date of char's birth} */
	game_time_type cur_age;   /* {Current game date	} */
	time_type play_tm;	/* { Time spent in game	} */
	unsigned char diffic;     /* { Difficulty of game	} */
	vtype name;		  /* { Name of character	} */
	vtype race;		  /* { Race of character	} */
	vtype sex;		  /* { Sex of character	} */
	vtype title;		  /* { Character's title	} */
	vtype tclass;		  /* { Character's class	} */
	long max_exp;		  /* { Max experience} */
	long exp;		  /* { Cur experienc	} */
	long rep;		  /* { XP from good creatures } */
	long deaths;		  /* {Number of insured restores} */
	long premium;		  /* {Base cost to restore } */
	unsigned short age;       /* { Characters age} */
	unsigned short ht;	/* { Height	} */
	unsigned short wt;	/* { Weight	} */
	unsigned short lev;       /* { Level		} */
	unsigned short max_lev;   /* { Max level explored} */
	short srh;		  /* { Chance in search} */
	short fos;		  /* { Frenq of search} */
	short bth;		  /* { Base to hit	} */
	short bthb;		  /* { BTH with bows	} */
	short mana;		  /* { Mana points	} */
	short mhp;		  /* { Max hit pts	} */
	short ptohit;		  /* { Pluses to hit	} */
	short ptodam;		  /* { Pluses to dam	} */
	short pac;		  /* { Total AC	} */
	short ptoac;		  /* { Magical AC	} */
	short dis_th;		  /* { Display +ToHit} */
	short dis_td;		  /* { Display +ToDam} */
	short dis_ac;		  /* { Display +ToAC } */
	short dis_tac;		  /* { Display +ToTAC} */
	short disarm;		  /* { % to Disarm	} */
	short save;		  /* { Saving throw	} */
	short sc;		  /* { Social Class	} */
	enum class_t pclass;      /* { # of class	} */
	unsigned char prace;      /* { # of race	} */
	unsigned char hitdie;     /* { Char hit die	} */
	unsigned char stl;	/* { Stealth factor} */
	float expfact;		  /* { Experience factor} */
	float cmana;		  /* { Cur mana pts  } */
	float chp;		  /* { Cur hit pts	} */
	vtype history[5];	 /* array [1..5] of vtype;{ History record} */
	boolean cheated;	  /*{ gone into wizard or god mode} */
	long mr;		  /* { mag.res.lev.delta } */
	unsigned char quests;     /* { # completed } {FUBAR} */
	unsigned short cur_quest; /* { creature # of quest } {FUBAR} */
	time_t creation_time;     /* used as key in master file */
	long save_count;	  /* compared to master file value */
	long claim_check;	 /* used to track trading post */
} p_misc;

typedef struct p_stat
{
	unsigned char p[STAT_SET_MAX + 1]; /* array[stat_set] of {permanent} */
	unsigned char
	    c[STAT_SET_MAX + 1]; /* array[stat_set] of {current=p-l+m*10} */
	signed char
	    m[STAT_SET_MAX + 1]; /* array[stat_set] of {net magical adj} */
	unsigned char l[STAT_SET_MAX + 1]; /* array[stat_set] of {amt lost} */
} p_stat;

typedef struct p_flags
{
	boolean insured;      /* { Character insured   } */
	boolean dead;	 /* { Currently restored  } */
	unsigned long status; /* { Status of player    } */
	long rest;	    /* { Rest counter	 } */
	long blind;	   /* { Blindness counter   } */
	long paralysis;       /* { Paralysis counter   } */
	long confused;	/* { Confusion counter   } */
	long foodc;	   /* { Food counter        } (was just food) */
	long food_digested;   /* { Food per round      } */
	long protection;      /* { Protection fr. evil } */
	long speed;	   /* { Cur speed adjust    } */
	long speed_paral;     /* { Slow speed adjust   } */
	boolean speed_flag;   /* { On if reset speed   } */
	long paral_init;      /* { Init val for slow   } */
	long move_rate;       /* { move_rate	         } */
	long swim;	    /* { Cur swim adjust     } */
	long fast;	    /* { Temp speed change   } */
	long slow;	    /* { Temp speed change   } */
	long petrification;   /* { Amount Petrified    } */
	long afraid;	  /* { Fear                } */
	long poisoned;	/* { Poisoned            } */
	long image;	   /* { Hallucinate         } */
	long protevil;	/* { Protect VS evil     } */
	long invuln;	  /* { Increases AC        } */
	long hero;	    /* { Heroism	         } */
	long shero;	   /* { Super Heroism	 } */
	long blessed;	 /* { Blessed	         } */
	long resist_heat;     /* { Timed heat resist   } */
	long resist_cold;     /* { Timed cold resist   } */
	long detect_inv;      /* { Timed see invisible } */
	long word_recall;     /* { Timed teleport level} */
	long see_infra;       /* { See warm creatures  } */
	long tim_infra;       /* { Timed infra vision  } */
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
	long resist_lght;		   /* { Timed lighting rst  } */
	long free_time;			   /* { Timed free action   } */
	long ring_fire;			   /* { Timed fire spell    } */
	long protmon;			   /* { Timed monst prot    } */
	long hoarse;			   /* { Timed no-bard spells} */
	long magic_prot;		   /* { Timed magic prot    } */
	long ring_ice;			   /* { Timed cold spell    } */
	long temp_stealth;		   /* { Timed stealth       } */
	long resist_petri;		   /* { Timed resist petrify} */
	long blade_ring;		   /* { Timed blade spell   } */
	boolean petri_resist;		   /* { Resist Petrification} */
	boolean quested;		   /* { Performing a Quest  } {FUBAR} */
	boolean light_on;		   /* { Light source is active } */
	boolean resting_till_full;
} p_flags;

typedef struct player_type
{
	p_misc misc;
	p_stat stat;
	p_flags flags;
} player_type;

typedef struct spell_t
{
	ctype sname;
	unsigned char slevel;
	unsigned char smana;
	unsigned short sexp;
	unsigned char sfail;
	boolean learned;
} spell_t;

typedef struct spl_rec
{
	long splnum;
	long splchn;
} spl_rec;

typedef spl_rec spl_type[MAX_SPELLS]; /* array [1..max_spells] of spl_rec; */

typedef struct background_type
{
	vtype info;	  /* { History information	} */
	unsigned char roll;  /* { Die roll needed for history} */
	unsigned char chart; /* { Table number		} */
	signed char next;    /* { Pointer to next table	} */
	signed char bonus;   /* { Bonus to the Social Class	} */
} background_type;

typedef struct floor_type
{
	unsigned char ftval; /*: [bit(7),pos(0)] 0..127; */
	boolean ftopen;      /*: [bit(1),pos(7)] boolean; */
} floor_type;

typedef struct cave_type
{
	unsigned char cptr; /*	: unsigned char; */
	unsigned char tptr; /*	: unsigned char; */
	unsigned char fval; /*	: [bit(7),pos(16)] 0..127; */
	boolean fopen;      /*	: [bit(1),pos(23)] boolean; */
	boolean fm;	 /*	: [bit(1),pos(24)] boolean; */
	boolean pl;	 /*	: [bit(1),pos(25)] boolean; */
	boolean tl;	 /*	: [bit(1),pos(26)] boolean; */
	boolean moved;      /*	: [bit(1),pos(27)] boolean; */
	unsigned char oct;  /*	: [bit(3),pos(28)] 0..7; { octant direction } */
	unsigned char h2o;  /*	: [bit(4),pos(31)] 0..15; */
} cave_type;

typedef cave_type
    row_floor[MAX_WIDTH + 1]; /*= array [1..max_width] of cave_type; */

typedef struct owner_type
{
	vtype owner_name;
	long max_cost;
	float max_inflate;
	float min_inflate;
	float haggle_per;
	unsigned char owner_race;
	unsigned char insult_max;
} owner_type;

typedef struct inven_record
{
	long scost;
	treasure_type sitem;
} inven_record;

typedef struct store_type
{
	game_time_type store_open;
	unsigned char owner;
	signed char insult_cur;
	unsigned char store_ctr;
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
	unsigned char des_key[8];
	unsigned char des_ivec[8];
	boolean doit;

	boolean got_eof; /* out of bytes and hit eof         */
	int buf_pos;     /* for read/write, current position */
	int buf_size;    /* for reading, bytes in buffer     */
	char data_buf[ENCRYPT_STAT_BUF_SIZE + 8];
} encrypt_state;

typedef struct master_key
{
	time_t creation_time;
} master_key;

typedef struct master_entry
{
	long save_count;
	long deaths;
	time_t updated;
} master_entry;

#endif /* TYPES_H */
