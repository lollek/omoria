#include <curses.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "constants.h"
#include "debug.h"
#include "misc.h"
#include "pascal.h"
#include "random.h"
#include "treasures.h"
#include "types.h"
#include "variables.h"

#include "desc.h"

#include "io.h"

/*{ Describe amount of item remaining...                  -RAK-   }*/
void desc_remain(const treas_rec *item_ptr) {

  char out_val[82];
  char out_val2[120];

  inven_temp.data = item_ptr->data;

  /* with inven_temp->data do; */

  inven_temp.data.number--;
  objdes(out_val, &inven_temp, true);
  sprintf(out_val2, "You have %s.", out_val);
  msg_print(out_val2);
}

/*{ Describe number of remaining charges...               -RAK-   }*/
void desc_charges(const treas_rec *item_ptr) {

  if (strstr(item_ptr->data.name, "^") == NULL) {
    char out_val[82];
    sprintf(out_val, "You have %ld charges remaining.", item_ptr->data.p1);
    msg_print(out_val);
  }
}

void rantitle(char *title) {
  /*{ Return random title						}*/

  const long i3 = randint(2) + 1; /* two or three words */
  strcpy(title, "Titled \"");

  for (long i1 = 0; i1 < i3; i1++) {
    const long i4 = randint(2); /* one or two syllables each */
    for (long i2 = 0; i2 < i4; i2++) {
      strcat(title, syllables[randint(MAX_SYLLABLES) - 1]);
    }

    if (i1 != i3 - 1) {
      strcat(title, " ");
    }
  }
  strcat(title, "\"");
}

void identification_set_identified(treasure_type *item); // identification.rs
void identify(treasure_type *item) {
  /*{ Something has been identified }*/


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

void known1(const char *object_str) {
  /*{ Remove 'Secret' symbol for identity of object
   * }*/

  insert_str(object_str, "|", "");
}

void known2(const char *object_str) {
  /*{ Remove 'Secret' symbol for identity of pluses
   * }*/

  insert_str(object_str, "^", "");
}

void unquote(char *object_str) {
  /*	{ Return string without quoted portion }*/

  const long pos0 = pindex(object_str, '"');
  if (pos0 > 0) {
    char str2[82];
    char str1[82];
    long pos1 = pindex(object_str, '~');
    const long pos2 = pindex(object_str, '|');
    const long olen = strlen(object_str);
    strncpy(str1, object_str, pos1);
    str1[pos1] = 0;
    strncpy(str2, &object_str[pos2], olen - pos2);
    str2[olen - pos2] = 0;
    sprintf(object_str, "%s%s", str1, str2);
  }
}

/**
 * objdes() - Returns a description of item for inventory
 * @out_val: Where to put the return string
 * @ptr: Pointer to the object to describe
 * @pref: ???
 */
void objdes(char *out_val, const treas_rec *ptr, const bool pref) {
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

char *bag_descrip(const treas_rec *bag, char result[134]) /* was func */
{
  /*{ Return description about the contents of a bag	-DMF-	}*/

  if (bag->next == NULL || bag->next->is_in == false) {
    sprintf(result, " (empty)");
  } else {
    long count = 0;
    long wgt = 0;

    for (const treas_rec *ptr = bag->next; ptr != NULL && ptr->is_in; ptr = ptr->next) {
      count += ptr->data.number;
      wgt += ptr->data.weight * ptr->data.number;
    }

    sprintf(result, " (%ld%% full, containing %ld item%s)",
            wgt * 100 / bag->data.p1, count, count != 1 ? "s" : "");
  }
  return result;
}
