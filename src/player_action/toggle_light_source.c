#include "../dungeon/light.h"
#include "../io.h"
#include "../player.h"
#include "../types.h"
#include "../screen.h"
#include "../variables.h"

void player_action_toggle_light_source(void) {
  char out_val[80];

  reset_flag = true;

  if (equipment[Equipment_light].tval <= 0) {
    msg_print("You are not carrying a light.");
    return;
  }

  if (equipment[Equipment_light].p1 <= 0) {
    msg_print("Your light has gone out!");
    return;
  }

  if (player_flags.light_on) {
    sprintf(out_val, "Light Off.  %lld turns left.",
            equipment[Equipment_light].p1);
    player_flags.light_on = false;
    player_light = false;
  } else {
    sprintf(out_val, "Light On.  %lld turns left.",
            equipment[Equipment_light].p1);
    player_flags.light_on = true;
    player_light = true;
  }
  prt_light_on();
  msg_print(out_val);
  dungeon_light_move(char_row, char_col, char_row, char_col);
}
