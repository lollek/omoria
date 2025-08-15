/* monsters.c */
/**/

#include "monsters.h"
#include "death.h"
#include "debug.h"
#include "generate_monster/generate_monster.h"
#include "io.h"
#include "loot/loot.h"
#include "misc.h"
#include "pascal.h"
#include "player.h"
#include "random.h"
#include "screen.h"
#include "types.h"
#include "variables.h"
#include <curses.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h> /* for ftruncate, usleep */

static float acc_exp = 0.0;       /*{ Accumulator for fractional exp} */


/*{ Creates objects nearby the coordinates given          -RAK-   }*/
/*{ BUG: Because of the range, objects can actually be placed into}*/
/*{      areas closed off to the player, this is rarely noticable,}*/
/*{      and never a problem to the game.                         }*/
static void summon_object(const long y, const long x, long num,
                          const long typ) {

  do {
    long i1 = 0;
    do {
      const long i2 = y - 3 + randint(5);
      const long i3 = x - 3 + randint(5);
      if (in_bounds(i2, i3)) {
        if (los(y, x, i2, i3)) { /*{OOK!}*/
          /* with cave[i2][i3]. do; */
          if (is_in(cave[i2][i3].fval, floor_set)) {
            if (cave[i2][i3].tptr == 0) {

              switch (typ) { /*{ Select
                                type of
                                object }*/
              case 1:
                place_random_dungeon_item(i2, i3);
                break;

              case 2:
                place_gold(i2, i3);
                break;

              case 3:
                if (randint(100) < 50) {
                  place_random_dungeon_item(i2, i3);
                } else {
                  place_gold(i2, i3);
                }
                break;

              default:
                break;
              }
              if (test_light(i2, i3)) {
                lite_spot(i2, i3);
              }
              i1 = 10;
            }
          }
        }
      }
      i1++;
    } while (i1 <= 10);
    num--;
  } while (num != 0);
}

void monster_death(const long y, const long x, const unsigned long flags) {

  const long i1 = (flags & 0x03000000) / 0x01000000;

  if ((flags & 0x04000000) != 0) {
    if (randint(100) < 60) {
      summon_object(y, x, 1, i1);
    }
  }

  if ((flags & 0x08000000) != 0) {
    if (randint(100) < 90) {
      summon_object(y, x, 1, i1);
    }
  }

  if ((flags & 0x10000000) != 0) {
    summon_object(y, x, randint(2), i1);
  }
  if ((flags & 0x20000000) != 0) {
    summon_object(y, x, damroll("2d2"), i1);
  }
  if ((flags & 0x40000000) != 0) {
    summon_object(y, x, damroll("4d3"), i1);
  }
  if ((flags & 0x80000000) != 0) {
    total_winner = true;
    msg_print("*** CONGRATULATIONS *** You have won the game...");
    msg_print("Use '@' when you are ready to quit.");
  }
}

long mon_take_hit(const long monptr, const long dam) {
  /*{ (Picking on my babies...)                             -RAK-   }*/

  long return_value = 0;

  ENTER(("mon_take_hit", "%d, %d", monptr, dam));

  /* with m_list[monptr]. do; */
  m_list[monptr].hp -= dam;
  m_list[monptr].csleep = 0;
  if (m_list[monptr].hp < 0) {
    long i1 = 0;

    monster_death(m_list[monptr].fy, m_list[monptr].fx,
                  monster_templates[m_list[monptr].mptr].cmove);

    if (m_list[monptr].mptr == player_cur_quest && player_flags.quested) {
      player_flags.quested = false;
      prt_quested();
      msg_print("*** QUEST COMPLETED ***");
      msg_print("Return to the surface and report to the "
                "Arch-Wizard.");
    }

    /* with monster_templates[m_list[monptr].mptr]. do; */
    /* with player_do; */
    if ((monster_templates[m_list[monptr].mptr].cmove & 0x00004000) == 0 &&
        monster_templates[m_list[monptr].mptr].mexp > 0) {

      const float acc_tmp = monster_templates[m_list[monptr].mptr].mexp *
                      ((monster_templates[m_list[monptr].mptr].level + 0.1) /
                       (float)player_lev);
      i1 = (long)acc_tmp;
      acc_exp += acc_tmp - i1;
      if (acc_exp > 1) {
        i1++;
        acc_exp -= 1.0;
      }
      C_player_add_exp(i1);

    } else if (monster_templates[m_list[monptr].mptr].mexp > 0) {

      change_rep(-monster_templates[m_list[monptr].mptr].mexp);
      if (player_rep > -250) {
        msg_print("The townspeople look at you sadly.");
        msg_print("They shake their heads at the "
                  "needless violence.");
      } else if (player_rep > -1000) {
        monster_summon_by_name(char_row, char_col, "Town Guard", true, false);
        msg_print("The townspeople call for the guards!");
      } else if (player_rep > -2500) {
        monster_summon_by_name(char_row, char_col, "Town Wizard", true, false);
        msg_print("A Town Wizard appears!");
      } else {
        msg_print("Your god disapproves of your recent "
                  "town killing spree.");
        msg_print("Unlike the townspeople, he can do "
                  "something about it.");
        msg_print(" ");
        strcpy(died_from, "The Wrath of God");
        upon_death(false);
      }
    }

    return_value = m_list[monptr].mptr;
    delete_monster(monptr);

    if (i1 > 0) {
      prt_stat_block();
    }

  } else {
    return_value = 0;
  }

  RETURN("mon_take_hit", "d", 'd', "monval", &return_value);
  return return_value;
}

void delete_monster(const long i2) {

  ENTER(("delete_monster", "%d", i2));

  const long i3 = m_list[i2].nptr;
  if (muptr == i2) {
    muptr = i3;
  } else {
    long i1;

    for (i1 = muptr; m_list[i1].nptr != i2;) {
      i1 = m_list[i1].nptr;
    }
    m_list[i1].nptr = i3;
  }

  /* with m_list[i2]. do; */
  cave[m_list[i2].fy][m_list[i2].fx].cptr = 0;
  if (m_list[i2].ml) {
    /* with cave[fy][fx]. do; */
    if (cave[m_list[i2].fy][m_list[i2].fx].pl ||
        cave[m_list[i2].fy][m_list[i2].fx].tl) {
      lite_spot(m_list[i2].fy, m_list[i2].fx);
    } else {
      unlite_spot(m_list[i2].fy, m_list[i2].fx);
    }
  }

  pushm(i2);
  mon_tot_mult--;

  LEAVE("delete_monster", "c");
}
