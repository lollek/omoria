#include <string.h>

#include "../misc.h"
#include "../monsters.h"
#include "../player.h"
#include "../random.h"
#include "../traps.h"
#include "../desc.h"
#include "../screen.h"
#include "../variables.h"


void player_action_open() {

  long y, x, tmp, temp_dun_level;
  boolean flag;
  char *tmpc;

  y = char_row;
  x = char_col;

  if (d__get_dir("Which direction?", &tmp, &tmp, &y, &x)) {
    /* with cave[y][x]. do; */
    if (cave[y][x].tptr > 0) {

      /*{ Closed door           }*/
      if (t_list[cave[y][x].tptr].tval == closed_door) {
        /* with t_list[cave[y][x].tptr]. do; */
        if (t_list[cave[y][x].tptr].p1 > 0) { /*{ It's locked...        }*/
          /* with player_do; */
          tmp = player_disarm + player_lev + 2 * C_player_disarm_from_dex() +
                C_player_mod_from_stat(INT);

          if (player_flags.confused > 0) {
            msg_print("You are too "
                      "confused to pick "
                      "the lock.");
          } else if ((tmp - t_list[cave[y][x].tptr].p1) > randint(100)) {
            msg_print("You have picked the "
                      "lock.");
            C_player_add_exp(1);
            prt_stat_block();
            t_list[cave[y][x].tptr].p1 = 0;
          } else {
            msg_print("You failed to pick "
                      "the lock.");
          }

        } else if (t_list[cave[y][x].tptr].p1 < 0) { /*{ It's stuck    }*/
          msg_print("It appears to be stuck.");
        }

        if (t_list[cave[y][x].tptr].p1 == 0) {
          t_list[cave[y][x].tptr] = door_list[DL_OPEN];
          cave[y][x].fopen = true;
          lite_spot(y, x);
        }

      } else if (t_list[cave[y][x].tptr].tval == chest) {
        /*{ Open a closed chest...                }*/
        /* with player_do; */
        tmp = player_disarm + player_lev + 2 * C_player_disarm_from_dex() +
              C_player_mod_from_stat(INT);

        /* with t_list[tptr] do; */
        flag = false;
        if ((0x00000001 & t_list[cave[y][x].tptr].flags) != 0) { /* locked? */
          if (player_flags.confused > 0) {
            msg_print("You are too "
                      "confused to pick "
                      "the lock.");
          } else if ((tmp - (2 * t_list[cave[y][x].tptr].level)) >
                     randint(100)) {
            msg_print("You have picked the "
                      "lock.");
            flag = true;
            C_player_add_exp(t_list[cave[y][x].tptr].level);
            prt_stat_block();
          } else {
            msg_print("You failed to pick "
                      "the lock.");
          }
        } else {
          flag = true;
        }

        if (flag) {
          t_list[cave[y][x].tptr].flags &= 0xFFFFFFFE; /* unlock */
          tmpc = strstr(t_list[cave[y][x].tptr].name, " (");
          if (tmpc != NULL) {
            *tmpc = 0;
          }
          strcat(t_list[cave[y][x].tptr].name, " (Empty)");
          known2(t_list[cave[y][x].tptr].name);
          t_list[cave[y][x].tptr].cost = 0;
        }

        flag = false;

        /*{ Was chest still trapped?  (Snicker)   }*/
        if ((0x00000001 & t_list[cave[y][x].tptr].flags) == 0) { /*unlocked?*/
          trigger_trap(y, x);
          if (cave[y][x].tptr > 0) {
            flag = true;
          }
        }

        /*{ Chest treasure is allocted as if a creature
         * }*/
        /*{ had been killed... }*/

        if (flag) {
          /* horrible hack alert: chests had a bug
           * where the treasure inside
           * a chest is determined by the current
           * dungeon level at the time
           * when the chest is opened, not when
           * the chest was created.  so, one
           * could carry a chest up to town level,
           * open it up, and get crap.
           * or conversely, carry one way down in
           * the dungeon and get better
           * treasure than you deserve.  There's
           * no way to pass a level
           * value from here, where the chest is
           * opened, all the way down into
           * place_object() where a
           * treasure/dungeon level is actually
           * used,
           * because the call stack
           * d__openobject->monster_death->summon_object
           * ->place_object->get_obj_num doesn't
           * have level as a parameter all
           * the way until get_obj_num().  the
           * only way around this i can think
           * of, aside from re-engineering all
           * those functions and all calls
           * to them, is just to temporarily
           * change dun_level for the duration
           * of the chest's call to
           * monster_death().  2/16/00 JEB */
          temp_dun_level = dun_level;
          dun_level = t_list[cave[y][x].tptr].p1;
          monster_death(y, x, t_list[cave[y][x].tptr].flags);
          dun_level = temp_dun_level;
          t_list[cave[y][x].tptr].flags = 0; /* clear traps, lock, treasure */
        }

      } else {
        msg_print("I do not see anything you can open "
                  "there.");
      }
    } else {
      msg_print("I do not see anything you can open there.");
    }
  }
}