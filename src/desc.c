#include <curses.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "configure.h"
#include "constants.h"
#include "debug.h"
#include "magic.h"
#include "pascal.h"
#include "routines.h"
#include "term.h"
#include "treasures.h"
#include "types.h"
#include "variables.h"

void rantitle(char *title) {
  /*{ Return random title						}*/

  long i1, i2, i3, i4;

  i3 = randint(2) + 1; /* two or three words */
  strcpy(title, "Titled \"");

  for (i1 = 0; i1 < i3; i1++) {
    i4 = randint(2); /* one or two syllables each */
    for (i2 = 0; i2 < i4; i2++) {
      strcat(title, syllables[randint(MAX_SYLLABLES) - 1]);
    }

    if (i1 != i3 - 1) {
      strcat(title, " ");
    }
  }
  strcat(title, "\"");
}

void identify(treasure_type *item) {
  /*{ Something has been identified }*/

  long i1, x1, x2;
  treas_rec *curse;

  x1 = item->tval;
  x2 = item->subval;

  if (strstr(item->name, "|") != NULL) {
    for (i1 = 0; i1 < MAX_TALLOC; i1++) {
      /* with t_list[i1] do; */
      if ((t_list[i1].tval == x1) && (t_list[i1].subval == x2)) {
        unquote(t_list[i1].name);
        known1(t_list[i1].name);
      }
    }
    for (i1 = Equipment_min; i1 <= Equipment_secondary; i1++) {
      /* with equipment[i1] do; */
      if ((equipment[i1].tval == x1) && (equipment[i1].subval == x2)) {
        unquote(equipment[i1].name);
        known1(equipment[i1].name);
      }
    }

    for (curse = inventory_list; curse != NULL; curse = curse->next) {
      /* with curse^.data do; */
      if ((curse->data.tval == x1) && (curse->data.subval == x2)) {
        unquote(curse->data.name);
        known1(curse->data.name);
      }
    }

    i1 = 0;
    do {
      i1++;
      /* with object_list[i1] do; */
      if ((object_list[i1].tval == x1) && (object_list[i1].subval == x2)) {
        if (strstr(object_list[i1].name, "%T") != NULL) {
          insert_str(object_list[i1].name, " %T|", "");
          object_ident[i1] = true;
        } else {
          unquote(object_list[i1].name);
          known1(object_list[i1].name);
          object_ident[i1] = true;
        }
      }
    } while (i1 != MAX_OBJECTS);
  } /* end if | */
}

void known1(char *object_str) {
  /*{ Remove 'Secret' symbol for identity of object
   * }*/

  insert_str(object_str, "|", "");
}

void known2(char *object_str) {
  /*{ Remove 'Secret' symbol for identity of pluses
   * }*/

  insert_str(object_str, "^", "");
}

void unquote(char *object_str) {
  /*	{ Return string without quoted portion }*/

  long pos0, pos1, pos2, olen;
  char str1[82], str2[82];

  pos0 = pindex(object_str, '"');
  if (pos0 > 0) {
    pos1 = pindex(object_str, '~');
    pos2 = pindex(object_str, '|');
    olen = strlen(object_str);
    strncpy(str1, object_str, pos1);
    str1[pos1] = 0;
    strncpy(str2, &(object_str[pos2]), olen - pos2);
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
void objdes(char *out_val, treas_rec *ptr, boolean pref) {
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

char *bag_descrip(treas_rec *bag, char result[134]) /* was func */
{
  /*{ Return description about the contents of a bag	-DMF-	}*/

  long count, wgt;
  treas_rec *ptr;

  if ((bag->next == NULL) || (bag->next->is_in == false)) {
    sprintf(result, " (empty)");
  } else {
    count = 0;
    wgt = 0;

    for (ptr = bag->next; (ptr != NULL) && (ptr->is_in); ptr = ptr->next) {
      count += ptr->data.number;
      wgt += ptr->data.weight * ptr->data.number;
    }

    sprintf(result, " (%ld%% full, containing %ld item%s)",
            (wgt * 100 / bag->data.p1), count, ((count != 1) ? "s" : ""));
  }
  return result;
}
