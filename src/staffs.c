#include <curses.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "configure.h"
#include "constants.h"
#include "debug.h"
#include "main_loop.h"
#include "magic.h"
#include "pascal.h"
#include "player.h"
#include "term.h"
#include "types.h"
#include "variables.h"
#include "inven.h"
#include "desc.h"
#include "screen.h"
#include "spells.h"
#include "misc.h"
#include "random.h"

void us__staff_effect(long effect, boolean *idented) {
  long i3, randnum;
  long y, x;
  boolean ident;

  ident = *idented;

  /*{ Staffs...                                             }*/

  switch (effect) {

  case 1:
    ident = light_area(char_row, char_col);
    break;

  case 2:
    ident = detect_sdoor();
    break;

  case 3:
    ident = detect_trap();
    break;

  case 4:
    ident = detect_item(SE_TREASURE);
    break;

  case 5:
    ident = detect_item(SE_OBJECT);
    break;

  case 6:
    teleport(100);
    ident = true;
    break;

  case 7:
    ident = earthquake();
    break;

  case 8:
    for (randnum = randint(4), i3 = 0; i3 < randnum; i3++) {
      y = char_row;
      x = char_col;
      if (is_in(cave[y][x].fval, water_set)) {
        summon_water_monster(&y, &x, false);
      } else {
        summon_land_monster(&y, &x, false);
      }
    }
    ident = true;
    break;

  case 9:
    ident = genocide();
    break;

  case 10:
    ident = destroy_area(char_row, char_col);
    break;

  case 11:
    msg_print("The end of the staff bursts into a blue shimmering "
              "light.");
    ident = starlite(char_row, char_col);
    break;

  case 12:
    ident = zap_area(0, 1, SE_SPEED); /*{haste}*/
    break;

  case 13:
    ident = zap_area(0, -1, SE_SPEED); /*{slow}*/
    break;

  case 14:
    ident = zap_area(0, 0, SE_SLEEP);
    break;

  case 15:
    ident = hp_player(randint(8), "a staff.");
    break;

  case 16:
    ident = detect_creatures(SE_INVISIBLE);
    break;

  case 17:
    player_flags.fast += randint(30) + 15;
    ident = true;
    break;

  case 18:
    player_flags.slow += randint(30) + 15;
    ident = true;
    break;

  case 19:
    ident = mass_poly();
    break;

  case 20:
    if (remove_curse()) {
      msg_print("The staff glows blue for a moment...");
      ident = true;
    }
    break;

  case 21:
    ident = detect_creatures(SE_EVIL);
    break;

  case 22:
    /* with player_flags do; */
    ident = cure_me(&((player_flags).blind));
    ident |= cure_me(&((player_flags).poisoned));
    ident |= cure_me(&((player_flags).confused));
    break;

  case 23:
    ident = zap_area(0x0004, 60, SE_HP);
    break;

  case 24:
    ident = mass_genocide();
    break;

  case 25:
    ident = unlight_area(char_row, char_col);
    break;

  case 26:
    ident = ident_spell();
    break;

  default:
    break;
  }
  /*{ End of staff actions...                               }*/

  *idented = ident;
}

void use_staff() {
  /*{ Use a staff...                                        -RAK-   }*/

  unsigned long i1;
  long i3, chance, i4;
  treas_rec *i2;
  treas_rec *item_ptr;
  char trash_char;
  boolean redraw, ident;
  obj_set this_be_a_staff = {staff, 0};

  reset_flag = true;

  if (inven_ctr > 0) {
    if (find_range(this_be_a_staff, false, &i2, &i3)) {

      redraw = false;
      if (get_item(&item_ptr, "Use which staff?", &redraw, i3, &trash_char,
                   false, false)) {
        /* with item_ptr^.data do; */

        if (redraw) {
          draw_cave();
        }
        reset_flag = false;

        /* with player_do; */
        chance = player_save + player_lev + C_player_mod_from_stat(INT) -
                 item_ptr->data.level - 5;

        if (player_flags.confused > 0) {
          chance /= 2;
        }
        if (chance < 0) {
          chance = 0;
        }

        if (randint(chance) < USE_DEVICE) {
          msg_print("You failed to use the staff "
                    "properly.");
        } else if (item_ptr->data.p1 > 0) {
          i1 = item_ptr->data.flags;
          ident = false;
          item_ptr->data.p1--;
          for (; i1 > 0;) {
            i4 = bit_pos(&i1) + 1;

            us__staff_effect(i4, &ident);
          }
          identify(&(item_ptr->data));
          if (ident) {
            if (item_ptr->data.flags != 0) {
              /* with player_do; */
              C_player_add_exp((item_ptr->data.level / (float)player_lev) + .5);
              prt_stat_block();
            }
          }
          desc_charges(item_ptr);
        }

      } else {
        if (redraw) {
          draw_cave();
        }
      }
    } else {
      msg_print("You are not carrying any staffs.");
    }
  } else {
    msg_print("But you are not carrying anything.");
  }
}
