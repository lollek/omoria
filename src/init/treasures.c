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

bool init__treasures(void) {
  return sort_treasures();
}
