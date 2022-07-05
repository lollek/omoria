#ifndef MAIN_LOOP_H
#define MAIN_LOOP_H

#include "types.h"

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
boolean find_range(obj_set const item_val, boolean inner, treas_rec **first,
                   long *count);
boolean player_test_hit(long bth, long level, long pth, long ac,
                        boolean was_fired);
boolean test_hit(long bth, long level, long pth, long ac);
void delete_monster(long i2);
void summon_object(long y, long x, long num, long typ);

/**
 * Prompt for what type of money to use
 */
long get_money_type(char prompt[134], boolean *back, boolean no_check);

/**
 * Figure out what kind of coin is beign asked about
 * 
 * typ: Initial of coin metal
 * type_num: ???
 */
boolean coin_stuff(char typ, long *type_num);

long movement_rate(long cspeed, long mon);
void get_player_move_rate();
void xp_loss(long amount);
boolean twall(long y, long x, long t1, long t2);
void dungeon();
void py_bonuses(treasure_type *tobj, long factor);
boolean delete_object(long y, long x);

#endif /* MAIN_LOOP_H */
