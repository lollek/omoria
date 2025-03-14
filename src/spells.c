#include "spells.h"
#include "constants.h"
#include "creature.h"
#include "debug.h"
#include "dungeon/light.h"
#include "effects.h"
#include "generate_item/generate_item.h"
#include "generate_monster/generate_monster.h"
#include "inventory/inven.h"
#include "io.h"
#include "loot/loot.h"
#include "magic.h"
#include "misc.h"
#include "monsters.h"
#include "pascal.h"
#include "player.h"
#include "player_action.h"
#include "random.h"
#include "screen.h"
#include "text_lines.h"
#include "traps.h"
#include "types.h"
#include "variables.h"
#include <curses.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h> /* for ftruncate, usleep */

#define OBJ_BOLT_RANGE 18 /*{ Maximum range of bolts and balls	} */
#define DRAW_BOLT_DELAY 50
#define DRAW_BALL_DELAY 30

static const treasure_type scare_monster = /* { Special trap	} */
    {"a strange rune",
     seen_trap,
     0x00000000,
     0x00000000,
     0,
     0,
     99,
     0,
     0,
     0,
     0,
     0,
     0,
     "0d0",
     -90,
     0};
/* used in get_flags, and other places if I needed them */
static obj_set null_obj_set = {0, 0};
static obj_set destroyed_by_lightning = {ring, rod, wand, 0};
static obj_set destroyed_by_acid = {arrow,
                                    bow_crossbow_or_sling,
                                    hafted_weapon,
                                    pole_arm,
                                    gem_helm,
                                    boots,
                                    gloves_and_gauntlets,
                                    cloak,
                                    helm,
                                    shield,
                                    hard_armor,
                                    soft_armor,
                                    staff,
                                    scroll1,
                                    scroll2,
                                    Food,
                                    open_door,
                                    closed_door,
                                    0};
static obj_set destroyed_by_cold = {potion1, potion2, 0};
static obj_set destroyed_by_fire = {arrow,
                                    bow_crossbow_or_sling,
                                    hafted_weapon,
                                    pole_arm,
                                    boots,
                                    gloves_and_gauntlets,
                                    cloak,
                                    soft_armor,
                                    staff,
                                    scroll1,
                                    scroll2,
                                    potion1,
                                    potion2,
                                    Food,
                                    open_door,
                                    closed_door,
                                    0};
static obj_set destroyed_by_petrify = {boots,   soft_armor, potion1,
                                       potion2, Food,       0};
static obj_set destroyed_by_sunray = {cloak,   scroll1, scroll2,
                                      potion1, potion2, 0};

/*{ Lose experience hack for lose_exp breath              -RAK-   }*/
static void xp_loss(long amount) {

  long i1;

  amount = player_exp / 100 * MON_DRAIN_LIFE; /* passed val?  XXXX */

  /* with player_do; */
  msg_print("You feel your life draining away!");
  if (amount > player_exp) {
    player_exp = 0;
  } else {
    player_exp -= amount;
  }

  for (i1 = 1; (long)(exp_per_level[i1] * player_expfact) <= player_exp;) {
    i1++;
  }
  long i2 = player_lev - i1;

  while (i2 > 0) {
    const long av_hp = C_player_max_hp() / player_lev;
    const long av_mn = player_mana / player_lev;
    player_lev--;
    i2--;
    const long lose_hp = randint(av_hp * 2 - 1);
    const long lose_mn = randint(av_mn * 2 - 1);
    C_player_modify_max_hp(lose_hp);
    player_mana -= lose_mn;
    if (player_mana < 0) {
      player_mana = 0;
    }

    if (C_player_uses_magic(M_ARCANE) || C_player_uses_magic(M_DIVINE) ||
        C_player_uses_magic(M_NATURE) || C_player_uses_magic(M_SONG) ||
        C_player_uses_magic(M_CHAKRA)) {
      i1 = 32;
      bool flag = false;
      do {
        i1--;
        if (C_player_knows_spell(i1)) {
          flag = true;
        }
      } while (!(flag || i1 < 2));
      if (flag) {
        C_player_set_knows_spell(i1, false);
        if (C_player_uses_magic(M_ARCANE)) {
          msg_print("You have forgotten a magic "
                    "spell!");
        } else if (C_player_uses_magic(M_DIVINE)) {
          msg_print("You have forgotten a prayer!");
        } else if (C_player_uses_magic(M_SONG)) {
          msg_print("You have forgotten a song!");
        } else {
          msg_print("You have forgotten a discipline!");
        }
      }
    }
  }

  if (C_player_current_hp() > C_player_max_hp()) {
    C_player_reset_current_hp();
  }
  if (player_cmana > player_mana) {
    player_cmana = player_mana;
  }

  prt_stat_block();
}

static void get_flags(const enum spell_effect_t typ, long *weapon_type,
                      long *harm_type, obj_set **destroy) {
  /*{ Return flags for given type area affect               -RAK-   }*/

  switch (typ) {
  case SE_LIGHTNING: /* {1} */
    *weapon_type = 0x00080000;
    *harm_type = 0x0100;
    *destroy = &destroyed_by_lightning;
    break;

  case SE_GAS: /* {2}*/
    *weapon_type = 0x00100000;
    *harm_type = 0x0040;
    *destroy = &null_obj_set;
    break;

  case SE_ACID: /* {3}*/
    *weapon_type = 0x00200000;
    *harm_type = 0x0080;
    *destroy = &destroyed_by_acid;
    break;

  case SE_COLD: /* {4}*/
    *weapon_type = 0x00400000;
    *harm_type = 0x0010;
    *destroy = &destroyed_by_cold;
    break;

  case SE_FIRE: /* {5}*/
    *weapon_type = 0x00800000;
    *harm_type = 0x0020;
    *destroy = &destroyed_by_fire;
    break;

  case SE_GOOD: /* {6}*/
    *weapon_type = 0x00000000;
    *harm_type = 0x0004;
    *destroy = &null_obj_set;
    break;

  case SE_EVIL: /* {7}*/
    *weapon_type = 0x00000000;
    *harm_type = 0x0000;
    *destroy = &null_obj_set;
    break;

  case SE_PETRIFY: /* {8}*/
    *weapon_type = 0x00000000;
    *harm_type = 0x0000;
    *destroy = &destroyed_by_petrify;
    break;

  case SE_SUNRAY: /* {9}*/
    *weapon_type = 0x00000000;
    *harm_type = 0x0108;
    *destroy = &destroyed_by_sunray;
    break;

  default:
    *weapon_type = 0;
    *harm_type = 0;
    *destroy = &null_obj_set;
    break;
  }
}

void lower_stat(const enum stat_t tstat, char msg1[82]) {
  C_player_modify_lost_stat(tstat, 1);
  C_player_recalc_stats();
  if (strcmp(msg1, "X") == 0) {
    switch (tstat) {
    case STR:
      msg_print("You feel very sick.");
      break;
    case INT:
      msg_print("You become very dizzy.");
      break;
    case WIS:
      msg_print("You feel very naive.");
      break;
    case DEX:
      msg_print("You feel very sore.");
      break;
    case CON:
      msg_print("You feel very sick.");
      break;
    case CHR:
      msg_print("Your skin starts to itch.");
      break;
    }
  } else if (strlen(msg1) != 0) {
    msg_print(msg1);
  }
  prt_stat_block();
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool lose_stat(const enum stat_t tstat, char msg1[82], char msg2[82]) {
  const bool return_value = true;

  if (!player_flags.sustain[(int)tstat]) {
    lower_stat(tstat, msg1);
  } else {
    if (strcmp(msg2, "X") == 0) {
      switch (tstat) {
      case STR:
        msg_print("You feel sick for a moment, then it "
                  "passes.");
        break;
      case INT:
        msg_print("You become dizzy for a moment, then "
                  "it passes.");
        break;
      case WIS:
        msg_print("You feel naive for a moment, then "
                  "it passes.");
        break;
      case DEX:
        msg_print("You feel sore for a moment, then it "
                  "passes.");
        break;
      case CON:
        msg_print("You feel sick for a moment, then it "
                  "passes.");
        break;
      case CHR:
        msg_print("Your skin starts to itch, but feels "
                  "better now.");
        break;
      }
    } else if (strlen(msg2) != 0) {
      msg_print(msg2);
    }
  }

  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool restore_stat(const enum stat_t tstat, char msg1[82]) {
  /*{stat adjusted by magic worn only}*/
  const bool return_value = true;

  if (C_player_has_lost_stat(tstat)) {
    C_player_reset_lost_stat(tstat);
    C_player_recalc_stats();
    if (strcmp(msg1, "X") == 0) {
      switch (tstat) {
      case STR:
        msg_print("You feel warm all over.");
        break;
      case INT:
        msg_print("You have a warm feeling.");
        break;
      case WIS:
        msg_print("You feel your wisdom returning.");
        break;
      case DEX:
        msg_print("You feel less clumsy.");
        break;
      case CON:
        msg_print("You feel your health returning.");
        break;
      case CHR:
        msg_print("You feel your looks returning.");
        break;
      }
    } else if (strlen(msg1) != 0) {
      msg_print(msg1);
    }
    prt_stat_block();
  }

  return return_value;
}

bool cure_player_status_effect(int64_t *what_flag) {

  if (*what_flag > 1) {
    *what_flag = 1;
    return true;
  }
  return false;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool remove_curse(void) {
  /*{ Removes curses from items in inventory		-RAK-	}*/

  bool return_value = false;

  for (long i1 = Equipment_primary; i1 <= Equipment_cloak; i1++) {
    /* with equipment[i1] do; */
    if ((0x80000000 & equipment[i1].flags) != 0) {
      equipment[i1].flags &= 0x7FFFFFFF;
      py_bonuses(&blank_treasure, 0);
      return_value = true;
    }
  }

  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool hp_player(const long num, char kind[82]) {
  /*{ Change players hit points in some manner		-RAK-	}*/

  bool return_value = false;

  /* with player_do; */
  if (num < 0) {
    take_hit(num, kind);
    if (C_player_current_hp() < 0) {
      msg_print("You feel your life slipping away!");
      msg_print(" ");
    }
    return_value = true;
  } else if (C_player_current_hp() < C_player_max_hp()) {
    C_player_modify_current_hp(num);
    if (C_player_current_hp() > C_player_max_hp())
      C_player_reset_current_hp();
    prt_stat_block();

    switch (num / 5) {
    case 0:
      msg_print("You feel a little better.");
      break;
    case 1:
    case 2:
      msg_print("You feel better.");
      break;
    case 3:
    case 4:
    case 5:
    case 6:
      msg_print("You feel much better.");
      break;
    default:
      msg_print("You feel very good.");
      break;
    }
    return_value = true;
  }

  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool aggravate_monster(const long dis_affect) {
  /*{ Get all the monsters on the level pissed off...	-RAK-	}*/

  /* I'm guessing that this was supposed to only return true if any
     monsters were in range of the effect, it used to always be true */

  bool return_value = false;

  long i1 = muptr;
  do {
    /* with m_list[i1] do; */
    m_list[i1].csleep = 0;
    if (m_list[i1].cdis <= dis_affect) {
      if (m_list[i1].cspeed < 2) {
        m_list[i1].cspeed++;
      }
      return_value = true;
    }
    i1 = m_list[i1].nptr;
  } while (i1 != 0);

  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool explode(const enum spell_effect_t typ, const long y, const long x,
                const long dam_hp,
                const char *descrip) {
  long i1;
  long i2;
  const long max_dis = 2;
  long thit = 0;
  long tkill = 0;
  long weapon_type;
  long harm_type;
  obj_set *destroy;
  char out_val[82];
  const bool return_value = true;

  get_flags(typ, &weapon_type, &harm_type, &destroy);

  for (i1 = y - max_dis; i1 <= y + max_dis; i1++) {
    for (i2 = x - max_dis; i2 <= x + max_dis; i2++) {
      if (in_bounds(i1, i2)) {
        if (distance(y, x, i1, i2) <= max_dis) {
          if (los(y, x, i1, i2)) { /*{ FIXED BUG V4.5 }*/
            /* with cave[i1,i2] do; */
            if (cave[i1][i2].tptr > 0) {
              if (is_in(t_list[cave[i1][i2].tptr].tval, *destroy)) {
                delete_object(i1, i2);
              }
            }
            if (cave[i1][i2].fopen) {
              if (panel_contains(i1, i2)) {
                print('*', i1, i2);
                refresh();
                usleep(DRAW_BALL_DELAY);
              }
              if (cave[i1][i2].cptr > 1) {
                /* with */
                /* m_list[cave[i1][i2].cptr]
                 */
                /* do; */
                /* with */
                /* monster_templates[m_list[cave[i1][i2].cptr].mptr]
                 */
                /* do; */
                long dam = dam_hp;

                if (typ == SE_ILLUSION || typ == SE_JOKE) {
                  if (mon_save(cave[i1][i2].cptr, 0, SC_MENTAL)) {
                    if (typ == SE_ILLUSION) {
                      dam = 0;
                    } else {
                      dam = dam / 4;
                    }
                  } else {
                    m_list[cave[i1][i2].cptr].confused = true;
                  }
                }

                if ((harm_type &
                     monster_templates[m_list[cave[i1][i2].cptr].mptr]
                         .cdefense) != 0) {
                  dam *= 2;
                } else if ((weapon_type &
                            monster_templates[m_list[cave[i1][i2].cptr].mptr]
                                .spells) != 0) {
                  dam /= 4;
                }

                dam = dam / (distance(i1, i2, y, x) + 1);

                if (dam > 0 && !mon_resists(cave[i1][i2].cptr)) {
                  thit++;
                  if (mon_take_hit(cave[i1][i2].cptr, dam) > 0) {
                    tkill++;
                  } else {
                    if (panel_contains(i1, i2)) {
                      print(monster_templates[m_list[cave[i1][i2].cptr].mptr]
                                .symbol,
                            i1, i2);
                      m_list[cave[i1][i2].cptr].ml = true;
                    }
                  }
                }
              }
            }
          }
        }
      }
    } /* end for */
  }   /* end for */

  for (i1 = y - max_dis; i1 <= y + max_dis; i1++) {
    for (i2 = x - max_dis; i2 <= x + max_dis; i2++) {
      if (in_bounds(i1, i2)) {
        if (panel_contains(i1, i2)) {

          if (distance(y, x, i1, i2) <= max_dis) {
            /* with cave[i1,i2] do; */
            if (test_light(i1, i2)) {
              lite_spot(i1, i2);
            } else if (cave[i1][i2].cptr == 1) {
              lite_spot(i1, i2);
            } else if (cave[i1][i2].cptr > 1) {
              if (m_list[cave[i1][i2].cptr].ml) {
                lite_spot(i1, i2);
              } else {
                unlite_spot(i1, i2);
              }
            } else {
              unlite_spot(i1, i2);
            }
          }
        }
      }
    }
  }
  /*{ End  explosion...                     }*/
  if (thit == 1) {
    sprintf(out_val, "The %s envelopes a creature!", descrip);
    msg_print(out_val);
  } else if (thit > 1) {
    sprintf(out_val, "The %s envelopes several creatures!", descrip);
    msg_print(out_val);
  }

  if (tkill == 1) {
    if (typ == SE_JOKE) {
      msg_print("There is a scream of side-splitting laughter!");
    } else {
      msg_print("There is a scream of agony!");
    }
  } else if (tkill > 1) {
    if (typ == SE_JOKE) {
      msg_print("There are several screams of agonized laughter!");
    } else {
      msg_print("There are several screams of agony!");
    }
  }
  /*{ End ball hitting...                   }*/

  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool mon_save(const long a_cptr, long bonus,
                 const enum spell_class_t spell_class) {

  const long mon_level = monster_templates[m_list[a_cptr].mptr].level;

  /* with m_list[a_cptr] do; */
  /* with monster_templates[mptr] do; */
  if ((0x1000 & monster_templates[m_list[a_cptr].mptr].cdefense) != 0) {
    switch (spell_class) {
    case SC_HOLD:
      bonus += 4;
      break;

    case SC_MENTAL:
      bonus += 20;
      break;

    case SC_NULL:
      break;
    }
  }

  const bool return_value = 20 + mon_level + randint(mon_level) + 5 * bonus >
                         randint(80) + randint(player_lev);
  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool mon_resists(const unsigned char a_cptr) {
  bool return_value;

  /* with m_list[a_cptr] do; */
  /* with monster_templates[m_list[a_cptr].mptr] do; */

  long res_chance = monster_templates[m_list[a_cptr].mptr].magic_resistance;

  long delta_lev = player_lev + player_mr;
  if (delta_lev < 0) {
    delta_lev = 0;
  }

  res_chance -= 5 * delta_lev;
  if (res_chance < 0) {
    res_chance = 0;
  }

  if (res_chance >= randint(100)) {
    char out_val[82];
    sprintf(out_val, "The %s is protected by a mysterious force.",
            monster_templates[m_list[a_cptr].mptr].name);
    msg_print(out_val);
    return_value = true;
  } else {
    return_value = false;
  }

  return return_value;
}

bool do_stun(const unsigned char a_cptr, const long save_bonus,
                const long time) {
  char m_name[82];
  char out_val[100];

  find_monster_name(m_name, a_cptr, true);
  if (mon_resists(a_cptr)) {
    return false;
  }
  if (mon_save(a_cptr, save_bonus, SC_HOLD)) {
    return false;
  }
  sprintf(out_val, "%s appears stunned!", m_name);
  msg_print(out_val);
  const long held = m_list[a_cptr].stunned + 1 + randint(time);
  if (held > 24) {
    m_list[a_cptr].stunned = 24;
  } else {
    m_list[a_cptr].stunned = held;
  }
  return true;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool teleport_away(const long monptr, long dis) {
  /*{ Move the creature record to a new location            -RAK-   }*/

  bool return_value = false;

  /* with m_list[monptr] do; */
  if (!mon_resists(monptr)) {
    long xn;
    long yn;
    long ctr = 0;
    do {
      do {
        yn = m_list[monptr].fy + (randint(2 * dis + 1) - (dis + 1));
        xn = m_list[monptr].fx + (randint(2 * dis + 1) - (dis + 1));
      } while (!in_bounds(yn, xn));
      ctr++;
      if (ctr > 9) {
        ctr = 0;
        dis += 5;
      }
    } while (!(cave[yn][xn].fopen && cave[yn][xn].cptr == 0));

    move_creature(m_list[monptr].fy, m_list[monptr].fx, yn, xn);
    if (test_light(m_list[monptr].fy, m_list[monptr].fx)) {
      lite_spot(m_list[monptr].fy, m_list[monptr].fx);
    }
    m_list[monptr].fy = yn;
    m_list[monptr].fx = xn;
    m_list[monptr].ml = false;
    return_value = true;
  }

  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool teleport_to(const long ny, const long nx) {
  /*{ Teleport player to spell casting creature             -RAK-   }*/

  long y, x;
  const bool return_value = true;

  long dis = 1;
  long ctr = 0;
  do {
    do {
      y = ny + (randint(2 * dis + 1) - (dis + 1));
      x = nx + (randint(2 * dis + 1) - (dis + 1));
    } while (!in_bounds(y, x));
    ctr++;
    if (ctr > 9) {
      ctr = 0;
      dis++;
    }
  } while (!(cave[y][x].fopen && cave[y][x].cptr < 2));

  move_creature(char_row, char_col, y, x);
  for (long i1 = char_row - 1; i1 <= char_row + 1; i1++) {
    for (long i2 = char_col - 1; i2 <= char_col + 1; i2++) {
      /* with cave[i1][i2]. do; */
      cave[i1][i2].tl = false;
      if (!test_light(i1, i2)) {
        unlite_spot(i1, i2);
      }
    }
  }

  if (test_light(char_row, char_col)) {
    lite_spot(char_row, char_col);
  }
  char_row = y;
  char_col = x;
  player_action_move(5);
  creatures(false);

  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool breath(const enum spell_effect_t typ, const long y, const long x,
               const long dam_hp,
               char ddesc[82]) {
  /*{ Breath weapon works like a fire_ball, but affects the player. }*/
  /*{ Note the area affect....                              -RAK-   }*/

  long i1, i2;
  long dam;
  long weapon_type, harm_type;
  obj_set *destroy;
  const bool return_value = true;

  const long max_dis = 2;
  get_flags(typ, &weapon_type, &harm_type, &destroy);
  for (i1 = y - 2; i1 <= y + 2; i1++) {
    for (i2 = x - 2; i2 <= x + 2; i2++) {
      if (!in_bounds(i1, i2))
        continue;
      if (distance(y, x, i1, i2) > max_dis)
        continue;

      if (cave[i1][i2].tptr > 0 &&
          is_in(t_list[cave[i1][i2].tptr].tval, *destroy))
        delete_object(i1, i2);

      /* ??? What is this?
      if (!fopen)
              continue;
      */

      if (panel_contains(i1, i2)) {
        print('*', i1, i2);
        refresh();
        usleep(DRAW_BOLT_DELAY);
      }

      if (cave[i1][i2].cptr > 1) {
        /* with */
        /* m_list[cave[i1][i2].cptr].
         */
        /* do; */
        /* with */
        /* monster_templates[m_list[cave[i1][i2].cptr].mptr].
         */
        /* do; */
        dam = dam_hp;
        if ((harm_type &
             monster_templates[m_list[cave[i1][i2].cptr].mptr].cdefense) != 0) {
          dam *= 2;
        } else if ((weapon_type &
                    monster_templates[m_list[cave[i1][i2].cptr].mptr].spells) !=
                   0) {
          dam = trunc(dam / 4.0);
        }
        dam = dam / (distance(i1, i2, y, x) + 1);
        if (!mon_resists(cave[i1][i2].cptr)) {
          m_list[cave[i1][i2].cptr].hp -= dam;
        }
        m_list[cave[i1][i2].cptr].csleep = 0;
        if (m_list[cave[i1][i2].cptr].hp < 0) {
          monster_death(
              m_list[cave[i1][i2].cptr].fy, m_list[cave[i1][i2].cptr].fx,
              monster_templates[m_list[cave[i1][i2].cptr].mptr].cmove);
          delete_monster(cave[i1][i2].cptr);
        }
      } else if (cave[i1][i2].cptr == 1) {
        dam = trunc(dam_hp / (distance(i1, i2, y, x) + 1));
        switch (typ) {
        case SE_LIGHTNING:
          light_dam(dam, ddesc);
          break;
        case SE_GAS:
          poison_gas(dam, ddesc);
          break;
        case SE_ACID:
          acid_dam(dam, ddesc);
          break;
        case SE_COLD:
          cold_dam(dam, ddesc);
          break;
        case SE_FIRE:
          fire_dam(dam, ddesc);
          break;
        case SE_EVIL:
          xp_loss(dam);
          break;
        default:
          // Not implemented
          break;
        }
      }
    }
  }

  for (i1 = y - 2; i1 <= y + 2; i1++) {
    for (i2 = x - 2; i2 <= x + 2; i2++) {
      if (!in_bounds(i1, i2))
        continue;
      if (!panel_contains(i1, i2))
        continue;
      if (distance(y, x, i1, i2) > max_dis)
        continue;

      if (test_light(i1, i2)) {
        lite_spot(i1, i2);
      } else if (cave[i1][i2].cptr == 1) {
        lite_spot(i1, i2);
      } else if (cave[i1][i2].cptr > 1) {
        if (m_list[cave[i1][i2].cptr].ml) {
          lite_spot(i1, i2);
        } else {
          unlite_spot(i1, i2);
        }
      } else {
        unlite_spot(i1, i2);
      }
    }
  }

  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void lose_exp(const long amount) {
  /*{ Lose experience                                       -RAK-   }*/

  long i1;

  /* with player_do; */
  if (amount > player_exp) {
    player_exp = 0;
  } else {
    player_exp -= amount;
  }

  for (i1 = 1; trunc(exp_per_level[i1] * player_expfact) <= player_exp; i1++) {
  }

  for (long i2 = player_lev - i1; i2 > 0;) {
    i2--;
    player_lev--;
    const long av_hp = trunc(C_player_max_hp() / player_lev);
    const long av_mn = trunc(player_mana / player_lev);
    const long lose_hp = randint(av_hp * 2 - 1);
    const long lose_mn = randint(av_mn * 2 - 1);
    C_player_modify_max_hp(lose_hp);
    player_mana -= lose_mn;
    if (player_mana < 0) {
      player_mana = 0;
    }

    if (C_player_uses_magic(M_ARCANE) || C_player_uses_magic(M_DIVINE) ||
        C_player_uses_magic(M_NATURE) || C_player_uses_magic(M_SONG)) {
      i1 = 32;
      bool flag = false;

      do {
        i1--;
        if (C_player_knows_spell(i1)) {
          flag = true;
        }
      } while (!(flag || i1 < 2));

      if (flag) {
        C_player_set_knows_spell(i1, false);
        if (C_player_uses_magic(M_ARCANE)) {
          msg_print("You have forgotten a magic "
                    "spell!");
        } else if (C_player_uses_magic(M_DIVINE)) {
          msg_print("You have forgotten a prayer!");
        } else {
          msg_print("You have forgotten a song!");
        }
      }
    }
  } /* end for */

  if (C_player_current_hp() > C_player_max_hp())
    C_player_reset_current_hp();
  if (player_cmana > player_mana)
    player_cmana = player_mana;
  prt_stat_block();
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool gain_stat(const enum stat_t tstat, char msg1[82]) {
  if (C_player_has_lost_stat(tstat)) {
    C_player_reset_lost_stat(tstat);
  } else {
    C_player_mod_perm_stat(tstat, 1);
  }
  C_player_recalc_stats();

  if (strcmp(msg1, "X") == 0) {
    switch (tstat) {
    case STR:
      msg_print("Wow!  What bulging muscles!");
      break;
    case INT:
      msg_print("Aren't you brilliant!");
      break;
    case WIS:
      msg_print("You suddenly have a profound thought!");
      break;
    case DEX:
      msg_print("You feel more limber!");
      break;
    case CON:
      msg_print("You feel tingly for a moment.");
      break;
    case CHR:
      msg_print("Gee ain't you cute!");
      break;
    }
  } else if (strlen(msg1) != 0) {
    msg_print(msg1);
  }

  prt_stat_block();

  return true;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool slow_poison(void) {
  /*{ Slow Poison                                           -RAK-   }*/
  bool return_value = false;

  if (player_flags.poisoned > 0) {
    player_flags.poisoned /= 2;
    if (player_flags.poisoned < 1) {
      player_flags.poisoned = 1;
    }
    return_value = true;
    msg_print("The effects of the poison has been reduced.");
  }

  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool restore_player_drained_levels(void) {
  /*{ Restores any drained experience                       -RAK-   }*/

  unsigned char max_level = 1;
  bool return_value = false;

  while ((long)(exp_per_level[max_level] * player_expfact) <= player_max_exp) {
    max_level++;
  }

  if (randint(100) > (max_level - player_lev) * 2.25) {

    /* with player_do; */
    return_value = true;
    msg_print("You feel your life energies returning...");
    while (player_exp < player_max_exp) {
      player_exp = player_max_exp;
      prt_stat_block();
    }

  } else {
    msg_print("The restoring fails!");
  }

  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool detect_creatures(const enum spell_effect_t typ) {
  /*{ Display evil creatures on current panel               -RAK-   }*/

  bool found;

  bool flag = false;
  long i1 = muptr;

  do {
    /* with m_list[i1]. do; */
    if (panel_contains(m_list[i1].fy, m_list[i1].fx)) {

      switch (typ) {
      case SE_EVIL:
        found = (0x0004 & monster_templates[m_list[i1].mptr].cdefense) != 0;
        break;
      case SE_MONSTER:
        found = (0x10000 & monster_templates[m_list[i1].mptr].cmove) == 0;
        break;
      case SE_INVISIBLE:
        found = (0x10000 & monster_templates[m_list[i1].mptr].cmove) != 0;
        break;
      default:
        MSG(("Unknown typ in detect_creatures"));
        found = false;
        break;
      }

      if (found) {
        m_list[i1].ml = true;
        print(monster_templates[m_list[i1].mptr].symbol, m_list[i1].fy,
              m_list[i1].fx);
        flag = true;
      }
    }

    i1 = m_list[i1].nptr;

  } while (i1 != 0);

  if (flag) {
    switch (typ) {
    case SE_EVIL:
      msg_print("You sense the presence of evil!");
      break;
    case SE_MONSTER:
      msg_print("You sense the presence of monsters!");
      break;
    case SE_INVISIBLE:
      msg_print("You sense the presence of invisible creatures!");
      break;
    default:
      break;
    }
    msg_print(" ");
    msg_flag = false;
  }

  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool detect_inv2(const long amount) {
  /*{ Detect Invisible for period of time                   -RAK-   }*/

  player_flags.detect_inv += amount;

  return true;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool detect_item(const long typ) {
  /*{ Detect treasure or object on current panel    -Steven-}*/

  bool show_it;
  const obj_set treasures = {valuable_jewelry, valuable_gems, valuable_metal, 0};
  bool flag = false;

  for (long i1 = panel_row_min; i1 <= panel_row_max; i1++) {
    for (long i2 = panel_col_min; i2 <= panel_col_max; i2++) {
      /* with cave[i1][i2]. do; */
      if (cave[i1][i2].tptr > 0) {
        if (typ == SE_TREASURE) {
          show_it = is_in(t_list[cave[i1][i2].tptr].tval, treasures);
        } else {
          show_it = t_list[cave[i1][i2].tptr].tval < valuable_metal;
        }

        if (show_it) {
          if (!test_light(i1, i2)) {
            lite_spot(i1, i2);
            cave[i1][i2].tl = true;
            flag = true;
          }
        }
      }
    }
  }

  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool detect_trap(void) {
  /*{ Locates and displays traps on current panel           -RAK-   }*/

  bool flag = false;

  for (long i1 = panel_row_min; i1 <= panel_row_max; i1++) {
    for (long i2 = panel_col_min; i2 <= panel_col_max; i2++) {
      /* with cave[i1][i2]. do; */
      if (cave[i1][i2].tptr > 0) {
        if (t_list[cave[i1][i2].tptr].tval == unseen_trap) {
          change_trap(i1, i2);
          cave[i1][i2].fm = true;
          flag = true;
        } else if (t_list[cave[i1][i2].tptr].tval == chest) {
          /* with t_list[tptr] do; */
          known2(t_list[cave[i1][i2].tptr].name);
        }
      }
    }
  }

  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool detect_sdoor(void) {
  /*{ Locates and displays all secret doors on current panel -RAK-  }*/

  const obj_set pick_a_stair = {up_staircase, down_staircase, up_steep_staircase,
                          down_steep_staircase, 0};

  bool flag = false;

  for (long i1 = panel_row_min; i1 <= panel_row_max; i1++) {
    for (long i2 = panel_col_min; i2 <= panel_col_max; i2++) {
      /* with cave[i1][i2]. do; */
      if (cave[i1][i2].tptr > 0) {
        /*{ Secret doors  }*/
        if (t_list[cave[i1][i2].tptr].tval == secret_door) {
          cave[i1][i2].fval = corr_floor3.ftval;
          change_trap(i1, i2);
          cave[i1][i2].fm = true;
          flag = true;

          /*{ Staircases    }*/
        } else if (is_in(t_list[cave[i1][i2].tptr].tval, pick_a_stair)) {
          if (!cave[i1][i2].fm) {
            cave[i1][i2].fm = true;
            lite_spot(i1, i2);
            flag = true;
          }
        }
      }
    }
  }

  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool enchant(short *pluses) {
  /*{ Enchants a plus onto an item...                       -RAK-   }*/

  long chance = 0;
  bool return_value = false;

  if (*pluses > 0) {
    switch (*pluses) {
    case 1:
      chance = 040;
      break;
    case 2:
      chance = 100;
      break;
    case 3:
      chance = 200;
      break;
    case 4:
      chance = 400;
      break;
    case 5:
      chance = 600;
      break;
    case 6:
      chance = 700;
      break;
    case 7:
      chance = 800;
      break;
    case 8:
      chance = 900;
      break;
    case 9:
      chance = 950;
      break;
    default:
      chance = 995;
      break;
    }
  }

  if (randint(1000) > chance) {
    (*pluses)++;
    return_value = true;
  }

  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool bless(const long amount) {
  /*{ Bless                                                 -RAK-   }*/

  player_flags.blessed += amount;

  return true;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool ident_spell(void) {
  /*{ Identify an object                                    -RAK-   }*/

  treas_rec *item_ptr;
  char trash_char;
  long count = 0;
  bool redraw = false;
  bool return_value = false;

  /*  change_all_ok_stats(true,true);*/

  /* only show things that need to be identified */
  inventory_change_all_ok_stats(false, false);
  for (treas_rec *ptr = inventory_list; ptr != NULL; ptr = ptr->next) {
    if (strchr(ptr->data.name, '^') || strchr(ptr->data.name, '|')) {
      ptr->ok = true;
      count++;
    }
  }

  if (count == 0) {
    msg_print("All your items are already identified.");
    msg_print("");
  } else {

    if (get_item(&item_ptr, "Item you wish identified?", &redraw, count,
                 &trash_char, false, false)) {
      char out_val[82];
      /* with item_ptr->data. do; */
      identify(&item_ptr->data);
      known2(item_ptr->data.name);
      objdes(out_val, item_ptr, true);
      msg_print(out_val);

      return_value = true;
    }
  }
  if (redraw) {
    /* msg_print(""); */
    draw_cave();
  }

  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool light_area(const long y, const long x) {
  /*{ Light an area: 1.  If corridor then light immediate area -RAK-}*/
  /*{                2.  If room then light entire room.            }*/

  const obj_set room_floors = {1, 2, 17, 18, 0};
  const bool flag = true;

  msg_print("You are surrounded by a white light.");

  if (is_in(cave[y][x].fval, room_floors) && dun_level > 0) {
    dungeon_light_room(y, x);
  } else {
    for (long i1 = y - 1; i1 <= y + 1; i1++) {
      for (long i2 = x - 1; i2 <= x + 1; i2++) {
        if (in_bounds(i1, i2)) {
          if (!test_light(i1, i2)) {
            lite_spot(i1, i2);
          }
          cave[i1][i2].pl = true;
        }
      }
    }
  }

  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool unlight_area(const long y, const long x) {
  /*{ Darken an area, opposite of light area                -RAK-   }*/

  long i1, i2;
  const obj_set room_floors = {1, 2, 17, 18, 0};
  const obj_set doors_and_corridors = {4, 5, 6, 0};
  bool flag = false;

  if (is_in(cave[y][x].fval, room_floors) && dun_level > 0) {
    char out_val[82];
    const long tmp1 = trunc(SCREEN_HEIGHT / 2);
    const long tmp2 = trunc(SCREEN_WIDTH / 2);
    const long start_row = trunc(y / tmp1) * tmp1 + 1;
    const long start_col = trunc(x / tmp2) * tmp2 + 1;
    const long end_row = start_row + tmp1 - 1;
    const long end_col = start_col + tmp2 - 1;
    for (i1 = start_row; i1 <= end_row; i1++) {
      out_val[0] = 0;
      long ov_len = 0;
      long i3 = 0;
      for (i2 = start_col; i2 <= end_col; i2++) {
        /* with cave[i1][i2]; */
        if (is_in(cave[i1][i2].fval, room_floors)) {
          cave[i1][i2].pl = false;
          cave[i1][i2].fval = dopen_floor.ftval;
          if (!test_light(i1, i2)) {
            if (i3 == 0) {
              i3 = i2;
            }
            out_val[ov_len++] = ' ';
          } else if (i3 > 0) {
            flag = true;
            out_val[ov_len] = 0;
            print_str(out_val, i1, i3);
            out_val[0] = 0;
            ov_len = 0;
            i3 = 0;
          }
        } else if (i3 > 0) {
          flag = true;
          out_val[ov_len] = 0;
          print_str(out_val, i1, i3);
          out_val[0] = 0;
          i3 = 0;
        }
      } /* end for i2 */

      if (i3 > 0) {
        flag = true;
        out_val[ov_len] = 0;
        print_str(out_val, i1, i3);
      }
    } /* end for i1 */

  } else {

    for (i1 = y - 1; i1 <= y + 1; i1++) {
      for (i2 = x - 1; i2 <= x + 1; i2++) {
        if (in_bounds(i1, i2)) {
          /* with cave[i1][i2]. do; */
          if (is_in(cave[i1][i2].fval, doors_and_corridors)) {
            if (cave[i1][i2].pl) {
              cave[i1][i2].pl = false;
              flag = true;
            }
          }
        }
      }
    }

    if (flag) {
      msg_print("Darkness surrounds you...");
    }
  }

  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool map_area(void) {
  /*{ Map the current area plus some                        -RAK-   }*/

  const long i1 = panel_row_min - randint(10);
  const long i2 = panel_row_max + randint(10);
  const long i3 = panel_col_min - randint(20);
  const long i4 = panel_col_max + randint(20);

  for (long i5 = i1; i5 <= i2; i5++) {
    for (long i6 = i3; i6 <= i4; i6++) {
      if (in_bounds(i5, i6)) {
        if (is_in(cave[i5][i6].fval, floor_set)) {
          for (long i7 = i5 - 1; i7 <= i5 + 1; i7++) {
            for (long i8 = i6 - 1; i8 <= i6 + 1; i8++) {
              /* with cave[i7][i8]. */
              /* do; */
              if (is_in(cave[i7][i8].fval, pwall_set)) {
                cave[i7][i8].pl = true;
              } else if (cave[i7][i8].tptr > 0) {
                if (is_in(t_list[cave[i7][i8].tptr].tval, light_set)) {
                  cave[i7][i8].fm = true;
                }
              }
            }
          }
        }
      }
    }
  }

  prt_map();

  return true;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool sleep_monsters1(const long y, const long x) {
  /*{ Sleep creatures adjacent to player                    -RAK-   }*/

  bool flag = false;

  for (long i1 = y - 1; i1 <= y + 1; i1++) {
    for (long i2 = x - 1; i2 <= x + 1; i2++) {
      /* with cave[i1][i2]. do; */
      if (cave[i1][i2].cptr > 1) {
        /* with m_list[cave[i1][i2].cptr]. do; */
        /* with monster_templates[m_list[cave[i1][i2].cptr].mptr].
         */
        /* do; */
        if (!mon_resists(cave[i1][i2].cptr)) {
          char out_val[82];
          flag = true;
          if (mon_save(cave[i1][i2].cptr, 0, SC_MENTAL)) {
            sprintf(out_val, "The %s is unaffected.",
                    monster_templates[m_list[cave[i1][i2].cptr].mptr].name);
          } else {
            sprintf(out_val, "The %s falls asleep.",
                    monster_templates[m_list[cave[i1][i2].cptr].mptr].name);
            m_list[cave[i1][i2].cptr].csleep = 500;
          }
          msg_print(out_val);
        }
      }
    }
  }
  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool trap_creation(void) {
  /*{ Surround the fool with traps (chuckle)                -RAK-   }*/

  for (long i1 = char_row - 1; i1 <= char_row + 1; i1++) {
    for (long i2 = char_col - 1; i2 <= char_col + 1; i2++) {
      /* with cave[i1][i2]. do; */
      if (is_in(cave[i1][i2].fval, floor_set)) {
        if (cave[i1][i2].tptr > 0) {
          delete_object(i1, i2);
        }
        place_trap(i1, i2, 1, randint(MAX_TRAPA));
      }
    }
  }

  return true;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool door_creation(void) {
  /*{ Surround the player with doors...                     -RAK-   }*/

  long i3;
  const bool flag = true;

  for (long i1 = char_row - 1; i1 <= char_row + 1; i1++) {
    for (long i2 = char_col - 1; i2 <= char_col + 1; i2++) {
      if (i1 != char_row || i2 != char_col) {
        /* with cave[i1][i2]. do; */
        if (is_in(cave[i1][i2].fval, floor_set)) {
          popt(&i3);
          if (cave[i1][i2].tptr > 0) {
            delete_object(i1, i2);
          }
          cave[i1][i2].fopen = false;
          cave[i1][i2].tptr = i3;
          t_list[i3] = door_list[2];
          if (test_light(i1, i2)) {
            lite_spot(i1, i2);
          }
        }
      }
    }
  }

  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool td_destroy(void) {
  /*{ Destroys any adjacent door(s)/trap(s)                 -RAK-   }*/

  const obj_set pick_a_door = {unseen_trap, seen_trap,   open_door,
                         closed_door, secret_door, 0};

  bool flag = false;

  for (long i1 = char_row - 1; i1 <= char_row + 1; i1++) {
    for (long i2 = char_col - 1; i2 <= char_col + 1; i2++) {
      /* with cave[i1][i2]. do; */
      if (cave[i1][i2].tptr > 0) {
        if (is_in(t_list[cave[i1][i2].tptr].tval, pick_a_door)) {
          if (delete_object(i1, i2)) {
            flag = true;
          }
        }
      }
    }
  }

  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool warding_glyph(void) {
  /*{ Leave a glyph of warding... Creatures will not pass over! -RAK-}*/

  long i1;

  /* with cave[char_row][char_col]. do; */
  if (cave[char_row][char_col].tptr == 0) {
    popt(&i1);
    cave[char_row][char_col].tptr = i1;
    t_list[i1] = scare_monster;
  }

  return true;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool protect_evil(void) {
  /*{ Evil creatures don't like this...                     -RAK-   }*/

  player_flags.protevil += randint(25) + 3 * player_lev;

  return true;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool recharge(long num) {
  /*{ Recharge a wand, staff, or rod.  Sometimes the item breaks. -RAK-}*/

  treas_rec *item_ptr;
  bool redraw = false;
  char trash_char;
  /* added valuable_gems to this set, which should include any item type
   */
  /* that */
  /* uses charges.  2/15/00 JEB */
  const obj_set batteries_not_included = {valuable_gems, staff, rod, wand,
                                    chime,         horn,  0};
  bool return_value = false;

  inventory_change_all_ok_stats(true, true);
  if (get_item(&item_ptr, "Recharge which item?", &redraw, inven_ctr,
               &trash_char, false, false)) {
    /* with item_ptr->data. do; */
    if (is_in(item_ptr->data.tval, batteries_not_included)) {
      if (randint(8) == 1) {
        return_value = true;
        msg_print("There is a bright flash of light...");
        inven_destroy(item_ptr);
      } else {
        return_value = true;
        num = num / (item_ptr->data.level + 2);
        item_ptr->data.p1 += 2 + randint(num);
        if (strchr(item_ptr->data.name, '^') != NULL) {
          insert_str(item_ptr->data.name, " (%P1", "^ (%P1");
        }
      }
    } else if (item_ptr->data.tval == lamp_or_torch &&
               item_ptr->data.subval == 17) {
      if (randint(50) == 1) {
        return_value = true;
        msg_print("There is a bright flash of light...");
        inven_destroy(item_ptr);
      } else {
        return_value = true;
        num *= 100;
        item_ptr->data.p1 += num / 3 + randint(num);
        if (strchr(item_ptr->data.name, '^') != NULL) {
          insert_str(item_ptr->data.name, " (%P1", "^ (%P1");
        }
      }
    } else if (item_ptr->data.tval == lamp_or_torch &&
               item_ptr->data.subval == 15) {
      if (randint(15) == 1) {
        return_value = true;
        msg_print("There is a bright flash of light...");
        inven_destroy(item_ptr);
      } else {
        return_value = true;
        num *= 80;
        item_ptr->data.p1 += num / 3 + randint(num);
        if (strchr(item_ptr->data.name, '^') != NULL) {
          insert_str(item_ptr->data.name, " (%P1", "^ (%P1");
        }
      }
    }
  }

  if (redraw) {
    /*msg_print(" ");*/
    draw_cave();
  } else {
    prt_stat_block();
  }

  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool mass_genocide(void) {
  /*{ Delete all creatures within MAX_SIGHT distance        -RAK-   }*/
  /*{ NOTE : Winning creatures cannot be genocided                  }*/

  bool flag = false;

  long i1 = muptr; /* what happens if there are no monsters in the world? */
  do {
    /* with m_list[i1]. do; */
    /* with monster_templates[m_list[i1].mptr]. do; */
    const long i2 = m_list[i1].nptr;
    if (m_list[i1].cdis <= MAX_SIGHT) {
      if ((monster_templates[m_list[i1].mptr].cmove & 0x80000000) == 0 &&
          !mon_resists(i1)) {
        delete_monster(i1);
        flag = true;
      }
    }
    i1 = i2;
  } while (i1 != 0);

  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool genocide(void) {
  /*{ Delete all creatures of a given type from level.      -RAK-   }*/
  /*{ This does not keep creatures of type from appearing later.    }*/

  char typ;
  const bool flag = true;

  long i1 = muptr; /* what happens if there are no monsters in the world? */

  if (get_com("Which type of creature do wish exterminated? ", &typ)) {
    do {
      /* with m_list[i1]. do; */
      /* with monster_templates[m_list[i1].mptr]. do; */
      const long i2 = m_list[i1].nptr;
      if (typ == monster_templates[m_list[i1].mptr].symbol) {
        if ((monster_templates[m_list[i1].mptr].cmove & 0x80000000) == 0 &&
            !mon_resists(i1)) {
          delete_monster(i1);
        } else {
          char out_val[82];
          sprintf(out_val, "The %s is unaffected.",
                  monster_templates[m_list[i1].mptr].name);
          msg_print(out_val);
        }
      }
      i1 = i2;
    } while (i1 != 0);
  }

  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool create_food(const long t0, const long t1, const long t2, const long t3,
                    const long t4) {
  /*{ Create food for the player.           -RAK-   }*/

  long this_one;

  for (long i1 = char_row - 2; i1 <= char_row + 2; i1++) {
    for (long i2 = char_col - 2; i2 <= char_col + 2; i2++) {
      /* with cave[i1][i2]. do; */
      long dist = labs(i1 - char_row) + labs(i2 - char_col);
      if (labs(i1 - char_row) == 2 || labs(i2 - char_col) == 2) {
        dist++;
      }

      if (cave[i1][i2].fopen && cave[i1][i2].tptr == 0 && dist < 5) {

        switch (dist) {
        case 0:
          this_one = t0;
          break; /*{food types dep. on dist = }*/
        case 1:
          this_one = t1;
          break; /*{X434X}*/
        case 2:
          this_one = t2;
          break; /*{42124}*/
        case 3:
          this_one = t3;
          break; /*{31013}*/
        case 4:
          this_one = t4;
          break; /*{42124}*/
        default:
          this_one = 0;
          break; /*{X434X}*/
        }

        if (this_one != 0) {
          place_random_dungeon_item(i1, i2);
          if (this_one < 0) { /*{junk food.}*/
            t_list[cave[i1][i2].tptr] = generate_item_for_all_night_deli();
          } else { /*{good food}*/
            t_list[cave[i1][i2].tptr] = yums[this_one];
          }
          if (test_light(i1, i2)) {
            lite_spot(i1, i2);
          }
        }
      }
    }
  }

  return true;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool za__did_it_work(const long monptr, const long cflag, const long dmge,
                        const long typ) {
  bool hmm = false;

  /* with m_list[i1]. do; */
  /* with monster_templates[m_list[i1].mptr]. do; */

  switch (typ) {
  case SE_CONFUSE:
  case SE_SLEEP:
  case SE_JOKE:
    hmm = !mon_save(monptr, 0, SC_MENTAL);
    break;

  case SE_HOLD:
  case SE_THUNDER:
    hmm = do_stun(monptr, 0, dmge);
    break;

  case SE_SPEED:
    hmm = !mon_save(monster_templates[m_list[monptr].mptr].level, 0, SC_NULL) ||
          dmge > 0;
    break;

  case SE_TURN:
    hmm = (monster_templates[m_list[monptr].mptr].cdefense & 0x0008) != 0;
    break;

  case SE_DRAIN:
    hmm = (monster_templates[m_list[monptr].mptr].cdefense & 0x0008) == 0;
    break;

  case SE_HP:
    hmm = (monster_templates[m_list[monptr].mptr].cdefense & cflag) != 0;
    break;

  default:
    hmm = true;
    break;
  }

  return hmm && !mon_resists(monptr);
}
/*//////////////////////////////////////////////////////////////////// */
void za__yes_it_did(const long monptr, const long dmge,
                    const enum spell_effect_t typ) {
  char out_val[82];

  const long mptr = m_list[monptr].mptr; /* monster might get deleted */

  /* with m_list[i1]. do; */

  switch (typ) {
  case SE_CONFUSE:
  case SE_TURN:
    sprintf(out_val, "The %s runs frantically!", monster_templates[mptr].name);
    msg_print(out_val);
    m_list[monptr].confused = true;
    break;

  case SE_SLEEP:
    m_list[monptr].csleep = 500;
    break;

  case SE_SPEED:
    m_list[monptr].cspeed += dmge;
    m_list[monptr].csleep = 0;
    break;

  case SE_HOLD:; /*{done in do_stun in did_it_work already}*/
    break;

  case SE_THUNDER:
    m_list[monptr].confused = true;
    break;

  case SE_HP:
  case SE_JOKE:
  case SE_DRAIN:
  case SE_HOLY_WORD:
    /* with monster_templates[m_list[i1].mptr]. do; */
    if (mon_take_hit(monptr, randint(dmge)) > 0) {
      if (typ == SE_JOKE) {
        sprintf(out_val, "The %s dies laughing!", monster_templates[mptr].name);
      } else {
        sprintf(out_val, "The %s dissolves!", monster_templates[mptr].name);
      }
      msg_print(out_val);
    } else {
      if (typ == SE_JOKE) {
        sprintf(out_val, "The %s chuckles.", monster_templates[mptr].name);
        msg_print(out_val);
        m_list[monptr].confused = true;
      } else {
        sprintf(out_val, "The %s shudders.", monster_templates[mptr].name);
        msg_print(out_val);
        if (typ == SE_HOLY_WORD) {
          if (do_stun(monptr, -4, 4 + randint(4))) {
            m_list[monptr].confused = true;
          }
        }
      }
    }
    break;
  default:
    break;
  } /* end switch */
}
/*//////////////////////////////////////////////////////////////////// */
bool za__no_it_didnt(const long monptr, const long dmge, const long typ) {
  const obj_set some_stuff = {SE_SLEEP, SE_CONFUSE, SE_SPEED, SE_HOLD, SE_JOKE, 0};
  bool flag = false;

  /* with m_list[i1]. do; */
  if (is_in(typ, some_stuff)) {
    char out_val[82];
    flag = true;
    if (typ == SE_JOKE) {
      sprintf(out_val, "The %s appears offended...",
              monster_templates[m_list[monptr].mptr].name);
      msg_print(out_val);
      if (mon_take_hit(monptr, randint(dmge) / 4) > 0) {
        msg_print("and dies from disgust!!!");
      }
    } else {
      sprintf(out_val, "The %s is unaffected...",
              monster_templates[m_list[monptr].mptr].name);
      msg_print(out_val);
    }
  }

  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
bool zap_area(const long cflag, const long dmge, const long typ) {
  bool flag = false;

  long i1 = muptr; /* what if no monsters? */

  do {
    const long m_next = m_list[i1].nptr;
    /* with m_list[i1]. do; */
    /* with monster_templates[m_list[i1].mptr] do; */
    if (m_list[i1].ml) {
      if (za__did_it_work(i1, cflag, dmge, typ)) {
        za__yes_it_did(i1, dmge, typ);
        flag = true;
      } else if (za__no_it_didnt(i1, dmge, typ)) {
        flag = true;
      }
    }
    i1 = m_next;
  } while (i1 != 0);

  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void da__replace_spot(const long y, const long x, const long typ) {
  /* with cave[y][x]. do; */
  switch (typ) {
  case 1:
  case 2:
  case 3:
    cave[y][x].fval = corr_floor1.ftval;
    cave[y][x].fopen = corr_floor1.ftopen;
    break;

  case 4:
  case 7:
  case 10:
    cave[y][x].fval = rock_wall1.ftval;
    cave[y][x].fopen = rock_wall1.ftopen;
    break;

  case 5:
  case 8:
  case 11:
    cave[y][x].fval = rock_wall2.ftval;
    cave[y][x].fopen = rock_wall2.ftopen;
    break;

  case 6:
  case 9:
  case 12:
    cave[y][x].fval = rock_wall3.ftval;
    cave[y][x].fopen = rock_wall3.ftopen;
    break;
  }

  cave[y][x].pl = false;
  cave[y][x].fm = false;

  if (cave[y][x].tptr > 0) {
    delete_object(y, x);
  }

  if (cave[y][x].cptr > 1 && !mon_resists(cave[y][x].cptr)) {
    delete_monster(cave[y][x].cptr);
  }
}
/*//////////////////////////////////////////////////////////////////// */
bool destroy_area(const long y, const long x) {
  /*{ The spell of destruction...                           -RAK-   }*/
  /*{ NOTE : Winning creatures that are deleted will be considered  }*/
  /*{        as teleporting to another level.  This will NOT win the}*/
  /*{        game...                                                }*/

  if (dun_level > 0) {
    for (long i1 = y - 15; i1 <= y + 15; i1++) {
      for (long i2 = x - 15; i2 <= x + 15; i2++) {
        if (in_bounds(i1, i2)) {
          if (cave[i1][i2].fval != boundry_wall.ftval) {
            const long i3 = distance(i1, i2, y, x);
            if (i3 < 13) {
              da__replace_spot(i1, i2, randint(6));
            } else if (i3 < 16) {
              da__replace_spot(i1, i2, randint(9));
            }
          }
        }
      }
    }
  }

  msg_print("There is a searing blast of light!");
  player_flags.blind += 10 + randint(10);

  return true;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool earthquake(void) {
  /*{ This is a fun one.  In a given block, pick some walls and     }*/
  /*{ turn them into open spots.  Pick some open spots and turn     }*/
  /*{ them into walls.  An "Earthquake" effect...           -RAK-   }*/

  obj_set room_floors = {1, 2, 0};

  for (long i1 = char_row - 8; i1 <= char_row + 8; i1++) {
    for (long i2 = char_col - 8; i2 <= char_col + 8; i2++) {
      if (i1 != char_row || i2 != char_col) {
        if (in_bounds(i1, i2)) {
          if (randint(8) == 1) {
            /* with cave[i1][i2]. do; */
            if (cave[i1][i2].tptr > 0) {
              delete_object(i1, i2);
            }
            if (cave[i1][i2].cptr > 1 && !mon_resists(cave[i1][i2].cptr)) {
              mon_take_hit(cave[i1][i2].cptr, damroll("2d8"));
            }
            if (is_in(cave[i1][i2].fval, wall_set)) {
              if (next_to4(i1, i2, room_floors) > 0) {
                cave[i1][i2].fval = corr_floor2.ftval;
                cave[i1][i2].fopen = corr_floor2.ftopen;
              } else {
                cave[i1][i2].fval = corr_floor1.ftval;
                cave[i1][i2].fopen = corr_floor1.ftopen;
              }
              if (test_light(i1, i2)) {
                unlite_spot(i1, i2);
              }
              cave[i1][i2].pl = false;
              cave[i1][i2].fm = false;
              if (cave[i1][i2].tl) {
                lite_spot(i1, i2);
              }
            } else if (is_in(cave[i1][i2].fval, floor_set)) {

              switch (randint(10)) {
              case 1:
              case 2:
              case 3:
              case 4:
              case 5:
                cave[i1][i2].fval = rock_wall3.ftval;
                cave[i1][i2].fopen = rock_wall3.ftopen;
                break;

              case 6:
              case 7:
              case 8:
                cave[i1][i2].fval = rock_wall2.ftval;
                cave[i1][i2].fopen = rock_wall2.ftopen;
                break;

              case 9:
              case 10:
                cave[i1][i2].fval = rock_wall1.ftval;
                cave[i1][i2].fopen = rock_wall1.ftopen;
              }

              cave[i1][i2].fm = false;
            }

            if (test_light(i1, i2)) {
              lite_spot(i1, i2);
            }
          }
        }
      }
    }
  }

  return true;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool mass_poly(void) {
  /*{ Polymorph any creature that player can see...         -RAK-   }*/
  /*{ NOTE: cannot polymorph a winning creature (BALROG)            }*/

  bool flag = false;

  long i1 = muptr;

  do {
    /* with m_list[i1]. do; */
    const long i2 = m_list[i1].nptr;
    if (m_list[i1].cdis < MAX_SIGHT) {
      /* with monster_templates[m_list[i1].mptr]. do; */
      if ((monster_templates[m_list[i1].mptr].cdefense & 0x80000000) == 0 &&
          !mon_resists(i1)) {
        const long y = m_list[i1].fy;
        const long x = m_list[i1].fx;
        delete_monster(i1);
        place_monster(y, x, randint(m_level[MAX_MONS_LEVEL]) + m_level[0],
                      false);
        flag = true;
      }
    }

    i1 = i2;
  } while (i1 != 0); /* what if no monsters? */

  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool light_line(const long dir, long y, long x, const long power) {
  /*{ Leave a line of light in given dir, blue light can sometimes  }*/
  /*{ hurt creatures...                                     -RAK-   }*/

  ENTER(("light_line", "%d, %d, %d, %d", dir, y, x, power));

  while (cave[y][x].fopen) {
    /* with cave[y][x]. do; */

    if (panel_contains(y, x)) {

      if (!(cave[y][x].tl || cave[y][x].pl)) {
        if (cave[y][x].fval == lopen_floor.ftval) {
          dungeon_light_room(y, x);
        } else {
          lite_spot(y, x);
        }
      }

      if (cave[y][x].cptr > 1) {
        /* with m_list[cave[y][x].cptr]. do; */
        /* with monster_templates[m_list[cave[y][x].cptr].mptr]. */
        /* do; */
        if (!mon_resists(cave[y][x].cptr)) {
          if (0x0100 &
              monster_templates[m_list[cave[y][x].cptr].mptr].cdefense) {
            char out_val[82];

            sprintf(out_val, "The %s wails out in pain!",
                    monster_templates[m_list[cave[y][x].cptr].mptr].name);

            msg_print(out_val);

            long i1 = 0;
            for (long i2 = 1; i2 <= power; i2++) {
              i1 += damroll("2d8");
            }

            if (mon_take_hit(cave[y][x].cptr, i1) > 0) {
              sprintf(out_val,
                      "The %s dies in a "
                      "fit of agony.",
                      monster_templates[m_list[cave[y][x].cptr].mptr].name);
              msg_print(out_val);
            }
          }
        }
      }
    }
    cave[y][x].pl = true;

    move_dir(dir, &y, &x);
  }

  LEAVE("light_line", "s");
  return true;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool starlite(const long y, const long x) {
  /*{ Light line in all directions                          -RAK-   }*/

  for (long i1 = 1; i1 <= 9; i1++) {
    if (i1 != 5) {
      light_line(i1, y, x, 2);
    }
  }

  return true;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */

bool fb__ill_joke(const long a_cptr, const enum spell_effect_t typ,
                     const long dam, char *str,
                     char *str2) {
  char out_val[82];

  find_monster_name(str, a_cptr, false);
  find_monster_name(str2, a_cptr, true);
  /* with m_list[a_cptr] do; */
  /* with monster_templates[mptr] do; */
  if (!mon_save(a_cptr, 0, SC_MENTAL)) {
    m_list[a_cptr].confused = true;
    const long i2 = mon_take_hit(a_cptr, dam);
    if (typ == SE_ILLUSION) {
      sprintf(out_val, "%s seems to believe the illusion...", str2);
      msg_print(out_val);
      if (i2 > 0) {
        msg_print("and dies from fright!");
      } else {
        msg_print("and appears quite shaken.");
      }
    } else {

      sprintf(out_val, "The punch line strikes %str...", str);
      msg_print(out_val);
      if (i2 > 0) {
        msg_print("who dies in a fit of laughter!");
      } else {
        msg_print("who becomes weak from laughter!");
      }
    }

  } else {
    if (typ == SE_ILLUSION) {
      sprintf(out_val, "%s is unaffected.", str2);
      msg_print(out_val);
    } else {
      find_monster_name(str, a_cptr, true);
      sprintf(out_val, "%s appears offended....", str);
      msg_print(out_val);
      if (mon_take_hit(a_cptr, dam / 4) > 0) {
        msg_print("but your joke still knocks it dead!");
      }
    }
  }

  return true;
}
/*//////////////////////////////////////////////////////////////////// */
bool fire_bolt(const enum spell_effect_t typ, const long dir, long y, long x, long dam,
                  char bolt_typ[28]) {
  /*{ Shoot a bolt in a given direction                     -RAK-   }*/

  long dist;
  long weapon_type;
  long harm_type;
  obj_set *dummy;

  get_flags(typ, &weapon_type, &harm_type, &dummy);
  dist = 0;
  if (bolt_to_creature(dir, &y, &x, &dist, OBJ_BOLT_RANGE, true)) {
    char str2[82];
    char str[82];
    /* with cave[y][x]. do; */
    if (typ == SE_ILLUSION || typ == SE_JOKE) {
      fb__ill_joke(cave[y][x].cptr, typ, dam, str, str2);
    } else {
      /* with m_list[cave[y][x].cptr]. do; */
      /* with monster_templates[m_list[cave[y][x].cptr].mptr]. do; */
      const long cptr = cave[y][x].cptr;
      const long mptr = m_list[cptr].mptr;
      if (!mon_resists(cptr)) {
        char out_val[120];

        find_monster_name(str, cptr, false);
        find_monster_name(str2, cptr, true);
        sprintf(out_val, "The %s strikes %s.", bolt_typ, str);
        msg_print(out_val);
        if ((harm_type & monster_templates[mptr].cdefense) != 0) {
          dam *= 2;
        } else if ((weapon_type & monster_templates[mptr].spells) != 0) {
          dam /= 4;
        }
        if (mon_take_hit(cptr, dam) > 0) {
          sprintf(out_val, "%s dies in a fit of agony.", str2);
          msg_print(out_val);
        } else {
          if (panel_contains(y, x)) {
            print(monster_templates[mptr].symbol, y, x);
            m_list[cptr].ml = true;
          }
        }
      }
    }
  }
  return true;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool fire_ball(const enum spell_effect_t typ, const long dir, long y, long x,
                  const long dam_hp, char descrip[28]) {
  /*{ Shoot a ball in a given direction.  Note that balls have an   }*/
  /*{ area affect....                                       -RAK-   }*/

  long dist = 0;
  bool flag = false;

  if (bolt_to_creature(dir, &y, &x, &dist, OBJ_BOLT_RANGE, true)) {
    explode(typ, y, x, dam_hp, descrip);
  } else if (dist <= OBJ_BOLT_RANGE) {
    move_dir(10 - dir, &y, &x);
    flag = explode(typ, y, x, dam_hp, descrip);
  }

  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool wall_to_mud(const long dir, long y, long x) {
  /*{ Turn stone to mud, delete wall....                    -RAK-   }*/

  bool flag = false;
  bool return_value = false;

  do {
    move_dir(dir, &y, &x);
    /* with cave[y][x]. do; */
    if (in_bounds(y, x)) {
      char out_val[82];
      if (is_in(cave[y][x].fval, wall_set)) {
        flag = true;
        twall(y, x, 1, 0);
        if (test_light(y, x)) {
          msg_print("The wall turns into mud.");
          return_value = true;
        } else {
          unlite_spot(y, x);
        }
      } else if (cave[y][x].tptr > 0 && !cave[y][x].fopen) {
        /* kills doors? */
        flag = true;
        if (panel_contains(y, x)) {
          if (test_light(y, x)) {
            char out_val2[120];
            inven_temp.data = t_list[cave[y][x].tptr];
            objdes(out_val, &inven_temp, false);
            sprintf(out_val2, "The %s turns into mud.", out_val);
            msg_print(out_val2);
            return_value = true;
          }
        }
        delete_object(y, x);
      }

      const long cptr = cave[y][x].cptr;
      const long mptr = m_list[cptr].mptr;
      if (cptr > 1) {

        if ((0x0200 &
             monster_templates[m_list[cave[y][x].cptr].mptr].cdefense) != 0) {
          const long i1 = mon_take_hit(cptr, 100);
          flag = true;
          if (m_list[cptr].ml) {
            if (i1 > 0) {
              sprintf(out_val,
                      "The %s dies in a "
                      "fit of agony.",
                      monster_templates[mptr].name);
            } else {
              sprintf(out_val,
                      "The %s wails out "
                      "in pain!.",
                      monster_templates[mptr].name);
            }
            msg_print(out_val);
          }
        }
      }
    } else {
      flag = true;
    }
  } while (!flag);

  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool td_destroy2(const long dir, long y, long x) {
  /*{ Destroy all traps and doors in a given direction      -RAK-   }*/

  const obj_set thump_stuff = {chest,       unseen_trap, seen_trap,
                         closed_door, secret_door, 0};
  bool flag = false;

  do {
    move_dir(dir, &y, &x);
    /* with cave[y][x]. do; */
    if (cave[y][x].tptr > 0) {
      /* with t_list[cave[y][x].tptr]. do; */
      if (is_in(t_list[cave[y][x].tptr].tval, thump_stuff)) {
        if (delete_object(y, x)) {
          msg_print("There is a bright flash of "
                    "light!");
          cave[y][x].fopen = true;
          flag = true;
        }
      }
    }
  } while (cave[y][x].fopen);

  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool poly_monster(const long dir, long y, long x) {
  /*{ Polymorph a monster                                   -RAK-   }*/
  /*{ NOTE: cannot polymorph a winning creature (BALROG)            }*/

  long dist = 0;
  bool flag = false;
  bool return_value = false;

  do {
    if (bolt_to_creature(dir, &y, &x, &dist, OBJ_BOLT_RANGE, false)) {
      /* with cave[y][x]. do; */
      const long cptr = cave[y][x].cptr;
      if (!mon_save(cptr, 0, SC_NULL)) {
        if (!mon_resists(cptr)) {
          flag = true;
          delete_monster(cptr);
          place_monster(y, x, randint(m_level[MAX_MONS_LEVEL]) + m_level[0],
                        false);
          if (panel_contains(y, x)) {
            if (test_light(y, x)) {
              return_value = true;
            }
          }
        }
      } else {
        char out_val[82];
        sprintf(out_val, "The %s is unaffected.",
                monster_templates[m_list[cptr].mptr].name);
        msg_print(out_val);
      }
    } else {
      flag = true;
    }
  } while (!flag);

  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool build_wall(const long dir, long y, long x) {
  /*{ Create a wall...                                      -RAK-   }*/

  long i1 = 0;
  bool flag = false;

  move_dir(dir, &y, &x);
  while (cave[y][x].fopen && i1 < 10) {
    /* with cave[y,x] do; */
    if (cave[y][x].tptr > 0) {
      delete_object(y, x);
    }
    if (cave[y][x].cptr > 1 && !mon_resists(cave[y][x].cptr)) {
      mon_take_hit(cave[y][x].cptr, damroll("2d8"));
    }
    cave[y][x].fval = rock_wall2.ftval;
    cave[y][x].fopen = rock_wall2.ftopen;
    cave[y][x].fm = false;
    if (test_light(y, x)) {
      lite_spot(y, x);
    }
    i1++;
    flag = true;
    move_dir(dir, &y, &x);
  }

  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool clone_monster(const long dir, long y, long x) {
  /*{ Replicate a creature                                  -RAK-   }*/

  bool flag = false;

  if (move_to_creature(dir, &y, &x)) {
    /* with cave[y][x]. do; */
    if (!mon_resists(cave[y][x].cptr)) {
      multiply_monster(y, x, m_list[cave[y][x].cptr].mptr, false);
      if (panel_contains(y, x)) {
        if (m_list[cave[y][x].cptr].ml) {
          flag = true;
        }
      }
    }
  }
  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool disarm_all(const long dir, long y, long x) {
  /*{ Disarms all traps/chests in a given direction         -RAK-   }*/

  long oldy, oldx;
  bool flag = false;

  do {
    /* with cave[y][x]. do; */
    if (cave[y][x].tptr > 0) {
      /* with t_list[tptr] do; */
      const long tval = t_list[cave[y][x].tptr].tval;
      if (tval == unseen_trap || tval == seen_trap) {
        if (delete_object(y, x)) {
          flag = true;
        }
      } else if (tval == closed_door) {
        t_list[cave[y][x].tptr].p1 = 0;
      } else if (tval == secret_door) {
        cave[y][x].fval = corr_floor3.ftval;
        change_trap(y, x);
        cave[y][x].fm = true;
        flag = true;
      } else if (tval == chest) {
        if (t_list[cave[y][x].tptr].flags > 0) {
          msg_print("Click!");
          t_list[cave[y][x].tptr].flags &= 0xFFFFFE0F; /* detrap */
          t_list[cave[y][x].tptr].flags &= 0xFFFFFFFE; /* unlock */
          flag = true;
          char *achar = strstr(t_list[cave[y][x].tptr].name, " (");
          if (achar != NULL) {
            *achar = 0;
          }
          strcat(t_list[cave[y][x].tptr].name, " (Unlocked)");
          known2(t_list[cave[y][x].tptr].name);
        }
      }
    }
    oldy = y;
    oldx = x;
    move_dir(dir, &y, &x);
  } while (cave[oldy][oldx].fopen);

  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool teleport_monster(const long dir, long y, long x) {
  /*{ Teleport all creatures in a given direction away      -RAK-   }*/

  bool flag = false;

  while (move_to_creature(dir, &y, &x)) {
    teleport_away(cave[y][x].cptr, MAX_SIGHT);
    flag = true;
  }

  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
static bool zm__did_it_work(const enum spell_effect_t zaptype,
                               const long cptr, const long aux) {
  bool flag;

  /* with cave[y,x] do; */
  /* with monster_templates[m_list[cptr].mptr] do; */

  switch (zaptype) {
  case SE_SLEEP:
  case SE_CONFUSE:
    flag = !mon_save(cptr, 0, SC_MENTAL) && !mon_resists(cptr);
    break;

  case SE_DRAIN:
    flag = (monster_templates[m_list[cptr].mptr].cdefense & 0x0008) == 0 &&
           !mon_resists(cptr);
    break;

  case SE_SPEED:
    flag = (!mon_save(cptr, 0, SC_NULL) && !mon_resists(cptr)) || aux > 0;
    break;

  case SE_HOLD:
    flag = !mon_save(cptr, 0, SC_HOLD) && !mon_resists(cptr);
    break;

  default:
    flag = true;
    break;
  }
  return flag;
}

/*//////////////////////////////////////////////////////////////////// */
void zm__yes_it_did(const enum spell_effect_t zaptype, const long cptr,
                    const long aux,
                    char *str1, char *str2) {
  long i1;
  char out_val[82];

  /* with cave[y,x] do; */
  /* with m_list[cptr]. do; */

  switch (zaptype) {
  case SE_PROBE:
    sprintf(out_val, "The mysterious ray strikes %s.", str2);
    msg_print(out_val);
    msg_print("A voice booms down from above!  It says..");
    sprintf(out_val, "This monster has %d hp left.", m_list[cptr].hp);
    msg_print(out_val);
    break;

  case SE_SLEEP:
    m_list[cptr].csleep = 500;
    sprintf(out_val, "%s falls asleep.", str1);
    msg_print(out_val);
    break;

  case SE_CONFUSE:
    m_list[cptr].confused = true;
    m_list[cptr].csleep = 0;
    sprintf(out_val, "%s appears confused.", str1);
    msg_print(out_val);
    break;

  case SE_HP:
  case SE_DRAIN:
    if (mon_take_hit(cptr, aux) > 0) {
      sprintf(out_val, "%s dies in a fit of agony.", str1);
    } else {
      sprintf(out_val, "%s screams in agony.", str1);
    }
    msg_print(out_val);
    break;

  case SE_SPEED:
    m_list[cptr].cspeed += aux;
    m_list[cptr].csleep = 0;
    break;

  case SE_HOLD:
    sprintf(out_val, "%s appears frozen!", str1);
    msg_print(out_val);
    i1 = m_list[cptr].stunned + aux;
    if (i1 > 31) {
      i1 = 31;
    }
    m_list[cptr].stunned = i1;
    break;

  default:
    break;
  }
}
/*//////////////////////////////////////////////////////////////////// */
bool zm__no_it_didnt(const enum spell_effect_t zaptype, char *str1) {
  /*{ returns true for item idents }*/

  const obj_set things_you_can_know = {SE_ILLUSION, SE_SLEEP, SE_CONFUSE, 0};

  const bool flag = is_in(zaptype, things_you_can_know);

  if (zaptype != SE_DRAIN) {
    char out_val[82];
    sprintf(out_val, "%s is unaffected.", str1);
    msg_print(out_val);
  }

  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
bool zap_monster(const long dir, long y, long x, const long aux,
                    const long zaptype) {
  /*{ Contains all aimed spell effects that stop at first victim.  New
    spell effects should be put into constants.inc.  Aux is used for
    damage
    or speed change if used.}*/

  bool flag = false;

  if (move_to_creature(dir, &y, &x)) {
    const long cptr = cave[y][x].cptr;
    char str1[82];
    char str2[82];

    find_monster_name(str1, cptr, true);
    find_monster_name(str2, cptr, false);
    if (zm__did_it_work(zaptype, cptr, aux)) {
      zm__yes_it_did(zaptype, cptr, aux, str1, str2);
      flag = true;
    } else if (zm__no_it_didnt(zaptype, str1)) {
      flag = true;
    }
  }
  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool bolt_to_creature(const long dir, long *y, long *x, long *dist,
                         const long max_dist, const bool visable) {
  bool hit_creature = false;

  do {
    const long oldy = *y;
    const long oldx = *x;
    move_dir(dir, y, x);
    (*dist)++;
    if (visable) { /*{ erase bolt }*/
      if (test_light(oldy, oldx)) {
        lite_spot(oldy, oldx);
      } else {
        unlite_spot(oldy, oldx);
      }
    }
    if (*dist <= max_dist) {
      /* with cave[y][x]. do; */
      if (cave[*y][*x].fopen) {
        hit_creature = cave[*y][*x].cptr > 1;
        if (visable && !hit_creature) {
          if (panel_contains(*y, *x)) {
            print('*', *y, *x); /*{ draw bolt }*/
            refresh();
            usleep(DRAW_BOLT_DELAY);
          }
        }
      }
    }
  } while (!(!cave[*y][*x].fopen || *dist > max_dist || hit_creature));

  return hit_creature;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool move_to_creature(const long dir, long *y, long *x) {
  long dist = 0;

  const bool hit_creature = bolt_to_creature(dir, y, x, &dist, 999, false);
  return hit_creature;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool am_i_dumb(void) { return player_lev < randint(randint(50)); }
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool lore_spell(void) {
  /*{ Give name for most items in inventory -Cap'n- }*/

  treas_rec *thingy = inventory_list;
  while (thingy != NULL) {
    if (!am_i_dumb()) {
      identify(&thingy->data);
      known2(thingy->data.name);
    }
    thingy = thingy->next;
  }

  return true;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool detect_curse(void) {
  char trash_char;
  treas_rec *item_ptr;
  bool redraw;
  bool flag = false;

  redraw = false;
  inventory_change_all_ok_stats(true, true);
  if (get_item(&item_ptr, "Item you wish to examine?", &redraw, inven_ctr,
               &trash_char, false, false)) {
    /* with item_ptr->data. do; */
    if ((Cursed_worn_bit & item_ptr->data.flags) != 0) {
      item_ptr->data.flags2 |= Known_cursed_bit;
      msg_print("The item is cursed!");
    } else {
      msg_print("This item is not cursed.");
    }
    flag = true;
  }
  if (redraw) {
    msg_print(" ");
    draw_cave();
  }

  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool detect_magic(void) {
  /*{ Determine whether an item is magical or not           -Cap'n- }*/

  treas_rec *item_ptr;
  char trash_char;
  bool redraw;
  bool flag = false;

  redraw = false;
  const bool dumb = am_i_dumb();
  const bool dumber = dumb && am_i_dumb();
  inventory_change_all_ok_stats(true, true);
  if (get_item(&item_ptr, "Item you wish to examine?", &redraw, inven_ctr,
               &trash_char, false, false)) {
    /* with item_ptr->data. do; */
    if (((item_ptr->data.flags != 0 || item_ptr->data.tohit > 0 ||
          item_ptr->data.todam > 0 || item_ptr->data.toac > 0) &&
         !dumb) ||
        dumber) {
      msg_print("The item seems magical!");
    } else {
      msg_print("The item does not seem to be magical...");
    }
    flag = true;
  }
  if (redraw) {
    msg_print(" ");
    draw_cave();
  }

  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool create_water(__attribute__((unused)) long y,
                     __attribute__((unused)) long x) {
  /*{ Creates an area of water on open floor                -Cap'n- }*/
  /* XXXX no code existed */

  return true;
}
/*//////////////////////////////////////////////////////////////////// */

bool destroy_water(__attribute__((unused)) long y,
                      __attribute((unused)) long x) {
  /*{ Makes an area of water into open floor                -Cap'n- }*/
  /* XXXX no code existed */

  return true;
}
/*//////////////////////////////////////////////////////////////////// */

bool item_petrify(void) {
  /*{ Does a petrify attack on the fool that used the bad item -Capn-}*/
  /* XXXX no code existed */
  return true;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool creeping_doom(const long dir, long y, long x, const long dam_hp,
                      const long range,
                      char ddesc[28]) {
  /*{ Creeping doom type spells, a missile, but with a set range    }*/

  long dist;

  dist = 0;

  if (bolt_to_creature(dir, &y, &x, &dist, range, true)) {
    /* with cave[y,x] do; */
    /* with m_list[cptr] do; */
    /* with monster_templates[mptr] do; */
    const long cptr = cave[y][x].cptr;
    const long mptr = m_list[cptr].mptr;

    if (!mon_resists(cptr)) {
      char out_val[82];
      sprintf(out_val, "The %s hits the %s.", ddesc,
              monster_templates[mptr].name);
      msg_print(out_val);
      if (mon_take_hit(cptr, dam_hp) > 0) {
        sprintf(out_val, "The %s dies in a fit of agony.",
                monster_templates[mptr].name);
        msg_print(out_val);
      } else {
        if (panel_contains(y, x)) {
          print(monster_templates[mptr].symbol, y, x);
          m_list[cptr].ml = true;
        }
      }
    }
  }
  return true;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
bool fire_line(const enum spell_effect_t typ, const long dir, long y, long x,
                  long dam_hp, char descrip[28]) {
  /*{ Fire a spell that affects a line of monsters                  }*/

  long dist;
  long weapon_type, harm_type;
  obj_set *dummy;

  get_flags(typ, &weapon_type, &harm_type, &dummy);
  dist = 0;
  while (bolt_to_creature(dir, &y, &x, &dist, OBJ_BOLT_RANGE, true)) {
    /* with cave[y,x] do; */
    /* with m_list[cptr] do; */
    /* with monster_templates[mptr] do; */
    const long cptr = cave[y][x].cptr;
    const long mptr = m_list[cptr].mptr;
    if (!mon_resists(cptr)) {
      char out_val[82];
      sprintf(out_val, "The %s strikes the %s.", descrip,
              monster_templates[mptr].name);
      msg_print(out_val);
      if ((harm_type & monster_templates[mptr].cdefense) != 0) {
        dam_hp *= 2;
      } else if ((weapon_type & monster_templates[mptr].spells) != 0) {
        dam_hp /= 4;
      }

      if (mon_take_hit(cptr, dam_hp) > 0) {
        sprintf(out_val, "The %s dies in a fit of agony.",
                monster_templates[mptr].name);
        msg_print(out_val);
      } else {
        if (panel_contains(y, x)) {
          print(monster_templates[mptr].symbol, y, x);
          m_list[cptr].ml = true;
        }
      }
    }
  }

  return true;
}

void teleport(const long dis) {

  long y, x;

  ENTER(("teleport", "%d", dis));

  do {
    y = randint(cur_height);
    x = randint(cur_width);
    while (distance(y, x, char_row, char_col) > dis) {
      y += trunc((char_row - y) / 2);
      x += trunc((char_col - x) / 2);
    }
  } while (!(cave[y][x].fopen && cave[y][x].cptr < 2));

  move_creature(char_row, char_col, y, x);
  for (long i1 = char_row - 1; i1 <= char_row + 1; i1++) {
    for (long i2 = char_col - 1; i2 <= char_col + 1; i2++) {
      /* with cave[i1,i2] do; */
      cave[i1][i2].tl = false;
      if (!test_light(i1, i2)) {
        unlite_spot(i1, i2);
      }
    }
  }

  if (test_light(char_row, char_col)) {
    lite_spot(char_row, char_col);
  }

  char_row = y;
  char_col = x;
  player_action_move(5);
  creatures(false);
  teleport_flag = false;

  LEAVE("teleport", "d");
}