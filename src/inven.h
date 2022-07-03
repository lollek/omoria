#ifndef INVEN_H
#define INVEN_H

#include "types.h"

long change_all_ok_stats(boolean nok, boolean nin);
char cur_char1();
char cur_char2();
char cur_insure();
boolean inven_command(char command, treas_rec **item_ptr, char prompt[82]);
void delete_inven_item(treas_rec *ptr);
void inven_destroy(treas_rec *item_ptr);
void inven_drop(treas_rec *item_ptr, long y, long x, boolean mon);
long inven_damage(obj_set typ, long perc);
boolean inven_check_weight();
boolean inven_check_num();
treas_rec *add_inven_item(treasure_type item);
treas_rec *inven_carry();
boolean get_item(treas_rec **com_ptr, char const *pmt, boolean *redraw,
                 long count, char *choice, boolean mon, boolean no_wait);
treas_rec *ic__remove(long item_val, boolean show_message);

/**
 * -OK-
 *  inv__equip_pos_string() - Returns a string describing equipment
 *  @out_val: Where the text will be put
 *  @equip_pos: Which equipment position to describe
 *  @counter: Counter position, will be a), b), c), etc
 */
void inv__equip_pos_string(char out_val[82], long equip_pos, long counter);

#endif // INVEN_H