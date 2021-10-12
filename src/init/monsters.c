#include <stdbool.h>
#include <stdio.h>
#include <string.h>

#include "../configure.h"
#include "../debug.h"
#include "../variables.h"

#define DEBUG_MONSTERS 0

#if DEBUG_MONSTERS
static void print_creature(creature_type *c, int c_num, int style) {
  printf("--- %03d ---\n\r", c_num);

  if (style) {
    printf("%d\n\r%d\n\r%s\n\r%08lx\n\r%08lx\n\r", c->aaf, c->ac, c->name,
           c->cmove, c->spells);

    printf("%04lx\n\r%d\n\r%ld\n\r%d\n\r%c\n\r", c->cdefense, c->sleep, c->mexp,
           c->speed, c->cchar);

    printf("%s\n\r%s\n\r%d\n\r%d\n\r", c->hd, c->damage, c->level, c->mr);

  } else {
    printf("aaf: %d\n\rac: %d\n\rname: %s\n\rcmove: "
           "%08lx\n\rspells: %08lx\n\r",
           c->aaf, c->ac, c->name, c->cmove, c->spells);

    printf("cdefense: %08lx\n\rsleep: %d\n\rmexp: %ld\n\rspeed: "
           "%d\n\rcchar: %c\n\r",
           c->cdefense, c->sleep, c->mexp, c->speed, c->cchar);

    printf("hd: %s\n\rdamage: %s\n\rlevel: %d\n\rmr: %d\n\r", c->hd, c->damage,
           c->level, c->mr);
  }
}
#endif


static void print_error(char const *msg) {
  fprintf(stderr, "There was an error parsing the monsters file. "
      "%s"
      "Will break and not attempt to parse any more monsters.\n",
      msg);
}

/**
 * parse_header() - Reads and verified the monster header
 * @file: File to read from
 *
 * @returns
 * true if there is a monster to parse
 * false if there is no monster to parse
 */
static bool parse_header(FILE *file) {
  int current_monster_number;
  switch (fscanf(file, "--- %d ---\n", &current_monster_number)) {
    case 0:
      print_error("Expected to find header");
      return false;
    case EOF:
      return false;
    default:
      return true;
  }
}

/**
 * parse_aaf() - Reads and parses monster value
 * @file: File to read from
 * @monster: Creature to write data to
 *
 * @returns
 * true on success
 * false on error
 */
static bool parse_aaf(FILE *file, creature_type *monster) {
  unsigned int value;
  switch (fscanf(file, "%u\n", &value)) {
    case 0: case EOF:
      print_error("Expected to find an aaf-value (number)");
      return false;
    default:
      monster->aaf = (uint8_t)value;
      return true;
  }
}

/**
 * parse_ac() - Reads and parses monster value
 * @file: File to read from
 * @monster: Creature to write data to
 *
 * @returns
 * true on success
 * false on error
 */
static bool parse_ac(FILE *file, creature_type *monster) {
  int value;
  switch (fscanf(file, "%d\n", &value)) {
    case 0: case EOF:
      print_error("Expected to find an ac-value (number)");
      return false;
    default:
      monster->ac = (int16_t)value;
      return true;
  }
}

/**
 * parse_cmove() - Reads and parses monster value
 * @file: File to read from
 * @monster: Creature to write data to
 *
 * @returns
 * true on success
 * false on error
 */
static bool parse_cmove(FILE *file, creature_type *monster) {
  unsigned long value;
  switch (fscanf(file, "%lx\n", &value)) {
    case 0: case EOF:
      print_error("Expected to find a cmove-value (hex field)");
      return false;
    default:
      monster->cmove = (uint64_t)value;
      return true;
  }
}

/**
 * parse_spells() - Reads and parses monster value
 * @file: File to read from
 * @monster: Creature to write data to
 *
 * @returns
 * true on success
 * false on error
 */
static bool parse_spells(FILE *file, creature_type *monster) {
  unsigned long value;
  switch (fscanf(file, "%lx\n", &value)) {
    case 0: case EOF:
      print_error("Expected to find a spells-value (hex field)");
      return false;
    default:
      monster->spells = (uint64_t)value;
      return true;
  }
}

/**
 * parse_cdefense() - Reads and parses monster value
 * @file: File to read from
 * @monster: Creature to write data to
 *
 * @returns
 * true on success
 * false on error
 */
static bool parse_cdefense(FILE *file, creature_type *monster) {
  unsigned long value;
  switch (fscanf(file, "%lx\n", &value)) {
    case 0: case EOF:
      print_error("Expected to find a cdefense-value (hex field)");
      return false;
    default:
      monster->cdefense = (uint64_t)value;
      return true;
  }
}

/**
 * parse_sleep() - Reads and parses monster value
 * @file: File to read from
 * @monster: Creature to write data to
 *
 * @returns
 * true on success
 * false on error
 */
static bool parse_sleep(FILE *file, creature_type *monster) {
  int value;
  switch (fscanf(file, "%d\n", &value)) {
    case 0: case EOF:
      print_error("Expected to find an sleep-value (number)");
      return false;
    default:
      monster->sleep = (int16_t)value;
      return true;
  }
}

/**
 * parse_mexp() - Reads and parses monster value
 * @file: File to read from
 * @monster: Creature to write data to
 *
 * @returns
 * true on success
 * false on error
 */
static bool parse_mexp(FILE *file, creature_type *monster) {
  long value;
  switch (fscanf(file, "%ld\n", &value)) {
    case 0: case EOF:
      print_error("Expected to find an mexp-value (number)");
      return false;
    default:
      monster->mexp = (int64_t)value;
      return true;
  }
}

/**
 * parse_speed() - Reads and parses monster value
 * @file: File to read from
 * @monster: Creature to write data to
 *
 * @returns
 * true on success
 * false on error
 */
static bool parse_speed(FILE *file, creature_type *monster) {
  long value;
  switch (fscanf(file, "%ld\n", &value)) {
    case 0: case EOF:
      print_error("Expected to find an speed-value (number)");
      return false;
    default:
      monster->speed = (int64_t)value;
      return true;
  }
}

/**
 * parse_cchar() - Reads and parses monster value
 * @file: File to read from
 * @monster: Creature to write data to
 *
 * @returns
 * true on success
 * false on error
 */
static bool parse_cchar(FILE *file, creature_type *monster) {
  switch (fscanf(file, "%c\n", &monster->cchar)) {
    case 0: case EOF:
      print_error("Expected to find an cchar-value (character)");
      return false;
    default:
      return true;
  }
}

/**
 * parse_string() - Reads and parses monster value
 * @file: File to read from
 * @monster: Creature to write data to
 *
 * @returns
 * true on success
 * false on error
 */
static bool parse_string(FILE *file, char *string, size_t string_size) {

  if (fgets(string, string_size, file) == NULL) {
      print_error("Expected to find a string-value");
    return false;
  }

  // Strip newline
  string[strlen(string) - 1] = 0;
  return true;
}

/**
 * parse_level() - Reads and parses monster value
 * @file: File to read from
 * @monster: Creature to write data to
 *
 * @returns
 * true on success
 * false on error
 */
static bool parse_level(FILE *file, creature_type *monster) {
  int value;
  switch (fscanf(file, "%d\n", &value)) {
    case 0: case EOF:
      print_error("Expected to find a level-value (number)");
      return false;
    default:
      monster->level = (int8_t)value;
      return true;
  }
}

/**
 * parse_mr() - Reads and parses monster value
 * @file: File to read from
 * @monster: Creature to write data to
 *
 * @returns
 * true on success
 * false on error
 */
static bool parse_mr(FILE *file, creature_type *monster) {
  unsigned int value;
  switch (fscanf(file, "%u\n", &value)) {
    case 0: case EOF:
      print_error("Expected to find a mr-value (number)");
      return false;
    default:
      monster->mr = (uint8_t)value;
      return true;
  }
}

bool load_monsters(void) {
  ENTER(("load_monsters", ""));

  /* Parse MORIA_MON-file and create monster types. The format looks like below.
   * More information is in monsters.info */

  FILE *file = fopen(MONSTERS_FILE, "r");
  if (file == NULL) {
    fprintf(stderr, "Unable to open monster file for reading: %s\n", MONSTERS_FILE);
    return false;
  }


  long num_monsters_loaded = 0;
  for (int i = 1; i <= MAX_CREATURES; i++) {

    creature_type *monster = &c_list[i];

    if (!parse_header(file)) break;
    if (!parse_aaf(file, monster)) break;
    if (!parse_ac(file, monster)) break;
    if (!parse_string(file, monster->name, sizeof(monster->name))) break;
    if (!parse_cmove(file, monster)) break;
    if (!parse_spells(file, monster)) break;
    if (!parse_cdefense(file, monster)) break;
    if (!parse_sleep(file, monster)) break;
    if (!parse_mexp(file, monster)) break;
    if (!parse_speed(file, monster)) break;
    if (!parse_cchar(file, monster)) break;
    if (!parse_string(file, monster->hd, sizeof(monster->hd))) break;
    if (!parse_string(file, monster->damage, sizeof(monster->damage))) break;
    if (!parse_level(file, monster)) break;
    if (!parse_mr(file, monster)) break;

    num_monsters_loaded++;
#if DEBUG_MONSTERS
    print_creature(monster, i, 1);
#endif
  }

  fclose(file);
  MSG(("load_monsters: loaded %d of max %d", num_monsters_loaded, MAX_CREATURES));

  LEAVE("load_monsters", "");
  return true;
}

/**
 * init_m_level() - Initializes M_LEVEL array for use with PLACE_MONSTER
 * */
static bool init_m_level(void) {
  int i1 = 1;
  int i2 = 0;
  int i3 = MAX_CREATURES - WIN_MON_TOT;

  ENTER(("init_m_level", ""));

  do {
    m_level[i2] = 0;
    while ((i1 <= i3) && (c_list[i1].level == i2)) {
      m_level[i2]++;
      i1++;
    }
    i2++;
  } while (i2 <= MAX_MONS_LEVEL);

  for (i1 = 2; i1 <= MAX_MONS_LEVEL; i1++) {
    m_level[i1] += m_level[i1 - 1];
  }

  /*  for (i1 = 0; i1 < MAX_MONS_LEVEL+1; i1++) { */
  /*    printf ("\n m_level[%d] : %d",i1,m_level[i1]);  fflush(stdout); */
  /*  } */
  LEAVE("init_m_level", "");
  return true;
}


bool init__monsters(void) {
  if (!load_monsters()) return false;
  if (!init_m_level()) return false;
  return true;
}
