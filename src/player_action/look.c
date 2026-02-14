#include "../io.h"
#include "../misc.h"
#include "../pascal.h"
#include "../player.h"
#include "../text_lines.h"
#include "../variables.h"

void player_action_look(void) {
  /*{ Look at an object, trap, or monster                   -RAK-   }*/
  /*{ Note: Looking is a free move, see where invoked...            }*/

  long y;
  long x;
  long dir;
  long dummy;
  bool flag = false;

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
  long i1 = 0;
  do {
    char out_val[82];
    move_dir(dir, &y, &x);
    /* with cave[y][x]. do; */
    if (cave[y][x].cptr > 1) {
      if (m_list[cave[y][x].cptr].is_seen) {
        const long i2 = m_list[cave[y][x].cptr].mptr;
        if (is_vowel(monster_templates[i2].name[0])) {
          sprintf(out_val, "You see an %s.", monster_templates[i2].name);
        } else {
          sprintf(out_val, "You see a %s.", monster_templates[i2].name);
        }
        msg_print(out_val);
        flag = true;
      }
    }

    if (cave[y][x].tl || cave[y][x].pl || cave[y][x].fm) {
      if (cave[y][x].tptr > 0) {
        if (t_list[cave[y][x].tptr].tval == secret_door) {
          msg_print("You see a granite wall.");
        } else if (t_list[cave[y][x].tptr].tval != unseen_trap) {
          inven_temp.data = t_list[cave[y][x].tptr];
          inven_temp.data.number = 1;
          item_name(out_val, &inven_temp);
          msg_printf("You see %s %s.", is_vowel(out_val[0]) ? "an" : "a", out_val);
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
  } while (!(!cave[y][x].fopen || i1 > MAX_SIGHT));

  if (!flag) {
    msg_print("You see nothing of interest in that direction.");
  }
}
