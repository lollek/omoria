#include "../io.h"
#include "../misc.h"
#include "../player_action.h"
#include "../variables.h"

void player_action_look(void) {
  long y = char_row;
  long x = char_col;
  long dummy;
  long dir;
  if (d__get_dir("Look which direction?", &dir, &dummy, &y, &x)) {
    C_player_action_look_direction(dir);
  }
}
