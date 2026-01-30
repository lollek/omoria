#include <curses.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "constants.h"
#include "debug.h"
#include "misc.h"
#include "pascal.h"
#include "types.h"
#include "variables.h"

#include "text_lines.h"

#include "io.h"

void msg_remaining_of_item(const treas_rec *item_ptr) {

  char out_val[82];
  char out_val2[120];

  inven_temp.data = item_ptr->data;

  inven_temp.data.number--;
  item_name(out_val, &inven_temp);
  sprintf(out_val2, "You have %s.", out_val);
  msg_print(out_val2);
}

void msg_charges_remaining(const treas_rec *item_ptr) {

  if (strstr(item_ptr->data.name, "^") == NULL) {
    char out_val[82];
    sprintf(out_val, "You have %ld charges remaining.", item_ptr->data.p1);
    msg_print(out_val);
  }
}

void identification_set_identified(treasure_type *item); // identification.rs
void identify(treasure_type *item) {


  if (strstr(item->name, "|") == NULL)
    return;

  for (long i = 0; i < MAX_TALLOC; i++) {
    if (t_list[i].tval == item->tval && t_list[i].subval == item->subval) {
      unquote(t_list[i].name);
      known1(t_list[i].name);
    }
  }

  for (long i = Equipment_min; i <= Equipment_secondary; i++) {
    if (equipment[i].tval == item->tval && equipment[i].subval == item->subval) {
      unquote(equipment[i].name);
      known1(equipment[i].name);
    }
  }

  for (treas_rec *curse = inventory_list; curse != NULL; curse = curse->next) {
    if (curse->data.tval == item->tval && curse->data.subval == item->subval) {
      unquote(curse->data.name);
      known1(curse->data.name);
    }
  }

  identification_set_identified(item);
}
