#include "../io.h"
#include "../misc.h"
#include "../player_action.h"
#include "../variables.h"

void player_action_tunnel(void) {
  long y, x, i1;

  y = char_row;
  x = char_col;

  if (d__get_dir("Which direction?", &i1, &i1, &y, &x)) {
    C_player_action_tunnel_target(y, x);
  }
}
