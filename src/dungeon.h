#ifndef DUNGEON_H
#define DUNGEON_H
/* there were not enough globals in variables.h */

#define DRAW_BOLT_DELAY 50
#define DRAW_BALL_DELAY 30

typedef long mm_type[6]; /* array [1..5] of long; */

extern long dir_val; /* { For movement          } */
/* extern	long	y,x,moves;      	// { For movement          } */
/* extern	long	i1,i2,tmp1;      	// { Temporaries           } */
extern long old_chp, old_cmana; /* { Detect change         } */
extern float regen_amount;      /* { Regenerate hp and mana} */
extern char command;		/* { Last command          } */
/* extern	vtype	out_val,out2;        	// { For messages          } */
/* extern	vtype	tmp_str;        	// { Temporary             } */
extern boolean moria_flag;    /* { Next level when true  } */
extern boolean reset_flag;    /* { Do not move creatures } */
extern boolean search_flag;   /* { Player is searching   } */
extern boolean teleport_flag; /* { Handle telport traps  } */
extern boolean player_light;  /* { Player carrying light } */
extern boolean save_msg_flag; /* { Msg flag after INKEY  } */
extern char s1[70], s2[70], s3[70], s4[70];  /* { Summon item strings   } */
extern long i_summ_count;     /* { Summon item count	   } */
/* extern	char		trash_char; */
/* extern	FILE *		f1; */
/* extern	stat_set	tstat; */
/* extern	treas_ptr	trash_ptr; */

#define DISPLAY_SIZE 20
#define MOO_DISPLAY_SIZE 18

/* index stuff for door_list */
#define DL_OPEN 0
#define DL_CLOSED 1
#define DL_SECRET 2

#define ML(mmm) (m_list[(mmm)])
#define MY(mmm) (m_list[(mmm)].fy)
#define MX(mmm) (m_list[(mmm)].fx)

boolean do_stun(unsigned char a_cptr, long save_bonus, long time);
void desc_remain(treas_ptr item_ptr);
void add_food(long num);
void desc_charges(treas_ptr item_ptr);
boolean move_to_creature(long dir, long *y, long *x);
boolean bolt_to_creature(long dir, long *y, long *x, long *dist,
		long max_dist, boolean visable);
boolean cast_spell(char prompt[82], treas_ptr item_ptr, long *sn, long *sc,
		boolean *redraw);
boolean d__get_dir(char prompt[82], long *dir, long *com_val, long *y,
		long *x);
boolean explode(long typ, long y, long x, long dam_hp, const char *descrip);
void teleport(long dis);
boolean create_water(long y, long x);
boolean destroy_water(long y, long x);
boolean item_petrify(void);
boolean xor (long thing1, long thing2);
void blow(void);
void d__quit(void);

void C_dungeon_show_class_restrictions(void);

#endif /* DUNGEON_H */
