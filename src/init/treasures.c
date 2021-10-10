#include <stdbool.h>
#include <stdio.h>

#include "../variables.h"
#include "../debug.h"
#include "../treasures.h"

#include "treasures.h"

/**
 * sort_treasures() - Order the treasure list by level
 */
static bool sort_treasures(void) {
  ENTER(("sort_treasures", ""));

  for (long i = 1; i < MAX_OBJECTS; ++i) {
    for (long j = i + 1; j <= MAX_OBJECTS; ++j) {
      if (object_list[i].level > object_list[j].level) {
        treasure_type tmp = object_list[i];
        object_list[i] = object_list[j];
        object_list[j] = tmp;
      }
    }
  }

#if DO_DEBUG
  /*  Verify that the sort worked */
  int8_t counter = 0;
  for (long i = 1; i <= MAX_OBJECTS; ++i) {
    if (counter > object_list[i].level) {
      fprintf(stderr, "Error: sort_treasures failed!\n"
          "Index: %ld, left: %d, right: %d",
          i, counter, object_list[i].level);
      return false;
    }
    counter = object_list[i].level;
  }
#endif
  LEAVE("sort_treasures", "");
  return true;
}

/**
 * init_t_level() - Initializes T_LEVEL array for use with PLACE_OBJECT
 *
 * I don't really understand this function.
 */
static bool init_t_level(void) {
  ENTER(("init_t_level", ""));

  int i1 = 1;
  int i2 = 0;

  do {
    while ((i1 <= MAX_OBJECTS) && (object_list[i1].level == i2)) {
      t_level[i2] = t_level[i2] + 1; /* number of treasures with this level */
      i1++;
    }
    i2++;
  } while (!((i2 > MAX_OBJ_LEVEL) || (i1 > MAX_OBJECTS)));

  for (i1 = 1; i1 <= MAX_OBJ_LEVEL; i1++) {
    t_level[i1] += t_level[i1 - 1];
  }

  LEAVE("init_t_level", "");
  return true;
}

bool init__treasures(void) {
  if (!sort_treasures()) return false;
  if (!init_t_level()) return false;
  return true;
}
