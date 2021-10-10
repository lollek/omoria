#include "../routines.h"
#include "../graphics.h"

#include "graphics.h"

bool init__graphics(void) {
  C_init_curses();
  curses_is_running = true;
  no_controly();

  return true;
}
