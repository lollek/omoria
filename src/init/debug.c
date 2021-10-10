#include "../debug.h"

#include "debug.h"

bool init__debug(void) {
#if DO_DEBUG
  debug_file = fopen("debug.out", "w");
  if (debug_file == NULL) {
    fprintf(stderr, "Failed to init debug file!\n");
    return false;
  }
#endif

  return true;
}
