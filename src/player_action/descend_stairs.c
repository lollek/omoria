#include "../io.h"
#include "../random.h"
#include "../variables.h"

#include <stdbool.h>

void player_action_descend_stairs(void) {
  if (cave[char_row][char_col].tptr > 0) {
    if (t_list[cave[char_row][char_col].tptr].tval == down_staircase) {
      dun_level++;
      moria_flag = true;
      msg_print("You enter a maze of down staircases.");
      msg_print("You pass through a one-way door.");
    } else if (t_list[cave[char_row][char_col].tptr].tval ==
               down_steep_staircase) {
      dun_level += randint(3) + 1;
      moria_flag = true;
      msg_print("You enter a long maze of down staircases.");
      msg_print("You pass through a one-way door.");
    } else {
      msg_print("I see no down staircase here.");
    }
  } else {
    msg_print("I see no down staircase here.");
  }
}
