/* creature.c */
/**/

#include <curses.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "constants.h"
#include "debug.h"
#include "effects.h"
#include "fighting/fighting.h"
#include "generate_monster/generate_monster.h"
#include "generate_monster/monster_template.h"
#include "inventory/inven.h"
#include "io.h"
#include "magic.h"
#include "misc.h"
#include "monsters.h"
#include "pascal.h"
#include "player.h"
#include "player_action.h"
#include "random.h"
#include "screen.h"
#include "spells.h"
#include "text_lines.h"
#include "types.h"
#include "variables.h"

#include "creature.h"

#define ML(mmm) (m_list[(mmm)])
#define MY(mmm) (m_list[(mmm)].fy)
#define MX(mmm) (m_list[(mmm)].fx)
#define OBJ_RUNE_PROT 3000 /*{ Rune of protection resistance	} */

typedef long mm_type[6];

static const long mon_mult_adj = 7; // High value slows multiplication

static void get_player_move_rate(void) {

  /* with player_flags do; */
  if (is_in(cave[char_row][char_col].fval, earth_set)) {
    player_flags.move_rate = 4;
  } else {
    const long cur_swim = (player_flags.swim + randint(5) - 1) / 5;

    if (cur_swim <= -2) {
      player_flags.move_rate = 0;
    } else if (cur_swim == -1) {
      player_flags.move_rate = 1;
    } else if (cur_swim == 0) {
      player_flags.move_rate = 2;
    } else if (cur_swim == 1) {
      player_flags.move_rate = 4;
    } else {
      player_flags.move_rate = 8;
    }
  }
}

/*{ Given speed, returns number of moves this turn.       -RAK-   }*/
/*{ NOTE: Player must always move at least once per iteration,    }*/
/*{       a slowed player is handled by moving monsters faster    }*/
static long movement_rate(const long cspeed, const long mon) {

  /*{ final speed as long }*/
  long c_rate; /*{ rate (0,1,2,3) = (0,1/4,1/2,1)
                             _                in wrong element }*/
  long return_value;

  /* with m_list[mon] do; */
  /* with monster_templates[mptr] do; */
  /* with cave[fy,fx] do; */
  uint8_t const monster_y = m_list[mon].fy;
  uint8_t const monster_x = m_list[mon].fx;
  if (xor(is_in(cave[monster_y][monster_x].fval, earth_set) ||
              is_in(cave[monster_y][monster_x].fval, pwall_set),
          (monster_templates[m_list[mon].mptr].cmove & 0x00000010) == 0)) {
    c_rate =
        (long)((monster_templates[m_list[mon].mptr].cmove & 0x00000300) / 256);
  } else {
    c_rate = 3;
  }

  if (c_rate == 3) {
    c_rate = 4; /* I wish I knew why they did this... rounding up? */
  }

  long py_rate = player_flags.move_rate;

  if (cspeed > 0) {
    c_rate *= cspeed;
  } else {
    py_rate *= 2 - cspeed;
  }

  long final_rate = c_rate / py_rate;

  if (c_rate * turn % py_rate < c_rate % py_rate) {
    final_rate++;
  }

  /* { if player resting, max monster move = 1 } */
  if (final_rate > 1 && player_flags.rest > 0) {
    return_value = 1;
  } else {
    return_value = final_rate;
  }

  return return_value;
}

uint8_t get_creature_at_location(coords const *coord) {
  return cave[coord->y][coord->x].cptr;
}

bool is_spot_empty_of_creatures(coords const *coord) {
  return cave[coord->y][coord->x].cptr == 0;
}

bool is_monster_at_location(coords const *coord) {
  return cave[coord->y][coord->x].cptr > 1;
}

void move_creature(const long from_y, const long from_x, const long to_y,
                   const long to_x) {
  const coords from = {.y = from_y, .x = from_x};
  const unsigned char character_to_move = get_creature_at_location(&from);
  cave[from_y][from_x].cptr = 0;
  cave[to_y][to_x].cptr = character_to_move;
}

void check_mon_lite(const long y, const long x) {
  const coords location = {.y = y, .x = x};
  if (is_monster_at_location(&location)) {
    if (!m_list[cave[y][x].cptr].ml) { // if (!lit up)
      if (cave[y][x].tl || cave[y][x].pl) {
        if (los(char_row, char_col, y, x)) {
          m_list[cave[y][x].cptr].ml = true;
          lite_spot(y, x);
        }
      }
    }
  }
}

/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*///////////           Begin the insanity               ///////////// */

static void c__update_mon(const long monptr, long *hear_count) {
  long h_range, s_range;

  /*  ENTER("c__update_mon", "c") */

  /* with m_list[monptr]. do; */
  /* with cave[MY(monptr)][MX(monptr)]. do; */
  bool flag = false;

  if (is_in(cave[MY(monptr)][MX(monptr)].fval, water_set) &&
      (monster_templates[ML(monptr).mptr].cmove & 0x00800000) == 0) {
    /*{in water, not flying}*/
    h_range = 10;
    s_range = 5;
  } else {
    h_range = -1;
    s_range = MAX_SIGHT;
  }

  if (player_flags.blind < 1 && panel_contains(MY(monptr), MX(monptr))) {
    if (wizard2) {
      flag = true;
    } else if (ML(monptr).cdis <= s_range) {
      if (los(char_row, char_col, MY(monptr), MX(monptr))) {
        /* with monster_templates[mptr] do; */
        if (cave[MY(monptr)][MX(monptr)].pl ||
            cave[MY(monptr)][MX(monptr)].tl) { /*{can see creature?}*/
          flag = player_flags.see_inv ||
                 (0x10000 & monster_templates[ML(monptr).mptr].cmove) == 0;
        } else if (player_flags.see_infra > 0) { /*{infravision?}*/
          flag = ML(monptr).cdis <= player_flags.see_infra &&
                 (0x2000 & monster_templates[ML(monptr).mptr].cdefense) != 0;
        }
      }
    }
  }

  if (ML(monptr).cdis <= h_range && /*{noise in water?}*/
      los(char_row, char_col, MY(monptr), MX(monptr)) && !flag) {
    (*hear_count)++;
  }

  if (flag) {
    /*{ Light it up...        }*/
    if (!ML(monptr).ml) {
      print(monster_templates[ML(monptr).mptr].symbol, MY(monptr), MX(monptr));
      ML(monptr).ml = true;
      if (search_flag) {
        search_off();
      }
      if (player_flags.rest > 0) {
        rest_off();
      }
      flush();
      if (find_flag) {
        find_flag = false;
        player_action_move(5);
      }
    }
  } else if (ML(monptr).ml) {
    /*{ Turn it off...        }*/
    ML(monptr).ml = false;
    if (cave[MY(monptr)][MX(monptr)].tl || cave[MY(monptr)][MX(monptr)].pl) {
      lite_spot(MY(monptr), MX(monptr));
    } else {
      unlite_spot(MY(monptr), MX(monptr));
    }
  }

  /*  LEAVE("c__update_mon", "c") */
}

static void c__monster_eaten_message(char const *squash, char const *doesit,
                                     const long cptr) {
  char out_val[1026];

  ENTER(("c__monster_eaten_message", "c"));

  switch (randint(10)) {
  case 1:
    sprintf(out_val, "Uh oh...it looks like the %s is in need of first aid.",
            squash);
    break;

  case 2:
    sprintf(out_val, "*splat* *crunch* *gobble* *BUUUUUUURP*");
    break;

  case 3:
    sprintf(out_val, "Look out!  The %s is going to-- Eeeeew...never mind.",
            doesit);
    break;

  case 4:
    sprintf(out_val, "Ick...the %s has %s all over his toes.", doesit, squash);

    break;

  case 5:
    sprintf(out_val, "The nice %s took out the %s for you.", doesit, squash);
    break;

  case 6:
    sprintf(out_val, "WoWEE, Auggie Ben-Doggie!  The %s just got blatted!",
            squash);
    break;

  case 7:
    sprintf(out_val, "The %s Society will not appreciate this. . .", squash);
    break;

  case 8:
    sprintf(out_val, "The %s is not amused.", squash);
    break;

  case 9:
    sprintf(out_val, "The %s pauses to clean the %s off.", doesit, squash);
    break;

  case 10:
    sprintf(out_val, "Aw, darn.  There goes %ld experience!",
            monster_templates[m_list[cptr].mptr].mexp);
    break;
  }

  msg_print(out_val);

  LEAVE("c__monster_eaten_message", "c");
}

static bool c__check_for_hit(const long monptr, const long atype) {
  bool flag = false;

  ENTER(("c__check_for_hit", "c"));

  const long level = monster_templates[m_list[monptr].mptr].level;
  const long armor_stuff = player_pac + player_ptoac;

  switch (atype) {
  case 1: /*{Normal attack  }*/
    flag = managed_to_hit(60, level, 0, armor_stuff);
    break;

  case 2: /*{Poison Strength}*/
    flag = managed_to_hit(-3, level, 0, armor_stuff);
    break;

  case 3: /*{Confusion attack}*/
    flag = managed_to_hit(10, level, 0, armor_stuff);
    break;

  case 4: /*{Fear attack    }*/
    flag = managed_to_hit(10, level, 0, armor_stuff);
    break;

  case 5: /*{Fire attack    }*/
    flag = managed_to_hit(10, level, 0, armor_stuff);
    break;

  case 6: /*{Acid attack    }*/
    flag = managed_to_hit(0, level, 0, armor_stuff);
    break;

  case 7: /*{Cold attack    }*/
    flag = managed_to_hit(10, level, 0, armor_stuff);
    break;

  case 8: /*{Lightning attack}*/
    flag = managed_to_hit(10, level, 0, armor_stuff);
    break;

  case 9: /*{Corrosion attack}*/
    flag = managed_to_hit(0, level, 0, armor_stuff);
    break;

  case 10: /*{Blindness attack}*/
    flag = managed_to_hit(2, level, 0, armor_stuff);
    break;

  case 11: /*{Paralysis attack}*/
    flag = managed_to_hit(2, level, 0, armor_stuff);
    break;

  case 12: /*{Steal Money    }*/
    flag = managed_to_hit(5, level, 0, player_lev) && player_money[TOTAL_] > 0;
    break;

  case 13: /*{Steal Object   }*/
    flag = managed_to_hit(2, level, 0, player_lev) && inven_ctr > 0;
    break;

  case 14: /*{Poison         }*/
    flag = managed_to_hit(5, level, 0, armor_stuff);
    break;

  case 15: /*{Lose dexterity}*/
    flag = managed_to_hit(0, level, 0, armor_stuff);
    break;

  case 16: /*{Lose constitution}*/
    flag = managed_to_hit(0, level, 0, armor_stuff);
    break;

  case 17: /*{Lose intelligence}*/
    flag = managed_to_hit(2, level, 0, armor_stuff);
    break;

  case 18: /*{Lose wisdom}*/
    flag = managed_to_hit(0, level, 0, armor_stuff);
    break;

  case 19: /*{Lose experience}*/
    flag = managed_to_hit(5, level, 0, armor_stuff);
    break;

  case 20: /*{Aggravate monsters}*/
    flag = true;
    break;

  case 21: /*{Disenchant        }*/
    flag = managed_to_hit(20, level, 0, armor_stuff);
    break;

  case 22: /*{Eat food          }*/
    flag = managed_to_hit(5, level, 0, armor_stuff);
    break;

  case 23: /*{Eat light         }*/
    flag = managed_to_hit(5, level, 0, armor_stuff);
    break;

  case 24: /*{Eat charges       }*/
    flag = managed_to_hit(15, level, 0, armor_stuff);
    break;

  case 25: /*{Lose charisma     }*/
    flag = managed_to_hit(2, level, 0, armor_stuff);
    break;

  case 26: /*{Petrification     }*/
    flag = managed_to_hit(10, level, 0, armor_stuff);
    break;

  case 27: /*{POISON poison     }*/
    flag = managed_to_hit(5, level, 0, armor_stuff);
    break;

  case 99: /* protevil or protmon repelled the attack */
    flag = true;
    break;

  default:
    flag = false;
    break;

  } /* end switch */

  RETURN("c__check_for_hit", "c", 'b', "test hit:", &flag);
  return flag;
}

static void c__print_attack(const long adesc, char *cdesc) {
  char the_attack[134];
  bool no_print = false;

  ENTER(("c__print_attack", "%ld,len: %d >%s<", adesc, strlen(cdesc), cdesc));
  strcpy(the_attack, cdesc);

  switch (adesc) {
  case 1:
    strcat(the_attack, "hits you.");
    break;
  case 2:
    strcat(the_attack, "bites you.");
    break;
  case 3:
    strcat(the_attack, "claws you.");
    break;
  case 4:
    strcat(the_attack, "stings you.");
    break;
  case 5:
    strcat(the_attack, "touches you.");
    break;
  case 6:
    strcat(the_attack, "kicks you.");
    break;
  case 7:
    strcat(the_attack, "gazes at you.");
    break;
  case 8:
    strcat(the_attack, "breathes on you.");
    break;
  case 9:
    strcat(the_attack, "spits on you.");
    break;
  case 10:
    strcat(the_attack, "makes a horrible wail.");
    break;
  case 11:
    strcat(the_attack, "embraces you.");
    break;
  case 12:
    strcat(the_attack, "crawls on you.");
    break;
  case 13:
    strcat(the_attack, "releases a cloud of spores.");
    break;
  case 14:
    strcat(the_attack, "begs you for money.");
    break;
  case 15:
    strcat(the_attack, "You've been slimed!");
    break;
  case 16:
    strcat(the_attack, "crushes you.");
    break;
  case 17:
    strcat(the_attack, "tramples you.");
    break;
  case 18:
    strcat(the_attack, "drools on you.");
    break;
  case 19:
    switch (randint(9)) {
    case 1:
      strcat(the_attack, "insults you!");
      break;
    case 2:
      strcat(the_attack, "insults your mother!");
      break;
    case 3:
      strcat(the_attack, "gives you the finger!");
      break;
    case 4:
      strcat(the_attack, "humiliates you!");
      break;
    case 5:
      strcat(the_attack, "wets on your leg!");
      break;
    case 6:
      strcat(the_attack, "defiles you!");
      break;
    case 7:
      strcat(the_attack, "dances around you!");
      break;
    case 8:
      strcat(the_attack, "makes obscene gestures!");
      break;
    case 9:
      strcat(the_attack, "moons you!!!");
      break;
    }
    break;
  case 23:
    strcat(the_attack, "sings a charming song");
    break;
  case 24:
    strcat(the_attack, "kisses you");
    break;
  case 25:
    strcat(the_attack, "gores you");
    break;
  case 26:
    switch (randint(2)) {
    case 1:
      strcat(the_attack, "moos forlornly");
      break;
    case 2:
      strcat(the_attack, "questioningly looks at you");
      break;
    }
    break;
  case 27:
    strcat(the_attack, "shocks you");
    break;
  case 28:
    strcat(the_attack, "squirts ink at you");
    break;
  case 29:
    strcat(the_attack, "entangles you");
    break;
  case 30:
    strcat(the_attack, "sucks your blood");
    break;
  case 31:
    strcat(the_attack, "goes for your throat!");
    break;
  case 32:
    strcat(the_attack, "blows bubbles at you");
    break;
  case 33:
    strcat(the_attack, "squawks at you");
    break;
  case 34:
    strcat(the_attack, "pecks at you");
    break;
  case 35:
    strcat(the_attack, "barks at you");
    break;
  case 36:
    strcat(the_attack, "rubs against your leg");
    break;
  case 37:
    strcat(the_attack, "follows you around");
    break;
  case 99:
    strcat(the_attack, "is repelled.");
    break;

  case 0:
  default:
    no_print = true;
    break;
  } /* end switch */

  if (!no_print) {
    msg_print(the_attack);
  }

  LEAVE("c__print_attack", "c");
}

static void c__apply_attack(const long monptr, const long atype, char ddesc[82],
                            const char *damstr) {
  long dam;
  long i1, i2;
  bool flag;
  treas_rec *item_ptr;
  const obj_set food_stuffs = {Food, 0};
  const obj_set staff_rod_or_wand = {staff, rod, wand, 0};

  ENTER(("c__apply_attack", "c"));

  const long level = monster_templates[m_list[monptr].mptr].level;

  switch (atype) {
  case 1: /*{Normal attack  }*/
    dam = damroll(damstr);
    /* with player_do; */
    dam -= (long)((player_pac + player_ptoac) / 200.0 * dam + .5);
    take_hit(dam, ddesc);
    prt_stat_block();
    break;

  case 2: /*{Poison Strength}*/
    take_hit(damroll(damstr), ddesc);
    lose_stat(STR, "You feel weaker.",
              "You feel weaker for a moment, then it passes.");
    prt_stat_block();
    break;

  case 3: /*{Confusion attack}*/
    /* with player_flags do; */
    take_hit(damroll(damstr), ddesc);
    if (randint(2) == 1) {
      if (player_flags.confused < 1) {
        msg_print("You feel confused.");
        player_flags.confused += randint(level);
      }
      player_flags.confused += 3;
    }
    prt_stat_block();
    break;

  case 4: /*{Fear attack    }*/
    /* with player_flags do; */
    take_hit(damroll(damstr), ddesc);
    if (player_spell_saves()) {
      msg_print("You resist the effects!");
    } else if (player_flags.afraid < 1) {
      msg_print("You are suddenly afraid!");
      player_flags.afraid += 3 + randint(level);
    } else {
      player_flags.afraid += 3;
    }
    prt_stat_block();
    break;

  case 5: /*{Fire attack    }*/
    msg_print("You are enveloped in flames!");
    fire_dam(damroll(damstr), ddesc);
    break;

  case 6: /*{Acid attack    }*/
    msg_print("You are covered in acid!");
    acid_dam(damroll(damstr), ddesc);
    break;

  case 7: /*{Cold attack    }*/
    msg_print("You are covered with frost!");
    cold_dam(damroll(damstr), ddesc);
    break;

  case 8: /*{Lightning attack}*/
    msg_print("Lightning strikes you!");
    light_dam(damroll(damstr), ddesc);
    break;

  case 9: /*{Corrosion attack}*/
    msg_print("A stinging red gas swirls about you.");
    corrode_gas(ddesc);
    take_hit(damroll(damstr), ddesc);
    prt_stat_block();
    break;

  case 10: /*{Blindness attack}*/
    /* with player_flags do; */
    take_hit(damroll(damstr), ddesc);
    if (player_flags.blind < 1) {
      player_flags.blind += 10 + randint(level);
      msg_print("Your eyes begin to sting.");
      msg_print(" ");
    }
    player_flags.blind +=
        5; /* blind the first time is worse than cumulitave blind */
    prt_stat_block();
    break;

  case 11: /*{Paralysis attack}*/
    /* with player_flags do; */
    take_hit(damroll(damstr), ddesc);
    if (player_spell_saves()) {
      msg_print("You resist the effects!");
    } else if (player_flags.paralysis < 1) {
      if (player_flags.free_act || player_flags.free_time > 0) {
        msg_print("You are unaffected.");
      } else {
        /* new paralysis overwrites old one, otherwise
         * you become dead fast */
        player_flags.paralysis = randint(level) + 3;
        msg_print("You are paralyzed.");
      }
    }
    prt_stat_block();
    break;

  case 12: /*{Steal Money     }*/
    /* with player_do; */
    if (randint(256) < C_player_get_stat(DEX) * 10 &&
        player_flags.paralysis < 1) {
      msg_print("You quickly protect your money pouch!");
    } else {
      if (player_money[TOTAL_] > 0) {
        subtract_money(randint(5) * (player_money[TOTAL_] * GOLD_VALUE) / 100,
                       false);
        msg_print("Your purse feels lighter.");
        prt_stat_block();
      }
    }
    if (randint(2) == 1) {

      msg_print("There is a puff of smoke!");
      teleport_away(monptr, MAX_SIGHT);
    }

    break;

  case 13: /*{Steal Object   }*/
    /* with py.stat do; */
    if (randint(256) < C_player_get_stat(DEX) * 10 &&
        player_flags.paralysis < 1) {
      msg_print("You grab hold of your backpack!");
    } else {
      item_ptr = inventory_list;
      for (i1 = randint(inven_ctr) - 1; i1 > 0; --i1) {
        if (item_ptr->next == NULL) {
          break;
        }
        item_ptr = item_ptr->next;
      }
      if (item_ptr != NULL) {
        if (!item_ptr->is_in) {
          if ((item_ptr->data.flags2 & Holding_bit) != 0) {
            if (item_ptr->insides == 0) {
              inven_destroy(item_ptr);
            }
          } else {
            inven_destroy(item_ptr);
          }
        } else {
          inven_destroy(item_ptr);
        }
        prt_stat_block();
        msg_print("Your backpack feels lighter.");
      }
    }

    if (randint(2) == 1) {
      msg_print("There is a puff of smoke!");
      teleport_away(monptr, MAX_SIGHT);
    }

    break;

  case 14: /*{Poison         }*/
    /* with player_flags do ; */
    take_hit(damroll(damstr), ddesc);
    prt_stat_block();
    msg_print("You feel very sick.");
    player_flags.poisoned += randint(level) + 5;
    break;

  case 15: /*{Lose dexterity }*/
    /* with player_flags do; */
    take_hit(damroll(damstr), ddesc);
    lose_stat(DEX, "You feel more clumsy",
              "You feel clumsy for a moment, then it passes.");
    prt_stat_block();
    break;

  case 16: /*{Lose constitution }*/
    /* with player_flags do; */
    take_hit(damroll(damstr), ddesc);
    lose_stat(CON, "Your health is damaged!",
              "Your body resists the effects of the disease.");
    prt_stat_block();
    break;

  case 17: /*{Lose intelligence }*/
    /* with player_flags do; */
    take_hit(damroll(damstr), ddesc);
    lose_stat(INT, "You feel your memories fading.",
              "You feel your memories fade, then they are restored!");
    prt_stat_block();
    break;

  case 18: /*{Lose wisdom      }*/
    /* with player_flags do; */
    take_hit(damroll(damstr), ddesc);
    lose_stat(WIS, "Your wisdom is drained.", "Your wisdom is sustained.");
    prt_stat_block();
    break;

  case 19: /*{Lose experience  }*/
    msg_print("You feel your life draining away!");
    i1 = damroll(damstr) + player_exp / 100 * MON_DRAIN_LIFE;
    lose_exp(i1);
    break;

  case 20: /*{Aggravate monster}*/
    aggravate_monster(5);
    break;

  case 21: /*{Disenchant       }*/
    flag = false;
    switch (randint(8)) {
    case 1:
      i1 = Equipment_primary;
      break;
    case 2:
      i1 = Equipment_armor;
      break;
    case 3:
      i1 = Equipment_belt;
      break;
    case 4:
      i1 = Equipment_shield;
      break;
    case 5:
      i1 = Equipment_cloak;
      break;
    case 6:
      i1 = Equipment_gloves;
      break;
    case 7:
      i1 = Equipment_bracers;
      break;
    case 8:
      i1 = Equipment_helm;
      break;
    default:
      MSG(("ERROR: randint returned an out of range value in "
           "c__apply_attack"));
      i1 = Equipment_primary;
      break;
    }

    /* with equipment[i1] do; */
    if (equipment[i1].tohit > 0) {
      equipment[i1].tohit -= randint(equipment[i1].tohit == 1 ? 1 : 2);
      flag = true;
    }
    if (equipment[i1].todam > 0) {
      equipment[i1].todam -= randint(equipment[i1].todam == 1 ? 1 : 2);
      flag = true;
    }
    if (equipment[i1].toac > 0) {
      equipment[i1].toac -= randint(equipment[i1].toac == 1 ? 1 : 2);
      flag = true;
    }

    if (flag) {
      msg_print("There is a static feeling in the air...");
      py_bonuses(&blank_treasure, 1);
    }
    break;

  case 22: /*{Eat food         }*/
    if (inventory_find_range(food_stuffs, false, &item_ptr, &i2)) {
      inven_destroy(item_ptr);
      prt_stat_block();
    }
    break;

  case 23: /*{Eat light        }*/
    /* with equipment[Equipment_light] do; */
    if (equipment[Equipment_light].p1 > 0) {
      equipment[Equipment_light].p1 -= 250 + randint(250);
      if (equipment[Equipment_light].p1 < 1) {
        equipment[Equipment_light].p1 = 1;
      }
      msg_print("Your light dims...");
    }
    break;

  case 24: /*{Eat charges     }*/
    if (inven_ctr > 0) {
      item_ptr = inventory_list;
      const long aning = randint(inven_ctr) - 1;
      for (i1 = 1; i1 <= aning; i1++) {
        item_ptr = item_ptr->next;
      }
      const long i4 = level;
      /* with item_ptr^.data do; */
      if (is_in(item_ptr->data.tval, staff_rod_or_wand)) {
        if (item_ptr->data.p1 > 0) {
          m_list[monptr].hp += i4 * item_ptr->data.p1;
          item_ptr->data.p1 = 0;
        }
      }
      msg_print("Energy drains from your pack!");
    }
    break;

  case 25: /*{Lose charisma   }*/
    /* with player_flags do; */
    take_hit(damroll(damstr), ddesc);
    lose_stat(CHR, "Your skin starts to itch.",
              "Your skin starts to itch, but feels better now.");
    prt_stat_block();
    break;

  case 26: /*{Petrification  }*/
    /* with player_flags do; */
    petrify(m_list[monptr].hp);
    break;

  case 27: /*{POISON Poison  }*/
    /* with player_flags do; */
    player_flags.poisoned += damroll(damstr);
    msg_print("You feel very sick.");
    break;

  case 99:
    break;

  default:
    break;
  } /* end switch */

  LEAVE("c__apply_attack", "c");
}

static void c__make_attack(const long monptr) {
  /*{ Make an attack on the player                          -RAK-   }*/

  long atype;
  long adesc; /*,dam;*/
  long acount;
  char attstr[82];
  char attx[82];
  char cdesc[82];
  char ddesc[82];

  ENTER(("c__make_attack", "c"));

  attstr[0] = 0;
  attx[0] = 0;

  strcpy(attstr, monster_templates[m_list[monptr].mptr].damage);
  find_monster_name(cdesc, monptr, true);
  strcat(cdesc, " ");

  /*{ For "DIED_FROM" string        }*/
  if ((0x80000000 & monster_templates[m_list[monptr].mptr].cmove) != 0) {
    sprintf(ddesc, "The %s", monster_templates[m_list[monptr].mptr].name);
  } else {
    sprintf(ddesc, "& %s", monster_templates[m_list[monptr].mptr].name);
  }
  strcpy(inven_temp.data.name, ddesc);
  inven_temp.data.number = 1;
  objdes(ddesc, &inven_temp, true);
  strcpy(died_from, ddesc);
  /*{ End DIED_FROM                 }*/

  while (attstr[0] != 0) {
    char damstr[36];

    /* attstr looks like this: "1 32 4d4|2 21 0d0" */
    char *achar = strchr(attstr, '|');
    if (achar != NULL) {
      strcpy(attx, attstr);
      achar = strchr(attx, '|');
      *achar = 0;
      achar++;
      strcpy(attstr, achar);
    } else {
      strcpy(attx, attstr);
      attstr[0] = 0;
    }

    sscanf(attx, "%ld %ld %s", &atype, &adesc, damstr);

    if (player_flags.protevil > 0) {
      if ((monster_templates[m_list[monptr].mptr].cdefense & 0x0004) != 0) {
        if (player_lev + 1 > monster_templates[m_list[monptr].mptr].level) {
          atype = 99;
          adesc = 99;
        }
      }
    }

    if (player_flags.protmon > 0) {
      if ((monster_templates[m_list[monptr].mptr].cdefense & 0x0002) != 0) {
        if (player_lev + 1 > monster_templates[m_list[monptr].mptr].level) {
          atype = 99;
          adesc = 99;
        }
      }
    }

    if ((achar = strstr(damstr, "-")) != NULL) {
      char s1[82];
      *achar = ' ';
      sscanf(damstr, "%ld %s", &acount, s1);
      strcpy(damstr, s1);
    } else {
      acount = 1;
    }

    /* with player_do; */
    for (long i5 = 1; i5 <= acount; i5++) {

      const bool flag = c__check_for_hit(monptr, atype);

      if (flag) {
        c__print_attack(adesc, cdesc);
        c__apply_attack(monptr, atype, ddesc, damstr);
      } else {
        char theattack[120];
        switch (adesc) {
        case 1:
        case 2:
        case 3:
        case 6:
          /* normal, poison strength, confusion
           * attack, acid attack */
          sprintf(theattack, "%smisses you.", cdesc);
          msg_print(theattack);
          break;
        default:
          break;
        }
      }
    } /* end for i5 */
  } /* end for attstr != null */

  LEAVE("c__make_attack", "c");
}

static bool c__make_move(const long monptr, mm_type mm, long *hear_count) {
  /*{ Make the move if possible, five choices               -RAK-   }*/

  long i2, newy, newx;
  bool return_value = false;

  ENTER(("c__make_move", "c"));

  long i1 = 1;
  bool flag = false;
  const unsigned long movebits = monster_templates[m_list[monptr].mptr].cmove;

  do {
    /*{ Get new positon               }*/
    newy = m_list[monptr].fy;
    newx = m_list[monptr].fx;
    move_dir(mm[i1], &newy, &newx);
    /* with cave[newy][newx]. do; */
    if (cave[newy][newx].fval != boundry_wall.ftval) {
      bool tflag = false;
      if (cave[newy][newx].cptr == 1) {
        tflag = true;
      } else if (cave[newy][newx].fopen) {
        if (is_in(cave[newy][newx].fval, floor_set)) {
          if ((movebits & 0x00000040) == 0) {
            tflag = true;
          } else if (!xor(is_in(cave[newy][newx].fval, earth_set),
                          (movebits & 0x00000010) == 0)) {
            tflag = true;
          }
        }
        /*{ Creature moves through walls? }*/
      } else if ((movebits & 0x40000) != 0) {
        tflag = true;
        /*{ Creature can open doors?      }*/
      } else if (cave[newy][newx].tptr > 0) {

        /* with t_list[cave[newy][newx].tptr]. do; */
        /* with m_list[monptr]. do; */
        if ((movebits & 0x20000) != 0) {
          /*{ Creature can open doors... }*/
          switch (t_list[cave[newy][newx].tptr].tval) {
          case closed_door:                              /*{ Closed doors...
                                                            }*/
            if (t_list[cave[newy][newx].tptr].p1 == 0) { /*{ Closed doors  }*/
              tflag = true;
              if (cave[newy][newx].fm) {
                if (los(char_row, char_col, newy, newx)) {
                  t_list[cave[newy][newx].tptr] = door_list[DL_OPEN];
                  cave[newy][newx].fopen = true;
                  lite_spot(newy, newx);
                  tflag = false;
                }
              }
            } else if (t_list[cave[newy][newx].tptr].p1 > 0) {
              /*{ Locked doors  }*/
              if (randint(100 - monster_templates[m_list[monptr].mptr].level) <
                  5) {
                t_list[cave[newy][newx].tptr].p1 = 0;
              }
            } else if (t_list[cave[newy][newx].tptr].p1 < 0) {
              /*{ Stuck doors   }*/
              if (randint(m_list[monptr].hp) >
                  10 + labs(t_list[cave[newy][newx].tptr].p1)) {
                t_list[cave[newy][newx].tptr].p1 = 0;
              }
            }
            break;

          case secret_door: /*{ Secret doors...
                               }*/
            tflag = true;
            if (cave[newy][newx].fm) {
              if (los(char_row, char_col, newy, newx)) {
                t_list[cave[newy][newx].tptr] = door_list[DL_OPEN];
                cave[newy][newx].fopen = true;
                lite_spot(newy, newx);
                tflag = false;
              }
            }
            break;

          default:
            break;
          }
        } else {
          /*  { Creature can not open doors, must
           * bash them   }*/
          switch (t_list[cave[newy][newx].tptr].tval) {
          case closed_door: /* { Closed doors...
                               }*/
            i2 = labs(t_list[cave[newy][newx].tptr].p1) + 20;
            if (randint(m_list[monptr].hp) > i2) {
              tflag = true;
              if (cave[newy][newx].fm) {
                if (los(char_row, char_col, newy, newx)) {
                  t_list[cave[newy][newx].tptr] = door_list[DL_OPEN];
                  t_list[cave[newy][newx].tptr].p1 = randint(2) - 1;
                  cave[newy][newx].fopen = true;
                  lite_spot(newy, newx);
                  tflag = false;
                }
              }
            }
            break;

          case secret_door: /* { Secret doors...
                               }*/
            break;

          default:
            break;
          }
        }
      }

      /* { Glyph of warding present?     }*/
      if (tflag) {
        if (cave[newy][newx].tptr > 0) {
          if (t_list[cave[newy][newx].tptr].tval == seen_trap) {
            if (t_list[cave[newy][newx].tptr].subval == 99) {
              if (randint(OBJ_RUNE_PROT) <
                  monster_templates[m_list[monptr].mptr].level) {
                if (newy == char_row && newx == char_col) {
                  msg_print("Th"
                            "e "
                            "ru"
                            "ne"
                            " o"
                            "f "
                            "pr"
                            "ot"
                            "ec"
                            "ti"
                            "on"
                            " i"
                            "s "
                            "br"
                            "ok"
                            "en"
                            "!");
                }
                delete_object(newy, newx);
              } else {
                tflag = false;
              }
            }
          }
        }
      }

      /* { Creature has attempted to move on player?     }*/
      if (tflag) {
        if (cave[newy][newx].cptr == 1) {

          if (!m_list[monptr].ml) {
            c__update_mon(monptr, hear_count);
          }

          if (find_flag) {
            find_flag = false;
            player_action_move(5);
          }

          c__make_attack(monptr);
          /* { Player has read a Confuse Monster?
           * }*/
          /* { Monster gets a saving throw... }*/
          if (player_flags.confuse_monster) {
            char out_val[82];
            /* with m_list[monptr] do; */
            /* with */
            /* monster_templates[m_list[monptr].mptr].
             */
            /* do; */
            msg_print("Your hands stop glowing.");
            player_flags.confuse_monster = false;
            if (mon_save(monptr, 0, SC_MENTAL)) {
              sprintf(out_val,
                      "The %s is "
                      "unaffected.",
                      monster_templates[m_list[monptr].mptr].name);
            } else {
              sprintf(out_val,
                      "The %s appears "
                      "confused.",
                      monster_templates[m_list[monptr].mptr].name);
              m_list[monptr].confused = true;
            }
            msg_print(out_val);
          }
          tflag = false;
          flag = true;

        } else { /* cptr != 1 */

          /*{ Creature is attempting to move on
           * other creature?     }*/
          if (cave[newy][newx].cptr > 1 &&
              (newy != m_list[monptr].fy || newx != m_list[monptr].fx)) {

            /*{ Creature eats other
             * creatures?        }*/
            if ((movebits & 0x80000) != 0) {
              if (m_list[cave[newy][newx].cptr].ml) {
                /*squash =
                 * monster_templates[m_list[cptr].mptr].name;*/
                /*doesit =
                 * monster_templates[m_list[monptr].mptr].name;*/
                c__monster_eaten_message(
                    monster_templates[m_list[cave[newy][newx].cptr].mptr].name,
                    monster_templates[m_list[monptr].mptr].name,
                    cave[newy][newx].cptr);
              }
              delete_monster(cave[newy][newx].cptr);
            } else {
              tflag = false;
            }
          }
        }
      } /* end if tflag */

      /*{ Creature has been allowed move...     }*/
      if (tflag) {
        /* with m_list[monptr] do */

        /*{ Pick up or eat an object              }*/
        if ((movebits & 0x100000) != 0) {
          /* with cave[newy,newx] do; */
          if (cave[newy][newx].tptr > 0) {
            if (t_list[cave[newy][newx].tptr].tval < valuable_metal) {
              delete_object(newy, newx);
            }
          }
        }

        /*{ Move creature record                  }*/
        move_creature(m_list[monptr].fy, m_list[monptr].fx, newy, newx);
        m_list[monptr].fy = newy;
        m_list[monptr].fx = newx;
        flag = true;
        return_value = true;
      }
    }
    i1++;
    /*{ Up to 5 attempts at moving, then give up...   }*/
  } while (!(flag || i1 > 5));

  RETURN("c__make_move", "c", 'b', "moved", &return_value);
  return return_value;
}

static bool c__move_confused(const long monptr, mm_type mm, long *hear_count) {
  bool return_value;

  ENTER(("c__move_confused", "c"));

  mm[1] = randint(9);
  mm[2] = randint(9);
  mm[3] = randint(9);
  mm[4] = randint(9);
  mm[5] = randint(9);
  return_value = c__make_move(monptr, mm, hear_count);

  RETURN("c__move_confused", "c", 'b', "moved", &return_value);
  return return_value;
}

static void c__get_moves(const long monptr, mm_type *mm) {
  /*{ Choose correct directions for monster movement        -RAK-   }*/

  ENTER(("c__get_moves", "c"));
  /*{ octant_side = +/-1 }*/

  if (m_list[monptr].csleep != 0) {
    m_list[monptr].csleep = 0;
  }

  const long y = char_row - m_list[monptr].fy;
  const long x = char_col - m_list[monptr].fx;

  const long move_val = get_hexdecant(y, x);
  const long octant_side = 2 * (move_val % 2) - 1;

  (*mm)[1] = key_of[move_val / 2];
  (*mm)[2] = rotate_dir((*mm)[1], octant_side);
  (*mm)[3] = rotate_dir((*mm)[1], -octant_side);
  (*mm)[4] = rotate_dir((*mm)[2], octant_side);
  (*mm)[5] = rotate_dir((*mm)[3], -octant_side);

  LEAVE("c__get_moves", "c");
}

static bool c__cast_spell(const long monptr, bool *took_turn) {
  /*{ Creatures can cast spells too.  (Dragon Breath)       -RAK-   }*/
  /*{ cast_spell := true if creature changes position       }*/
  /*{ took_turn  := true if creature casts a spell          }*/

  unsigned long i1;
  long y;
  long x;
  bool return_value;

  ENTER(("c__cast_spell", "c"));
  /* with m_list[monptr] do; */
  /* with monster_templates[m_list[monptr].mptr] do; */
  const long chance =
      monster_templates[m_list[monptr].mptr].spells & 0x0000000F;
  const long chance2 =
      monster_templates[m_list[monptr].mptr].spells & 0x80000000;

  /*{ 1 in x chance of casting spell                }*/
  /*{ if chance2 is true then 1 in x of not casting }*/
  if ((chance2 == 0 && randint(chance) != 1) ||
      (chance2 != 0 && randint(chance) == 1)) {

    return_value = false;
    *took_turn = false;

    /*{ Must be within certain range                  }*/
  } else if (m_list[monptr].cdis > MAX_SPELL_DIS) {

    return_value = false;
    *took_turn = false;

    /*{ Must have unobstructed Line-Of-Sight          }*/
  } else if (!los(char_row, char_col, m_list[monptr].fy, m_list[monptr].fx)) {

    return_value = false;
    *took_turn = false;

  } else {
    char ddesc[82];
    char cdesc[82];
    long spell_choice[32];
    /*{ Creature is going to cast a spell     }*/

    *took_turn = true;
    return_value = false;

    /*{ Describe the attack                           }*/
    find_monster_name(cdesc, monptr, true);
    strcat(cdesc, " ");
    /*{ For "DIED_FROM" string  }*/
    if ((0x80000000 & monster_templates[m_list[monptr].mptr].cmove) != 0) {
      sprintf(ddesc, "The %s", monster_templates[m_list[monptr].mptr].name);
    } else {
      sprintf(ddesc, "& %s", monster_templates[m_list[monptr].mptr].name);
    }
    strcpy(inven_temp.data.name, ddesc);
    inven_temp.data.number = 1;
    item_name(ddesc, &inven_temp);
    /*{ End DIED_FROM                 }*/

    /*{ Extract all possible spells into spell_choice }*/
    i1 = monster_templates[m_list[monptr].mptr].spells & 0x0FFFFFF0;
    long i3 = 0;
    while (i1 != 0) {
      const long i2 = bit_pos(&i1) + 1;
      i3++;
      spell_choice[i3] = i2;
    }

    /*{ Choose a spell to cast                        }*/
    const long thrown_spell = spell_choice[randint(i3)];
    bool stop_player = false;

    /*{ Cast the spell...                             }*/
    switch (thrown_spell) {
    case 5: /*{Teleport Short}*/
      teleport_away(monptr, 5);
      return_value = true;
      break;

    case 6: /*{Teleport Long }*/
      teleport_away(monptr, MAX_SIGHT);
      return_value = true;
      break;

    case 7: /*{Teleport To   }*/
      stop_player = true;
      strcat(cdesc, "casts a spell.");
      msg_print(cdesc);
      msg_print(" ");
      teleport_to(m_list[monptr].fy, m_list[monptr].fx);
      break;

    case 8: /*{Light Wound   }*/
      stop_player = true;
      if (strstr(cdesc, "Bard") != NULL || strstr(cdesc, "Ranger") != NULL ||
          strstr(cdesc, "Master Bard") != NULL) {
        strcat(cdesc, "shoots an arrow.");
      } else {
        strcat(cdesc, "casts a spell.");
      }
      msg_print(cdesc);
      if (player_spell_saves()) {
        msg_print("You resist the effects of the spell.");
      } else {
        take_hit(damroll("3d8"), ddesc);
      }
      break;

    case 9: /*{Serious Wound }*/
      stop_player = true;
      strcat(cdesc, "casts a spell.");
      msg_print(cdesc);
      if (player_spell_saves()) {
        msg_print("You resist the affects of the spell.");
      } else {
        take_hit(damroll("8d8"), ddesc);
      }
      break;

    case 10: /*{Hold Person   }*/
      stop_player = true;
      strcat(cdesc, "casts a spell.");
      msg_print(cdesc);
      if (player_flags.free_act || player_flags.free_time > 0) {
        msg_print("You are unaffected...");
      } else if (player_spell_saves()) {
        msg_print("You resist the affects of the spell.");
      } else {
        msg_print("You can't move!");
        if (player_flags.paralysis > 0) {
          player_flags.paralysis += 2;
        } else {
          player_flags.paralysis = randint(5) + 4;
        }
      }
      break;

    case 11: /*{Cause Blindnes}*/
      stop_player = true;
      strcat(cdesc, "casts a spell.");
      msg_print(cdesc);
      if (player_spell_saves()) {
        msg_print("You resist the affects of the spell.");
      } else if (player_flags.blind > 0) {
        player_flags.blind += 6;
      } else {
        player_flags.blind = 12 + randint(3);
        msg_print(" ");
      }
      break;

    case 12: /*{Cause Confuse }*/
      stop_player = true;
      strcat(cdesc, "casts a spell.");
      msg_print(cdesc);
      if (player_spell_saves()) {
        msg_print("You resist the affects of the spell.");
      } else if (player_flags.confused > 0) {
        player_flags.confused += 2;
      } else {
        player_flags.confused = randint(5) + 3;
      }
      break;

    case 13: /*{Cause Fear    }*/
      stop_player = true;
      strcat(cdesc, "casts a spell.");
      msg_print(cdesc);
      if (player_spell_saves()) {
        msg_print("You resist the affects of the spell.");
      } else if (player_flags.afraid > 0) {
        player_flags.afraid += 2;
      } else {
        player_flags.afraid = randint(5) + 3;
      }
      break;

    case 14: /*{Summon Monster}*/
      stop_player = true;
      strcat(cdesc, "magically summons a monster!");
      msg_print(cdesc);
      y = char_row;
      x = char_col;
      if (is_in(cave[y][x].fval, water_set)) {
        summon_water_monster(&y, &x, false);
      } else {
        summon_land_monster(&y, &x, false);
      }
      check_mon_lite(y, x);
      break;

    case 15: /*{Summon Undead}*/
      stop_player = true;
      strcat(cdesc, "magically summons an undead!");
      msg_print(cdesc);
      y = char_row;
      x = char_col;
      summon_undead(&y, &x);
      check_mon_lite(y, x);
      break;

    case 16: /*{Slow Person  }*/
      /* with player_flags do; */
      stop_player = true;
      strcat(cdesc, "casts a spell.");
      msg_print(cdesc);
      if (player_flags.free_act || player_flags.free_time > 0) {
        msg_print("You are unaffected...");
      } else if (player_spell_saves()) {
        msg_print("You resist the affects of the spell.");
      } else if (player_flags.slow > 0) {
        player_flags.slow += 2;
      } else {
        player_flags.slow = randint(5) + 3;
      }
      break;

    case 17:
      if (trunc(player_cmana) > 0) {
        char outval[120];
        /*{Drain Mana   }*/
        stop_player = true;
        sprintf(outval, "%sdraws psychic energy from you!", cdesc);
        msg_print(outval);
        sprintf(outval, "%sappears healthier...", cdesc);
        msg_print(outval);
        float r1 =
            randint(monster_templates[m_list[monptr].mptr].level) / 2 + 1;
        if (r1 > player_cmana) {
          r1 = player_cmana;
        }
        player_cmana -= r1;
        m_list[monptr].hp += 6 * trunc(r1);
      }
      break;

    case 18: /*{Breath Evil  }*/
      stop_player = true;
      if (strstr(cdesc, "High Priest") != NULL) {
        strcat(cdesc, "throws a cloud of black vapors at you!");
      } else {
        strcat(cdesc, "breathes black vapors around you!");
      }
      msg_print(cdesc);
      i1 = player_exp / 100 * MON_DRAIN_LIFE;
      breath(SE_EVIL, char_row, char_col, 1, ddesc);

      break;

    case 19: /*{Breath Petrify }*/
      stop_player = true;
      strcat(cdesc, "breathes petrifying gas at you!");
      msg_print(cdesc);
      breath(SE_PETRIFY, char_row, char_col, 1, ddesc);
      break;

    case 20: /*{Breath Light }*/
      stop_player = true;
      if (strstr(cdesc, "Druid") != NULL || strstr(cdesc, "Titan") != NULL) {
        strcat(cdesc, "casts a spell.");
      } else {
        strcat(cdesc, "breathes lightning.");
      }
      msg_print(cdesc);
      if (strstr(cdesc, "Druid") != NULL || strstr(cdesc, "Titan") != NULL) {
        breath(SE_LIGHTNING, char_row, char_col, 32, ddesc);
      } else {
        breath(SE_LIGHTNING, char_row, char_col,
               (long)(m_list[monptr].hp / 4.0), ddesc);
      }
      break;

    case 21: /*{Breath Gas   }*/
      stop_player = true;
      strcat(cdesc, "breathes gas.");
      msg_print(cdesc);
      breath(SE_GAS, char_row, char_col, (long)(m_list[monptr].hp / 3.0),
             ddesc);
      break;

    case 22: /*{Breath Acid  }*/
      stop_player = true;
      strcat(cdesc, "breathes acid.");
      msg_print(cdesc);
      breath(SE_ACID, char_row, char_col, (long)(m_list[monptr].hp / 3.0),
             ddesc);
      break;

    case 23: /*{Breath Frost }*/
      stop_player = true;
      strcat(cdesc, "breathes frost.");
      msg_print(cdesc);
      breath(SE_COLD, char_row, char_col, (long)(m_list[monptr].hp / 3.0),
             ddesc);
      break;

    case 24: /*{Breath Fire  }*/
      stop_player = true;
      if (strstr(cdesc, "Heirophant Druid") != NULL) {
        strcat(cdesc, "casts a spell.");
      } else {
        strcat(cdesc, "breathes fire.");
      }
      msg_print(cdesc);
      if (strstr(cdesc, "Heirophant Druid") != NULL) {
        breath(SE_FIRE, char_row, char_col, 48, ddesc);
      } else {
        breath(SE_FIRE, char_row, char_col, (long)(m_list[monptr].hp / 3.0),
               ddesc);
      }
      break;

    case 25: /*{Cast Illusion }*/
      stop_player = true;
      strcat(cdesc, "casts a spell.");
      msg_print(cdesc);
      if (player_spell_saves()) {
        msg_print("You resist the affects of the spell.");
      } else if (player_flags.image > 0) {
        player_flags.image += 2;
      } else {
        player_flags.image = randint(20) + 10;
      }
      break;

    case 26: /*{Summon Demon}*/
      stop_player = true;
      strcat(cdesc, "magically summons a demon!");
      msg_print(cdesc);
      y = char_row;
      x = char_col;
      summon_demon(&y, &x);
      check_mon_lite(y, x);
      break;

    case 27: /*{Summon Breed }*/
      stop_player = true;
      strcat(cdesc, "magically summons a monster!");
      msg_print(cdesc);
      y = char_row;
      x = char_col;
      summon_breed(&y, &x);
      check_mon_lite(y, x);
      break;

    case 28: /*{Stoning Gaze}*/
      stop_player = true;
      strcat(cdesc, "gazes at you!");
      msg_print(cdesc);
      if (player_spell_saves()) {
        msg_print("You resist the affects!");
      } else {
        /* XXXX hit points of monster sets petrify?? */
        petrify(m_list[monptr].hp);
      }
      break;

    default:
      stop_player = true;
      msg_print("Lucky you, creature casts unknown spell.");
      cdesc[0] = 0;

      MSG(("ERROR: cast bad spell: i3 = %ld "
           "spell_choice[i3] = %ld\n       "
           "monster = >%s<\n",
           i3, spell_choice[i3], monster_templates[m_list[monptr].mptr].name));
      break;
    }

    /*{ End of spells                                 }*/

    /*{ Stop player if in find mode   -DCJ/KRC-       }*/
    if (stop_player) {
      if (find_flag) {
        find_flag = false;
        player_action_move(5);
      }
      if (player_flags.rest > 0) {
        rest_off();
      }
    }
  }
  RETURN("c__cast_spell", "c", 'b', "moved", &return_value);
  return return_value;
}

static bool mon_move(const long monptr, long *hear_count) {
  /*{ Move the critters about the dungeon                   -RAK-   }*/

  mm_type mm;
  bool move_test;
  bool return_value = false;

  ENTER(("mon_move", "c"));
  /* with monster_templates[m_list[monptr].mptr] do; */

  /*{ Does the creature regenerate?                         }*/
  if ((monster_templates[ML(monptr).mptr].cdefense & 0x8000) != 0) {
    m_list[monptr].hp += randint(4);
  }

  if (m_list[monptr].hp > max_hp(monster_templates[ML(monptr).mptr].hit_die)) {
    m_list[monptr].hp = max_hp(monster_templates[ML(monptr).mptr].hit_die);
  }

  /*{ Does the critter multiply?                            }*/
  if (monster_templates[ML(monptr).mptr].attributes.multiplies) {
    if (MAX_MON_MULT >= mon_tot_mult) {
      if (player_flags.rest % mon_mult_adj == 0) {
        /* with m_list[monptr] do; */
        long i3 = 0;

        for (long i1 = MY(monptr) - 1; i1 <= MY(monptr) + 1; i1++) {
          for (long i2 = MX(monptr) - 1; i2 <= MX(monptr) + 1; i2++) {
            if (in_bounds(i1, i2)) {
              if (cave[i1][i2].cptr > 1) {
                i3++;
              }
            }
          }
        }

        if (i3 < 4) {
          if (randint(i3 * mon_mult_adj) == 1) {
            multiply_monster(MY(monptr), MX(monptr), ML(monptr).mptr, false);
          }
        }
      }
    }
  }

  /*{ Creature is confused?  Chance it becomes un-confused  }*/
  move_test = false;

  if (m_list[monptr].confused) {
    return_value = c__move_confused(monptr, mm, hear_count);
    m_list[monptr].confused = randint(8) != 1;
    move_test = true;
  } else if (monster_templates[ML(monptr).mptr].spells > 0) {
    /*{ Creature may cast a spell                             }*/
    return_value = c__cast_spell(monptr, &move_test);
  }

  if (!move_test) {

    /*{ 75% random movement                                   }*/
    if (randint(100) <= 75 &&
        (monster_templates[ML(monptr).mptr].cmove & 0x00000008) != 0) {
      return_value = c__move_confused(monptr, mm, hear_count);

      /*{ 40% random movement }*/
    } else if (randint(100) <= 40 &&
               (monster_templates[ML(monptr).mptr].cmove & 0x00000004) != 0) {
      return_value = c__move_confused(monptr, mm, hear_count);

      /*{ 20% random movement }*/
    } else if (randint(100) <= 20 &&
               (monster_templates[ML(monptr).mptr].cmove & 0x00000002) != 0) {
      return_value = c__move_confused(monptr, mm, hear_count);

      /*{ Normal movement }*/
    } else if ((monster_templates[ML(monptr).mptr].cmove & 0x00000001) == 0) {
      if (randint(200) == 1) {
        return_value = c__move_confused(monptr, mm, hear_count);
      } else {
        c__get_moves(monptr, &mm);
        return_value = c__make_move(monptr, mm, hear_count);
      }

      /*{ Attack, but don't move }*/
    } else {
      if (m_list[monptr].cdis < 2) {
        c__get_moves(monptr, &mm);
        return_value = c__make_move(monptr, mm, hear_count);
      }
    }
  }

  RETURN("mon_move", "c", 'b', "moved", &return_value);
  return return_value;
}

static void c__splash(const long m_list_i) {

  ENTER(("c__splash", "c"));
  const long mon_swimming =
      (monster_templates[m_list[m_list_i].mptr].cmove & 0x00000700) / 256;
  long drown_dam = randint(OUT_OF_ENV_DAM);

  /*{ here will also be modifiers due to waterspeed,depth }*/
  /*{ divide damage by 2 for each mon_swimming level, random
    rounding procedure }*/
  for (long i1 = 1; i1 <= mon_swimming; i1++) {
    drown_dam = (drown_dam + (randint(3) - 1)) / 3; /* kinda large for a 2 */
  }

  m_list[m_list_i].hp -= drown_dam;
  m_list[m_list_i].csleep = 0;

  if (m_list[m_list_i].hp < 0) {
    monster_death(m_list[m_list_i].fy, m_list[m_list_i].fx,
                  monster_templates[m_list[m_list_i].mptr].cmove);
    delete_monster(cave[m_list[m_list_i].fy][m_list[m_list_i].fx].cptr);
  }

  LEAVE("c__splash", "c");
}

static void c__maybe_splash(const long m_list_i) {
  if (is_in(cave[m_list[m_list_i].fy][m_list[m_list_i].fx].fval, floor_set)) {
    if (is_in(cave[m_list[m_list_i].fy][m_list[m_list_i].fx].fval,
              water_set) != ((monster_templates[m_list[m_list_i].mptr].cmove &
                              0x00000010) != 0) &&
        (monster_templates[m_list[m_list_i].mptr].cmove & 0x00000040) != 0) {
      c__splash(m_list_i);
    }
  }
}

/*{ Main procedure for creatures                              -RAK- }*/
void creatures(const bool attack) {
  ENTER(("creatures", "c"));

  get_player_move_rate();
  if (muptr <= 0) {
    LEAVE("creatures", "c");
    return;
  }

  /*{ Process the monsters  }*/
  long hear_count = 0;
  long monster_i = muptr;
  for (monster_i = muptr; monster_i != 0 && !moria_flag; monster_i = m_list[monster_i].nptr) {
    m_list[monster_i].cdis = distance(char_row, char_col, m_list[monster_i].fy,
                                      m_list[monster_i].fx);
    if (!attack) {
      c__update_mon(monster_i, &hear_count);
      c__maybe_splash(monster_i);
      continue;
    }

    /*{ Attack is argument passed to CREATURE}*/
    const long num_moves = movement_rate(m_list[monster_i].cspeed, monster_i);
    if (num_moves <= 0) {
      c__update_mon(monster_i, &hear_count);
      c__maybe_splash(monster_i);
      continue;
    }

    for (long _ = 1; _ <= num_moves; _++) {
      const bool is_monster_close_enough_to_act = m_list[monster_i].cdis <= monster_templates[m_list[monster_i].mptr].area_effect_radius;
      if (is_monster_close_enough_to_act || m_list[monster_i].ml) {
        if (m_list[monster_i].csleep > 0) {
          if (player_flags.aggravate) {
            m_list[monster_i].csleep = 0;
          } else if (player_flags.rest < 1) {
            if (randint(10) > player_stl) {
              m_list[monster_i].csleep -= (long)(75.0 / m_list[monster_i].cdis);
            }
          }
        }
        if (m_list[monster_i].stunned > 0) {
          m_list[monster_i].stunned--;
        }
        if (m_list[monster_i].csleep <= 0 && m_list[monster_i].stunned <= 0) {
          const long old_monster_y = m_list[monster_i].fy;
          const long old_monster_x = m_list[monster_i].fx;
          if (mon_move(monster_i, &hear_count)) {
            if (m_list[monster_i].ml) {
              m_list[monster_i].ml = false;
              if (test_light(old_monster_y, old_monster_x)) {
                lite_spot(old_monster_y, old_monster_x);
              } else {
                unlite_spot(old_monster_y, old_monster_x);
              }
            }
          }
        }
      }
      c__update_mon(monster_i, &hear_count);
    }
    c__maybe_splash(monster_i);
  }

  if (want_warn) {
    if (hear_count == 1) {
      msg_print("You hear a noise in the water.");
    } else if (hear_count > 1) {
      msg_print("You hear some noises in the water.");
    }
  }
  LEAVE("creatures", "c");
}

long find_mon(const char *virtual_name) {

  long count;
  bool maybe = false;

  for (count = 1; count < monster_template_size && !maybe;) {
    if (!strcmp(virtual_name, monster_templates[count].name)) {
      maybe = true;
    } else {
      count++;
    }
  }
  if (!maybe) {
    count = 0;
  }
  return count;
}
