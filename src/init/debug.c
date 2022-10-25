#include "../debug.h"

#include "debug.h"

void debug_init(); // Rust part

bool init__debug(void) {
#if DO_DEBUG
  debug_file = fopen("debug.out", "w");
  if (debug_file == NULL) {
    fprintf(stderr, "Failed to init debug file!\n");
    return false;
  }

  debug_init();
#endif

  return true;
}
