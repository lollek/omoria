#include <stdio.h>

#include "../configure.h"

#include "death.h"

static bool death_file_exists(void) {
  FILE *file = fopen(DEATH_FILE, "r");
  if (file != NULL) {
    fclose(file);
    return true;
  }

  return false;
}

static bool create_death_file(void) {
  FILE *file = fopen(DEATH_FILE, "w");
  if (file == NULL) {
    fprintf(stderr, "Failed to create %s\n", DEATH_FILE);
    return false;
  }

  fprintf(file, "Moria death log file\n");
  fprintf(file, "\n");
  fprintf(file, "Key to abbreviations:\n");
  fprintf(file, "\n");
  fprintf(file, "For Race:          For Class:\n");
  fprintf(file, "  1 = Human          1 = Warrior\n");
  fprintf(file, "  2 = Half-Elf       2 = Mage\n");
  fprintf(file, "  3 = Elf            3 = Priest\n");
  fprintf(file, "  4 = Halfling       4 = Rogue\n");
  fprintf(file, "  5 = Gnome          5 = Ranger\n");
  fprintf(file, "  6 = Dwarf          6 = Paladin\n");
  fprintf(file, "  7 = Half-Orc       7 = Druid\n");
  fprintf(file, "  8 = Half-Troll     8 = Bard\n");
  fprintf(file, "  9 = Phraint        9 = Adventurer\n");
  fprintf(file, " 10 = Dryad         10 = Monk\n");
  fprintf(file, "\n");
  fprintf(file, "             Dif Class   Dung Dung\n");
  fprintf(file, " Username      Race  Lvl Cur  Max  Died from\n");
  fprintf(file, " ------------ - -- -- -- ---- ---- "
      "---------------------------------------"
      "-----\n");
  fclose(file);

  return true;
}

static bool init_death_file(void) {
  if (!death_file_exists()) {
    if (!create_death_file()) return false;
  }

  return true;
}

bool init__death(void) {
  if (!init_death_file()) return false;
  return true;
}
