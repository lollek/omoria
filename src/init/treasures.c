#include <stdbool.h>
#include <stdio.h>
#include <math.h>
#include <string.h>

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

static bool adjust_prices(void) {
  if (COST_ADJ == 1.00) return true;

  for (long i = 1; i <= MAX_OBJECTS; i++) {
    object_list[i].cost = trunc(object_list[i].cost * COST_ADJ + 0.99);
  }

  for (long i = 1; i <= INVEN_INIT_MAX; i++) {
    inventory_init[i].cost = trunc(inventory_init[i].cost * COST_ADJ + 0.99);
  }

  return true;
}

static bool adjust_weights(void) {
  if (WEIGHT_ADJ == 1) return true;

  for (long i = 1; i <= MAX_OBJECTS; i++) {
    object_list[i].weight *= WEIGHT_ADJ;
  }

  for (long i = 1; i <= INVEN_INIT_MAX; i++) {
    inventory_init[i].weight *= WEIGHT_ADJ;
  }

  return true;
}

static bool adjust_gem_values(void) {
  for (size_t count = 1; count <= MAX_OBJECTS; count++) {
    if ((strstr(object_list[count].name, "Finely cut") != NULL) &&
        (strstr(object_list[count].name, "of") != NULL)) {

      if (strstr(object_list[count].name, "Amber") != NULL) {
        object_list[count].cost += 1000;
      }
      if (strstr(object_list[count].name, "Agate") != NULL) {
        object_list[count].cost += 1000;
      }
      if (strstr(object_list[count].name, "Alexandrite") != NULL) {
        object_list[count].cost += 5000;
      }
      if (strstr(object_list[count].name, "Amethyst") != NULL) {
        object_list[count].cost += 2000;
      }
      if (strstr(object_list[count].name, "Antlerite") != NULL) {
        object_list[count].cost += 1000;
      }
      if (strstr(object_list[count].name, "Aquamarine") != NULL) {
        object_list[count].cost += 6000;
      }
      if (strstr(object_list[count].name, "Argentite") != NULL) {
        object_list[count].cost += 1000;
      }
      if (strstr(object_list[count].name, "Azurite") != NULL) {
        object_list[count].cost += 1000;
      }
      if (strstr(object_list[count].name, "Beryl") != NULL) {
        object_list[count].cost += 2000;
      }
      if (strstr(object_list[count].name, "Bloodstone") != NULL) {
        object_list[count].cost += 3500;
      }
      if (strstr(object_list[count].name, "Calcite") != NULL) {
        object_list[count].cost += 1500;
      }
      if (strstr(object_list[count].name, "Carnelian") != NULL) {
        object_list[count].cost += 1000;
      }
      if (strstr(object_list[count].name, "Coral") != NULL) {
        object_list[count].cost += 1000;
      }
      if (strstr(object_list[count].name, "Corundum") != NULL) {
        object_list[count].cost += 1000;
      }
      if (strstr(object_list[count].name, "Cryolite") != NULL) {
        object_list[count].cost += 1000;
      }
      if (strstr(object_list[count].name, "Diamond") != NULL) {
        object_list[count].cost += 35000;
      }
      if (strstr(object_list[count].name, "Diorite") != NULL) {
        object_list[count].cost += 1000;
      }
      if (strstr(object_list[count].name, "Emerald") != NULL) {
        object_list[count].cost += 20000;
      }
      if (strstr(object_list[count].name, "Flint") != NULL) {
        object_list[count].cost += 5000;
      }
      if (strstr(object_list[count].name, "Fluorite") != NULL) {
        object_list[count].cost += 1000;
      }
      if (strstr(object_list[count].name, "Gabbro") != NULL) {
        object_list[count].cost += 5000;
      }
      if (strstr(object_list[count].name, "Garnet") != NULL) {
        object_list[count].cost += 6500;
      }
      if (strstr(object_list[count].name, "Granite") != NULL) {
        object_list[count].cost += 500;
      }
      if (strstr(object_list[count].name, "Gypsum") != NULL) {
        object_list[count].cost += 3000;
      }
      if (strstr(object_list[count].name, "Hematite") != NULL) {
        object_list[count].cost += 1000;
      }
      if (strstr(object_list[count].name, "Jade") != NULL) {
        object_list[count].cost += 12000;
      }
      if (strstr(object_list[count].name, "Jasper") != NULL) {
        object_list[count].cost += 3000;
      }
      if (strstr(object_list[count].name, "Kryptonite") != NULL) {
        object_list[count].cost += 5000;
      }
      if (strstr(object_list[count].name, "Lapus lazuli") != NULL) {
        object_list[count].cost += 4500;
      }
      if (strstr(object_list[count].name, "Limestone") != NULL) {
        object_list[count].cost += 1000;
      }
      if (strstr(object_list[count].name, "Malachite") != NULL) {
        object_list[count].cost += 3000;
      }
      if (strstr(object_list[count].name, "Manganite") != NULL) {
        object_list[count].cost += 5000;
      }
      if (strstr(object_list[count].name, "Marble") != NULL) {
        object_list[count].cost += 5500;
      }
      if (strstr(object_list[count].name, "Mica") != NULL) {
        object_list[count].cost += 1500;
      }
      if (strstr(object_list[count].name, "Moonstone") != NULL) {
        object_list[count].cost += 3000;
      }
      if (strstr(object_list[count].name, "Neptunite") != NULL) {
        object_list[count].cost += 1000;
      }
      if (strstr(object_list[count].name, "Obsidian") != NULL) {
        object_list[count].cost += 2500;
      }
      if (strstr(object_list[count].name, "Onyx") != NULL) {
        object_list[count].cost += 1500;
      }
      if (strstr(object_list[count].name, "Opal") != NULL) {
        object_list[count].cost += 1000;
      }
      if (strstr(object_list[count].name, "Pearl") != NULL) {
        object_list[count].cost += 11500;
      }
      if (strstr(object_list[count].name, "Pyrite") != NULL) {
        object_list[count].cost += 1000;
      }
      if (strstr(object_list[count].name, "Quartz") != NULL) {
        object_list[count].cost += 1000;
      }
      if (strstr(object_list[count].name, "Quartzite") != NULL) {
        object_list[count].cost += 1500;
      }
      if (strstr(object_list[count].name, "Rhodonite") != NULL) {
        object_list[count].cost += 1000;
      }
      if (strstr(object_list[count].name, "Rhyolite") != NULL) {
        object_list[count].cost += 1000;
      }
      if (strstr(object_list[count].name, "Ruby") != NULL) {
        object_list[count].cost += 14500;
      }
      if (strstr(object_list[count].name, "Sapphire") != NULL) {
        object_list[count].cost += 14500;
      }
      if (strstr(object_list[count].name, "Sphalerite") != NULL) {
        object_list[count].cost += 1000;
      }
      if (strstr(object_list[count].name, "Staurolite") != NULL) {
        object_list[count].cost += 1000;
      }
      if (strstr(object_list[count].name, "Tiger eye") != NULL) {
        object_list[count].cost += 8500;
      }
      if (strstr(object_list[count].name, "Topaz") != NULL) {
        object_list[count].cost += 1000;
      }
      if (strstr(object_list[count].name, "Turquoise") != NULL) {
        object_list[count].cost += 3000;
      }
      if (strstr(object_list[count].name, "Zircon") != NULL) {
        object_list[count].cost += 1000;
      }
    }
  }

  return true;
}

bool init__treasures(void) {
  if (!sort_treasures()) return false;
  if (!init_t_level()) return false;
  if (!adjust_prices()) return false;
  if (!adjust_weights()) return false;
  if (!adjust_gem_values()) return false;
  return true;
}
