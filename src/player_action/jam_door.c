#include "../io.h"
#include "../misc.h"
#include "../player_action.h"
#include "../variables.h"

void player_action_jam_door(void) {

  long y = char_row;
  long x = char_col;
  long tmp;

  if (d__get_dir("Which direction?", &tmp, &tmp, &y, &x)) {
    C_player_action_jam_door_target(y, x);
  }
}
