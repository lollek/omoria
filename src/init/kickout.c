#include <string.h>

#include "../configure.h"
#include "../kickout.h"

#include "kickout.h"

static bool operating_hours_file_exists(void) {
  FILE *file = fopen(OPERATING_HOURS_FILE, "r");
  if (file != NULL) {
    fclose(file);
    return true;
  }

  return false;
}

static bool create_operating_hours_file(void) {
    FILE *file = fopen(OPERATING_HOURS_FILE, "w");
    if (file == NULL) {
      fprintf(stderr, "Failed to create %s\n", OPERATING_HOURS_FILE);
      return false;
    }

    kick__dump_operating_hours_to_file(file);
    fclose(file);

    return true;
}

/**
 * init_operating_hours() - Parse the operating hours file and init the data
 *
 * This function will also create the file if it's missing
 */
static bool init_operating_hours(void) {
  if (!operating_hours_file_exists()) {
    if (!create_operating_hours_file()) return false;
  }

  FILE *file = fopen(OPERATING_HOURS_FILE, "r");
  if (file == NULL) {
    fprintf(stderr, "Failed to load %s\n", OPERATING_HOURS_FILE);
    return false;
  }

  char buf[80];

  // Discard the first four lines
  for (int i = 0; i < 4; ++i) {
    if (fgets(buf, sizeof(buf), file) == NULL) {
      fprintf(stderr, "Error while parsing %s!\n", OPERATING_HOURS_FILE);
      fclose(file);
      return false;
    }
  }

  // Read the seven weekdays, ordered Sun -> Mon -> Tue -> ...
  for (int i = 0; i < 7; ++i) {
    if (fgets(buf, sizeof(buf), file) == NULL) {
      fprintf(stderr, "Error while parsing %s!\n", OPERATING_HOURS_FILE);
      fclose(file);
      return false;
    }

    const long x = strlen(buf);
    if (x && buf[x - 1] == '\n') {
      buf[x - 1] = 0;
    }
    strncpy(operating_hours[i], buf, sizeof(operating_hours[i]));
  }

  return true;
}

bool init__kickout(void) {
  if (!init_operating_hours()) return false;
  return true;
}
