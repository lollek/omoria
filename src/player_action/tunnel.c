#include "../io.h"
#include "../misc.h"
#include "../player.h"
#include "../player_action.h"
#include "../random.h"
#include "../screen.h"
#include "../variables.h"

void player_action_tunnel(void) {
  /*{ Must take into account: secret doors, special tools           }*/

  long y, x, i1;

  y = char_row;
  x = char_col;

  if (d__get_dir("Which direction?", &i1, &i1, &y, &x)) {
    /* with cave[y][x]. do; */

    /*{ Compute the digging ability of player; based on       }*/
    /*{ strength, and type of tool used                       }*/
    long tabil = (C_player_get_stat(STR) * 10 + 20) / 5;
    if (equipment[Equipment_primary].tval > 0) {
      /* with equipment[Equipment_primary] do; */
      if ((Tunneling_worn_bit & equipment[Equipment_primary].flags) != 0) {
        tabil += 25 + equipment[Equipment_primary].p1 * 50;
      }
    }

    /*{ Regular walls; Granite, magma intrusion, quartz vein  }*/
    /*{ Don't forget the boundry walls, made of titanium (255)}*/
    switch (cave[y][x].fval) {
    case 10:
      i1 = randint(1200) + 80;
      if (twall(y, x, tabil, i1)) {
        msg_print("You have finished the tunnel.");
      } else {
        msg_print("You tunnel into the granite wall.");
      }
      break;

    case 11:
      i1 = randint(600) + 10;
      if (twall(y, x, tabil, i1)) {
        msg_print("You have finished the tunnel.");
      } else {
        msg_print("You tunnel into the magma intrusion.");
      }
      break;

    case 12:
      i1 = randint(400) + 10;
      if (twall(y, x, tabil, i1)) {
        msg_print("You have finished the tunnel.");
      } else {
        msg_print("You tunnel into the quartz vein.");
      }
      break;

    case 15:
      msg_print("This seems to be permanent rock.");
      break;

    case 16:
      msg_print("You can't tunnel through water!");
      break;

    default:
      /*{ Is there an object in the way?  (Rubble and secret
       * doors)}*/
      if (cave[y][x].tptr > 0) {

        /*{ Rubble...     }*/
        if (t_list[cave[y][x].tptr].tval == rubble) {
          if (tabil > randint(180)) {
            pusht(cave[y][x].tptr);
            cave[y][x].tptr = 0;
            cave[y][x].fm = false;
            cave[y][x].fopen = true;
            msg_print("You have removed "
                      "the rubble.");
            if (randint(10) == 1) {
              place_object(y, x);
              if (test_light(y, x)) {
                msg_print("You have "
                          "found "
                          "something"
                          "!");
              }
            }
            lite_spot(y, x);
          } else {
            msg_print("You dig in the rubble...");
          }

          /*{ Secret doors...}*/
        } else if (t_list[cave[y][x].tptr].tval == secret_door) {
          msg_print("You tunnel into the granite "
                    "wall.");
          player_action_search(char_row, char_col,
                               C_player_curr_search_skill());
        } else {
          msg_print("You can't tunnel through that.");
        }
      } else {
        msg_print("Tunnel through what?  Empty air???");
      }
      break;
    }
  }
}
