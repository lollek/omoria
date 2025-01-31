#include <string.h>

#include "../io.h"
#include "../misc.h"
#include "../player.h"
#include "../player_action.h"
#include "../random.h"
#include "../screen.h"
#include "../text_lines.h"
#include "../traps.h"
#include "../variables.h"

void player_action_disarm_trap(void) {
  long y, x, i1, tdir;

  y = char_row;
  x = char_col;

  if (d__get_dir("Which direction?", &tdir, &i1, &y, &x)) {
    /* with cave[y][x]. do; */
    if (cave[y][x].tptr > 0) {
      const long t1 = player_disarm;                  /*{ Ability to disarm     }*/
      const long t2 = player_lev;                     /*{ Level adjustment      }*/
      const long t3 = 2 * C_player_disarm_from_dex(); /*{ Dexterity
                                              adjustment  }*/
      const long t4 = C_player_mod_from_stat(INT);    /*{ Intelligence adjustment}*/
      long tot = t1 + t2 + t3 + t4;

      if (player_flags.blind > 0) {
        tot /= 5;
      } else if (player_has_no_light()) {
        tot /= 2;
      }

      if (player_flags.confused > 0) {
        tot /= 3;
      }

      i1 = t_list[cave[y][x].tptr].tval;
      const long t5 = t_list[cave[y][x].tptr].level;

      if (i1 == seen_trap) { /* { Floor trap    } */
        /* with t_list[cave[y][x].tptr]. do; */
        if (tot - t5 > randint(100)) {
          msg_print("You have disarmed the trap.");
          C_player_add_exp(t_list[cave[y][x].tptr].p1);
          cave[y][x].fm = false;
          pusht(cave[y][x].tptr);
          cave[y][x].tptr = 0;
          player_action_move(tdir);
          lite_spot(y, x);
          prt_stat_block();
        } else if (randint(tot) > 5) {
          msg_print("You failed to disarm the trap.");
        } else {
          msg_print("You set the trap off!");
          player_action_move(tdir);
        }
      } else if (i1 == 2) { /*{ Chest trap    }*/
        /* with t_list[cave[y][x].tptr]. do; */
        if (strstr(t_list[cave[y][x].tptr].name, "^") != NULL) {
          msg_print("I don't see a trap...");
        } else if ((0x000001F0 & t_list[cave[y][x].tptr].flags) != 0) {
          if (tot - t5 > randint(100)) {
            t_list[cave[y][x].tptr].flags &= 0xFFFFFE0F;
            char *tmpc = strstr(t_list[cave[y][x].tptr].name, " (");
            if (tmpc != NULL) {
              *tmpc = 0;
            }
            if ((0x00000001 & t_list[cave[y][x].tptr].flags) != 0) {
              strcat(t_list[cave[y][x].tptr].name, " (Locked)");
            } else {
              strcat(t_list[cave[y][x].tptr].name, " (Disarmed)");
            }
            msg_print("You have disarmed "
                      "the chest.");
            known2(t_list[cave[y][x].tptr].name);
            C_player_add_exp(t5);
            prt_stat_block();
          } else if (randint(tot) > 5) {
            msg_print("You failed to "
                      "disarm the chest.");
          } else {
            msg_print("You set a trap off!");
            known2(t_list[cave[y][x].tptr].name);
            trigger_trap(y, x);
          }
        } else {
          msg_print("The chest was not trapped.");
        }
      } else {
        msg_print("I do not see anything to disarm there.");
      }
    } else {
      msg_print("I do not see anything to disarm there.");
    }
  }
}
