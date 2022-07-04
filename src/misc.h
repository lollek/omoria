#ifndef MISC_H
#define MISC_H

#include <curses.h>

#include "types.h"

/* with fake boolean values you cant really do a (bool1 != bool2) and expect it to work.  */
boolean xor (long thing1, long thing2);

/*{ Prompts for a direction                               -RAK-   }*/
boolean d__get_dir(char prompt[82], long *dir, long *com_val, long *y, long *x);
long maxmin(long x, long y, long z);
long minmax(long x, long y, long z);
long bit_pos(unsigned long *test);
long bit_pos64(unsigned long *high, unsigned long *low);
long distance(long y1, long x1, long y2, long x2);
void insert_str(char *object_str, char const *mtc_str, char const *insert_str);
boolean in_bounds(long y, long x);
long next_to4(long y, long x, obj_set group_set);
long next_to8(long y, long x, obj_set group_set);
long rotate_dir(long dir, long rot);
long get_hexdecant(long dy, long dx);
void tlink();
void mlink();
long damroll(char const *dice);
boolean los(long y1, long x1, long y2, long x2);
chtype get_loc_symbol(long y, long x);
chtype loc_symbol(long y, long x);
boolean test_light(long y, long x);
void validate_monsters();
void compact_monsters();
void popm(long *x);
void pushm(long x);
long max_hp(char const *hp_str);
void place_win_monster();
boolean summon_land_monster(long *y, long *x, boolean slp);
boolean summon_water_monster(long *y, long *x, boolean slp);
boolean summon_undead(long *y, long *x);
boolean summon_demon(long *y, long *x);
boolean summon_breed(long *y, long *x);
void petrify(long amt);
void compact_objects();
void popt(long *x);
void pusht(long x);
void place_open_door(long y, long x);
void place_broken_door(long y, long x);
void place_closed_door(long y, long x);
void place_locked_door(long y, long x);
void place_stuck_door(long y, long x);
void place_secret_door(long y, long x);
void place_door(long y, long x);
void place_a_staircase(long y, long x, long typ);
void place_stairs(long typ, long num, long walls);
void place_gold(long y, long x);
void place_object(long y, long x);
void alloc_object(obj_set alloc_set, long typ, long num);
void random_object(long y, long x, long num);
char *place_string(long num, char result[134]);
char *day_of_week_string(long day, unsigned wid, char result[134]);
char *month_string(long mon, char result[134]);
char *time_string(long hour, long sec, char result[134]);
void time_diff(game_time_type a, game_time_type b, game_time_type *c);
void add_days(game_time_type *ti, long d);
char *full_date_string(game_time_type time, char result[134]);
void adv_time(boolean flag);
char *play_time(time_type *t, char result[134]);
time_type *convert_seconds_to_time(time_t seconds, time_type *tim);
time_t convert_time_to_seconds(time_type *tim);
char *show_char_age(char result[134]);
char *show_current_time(char result[134]);
char *likert(long x, long y, char *result);
unsigned char characters_sex();
treas_rec *money_carry();
char *cost_str(long amt, char result[134]);
uint16_t max_allowable_weight();
uint16_t min_allowable_weight();
void add_money(long amount);
void subtract_money(long amount, boolean make_change);
boolean send_page(long to_bank);
void spell_chance(spl_rec *spell);
boolean get_spell(spl_type spell, long num, long *sn, long *sc, char prompt[82],
                  boolean *redraw);
long num_new_spells(long smarts);
void learn_magic(boolean redraw);
void gain_level();
void insert_num(char *object_str, char *mtc_str, long number,
                boolean show_sign);
long attack_blows(long weight, long *wtohit);
long critical_blow(long weight, long plus, boolean cs_sharp, boolean is_fired);
boolean move_dir(long dir, long *y, long *x);
boolean player_saves(long adjust);
boolean player_spell_saves();
void find_monster_name(char m_name[82], const long ptr, boolean begin_sentence);

#endif // MISC_H