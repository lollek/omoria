#include "../variables.h"
#include "../player.h"
#include "../misc.h"
#include "../desc.h"
#include "../pascal.h"

void player_action_look() {
  /*{ Look at an object, trap, or monster                   -RAK-   }*/
  /*{ Note: Looking is a free move, see where invoked...            }*/

  long i1;
  long i2;
  long y;
  long x;
  long dir;
  long dummy;
  boolean flag = false;
  char out_val[82];
  char out_val2[120];

  y = char_row;
  x = char_col;

  if (!d__get_dir("Look which direction?", &dir, &dummy, &y, &x)) {
    return;
  }

  if (player_flags.blind >= 1) {
    msg_print("You can't see a damn thing!");
    return;
  }

  y = char_row;
  x = char_col;
  i1 = 0;
  do {
    move_dir(dir, &y, &x);
    /* with cave[y][x]. do; */
    if (cave[y][x].cptr > 1) {
      if (m_list[cave[y][x].cptr].ml) {
        i2 = m_list[cave[y][x].cptr].mptr;
        if (is_vowel(c_list[i2].name[0])) {
          sprintf(out_val, "You see an %s.", c_list[i2].name);
        } else {
          sprintf(out_val, "You see a %s.", c_list[i2].name);
        }
        msg_print(out_val);
        flag = true;
      }
    }

    if ((cave[y][x].tl) || (cave[y][x].pl) || (cave[y][x].fm)) {
      if (cave[y][x].tptr > 0) {
        if (t_list[cave[y][x].tptr].tval == secret_door) {
          msg_print("You see a granite wall.");
        } else if (t_list[cave[y][x].tptr].tval != unseen_trap) {
          inven_temp.data = t_list[cave[y][x].tptr];
          inven_temp.data.number = 1;
          objdes(out_val, &inven_temp, true);
          sprintf(out_val2, "You see %s.", out_val);
          msg_print(out_val2);
          flag = true;
        }
      }

      if (!cave[y][x].fopen) {
        flag = true;
        switch (cave[y][x].fval) {
        case 10:
          msg_print("You see a granite wall.");
          break;
        case 11:
          msg_print("You see some dark rock.");
          break;
        case 12:
          msg_print("You see a quartz vein.");
          break;
        case 15:
          msg_print("You see a granite wall.");
          break;
        default:
          break;
        }
      } else {
        switch (cave[y][x].fval) {
        case 16:
        case 17:
          flag = true;
          msg_print("You see some water.");
          break;
        default:
          break;
        }
      }
    }

    i1++;
  } while (!(((!cave[y][x].fopen) || (i1 > MAX_SIGHT))));

  if (!flag) {
    msg_print("You see nothing of interest in that direction.");
  }
}