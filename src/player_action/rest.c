#include <string.h>

#include "../player.h"
#include "../screen.h"
#include "../variables.h"

void player_action_rest() {

  long rest_num;
  char rest_str[82];

  prt("Rest for how long (or *) ? ", 1, 1);
  get_string(rest_str, 1, 28, 10);

  if (!strcmp(rest_str, "*")) {
    rest_num = 20;
    (player_flags).resting_till_full = true;
  } else {
    rest_num = 0;
    sscanf(rest_str, "%ld", &rest_num);
  }

  if (rest_num > 0) {
    if (search_flag) {
      search_off();
    }
    player_flags.rest = rest_num;
    turn_counter += rest_num;
    player_flags.status |= IS_RESTING;
    prt_rest();
    msg_print("Press any key to wake up...");
    refresh();
  } else {
    erase_line(msg_line, msg_line);
    reset_flag = true;
  }
}