#ifndef DUNGEON_H
#define DUNGEON_H
/* there were not enough globals in variables.h */

#include "types.h"

#define DRAW_BOLT_DELAY 50
#define DRAW_BALL_DELAY 30

typedef long mm_type[6]; /* array [1..5] of long; */

extern float regen_amount;      /* { Regenerate hp and mana} */
extern boolean moria_flag;      /* { Next level when true  } */
extern boolean reset_flag;      /* { Do not move creatures } */
extern boolean search_flag;     /* { Player is searching   } */
extern boolean teleport_flag;   /* { Handle telport traps  } */
extern char s1[70];             /* { Summon item strings   } */
extern char s2[70];             /* { Summon item strings   } */
extern char s3[70];             /* { Summon item strings   } */
extern char s4[70];             /* { Summon item strings   } */
extern long i_summ_count;       /* { Summon item count	   } */

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
void desc_remain(treas_rec *item_ptr);
void add_food(long num);
void desc_charges(treas_rec *item_ptr);
boolean move_to_creature(long dir, long *y, long *x);
boolean bolt_to_creature(long dir, long *y, long *x, long *dist, long max_dist,
                         boolean visable);
boolean cast_spell(char prompt[82], treas_rec *item_ptr, long *sn, long *sc,
                   boolean *redraw);
boolean d__get_dir(char prompt[82], long *dir, long *com_val, long *y, long *x);
boolean explode(enum spell_effect_t typ, long y, long x, long dam_hp,
                const char *descrip);
void teleport(long dis);
boolean create_water(long y, long x);
boolean destroy_water(long y, long x);
boolean item_petrify(void);
boolean xor (long thing1, long thing2);
void blow(void);
void d__quit(void);

void C_commands_show_class_restrictions(void);
void move_rec(long y1, long x1, long y2, long x2);
void change_speed(long num);
void py_bonuses(treasure_type *tobj, long factor);
boolean get_panel(long y, long x, boolean forceit);
void search(long y, long x, long chance);
void area_affect(long dir, long y, long x);
void carry(long y, long x);
void move_light(long y1, long x1, long y2, long x2);
void light_room(long y, long x);
void lite_spot(long y, long x);
void unlite_spot(long y, long x);
boolean panel_contains(long y, long x);
boolean no_light();
void kicked_out();
void call_guards(char who[82]);
void call_wizards();
void beg_food();
void beg_money();
void invite_for_meal();
void party();
void spend_the_night(char who[82]);
void worship();
void battle_game(long plus, char kb_str[82]);
void brothel_game();
void thief_games();
long react(long x);
void change_rep(long amt);
boolean minus_ac(long typ_dam);
void corrode_gas(char kb_str[82]);
void poison_gas(long dam, char kb_str[82]);
void fire_dam(long dam, char kb_str[82]);
void acid_dam(long dam, char kb_str[82]);
void cold_dam(long dam, char kb_str[82]);
void light_dam(long dam, char kb_str[82]);
void monster_death(long y, long x, unsigned long flags);
long mon_take_hit(long monptr, long dam);
long tot_dam(treasure_type *item, long tdam, creature_type *monster);
boolean py_attack(long y, long x);
boolean find_range(obj_set const item_val, boolean inner,
                          treas_rec **first, long *count);
boolean player_test_hit(long bth, long level, long pth, long ac,
                               boolean was_fired);
boolean test_hit(long bth, long level, long pth, long ac);
void delete_monster(long i2);
void summon_object(long y, long x, long num, long typ);

/**
 * -DMF-
 * get_money_type() - Prompt for what type of money to use
 */
long get_money_type(char prompt[134], boolean *back, boolean no_check);

/**
 * coin_stuff() - Figure out what kind of coin is beign asked about
 * @typ: Initial of coin metal
 * @type_num: ???
 */
boolean coin_stuff(char typ, long *type_num);

long movement_rate(long cspeed, long mon);
void get_player_move_rate();
void xp_loss(long amount);
boolean twall(long y, long x, long t1, long t2);
void dungeon();
void py_bonuses(treasure_type *tobj, long factor);
boolean delete_object(long y, long x);

#endif /* DUNGEON_H */
