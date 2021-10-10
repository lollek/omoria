#include <stdbool.h>
#include <stdio.h>

#include "../variables.h"
#include "../debug.h"

#include "treasures.h"

/**
 * sort_treasures() - Order the treasure list by level
 */
static bool sort_treasures(void) {
  ENTER(("sort_treasures", ""));

  for (long gap = MAX_OBJECTS / 2; gap > 0; gap = gap / 2) {
    for (long i = gap + 1; i <= MAX_OBJECTS; i++) {
      for (long j = i - gap; j > 0; j = j - gap) {
        long k = j + gap;
        if (object_list[j].level > object_list[k].level) {
          treasure_type tmp = object_list[j];
          object_list[j] = object_list[k];
          object_list[k] = tmp;
        } else {
          j = 0;
        }
      }
    }
  }

#if DO_DEBUG
  /*  Verify that the sort worked */
  long counter = 0;
  for (long i = 1; 1 <= MAX_OBJECTS; ++i) {
    if (counter > object_list[i].level) {
      fprintf(stderr, "Error: sort_treasures failed\n");
      return false;
    }
    counter = object_list[i].level;
  }
#endif
  LEAVE("sort_treasures", "");
  return true;
}

bool init__treasures(void) {
  return sort_treasures();
}
