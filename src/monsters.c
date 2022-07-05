/* monsters.c */
/**/

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
#include "magic.h"
#include "misc.h"
#include "pascal.h"
#include "player.h"
#include "random.h"
#include "screen.h"
#include "term.h"
#include "death.h"
#include "types.h"
#include "variables.h"
#include "wizard.h"

#include "monsters.h"

static const long mon_nasty = 50; /// Dun_level/x chance of high level creature
static float acc_exp = 0.0;       /*{ Accumulator for fractional exp} */

/*{ Places a monster at given location -RAK- }*/
void place_monster(long y, long x, long z, boolean slp) {
  long cur_pos;

  popm(&cur_pos);
  m_list[cur_pos].fy = y;
  m_list[cur_pos].fx = x;
  m_list[cur_pos].mptr = z;
  m_list[cur_pos].nptr = muptr;
  muptr = cur_pos;

  if ((c_list[z].cdefense & 0x4000) != 0) {
    m_list[cur_pos].hp = max_hp(c_list[z].hd);
  } else {
    m_list[cur_pos].hp = damroll(c_list[z].hd);
  }

  m_list[cur_pos].cdis = distance(char_row, char_col, y, x);
  m_list[cur_pos].cspeed = c_list[z].speed + player_flags.speed;
  m_list[cur_pos].stunned = 0;

  if (slp) {
    m_list[cur_pos].csleep = (c_list[z].sleep / 5.0) + randint(c_list[z].sleep);
  } else {
    m_list[cur_pos].csleep = 0;
  }

  cave[y][x].cptr = cur_pos;
}

/*{ Allocates a random land monster -RAK- }*/
void alloc_land_monster(obj_set alloc_set, long num, long dis, boolean slp,
                        boolean water) {
  long count = 0;
  long count2 = 0;

  for (int i = 0; i < num; i++) {
    long y;
    long x;
    do {
      y = randint(cur_height - 2) + 1;
      x = randint(cur_width - 2) + 1;
      if (++count2 > 7500)
        break;
    } while (!(is_in(cave[y][x].fval, alloc_set) && cave[y][x].cptr == 0 &&
               cave[y][x].fopen && distance(y, x, char_row, char_col) > dis));

    for (;;) {
      long monster_i;
      if (dun_level == 0) {
        monster_i = randint(m_level[0]);
      } else if (dun_level > MAX_MONS_LEVEL) {
        monster_i = randint(m_level[MAX_MONS_LEVEL]) + m_level[0];
      } else if (randint(mon_nasty) == 1) {
        monster_i = dun_level + labs(randnor(0, 4)) + 1;
        if (monster_i > MAX_MONS_LEVEL) {
          monster_i = MAX_MONS_LEVEL;
        }
        long i3 = m_level[monster_i] - m_level[monster_i - 1];
        monster_i = randint(i3) + m_level[monster_i - 1];
      } else {
        monster_i = randint(m_level[dun_level]) + m_level[0];
      }

      boolean ok_monster_found;
      if (!water) {
        ok_monster_found = (((c_list[monster_i].cmove & 0x00008000) == 0) &&
                            (((c_list[monster_i].cmove & 0x00000010) == 0) ||
                             ((c_list[monster_i].cmove & 0x00000040) == 0) ||
                             ((c_list[monster_i].cmove & 0x00800000) != 0)));
      } else {
        ok_monster_found = (((c_list[monster_i].cmove & 0x00008000) == 0) &&
                            ((c_list[monster_i].cmove & 0x00000010) != 0));
      }

      if (ok_monster_found) {
        if (count2 < 7500) {
          place_monster(y, x, monster_i, slp);
        }
        break;
      } else if (++count > 10) {
        break;
      }
    }
  }
}

/*{ Creates objects nearby the coordinates given          -RAK-   }*/
/*{ BUG: Because of the range, objects can actually be placed into}*/
/*{      areas closed off to the player, this is rarely noticable,}*/
/*{      and never a problem to the game.                         }*/
static void summon_object(long y, long x, long num, long typ) {

  long i1, i2, i3;

  do {
    i1 = 0;
    do {
      i2 = y - 3 + randint(5);
      i3 = x - 3 + randint(5);
      if (in_bounds(i2, i3)) {
        if (los(y, x, i2, i3)) { /*{OOK!}*/
          /* with cave[i2][i3]. do; */
          if (is_in(cave[i2][i3].fval, floor_set)) {
            if (cave[i2][i3].tptr == 0) {

              switch (typ) { /*{ Select
                                type of
                                object }*/
              case 1:
                place_object(i2, i3);
                break;

              case 2:
                place_gold(i2, i3);
                break;

              case 3:
                if (randint(100) < 50) {
                  place_object(i2, i3);
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
    } while (!(i1 > 10));
    num--;
  } while (num != 0);
}

void monster_death(long y, long x, unsigned long flags) {

  long i1 = (long)((flags & 0x03000000) / (0x01000000));

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

long mon_take_hit(long monptr, long dam) {
  /*{ (Picking on my babies...)                             -RAK-   }*/

  float acc_tmp;
  long i1 = 0;
  long return_value = 0;

  ENTER(("mon_take_hit", "%d, %d", monptr, dam));

  /* with m_list[monptr]. do; */
  m_list[monptr].hp -= dam;
  m_list[monptr].csleep = 0;
  if (m_list[monptr].hp < 0) {

    monster_death(m_list[monptr].fy, m_list[monptr].fx,
                  c_list[m_list[monptr].mptr].cmove);

    if ((m_list[monptr].mptr == player_cur_quest) && ((player_flags).quested)) {
      (player_flags).quested = false;
      prt_quested();
      msg_print("*** QUEST COMPLETED ***");
      msg_print("Return to the surface and report to the "
                "Arch-Wizard.");
    }

    /* with c_list[m_list[monptr].mptr]. do; */
    /* with player_do; */
    if (((c_list[m_list[monptr].mptr].cmove & 0x00004000) == 0) &&
        (c_list[m_list[monptr].mptr].mexp > 0)) {

      acc_tmp =
          (c_list[m_list[monptr].mptr].mexp *
           ((c_list[m_list[monptr].mptr].level + 0.1) / (float)player_lev));
      i1 = (long)(acc_tmp);
      acc_exp += (acc_tmp - i1);
      if (acc_exp > 1) {
        i1++;
        acc_exp -= 1.0;
      }
      C_player_add_exp(i1);

    } else if (c_list[m_list[monptr].mptr].mexp > 0) {

      change_rep(-c_list[m_list[monptr].mptr].mexp);
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
        upon_death();
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

void delete_monster(long i2) {

  long i1, i3;

  ENTER(("delete_monster", "%d", i2));

  i3 = m_list[i2].nptr;
  if (muptr == i2) {
    muptr = i3;
  } else {

    for (i1 = muptr; m_list[i1].nptr != i2;) {
      i1 = m_list[i1].nptr;
    }
    m_list[i1].nptr = i3;
  }

  /* with m_list[i2]. do; */
  cave[m_list[i2].fy][m_list[i2].fx].cptr = 0;
  if (m_list[i2].ml) {
    /* with cave[fy][fx]. do; */
    if ((cave[m_list[i2].fy][m_list[i2].fx].pl) ||
        (cave[m_list[i2].fy][m_list[i2].fx].tl)) {
      lite_spot(m_list[i2].fy, m_list[i2].fx);
    } else {
      unlite_spot(m_list[i2].fy, m_list[i2].fx);
    }
  }

  pushm(i2);
  mon_tot_mult--;

  LEAVE("delete_monster", "c");
}