#include "../io.h"
#include "../misc.h"
#include "../player_action.h"
#include "../variables.h"

void player_action_disarm_trap(void) {
  long y = char_row;
  long x = char_col;
  long direction, unused;

  if (d__get_dir("Which direction?", &direction, &unused, &y, &x)) {
    C_player_action_disarm_target(y, x, direction);
  }
}
