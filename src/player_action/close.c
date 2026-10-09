#include "../io.h"
#include "../misc.h"
#include "../player_action.h"
#include "../variables.h"

void player_action_close(void) {

  long y, x, tmp;

  y = char_row;
  x = char_col;

  if (d__get_dir("Which direction?", &tmp, &tmp, &y, &x)) {
    C_player_action_close_target(y, x);
  }
}
