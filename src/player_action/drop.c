#include "../inven.h"
#include "../io.h"
#include "../player.h"
#include "../screen.h"
#include "../text_lines.h"
#include "../variables.h"

#include <stdbool.h>

void player_action_drop(void) {
  treas_rec *com_ptr;
  bool redraw;
  char trash_char;

  reset_flag = true;

  /* with player_do; */
  const long temp = player_money[6] + player_money[5] + player_money[4] +
              player_money[3] + player_money[2] + player_money[1];

  if (inven_ctr > 0 || temp > 0) {
    long count = inventory_change_all_ok_stats(true, false);
    com_ptr = inventory_list;
    while (com_ptr != NULL) {
      if (com_ptr->data.tval == bag_or_sack && com_ptr->insides != 0) {
        com_ptr->ok = false;
        count--;
      }
      com_ptr = com_ptr->next;
    }

    /*{Someone said that it always redraws when drop}*/
    redraw = false;

    if (get_item(&com_ptr, "Which one? ", &redraw, count, &trash_char, true,
                 false)) {
      if (redraw) {
        draw_cave();
      }
      /* with cave[char_row][char_col]. do; */
      if (cave[char_row][char_col].tptr > 0) {
        msg_print("There is something there already.");
      } else {
        char out_val2[120];
        char out_val[82];
        if (trash_char == '$') {
          inven_drop(com_ptr, char_row, char_col, true);
        } else {
          inven_drop(com_ptr, char_row, char_col, false);
        }
        prt_stat_block();
        objdes(out_val, &inven_temp, true);
        sprintf(out_val2, "Dropped %s.", out_val);
        msg_print(out_val2);
        reset_flag = false;
      }
    } else if (redraw) {
      draw_cave();
    }
  } else {
    msg_print("You are not carrying anything.");
  }
}
