#include <string.h>

#include "../misc.h"
#include "../screen.h"
#include "../variables.h"

void player_action_close(void) {

  long y, x, tmp;
  char m_name[82];

  y = char_row;
  x = char_col;

  if (d__get_dir("Which direction?", &tmp, &tmp, &y, &x)) {
    /* with cave[y][x]. do; */
    if (cave[y][x].tptr > 0) {
      if (t_list[cave[y][x].tptr].tval == open_door) {
        if (cave[y][x].cptr == 0) {
          if (t_list[cave[y][x].tptr].p1 == 0) {
            t_list[cave[y][x].tptr] = door_list[1];
            cave[y][x].fopen = false;
            lite_spot(y, x);
          } else {
            msg_print("The door appears to "
                      "be broken.");
          }
        } else {
          find_monster_name(m_name, cave[y][x].cptr, true);
          strcat(m_name, " is in your way!");
          msg_print(m_name);
        }
      } else {
        msg_print("I do not see anything you can close "
                  "there.");
      }
    } else {
      msg_print("I do not see anything you can close there.");
    }
  }
}