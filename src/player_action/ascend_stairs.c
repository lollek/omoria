#include "../random.h"
#include "../variables.h"

void player_action_ascend_stairs(void) {
  if (cave[char_row][char_col].tptr > 0) {
    if (t_list[cave[char_row][char_col].tptr].tval == up_staircase) {
      dun_level--;
      moria_flag = true;
      msg_print("You enter a maze of up staircases.");
      msg_print("You pass through a one-way door.");
    } else if (t_list[cave[char_row][char_col].tptr].tval ==
               up_steep_staircase) {
      dun_level -= randint(3) + 1;
      if (dun_level < 0) {
        dun_level = 0;
      }
      moria_flag = true;
      msg_print("You enter a long maze of up staircases.");
      msg_print("You pass through a one-way door.");
    } else {
      msg_print("I see no up staircase here.");
    }
  } else {
    msg_print("I see no up staircase here.");
  }
}