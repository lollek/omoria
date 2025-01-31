#pragma once

#include <curses.h>
// ReSharper disable once CppUnusedIncludeDirective
#include <time.h> // time_t

#include "types.h"

enum keypad_direction_t {
  KEYPAD_DOWN_LEFT = 1,
  KEYPAD_DOWN = 2,
  KEYPAD_DOWN_RIGHT = 3,
  KEYPAD_LEFT = 4,
  KEYPAD_NONE = 5,
  KEYPAD_RIGHT = 6,
  KEYPAD_UP_LEFT = 7,
  KEYPAD_UP = 8,
  KEYPAD_UP_RIGHT = 9,
};

long y_from_keypad_direction(enum keypad_direction_t keypad_direction);
long x_from_keypad_direction(enum keypad_direction_t keypad_direction);

/* with fake bool values you cant really do a (bool1 != bool2) and expect it to work.  */
bool xor (long thing1, long thing2);

/*{ Prompts for a direction                               -RAK-   }*/
bool d__get_dir(char prompt[82], long *dir, long *com_val, long *y, long *x);
long maxmin(long x, long y, long z);
long minmax(long x, long y, long z);
long bit_pos(unsigned long *test);
long bit_pos64(unsigned long *high, unsigned long *low);
long distance(long y1, long x1, long y2, long x2);
void insert_str(const char *object_str, char const *mtc_str, char const *insert_str);
bool in_bounds(long y, long x);
/*{ Counts number of the type in north, south, east, and west positions	}*/
long next_to4(long y, long x, obj_set group_set);
/*{ Checks all adjacent spots for elements		-RAK-	*/
long next_to8(long y, long x, obj_set group_set);
long rotate_dir(long dir, long rot);
long get_hexdecant(long dy, long dx);
void tlink(void);
void mlink(void);
long damroll(char const *dice);
bool los(long y1, long x1, long y2, long x2);
chtype get_loc_symbol(long y, long x);
chtype loc_symbol(long y, long x);
bool test_light(long y, long x);
void validate_monsters(void);
void compact_monsters(void);
void popm(long *x);
void pushm(long x);
long max_hp(char const *hp_str);
void petrify(long amt);
void compact_objects(void);
void popt(long *x);
void pusht(long x);
void alloc_object(obj_set alloc_set, long typ, long num);
char *place_string(long num, char result[134]);
char *day_of_week_string(long day, unsigned wid, char result[134]);
char *month_string(long mon, char result[134]);
char *time_string(long hour, long sec, char result[134]);
void time_diff(game_time_type a, game_time_type b, game_time_type *c);
void add_days(game_time_type *ti, long d);
char *full_date_string(game_time_type time, char result[134]);
void adv_time(bool flag);
char *play_time(const time_type *t, char result[134]);
time_type *convert_seconds_to_time(time_t seconds, time_type *tim);
time_t convert_time_to_seconds(const time_type *tim);
char *show_char_age(char result[134]);
char *show_current_time(char result[134]);
char *likert(long x, long y, char *result);
unsigned char characters_sex(void);
treas_rec *money_carry(void);
char *cost_str(long amt, char result[134]);
uint16_t max_allowable_weight(void);
uint16_t min_allowable_weight(void);
void add_money(long amount);
void subtract_money(long amount, bool make_change);
bool send_page(long to_bank);
void spell_chance(spl_rec *spell);
bool get_spell(spl_type spell, long num, long *sn, long *sc, char prompt[82],
                  bool *redraw);
long num_new_spells(long smarts);
void learn_magic(bool redraw);
void gain_level(void);
void insert_num(const char *object_str, const char *mtc_str, long number,
                bool show_sign);
long critical_blow(long weight, long plus, bool cs_sharp, bool is_fired);
bool move_dir(enum keypad_direction_t dir, long *y, long *x);
bool player_saves(long adjust);
bool player_spell_saves(void);
void find_monster_name(char m_name[82], long ptr, bool begin_sentence);

/**
 * Figure out what kind of coin is beign asked about
 * 
 * typ: Initial of coin metal
 * type_num: ???
 */
bool coin_stuff(char typ, long *type_num);

/*{ Deletes object from given location                    -RAK-   }*/
bool delete_object(long y, long x);

/*{ Tunneling through wall }*/
bool twall(long y, long x, long t1, long t2);

/**
 * Get a 'dir' from a roguelike command
 * 
 * c: character to translate
 * return: number [0-9] or -1 on error
 */
int char_to_dir(char c);

/**
 * Get roguelike command for a direction
 * 
 * dir: Direction to get command for
 * return: Roguelike command in char, or '?' on error;
 */
char dir_to_char(int dir);

/*{ recomputes cash totals for player and bank }*/
void reset_total_cash(void);
