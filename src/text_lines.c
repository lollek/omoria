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
