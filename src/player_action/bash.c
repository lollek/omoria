#include <math.h>
#include <stdlib.h>
#include <string.h>

#include "../io.h"
#include "../misc.h"
#include "../player.h"
#include "../player_action.h"
#include "../random.h"
#include "../screen.h"
#include "../variables.h"
#include "../spells.h"
#include "../fighting.h"

/**
 * -RAK-
 *  d__bash() - Bash open a door or chest
 * Note: Affected by strength and weight of character
 */
void player_action_bash(void) {

  long y = char_row;
  long x = char_col;
  long tmp;

  if (!d__get_dir("Which direction?", &tmp, &tmp, &y, &x))
    return;

  if (cave[y][x].cptr > 1) {
    if (player_flags.afraid > 0) {
      msg_print("You are afraid!");
    } else {
      /*{ Save old values of attacking  }*/
      inven_temp.data = equipment[Equipment_primary];
      const long old_ptodam = player_ptohit;
      const long old_ptohit = player_ptodam;
      const long old_bth = player_bth;

      /*{ Use these values              }*/
      equipment[Equipment_primary] = blank_treasure;
      strcpy(equipment[Equipment_primary].damage,
             equipment[Equipment_shield].damage);
      equipment[Equipment_primary].weight =
          (C_player_get_stat(STR) * 10 + 20) * 100;
      equipment[Equipment_primary].tval = 1;

      player_bth =
          trunc(((C_player_get_stat(STR) * 10 + 20) / 5 + player_wt) / 6.0);
      player_ptohit = 0;
      player_ptodam = trunc(player_wt / 75.0) + 1;

      if (player_action_attack(y, x)) {
        do_stun(cave[y][x].cptr, -10, 2);
      }

      /*{ Restore old values            }*/
      equipment[Equipment_primary] = inven_temp.data;
      player_ptohit = old_ptohit;
      player_ptodam = old_ptodam;
      player_bth = old_bth;
      if (randint(300) > C_player_get_stat(DEX) * 10) {
        msg_print("You are off-balance.");
        player_flags.paralysis = randint(3);
      }
    }
  } else if (cave[y][x].tptr > 0) {
    if (t_list[cave[y][x].tptr].tval == closed_door) {
      const int from_str = C_player_get_stat(STR) * 10;

      if (test_hit(player_wt + from_str * from_str / 500, 0, 0,
                   labs(t_list[cave[y][x].tptr].p1) + 150)) {
        msg_print("You smash into the door! "
                  "The door crashes open!");
        t_list[cave[y][x].tptr] = door_list[DL_OPEN];
        t_list[cave[y][x].tptr].p1 = 1;
        cave[y][x].fopen = true;
        lite_spot(y, x);
      } else {
        msg_print("You smash into the door! "
                  "The door holds firm.");
        player_flags.paralysis = 2;
      }

    } else if (t_list[cave[y][x].tptr].tval == chest) {

      if (randint(10) == 1) {
        msg_print("You have destroyed the chest...");
        msg_print("and its contents!");
        strcpy(t_list[cave[y][x].tptr].name, "& ruined chest");
        t_list[cave[y][x].tptr].flags = 0;
      } else if ((0x00000001 & t_list[cave[y][x].tptr].flags) != 0) {
        if (randint(10) == 1) {
          /* just "unlocks", traps are
           * still in place */
          msg_print("The lock breaks open!");
          t_list[cave[y][x].tptr].flags &= 0xFFFFFFFE; /* unlock */
        }
      }

    } else {
      msg_print("I do not see anything you can bash "
                "there.");
    }
  } else {
    msg_print("I do not see anything you can bash there.");
  }
}
