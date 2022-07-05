#include <stdlib.h>
#include <string.h>

#include "../types.h"
#include "../player.h"
#include "../variables.h"
#include "../misc.h"
#include "../inven.h"
#include "../screen.h"

void player_action_jam_door() {

  treas_rec *i1;
  long y = char_row;
  long x = char_col;
  long i2;
  long tmp;
  char m_name[82];
  obj_set pick_a_spike = {spike, 0};

  if (!d__get_dir("Which direction?", &tmp, &tmp, &y, &x)) {
    return;
  }

  if (cave[y][x].tptr <= 0) {
    msg_print("That isn't a door!");
    return;
  }

  if (t_list[cave[y][x].tptr].tval == closed_door) {
    if (cave[y][x].cptr == 0) {
      if (find_range(pick_a_spike, false, &i1, &i2)) {
        msg_print("You jam the door with a spike.");
        if (i1->data.number > 1) {
          i1->data.number--;
        } else {
          inven_destroy(i1);
        }
        prt_stat_block();
        t_list[cave[y][x].tptr].p1 = -labs(t_list[cave[y][x].tptr].p1) - 20;
      } else {
        msg_print("But you have no spikes...");
      }
    } else {
      find_monster_name(m_name, cave[y][x].cptr, true);
      strcat(m_name, " is in your way!");
      msg_print(m_name);
    }
  } else if (t_list[cave[y][x].tptr].tval == open_door) {
    msg_print("The door must be closed first.");
  } else {
    msg_print("That isn't a door!");
  }
}