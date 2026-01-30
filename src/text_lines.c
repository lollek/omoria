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

void objdes(char out_val[82], const treas_rec *ptr, const bool pref) {
  char *cpos;
  char tmp_val[82];

  ENTER(("objdes", "i"));

  strcpy(tmp_val, ptr->data.name);

#if DO_DEBUG
  if (do_debug_objdes) {
    MSG(("obj start: >%s<\n", tmp_val));
  }
#endif

  cpos = strstr(tmp_val, "|");
  if (cpos != NULL) {
    *cpos = '\0';
  }

#if DO_DEBUG
  if (do_debug_objdes) {
    MSG(("obj thmp1: >%s<\n", tmp_val));
  }
#endif

  cpos = strstr(tmp_val, "^");
  if (cpos != NULL) {
    *cpos = '\0';
  }

#if DO_DEBUG
  if (do_debug_objdes) {
    MSG(("obj thmp2: >%s<\n", tmp_val));
  }
#endif

  if (!pref) {
    cpos = strstr(tmp_val, " (");
    if (cpos != NULL) {
      *cpos = '\0';
    }
  }

#if DO_DEBUG
  if (do_debug_objdes) {
    MSG(("obj thmp3: >%s<\n", tmp_val));
  }
#endif

  insert_str(tmp_val, "%P0", ptr->data.damage);
  insert_num(tmp_val, "%P1", ptr->data.p1, true);
  insert_num(tmp_val, "%P2", ptr->data.tohit, true);
  insert_num(tmp_val, "%P3", ptr->data.todam, true);
  insert_num(tmp_val, "%P4", ptr->data.toac, true);
  insert_num(tmp_val, "%P5", ptr->data.p1, false);
  insert_num(tmp_val, "%P6", ptr->data.ac, false);

#if DO_DEBUG
  if (do_debug_objdes) {
    MSG(("obj thmp4: >%s<\n", tmp_val));
  }
#endif

  if (ptr->data.number != 1) {
    insert_str(tmp_val, "ch~", "ches");
    insert_str(tmp_val, "y~", "ies");
    insert_str(tmp_val, "~", "s");
  } else {
    insert_str(tmp_val, "~", "");
  }

#if DO_DEBUG
  if (do_debug_objdes) {
    MSG(("obj thmp5: >%s<\n", tmp_val));
  }
#endif

  if (pref) {
    if (strstr(tmp_val, "&") != NULL) {
      insert_str(tmp_val, "&", "");

      if (ptr->data.number > 1) {
        sprintf(out_val, "%d%s", ptr->data.number, tmp_val);
      } else if (ptr->data.number < 1) {
        sprintf(out_val, "no more%s", tmp_val);
      } else if (is_vowel(tmp_val[1])) {
        sprintf(out_val, "an%s", tmp_val);
      } else {
        sprintf(out_val, "a%s", tmp_val);
      }

    } else {
      if (ptr->data.number > 0) {
        strcpy(out_val, tmp_val);
      } else {
        sprintf(out_val, "no more %s", tmp_val);
      }
    }

  } else {
    insert_str(tmp_val, "& ", "");
    strcpy(out_val, tmp_val);
  }

#if DO_DEBUG
  if (do_debug_objdes) {
    MSG(("obj final: >%s<\n", out_val));
  }
#endif

  LEAVE("objdes", "i");
}
