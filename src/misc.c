#include <curses.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "treasures.h"
#include "configure.h"
#include "constants.h"
#include "debug.h"
#include "magic.h"
#include "pascal.h"
#include "player.h"
#include "routines.h"
#include "save.h"
#include "term.h"
#include "types.h"
#include "variables.h"

void C_print_new_spell_line(uint8_t i, long slot, long failchance);

static long tcptr; /* { Cur treasure heap ptr} */
static long mfptr; /* { Cur free monster ptr	} */
static treasure_type up_stair = {"an up staircase",
                                 up_staircase,
                                 0x00000000,
                                 0x00000000,
                                 0,
                                 0,
                                 1,
                                 0,
                                 0,
                                 0,
                                 0,
                                 0,
                                 0,
                                 "1d1",
                                 0,
                                 0};
static treasure_type down_stair = {"a down staircase",
                                   down_staircase,
                                   0x00000000,
                                   0x00000000,
                                   0,
                                   0,
                                   1,
                                   0,
                                   0,
                                   0,
                                   0,
                                   0,
                                   0,
                                   "1d1",
                                   0,
                                   0};
static treasure_type up_steep = {"a steep staircase",
                                 up_steep_staircase,
                                 0x00000000,
                                 0x00000000,
                                 0,
                                 0,
                                 1,
                                 0,
                                 0,
                                 0,
                                 0,
                                 0,
                                 0,
                                 "1d1",
                                 0,
                                 0};
static treasure_type down_steep = {"a steep staircase",
                                   down_steep_staircase,
                                   0x00000000,
                                   0x00000000,
                                   0,
                                   0,
                                   1,
                                   0,
                                   0,
                                   0,
                                   0,
                                   0,
                                   0,
                                   "1d1",
                                   0,
                                   0};
static monster_type blank_monster = /* { Blank monster values	} */
    {0, 0, 0, 0, 0, 0, 0, 0, 0, false, false, false};

static const long obj_great = 30; //  1/n Chance of item being a Great Item

/* Print list of spells     -RAK- */
static void print_new_spells(spl_type spell, long num, boolean *redraw) {

  uint8_t i;

  ENTER(("print_new_spells", "%ld, %d", num, *redraw));

  *redraw = true;
  clear_from(1);
  if (num >= 23)
    num = 22;

  prt("   Name                          Level  Mana  %Failure", 2, 1);
  for (i = 0; i < num; i++) {
    if (spell[i].splnum != -1)
      spell_chance(&spell[i]);
    C_print_new_spell_line(i, spell[i].splnum, spell[i].splchn);
  }

  LEAVE("print_new_spells", "");
}

/* Learn some magic spells (Mage)			-RAK-	*/
static boolean learn_spell(boolean *redraw) {
  ENTER(("learn_spell", ""));

  unsigned long spell_flag = 0;
  unsigned long spell_flag2 = 0;
  spl_type spells_to_choose_from;
  long new_spells = num_new_spells(C_player_mod_from_stat(INT));
  boolean return_value = false;
  treas_rec *ptr;

  for (ptr = inventory_list; ptr != NULL; ptr = ptr->next) {
    if (ptr->data.tval == magic_book) {
      spell_flag |= ptr->data.flags;
      spell_flag2 |= ptr->data.flags2;
    }
  }

  while (new_spells > 0 && (spell_flag > 0 || spell_flag2 > 0)) {
    unsigned long spell_counter = 0;
    unsigned long flag1 = spell_flag;
    unsigned long flag2 = spell_flag2;
    long selected;
    long trash;

    do {
      unsigned long spell_index = bit_pos64(&flag2, &flag1);
      if (spell_index > 31)
        spell_index--;

      if (C_magic_spell_level(spell_index) > player_lev)
        continue;

      if (C_player_knows_spell(spell_index))
        continue;

      spells_to_choose_from[spell_counter++].splnum = spell_index;

    } while (flag1 != 0 || flag2 != 0);

    if (spell_counter == 0)
      break; /* No spells to learn */

    print_new_spells(spells_to_choose_from, spell_counter, redraw);
    if (get_spell(spells_to_choose_from, spell_counter, &selected, &trash,
                  "Learn which spell?", redraw)) {
      C_player_set_knows_spell(selected, true);
      return_value = true;
      if (player_mana == 0) {
        player_mana = 1;
        player_cmana = 1;
      }
    } else {
      new_spells = 0;
    }

    new_spells--;
  } /* end while new_spells > 0 */

  LEAVE("learn_spell", "");
  return return_value;
}

/*{ Learn some prayers (Priest)				-RAK-	}*/
static boolean learn_prayer() {

  unsigned long new_spells_to_learn =
      num_new_spells(C_player_mod_from_stat(WIS));
  boolean return_value = false;

  ENTER(("learn_prayer", ""));
  MSG(("new spells: %d", new_spells_to_learn));

  if (new_spells_to_learn > 0) {
    unsigned spells_learned = 0;
    unsigned long i;

    for (i = 0; i < MAX_SPELLS; ++i) {
      if (C_magic_spell_level(i) > player_lev)
        continue;
      if (C_player_knows_spell(i))
        continue;

      C_player_set_knows_spell(i, true);
      spells_learned++;
      return_value = true;

      if (--new_spells_to_learn == 0)
        break;
    }

    if (player_mana == 0) {
      player_mana = 1;
      player_cmana = 1;
    }
    if (spells_learned > 0) {
      msg_print(spells_learned > 1 ? "You learned new prayers!"
                                   : "You learned a new prayer!");
    }
  }

  LEAVE("learn_prayer", "");
  return return_value;
}

/* Learn some disciplines (Monk)				-RAK-*/
static boolean learn_discipline() {
  long i1, i2, i3, i4, new_spell;
  long test_array[33]; /*	: array [1..32] of long;*/
  unsigned long spell_flag, spell_flag2;
  boolean return_value = false;

  /*  printf("\n\n  ^^^ENTER learn_discip^^^\n\n");fflush(stdout); */
  ENTER(("learn_discipline", ""));

  i1 = 0; /* btw, we only use test_array[1..32] */
  spell_flag = 0x00003FFF;
  spell_flag2 = 0x00000000;

  while ((spell_flag > 0) || (spell_flag2 > 0)) {
    i2 = bit_pos64(&spell_flag2, &spell_flag);
    if (i2 > 31) {
      i2--;
    }
    if (C_magic_spell_level(i2) <= player_lev) {
      if (!C_player_knows_spell(i2)) {
        i1++;
        test_array[i1] = i2;
      }
    }
  }

  i2 = num_new_spells(C_player_mod_from_stat(WIS));
  new_spell = 0;

  while ((i1 > 0) && (i2 > 0)) {
    i3 = randint(i1);
    C_player_set_knows_spell(test_array[i3], true);
    new_spell++;

    for (i4 = i3; i4 < i1; i4++) {
      test_array[i4] = test_array[i4 + 1];
    }

    i1--; /*{ One less spell to learn	}*/
    i2--; /*{ Learned one			}*/
  }
  if (new_spell > 0) {
    if (new_spell > 1) {
      msg_print("You learned new disciplines!");
    } else {
      msg_print("You learned a new discipline!");
    }
    if (player_exp == 0) {
      msg_print(" ");
    }
    if (player_mana == 0) {
      player_mana = 1;
      player_cmana = 1;
    }
    return_value = true;

  } else {
    return_value = false;
  }

  LEAVE("learn_discipline", "");

  return return_value;
}
/*{ Learn some magic songs (Bard)			-Cap'n-	}*/
static boolean learn_song(boolean *redraw) {
  unsigned long i2;
  unsigned long i4;
  long i1;
  long i3;
  long sn;
  long sc;
  long new_spells;
  unsigned long spell_flag = 0;
  unsigned long spell_flag2 = 0;
  spl_type spell;
  treas_rec *curse;

  boolean return_value = false;

  ENTER(("learn_song", ""));

  curse = inventory_list;
  new_spells = num_new_spells(C_player_mod_from_stat(CHR));

  while (curse != NULL) {
    if (curse->data.tval == song_book) {
      spell_flag |= curse->data.flags;
      spell_flag2 |= curse->data.flags2;
    }
    curse = curse->next;
  }

  while ((new_spells > 0) && ((spell_flag > 0) || (spell_flag2 > 0))) {
    i1 = 0;
    i2 = spell_flag;
    i4 = spell_flag2;

    do {
      i3 = bit_pos64(&i4, &i2);
      if (i3 > 31) {
        i3--;
      }
      if (C_magic_spell_level(i3) <= player_lev) {
        if (!(C_player_knows_spell(i3))) {
          spell[i1++].splnum = i3;
        }
      }
    } while ((i2 != 0) || (i4 != 0));

    if (i1 > 0) {
      print_new_spells(spell, i1, redraw);
      if (get_spell(spell, i1, &sn, &sc, "Learn which spell?", redraw)) {
        C_player_set_knows_spell(sn, true);
        return_value = true;
        if (player_mana == 0) {
          player_mana = 1;
          player_cmana = 1;
        }
      } else {
        new_spells = 0;
      }
    } else {
      new_spells = 0;
    }
    new_spells--;
  } /* end while new_spells > 0 */

  LEAVE("learn_song", "");
  return return_value;
}

/*{ Learn some druid spells (Druid)			-Cap'n-	}*/
static boolean learn_druid(boolean *redraw) {

  int i;
  long num_spells_to_learn = num_new_spells(C_player_mod_from_stat(WIS));
  spl_type spells_to_choose_from;
  int spell_counter = 0;
  boolean return_value = false;

  ENTER(("learn_druid", ""));

  for (i = 0; i < MAX_SPELLS; ++i) {
    if (C_magic_spell_level(i) > player_lev)
      continue;
    if (C_player_knows_spell(i))
      continue;
    spells_to_choose_from[spell_counter++].splnum = i;
  }

  for (i = 0; i < num_spells_to_learn; ++i) {
    long selected;
    long trash;
    print_new_spells(spells_to_choose_from, spell_counter, redraw);
    if (get_spell(spells_to_choose_from, spell_counter, &selected, &trash,
                  "Learn which spell?", redraw)) {
      int j;
      C_player_set_knows_spell(selected, true);
      return_value = true;
      if (player_mana == 0) {
        player_mana = 1;
        player_cmana = 1;
      }
      /* Remove selected value from list */
      for (j = 0; j < spell_counter; ++j) {
        if (spells_to_choose_from[j].splnum == selected) {
          break;
        }
      }
      for (; j < spell_counter; ++j) {
        spells_to_choose_from[j].splnum = spells_to_choose_from[j + 1].splnum;
      }
      spell_counter--;
    }
  }

  LEAVE("learn_druid", "");
  return return_value;
}

char *cost_str(long amt, char result[134]) {
  /*{ Return string describing how much the amount is worth	-DMF-
   * }*/
  long amtd9 = amt / 9;

  if (amtd9 >= MITHRIL_VALUE) {
    sprintf(result, "%ld mithril", ((amt + MITHRIL_VALUE - 1) / MITHRIL_VALUE));
  } else if (amtd9 >= PLATINUM_VALUE) {
    sprintf(result, "%ld platinum",
            ((amt + PLATINUM_VALUE - 1) / PLATINUM_VALUE));
  } else if (amtd9 >= GOLD_VALUE) {
    sprintf(result, "%ld gold", ((amt + GOLD_VALUE - 1) / GOLD_VALUE));
  } else if (amtd9 >= SILVER_VALUE) {
    sprintf(result, "%ld silver", ((amt + SILVER_VALUE - 1) / SILVER_VALUE));
  } else if (amtd9 >= COPPER_VALUE) {
    sprintf(result, "%ld copper", ((amt + COPPER_VALUE - 1) / COPPER_VALUE));
  } else {
    sprintf(result, "%ld iron", amt);
  }

  return result;
}

void adv_time(boolean flag) {
  /*{ Advance the game clock by one 'second'		-DMF-	}*/

  /* with player_cur_age do; */
  player_cur_age.secs++;

  if (player_cur_age.secs > 399) {
    player_cur_age.hour++;
    player_cur_age.secs = 0;
    if (player_cur_age.hour == 24) {
      player_cur_age.day++;
      player_cur_age.hour = 0;
      if (player_cur_age.day == 29) {
        player_cur_age.month++;
        player_cur_age.day = 1;
        if (player_cur_age.month == 14) {
          player_cur_age.month = 1;
          player_cur_age.year++;
        }
      }
    }
  }

  if (flag && (player_cur_age.secs % 100) == 0) {
    prt_stat_block();
  }
}

chtype get_loc_symbol(long y, long x) {
  /* check lights and stuff before calling loc_symbol */

  if (test_light(y, x))
    return loc_symbol(y, x);
  else if (cave[y][x].cptr == 1 && !find_flag)
    return '@';
  else if (cave[y][x].cptr > 1)
    return m_list[cave[y][x].cptr].ml ? loc_symbol(y, x) : ' ';
  else
    return ' ';
}

chtype loc_symbol(long y, long x) {
  unsigned char const cptr = cave[y][x].cptr;
  unsigned char const tptr = cave[y][x].tptr;
  unsigned char const fval = cave[y][x].fval;

  chtype sym;

  if (cptr == 1 && !find_flag) {
    sym = '@';
  } else if (player_flags.blind > 0) {
    sym = ' ';
  } else {

    if (cptr > 1) {

      /* with m_list[cptr] do; */
      unsigned short const mptr = m_list[cptr].mptr;
      if ((m_list[cptr].ml) &&
          (!is_in(fval, water_set) ||
           (is_in(fval, water_set) &&
            (((c_list[mptr].cmove & 0x00800000) != 0) ||
             (distance(char_row, char_col, y, x) <= 5)))) &&
          (((c_list[mptr].cmove & 0x00010000) == 0) ||
           (player_flags.see_inv))) {
        sym = c_list[mptr].cchar;
      } else if (tptr > 0) {
        sym = C_item_get_tchar(&t_list[tptr]);
      } else if (is_in(fval, earth_set)) { /* 0, 3, 8 and 9
                                              were here too */
        sym = '.';
      } else if (is_in(fval, pwall_set)) {
        sym = '#';
      } else if (is_in(fval, water_set)) {
        sym = '`' | COLOR_PAIR(COLOR_BLUE);
      } else {
        /* unknown terrain type */
        sym = '.' | A_DIM;
      }

    } else if (tptr > 0) {

      if (is_in(fval, water_set)) {
        if (is_in(t_list[tptr].tval, float_set) ||
            ((distance(char_row, char_col, y, x) <= 5) &&
             (los(char_row, char_col, y, x)))) {
          sym = C_item_get_tchar(&t_list[tptr]);
        } else {
          sym = '`' | COLOR_PAIR(COLOR_BLUE);
        }
      } else {
        sym = C_item_get_tchar(&t_list[tptr]);
      }

      /* 0, 3, 8 and 9 were here too */
    } else if (is_in(fval, earth_set)) {
      sym = '.';
    } else if (is_in(fval, pwall_set)) {
      sym = '#';
    } else if (is_in(fval, water_set)) {
      sym = '`' | COLOR_PAIR(COLOR_BLUE);
    } else {
      /* unknown terrain type */
      sym = '.' | A_DIM;
    }
  }

#if DO_DEBUG
  if ((sym & 0x7F) < 32 || (sym & 0x7F) > 126) {
    MSG(("ERROR: loc_sym: (%ld, %ld) = %ld   "
         "cptr=%ld tptr=%ld fval=%ld\n",
         x, y, (long)sym, (long)cptr, (long)tptr, (long)fval));
  }
#endif

  return sym;
}

/*{ Determine character's sex				-DCJ- }*/
unsigned char characters_sex() {
  return ((player_sex[0] == 'F') ? FEMALE : MALE);
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void add_days(game_time_type *ti, long d) {
  /* Add days to the current date -DMF-
   *  ti->day++;
   *  ti->month += (ti->day-1) / 28;
   *  ti->day    = ((ti->day-1) % 28) + 1;
   *  ti->year  += (ti->month-1) / 13;
   *  ti->month  = ((ti->month-1) % 13) + 1;
   */

  /* 10/26/00 -- JEB:
   * DMF's code works great (if a little strangely) if you only ever add
   * 1 day, which this function did. Notice that the above code ignores
   * the 'd' parameter, which in turn means that no matter how long of a
   * stay you buy in the inn, you really only get 1 day. I thought about
   * just putting a loop around the above code to iterate 'd' times, but
   * that's lame so here's some more robust code that simply calculates
   * the day, month, and year increments for any value of 'd'. Note that
   * the above code implies that the year is 364 days long (13 months of
   * 28 days each), which i've kept:
   */

  unsigned char yrs;
  unsigned char mos;

  yrs = (int)(d / 364); /* how many years you get from 'd' days */
  d -= 364 * yrs;       /* d = however many days are left over... */
  mos = (int)(d / 28);  /* how many months you get from the remaining days */
  d -= 28 * mos;        /* d = however many days are left over... */
  ti->day += d;         /* add the remaining days, months, and years */
  ti->month += mos;
  ti->year += yrs;
  if (ti->day > 28) {
    ti->month++;
    ti->day %= 28;
  } /* fix any overflows */
  if (ti->month > 13) {
    ti->year++;
    ti->month %= 13;
  }
}

void am__add_munny(long *amount, long *to_bank, long wl, long type_num) {
  long trans, w_max;
  long coin_num;

  coin_num = player_money[type_num];
  trans = *amount / coin_value[type_num];
  w_max = (wl * 100 - inven_weight) / COIN_WEIGHT;
  if (w_max < -coin_num) {
    w_max = -coin_num;
  }
  if (w_max < trans) {
    *to_bank += (trans - w_max) * coin_value[type_num];
    trans = w_max;
  }
  inven_weight += COIN_WEIGHT * trans;
  player_money[type_num] = coin_num + trans;
  *amount = *amount % coin_value[type_num];
}
/*//////////////////////////////////////////////////////////////////// */
void add_money(long amount) {
  /*	{ Add money in the lightest possible amounts.
   * -DMF-/DY}*/

  long to_bank;
  long weight_limit;
  long i1;
  char out_val[134];
  char out2[134];
  long type_num;

  ENTER(("add_money", ""));

  to_bank = 0;
  weight_limit = C_player_max_bulk();
  /* with player_do; */

  for (type_num = MITHRIL; type_num >= IRON; type_num--) {
    am__add_munny(&amount, &to_bank, weight_limit, type_num);
  }

  reset_total_cash();

  if (to_bank > 0) {
    sprintf(out_val, "You cannot carry %s of the money",
            cost_str(to_bank, out2));
    msg_print(out_val);
    if (get_yes_no("Do you wish to send a page to the bank with "
                   "the excess money?")) {
      i1 = (((95 * to_bank) / 100) / GOLD_VALUE);
      if (i1 < 5) {
        msg_print("The page cannot be moved by such "
                  "paltry sums of gold.");
      } else {
        if (randint(mugging_chance) == 1) {
          msg_print("The page is mugged!");
          sprintf(out_val, "The %s is lost!", cost_str(to_bank, out2));
          msg_print(out_val);
        } else {
          bank[GOLD] += i1;
          player_account += i1;
          bank[TOTAL_] = ((bank[MITHRIL] * coin_value[MITHRIL] +
                           bank[PLATINUM] * coin_value[PLATINUM]) /
                              GOLD_VALUE +
                          bank[GOLD]);
          sprintf(out_val,
                  "The page deposits "
                  "%ld gold at the bank "
                  "for you.",
                  i1);
          msg_print(out_val);
        }
      }
    } else {
      msg_print("You cannot carry the change, so it is lost.");
    }
  }

  LEAVE("add_money", "");
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
treas_rec *money_carry() {
  /*{ Pick up some money	-DMF-	}*/

  /* with player_do; */
  /* with inven_temp^.data do; */

  player_money[inven_temp.data.level] += inven_temp.data.number;
  reset_total_cash();
  inven_weight += inven_temp.data.number * inven_temp.data.weight;
  prt_stat_block();

  return &inven_temp;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
boolean sm__sub_munny(long *amt, long *wt, long type_num) {
  long trans, coin_num;
  boolean return_value;

  coin_num = player_money[type_num];
  trans = (*amt + coin_value[type_num] - 1) / coin_value[type_num];
  if (coin_num < trans) {
    trans = coin_num;
  }
  (*wt) += COIN_WEIGHT * trans;
  player_money[type_num] = coin_num - trans;
  (*amt) -= trans * coin_value[type_num];

  return_value = *amt > 0;
  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
void subtract_money(long amount, boolean make_change) {
  /*{ Give money to store, but can give back change	 -DMF-/DY}*/

  long amt, wt;
  long type_num;

  amt = amount;
  wt = 0;
  type_num = 1;

  for (; sm__sub_munny(&amt, &wt, type_num) && (type_num < MITHRIL);
       type_num++) {
  }

  inven_weight -= wt;
  reset_total_cash();

  if (make_change) {
    add_money(-amt);
  }
}

boolean get_spell(spl_type spell, long num, long *sn, long *sc, char prompt[82],
                  boolean *redraw) {
  /*{ Returns spell pointer					-RAK-
   * }*/

  boolean flag = true;
  char choice;
  char out_val1[82];

  *sn = -1;

  sprintf(out_val1, "(Spells a-%c, *,<space>=List, <ESCAPE>=exit) %s",
          (int)num + 96, prompt);

  while (((*sn < 0) || (*sn >= num)) && (flag)) {
    *sn = -1;

    prt(out_val1, 1, 1);
    choice = inkey();
    switch (choice) {
    case 0:
    case 3:
    case 25:
    case 26:
    case 27:
      flag = false;
      /*{ reset_flag := true;}*/
      break;

    case 42:
    case 32:
      print_new_spells(spell, num, redraw);
      break;

    default:
      *sn = choice - 97;
      if (spell[*sn].splnum < 0) {
        *sn = -1;
      }
      break;
    }
  } /* end while */

  msg_flag = false;
  if (flag) {
    spell_chance(&(spell[*sn]));
    *sc = spell[*sn].splchn;
    *sn = spell[*sn].splnum;
  }

  /*  printf("\n\n  ^^^EXIT get_spell %d^^^\n\n", *sn);fflush(stdout); */
  return flag;
}

/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
long num_new_spells(long smarts) {
  long return_value;

  ENTER(("num_new_spells", ""));

  switch (smarts) {
  case 1:
  case 2:
  case 3:
    return_value = 1;
    break;
  case 4:
  case 5:
    return_value = randint(2);
    break;
  case 6:
    return_value = randint(3);
    break;
  case 7:
    return_value = randint(2) + 1;
    break;
  default:
    return_value = 0;
    break;
  }

  LEAVE("num_new_spells", "");
  return return_value;
}

/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */

void spell_chance(spl_rec *spell) {
  /*	{ Returns spell chance of failure for spell		-RAK-
   * }*/

  /*	with magic_spell[player_pclass,spell.splnum] do*/
  /*	  with spell do                                 */

  spell->splchn = C_magic_spell_failchance(spell->splnum) -
                  3 * (player_lev - C_magic_spell_level(spell->splnum));

  if (C_player_uses_magic(M_ARCANE)) {
    spell->splchn -= 3 * (C_player_mod_from_stat(INT) - 1);
  } else if (C_player_uses_magic(M_SONG)) {
    spell->splchn -= 3 * (C_player_mod_from_stat(CHR) - 1);
  } else if (C_player_uses_magic(M_NATURE)) {
    spell->splchn -= 3 * (C_player_mod_from_stat(WIS) - 1);
  } else {
    spell->splchn -= 3 * (C_player_mod_from_stat(WIS) - 1);
  }

  if (C_magic_spell_mana(spell->splnum) > player_cmana) {
    spell->splchn +=
        5 * (int)(C_magic_spell_mana(spell->splnum) - player_cmana);
  }

  if (spell->splchn > 95) {
    spell->splchn = 95;
  } else if (spell->splchn < 5) {
    spell->splchn = 5;
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */

/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
long bit_pos(unsigned long *test) {
  /* Returns position of first set bit			-RAK-	*/
  /*     and clears that bit */

  const unsigned size = sizeof(*test) * 8;
  unsigned long mask = 0x1;
  unsigned i;

  for (i = 0; i < size; i++) {
    if (*test & mask) {
      *test &= ~mask;
      return (i);
    }
    mask <<= 1;
  }

  /* no one bits found */
  return (-1);
}

/*//////////////////////////////////////////////////////////////////// */

long bit_pos64(unsigned long *high, unsigned long *low) {
  /*!	This is the 64-bit version of bit_pos */

  long pos;

  pos = bit_pos(low);
  if (pos == -1) {
    pos = bit_pos(high);
    if (pos != -1) {
      pos += 32;
    }
  }

  return pos;
}

/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void insert_str(char *object_str, char const *mtc_str, char const *insert_str) {
  char *s1;
  char ending[80];

  s1 = strstr(object_str, mtc_str);
  if (s1 != NULL) {
    strcpy(ending, s1 + strlen(mtc_str));
    strcpy(s1, insert_str);
    strcat(s1, ending);
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void insert_num(char *object_str, char *mtc_str, long number,
                boolean show_sign) {
  char numstr[82];
  char const *sign = number > 0 && show_sign ? "+" : "";

  sprintf(numstr, "%s%ld", sign, number);
  /*  strcat(object_str, " "); */

  insert_str(object_str, mtc_str, numstr);
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
char *day_of_week_string(long day, unsigned wid, char result[134]) {
  /*{ Return first X characters of day of week		-DMF-	}*/
  switch (day % 7) {
  case 0:
    strcpy(result, "Saturday  ");
    break;
  case 1:
    strcpy(result, "Sunday    ");
    break;
  case 2:
    strcpy(result, "Monday    ");
    break;
  case 3:
    strcpy(result, "Tuesday   ");
    break;
  case 4:
    strcpy(result, "Wednesday ");
    break;
  case 5:
    strcpy(result, "Thursday  ");
    break;
  case 6:
    strcpy(result, "Friday    ");
    break;
  }
  if (strlen(result) > wid) {
    result[wid] = 0;
  }

  return result;
}

/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
char *month_string(long mon, char result[134]) {
  /*{ Return the name of a numbered month			-DMF-	}*/
  switch (mon) {
  case 1:
    strcpy(result, "January");
    break;
  case 2:
    strcpy(result, "February");
    break;
  case 3:
    strcpy(result, "March");
    break;
  case 4:
    strcpy(result, "April");
    break;
  case 5:
    strcpy(result, "May");
    break;
  case 6:
    strcpy(result, "June");
    break;
  case 7:
    strcpy(result, "July");
    break;
  case 8:
    strcpy(result, "August");
    break;
  case 9:
    strcpy(result, "September");
    break;
  case 10:
    strcpy(result, "October");
    break;
  case 11:
    strcpy(result, "November");
    break;
  case 12:
    strcpy(result, "December");
    break;
  default:
  case 13:
    strcpy(result, "Moria");
    break;
  }
  return result;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
char *time_string(long hour, long sec, char result[134]) {
  /*{ Return the time in the format HH:MM			-DMF-	}*/

  long min;

  min = (sec * 0.15);
  sprintf(result, "%02ld:%02ld", hour, min);
  /* insert_str(result," ","0"); */
  /* insert_str(result," ","0"); */
  return result;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
char *place_string(long num, char result[134]) {
  /*{ Return the ending to a number string (1st, 2nd, etc)	-DMF-
   * }*/
  switch (num) {
  case 1:
    sprintf(result, "%ldst", num);
    break;
  case 2:
    sprintf(result, "%ldnd", num);
    break;
  case 3:
    sprintf(result, "%ldrd", num);
    break;
  default:
    if (num < 20) {
      sprintf(result, "%ldth", num);
    } else {
      switch (num % 10) {
      case 1:
        sprintf(result, "%ldst", num);
        break;
      case 2:
        sprintf(result, "%ldnd", num);
        break;
      case 3:
        sprintf(result, "%ldrd", num);
        break;
      default:
        sprintf(result, "%ldth", num);
        break;
      }
    } /* end second switch */
    break;
  } /* end first switch */

  return result;
}

void gain_level() {
  /*{ Increases hit points and level			-RAK-	}*/

  ENTER(("gain_level", ""));
  long nhp = C_player_roll_hp_for_levelup();
  C_player_modify_max_hp(nhp);
  player_lev++;

  long need_exp = trunc(exp_per_level[player_lev] * player_expfact);
  if (player_exp > need_exp) {
    long dif_exp = player_exp - need_exp;
    player_exp = need_exp + (dif_exp / 2);
  }

  msg_print("Your skills have improved.");
  msg_print(" ");
  learn_magic(true);
  msg_flag = false;

  prt_stat_block();

  LEAVE("gain_level", "");
}

void C_gain_mana(void);
void learn_magic(boolean redraw_now) {

  redraw = false;
  if (C_player_uses_magic(M_ARCANE)) {
    learn_spell(&redraw);
  }
  if (C_player_uses_magic(M_NATURE)) {
    learn_druid(&redraw);
  }
  if (C_player_uses_magic(M_SONG)) {
    learn_song(&redraw);
  }
  if (C_player_uses_magic(M_DIVINE)) {
    learn_prayer();
  }
  if (C_player_uses_magic(M_CHAKRA)) {
    learn_discipline();
  }

  C_gain_mana();

  if (redraw && redraw_now) {
    draw_cave();
  }
}

/**
 * in_bounds() - Checks if a coordinate is in the dungeon bounds -RAK-
 * @y: Y-coordiate
 * @x: X-coordinate
 */
boolean in_bounds(long y, long x) {
  boolean return_value;

  if ((y > 1) && (y < cur_height) && (x > 1) && (x < cur_width)) {
    return_value = true;
  } else {
    return_value = false;
  }

  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
boolean move_dir(long dir, long *y, long *x) /* was move */
{
  /*{ Given direction 'dir', returns new row, column location -RAK- }*/

  long new_row, new_col;
  boolean return_value = false;

  new_row = *y + dy_of[dir];
  new_col = *x + dx_of[dir];
  if ((new_row >= 1) && (new_row <= cur_height)) {
    if ((new_col >= 1) && (new_col <= cur_width)) {
      *y = new_row;
      *x = new_col;
      return_value = true;
    }
  }
  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void popm(long *x) {
  /*{ Returns a pointer to next free space			-RAK-
   * }*/

  if (mfptr < 1) {
    compact_monsters();
    validate_monsters();
  }

  *x = mfptr;
  mfptr = m_list[*x].nptr;
  m_list[*x].nptr = 0;
}

void pushm(long x) {
  /*{ Pushs a record back onto free space list		-RAK-	}*/

  m_list[x] = blank_monster;
  m_list[x].nptr = mfptr;
  mfptr = x;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void report_mlist_error(const char *err_msg, int error_node, int prev_node) {
  (void)error_node;
  (void)prev_node;

  msg_print((char *)err_msg);
}

void validate_monsters() {
  boolean used_list[MAX_MALLOC + 1] = {false};
  boolean free_list[MAX_MALLOC + 1] = {false};

  boolean busted = false;
  int i1;
  int i2;
  for (i2 = 0, i1 = muptr; i1; i2 = i1, i1 = m_list[i1].nptr) {
    if (used_list[i1]) {
      /* there is a loop in the monster list! */
      report_mlist_error("Internal Error: m_list has a loop!", i1, i2);
      busted = true;
    } else {
      used_list[i1] = true;
    }
  }

  if (busted) {
    if (i2) {
      m_list[i2].nptr = 0;
    } else {
      muptr = 0;
    }
  }

  busted = false;
  for (i2 = 0, i1 = mfptr; i1; i2 = i1, i1 = m_list[i1].nptr) {
    if (free_list[i1]) {
      /*
       * there is a loop in the free list
       */
      report_mlist_error("Internal Error: m_list has a free list loop!", i1,
                         i2);
      busted = true;
      break;
    } else {
      free_list[i1] = true;
    }

    if (used_list[i1]) {
      /*
       * the monster list and free list overlap!
       */
      report_mlist_error("Internal Error: m_list lists overlap!", i1, i2);
      busted = true;
      break;
    }
  }

  if (busted) {
    if (i2) {
      m_list[i2].nptr = 0;
    } else {
      mfptr = 0;
    }
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void compact_monsters() {
  int monsters_deleted = 0;
  long delete_distance = 66;

  validate_monsters();

  while (monsters_deleted == 0) {
    long i = muptr;
    long prev_i = 0;
    do {
      boolean deleted_this = false;
      long next_i = m_list[i].nptr;
      if (delete_distance < m_list[i].cdis) {
        if (randint(3) == 1) {
          if (prev_i == 0) {
            muptr = next_i;
          } else {
            m_list[prev_i].nptr = next_i;
          }
          cave[m_list[i].fy][m_list[i].fx].cptr = 0;
          m_list[i] = blank_monster;
          m_list[i].nptr = mfptr;
          mfptr = i;
          deleted_this = true;
          monsters_deleted++;
        }
      }
      if (!deleted_this) {
        prev_i = i;
      }
      i = next_i;
    } while (i != 0);

    if (monsters_deleted == 0) {
      // narrow the search closed to the player
      delete_distance -= 6;
    }
  }

  if (delete_distance < 66) {
    prt_map();
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void popt(long *x) {
  /*{ Gives pointer to next free space			-RAK-	}*/

  if (tcptr < 1) {
    compact_objects();
  }

  *x = tcptr;
  tcptr = t_list[*x].p1;
}

void pusht(long x) {
  /*{ Pushs a record back onto free space list		-RAK-	}*/

  t_list[x] = blank_treasure;
  t_list[x].p1 = tcptr;
  tcptr = x;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void compact_objects() {
  /*{ If too many objects on floor level, delete some of them-RAK-
   * }*/

  long i1, i2, ctr, cur_dis;
  boolean flag;
  obj_set fragile_stuff = {1, 6, 9, 0}; /* open pit, loose rock, loose rock */

  ctr = 0;
  cur_dis = 66;

  do {
    for (i1 = 1; i1 <= cur_height; i1++) {
      for (i2 = 1; i2 <= cur_width; i2++) {
        /* with cave[i1][i2]. do; */
        if (cave[i1][i2].tptr > 0) {
          if (distance(i1, i2, char_row, char_col) > cur_dis) {
            flag = false;
            /* with */
            /* t_list[cave[i1][i2].tptr] do;
             */
            switch (t_list[cave[i1][i2].tptr].tval) {
            case seen_trap:
              if (is_in(t_list[cave[i1][i2].tptr].subval, fragile_stuff)) {
                flag = true;
              } else if (randint(4) == 1) {
                flag = true;
              }
              break;

            case rubble:
              flag = true;
              break;

            case open_door:
            case closed_door:
              if (randint(8) == 1) {
                flag = true;
              }
              break;

            case up_staircase:
            case down_staircase:
            case up_steep_staircase:
            case down_steep_staircase:
            case entrance_to_store:
              break;

            default:
              if (randint(8) == 1) {
                flag = true;
              }
              break;
            }

            if (flag) {
              cave[i1][i2].fopen = true;
              t_list[cave[i1][i2].tptr] = blank_treasure;
              t_list[cave[i1][i2].tptr].p1 = tcptr;
              tcptr = cave[i1][i2].tptr;
              cave[i1][i2].tptr = 0;
              ctr++;
            }
          }
          if (ctr == 0) {
            cur_dis -= 6;
          }
        }
      }
    }
  } while (ctr <= 0);

  if (cur_dis < 66) {
    prt_map();
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
boolean test_light(long y, long x) {
  /*{ Tests a spot for light or field mark status		-RAK-	}*/
  return ((cave[y][x].pl) || (cave[y][x].fm) || (cave[y][x].tl));
}

/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
long distance(long y1, long x1, long y2, long x2) {
  register int dy, dx;

  /*
          ;	Distance returned is only an approximation based on :
          ;
          ;	dy = abs(y1-y2)
          ;	dx = abs(x1-x2)
          ;
          ;	distance =  2*(dy+dx) - MIN(dy,dx)
          ;		    ----------------------
          ;			      2
  */
  dy = y1 - y2;
  if (dy < 0)
    dy = -dy;
  dx = x1 - x2;
  if (dx < 0)
    dx = -dx;

  return ((((dy + dx) << 1) - (dy > dx ? dx : dy)) >> 1);
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
boolean los(long y1, long x1, long y2, long x2) {
  /*{ Returns true if no obstructions between two given points -RAK-}*/

  long ty, tx, stepy, stepx, p1, p2;
  float slp, tmp;
  boolean flag = true;

  /*  ENTER("los", "m") */
  /*  fprintf(debug_file,"los: (%d, %d) to (%d, %d)\n",x1,y1,x2,y2); */

  ty = (y1 - y2);
  tx = (x1 - x2);

  /*  fprintf(debug_file,"los: ty = %d   tx = %d\n",ty,tx); */

  if ((ty != 0) || (tx != 0)) {
    if (ty < 0) {
      stepy = -1;
    } else {
      stepy = 1;
    }
    if (tx < 0) {
      stepx = -1;
    } else {
      stepx = 1;
    }
    if (ty == 0) {

      /*      fprintf(debug_file,"los: ty==0  stepx = */
      /*      %d\n",stepx); */
      do {
        x2 += stepx;
        /*	fprintf(debug_file,"los: checking y=[%d]
         */
        /* x=[%d]  %d\n", */
        /*		y2,x2,cave[y2][x2].fopen); */
        flag = cave[y2][x2].fopen;
      } while (!((x1 == x2) || !flag));

    } else if (tx == 0) {

      /*      fprintf(debug_file,"los: tx==0  stepy = */
      /*      %d\n",stepy); */
      do {
        y2 += stepy;
        /*	fprintf(debug_file,"los: checking y=[%d]
         */
        /* x=[%d]  %d\n", */
        /*		y2,x2,cave[y2][x2].fopen); */
        flag = cave[y2][x2].fopen;
      } while (!((y1 == y2) || !flag));

    } else if (labs(ty) > labs(tx)) {

      /*      fprintf(debug_file,"los: ty>tx  stepy = */
      /*      %d\n",stepy); */
      slp = ((float)labs(tx) / (float)labs(ty)) * stepx;
      tmp = x2;
      do {
        y2 += stepy;
        tmp += slp;
        p1 = tmp - 0.1 + .5;
        p2 = tmp + 0.1 + .5;
        /*	fprintf(debug_file,"los: checking y=[%d]
         */
        /* x=[%d]  x=[%d]  %d : %d\n", */
        /*		y2,p1,p2,cave[y2][p1].fopen,cave[y2][p2].fopen);
         */
        if (!((cave[y2][p1].fopen) || (cave[y2][p2].fopen))) {
          /*	  fprintf(debug_file,"los: */
          /* setting ty>tx false\n"); */
          flag = false;
        }
      } while (!((y1 == y2) || !flag));

    } else {

      /*      fprintf(debug_file,"los: tx>ty  stepx = */
      /*      %d\n",stepx); */
      slp = ((float)labs(ty) / (float)labs(tx)) * stepy;
      tmp = y2;
      do {
        x2 += stepx;
        tmp += slp;
        p1 = tmp - 0.1 + .5;
        p2 = tmp + 0.1 + .5;
        /*	fprintf(debug_file,"los: checking y=[%d]
         */
        /* y=[%d]  x=[%d]  %d : %d\n", */
        /*		p1,p2,x2,cave[p1][x2].fopen,cave[p2][x2].fopen);
         */
        if (!((cave[p1][x2].fopen) || (cave[p2][x2].fopen))) {
          /*	  fprintf(debug_file,"los: */
          /* setting tx>ty false\n"); */
          flag = false;
        }
      } while (!((x1 == x2) || !flag));
    }
  }

  /*  RETURN("los", "m", 'b', "have los: ",&flag); */
  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void tlink() {
  /*{ Link all free space in treasure list together
   * }*/

  long i1;

  for (i1 = 0; i1 <= MAX_TALLOC; i1++) {
    t_list[i1] = blank_treasure;
    t_list[i1].p1 = i1 - 1;
  }

  tcptr = MAX_TALLOC;
}

void mlink() {
  /*{ Link all free space in monster list together
   * }*/

  long i1;

  for (i1 = 0; i1 <= MAX_MALLOC; i1++) {
    m_list[i1] = blank_monster;
    m_list[i1].nptr = i1 - 1;
  }

  m_list[2].nptr = 0; /* extra space saved for the win creatures? */
  muptr = 0;
  mfptr = MAX_MALLOC;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void place_gold(long y, long x) {
  /*{ Places a treasure (Gold or Gems) at given row, column -RAK-	}*/

  long cur_pos, i1;

  popt(&cur_pos);
  i1 = (2 + randint(dun_level + 4) + randint(dun_level + 4)) / 4;
  if (randint(obj_great) == 1) {
    i1 += randint(dun_level);
  }
  if (i1 > MAX_GOLD) {
    i1 = MAX_GOLD + 1 - randint(randint(3));
  }

  cave[y][x].tptr = cur_pos;
  t_list[cur_pos] = gold_list[i1 - 1];

  if (t_list[cur_pos].tval == valuable_metal) {
    t_list[cur_pos].number =
        randint(t_list[cur_pos].number) + t_list[cur_pos].number / 2;
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void place_a_staircase(long y, long x, long typ) {
  /*{ Place an up staircase at given y,x			-RAK-	}*/

  long cur_pos;

  if (cave[y][x].tptr != 0) {
    pusht(cave[y][x].tptr);
    cave[y][x].tptr = 0;
    cave[y][x].fopen = true;
  }

  popt(&cur_pos);
  cave[y][x].tptr = cur_pos;
  switch (typ) {
  case up_staircase:
    t_list[cur_pos] = up_stair;
    break;
  case down_staircase:
    t_list[cur_pos] = down_stair;
    break;
  case up_steep_staircase:
    t_list[cur_pos] = up_steep;
    break;
  case down_steep_staircase:
    t_list[cur_pos] = down_steep;
    break;
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void place_stairs(long typ, long num, long walls) {
  /*{ Places a staircase 1=up, 2=down			-RAK-	}*/

  long i1, i2, y1, x1, y2, x2;
  boolean flag;

  for (i1 = 0; i1 < num; i1++) {
    flag = false;
    do {
      i2 = 0;
      do {
        y1 = randint(cur_height - 12);
        x1 = randint(cur_width - 12);
        y2 = y1 + 12;
        x2 = x1 + 12;
        do {
          do {
            if (is_in(cave[y1][x1].fval, open_dry_floors)) {
              if (cave[y1][x1].tptr == 0) {
                if (next_to4(y1, x1, wall_set) >= walls) {
                  flag = true;
                  place_a_staircase(y1, x1, typ);
                }
              }
            }
            x1++;
          } while (!((x1 == x2) || (flag)));
          x1 = x2 - 12;
          y1++;
        } while (!((y1 == y2) || (flag)));
        i2++;
      } while (!((flag) || (i2 > 30)));
      walls--;
    } while (!flag);
  } /* end for */
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
long get_obj_num(long level, long tries) {
  /*{ Returns the array number of a random object		-RAK-	}*/

  /* pick an object tries times, keep the one with the highest level */
  /* here are the comments from umoria:                              */

  /* This code has been added to make it slightly more likely to get the
     higher level objects.	Originally a uniform distribution over
     all
     objects less than or equal to the dungeon level.  This distribution
     makes a level n objects occur approx 2/n% of the time on level n,
     and 1/2n are 0th level. */

  long i1, i2, i3, i4;

  if (level > MAX_OBJ_LEVEL) {
    level = MAX_OBJ_LEVEL;
  }

  if (randint(obj_great) == 1) {
    level = MAX_OBJ_LEVEL;
  }

  i4 = 1;
  i3 = i2 = 0;
  do {
    if (level == 0) {
      i1 = randint(t_level[0]);
      if (object_list[i1].level >= i3) {
        i4 = i1;
        i3 = object_list[i1].level;
      }
    } else {
      i1 = randint(t_level[level]);
      if (object_list[i1].level >= i3) {
        i4 = i1;
        i3 = object_list[i1].level;
      }
    }
  } while (++i2 < tries);

  return i4;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */

void place_object(long y, long x) {
  /*{ Places an object at given row, column co-ordinate	-RAK-	}*/
  long cur_pos;

  popt(&cur_pos);
  cave[y][x].tptr = cur_pos;
  t_list[cur_pos] = object_list[get_obj_num(dun_level, PLACE_OBJECT_TRIES)];
  magic_treasure(cur_pos, dun_level, false);
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
long next_to4(long y, long x, obj_set group_set) {
  /*{ Checks points north, south, east, and west for a type -RAK-	}*/

  long i1;

  i1 = 0;

  if (y > 1) {
    if (is_in(cave[y - 1][x].fval, group_set)) {
      i1++;
    }
  }

  if (y < cur_height) {
    if (is_in(cave[y + 1][x].fval, group_set)) {
      i1++;
    }
  }

  if (x > 1) {
    if (is_in(cave[y][x - 1].fval, group_set)) {
      i1++;
    }
  }

  if (x < cur_width) {
    if (is_in(cave[y][x + 1].fval, group_set)) {
      i1++;
    }
  }

  return i1;
}

long next_to8(long y, long x, obj_set group_set) {
  /*{ Checks all adjacent spots for elements		-RAK-	*/

  long i1, i2, i3;

  i1 = 0;
  for (i2 = (y - 1); i2 <= (y + 1); i2++) {
    for (i3 = (x - 1); i3 <= (x + 1); i3++) {
      if (in_bounds(i2, i3)) {
        if (is_in(cave[i2][i3].fval, group_set)) {
          i1++;
        }
      }
    }
  }

  return i1;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
long max_hp(char const *hp_str) {
  /* Gives Max hit points    -RAK- */

  long num;
  long die;
  char hp_copy[7];
  long return_value;
  char *ptr;

  strcpy(hp_copy, hp_str);
  ptr = strchr(hp_copy, 'd');
  if (ptr != NULL)
    *ptr = ' ';
  sscanf(hp_copy, "%ld %ld", &num, &die);
  return_value = num * die;

  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
long damroll(char const *dice) {
  /*{ Converts input string into a dice roll		-RAK-	}*/
  /*{	Normal input string will look like '2d6', '3d8'... etc. }*/

  char dice_copy[7];
  long num = 0;
  long sides = 0;
  long return_value = 0;
  char *ptr;

  ENTER(("damroll", "m"));

  strcpy(dice_copy, dice);
  ptr = strchr(dice_copy, 'd');
  if (ptr != NULL)
    *ptr = ' ';
  sscanf(dice_copy, "%ld %ld", &num, &sides);
  return_value = rand_rep(num, sides);

  RETURN("damroll", "m", 'd', "damage", &return_value);
  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
#define max(xx, yy) (((xx) > (yy)) ? (xx) : (yy))
#define min(xx, yy) (((xx) < (yy)) ? (xx) : (yy))

long maxmin(long x, long y, long z) {
  /* return max( min(x,y) - 1, z ) */
  long i1;

  i1 = min(x, y) - 1;
  return max(i1, z);
}

long minmax(long x, long y, long z) {
  /* return min( max(x,y) + 1, z ) */
  long i1;

  i1 = max(x, y) + 1;
  return min(i1, z);
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
boolean summon_land_monster(long *y, long *x, boolean slp) {
  /*{ Places land creature adjacent to given location	-RAK-	}*/

  long i1, i2, i3, i4, i5, count;
  boolean flag;
  boolean return_value = false;

  i1 = 0;
  i5 = dun_level + MON_SUMMON_ADJ;

  do {
    i2 = *y - 2 + randint(3);
    i3 = *x - 2 + randint(3);
    if (in_bounds(i2, i3)) {
      /* with cave[i2][i3]. do; */
      if (is_in(cave[i2][i3].fval, earth_set)) {
        if (cave[i2][i3].cptr == 0) {
          if (cave[i2][i3].fopen) {
            flag = false;
            count = 0;
            do {
              if (i5 > MAX_MONS_LEVEL) {
                i4 = MAX_MONS_LEVEL;
              } else {
                i4 = i5;
              }
              if (dun_level == 0) {
                i4 = randint(m_level[0]);
              } else {
                i4 = randint(m_level[i4]) + m_level[0];
              }
              if (i4 > MAX_CREATURES) {
                i4 = MAX_CREATURES;
              }
              if (((c_list[i4].cmove & 0x00008000) == 0) &&
                  (((c_list[i4].cmove & 0x00000010) == 0) ||
                   ((c_list[i4].cmove & 0x00000040) == 0) ||
                   ((c_list[i4].cmove & 0x00800000) != 0))) {
                place_monster(i2, i3, i4, slp);
                return_value = true;
                flag = true;
              }
              count++;
            } while (!((flag) || (count > 10)));
            i1 = 9;
            *y = i2;
            *x = i3;
          }
        }
      }
    }
    i1++;
  } while (i1 <= 9);

  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
boolean summon_water_monster(long *y, long *x, boolean slp) {
  /*{ Places water creature adjacent to given location  -DMF-   }*/

  long i1, i2, i3, i4, i5, count;
  boolean flag;
  boolean return_value = false;

  i1 = 0;
  i5 = dun_level + MON_SUMMON_ADJ;

  do {
    i2 = *y - 2 + randint(3);
    i3 = *x - 2 + randint(3);
    if (in_bounds(i2, i3)) {
      /* with cave[i2][i3]. do; */
      if (is_in(cave[i2][i3].fval, water_set)) {
        if (cave[i2][i3].cptr == 0) {
          if (cave[i2][i3].fopen) {
            flag = false;
            count = 0;
            do {
              if (i5 > MAX_MONS_LEVEL) {
                i4 = MAX_MONS_LEVEL;
              } else {
                i4 = i5;
              }
              if (dun_level == 0) {
                i4 = randint(m_level[0]);
              } else {
                i4 = randint(m_level[i4]) + m_level[0];
              }
              if (i4 > MAX_CREATURES) {
                i4 = MAX_CREATURES;
              }
              if ((((c_list[i4].cmove & 0x00008000) == 0) &&
                   (((c_list[i4].cmove & 0x00000010) != 0) ||
                    ((c_list[i4].cmove & 0x00000040) == 0) ||
                    ((c_list[i4].cmove & 0x00800000) != 0)))) {
                place_monster(i2, i3, i4, slp);
                return_value = true;
                flag = true;
              }
              count++;
            } while (!((flag) || (count > 10)));
            i1 = 9;
            *y = i2;
            *x = i3;
          }
        }
      }
    }
    i1++;
  } while (i1 <= 9);

  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
boolean summon_undead(long *y, long *x) {
  /*{ Places undead adjacent to given location          -RAK-   }*/

  long i1, i2, i3, i4, i5, ctr;
  obj_set undead_set = {1, 2, 4, 5, 0};
  boolean return_value = false;

  i1 = 0;
  i4 = m_level[MAX_MONS_LEVEL] + m_level[0];

  do {
    i5 = randint(i4);
    ctr = 0;
    do {
      if ((c_list[i5].cdefense & 0x0008) != 0) {
        ctr = 20;
        i4 = 0;
      } else {
        i5++;
        if (i5 > i4) {
          ctr = 20;
        } else {
          ctr++;
        }
      }
    } while (ctr <= 19);
  } while (i4 != 0);

  do {
    i2 = *y - 2 + randint(3);
    i3 = *x - 2 + randint(3);
    if (in_bounds(i2, i3)) {
      /* with cave[i2,i3] do; */
      if (is_in(cave[i2][i3].fval, undead_set)) {
        if ((cave[i2][i3].cptr == 0) && (cave[i2][i3].fopen)) {
          place_monster(i2, i3, i5, false);
          return_value = true;
          i1 = 9;
          *y = i2;
          *x = i3;
        }
      }
    }
    i1++;
  } while (i1 <= 9);

  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
boolean summon_demon(long *y, long *x) {
  /*{ Places demon adjacent to given location           -RAK-   }*/

  long i1, i2, i3, i4, i5, ctr;
  obj_set demon_set = {1, 2, 4, 5, 0};
  boolean return_value = false;

  i1 = 0;
  i4 = m_level[MAX_MONS_LEVEL] + m_level[0];
  do {
    i5 = randint(i4);
    ctr = 0;
    do {
      /*{        Check monsters for demon }*/
      if ((c_list[i5].cdefense & 0x0400) != 0) {
        ctr = 20;
        i4 = 0;
      } else {
        i5++;
        if (i5 > i4) {
          ctr = 20;
        } else {
          ctr++;
        }
      }
    } while (ctr <= 19);
  } while (i4 != 0);

  do {
    do {
      i2 = *y - 2 + randint(3);
      i3 = *x - 2 + randint(3);
    } while (!((i2 != *y) || (i3 != *x)));
    if (in_bounds(i2, i3)) {
      /* with cave[i2,i3] do; */
      if (is_in(cave[i2][i3].fval, demon_set)) {
        if ((cave[i2][i3].cptr == 0) && (cave[i2][i3].fopen)) {
          place_monster(i2, i3, i5, false);
          return_value = true;
          i1 = 9;
          *y = i2;
          *x = i3;
        }
      }
    }
    i1++;
  } while (i1 <= 9);

  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
boolean summon_breed(long *y, long *x) {
  /*{ Places breeding monster adjacent to given location }*/

  long i1, i2, i3, i4, i5, ctr;
  /*  obj_set   breed_set = {1,2,4,5,0}; */
  boolean return_value = false;

  i1 = 0;
  do {
    i2 = *y - 2 + randint(3);
    i3 = *x - 2 + randint(3);
    if (in_bounds(i2, i3)) {
      /* with cave[i2,i3] do; */
      if ((is_in(cave[i2][i3].fval, earth_set)) ||
          (is_in(cave[i2][i3].fval, water_set))) {
        if ((cave[i2][i3].cptr == 0) && (cave[i2][i3].fopen)) {
          i4 = m_level[MAX_MONS_LEVEL] + m_level[0];
          do {
            i5 = randint(i4);
            ctr = 0;
            do {
              if (((c_list[i5].cmove & 0x00200000) != 0) &&
                  (((is_in(cave[i2][i3].fval, earth_set)) &&
                    (((c_list[i5].cmove & 0x00000010) == 0) ||
                     ((c_list[i5].cmove & 0x00000040) == 0) ||
                     ((c_list[i5].cmove & 0x00800000) != 0))) ||
                   ((is_in(cave[i2][i3].fval, water_set)) &&
                    (((c_list[i5].cmove & 0x00000010) != 0) ||
                     ((c_list[i5].cmove & 0x00000040) == 0) ||
                     ((c_list[i5].cmove & 0x00800000) != 0))))) {
                ctr = 20;
                i4 = 0;
              } else {
                i5++;
                if (i5 > i4) {
                  ctr = 20;
                } else {
                  ctr++;
                }
              }
            } while (ctr <= 19);
          } while (i4 != 0);
          place_monster(i2, i3, i5, false);
          return_value = true;
          i1 = 9;
          *y = i2;
          *x = i3;
        }
      }
    }
    i1++;
  } while (i1 <= 9);

  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
boolean player_saves(long adjust) {
  /*{ Saving throws for player character... 		-RAK-	}*/
  boolean return_value;
  return_value = (randint(100) <= player_save + adjust) && (randint(20) != 1);
  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
boolean player_spell_saves() {
  boolean return_value;
  return_value = player_saves(player_lev + 5 * C_player_mod_from_stat(WIS));
  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void sp__takey_munny(long coin_value, long *bank_assets, long *to_bank,
                     long *from_bank) {
  long trans;

  trans = (*to_bank * GOLD_VALUE) / coin_value;
  if (*bank_assets < trans) {
    trans = *bank_assets;
  }
  *bank_assets -= trans;
  *from_bank += (trans * coin_value) / GOLD_VALUE;
  *to_bank -= (trans * coin_value) / GOLD_VALUE;
  player_account -= (trans * coin_value) / GOLD_VALUE;
}
/*//////////////////////////////////////////////////////////////////// */
boolean send_page(long to_bank) {
  /*{ Send a page to the bank to fetch money		-DMF-	}*/

  boolean back;
  long from_bank;
  char out_val[134];

  back = false;
  if (get_yes_no("Do you wish to send a page to the bank for money?")) {
    from_bank = 0;
    if (player_account < to_bank) {
      msg_print("The page returns and says that your balance "
                "is too low.");
    } else if (bank[TOTAL_] < to_bank) {
      msg_print("The page returns and says that the bank is "
                "out of money.");
    } else {
      sp__takey_munny(coin_value[MITHRIL], &bank[MITHRIL], &to_bank,
                      &from_bank);
      sp__takey_munny(coin_value[PLATINUM], &bank[PLATINUM], &to_bank,
                      &from_bank);
      sp__takey_munny(GOLD_VALUE, &bank[GOLD], &to_bank, &from_bank);
      if (randint(mugging_chance) == 1) {
        msg_print("The page was mugged while returning "
                  "from the bank!");
        sprintf(out_val, "You have lost %ld gold pieces!", from_bank);
        msg_print(out_val);
      } else {
        sprintf(out_val, "The page returns with %ld gold pieces.", from_bank);
        msg_print(out_val);
        subtract_money(player_money[TOTAL_] * GOLD_VALUE, false);
        back = true;
      }
    }
    msg_print(" ");
  } else {
    msg_print("You cannot buy that with the money you are carrying.");
  }

  return back;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void time_diff(game_time_type a, game_time_type b, game_time_type *c) {
  /*{ Return the difference of two time records             -DMF-   }*/

  if (a.secs < b.secs) {
    a.secs = a.secs + 400;
    a.hour = a.hour - 1;
  }
  c->secs = a.secs - b.secs;

  if (a.hour < b.hour) {
    a.hour = a.hour + 24;
    a.day = a.day - 1;
  }
  c->hour = a.hour - b.hour;

  if (a.day < b.day) {
    a.day = a.day + 28;
    a.month = a.month - 1;
  }
  c->day = a.day - b.day;

  if (a.month < b.month) {
    a.month = a.month + 13;
    a.year = a.year - 1;
  }
  c->month = a.month - b.month;

  c->year = a.year - b.year;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
char *show_char_age(char result[134]) {
  /*{ Return string for the age of the character            -DMF-   }*/

  game_time_type dif;
  char out_val[82];

  time_diff(player_cur_age, player_birth, &dif);

  sprintf(result, "You are %ld years, %d months, %d days, and %s hours old.",
          dif.year, dif.month, dif.day,
          time_string(dif.hour, dif.secs, out_val));

  if (dif.year == 1) {
    insert_str(result, "years", "year");
  }
  if (dif.month == 1) {
    insert_str(result, "months", "month");
  }
  if (dif.day == 1) {
    insert_str(result, "days", "day");
  }

  return result;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
char *play_time(time_type *t, char *result) {
  /*{ Return string for how long character has been playing -DMF-   }*/

  sprintf(result, "%d day%s and %d:%02d:%02d hours.", t->days,
          ((t->days == 1) ? "" : "s"), t->hours, t->minutes, t->seconds);

  return result;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*
        { Add two time_types together                           -DMF-   }
[global,psect(misc5$code)] procedure add_play_time(
                var res : time_type;
                add     : time_type);
      begin
        with res do
          begin
            days := days + add.days;
            hours := hours + add.hours;
            minutes := minutes + add.minutes;
            seconds := seconds + add.seconds;
            hundredths := hundredths + add.hundredths;
            if hundredths > 100 then
              begin
                hundredths := hundredths - 100;
                seconds := seconds + 1;
              end;
            if seconds > 60 then
              begin
                seconds := seconds - 60;
                minutes := minutes + 1;
              end;
            if minutes > 60 then
              begin
                minutes := minutes - 60;
                hours := hours + 1;
              end;
            if hours > 24 then
              begin
                hours :=  hours - 24;
                days := days + 1;
              end;
          end;
      end;
*/
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
time_type *convert_seconds_to_time(time_t seconds, time_type *tim) {
  tim->years = 0;
  tim->months = 0;
  tim->hundredths = 0;

  tim->days = seconds / (60 * 60 * 24);
  seconds -= tim->days * (60 * 60 * 24);

  tim->hours = seconds / (60 * 60);
  seconds -= tim->hours * (60 * 60);

  tim->minutes = seconds / (60);
  seconds -= tim->minutes * (60);

  tim->seconds = seconds;

  return tim;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
time_t convert_time_to_seconds(time_type *tim) {
  time_t t;

  t = (tim->days * (60 * 60 * 24) + tim->hours * (60 * 60) +
       tim->minutes * (60) + tim->seconds);

  return t;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
char *full_date_string(game_time_type time, char result[134]) {
  /*{ Return string with entire date/time                   -DMF-   }*/

  char out1[134], out2[134], out3[134], out4[134];
  char *pos;

  day_of_week_string(time.day, 10, out1);
  pos = strstr(out1, " ");
  if (pos != NULL) {
    *pos = 0;
  }
  /* with time do; */
  sprintf(result, "%s, %s the %s, %s", out1, month_string(time.month, out2),
          place_string(time.day, out3),
          time_string(time.hour, time.secs, out4));

  return result;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
char *show_current_time(char *result) {
  /*{ Return current time in the game                       -DMF-   }*/
  /* Tue Jul 07 00:05:40 1998 */

  /* quad_type    current_time; */
  /* vtype        out; */

  time_t cur_time;
  struct tm split_time;

  cur_time = time(NULL);
  split_time = *localtime(&cur_time);
  strftime(result, 70, "%c", &split_time);

  return result;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
long rotate_dir(long dir, long rot) {
  long return_value;

  /*  ENTER("rotate_dir", "m") */
  if (dir == 5) {
    return_value = 5;
  } else {
    for (; rot < 0; rot += 8)
      ; /* make sure the mod is a positive number */
    return_value = key_of[(oct_of[dir] + rot) % 8];
  }

  /*  RETURN("rotate_dir", "m", 'd',"dir",&return_value) */
  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
long attack_blows(long weight, long *wtohit) {
  /*{ Weapon weight VS strength and dexterity               -RAK-   }*/

  const long max_wield = C_player_max_bulk() / 10;
  const int dex_mod = C_player_mod_from_stat(DEX);
  const int approx_str_stat = (10 + (C_player_mod_from_stat(STR) * 2)) * 10;

  long adj_weight;
  long blows = 1;
  long lev_skill;

  *wtohit = 0;

  /*{ make to-hit drop off gradually instead of being so abrupt -DCJ- }*/
  if (max_wield < (weight / 100)) {
    *wtohit = max_wield - (weight / 100);
    return blows;
  }

  blows = 5 + dex_mod;
  blows = min(12, blows);
  blows = max(3, blows);

  lev_skill = C_class_melee_bonus(player_pclass) * (player_lev + 10);

  /*{warriors 100-500, paladin 80-400, priest 60-300, mage
   * 40-200}*/
  blows = trunc(0.8 + (float)blows / 3.0 + (float)lev_skill / 350.0);

  /*{usually 3 for 18+ dex, 5 max except 6 for high level
   * warriors}*/
  adj_weight = (long)((float)approx_str_stat / (float)(weight / 100) * 2.5);

  if (adj_weight < 1) {
    blows = 1;
  } else if (adj_weight < 2) {
    blows = (blows / 3.00);
  } else if (adj_weight < 3) {
    blows = (blows / 2.50);
  } else if (adj_weight < 5) {
    blows = (blows / 2.00);
  } else if (adj_weight < 10) {
    blows = (blows / 1.66);
  } else {
    blows = (blows / 1.50);
  }

  return blows;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
long critical_blow(long weight, long plus, boolean cs_sharp, boolean is_fired) {
  /*{ Critical hits, Nasty way to die...                    -RAK-   }*/

  long randomthing, py_crit;
  long return_value = 0;

  weight = weight / 100;
  if (cs_sharp) {
    weight += 600;
  }

  /*{ Weight of weapon, pluses to hit, and character level all      }*/
  /*{ contribute to the chance of a critical                        }*/

  /* with player_do; */
  if (is_fired) {
    py_crit = C_class_ranged_bonus(player_pclass);
  } else {
    py_crit = C_class_melee_bonus(player_pclass);

    if (player_pclass == C_MONK) { /*{ monks are crit specialists }*/
      py_crit *= 2;
    }
  }

  if (randint(5000) <= (weight + 6 * plus + py_crit * (player_lev + 10))) {
    randomthing = randint(300 + randint(weight));
    if (randomthing <= 150) {
      return_value = 1;
      msg_print("It was a good hit! (x2 damage)");
    } else if (randomthing <= 250) {
      return_value = 2;
      msg_print("It was an excellent hit! (x3 damage)");
    } else if (randomthing <= 375) {
      return_value = 3;
      msg_print("It was a superb hit! (x4 damage)");
    } else if (randomthing <= 550) {
      return_value = 4;
      msg_print("It was a *GREAT* hit! (x5 damage)");
    } else if (randomthing < 700) {
      return_value = 6;
      msg_print("It was an *INCREDIBLE* hit! (x7 damage)");
    } else if (randomthing < 875) {
      return_value = 9;
      msg_print("It was an *AMAZING* hit! (x10 damage)");
    } else {
      return_value = 14;
      msg_print("It was a **PERFECT** hit! (x15 damage)");
    }
  }

  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void find_monster_name(char m_name[82], const long ptr,
                       boolean begin_sentence) {
  long i2;

  i2 = m_list[ptr].mptr;

  /*{ Does the player know what he's fighting?      }*/
  if ((((0x10000 & c_list[i2].cmove) != 0) && (!(player_flags.see_inv))) ||
      (player_flags.blind > 0) || (!(m_list[ptr].ml))) {
    if (begin_sentence) {
      strcpy(m_name, "It");
    } else {
      strcpy(m_name, "it");
    }
  } else {
    if (begin_sentence) {
      sprintf(m_name, "The %s", c_list[i2].name);
    } else {
      sprintf(m_name, "the %s", c_list[i2].name);
    }
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
long get_hexdecant(long dy, long dx) {
  /*{ Returns hexdecant of dy,dx                    }*/
  /*{ 0,1 = ea 2,3 = ne, 4,5 = n ... 14,15 = se     }*/

  long ay, ax;
  long hexdecant;
  long return_value;

  ENTER(("get_hexdecant", "m"));

  ay = labs(dy);
  ax = labs(dx);

  if (ay * 2.41421 < ax) {
    hexdecant = 1;
  } else if (ay < ax) {
    hexdecant = 2;
  } else if (ay / 2.41421 < ax) {
    hexdecant = 3;
  } else {
    hexdecant = 4;
  }

  if (dx < 0) {
    hexdecant = 9 - hexdecant;
  }

  if (dy > 0) {
    return_value = (17 - hexdecant) % 16;
  } else {
    return_value = hexdecant;
  }

  RETURN("get_hexdecant", "m", 'd', "dir", &return_value);
  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void petrify(long amt) {
  (player_flags).petrification += randint(amt);

  if ((player_flags).petrification < 100) {
    msg_print("You feel your joints stiffening.");
  } else if ((player_flags).petrification < 150) {
    msg_print("Your feet are beginning to feel heavy.");
  } else if ((player_flags).petrification < 200) {
    msg_print("Your knees are no longer able to bend.");
  } else if ((player_flags).petrification < 250) {
    msg_print("Your legs feel like blocks of stone.");
  } else if ((player_flags).petrification < 300) {
    msg_print("You are finding it difficult to breathe.");
  } else {
    msg_print("You have turned to stone.");
    strcpy(died_from, "petrification");
    upon_death();
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void place_win_monster() {
  /*{ Places a monster at given location                    -RAK-   }*/

  long cur_pos;
  long y, x;

  if (!total_winner) {
    popm(&cur_pos);
    /* with m_list[cur_pos] do; */
    do {
      y = randint(cur_height - 2) + 1;
      x = randint(cur_width - 2) + 1;
    } while (!((is_in(cave[y][x].fval, open_dry_floors)) &&
               (cave[y][x].cptr == 0) && (cave[y][x].tptr == 0) &&
               (distance(y, x, char_row, char_col) > MAX_SIGHT)));

    m_list[cur_pos].fy = y;
    m_list[cur_pos].fx = x;

    m_list[cur_pos].mptr =
        (randint(WIN_MON_TOT) + m_level[MAX_MONS_LEVEL] + m_level[0]);
    m_list[cur_pos].nptr = muptr;
    muptr = cur_pos;

    if ((c_list[m_list[cur_pos].mptr].cdefense & 0x4000) != 0) {
      m_list[cur_pos].hp = max_hp(c_list[m_list[cur_pos].mptr].hd);
    } else {
      m_list[cur_pos].hp = damroll(c_list[m_list[cur_pos].mptr].hd);
    }

    m_list[cur_pos].cdis = distance(char_row, char_col, y, x);
    m_list[cur_pos].cspeed =
        c_list[m_list[cur_pos].mptr].speed + (player_flags).speed;
    m_list[cur_pos].stunned = 0;
    m_list[cur_pos].csleep = 0;
    cave[y][x].cptr = cur_pos;
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void alloc_object(obj_set alloc_set, long typ, long num) {
  /*{ Allocates an object for tunnels and rooms             -RAK-   }*/

  long i1, i2, i3;

  for (i3 = 1; i3 <= num; i3++) {
    do {
      i1 = randint(cur_height);
      i2 = randint(cur_width);
    } while (
        !((is_in(cave[i1][i2].fval, alloc_set)) && (cave[i1][i2].tptr == 0)));

    switch (typ) {
    case 1:
      place_trap(i1, i2, 1, randint(MAX_TRAPA));
      break;
    case 2:
      place_trap(i1, i2, 2, randint(MAX_TRAPB));
      break;
    case 3:
      place_rubble(i1, i2);
      break;
    case 4:
      place_gold(i1, i2);
      break;
    case 5:
      place_object(i1, i2);
      break;
    }
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void place_open_door(long y, long x) {
  long cur_pos;

  popt(&cur_pos);

  cave[y][x].tptr = cur_pos;
  t_list[cur_pos] = door_list[0];
  cave[y][x].fval = corr_floor3.ftval;
  cave[y][x].fopen = true;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void place_broken_door(long y, long x) {
  long cur_pos;

  popt(&cur_pos);

  cave[y][x].tptr = cur_pos;
  t_list[cur_pos] = door_list[0];
  cave[y][x].fval = corr_floor3.ftval;
  cave[y][x].fopen = true;
  t_list[cur_pos].p1 = 1;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void place_closed_door(long y, long x) {
  long cur_pos;

  popt(&cur_pos);

  cave[y][x].tptr = cur_pos;
  t_list[cur_pos] = door_list[1];
  cave[y][x].fval = corr_floor3.ftval;
  cave[y][x].fopen = false;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void place_locked_door(long y, long x) {
  long cur_pos;

  popt(&cur_pos);

  cave[y][x].tptr = cur_pos;
  t_list[cur_pos] = door_list[1];
  cave[y][x].fval = corr_floor3.ftval;
  cave[y][x].fopen = false;
  t_list[cur_pos].p1 = randint(10) + 10;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void place_stuck_door(long y, long x) {
  long cur_pos;

  popt(&cur_pos);

  cave[y][x].tptr = cur_pos;
  t_list[cur_pos] = door_list[1];
  cave[y][x].fval = corr_floor3.ftval;
  cave[y][x].fopen = false;
  t_list[cur_pos].p1 = -randint(10) - 10;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void place_secret_door(long y, long x) {
  long cur_pos;

  popt(&cur_pos);

  cave[y][x].tptr = cur_pos;
  t_list[cur_pos] = door_list[2];
  cave[y][x].fval = corr_floor4.ftval;
  cave[y][x].fopen = false;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void place_door(long y, long x) {

  switch (randint(3)) {
  case 1:
    switch (randint(4)) {
    case 1:
      place_broken_door(y, x);
      break;
    default:
      place_open_door(y, x);
      break;
    }
    break;

  case 2:
    switch (randint(12)) {
    case 1:
    case 2:
      place_locked_door(y, x);
      break;
    case 3:
      place_stuck_door(y, x);
      break;
    default:
      place_closed_door(y, x);
      break;
    }
    break;

  case 3:
    place_secret_door(y, x);
    break;
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void random_object(long y, long x, long num) {
  /*{ Creates objects nearby the coordinates given          -RAK-   }*/

  long i1, i2, i3;

  do {
    i1 = 0;
    do {
      i2 = y - 3 + randint(5);
      i3 = x - 4 + randint(7);

      /* with cave[i2][i3]. do; */
      if (is_in(cave[i2][i3].fval, floor_set)) {
        if (cave[i2][i3].tptr == 0) {
          if (randint(100) < 75) {
            place_object(i2, i3);
          } else {
            place_gold(i2, i3);
          }
          i1 = 9;
        }
      }
      i1++;
    } while (i1 <= 10);
    num--;
  } while (num != 0);
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*{ Returns a rating of x depending on y                  -JWT-   }
 * misc::mod_to_string
 */
char *likert(long x, long y, char *out_str) {

  if ((int)(x / y) < -3) {
    strcpy(out_str, "Very Bad");
  } else {
    switch ((int)(x / y)) {
    case -3:
    case -2:
    case -1:
      strcpy(out_str, "Very Bad");
      break;
    case 0:
    case 1:
      strcpy(out_str, "Bad");
      break;
    case 2:
      strcpy(out_str, "Poor");
      break;
    case 3:
    case 4:
      strcpy(out_str, "Fair");
      break;
    case 5:
      strcpy(out_str, "Good");
      break;
    case 6:
      strcpy(out_str, "Very Good");
      break;
    case 7:
    case 8:
      strcpy(out_str, "Superb");
      break;
    default:
      strcpy(out_str, "Excellent");
      break;
    }
  }

  return out_str;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */

/* END FILE  misc.c */
