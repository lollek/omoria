#pragma once

#include "../types.h"

#define INVEN_DISPLAY_SIZE 20

/**
 * -OK- rust
 *  display_inventory() - Display the player's inventory
 */
void display_inventory(void);

long inventory_change_all_ok_stats(bool nok, bool nin);
char cur_char1(void);
char cur_char2(void);
char cur_insure(void);
bool inven_command(char command, treas_rec **item_ptr, char prompt[82]);
void delete_inven_item(treas_rec *ptr);
void inven_destroy(treas_rec *item_ptr);
void inven_drop(treas_rec *item_ptr, long y, long x, bool mon);
long inven_damage(obj_set typ, long perc);
bool inven_check_weight(void);
bool inven_check_num(void);
treas_rec *add_inven_item(treasure_type item);
treas_rec *inven_carry(void);
bool get_item(treas_rec **com_ptr, char const *pmt, bool *redraw,
                 long count, char *choice, bool mon, bool no_wait);
treas_rec *ic__remove(long item_val, bool show_message);

/**
 * -OK-
 *  inv__equip_pos_string() - Returns a string describing equipment
 *  @out_val: Where the text will be put
 *  @equip_pos: Which equipment position to describe
 *  @counter: Counter position, will be a), b), c), etc
 */
void inv__equip_pos_string(char out_val[82], long equip_pos, long counter);

typedef struct inventory_find_result_t {
  treas_rec *first;
  long count;
} inventory_find_result_t;
inventory_find_result_t inventory_find_wearables(void);
bool inventory_find_range(obj_set const item_val, bool inner, treas_rec **first,
                   long *count);

/**
 * Prompt for what type of money to use
 */
long get_money_type(char prompt[134], bool *back, bool no_check);

/*  { Displays inventory items, returns chosen item if want_back. }*/
/*{ bool returns if chosen }*/
bool ic__show_inven(treas_rec **ret_ptr, bool want_back, bool clean_flag,
                    long *scr_state, bool *valid_flag, char const *prompt,
                    treas_rec *cur_display[]);
void ic__clear_display(treas_rec *cur_display[]);
