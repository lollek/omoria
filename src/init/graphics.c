#include "../graphics.h"
#include "../ncurses_utils.h"
#include "../io.h"

#include "graphics.h"

bool init__graphics(void) {
  C_init_curses();
  curses_is_running = true;
  no_controly();

  return true;
}
