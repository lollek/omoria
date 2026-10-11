#include "../io.h"
#include "../misc.h"
#include "../player.h"
#include "../player_action.h"
#include "../variables.h"

/**
 * -RAK-
 *  d__bash() - Bash open a door or chest
 * Note: Affected by strength and weight of character
 */
void player_action_bash(void) {

  long y = char_row;
  long x = char_col;
  long tmp;

  if (search_flag) {
    search_off();
  }
  if (player_flags.rest > 0) {
    rest_off();
  }

  if (d__get_dir("Which direction?", &tmp, &tmp, &y, &x)) {
    C_player_action_bash_target(y, x);
  }
}
