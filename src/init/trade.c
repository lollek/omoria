#include <stdio.h>

#include "../configure.h"

#include "trade.h"

static bool trade_file_exists(void) {
  FILE *file = fopen(TRADE_FILE, "r");
  if (file != NULL) {
    fclose(file);
    return true;
  }

  return false;
}

static bool create_trade_file(void) {
  FILE *file = fopen(DEATH_FILE, "w");
  if (file == NULL) {
    fprintf(stderr, "Failed to create %s\n", TRADE_FILE);
    return false;
  }

  fclose(file);

  return true;
}

static bool init_trade_file(void) {
  if (trade_file_exists()) {
    if (!create_trade_file()) return false;
  }

  return true;
}

bool init__trade(void) {
  if (!init_trade_file()) return false;
  return true;
}

