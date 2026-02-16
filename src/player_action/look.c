#include "../floor.h"
#include "../io.h"
#include "../misc.h"
#include "../monsters.h"
#include "../pascal.h"
#include "../player.h"
#include "../text_lines.h"
#include "../variables.h"

void player_action_look(void) {
  /*{ Look at an object, trap, or monster                   -RAK-   }*/
  /*{ Note: Looking is a free move, see where invoked...            }*/

  long y = char_row;
  long x = char_col;

  long dummy;
  long dir;
  if (!d__get_dir("Look which direction?", &dir, &dummy, &y, &x)) {
    return;
  }

  if (player_flags.blind >= 1) {
    msg_print("You can't see a damn thing!");
    return;
  }

  y = char_row;
  x = char_col;
  bool something_has_been_seen = false;
  long i = 0;
  do {
    move_dir(dir, &y, &x);
    const uint8_t cptr = cave[y][x].cptr;
    if (cptr > 1) {
      if (m_list[cptr].is_seen) {
        const long mptr = m_list[cptr].mptr;
        char const * const monster_name = get_monster_name(mptr);
        if (is_vowel(monster_name[0])) {
          msg_printf("You see an %s.", monster_name);
        } else {
          msg_printf("You see a %s.", monster_name);
        }
        something_has_been_seen = true;
      }
    }

    if (cave[y][x].tl || cave[y][x].pl || cave[y][x].fm) {
      if (cave[y][x].tptr > 0) {
        if (t_list[cave[y][x].tptr].tval == secret_door) {
          msg_print("You see a granite wall.");
        } else if (t_list[cave[y][x].tptr].tval != unseen_trap) {
          char out_val[82];
          inven_temp.data = t_list[cave[y][x].tptr];
          inven_temp.data.number = 1;
          item_name(out_val, &inven_temp);
          msg_printf("You see %s %s.", is_vowel(out_val[0]) ? "an" : "a", out_val);
          something_has_been_seen = true;
        }
      }

      if (!cave[y][x].fopen) {
        something_has_been_seen = true;
        switch (cave[y][x].fval) {
        case ft_wall_granite:
          msg_print("You see a granite wall.");
          break;
        case ft_wall_magma:
          msg_print("You see some dark rock.");
          break;
        case ft_wall_quartz:
          msg_print("You see a quartz vein.");
          break;
        case ft_boundry_wall:
          msg_print("You see a granite wall.");
          break;
        default:
          break;
        }
      } else {
        switch (cave[y][x].fval) {
        case ft_water_on_floor:
        case ft_water_on_room_floor:
          something_has_been_seen = true;
          msg_print("You see some water.");
          break;
        default:
          break;
        }
      }
    }

    i++;
  } while (cave[y][x].fopen && i <= MAX_SIGHT);

  if (!something_has_been_seen) {
    msg_print("You see nothing of interest in that direction.");
  }
}
