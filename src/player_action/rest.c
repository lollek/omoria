#include "../io.h"
#include "../player_action.h"
#include "../screen.h"
#include "../variables.h"

void player_action_rest(void) {

  char rest_str[82];

  prt("Rest for how long (or *) ? ", 1, 1);
  if (!get_string(rest_str, 1, 28, 10)) {
    rest_str[0] = '\0';
  }

  if (C_player_action_rest_input(rest_str)) {
    prt_rest();
    msg_print("Press any key to wake up...");
    refresh();
  } else {
    erase_line(msg_line, msg_line);
  }
}
