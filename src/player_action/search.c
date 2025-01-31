#include <math.h>
#include <time.h>

#include "../io.h"
#include "../misc.h"
#include "../pascal.h"
#include "../player.h"
#include "../random.h"
#include "../text_lines.h"
#include "../traps.h"
#include "../variables.h"

#include "search.h"

void player_action_search(const long player_y, const long player_x, long chance) {
  if (player_flags.blind > 0) {
    msg_print("You are incapable of searching while blind.");
    return;
  }

  if (player_flags.confused + player_flags.blind > 0) {
    chance = trunc(chance / 10.0);
  } else if (player_has_no_light()) {
    chance = (long)(chance / 5.0);
  }

  for (long y = player_y - 1; y <= player_y + 1; y++) {
    for (long x = player_x - 1; x <= player_x + 1; x++) {
      if (!in_bounds(y, x)) {
        continue;
      }
      if (y == player_y && x == player_x) {
        // There can be no unfound thing where the
        // player is standing?
        continue;
      }
      if (randint(100) >= chance) {
        continue;
      }

      // Search for hidden objects
      if (cave[y][x].tptr <= 0) {
        continue;
      }

      if (t_list[cave[y][x].tptr].tval == unseen_trap) {
        // Trap on floor
        char out_val[86];
        sprintf(out_val, "You have found %s.", t_list[cave[y][x].tptr].name);
        msg_print(out_val);
        change_trap(y, x);
        find_flag = false;

      } else if (t_list[cave[y][x].tptr].tval == secret_door) {
        // Secret door
        msg_print("You have found a secret door.");
        cave[y][x].fval = corr_floor2.ftval;
        change_trap(y, x);
        find_flag = false;

      } else if (t_list[cave[y][x].tptr].tval == chest) {
        if (t_list[cave[y][x].tptr].flags > 1) {
          if (pindex(t_list[cave[y][x].tptr].name, '^') > 0) {
            // Chest is trapped
            known2(t_list[cave[y][x].tptr].name);
            msg_print("You have discovered a trap on the chest!");
          }
        }
      }
    }
  }
}
