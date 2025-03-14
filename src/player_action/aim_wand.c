#include "../constants.h"
#include "../inventory/inven.h"
#include "../io.h"
#include "../misc.h"
#include "../player.h"
#include "../random.h"
#include "../screen.h"
#include "../spells.h"
#include "../text_lines.h"
#include "../types.h"
#include "../variables.h"

static void aw__wand_effects(long effect, bool *idented, const long dir,
                             const long i3, const long i4) {

  bool ident = *idented;

  /*{ Wands							}*/

  /* handle wand of wonder */
  if (effect == 24) {
    effect = randint(23);
  }

  switch (effect) {
  case 1:
    msg_print("A line of blue shimmering light appears.");
    light_line(dir, char_row, char_col, 1);
    ident = true;
    break;

  case 2:
    fire_bolt(SE_LIGHTNING, dir, i3, i4, damroll("3d8"), "Lightning Bolt");
    ident = true;
    break;

  case 3:
    fire_bolt(SE_COLD, dir, i3, i4, damroll("4d8"), "Frost Bolt");
    ident = true;
    break;

  case 4:
    fire_bolt(SE_FIRE, dir, i3, i4, damroll("6d8"), "Fire Bolt");
    ident = true;
    break;

  case 5:
    ident = wall_to_mud(dir, i3, i4);
    break;

  case 6:
    ident = poly_monster(dir, i3, i4);
    break;

  case 7:
    ident = zap_monster(dir, i3, i4, -damroll("4d6"), SE_HP);
    break;

  case 8:
    ident = zap_monster(dir, i3, i4, 1, SE_SPEED);
    break;

  case 9:
    ident = zap_monster(dir, i3, i4, -1, SE_SPEED);
    break;

  case 10:
    ident = zap_monster(dir, i3, i4, 0, SE_CONFUSE);
    break;

  case 11:
    ident = zap_monster(dir, i3, i4, 0, SE_SLEEP);
    break;

  case 12:
    ident = zap_monster(dir, i3, i4, 50, SE_DRAIN);
    break;

  case 13:
    ident = td_destroy2(dir, i3, i4);
    break;

  case 14:
    fire_bolt(0, dir, i3, i4, damroll("2d6"), "Magic Missile");
    ident = true;
    break;

  case 15:
    ident = build_wall(dir, i3, i4);
    break;

  case 16:
    ident = clone_monster(dir, i3, i4);
    break;

  case 17:
    ident = teleport_monster(dir, i3, i4);
    break;

  case 18:
    ident = disarm_all(dir, i3, i4);
    break;

  case 19:
    fire_ball(SE_LIGHTNING, dir, i3, i4, 24, "Lightning Ball");
    ident = true;
    break;

  case 20:
    fire_ball(SE_COLD, dir, i3, i4, 32, "Cold Ball");
    ident = true;
    break;

  case 21:
    fire_ball(SE_FIRE, dir, i3, i4, 48, "Fire Ball");
    ident = true;
    break;

  case 22:
    fire_ball(SE_GAS, dir, i3, i4, 8, "Stinking Cloud");
    ident = true;
    break;

  case 23:
    fire_ball(SE_ACID, dir, i3, i4, 40, "Acid Ball");
    ident = true;
    break;

  case 24:
    /* wand of wonder */
    break;

  case 25:
    ident = zap_monster(dir, i3, i4, 0, SE_PROBE);
    break;

  default:
    break;
  }
  /*{ End of Wands...					}*/

  *idented = ident;
}

void player_action_aim_wand(void) {
  unsigned long i1;
  long i3;
  long dir;
  treas_rec *i2;
  treas_rec *item_ptr;
  long dumy, y_dumy, x_dumy;
  char trash_char;
  bool redraw, ident;
  const obj_set give_me_a_wand = {wand, 0};

  redraw = false;
  reset_flag = true;

  if (inven_ctr > 0) {
    if (inventory_find_range(give_me_a_wand, false, &i2, &i3)) {
      if (get_item(&item_ptr, "Aim which wand?", &redraw, i3, &trash_char,
                   false, false)) {
        /* with item_ptr^.data do; */
        if (redraw) {
          draw_cave();
        }
        reset_flag = false;
        redraw = false;
        y_dumy = char_row;
        x_dumy = char_col;
        if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
          if (player_flags.confused > 0) {
            msg_print("You are confused...");
            do {
              dir = randint(9);
            } while (dir == 5);
          }
          i1 = item_ptr->data.flags;
          ident = false;

          /* with player_do; */
          long chance = player_save + player_lev + C_player_mod_from_stat(INT) -
                        item_ptr->data.level;

          if (player_flags.confused > 0) {
            chance /= 2;
          }
          if (chance < 0) {
            chance = 0;
          }

          if (randint(chance) < USE_DEVICE) {
            msg_print("You failed to use "
                      "the wand properly.");
          } else if (item_ptr->data.p1 > 0) {
            item_ptr->data.p1--;
            while (i1 > 0) {
              const long i5 = bit_pos(&i1) + 1;
              i3 = char_row;
              const long i4 = char_col;
              aw__wand_effects(i5, &ident, dir, i3, i4);
            }
            if (ident) {
              identify(&item_ptr->data);
            }
            if (item_ptr->data.flags != 0) {
              /* with player_do; */
              C_player_add_exp(item_ptr->data.level / (float)player_lev + .5);
              prt_stat_block();
            }

            msg_charges_remaining(item_ptr);

          } /* end if have charges */
        }   /* end if get_dir */
      }     /* end if get_item */

    } else {
      msg_print("You are not carrying any wands.");
    }
  } else {
    msg_print("But you are not carrying anything.");
  }
  if (redraw) {
    draw_cave();
  }
}
