/* Here you should find the guts of the game */

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
#include "dungeon.h"
#include "fighting_ranged.h"
#include "kickout.h"
#include "magic.h"
#include "pascal.h"
#include "player.h"
#include "routines.h"
#include "save.h"
#include "stores.h"
#include "term.h"
#include "types.h"
#include "variables.h"

void C_print_known_spells();

#define OBJ_LAMP_MAX 15000 /*{ Maximum amount that lamp can be filled} */

static boolean light_flag;        /*	{ Used in MOVE_LIGHT  } */
static boolean cave_flag = false; /*	{ Used in GET_PANEL   } */
static float acc_exp = 0.0;       /*{ Accumulator for fractional exp} */
static long old_chp;               /* { Detect change         } */
static long old_cmana;             /* { Detect change         } */
static boolean player_light;    /* { Player carrying light } */
static boolean save_msg_flag;   /* { Msg flag after INKEY  } */

float regen_amount;      /* { Regenerate hp and mana} */
boolean moria_flag;      /* { Next level when true  } */
boolean reset_flag;      /* { Do not move creatures } */
boolean search_flag;     /* { Player is searching   } */
boolean teleport_flag;   /* { Handle telport traps  } */
char s1[70];             /* { Summon item strings   } */
char s2[70];             /* { Summon item strings   } */
char s3[70];             /* { Summon item strings   } */
char s4[70];             /* { Summon item strings   } */
long i_summ_count;       /* { Summon item count	   } */

/**
 * -RAK-
 * s__panel_bounds() - Calculates current boundries
 */
static void s__panel_bounds() {
  panel_row_min = (trunc(panel_row * (SCREEN_HEIGHT / 2)) + 1);
  panel_row_max = panel_row_min + SCREEN_HEIGHT - 1;

  panel_col_min = (trunc(panel_col * (SCREEN_WIDTH / 2)) + 1);
  panel_col_max = panel_col_min + SCREEN_WIDTH - 1;

  panel_row_prt = panel_row_min - 2;
  panel_col_prt = panel_col_min - 15;
}

static void s__get_money_type__prompt_money(char astr[82], char out_val[134],
                                            boolean *commas) {
  if (*commas) {
    strcat(out_val, ", ");
  }
  strcat(out_val, astr);
  *commas = true;
}

static void ml__draw_block(long y1, long x1, long y2, long x2) {
  /*{ Given two sets of points, draw the block		}*/

  long const topp = maxmin(y1, y2, panel_row_min);
  long const bott = minmax(y1, y2, panel_row_max);
  long const left = maxmin(x1, x2, panel_col_min);
  long const right = minmax(x1, x2, panel_col_max);
  long const new_topp = y2 - 1; /*{ Margins for new things to appear}*/
  long const new_bott = y2 + 1;
  long const new_left = x2 - 1;
  long const new_righ = x2 + 1;

  long xmax = 0;

  ENTER(("ml__draw_block", "%d, %d, %d, %d", y1, x2, y2, x2));

  /*{ From uppermost to bottom most lines player was on...  }*/
  /*{ Points are guaranteed to be on the screen (I hope...) }*/

  for (long y = topp; y <= bott; y++) {
    long xpos = 0;
    chtype floor_str[82] = {0};
    long floor_str_len = 0;

    /*{ Leftmost to rightmost do}*/
    for (long x = left; x <= right; x++) {
      chtype tmp_char = ' ';
      boolean flag;

      if (cave[y][x].pl || cave[y][x].fm) {
        flag = ((y == y1 && x == x1) || (y == y2 && x == x2));
        /* flag = true; */
      } else {
        flag = true;
        if ((y >= new_topp && y <= new_bott) &&
            (x >= new_left && x <= new_righ) && cave[y][x].tl) {
          if (is_in(cave[y][x].fval, pwall_set)) {
            cave[y][x].pl = true;
          } else if (cave[y][x].tptr > 0 &&
                     is_in(t_list[cave[y][x].tptr].tval, light_set) &&
                     !(cave[y][x].fm)) {
            cave[y][x].fm = true;
          }
        }
      }

      if (cave[y][x].pl || cave[y][x].tl || cave[y][x].fm)
        tmp_char = loc_symbol(y, x);
      if (player_flags.image > 0 && randint(12) == 1)
        tmp_char = (char)(randint(95) + 31);

      if (flag) {
        if (xpos == 0)
          xpos = x;
        xmax = x;
      }

      if (xpos > 0) {
        if (floor_str_len > 80)
          MSG((": ERROR draw_block floor_str_len "
               "too big: %d",
               floor_str_len));
        floor_str[floor_str_len++] = tmp_char;
      }

    } /* end for  x */

    if (floor_str_len > 80)
      MSG((": ERROR draw_block floor_str_len too big: %d", floor_str_len));

    floor_str[floor_str_len] = 0;

    if (xpos > 0) {
      /*{ Var for PRINT cannot be loop index}*/
      long const y2 = y;
      /*print(substr(floor_str,1,1+xmax-xpos),y2,xpos);*/

      if (1 + xmax - xpos + 1 > 80 || 1 + xmax - xpos + 1 < 0)
        MSG((": ERROR draw_block xmax-xpos is bad\n"));

      floor_str[1 + xmax - xpos + 1] = 0;
      print_chstr(floor_str, y2, xpos);
    }
  }

  LEAVE("ml__draw_block", "m");
}

static void ml__sub1_move_light(long y1, long x1, long y2, long x2) {
  /*{ Normal movement                                   }*/

  ENTER(("ml__sub1_move_light", "%d, %d, %d, %d", y1, x1, y2, x2));

  light_flag = true;

  /* Turn off lamp light */
  for (long i = y1 - 1; i <= y1 + 1; i++)
    for (long j = x1 - 1; j <= x1 + 1; j++)
      cave[i][j].tl = false;

  for (long i = y2 - 1; i <= y2 + 1; i++)
    for (long j = x2 - 1; j <= x2 + 1; j++)
      cave[i][j].tl = true;

  ml__draw_block(y1, x1, y2, x2); /*{ Redraw area           }*/

  LEAVE("ml__sub1_move_light", "m");
}

static void ml__sub2_move_light(long y1, long x1, long y2, long x2) {
  /*{ When FIND_FLAG, light only permanent features     }*/

  ENTER(("ml__sub2_move_light", "%d, %d, %d, %d", y1, x1, y2, x1));

  if (light_flag) {
    for (long y = y1 - 1; y <= y1 + 1; y++)
      for (long x = x1 - 1; x <= x1 + 1; x++)
        cave[y][x].tl = false;
    ml__draw_block(y1, x1, y1, x1);
    light_flag = false;
  }

  for (long y = y2 - 1; y <= y2 + 1; y++) {
    chtype floor_str[82] = {0};
    chtype save_str[82] = {0};
    long floor_str_len = 0;
    long save_str_len = 0;
    long xpos = 0;
    chtype tmp_char;

    for (long x = x2 - 1; x <= x2 + 1; x++) {
      boolean flag = false;
      if (!(cave[y][x].fm || (cave[y][x].pl))) {
        tmp_char = ' ';
        if (player_light) {
          if (is_in(cave[y][x].fval, pwall_set)) {
            /* Turn on perm light */
            cave[y][x].pl = true;
            tmp_char = loc_symbol(y, x);
            flag = true;
          } else if (cave[y][x].tptr > 0 &&
                     is_in(t_list[cave[y][x].tptr].tval, light_set)) {
            /* Turn on field marker */
            cave[y][x].fm = true;
            tmp_char = loc_symbol(y, x);
            flag = true;
          }
        }
      } else {
        tmp_char = loc_symbol(y, x);
      }

      if (flag) {
        if (xpos == 0)
          xpos = x;
        if (save_str[0] != 0) {
          long i;
          for (i = 0; i < save_str_len; ++i)
            floor_str[floor_str_len++] = save_str[i];
          save_str[0] = 0;
          save_str_len = 0;
        }
        floor_str[floor_str_len++] = tmp_char;
      } else if (xpos > 0) {
        save_str[save_str_len++] = tmp_char;
      }
    } /* end for x */

    if (xpos > 0) {
      long const tmp_y = y;
      floor_str[floor_str_len] = 0;
      print_chstr(floor_str, tmp_y, xpos);
    }
  } /* end for y */

  LEAVE("ml__sub2_move_light", "m");
}

static void ml__sub3_move_light(long y1, long x1, long y2, long x2) {
  /*{ When blinded, move only the player symbol...              }*/

  ENTER(("ml__sub3_move_light", "%d, %d, %d, %d", y1, x1, y2, x1));

  if (light_flag) {
    long i1;
    for (i1 = y1 - 1; i1 <= y1 + 1; i1++) {
      long i2;
      for (i2 = x1 - 1; i2 <= x1 + 1; i2++) {
        cave[i1][i2].tl = false;
      }
    }
    light_flag = false;
  }
  print(' ', y1, x1);
  print('@', y2, x2);

  LEAVE("ml__sub3_move_light", "m");
}

static void ml__sub4_move_light(long y1, long x1, long y2, long x2) {
  /*{ With no light, movement becomes involved...               }*/

  ENTER(("ml__sub4_move_light", "%d, %d, %d, %d", y1, x1, y2, x2));

  light_flag = true;
  if (cave[y1][x1].tl) {
    for (long i1 = y1 - 1; i1 <= y1 + 1; i1++) {
      for (long i2 = x1 - 1; i2 <= x1 + 1; i2++) {
        cave[i1][i2].tl = false;
        if (test_light(i1, i2))
          lite_spot(i1, i2);
        else
          unlite_spot(i1, i2);
      }
    }
  } else if (test_light(y1, x1)) {
    lite_spot(y1, x1);
  } else {
    unlite_spot(y1, x1);
  }
  print('@', y2, x2);

  LEAVE("ml__sub4_move_light", "m");
}

static void d__jamdoor() {
  /*{ Jam a closed door                                     -RAK-   }*/

  treas_rec *i1;
  long y = char_row;
  long x = char_col;
  long i2;
  long tmp;
  char m_name[82];
  obj_set pick_a_spike = {spike, 0};

  if (!d__get_dir("Which direction?", &tmp, &tmp, &y, &x)) {
    return;
  }

  if (cave[y][x].tptr <= 0) {
    msg_print("That isn't a door!");
    return;
  }

  if (t_list[cave[y][x].tptr].tval == closed_door) {
    if (cave[y][x].cptr == 0) {
      if (find_range(pick_a_spike, false, &i1, &i2)) {
        msg_print("You jam the door with a spike.");
        if (i1->data.number > 1) {
          i1->data.number--;
        } else {
          inven_destroy(i1);
        }
        prt_stat_block();
        t_list[cave[y][x].tptr].p1 = -labs(t_list[cave[y][x].tptr].p1) - 20;
      } else {
        msg_print("But you have no spikes...");
      }
    } else {
      find_monster_name(m_name, cave[y][x].cptr, true);
      strcat(m_name, " is in your way!");
      msg_print(m_name);
    }
  } else if (t_list[cave[y][x].tptr].tval == open_door) {
    msg_print("The door must be closed first.");
  } else {
    msg_print("That isn't a door!");
  }
}

static void d__look() {
  /*{ Look at an object, trap, or monster                   -RAK-   }*/
  /*{ Note: Looking is a free move, see where invoked...            }*/

  long i1;
  long i2;
  long y;
  long x;
  long dir;
  long dummy;
  boolean flag = false;
  char out_val[82];
  char out_val2[120];

  y = char_row;
  x = char_col;

  if (!d__get_dir("Look which direction?", &dir, &dummy, &y, &x)) {
    return;
  }

  if (player_flags.blind >= 1) {
    msg_print("You can't see a damn thing!");
    return;
  }

  y = char_row;
  x = char_col;
  i1 = 0;
  do {
    move_dir(dir, &y, &x);
    /* with cave[y][x]. do; */
    if (cave[y][x].cptr > 1) {
      if (m_list[cave[y][x].cptr].ml) {
        i2 = m_list[cave[y][x].cptr].mptr;
        if (is_vowel(c_list[i2].name[0])) {
          sprintf(out_val, "You see an %s.", c_list[i2].name);
        } else {
          sprintf(out_val, "You see a %s.", c_list[i2].name);
        }
        msg_print(out_val);
        flag = true;
      }
    }

    if ((cave[y][x].tl) || (cave[y][x].pl) || (cave[y][x].fm)) {
      if (cave[y][x].tptr > 0) {
        if (t_list[cave[y][x].tptr].tval == secret_door) {
          msg_print("You see a granite wall.");
        } else if (t_list[cave[y][x].tptr].tval != unseen_trap) {
          inven_temp.data = t_list[cave[y][x].tptr];
          inven_temp.data.number = 1;
          objdes(out_val, &inven_temp, true);
          sprintf(out_val2, "You see %s.", out_val);
          msg_print(out_val2);
          flag = true;
        }
      }

      if (!cave[y][x].fopen) {
        flag = true;
        switch (cave[y][x].fval) {
        case 10:
          msg_print("You see a granite wall.");
          break;
        case 11:
          msg_print("You see some dark rock.");
          break;
        case 12:
          msg_print("You see a quartz vein.");
          break;
        case 15:
          msg_print("You see a granite wall.");
          break;
        default:
          break;
        }
      } else {
        switch (cave[y][x].fval) {
        case 16:
        case 17:
          flag = true;
          msg_print("You see some water.");
          break;
        default:
          break;
        }
      }
    }

    i1++;
  } while (!(((!cave[y][x].fopen) || (i1 > MAX_SIGHT))));

  if (!flag) {
    msg_print("You see nothing of interest in that direction.");
  }
}

static void d__set_coords(long *c_row, long *c_col) {
  /*{ Set up the character co-ords          }*/
  if ((*c_row == -1) || (*c_col == -1)) {
    do {
      *c_row = randint(cur_height);
      *c_col = randint(cur_width);

      /*      *c_row = 8;*/
      /*      *c_col = 20;*/
    } while (!((cave[*c_row][*c_col].fopen) &&
               (cave[*c_row][*c_col].cptr == 0) &&
               (cave[*c_row][*c_col].tptr == 0) &&
               (!(is_in(cave[*c_row][*c_col].fval, water_set)))));
  }
}

static void d__sun_rise_or_set() {
  long i1, i2;

  /*{ Sunrise and Sunset on town level	  -KRC-	}*/
  /* with player_cur_age do; */
  if (dun_level == 0) {
    if ((player_cur_age.hour == 6) && (player_cur_age.secs == 0)) {
      for (i1 = 1; i1 <= cur_height; i1++) {
        for (i2 = 1; i2 <= cur_width; i2++) {
          cave[i1][i2].pl = true;
        }
      }
      store_maint();
      draw_cave();
    } else if ((player_cur_age.hour == 18) && (player_cur_age.secs == 0)) {
      for (i1 = 1; i1 <= cur_height; i1++) {
        for (i2 = 1; i2 <= cur_width; i2++) {
          if (cave[i1][i2].fval != dopen_floor.ftval) {
            cave[i1][i2].pl = true;
          } else {
            cave[i1][i2].pl = false;
          }
        }
      }
      store_maint();
      draw_cave();
    }
  }
}

static void d__check_hours() {
  /*{ Check for game hours                          }*/
  if (turn % 100 != 1) return;
  if (!kick__should_kickout()) return;

    if (search_flag) {
      search_off();
    }
    if (player_flags.rest > 0) {
      rest_off();
    }
    find_flag = false;

    kick__kickout_player_if_time();
}

static void d__print_updated_stats() {
  if (print_stat != 0) {
    prt_stat_block();
  }
}

static void d__check_light_status() {
  /*{ Check light status                            }*/
  /* with equipment[Equipment_light] do; */
  ENTER(("d__check_light_status", "d"));
  if (player_light) {
    if ((equipment[Equipment_light].p1 > 0) && (player_flags).light_on) {
      equipment[Equipment_light].p1--;
      if (equipment[Equipment_light].p1 == 0) {
        msg_print("Your light has gone out!");
        (player_flags).light_on = false;
        player_light = false;
        find_flag = false;
        move_light(char_row, char_col, char_row, char_col);
      } else if (equipment[Equipment_light].p1 < 40) {
        if (randint(5) == 1) {
          if (find_flag) {
            find_flag = false;
            move_light(char_row, char_col, char_row, char_col);
          }
          msg_print("Your light is growing faint.");
        }
      }
    } else {
      (player_flags).light_on = false;
      player_light = false;
      find_flag = false;
      move_light(char_row, char_col, char_row, char_col);
    }
  } else if ((equipment[Equipment_light].p1 > 0) && (player_flags).light_on) {
    equipment[Equipment_light].p1--;
    player_light = true;
    move_light(char_row, char_col, char_row, char_col);
  }

  LEAVE("d__check_light_status", "d");
}

static void d__hunger_interrupt(char *message) {
  msg_print(message);
  msg_flag = 0;
  rest_off();
}

static void d__check_food() {
  /*{ Check food status             }*/
  regen_amount = PLAYER_REGEN_NORMAL;
  if (((player_flags).hunger_item) &&
      ((player_flags).foodc > (PLAYER_FOOD_ALERT + 15))) {
    (player_flags).foodc = PLAYER_FOOD_ALERT + 15;
  }

  if ((player_flags).foodc < PLAYER_FOOD_ALERT) {
    if ((player_flags).foodc < PLAYER_FOOD_WEAK) {

      if ((player_flags).foodc < 0) {
        regen_amount = 0;
      } else if ((player_flags).foodc < PLAYER_FOOD_FAINT) {
        regen_amount = PLAYER_REGEN_FAINT;
      } else if ((player_flags).foodc < PLAYER_FOOD_WEAK) {
        regen_amount = PLAYER_REGEN_WEAK;
      }

      if ((IS_WEAK & (player_flags).status) == 0) {
        (player_flags).status |= (IS_WEAK | IS_HUNGERY);
        d__hunger_interrupt("You are getting weak from hunger.");
        if (find_flag) {
          move_char(5);
        }
        prt_hunger();
        player_wt -= trunc(player_wt * 0.015);
        d__hunger_interrupt("Your clothes seem to be getting loose.");
        if (player_wt < min_allowable_weight()) {
          msg_print("Oh no...  Now you've done it.");
          death = true;
          moria_flag = true;
          total_winner = false;
          strcpy(died_from, "starvation");
        }
      }

      if ((player_flags).foodc < 0) {
        if (randint(5) == 1) {
          (player_flags).paralysis += randint(3);
          d__hunger_interrupt("You faint from the lack of food.");
          if (find_flag) {
            move_char(5);
          }
        } else if ((player_flags).foodc < PLAYER_FOOD_FAINT) {
          if (randint(8) == 1) {
            (player_flags).paralysis += randint(5);
            d__hunger_interrupt("You faint from the lack "
                                "of food.");
            if (find_flag) {
              move_char(5);
            }
          }
        }
      } /* end if (food < 0) */

    } else {
      /* alert, but not weak */
      if ((IS_HUNGERY & (player_flags).status) == 0) {
        (player_flags).status |= IS_HUNGERY;
        d__hunger_interrupt("You are getting hungry.");
        if (find_flag) {
          move_char(5);
        }
        prt_hunger();
      }
    }

  } /* end if (food < alert) */
}

static void d__eat_food() {
  /*{ Food consumtion       }*/
  /*{ Note: Speeded up characters really burn up the food!  }*/

  (player_flags).food_digested = BASE_FOOD_DIGESTED;

  if ((player_flags).status & IS_RESTING) {
    (player_flags).food_digested -= 1;
  }
  if ((player_flags).slow_digest) {
    (player_flags).food_digested -= 1;
  }
  if ((player_flags).status & IS_SEARCHING) {
    (player_flags).food_digested += 1;
  }
  if ((player_flags).regenerate) {
    (player_flags).food_digested += 3;
  }

  if ((player_flags).food_digested < 0) {
    (player_flags).food_digested = 0;
  }

  if ((player_flags).speed < 0) {
    (player_flags).foodc -= ((player_flags).speed * (player_flags).speed) +
                            (player_flags).food_digested;
  } else {
    (player_flags).foodc -= (player_flags).food_digested;
  }
}

static void d__regenerate() {
  /*{ Regenerate            }*/
  /* with player_do; */
  if ((player_flags).regenerate) {
    regen_amount *= 1.5;
  }
  if ((player_flags).rest > 0) {
    regen_amount *= 2;
  }
  if (player_flags.poisoned < 1) {
    if (C_player_current_hp() < C_player_max_hp()) {
      C_player_regen_hp(regen_amount);
    }
  }
  if (player_cmana < player_mana) {
    regenmana(regen_amount);
  }
}

static void d__update_blindness() {
  /*{ Blindness             }*/
  if ((player_flags).blind > 0) {
    if ((IS_BLIND & (player_flags).status) == 0) {
      (player_flags).status |= IS_BLIND;
      prt_map();
      prt_blind();
      if (search_flag) {
        search_off();
      }
    }
    (player_flags).blind--;
    if ((player_flags).blind == 0) {
      (player_flags).status &= ~IS_BLIND;
      prt_blind();
      prt_map();
      msg_print("The veil of darkness lifts.");
      move_char(5);
    }
  }
}

static void d__update_confusion() {
  /*{ Confusion             }*/
  if ((player_flags).confused > 0) {
    if ((IS_CONFUSED & (player_flags).status) == 0) {
      (player_flags).status |= IS_CONFUSED;
      prt_confused();
    }
    (player_flags).confused--;
    if ((player_flags).confused == 0) {
      (player_flags).status &= ~IS_CONFUSED;
      prt_confused();
      msg_print("You feel less confused now.");
      if (find_flag) {
        move_char(5);
      }
    }
  }
}

static void d__update_resist_lightning() {
  /*{ Resist Lightning }*/
  if ((player_flags).resist_lght > 0) {
    (player_flags).resist_lght--;
  }
}

static void d__update_monster_protect() {
  /*{ Protection from Monsters }*/
  if ((player_flags).protmon > 0) {
    (player_flags).protmon--;
  }
}

static void d__update_fire_ring() {
  /*{ Ring of Fire }*/
  if ((player_flags).ring_fire > 0) {
    msg_print("Flames arise!");
    explode(SE_FIRE, char_row, char_col, 20 + randint(20), "Ring of Fire");
    (player_flags).ring_fire--;
  }
}

static void d__update_frost_ring() {

  /*{ Ring of Frost }*/
  if ((player_flags).ring_ice > 0) {
    explode(SE_COLD, char_row, char_col, 10 + randint(20), "Ring of Frost");
    (player_flags).ring_ice--;
  }
}

static void d__update_blade_barrier() {

  /*{ Blade Barrier }*/
  if ((player_flags).blade_ring > 0) {
    explode(SE_NULL, char_row, char_col, 12 + randint(player_lev),
            "Blade Barrier");
    (player_flags).blade_ring--;
  }
}

static void d__update_magic_protect() {
  /*{ Magic protection }*/
  if ((player_flags).magic_prot > 0) {
    if ((IS_MAGIC_PROTECED & (player_flags).status) == 0) {
      (player_flags).status |= IS_MAGIC_PROTECED;
      player_save += 25;
    }
    (player_flags).magic_prot--;
    if ((player_flags).magic_prot == 0) {
      player_save -= 25;
      (player_flags).status &= ~IS_MAGIC_PROTECED;
    }
  }
}

static void d__update_resist_petrfy() {
  /*{Timed resist Petrification}*/
  if ((player_flags).resist_petri > 0) {
    (player_flags).resist_petri--;
  }
}

static void d__update_stealth() {
  /*{ Timed Stealth    }*/
  if ((player_flags).temp_stealth > 0) {
    if ((IS_STEALTHY & (player_flags).status) == 0) {
      (player_flags).status |= IS_STEALTHY;
      player_stl += 3;
    }
    (player_flags).temp_stealth--;
    if ((player_flags).temp_stealth == 0) {
      (player_flags).status &= ~IS_STEALTHY;
      player_stl -= 3;
      msg_print("The monsters can once again detect you with "
                "ease.");
    }
  }
}

static void d__update_resist_charm() {
  /*{ Resist Charm }*/
  if ((player_flags).free_time > 0) {
    if ((IS_CHARM_PROOF & (player_flags).status) == 0) {
      (player_flags).status |= IS_CHARM_PROOF;
      (player_flags).free_time--;
      if ((player_flags).free_time == 0) {
        (player_flags).status &= ~IS_CHARM_PROOF;
        if (find_flag) {
          move_char(5);
        }
      }
    }
  }
}

static void d__update_hoarse() {
  /*{ Hoarse                }*/
  if ((player_flags).hoarse > 0) {
    (player_flags).hoarse--;
    if ((player_flags).hoarse == 0) {
      msg_print("You feel your voice returning.");
    }
  }
}

static void d__update_fear() {
  /*{ Afraid                }*/
  if ((player_flags).afraid > 0) {
    if ((IS_AFRAID & (player_flags).status) == 0) {
      if (((player_flags).shero + (player_flags).hero) > 0) {
        (player_flags).afraid = 0;
      } else {
        (player_flags).status |= IS_AFRAID;
        prt_afraid();
      }
    } else if (((player_flags).shero + (player_flags).hero) > 0) {
      (player_flags).afraid = 1;
    }

    (player_flags).afraid--;
    if ((player_flags).afraid == 0) {
      (player_flags).status &= ~IS_AFRAID;
      prt_afraid();
      msg_print("You feel bolder now.");
      if (find_flag) {
        move_char(5);
      }
    }
  }
  if ((player_flags).afraid < 0) {
    (player_flags).afraid =
        0; /* fix when getting hit with fear while shero or hero */
  }
}

static void d__update_poison() {
  /*{ Poisoned              }*/
  if ((player_flags).poisoned > 0) {
    if ((IS_POISONED & (player_flags).status) == 0) {
      (player_flags).status |= IS_POISONED;
      prt_poisoned();
    }
    (player_flags).poisoned--;
    if ((player_flags).poisoned == 0) {
      (player_flags).status &= ~IS_POISONED;
      prt_poisoned();
      msg_print("You feel better.");
      if (find_flag) {
        move_char(5);
      }
    } else {
      switch (C_player_hp_from_con()) {
      case -4:
        take_hit(4, "poison");
        break;
      case -3:
      case -2:
        take_hit(3, "poison");
        break;
      case -1:
        take_hit(2, "poison");
        break;
      case 0:
        take_hit(1, "poison");
        break;
      case 1:
      case 2:
      case 3:
        if ((turn % 2) == 0) {
          take_hit(1, "poison");
        }
        break;
      case 4:
      case 5:
        if ((turn % 3) == 0) {
          take_hit(1, "poison");
        }
        break;
      case 6:
        if ((turn % 4) == 0) {
          take_hit(1, "poison.");
        }
        break;
      } /* end switch */
    }
  }
}

static void d__update_fast() {

  /*{ Fast                  }*/
  if ((player_flags).fast > 0) {
    if ((IS_HASTED & (player_flags).status) == 0) {
      (player_flags).status |= IS_HASTED;
      msg_print("You feel yourself moving faster.");
      change_speed(-1);
      if (find_flag) {
        move_char(5);
      }
    }
    (player_flags).fast--;
    if ((player_flags).fast == 0) {
      (player_flags).status &= ~IS_HASTED;
      msg_print("You feel yourself slow down.");
      change_speed(+1);
      if (find_flag) {
        move_char(5);
      }
    }
  }
}

static void d__update_slow() {

  /*{ Slow                  }*/
  if ((player_flags).slow > 0) {
    if ((IS_SLOW & (player_flags).status) == 0) {
      (player_flags).status |= IS_SLOW;
      msg_print("You feel yourself moving slower.");
      change_speed(+1);
      if (find_flag) {
        move_char(5);
      }
    }
    (player_flags).slow--;
    if ((player_flags).slow == 0) {
      (player_flags).status &= ~IS_SLOW;
      msg_print("You feel yourself speed up.");
      change_speed(-1);
      if (find_flag) {
        move_char(5);
      }
    }
  }
}

static void bother(long num) {
  if (num > 5) {
    msg_print("Your sword screams insults at passing monsters!");
  } else {
    msg_print("Your sword loudly shouts to all nearby creatures,");
    switch (num) {
    case 1:
      msg_print("What kinda monsters are you, mice -- or "
                "giant mice???");
      break;
    case 2:
      msg_print("You pathetic creatures are not worth "
                "tarnishing my blade!");
      break;
    case 3:
      msg_print("Attention all monsters:  SUPPERTIME!!!");
      break;
    case 4:
      msg_print("Boy are we wounded!! Sure hope we don't run "
                "into a kobold!");
      break;
    case 5:
      msg_print("Now where did I misplace my armor?  Hmmm...");
      break;
    }
  }

  if (aggravate_monster(20)) {
    msg_print("You hear the sounds of movement in the distance!");
  }

  msg_print(" ");
}

static void d__update_resting() {
  /*{ Resting is over?      }*/
  if ((player_flags).rest > 0) {
    /*{ Hibernate every 20 iterations so that process does  }*/
    /*{ not eat up system...                                }*/
    /*{ NOTE: Remove comments for VMS version 4.0 or greater}*/
    /*{       INKEY_DELAY takes care of hibernation for     }*/
    /*{       VMS 3.7 or less                               }*/
    if (((player_flags).rest % 20) == 1) {
      usleep(500);
      if ((equipment[Equipment_primary].flags2 & Soul_Sword_worn_bit) != 0) {
        bother(randint(10));
        (player_flags).rest = 1;
        (player_flags).resting_till_full = false;
      }
    }
    (player_flags).rest--;
    /*{ Test for any key being hit to abort rest.  Also,    }*/
    /*{ this will do a PUT_QIO which updates the screen...  }*/
    /*{ One more side benifit; since inkey_delay hibernates }*/
    /*{ small amount before executing, this makes resting   }*/
    /*{ less CPU intensive...                               }*/
    char command;
    inkey_delay(&command, 0);
    /*if (want_trap) { dump_ast_mess; XXXX}*/
    if ((player_flags).rest == 0) { /*{ Resting over          }*/
      if ((player_flags).resting_till_full &&
          (player_cmana < player_mana ||
           C_player_current_hp() < C_player_max_hp())) {
        (player_flags).rest = 20;
        turn_counter += (player_flags).rest;
      } else {
        rest_off();
      }
    } else if (command != 0) { /*{ Resting aborted       }*/
      rest_off();
    }
  }
}

static void d__update_hallucinate() {
  /*{ Hallucinating?  (Random characters appear!)}*/
  if ((player_flags).image > 0) {
    (player_flags).image--;
    if ((player_flags).image == 0) {
      draw_cave();
    }
  }
}

static void d__update_petrify() {
  /*{  Petrification wears off slowly  } */
  if ((turn % 100) == 0) {
    /* with player_flags do; */
    if ((player_flags).petrification > 100) {
      (player_flags).petrification--;
    }
  }

  /* not sure what this did, but it was commented out... */
  /*
  if ((player_flags).speed > 0) and (paral_init = speed_paral) then
                   paralysis := paralysis + paral_init + 1;
  */

  /*{ Paralysis             }*/
  if ((player_flags).paralysis > 0) {
    (player_flags).paralysis--;
    if ((player_flags).rest > 0) {
      rest_off();
    }
    if ((search_flag) &&
        ((player_flags).paralysis > (player_flags).paral_init)) {
      search_off();
    }
  }

  /* hey look!  more commented out code!! */
  /*
  if (speed > 0) and (speed_flag) then
                   begin
                   speed_flag := false;
  speed_paral := paral_init;
  end
    else if (speed_paral > 1) then
                                speed_paral := speed_paral - 1
                                else
                                  begin
                                    speed_paral := 0;
  speed_flag := true;
  end;
  */
}

static void d__update_evil_protect() {
  /*{ Protection from evil counter}*/
  if ((player_flags).protevil > 0) {
    (player_flags).protevil--;
  }
}

static void d__update_invulnerable() {
  /*{ Invulnerability        }*/
  if ((player_flags).invuln > 0) {
    if ((IS_INVULNERABLE & (player_flags).status) == 0) {
      (player_flags).status |= IS_INVULNERABLE;
      if (find_flag) {
        move_char(5);
      }
      msg_print("Your skin turns into steel!");
      player_pac += 100;
      player_dis_ac += 100;
    }
    (player_flags).invuln--;
    if ((player_flags).invuln == 0) {
      (player_flags).status &= ~IS_INVULNERABLE;
      if (find_flag) {
        move_char(5);
      }
      msg_print("Your skin returns to normal...");
      player_pac -= 100;
      player_dis_ac -= 100;
    }
    prt_stat_block();
  }
}

static void d__update_heroism() {
  /*{ Heroism       }*/
  if ((player_flags).hero > 0) {
    if ((IS_HERO & (player_flags).status) == 0) {
      (player_flags).status |= IS_HERO;
      if (find_flag) {
        move_char(5);
      }
      /* with player_do; */
      C_player_modify_max_hp(10);
      player_bth += 12;
      player_bthb += 12;
      msg_print("You feel like a HERO!");
      prt_stat_block();
    }
    (player_flags).hero--;
    if ((player_flags).hero == 0) {
      (player_flags).status &= ~IS_HERO;
      if (find_flag) {
        move_char(5);
      }
      /* with player_do; */
      C_player_modify_max_hp(-10);
      C_player_modify_current_hp(10);
      if (C_player_current_hp() > C_player_current_hp())
        C_player_reset_current_hp();
      player_bth -= 12;
      player_bthb -= 12;
      msg_print("The heroism wears off.");
      prt_stat_block();
    }
  }
}

static void d__update_super_heroism() {
  /*{ Super Heroism }*/
  if ((player_flags).shero > 0) {
    if ((IS_SUPER_HERO & (player_flags).status) == 0) {
      (player_flags).status |= IS_SUPER_HERO;
      if (find_flag) {
        move_char(5);
      }
      /* with player_do; */
      C_player_modify_max_hp(20);
      player_bth += 24;
      player_bthb += 24;
      msg_print("You feel like a SUPER HERO!");
      prt_stat_block();
    }
    (player_flags).shero--;
    if ((player_flags).shero == 0) {
      (player_flags).status &= ~IS_SUPER_HERO;
      if (find_flag) {
        move_char(5);
      }
      /* with player_do; */
      C_player_modify_max_hp(-20);
      C_player_modify_current_hp(20);
      if (C_player_current_hp() > C_player_current_hp())
        C_player_reset_current_hp();
      player_bth -= 24;
      player_bthb -= 24;
      msg_print("The super heroism wears off.");
      prt_stat_block();
    }
  }
}

static void d__update_blessed() {
  /*{ Blessed       }*/
  if ((player_flags).blessed > 0) {
    if ((IS_BLESSED & (player_flags).status) == 0) {
      (player_flags).status |= IS_BLESSED;
      if (find_flag) {
        move_char(5);
      }
      /* with player_do; */
      player_bth += 5;
      player_bthb += 5;
      player_pac += 5;
      player_dis_ac += 5;
      msg_print("You feel righteous!");
      prt_stat_block();
    }
    (player_flags).blessed--;
    if ((player_flags).blessed == 0) {
      (player_flags).status &= ~IS_BLESSED;
      if (find_flag) {
        move_char(5);
      }
      /* with player_do; */
      player_bth -= 5;
      player_bthb -= 5;
      player_pac -= 5;
      player_dis_ac -= 5;
      msg_print("The prayer has expired.");
      prt_stat_block();
    }
  }
}

static void d__update_resist_heat() {
  /*{ Resist Heat   }*/
  if ((player_flags).resist_heat > 0) {
    (player_flags).resist_heat--;
  }
}

static void d__update_resist_cold() {
  /*{ Resist Cold   }*/
  if ((player_flags).resist_cold > 0) {
    (player_flags).resist_cold--;
  }
}

static void d__update_detect_invisible() {
  /*{ Detect Invisible      }*/
  if ((player_flags).detect_inv > 0) {
    if ((IS_ABLE_TO_SEE_INVIS & (player_flags).status) == 0) {
      (player_flags).status |= IS_ABLE_TO_SEE_INVIS;
      (player_flags).see_inv = true;
    }
    (player_flags).detect_inv--;
    if ((player_flags).detect_inv == 0) {
      (player_flags).status &= ~IS_ABLE_TO_SEE_INVIS;
      (player_flags).see_inv = false;
      py_bonuses(&blank_treasure, 0);
    }
  }
}

static void d__update_infra_vision() {
  /*{ Timed infra-vision    }*/
  if ((player_flags).tim_infra > 0) {
    if ((IS_ABLE_TO_SEE_HEAT & (player_flags).status) == 0) {
      (player_flags).status |= IS_ABLE_TO_SEE_HEAT;
      (player_flags).see_infra++;
    }
    (player_flags).tim_infra--;
    if ((player_flags).tim_infra == 0) {
      (player_flags).status &= ~IS_ABLE_TO_SEE_HEAT;
      (player_flags).see_infra--;
      msg_print("Your eyes stop tingling.");
    }
  }
}

static void d__update_word_of_recall() {
  /*{ Word-of-Recall  Note: Word-of-Recall is a delayed action      }*/
  if ((player_flags).word_recall > 0) {
    if ((player_flags).word_recall == 1) {
      if (dun_level > 0) {
        msg_print("You feel yourself yanked upwards!");
        dun_level = 0;
      } else if (player_max_lev > 0) {
        msg_print("You feel yourself yanked downwards!");
        dun_level = player_max_lev;
      }
      moria_flag = true;
      (player_flags).paralysis++;
      (player_flags).word_recall = 0;
    } else {
      (player_flags).word_recall--;
    }
  }
}

static void d__update_hit_points() {
  /*{ Check hit points for adjusting...                     }*/
  /* with player_do; */
  ENTER(("d__update_hit_points", "d"));

  if (!(find_flag)) {
    if (player_flags.rest < 1) {
      if (old_chp != C_player_current_hp()) {
        if (C_player_current_hp() > C_player_max_hp())
          C_player_reset_current_hp();
        old_chp = trunc(C_player_current_hp());
      }
      if (old_cmana != trunc(player_cmana)) {
        if (player_cmana > player_mana) {
          player_cmana = player_mana;
        }
        old_cmana = trunc(player_cmana);
      }
    }
    prt_stat_block();
  }
  LEAVE("d__update_hit_points", "d");
}

static void d__go_up() {
  /*{ Go up one level                                       -RAK-   }*/
  /*{ Or several, with a steep staircase                    -DMF-   }*/

  /* with cave[char_row][char_col]. do; */
  if (cave[char_row][char_col].tptr > 0) {
    if (t_list[cave[char_row][char_col].tptr].tval == up_staircase) {
      dun_level--;
      moria_flag = true;
      msg_print("You enter a maze of up staircases.");
      msg_print("You pass through a one-way door.");
    } else if (t_list[cave[char_row][char_col].tptr].tval ==
               up_steep_staircase) {
      dun_level -= randint(3) + 1;
      if (dun_level < 0) {
        dun_level = 0;
      }
      moria_flag = true;
      msg_print("You enter a long maze of up staircases.");
      msg_print("You pass through a one-way door.");
    } else {
      msg_print("I see no up staircase here.");
    }
  } else {
    msg_print("I see no up staircase here.");
  }
}

static void d__go_down() {
  /*{ Go down one level                                     -RAK-   }*/
  /*{ Or several, with a steep staircase                    -DMF-   }*/

  /* with cave[char_row][char_col]. do; */
  if (cave[char_row][char_col].tptr > 0) {
    if (t_list[cave[char_row][char_col].tptr].tval == down_staircase) {
      dun_level++;
      moria_flag = true;
      msg_print("You enter a maze of down staircases.");
      msg_print("You pass through a one-way door.");
    } else if (t_list[cave[char_row][char_col].tptr].tval ==
               down_steep_staircase) {
      dun_level += randint(3) + 1;
      moria_flag = true;
      msg_print("You enter a long maze of down staircases.");
      msg_print("You pass through a one-way door.");
    } else {
      msg_print("I see no down staircase here.");
    }
  } else {
    msg_print("I see no down staircase here.");
  }
}

/**
 * -RAK-
 *  d__bash() - Bash open a door or chest
 * Note: Affected by strength and weight of character
 */
static void d__bash() {

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
          ((C_player_get_stat(STR) * 10) + 20) * 100;
      equipment[Equipment_primary].tval = 1;

      player_bth =
          trunc((((C_player_get_stat(STR) * 10) + 20) / 5 + player_wt) / 6.0);
      player_ptohit = 0;
      player_ptodam = trunc(player_wt / 75.0) + 1;

      if (py_attack(y, x)) {
        do_stun(cave[y][x].cptr, -10, 2);
      }

      /*{ Restore old values            }*/
      equipment[Equipment_primary] = inven_temp.data;
      player_ptohit = old_ptohit;
      player_ptodam = old_ptodam;
      player_bth = old_bth;
      if (randint(300) > (C_player_get_stat(DEX) * 10)) {
        msg_print("You are off-balance.");
        player_flags.paralysis = randint(3);
      }
    }
  } else if (cave[y][x].tptr > 0) {
    if (t_list[cave[y][x].tptr].tval == closed_door) {
      const int from_str = C_player_get_stat(STR) * 10;

      if (test_hit(player_wt + (from_str * from_str) / 500, 0, 0,
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

static void d__chest_trap(long y, long x) {
  /*{ Chests have traps too...                              -RAK-   }*/
  /*{ Note: Chest traps are based on the FLAGS value                }*/

  long i1, i2, i3;
  unsigned long flags;

  flags = t_list[cave[y][x].tptr].flags;

  /* with t_list[cave[y][x].tptr]. do; */

  if ((0x00000010 & flags) != 0) {
    msg_print("A small needle has pricked you!");
    if (lose_stat(STR, "", "You are unaffected.")) {
      take_hit(damroll("1d4"), "a poison needle");
      print_stat = 1;
      msg_print("You feel weakened!");
    }
  }

  if ((0x00000020 & flags) != 0) {
    msg_print("A small needle has pricked you!");
    take_hit(damroll("1d6"), "a poison needle.");
    player_flags.poisoned += 10 + randint(20);
  }

  if ((0x00000040 & flags) != 0) {
    msg_print("A puff of yellow gas surrounds you!");
    if (player_flags.free_act) {
      msg_print("You are unaffected.");
    } else {
      msg_print("You choke and pass out.");
      player_flags.paralysis = 10 + randint(20);
    }
  }

  if ((0x00000080 & flags) != 0) {
    msg_print("There is a sudden explosion!");
    delete_object(y, x);
    take_hit(damroll("5d8"), "an exploding chest");
  }

  if ((0x00000100 & flags) != 0) {
    for (i1 = 0; i1 < 3; i1++) {
      i2 = y;
      i3 = x;
      if (is_in(cave[i2][i3].fval, water_set)) {
        summon_water_monster(&i2, &i3, false);
      } else {
        summon_land_monster(&i2, &i3, false);
      }
    }
  }
}

static void d__openobject() {
  /*{ Opens a closed door or closed chest...                -RAK-   }*/

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
          d__chest_trap(y, x);
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

static void d__closeobject() {
  /*{ Closes an open door...                                -RAK-   }*/

  long y, x, tmp;
  char m_name[82];

  y = char_row;
  x = char_col;

  if (d__get_dir("Which direction?", &tmp, &tmp, &y, &x)) {
    /* with cave[y][x]. do; */
    if (cave[y][x].tptr > 0) {
      if (t_list[cave[y][x].tptr].tval == open_door) {
        if (cave[y][x].cptr == 0) {
          if (t_list[cave[y][x].tptr].p1 == 0) {
            t_list[cave[y][x].tptr] = door_list[1];
            cave[y][x].fopen = false;
            lite_spot(y, x);
          } else {
            msg_print("The door appears to "
                      "be broken.");
          }
        } else {
          find_monster_name(m_name, cave[y][x].cptr, true);
          strcat(m_name, " is in your way!");
          msg_print(m_name);
        }
      } else {
        msg_print("I do not see anything you can close "
                  "there.");
      }
    } else {
      msg_print("I do not see anything you can close there.");
    }
  }
}

static void d__disarm_trap() {
  /*{ Disarms a trap                                        -RAK-   }*/

  long y, x, i1, tdir;
  long tot, t1, t2, t3, t4, t5;
  char *tmpc;

  y = char_row;
  x = char_col;

  if (d__get_dir("Which direction?", &tdir, &i1, &y, &x)) {
    /* with cave[y][x]. do; */
    if (cave[y][x].tptr > 0) {
      t1 = player_disarm;                  /*{ Ability to disarm     }*/
      t2 = player_lev;                     /*{ Level adjustment      }*/
      t3 = 2 * C_player_disarm_from_dex(); /*{ Dexterity
                                              adjustment  }*/
      t4 = C_player_mod_from_stat(INT);    /*{ Intelligence adjustment}*/
      tot = t1 + t2 + t3 + t4;

      if (player_flags.blind > 0) {
        tot /= 5;
      } else if (no_light()) {
        tot /= 2;
      }

      if (player_flags.confused > 0) {
        tot /= 3;
      }

      i1 = t_list[cave[y][x].tptr].tval;
      t5 = t_list[cave[y][x].tptr].level;

      if (i1 == seen_trap) { /* { Floor trap    } */
        /* with t_list[cave[y][x].tptr]. do; */
        if ((tot - t5) > randint(100)) {
          msg_print("You have disarmed the trap.");
          C_player_add_exp(t_list[cave[y][x].tptr].p1);
          cave[y][x].fm = false;
          pusht(cave[y][x].tptr);
          cave[y][x].tptr = 0;
          move_char(tdir);
          lite_spot(y, x);
          prt_stat_block();
        } else if (randint(tot) > 5) {
          msg_print("You failed to disarm the trap.");
        } else {
          msg_print("You set the trap off!");
          move_char(tdir);
        }
      } else if (i1 == 2) { /*{ Chest trap    }*/
        /* with t_list[cave[y][x].tptr]. do; */
        if (strstr(t_list[cave[y][x].tptr].name, "^") != NULL) {
          msg_print("I don't see a trap...");
        } else if ((0x000001F0 & t_list[cave[y][x].tptr].flags) != 0) {
          if ((tot - t5) > randint(100)) {
            t_list[cave[y][x].tptr].flags &= 0xFFFFFE0F;
            tmpc = strstr(t_list[cave[y][x].tptr].name, " (");
            if (tmpc != NULL) {
              *tmpc = 0;
            }
            if ((0x00000001 & t_list[cave[y][x].tptr].flags) != 0) {
              strcat(t_list[cave[y][x].tptr].name, " (Locked)");
            } else {
              strcat(t_list[cave[y][x].tptr].name, " (Disarmed)");
            }
            msg_print("You have disarmed "
                      "the chest.");
            known2(t_list[cave[y][x].tptr].name);
            C_player_add_exp(t5);
            prt_stat_block();
          } else if (randint(tot) > 5) {
            msg_print("You failed to "
                      "disarm the chest.");
          } else {
            msg_print("You set a trap off!");
            known2(t_list[cave[y][x].tptr].name);
            d__chest_trap(y, x);
          }
        } else {
          msg_print("The chest was not trapped.");
        }
      } else {
        msg_print("I do not see anything to disarm there.");
      }
    } else {
      msg_print("I do not see anything to disarm there.");
    }
  }
}

static void d__refill_lamp() {
  /*{ Refill the players lamp                               -RAK-   }*/

  long i2, i3;
  treas_rec *i1;
  obj_set this_be_oil = {flask_of_oil, 0};

  i3 = equipment[Equipment_light].subval;

  if ((i3 > 0) && (i3 < 10)) {
    if (find_range(this_be_oil, false, &i1, &i2)) {
      msg_print("Your lamp is full.");
      /* with equipment[Equipment_light]. do; */
      equipment[Equipment_light].p1 += i1->data.p1;
      if (equipment[Equipment_light].p1 > OBJ_LAMP_MAX) {
        equipment[Equipment_light].p1 = OBJ_LAMP_MAX;
      }
      desc_remain(i1);
      inven_destroy(i1);
      prt_stat_block();
    } else {
      msg_print("You have no oil.");
    }
  } else {
    msg_print("But you are not using a lamp.");
  }
}

static void d__tunnel() {
  /*{ Tunnels through rubble and walls                      -RAK-   }*/
  /*{ Must take into account: secret doors, special tools           }*/

  long y, x, i1, tabil;

  y = char_row;
  x = char_col;

  if (d__get_dir("Which direction?", &i1, &i1, &y, &x)) {
    /* with cave[y][x]. do; */

    /*{ Compute the digging ability of player; based on       }*/
    /*{ strength, and type of tool used                       }*/
    tabil = ((C_player_get_stat(STR) * 10) + 20) / 5;
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
          search(char_row, char_col, C_player_curr_search_skill());
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

static void d__drop() {
  /*{ Drop an object being carried                          -RAK-   }*/
  /*{ Note: Only one object per floor spot...                       }*/

  treas_rec *com_ptr;
  boolean redraw;
  char trash_char;
  char out_val[82];
  char out_val2[120];
  long temp;
  long count;

  reset_flag = true;

  /* with player_do; */
  temp = (player_money[6] + player_money[5] + player_money[4] +
          player_money[3] + player_money[2] + player_money[1]);

  if ((inven_ctr > 0) || (temp > 0)) {
    count = change_all_ok_stats(true, false);
    com_ptr = inventory_list;
    for (; com_ptr != NULL;) {
      if ((com_ptr->data.tval == bag_or_sack) && (com_ptr->insides != 0)) {
        com_ptr->ok = false;
        count--;
      }
      com_ptr = com_ptr->next;
    }

    /*{Someone said that it always redraws when drop}*/
    redraw = false;

    if (get_item(&com_ptr, "Which one? ", &redraw, count, &trash_char, true,
                 false)) {
      if (redraw) {
        draw_cave();
      }
      /* with cave[char_row][char_col]. do; */
      if (cave[char_row][char_col].tptr > 0) {
        msg_print("There is something there already.");
      } else {
        if (trash_char == '$') {
          inven_drop(com_ptr, char_row, char_col, true);
        } else {
          inven_drop(com_ptr, char_row, char_col, false);
        }
        prt_stat_block();
        objdes(out_val, &inven_temp, true);
        sprintf(out_val2, "Dropped %s.", out_val);
        msg_print(out_val2);
        reset_flag = false;
      }
    } else if (redraw) {
      draw_cave();
    }
  } else {
    msg_print("You are not carrying anything.");
  }
}

static void rest() {
  /*{ Resting allows a player to safely restore his hp      -RAK-   }*/

  long rest_num;
  char rest_str[82];

  prt("Rest for how long (or *) ? ", 1, 1);
  get_string(rest_str, 1, 28, 10);

  if (!strcmp(rest_str, "*")) {
    rest_num = 20;
    (player_flags).resting_till_full = true;
  } else {
    rest_num = 0;
    sscanf(rest_str, "%ld", &rest_num);
  }

  if (rest_num > 0) {
    if (search_flag) {
      search_off();
    }
    player_flags.rest = rest_num;
    turn_counter += rest_num;
    player_flags.status |= IS_RESTING;
    prt_rest();
    msg_print("Press any key to wake up...");
    refresh();
  } else {
    erase_line(msg_line, msg_line);
    reset_flag = true;
  }
}

static void d__execute_command(long *command) {
  treas_rec *trash_ptr;
  char out_val[82];
  char out2[82];

  ENTER(("d__execute_command", "%d, '%c'", *command, *command));

  switch (*command) {

    /* case   1  :     ^A = Cure all     W1 */
    /* case   2  :     ^B = objects      W1 */
    /* case   4  :     ^D = up/down      W1 */
    /* case   5  :     ^E = wizchar      W2 */
    /* case   6  :     ^F = genocide     W2 */
    /* case   7  :     ^G = treasure     W2 */
    /* case   8  :     ^H = wizhelp      W1 */
    /* case   9  :     ^I = identify     W1 */
    /* case  10  :     ^J = gain exp     W2 */
    /* case  11  :     ^K = summon       W2 */
    /* case  12  :     ^L = wizlight     W1 */
    /* case  14  :     ^N = mon map      W1 */
    /* case  15  :     ^O = summon       W2 */
    /* case  20  :     ^T = teleport     W1 */
    /* case  22  :     ^V = restore      W1 */
    /* case  21  :     ^U = summon       W2 */
    /* case  23  :     ^W = create       W2 */
    /* case  24  :     ^X = ed score     W2 */
    /* case  27  :     ^3 = store maint  W2 */
    /* case  31  :     ^_                W1 */

  case 0:      /* \0 */
  case CTRL_C: /* ^C signalquit in io.c handles this one, it calls d__quit
                */
  case '@':
    d__quit();
    reset_flag = true;
    break;

  case CTRL_A:
    reset_flag = C_select_ability();
    draw_cave();
    break;

  case CTRL_B:
    find_flag = true;
    move_char(1);
    break;

  case CTRL_H:
    find_flag = true;
    move_char(4);
    break;
  case CTRL_I:
    blow();
    break;
  case CTRL_J:
    find_flag = true;
    move_char(2);
    break;
  case CTRL_K:
    find_flag = true;
    move_char(8);
    break;
  case CTRL_L:
    find_flag = true;
    move_char(6);
    break;

  case CTRL_N:
    find_flag = true;
    move_char(3);
    break;

  case CTRL_P:
    msg_print(old_msg);
    reset_flag = true;
    break;

  case CTRL_R: /* redraw */
    draw_cave();
    reset_flag = true;
    break;
  case CTRL_S:
    d__jamdoor();
    break;

  case CTRL_U:
    find_flag = true;
    move_char(9);
    break;

  case CTRL_W: /* Password */
    if (!wizard1)
      enter_wizard_mode(true);
    else
      wizard_command();
    reset_flag = true;
    break;

  case CTRL_X:
    d__look();
    reset_flag = true;
    break;
  case CTRL_Y:
    find_flag = true;
    move_char(7);
    break;
  case CTRL_Z: /* suspend  (we never get this, look at signalsuspend) */
    reset_flag = true;
    break;

  case 27: /* ALT */
    *command = inkey();
    MSG(("command: %d '%c'\n", (int)*command, (int)*command));
    switch (*command) {
    case 'a': /* Armor help */
      moria_help("Adventuring Armor_Class Armor_List");
      draw_cave();
      reset_flag = true;
      break;
    case 'b':
      move_char(1);
      break;
    case 'c':
      C_commands_show_class_restrictions();
      draw_cave();
      break;
    case 'd':
      sprintf(out_val, "The date is %s",
              full_date_string(player_cur_age, out2));
      msg_print(out_val);
      reset_flag = true;
      break;
    case 'e':
      sprintf(out_val, "Character Classes Experience %4.2f", player_expfact);
      moria_help(out_val);
      draw_cave();
      reset_flag = true;
      break;

    case 'h':
      move_char(4);
      break;

    case 'j':
      move_char(2);
      break;
    case 'k':
      move_char(8);
      break;
    case 'l':
      move_char(6);
      break;
    case 'm':
      moria_help("");
      draw_cave();
      reset_flag = true;
      break;
    case 'n':
      move_char(3);
      break;

    case 'r':
      msg_terse = !msg_terse;
      if (msg_terse) {
        msg_print("Question '-More-' toggled off");
        msg_terse = true; /* try to only use true and false */
      } else {
        msg_print("Question '-More-' toggled on");
        msg_terse = false;
      }
      reset_flag = true;
      break;
    case 's': /* Save and quit */
      if (total_winner) {
        msg_print("You are a Total Winner, your "
                  "character must "
                  "be retired...");
        msg_print("Use '@' when you are ready to quit.");
      } else {
        if (search_flag)
          search_off();
        if (sav__save_character())
          exit_game();
      }
      reset_flag = true;
      break;
    case 't':
      sprintf(out_val, "The current time is %s", show_current_time(out2));
      msg_print(out_val);
      reset_flag = true;
      break;
    case 'u':
      move_char(9);
      break;

    case 'w':
      moria_help("Adventuring Weapons Weapon_List");
      draw_cave();
      reset_flag = true;
      break;

    case 'y':
      move_char(7);
      break;
    }
    break;

  case '/': /* identify */
    ident_char();
    reset_flag = true;
    break;

  case '<':
    d__go_up();
    break;
  case '>':
    d__go_down();
    break;

  case '?': /* help */
    help();
    reset_flag = true;
    break;

  case '.': /* Rest one turn */
    move_char(5);
    usleep(10);
    flush();
    break;

  case 'A':
    throw_something();
    break;
  case 'B':
    find_flag = true;
    move_char(1);
    break;
  case 'C': /* Show character */
    change_name();
    draw_cave();
    reset_flag = true;
    break;
  case 'D':
    d__disarm_trap();
    break;
  case 'E':
    eat();
    break;
  case 'F':
    d__refill_lamp();
    break;

  case 'H':
    find_flag = true;
    move_char(4);
    break;
  case 'I': /* Selected inv */
    reset_flag = true;
    if (inven_command('I', &trash_ptr, "")) {
      draw_cave();
    }
    break;
  case 'J':
    find_flag = true;
    move_char(2);
    break;
  case 'K':
    find_flag = true;
    move_char(8);
    break;
  case 'L':
    find_flag = true;
    move_char(6);
    break;
  case 'N':
    find_flag = true;
    move_char(3);
    break;

  case 'P':
    C_print_known_spells();
    draw_cave();
    break;
  case 'Q':
    if (player_flags.quested) {
      sprintf(out_val, "Current quest is to kill a %s",
              c_list[player_cur_quest].name);
      msg_print(out_val);
    } else {
      msg_print("No quest currently.");
    }
    reset_flag = true;
    break;
  case 'R':
    rest();
    break;
  case 'S': /* Search mode */
    if (search_flag) {
      search_off();
      reset_flag = true;
    } else if (player_flags.blind > 0) {
      msg_print("You are incapable of searching while blind.");
    } else {
      search_on();
      reset_flag = true;
    }
    break;
  case 'T':
    d__tunnel();
    break;
  case 'U':
    find_flag = true;
    move_char(9);
    break;
  case 'V':
    msg_record("", false);
    reset_flag = true;
    break;

  case 'X': /* Toggle light source */
    reset_flag = true;
    if (equipment[Equipment_light].tval > 0) {
      if (equipment[Equipment_light].p1 > 0) {
        if ((player_flags).light_on) {
          sprintf(out_val, "Light Off.  %ld turns left.",
                  equipment[Equipment_light].p1);
          (player_flags).light_on = false;
          player_light = false;
        } else {
          sprintf(out_val, "Light On.  %ld turns left.",
                  equipment[Equipment_light].p1);
          (player_flags).light_on = true;
          player_light = true;
        }
        prt_light_on();
        msg_print(out_val);
        move_light(char_row, char_col, char_row, char_col);
      } else {
        msg_print("Your light has gone out!");
      }
    } else {
      msg_print("You are not carrying a light.");
    }
    break;
  case 'Y':
    find_flag = true;
    move_char(7);
    break;
  case 'Z':
    use_staff();
    break;

  case 'a':
    shoot_something();
    break;
  case 'b':
    move_char(1);
    break;
  case 'c':
    d__closeobject();
    break;
  case 'd':
    d__drop();
    break;
  case 'f':
    d__bash();
    break;
  case 'h':
    move_char(4);
    break;
  case 'i': /* Inventory */
    reset_flag = true;
    if (inven_command('i', &trash_ptr, "")) {
      draw_cave();
    }
    break;
  case 'j':
    move_char(2);
    break;
  case 'k':
    move_char(8);
    break;
  case 'l':
    move_char(6);
    break;
  case 'm': /* magick, monk, music */
    if (C_player_uses_magic(M_NATURE)) {
      cast(M_NATURE); /* play */
    } else if (C_player_uses_magic(M_ARCANE)) {
      cast(M_ARCANE); /*  magick   } */
    } else if (C_player_uses_magic(M_CHAKRA)) {
      cast(M_CHAKRA); /* m = monk? :) */
    } else {
      cast(M_SONG); /* music */
    }
    break;
  case 'n':
    move_char(3);
    break;
  case 'o':
    d__openobject();
    break;
  case 'p': /* pray */
    if (C_player_uses_magic(M_DIVINE)) {
      cast(M_DIVINE);
    } else {
      msg_print("You pray for a moment");
    }
    break;
  case 'q':
    quaff();
    break;
  case 'r':
    read_scroll();
    break;
  case 's': /* Search */
    if (player_flags.blind > 0) {
      msg_print("You are incapable of searching while blind.");
    } else {
      search(char_row, char_col, C_player_curr_search_skill());
    }
    break;
  case 't': /* take off */
    reset_flag = true;
    if (inven_command('t', &trash_ptr, "")) {
      draw_cave();
    }
    break;
  case 'u':
    move_char(9);
    break;
  case 'v': /* version */
    reset_flag = true;
    game_version();
    break;
  case 'w': /* wear */
    reset_flag = true;
    if (inven_command('w', &trash_ptr, "")) {
      draw_cave();
    } else {
      prt_stat_block();
    }
    break;
  case 'x': /* exchange weapon */
    reset_flag = true;
    if (inven_command('x', &trash_ptr, "")) {
      draw_cave();
    }
    break;
  case 'y':
    move_char(7);
    break;
  case 'z':
    aim_wand();
    break;
  default:
    reset_flag = true;
    prt("Type '?' for help...", 1, 1);
    break;

  }

  LEAVE("d__execute_command", "d");
}

boolean coin_stuff(char typ, long *type_num) {
  boolean return_value;

  return_value = true;
  switch (typ) {
  case 'm':
    *type_num = MITHRIL;
    break;
  case 'p':
    *type_num = PLATINUM;
    break;
  case 'g':
    *type_num = GOLD;
    break;
  case 's':
    *type_num = SILVER;
    break;
  case 'c':
    *type_num = COPPER;
    break;
  case 'i':
    *type_num = IRON;
    break;

  default:
    return_value = false;
    break;
  }
  return return_value;
}

long get_money_type(char prompt[134], boolean *back, boolean no_check) {
  boolean comma_flag = false;
  boolean test_flag = false;
  char out_val[134];
  long com_val;

  strncpy(out_val, prompt, sizeof(char[134]));

  if ((player_money[6] > 0) || (no_check))
    s__get_money_type__prompt_money("<m>ithril", out_val, &comma_flag);
  if ((player_money[5] > 0) || (no_check))
    s__get_money_type__prompt_money("<p>latinum", out_val, &comma_flag);
  if ((player_money[4] > 0) || (no_check))
    s__get_money_type__prompt_money("<g>old", out_val, &comma_flag);
  if ((player_money[3] > 0) || (no_check))
    s__get_money_type__prompt_money("<s>ilver", out_val, &comma_flag);
  if ((player_money[2] > 0) || (no_check))
    s__get_money_type__prompt_money("<c>opper", out_val, &comma_flag);
  if ((player_money[1] > 0) || (no_check))
    s__get_money_type__prompt_money("<i>ron", out_val, &comma_flag);

  prt(out_val, 1, 1);
  *back = true;

  do {
    char command = inkey();
    com_val = (long)(command);
    switch (com_val) {
    case 0:
    case 3:
    case 25:
    case 26:
    case 27:
      test_flag = true;
      *back = false;
      break;
    case 109:
      test_flag = ((player_money[MITHRIL] > 0) || (no_check));
      break;
    case 112:
      test_flag = ((player_money[PLATINUM] > 0) || (no_check));
      break;
    case 103:
      test_flag = ((player_money[GOLD] > 0) || (no_check));
      break;
    case 115:
      test_flag = ((player_money[SILVER] > 0) || (no_check));
      break;
    case 99:
      test_flag = ((player_money[COPPER] > 0) || (no_check));
      break;
    case 105:
      test_flag = ((player_money[IRON] > 0) || (no_check));
      break;
    } /* end switch */
  } while (!test_flag);

  return com_val;
}

void py_bonuses(treasure_type *tobj, long factor) {
  /*
        { Player bonuses					-RAK-	}
        { When an item is worn or taken off, this re-adjusts the player }
        { bonuses.  Factor=1 : wear; Factor=-1 : removed                }
  */

  unsigned long item_flags, item_flags2;
  long i1, old_dis_ac;
  enum stat_t tstat;

  (player_flags).see_inv = false;
  (player_flags).teleport = false;
  (player_flags).free_act = false;
  (player_flags).slow_digest = false;
  (player_flags).aggravate = false;
  for (tstat = STR; tstat <= CHR; tstat++) {
    (player_flags).sustain[(int)tstat] = false;
  }
  (player_flags).fire_resist = false;
  (player_flags).hunger_item = false;
  (player_flags).acid_resist = false;
  (player_flags).cold_resist = false;
  (player_flags).regenerate = false;
  (player_flags).lght_resist = false;
  (player_flags).ffall = false;

  if ((Strength_worn_bit & tobj->flags) != 0) {
    C_player_mod_stat(STR, tobj->p1 * factor);
    print_stat = 1;
  }
  if ((Dexterity_worn_bit & tobj->flags) != 0) {
    C_player_mod_stat(DEX, tobj->p1 * factor);
    print_stat = 1;
  }
  if ((Constitution_worn_bit & tobj->flags) != 0) {
    C_player_mod_stat(CON, tobj->p1 * factor);
    print_stat = 1;
  }
  if ((Intelligence_worn_bit & tobj->flags) != 0) {
    C_player_mod_stat(INT, tobj->p1 * factor);
    print_stat = 1;
  }
  if ((Wisdom_worn_bit & tobj->flags) != 0) {
    C_player_mod_stat(WIS, tobj->p1 * factor);
    print_stat = 1;
  }
  if ((Charisma_worn_bit & tobj->flags) != 0) {
    C_player_mod_stat(CHR, tobj->p1 * factor);
    print_stat = 1;
  }
  C_player_recalc_stats();
  if ((Magic_proof_worn_bit & tobj->flags2) != 0) {
    player_save += (25 * factor);
  }
  if ((Bad_repute_worn_bit & tobj->flags2) != 0) {
    change_rep(-100 * factor); /*{XXX hey!  this is bad! new variable!-ste}*/
  }
  if ((Disarm_worn_bit & tobj->flags2) != 0) {
    player_disarm += (tobj->p1 * factor);
  }
  if ((Searching_worn_bit & tobj->flags) != 0) {
    C_player_mod_search_skill(tobj->p1 * factor);
    player_fos -= (tobj->p1 * factor);
  }
  if ((Stealth_worn_bit & tobj->flags) != 0) {
    player_stl += (tobj->p1 * factor) + factor;
  }
  if ((Speed_worn_bit & tobj->flags) != 0) {
    i1 = tobj->p1 * factor;
    change_speed(-i1);
  }
  if ((Blindness_worn_bit & tobj->flags) != 0) {
    if (factor > 0) {
      player_flags.blind += 1000;
    }
  }
  if ((Timidness_worn_bit & tobj->flags) != 0) {
    if (factor > 0) {
      player_flags.afraid += 50;
    }
  }
  if ((Infra_Vision_worn_bit & tobj->flags) != 0) {
    player_flags.see_infra += (tobj->p1 * factor);
  }
  /* This has no effect, what was it supposed to do?
  if ((Swimming_worn_bit & tobj->flags2) != 0) {
          i1 = tobj->p1 * factor;
  }
  */
  if ((Increase_carry_worn_bit & tobj->flags2) != 0) {
    switch (tobj->p1) {
    case 1:
      i1 = 500;
      break;
    case 2:
      i1 = 1000;
      break;
    case 3:
      i1 = 1750;
      break;
    case 4:
      i1 = 2500;
      break;
    case 5:
      i1 = 3500;
      break;
    case 6:
      i1 = 4500;
      break;
    case 7:
      i1 = 6000;
      break;
    default:
      MSG(("Increase carry worn value (p1) out of range"));
      i1 = 500;
      break;
    }
    C_player_set_extra_bulk_carry(i1 * factor);
  }

  /* with player_do; */
  old_dis_ac = player_dis_ac;
  player_ptohit = C_player_tohit_from_stats();
  player_ptodam = C_player_dmg_from_str();
  player_ptoac = C_player_ac_from_dex();
  player_pac = 0;                /*{ Real AC       } */
  player_dis_th = player_ptohit; /*{ Display To Hit        } */
  player_dis_td = player_ptodam; /*{ Display To Dam        } */
  player_dis_ac = 0;             /*{ Display To AC         } */
  player_dis_tac = player_ptoac; /*{ Display AC            } */

  for (i1 = Equipment_min; i1 <= EQUIP_MAX - 2; i1++) {
    /* with equipment[i1] do; */
    if (equipment[i1].tval > 0) {
      if ((Cursed_worn_bit & equipment[i1].flags) == 0) {
        player_pac += equipment[i1].ac;
        player_dis_ac += equipment[i1].ac;
      }
      player_ptohit += equipment[i1].tohit;
      player_ptodam += equipment[i1].todam;
      player_ptoac += equipment[i1].toac;
      if (strstr(equipment[i1].name, "^") == NULL) {
        player_dis_th += equipment[i1].tohit;
        player_dis_td += equipment[i1].todam;
        player_dis_tac += equipment[i1].toac;
      }
    }
  }
  player_dis_ac += player_dis_tac;

  /* { Add in temporary spell increases	}*/
  /* with player_flags do; */
  if ((player_flags).invuln > 0) {
    player_pac += 100;
    player_dis_ac += 100;
  }
  if ((player_flags).blessed > 0) {
    player_pac += 5;
    player_dis_ac += 5;
  }
  if ((player_flags).detect_inv > 0) {
    (player_flags).see_inv =
        true; /* does this mean that if you put on/take off stuff
                 you are going to lose magic detect_inv ? */
  }

  if (old_dis_ac != player_dis_ac) {
    print_stat = 1;
  }
  item_flags2 = 0;
  item_flags = 0;

  for (i1 = Equipment_min; i1 <= EQUIP_MAX - 2; i1++) {
    /* with equipment[i1] do; */
    item_flags = (item_flags | equipment[i1].flags);
    item_flags2 = (item_flags2 | equipment[i1].flags2);
  }

  /* with player_flags do; */
  (player_flags).slow_digest = (Slow_Digestion_worn_bit & item_flags) != 0;
  (player_flags).aggravate = (Aggravation_worn_bit & item_flags) != 0;
  (player_flags).teleport = (Teleportation_worn_bit & item_flags) != 0;
  (player_flags).regenerate = (Regeneration_worn_bit & item_flags) != 0;
  (player_flags).hunger_item = (Hunger_worn_bit & item_flags2) != 0;
  (player_flags).fire_resist = (Resist_Fire_worn_bit & item_flags) != 0;
  (player_flags).acid_resist = (Resist_Acid_worn_bit & item_flags) != 0;
  (player_flags).cold_resist = (Resist_Cold_worn_bit & item_flags) != 0;
  (player_flags).free_act = (Free_Action_worn_bit & item_flags) != 0;
  (player_flags).see_inv |= (See_Invisible_worn_bit & item_flags) != 0;
  (player_flags).lght_resist = (Resist_Lightning_worn_bit & item_flags) != 0;
  (player_flags).ffall = (Feather_Fall_worn_bit & item_flags) != 0;

  for (i1 = Equipment_min; i1 <= EQUIP_MAX - 2; i1++) {
    /* with equipment[i1] do; */
    if ((Sustain_Stat_worn_bit & equipment[i1].flags) != 0) {
      if ((equipment[i1].p1 > 0) && (equipment[i1].p1 < 7)) {
        player_flags.sustain[equipment[i1].p1 - 1] = true;
      }
    }
  }
}

void change_speed(long num) {
  /*
    { Changes speed of monsters relative to player		-RAK-
    }
    { Note: When the player is sped up or slowed down, I simply     }
    {       change the speed of all the monsters.  This greatly     }
    {       simplified the logic...                                 }
  */
  long i1;

  player_flags.speed += num;

  for (i1 = muptr; i1 != 0; i1 = m_list[i1].nptr) {
    m_list[i1].cspeed += num;
  }
}

void change_rep(long amt) {
  /* with player_do; */
  if ((amt < 0) || (player_rep + amt <= 0)) { /*{bad deed or make up for sins}*/
    player_rep += amt;
  } else { /*{ good deed that puts char into positive reputation }*/
           /*{ good characters progress slowly -- past 0 it costs 2, past
            * 20 costs 3...}*/
    if (player_rep < 0) { /*{ go from bad to good }*/
      amt += player_rep;
      player_rep = 0;
    } /*{increase goodness}*/
    player_rep =
        trunc(sqrt((20 + player_rep) * (20 + player_rep) + 40 * amt) - 20);
  }
}

boolean panel_contains(long y, long x) {
  /*{ Tests a given point to see if it is within the screen -RAK-   }*/
  /*{ boundries.                                                    }*/

  boolean return_value = false;

  if ((y >= panel_row_min) && (y <= panel_row_max)) {
    if ((x >= panel_col_min) && (x <= panel_col_max)) {
      return_value = true;
    }
  }

  return return_value;
}

void move_light(long y1, long x1, long y2, long x2) {
  /*{ Package for moving the character's light about the screen     }*/
  /*{ Three cases : Normal, Finding, and Blind              -RAK-   }*/
  /* (so what is that sub4 thing?) */

  if (player_flags.blind > 0) {
    ml__sub3_move_light(y1, x1, y2, x2); /* blind */
  } else if (find_flag) {
    ml__sub2_move_light(y1, x1, y2, x2); /* searching */
  } else if (!player_light) {
    ml__sub4_move_light(y1, x1, y2, x2); /* no light */
  } else {
    ml__sub1_move_light(y1, x1, y2, x2); /* normal */
  }
}

void lite_spot(long y, long x) {
  if (panel_contains(y, x)) {
    print(loc_symbol(y, x), y, x);
  }
}

void unlite_spot(long y, long x) {
  if (panel_contains(y, x)) {
    print(' ', y, x);
  }
}

void teleport(long dis) {
  /*{ Teleport the player to a new location                 -RAK-   }*/

  long y, x, i1, i2;

  ENTER(("teleport", "%d", dis));

  do {
    y = randint(cur_height);
    x = randint(cur_width);
    for (; distance(y, x, char_row, char_col) > dis;) {
      y += trunc((char_row - y) / 2);
      x += trunc((char_col - x) / 2);
    }
  } while (!((cave[y][x].fopen) && (cave[y][x].cptr < 2)));

  move_rec(char_row, char_col, y, x);
  for (i1 = char_row - 1; i1 <= char_row + 1; i1++) {
    for (i2 = char_col - 1; i2 <= char_col + 1; i2++) {
      /* with cave[i1,i2] do; */
      cave[i1][i2].tl = false;
      if (!(test_light(i1, i2))) {
        unlite_spot(i1, i2);
      }
    }
  }

  if (test_light(char_row, char_col)) {
    lite_spot(char_row, char_col);
  }

  char_row = y;
  char_col = x;
  move_char(5);
  creatures(false);
  teleport_flag = false;

  LEAVE("teleport", "d");
}

boolean get_panel(long y, long x, boolean forceit) {
  /*{ Given an row (y) and col (x), this routine detects  -RAK-     }*/
  /*{ when a move off the screen has occurred and figures new borders}*/
  /* forceit forcses the panel bounds to be recalculated (show_location).
   */

  long prow, pcol;
  boolean return_value;

  prow = panel_row;
  pcol = panel_col;

  if (forceit || (y < panel_row_min + 2) || (y > panel_row_max - 2)) {
    prow = trunc((y - 2) / (SCREEN_HEIGHT / 2));
    if (prow > max_panel_rows) {
      prow = max_panel_rows;
    } else if (prow < 0) {
      prow = 0;
    }
  }

  if (forceit || (x < panel_col_min + 3) || (x > panel_col_max - 3)) {
    pcol = trunc((x - 3) / (SCREEN_WIDTH / 2));
    if (pcol > max_panel_cols) {
      pcol = max_panel_cols;
    } else if (pcol < 0) {
      pcol = 0;
    }
  }

  if ((prow != panel_row) || (pcol != panel_col) || !(cave_flag)) {
    panel_row = prow;
    panel_col = pcol;
    s__panel_bounds();
    cave_flag = true;
    return_value = true;
  } else {
    return_value = false;
  }

  return return_value;
}

void move_rec(long y1, long x1, long y2, long x2) {
  /*{ Moves creature record from one space to another       -RAK-   }*/
  /* (x1,y1) might equal (x2,y2) so use a temp var */

  unsigned char i1;

  i1 = cave[y1][x1].cptr;
  cave[y1][x1].cptr = 0;
  cave[y2][x2].cptr = i1;
}

boolean find_range(obj_set const item_val, boolean inner, treas_rec **first,
                   long *count) {
  treas_rec *ptr;

  ENTER(("find_range", ""));

  *count = 0;
  *first = NULL;

  change_all_ok_stats(false, false);

  for (ptr = inventory_list; ptr != NULL; ptr = ptr->next) {

    /* Filter */
    if (!is_in(ptr->data.tval, item_val))
      continue;
    if (ptr->is_in && !inner)
      continue;
    if (ptr->data.tval == bag_or_sack && ptr->insides > 0)
      continue;

    /* Do */
    if (*count == 0)
      *first = ptr;
    ptr->ok = true;
    (*count)++;
  }

  MSG(("find: count=%ld\n", *count));

  LEAVE("find_range", "");
  return *count > 0;
}

void carry(long y, long x) {
  /*{ Player is on an object.  Many things can happen BASED -RAK-   }*/
  /*{ on the TVAL of the object.  Traps are set off, money and most }*/
  /*{ objects are picked up.  Some objects, such as open doors, just}*/
  /*{ sit there...                                                  }*/

  treas_rec *item_ptr;
  char out_val[82];
  char out2[120];
  char page_char;
  char inv_char;
  treas_rec *tmp_ptr;
  long count;
  boolean money_flag;

  ENTER(("carry", "%d, %d", y, x));

  money_flag = false;
  find_flag = false;

  /* with cave[y][x]. do; */
  inven_temp.data = t_list[cave[y][x].tptr];

  /*{ There's GOLD in them thar hills!      }*/
  /*{ OPPS!                                 }*/

  if (is_in(t_list[cave[y][x].tptr].tval, trap_set)) {
    hit_trap(&y, &x);
  } else if (t_list[cave[y][x].tptr].tval <= valuable_metal) {

    /*{ Attempt to pick up an object.         }*/
    if (inven_check_num()) {      /*{ Too many objects?     }*/
      if (inven_check_weight()) { /*{ Weight limit check }*/

        /*{ Okay, pick it up      }*/
        pusht(cave[y][x].tptr);
        cave[y][x].tptr = 0;

        if (inven_temp.data.tval == valuable_metal) {
          item_ptr = money_carry();
          money_flag = true;
        } else {
          item_ptr = inven_carry();
        }

        prt_stat_block();
        objdes(out_val, item_ptr, true);

        if (money_flag) {

          page_char = '$';
          inv_char = '$';

        } else {

          count = 0;
          tmp_ptr = inventory_list;

          if (tmp_ptr->next == item_ptr->next) {
            count = 0;
          } else {
            do {
              count++;
              tmp_ptr = tmp_ptr->next;
            } while (tmp_ptr->next != item_ptr->next);
          }

          if ((count / 20) > 9) {
            page_char = '*';
            inv_char = '*';
          } else {
            page_char = ((count / 20) + 49);
            inv_char = (count - (count / 20) * 20 + 97);
          }
        }
        sprintf(out2, "You have %s. (%c%c)", out_val, page_char, inv_char);
        msg_print(out2);
      } else {
        msg_print("You can't carry that much weight.");
      }
    } else {
      msg_print("You can't carry that many items.");
    }
  }
  LEAVE("carry", "");
}

long react(long x) {
  /*  returns 0 to 10 -- SD 2.4; */
  /*  x is average reaction for a 0 SC ugly half-troll*/

  long ans = (C_player_get_stat(CHR) * 10 + (player_rep * 2) + randint(200) +
              randint(200) + randint(200)) /
                 50 +
             x - 4;

  if (ans < 0) {
    ans = 0;
  } else if (ans > 10) {
    ans = 10;
  }

  return ans;
}

void battle_game(long plus, char kb_str[82]) {
  long score, i1, time;
  char out_val[82];

  if (get_yes_no("Do you accept their invitation?")) {
    msg_print("Good for you!");
    score = 0;
    time = 10;

    /* with player_do; */
    for (i1 = 1; i1 <= 7; i1++) {
      if (player_test_hit(player_bth, player_lev, plus, 20 * i1, false)) {
        score++;
        time = time * 2 + 10;
      }
    }

    sprintf(out_val, "with some %s", kb_str);
    spend_time(time, out_val, false);

    switch (score) {
    case 1:
      msg_print("They ridicule your clumsy performance...");
      msg_print("'Come back when you are more experienced!!'");
      change_rep(-2);
      break;

    case 2:
      msg_print("You do not do well...");
      break;

    case 3:
      msg_print("'Pretty good for a beginner!'");
      break;

    case 4:
      msg_print("They are quite impressed!");
      break;

    case 5:
      msg_print("They are amazed by your incredible prowess!");
      change_rep(2);
      break;

    case 6:
    case 7:
      msg_print("You handle them all with ease!");
      msg_print("'Thanks for the workout! Come back anytime!!'");
      C_player_add_exp(10);
      change_rep(5);
      break;

    default:
      msg_print("'Boy that was quick!! What a little wimp!'");
      msg_print("They pummel you senseless and toss you out "
                "into the street!");
      take_hit(damroll("2d4"), kb_str);
      change_rep(-5);
      player_flags.confused += 5 + randint(5);
      break;
    }
  }
}

void brothel_game() {
  if (get_yes_no("Do you accept?")) {
    change_rep(-3);
    /* with player_do; */
    if ((player_disarm + player_lev + 2 * C_player_disarm_from_dex() +
         C_player_mod_from_stat(INT)) > randint(100)) {
      msg_print("Good! You are invited to join the house!");
      C_player_add_exp(5);
      spend_time(600, "putting out for peasants", false);
    } else {
      msg_print("You fail to please your customers.");
      spend_time(400, "imitating a pine board", false);
    }
  }
}

void guild_or_not(boolean passed) {
  if (passed) {
    spend_time(600, "showing off your skills", false);
    msg_print("Good! You are invited to join the guild!");
    C_player_add_exp(5);
    change_rep(-3);
  } else {
    spend_time(400, "or lack thereof", false);
    msg_print("You fail to impress them.");
    if (randint(3) == 1) {
      msg_print("They think you are with the guard!");
      msg_print("You are stabbed by one of them before you escape");
      take_hit(randint(randint(16)), "Thieves Guild Member");
      prt_stat_block();
    }
  }
}

void thief_games() {
  if (randint(2) == 1) {
    msg_print("The thieves invite you to prove your ability to "
              "pick locks.");
    if (get_yes_no("Do you accept?")) {
      /* with player_do; */
      guild_or_not((player_disarm + player_lev +
                    2 * C_player_disarm_from_dex() +
                    C_player_mod_from_stat(INT)) > randint(100));
    }
  } else {
    msg_print("The thieves invite you to show your stealthiness.");
    if (get_yes_no("Do you accept?")) {
      guild_or_not(player_stl > randint(12));
    }
  }
}

void kicked_out() { msg_print("The owner kicks you out..."); }

void call_guards(char who[82]) {
  char out_str[82];

  sprintf(out_str, "The %s call(s) for the Town Guards!", who);
  msg_print(out_str);
  monster_summon_by_name(char_row, char_col, "Town Guard", true, false);
  monster_summon_by_name(char_row, char_col, "Town Guard", true, false);
}

void call_wizards() {
  msg_print("The mage calls for a Town Wizard to remove you.");
  monster_summon_by_name(char_row, char_col, "Town Wizard", true, false);
}

void eat_the_meal() {
  long yummers, old_food;

  old_food = player_flags.foodc;

  yummers = react(randint(8) - 2);
  if ((yummers == 10) && (randint(2) == 1)) {
    yummers = 15;
  }

  spend_time(50 + 50 * yummers, "eating like a pig", false);

  switch (yummers) {
  case 15:
    msg_print("It is a sumptuous banquet, and you feel quite stuffed.");
    player_flags.foodc = PLAYER_FOOD_MAX;
    player_flags.status &= ~(IS_WEAK | IS_HUNGERY);
    prt_hunger();
    change_rep(3);
    break;

  case 6:
  case 7:
  case 8:
  case 9:
  case 10:
    msg_print("It is an ample meal, and you feel full.");
    player_flags.foodc = PLAYER_FOOD_FULL;
    player_flags.status &= ~(IS_WEAK | IS_HUNGERY);
    prt_hunger();
    change_rep(1);
    break;

  default:
    if ((yummers > 0) ||
        player_saves(player_lev + 5 * C_player_mod_from_stat(CON))) {
      msg_print("It was a boring meal, and you eat very little.");
      player_flags.foodc = old_food;
      prt_hunger();
    } else {
      msg_print("Yuk!  That meal was AWFUL!");
      msg_print("You throw up!");
      if (player_flags.foodc > 150) {
        player_flags.foodc = 150;
      }
      msg_print("You get food poisoning.");
      player_flags.poisoned += randint(10) + 5;
      change_rep(-2);
    }
    break;
  }
}

void invite_for_meal() {
  msg_print("The occupants invite you in for a meal.");
  if (get_yes_no("Do you accept?")) {
    eat_the_meal();
  }
}

void party() {
  msg_print("The owner invites you to join the party!");
  if (get_yes_no("Do you accept?")) {
    spend_time(400 + randint(1600), "at a party", false);

    switch (randint(6)) {
    case 1:
      msg_print("Someone must have spiked the punch!");
      msg_print("Oh, your aching head!");
      player_flags.confused += 25 + randint(25);
      break;

    case 2:
      msg_print("Gee, those brownies were awfully unusual....");
      msg_print("You feel a little strange now.");
      player_flags.image += 200 + randint(100);
      break;

    case 3:
      msg_print("You smoked something strange at the party.");
      switch (randint(2)) {
      case 1:
        player_flags.hero += 25 + randint(25);
        break;
      case 2:
        player_flags.afraid += 25 + randint(25);
        break;
      }
      break;

    case 4:
    case 5:
    case 6:
      msg_print("It is an interesting party, and you enjoy "
                "yourself.");
      break;
    }
  }
}

void spend_the_night(char who[82]) {
  char out_str[82];

  msg_print("The occupant(s) invite you to rest in his house.");
  if (get_yes_no("Do you accept?")) {
    sprintf(out_str, "at the home of the %s.", who);
    spend_time(1, out_str, true);
    change_rep(2);
  } else if (get_yes_no("Okay, how about staying for a meal?")) {
    eat_the_meal();
  }
}

void worship() {
  long preachy, i1;

  msg_print("The priest invites you to participate in the service.");

  if (get_yes_no("Do you accept?")) {
    preachy = randint(4);

    switch (preachy) {
    case 1:
      msg_print("You sit through a fascinating church service.");
      break;
    case 2:
      msg_print("You sit through an interesting church service.");
      break;
    case 3:
      msg_print("You sit through a boring church service.");
      break;
    case 4:
      msg_print("You sit through a long, boring church service.");
      break;
    }

    spend_time(100 * (randint(7) + preachy * preachy), "at the Church", false);

    msg_print("The priest asks for donations for a new church.");
    if (get_yes_no("Will you give him some money?")) {
      /* with player_do; */
      if (player_money[TOTAL_] > 0) {
        msg_print("Bless you, dude!");

        i1 = ((randint(12) * player_money[TOTAL_]) / 1000 + 20) * GOLD_VALUE;
        if (i1 > player_money[TOTAL_] * GOLD_VALUE / 2) {
          i1 = player_money[TOTAL_] * GOLD_VALUE / 2;
        }

        subtract_money(i1, false);
        prt_stat_block();

        if (i1 > 20 * GOLD_VALUE) {
          change_rep(5);
        } else {
          change_rep((i1 + 5 * GOLD_VALUE - 1) / (5 * GOLD_VALUE));
        }

      } else {
        msg_print("He says 'It is the thought that "
                  "counts, my child.'");
        msg_print("Thank you for being willing to give.");
      }
    } else {
      msg_print("Syo problem, man?");
      change_rep(-5);
    }
  } else if (react(6) == 0) {
    msg_print("You heathen!  Get out of my temple!");
    change_rep(-5);
  }
}

void beg_food() {
  /*
       var      i2              : long;
                item_ptr        : treas_rec;*
       begin
        if (find_range([food],false,item_ptr,i2)) then
          begin
            msg_print("The occupants beg you for food.");
            if get_yes_no("Will you feed them?") then
              begin
                spend_time(200,"feeding people",false);
                msg_print("How kind of you!");
                inven_destroy(item_ptr);
                change_rep(5);
                prt_stat_block();
              end
            else
              begin
                msg_print("What a jerk!");
                change_rep(-10);
              end;
          end
        else
          beg_money();
       end;
       */
}

void beg_money() {
  long i1;

  msg_print("The occupants beg you for money.");

  if (get_yes_no("Will you give them some?")) {
    /* with player_do; */
    if (player_money[TOTAL_] > 0) {
      msg_print("How kind of you!");
      spend_time(100, "giving handouts", false);
      i1 = ((randint(12) * player_money[TOTAL_]) / 1000 + 20) * GOLD_VALUE;
      if (i1 > player_money[TOTAL_] * GOLD_VALUE / 2) {
        i1 = player_money[TOTAL_] * GOLD_VALUE / 2;
      }
      subtract_money(i1, false);
      if (i1 > 20 * GOLD_VALUE) {
        change_rep(5);
      } else {
        change_rep((i1 + 5 * GOLD_VALUE - 1) / (5 * GOLD_VALUE));
      }
      prt_stat_block();
    } else {
      msg_print("They are disappointed because you have no money.");
    }
  } else {
    msg_print("What a jerk!");
    change_rep(-10); /*{bug fixed here; used to be 10 -- MAV }*/
  }
}

boolean player_test_hit(long base_to_hit, long level, long plus_to_hit,
                        long enemy_ac, boolean was_fired) {
  if (search_flag) {
    search_off();
  }
  if (player_flags.rest > 0) {
    rest_off();
  }

  /* compare player::melee_tohit()  and player::ranged_tohit() */
  long hit_value = base_to_hit + (plus_to_hit * BTH_PLUS_ADJ);

  if (was_fired) {
    hit_value += (level * C_class_ranged_bonus(player_pclass)) / 2;
  } else {
    hit_value += (level * C_class_melee_bonus(player_pclass)) / 2;
  }

  /*{ hits if above ac or 1 in 20.  OOK! }*/
  if (randint(hit_value) > enemy_ac) {
    return true;
  } else if (randint(20) == 1) {
    return true;
  } else {
    return false;
  }
}

boolean test_hit(long bth, long level, long pth, long ac) {
  /*{ Attacker's level and pluses, defender's AC            -RAK-   }*/

  long i1;
  boolean return_value;

  if (search_flag) {
    search_off();
  }
  if (player_flags.rest > 0) {
    rest_off();
  }

  i1 = bth + level * BTH_LEV_ADJ + pth * BTH_PLUS_ADJ;

  /*{ hits if above ac or 1 in 20.  OOK! }*/
  return_value = ((randint(i1) > ac) || (randint(20) == 1));

  return return_value;
}

boolean minus_ac(long typ_dam) {
  /*{ AC gets worse                                         -RAK-   }*/
  /*{ Note: This routine affects magical AC bonuse so that stores   }*/
  /*{       can detect the damage.                                  }*/

  long i1 = 0;
  long i2;
  long tmp[9]; /*  : array [1..8] of long;*/
  char out_str[120];
  char out_val[82];
  boolean return_value = false;

  if (equipment[Equipment_armor].tval > 0) {
    i1++;
    tmp[i1] = Equipment_armor;
  }
  if (equipment[Equipment_shield].tval > 0) {
    i1++;
    tmp[i1] = Equipment_shield;
  }
  if (equipment[Equipment_cloak].tval > 0) {
    i1++;
    tmp[i1] = Equipment_cloak;
  }
  if (equipment[Equipment_gloves].tval > 0) {
    i1++;
    tmp[i1] = Equipment_gloves;
  }
  if (equipment[Equipment_helm].tval > 0) {
    i1++;
    tmp[i1] = Equipment_helm;
  }
  if (equipment[Equipment_boots].tval > 0) {
    i1++;
    tmp[i1] = Equipment_boots;
  }
  if (equipment[Equipment_belt].tval > 0) {
    i1++;
    tmp[i1] = Equipment_belt;
  }
  if (equipment[Equipment_bracers].tval > 0) {
    i1++;
    tmp[i1] = Equipment_bracers;
  }

  if (i1 > 0) {
    i2 = tmp[randint(i1)];
    inven_temp.data = equipment[i2];
    /* with equipment[i2] do; */
    if ((equipment[i2].flags & typ_dam) != 0) {
      objdes(out_val, &inven_temp, false);
      sprintf(out_str, "Your %s resists damage!", out_val);
      msg_print(out_str);
      return_value = true;
    } else if ((equipment[i2].ac + equipment[i2].toac) > 0) {
      objdes(out_val, &inven_temp, false);
      sprintf(out_str, "Your %s is damaged!", out_val);
      msg_print(out_str);
      equipment[i2].toac--;
      py_bonuses(&blank_treasure, 0);
      return_value = true;
    }
  }

  return return_value;
}

void fire_dam(long dam, char kb_str[82]) {
  /*{ Burn the fool up...                                   -RAK-   }*/

  obj_set things_that_burn = {arrow,
                              bow_crossbow_or_sling,
                              hafted_weapon,
                              pole_arm,
                              maul,
                              boots,
                              gloves_and_gauntlets,
                              cloak,
                              soft_armor,
                              staff,
                              Scroll,
                              0};

  if (player_flags.fire_resist) {
    dam /= 3;
  }
  if (player_flags.resist_heat > 0) {
    dam /= 3;
  }

  take_hit(dam, kb_str);

  print_stat = 1;

  if (inven_damage(things_that_burn, 3) > 0) {
    msg_print("There is smoke coming from your pack!");
    prt_stat_block();
  }
}

void cold_dam(long dam, char kb_str[82]) {
  /*{ Freeze him to death...                                -RAK-   }*/

  obj_set things_that_freeze = {potion, 0};

  if (player_flags.cold_resist) {
    dam /= 3;
  }
  if (player_flags.resist_cold > 0) {
    dam /= 3;
  }

  take_hit(dam, kb_str);

  print_stat = 1;

  if (inven_damage(things_that_freeze, 5) > 0) {
    msg_print("Something shatters inside your pack!");
    prt_stat_block();
  }
}

void light_dam(long dam, char kb_str[82]) {
  /*{ Lightning bolt the sucker away...                     -RAK-   }*/

  if (player_flags.lght_resist) {
    dam /= 3;
  }
  if (player_flags.resist_lght > 0) {
    dam /= 3;
  }

  take_hit(dam, kb_str);

  print_stat = 1;
}

void acid_dam(long dam, char kb_str[82]) {
  /*{ Throw acid on the hapless victim                      -RAK-   }*/

  long flag = 0;
  obj_set things_that_dilute = {
      miscellaneous_object,  chest,         bolt,       arrow,
      bow_crossbow_or_sling, hafted_weapon, pole_arm,   boots,
      gloves_and_gauntlets,  cloak,         soft_armor, 0};

  if (minus_ac(Resist_Acid_worn_bit)) {
    flag = 1;
  }
  if (player_flags.acid_resist) {
    flag += 2;
  }

  switch (flag) {
  case 0:
    take_hit((dam / 1), kb_str);
    break;
  case 1:
    take_hit((dam / 2), kb_str);
    break;
  case 2:
    take_hit((dam / 3), kb_str);
    break;
  case 3:
    take_hit((dam / 4), kb_str);
    break;
  }

  print_stat = 1;

  if (inven_damage(things_that_dilute, 3) > 0) {
    msg_print("There is an acrid smell coming from your pack!");
    prt_stat_block();
  }
}

void xp_loss(long amount) {
  /*{ Lose experience hack for lose_exp breath              -RAK-   }*/

  long i1, i2;
  long av_hp, lose_hp;
  long av_mn, lose_mn;
  boolean flag;

  amount = (player_exp / 100) * MON_DRAIN_LIFE; /* passed val?  XXXX */

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
  i2 = player_lev - i1;

  for (; i2 > 0;) {
    av_hp = (C_player_max_hp() / player_lev);
    av_mn = (player_mana / player_lev);
    player_lev--;
    i2--;
    lose_hp = randint(av_hp * 2 - 1);
    lose_mn = randint(av_mn * 2 - 1);
    C_player_modify_max_hp(lose_hp);
    player_mana -= lose_mn;
    if (player_mana < 0) {
      player_mana = 0;
    }

    if (C_player_uses_magic(M_ARCANE) || C_player_uses_magic(M_DIVINE) ||
        C_player_uses_magic(M_NATURE) || C_player_uses_magic(M_SONG) ||
        C_player_uses_magic(M_CHAKRA)) {
      i1 = 32;
      flag = false;
      do {
        i1--;
        if (C_player_knows_spell(i1)) {
          flag = true;
        }
      } while (!((flag) || (i1 < 2)));
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

void corrode_gas(char kb_str[82]) {
  /*{ Corrode the unsuspecting person's armor               -RAK-   }*/

  obj_set things_that_corrode = {dagger, sword,      gem_helm, helm,
                                 shield, hard_armor, wand,     0};

  if (!(minus_ac(Resist_Acid_worn_bit))) {
    /* if nothing corrodes then take damage */
    take_hit(randint(8), kb_str);
  }

  print_stat = 1;

  if (inven_damage(things_that_corrode, 5) > 0) {
    msg_print("There is an acrid smell coming from your pack.");
    prt_stat_block();
  }
}

void poison_gas(long dam, char kb_str[82]) {
  /*{ Poison gas the idiot...                               -RAK-   }*/

  take_hit(dam, kb_str);
  print_stat = 1;
  player_flags.poisoned += 12 + randint(dam);
}

boolean no_light() {
  /*{ Returns true if player has no light                   -RAK-   }*/

  boolean return_value = false;

  /* with cave[char_row,char_col] do; */
  if (!(cave[char_row][char_col].tl)) {
    if (!(cave[char_row][char_col].pl)) {
      return_value = true;
    }
  }
  return return_value;
}

/**
 * water_move_item() - I sense a patter about water moves...
 */
boolean water_move_item(__attribute__((unused)) long row,
                        __attribute__((unused)) long col,
                        __attribute__((unused)) long num) {
  return true;
}

boolean water_move() {
  boolean flag = false;

  // water_move_player();

  for (long i = muptr; i != 0; i = m_list[i].nptr) {
    m_list[i].moved = false;
  }

  for (long i = muptr; i != 0; i = m_list[i].nptr) {
    // flag = water_move_creature(i);
  }

  return flag;
}

void search(long player_y, long player_x, long chance) {
  /*{ Searches for hidden things...                         -RAK-   }*/

  if ((player_flags).confused + (player_flags).blind > 0) {
    chance = trunc(chance / 10.0);
  } else if (no_light()) {
    chance = (long)(chance / 5.0);
  }

  for (long y = (player_y - 1); y <= (player_y + 1); y++) {
    for (long x = (player_x - 1); x <= (player_x + 1); x++) {
      if (!in_bounds(y, x)) {
        continue;
      }
      if (y == player_y && x == player_x) {
        // There can be no unfound thing where the
        // player is standing?
        continue;
      }
      if (randint(100) >= chance) {
        continue;
      }

      // Search for hidden objects
      if (cave[y][x].tptr <= 0) {
        continue;
      }

      if (t_list[cave[y][x].tptr].tval == unseen_trap) {
        // Trap on floor
        char out_val[86];
        sprintf(out_val, "You have found %s.", t_list[cave[y][x].tptr].name);
        msg_print(out_val);
        change_trap(y, x);
        find_flag = false;

      } else if (t_list[cave[y][x].tptr].tval == secret_door) {
        // Secret door
        msg_print("You have found a secret door.");
        cave[y][x].fval = corr_floor2.ftval;
        change_trap(y, x);
        find_flag = false;

      } else if (t_list[cave[y][x].tptr].tval == chest) {
        if (t_list[cave[y][x].tptr].flags > 1) {
          if (pindex(t_list[cave[y][x].tptr].name, '^') > 0) {
            // Chest is trapped
            known2(t_list[cave[y][x].tptr].name);
            msg_print("You have discovered a trap on the chest!");
          }
        }
      }
    }
  }
}

void lr__find_light(long y1, long x1, long y2, long x2) {
  obj_set room_floors;
  long i1;

  memset(room_floors, 0, sizeof(room_floors));
  room_floors[0] = dopen_floor.ftval;
  room_floors[1] = lopen_floor.ftval;
  room_floors[2] = water2.ftval;

  for (i1 = y1; i1 <= y2; i1++) {
    long i2;
    for (i2 = x1; i2 <= x2; i2++) {
      long i3;
      if (!is_in(cave[i1][i2].fval, room_floors))
        continue;

      for (i3 = i1 - 1; i3 <= i1 + 1; i3++) {
        long i4;
        for (i4 = i2 - 1; i4 <= i2 + 1; i4++)
          cave[i3][i4].pl = true;
      }

      if (cave[i1][i2].fval == water2.ftval) {
        /* water on room floor */
        cave[i1][i2].fval = water3.ftval;
        /* lit rm water on floor */
      } else {
        cave[i1][i2].fval = lopen_floor.ftval;
        /* lit rm floor */
      }
    }
  }
}

void light_room(long param_y, long param_x) {
  /*{ Room is lit, make it appear                           -RAK-   }*/

  long const half_height = (long)(SCREEN_HEIGHT / 2);
  long const half_width = (long)(SCREEN_WIDTH / 2);
  long const start_row = (long)(param_y / half_height) * half_height + 1;
  long const start_col = (long)(param_x / half_width) * half_width + 1;
  long const end_row = start_row + half_height - 1;
  long const end_col = start_col + half_width - 1;
  long y;
  long xpos = 0;

  ENTER(("light_room", "%d, %d", param_y, param_x));

  lr__find_light(start_row, start_col, end_row, end_col);

  for (y = start_row; y <= end_row; y++) {
    chtype floor_str[82] = {0};
    long floor_str_len = 0;
    long x;
    long const ypos = y;
    for (x = start_col; x <= end_col; x++) {
      if (cave[y][x].pl || cave[y][x].fm) {
        if (floor_str_len == 0)
          xpos = x;
        floor_str[floor_str_len++] = loc_symbol(y, x);
      } else {
        if (floor_str_len > 0) {
          floor_str[floor_str_len] = 0;
          print_chstr(floor_str, ypos, xpos);
          floor_str[0] = 0;
          floor_str_len = 0;
        }
      }
    }
    if (floor_str_len > 0) {
      floor_str[floor_str_len] = 0;
      print_chstr(floor_str, ypos, xpos);
    }
  }

  LEAVE("light_room", "");
}

void area_affect(long dir, long y, long x) {
  /*        { Turns off Find_flag if something interesting appears  -RAK-
   * }*/
  /*        { BUG: Does not handle corridor/room corners, but I didn't
   * want }*/
  /*        {      to add a lot of checking for such a minor detail }*/

  long z[4]; /*: array [1..3] of long;*/
  long i1, row, col;
  obj_set corridors = {4, 5, 6, 0};
  obj_set some_hidden_stuff = {unseen_trap, secret_door, 0};

  if (cave[y][x].fval == corr_floor1.ftval) {
    if (next_to4(y, x, corridors) > 2) {
      find_flag = false;
    }
  }

  if ((find_flag) && (player_flags.blind < 1)) {

    switch (dir) {
    case 1:
    case 3:
    case 7:
    case 9:
      z[1] = rotate_dir(dir, -1);
      z[2] = dir;
      z[3] = rotate_dir(dir, 1);
      break;

    case 2:
    case 4:
    case 6:
    case 8:
      z[1] = rotate_dir(dir, -2);
      z[2] = dir;
      z[3] = rotate_dir(dir, 2);
      break;
    }

    for (i1 = 1; i1 <= 3; i1++) {
      row = y;
      col = x;
      if (move_dir(z[i1], &row, &col)) {

        /* with cave[row,col] do; */

        /* { Empty doorways        }*/
        if (cave[row][col].fval == corr_floor2.ftval) {
          find_flag = false;
        }

        /*  { Objects player can see}*/
        /*  { Including doors       }*/
        if (find_flag) {
          if (player_light) {
            if (cave[row][col].tptr > 0) {
              if (!(is_in(t_list[cave[row][col].tptr].tval,
                          some_hidden_stuff))) {
                find_flag = false;
              }
            }
          } else if ((cave[row][col].tl) || (cave[row][col].pl) ||
                     (cave[row][col].fm)) {
            if (cave[row][col].tptr > 0) {
              if (!(is_in(t_list[cave[row][col].tptr].tval,
                          some_hidden_stuff))) {
                find_flag = false;
              }
            }
          }
        }

        /*{ Creatures             }*/
        if (find_flag) {
          if ((cave[row][col].tl) || (cave[row][col].pl) || (player_light)) {
            if (cave[row][col].cptr > 1) {
              /* with */
              /* m_list[cave[row][col].cptr]
               */
              /* do; */
              if (m_list[cave[row][col].cptr].ml) {
                find_flag = false;
              }
            }
          }
        }

      } /* end if move_char */
    }   /* end for */
  }     /* end if find and not blind */
}

boolean delete_object(long y, long x) {
  /*{ Deletes object from given location                    -RAK-   }*/

  boolean return_value = false;

  /* with cave[y,x] do; */
  if (t_list[cave[y][x].tptr].tval == secret_door) {
    cave[y][x].fval = corr_floor3.ftval;
  }
  cave[y][x].fopen = true;
  pusht(cave[y][x].tptr);
  cave[y][x].tptr = 0;
  cave[y][x].fm = false;
  if (test_light(y, x)) {
    lite_spot(y, x);
    return_value = true;
  } else {
    unlite_spot(y, x);
  }

  return return_value;
}

long mon_take_hit(long monptr, long dam) {
  /*{ Decreases monsters hit points and deletes monster if needed.  }*/
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

void monster_death(long y, long x, unsigned long flags) {
  /*{ Allocates objects upon a creatures death              -RAK-   }*/
  /*{ Oh well, another creature bites the dust...  Reward the victor}*/
  /*{ based on flags set in the main creature record                }*/

  long i1;

  i1 = (long)((flags & 0x03000000) / (0x01000000));

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

void summon_object(long y, long x, long num, long typ) {
  /*{ Creates objects nearby the coordinates given          -RAK-   }*/
  /*{ BUG: Because of the range, objects can actually be placed into}*/
  /*{      areas closed off to the player, this is rarely noticable,}*/
  /*{      and never a problem to the game.                         }*/

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

void delete_monster(long i2) {
  /*{ Deletes a monster entry from the level                -RAK-   }*/

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

boolean py_attack(long y, long x) {
  /*{ Player attacks a (poor, defenseless) creature         -RAK-   }*/

  long a_cptr;
  long a_mptr;
  long i3;
  long blows;
  long tot_tohit;
  long crit_mult;
  char m_name[82];
  char out_val[120];
  boolean mean_jerk_flag;
  boolean is_sharp;
  boolean backstab_flag;

  obj_set mages_suck = {hafted_weapon, pole_arm, sword, maul, 0};
  obj_set priests_suck = {hafted_weapon, pole_arm, dagger, sword, 0};
  obj_set druids_suck = {hafted_weapon, pole_arm, sword, 0};
  obj_set monks_suck = {hafted_weapon, pole_arm, maul, 0};
  obj_set catch_this = {sling_ammo, bolt, arrow, 0};

  boolean return_value = false;

  ENTER(("py_attack", "%d, %d", y, x));

  a_cptr = cave[y][x].cptr;
  a_mptr = m_list[a_cptr].mptr;

  if ((player_pclass == C_ROGUE) && (m_list[a_cptr].csleep != 0)) {
    backstab_flag = true;
  } else {
    backstab_flag = false;
  }
  m_list[a_cptr].csleep = 0;

  find_monster_name(m_name, a_cptr, false);

  if (equipment[Equipment_primary].tval > 0) { /*  { Proper weapon }*/
    blows = attack_blows(equipment[Equipment_primary].weight, &tot_tohit);
  } else { /*{ Bare hands?   }*/
    if (player_pclass == C_MONK) {
      blows = attack_blows(12000, &tot_tohit) + 1; /* a bit heavy handed... */
      tot_tohit = 0;
    } else {
      blows = 2;
      tot_tohit = -3;
    }
  }

  if (backstab_flag) {
    tot_tohit += (player_lev / 4);
  }

  /*{ Adjust weapons for class }*/
  if (player_pclass == C_WARRIOR) {
    tot_tohit += 1 + (player_lev / 2);

  } else if ((player_pclass == C_MAGE) &&
             (is_in(equipment[Equipment_primary].tval, mages_suck))) {
    tot_tohit -= 5;

  } else if ((player_pclass == C_PRIEST) &&
             (is_in(equipment[Equipment_primary].tval, priests_suck))) {
    tot_tohit -= 4;

  } else if ((player_pclass == C_DRUID) &&
             (is_in(equipment[Equipment_primary].tval, druids_suck))) {
    tot_tohit -= 4;

  } else if ((player_pclass == C_MONK) &&
             (is_in(equipment[Equipment_primary].tval, monks_suck))) {
    tot_tohit -= 3;
  }

  /*{ Fix for arrows}*/
  if (is_in(equipment[Equipment_primary].tval, catch_this)) {
    blows = 1;
  }
  tot_tohit += player_ptohit;

  /*{ stopped from killing town creatures?? }*/
  if (((c_list[a_mptr].cmove & 0x00004000) == 0) ||
      (randint(100) < -player_rep)) {
    mean_jerk_flag = true;
  } else {
    mean_jerk_flag = get_yes_no("Are you sure you want to?");
  }

  /*{ Loop for number of blows, trying to hit the critter...        }*/
  if (mean_jerk_flag) {
    /* with player_do; */
    do {
      if (player_test_hit(player_bth, player_lev, tot_tohit, c_list[a_mptr].ac,
                          false)) {
        if (backstab_flag) {
          sprintf(out_val, "You backstab %s!", m_name);
        } else {
          sprintf(out_val, "You hit %s.", m_name);
        }
        msg_print(out_val);
        /* with equipment[Equipment_primary]. do; */
        /*{ Weapon?       }*/
        if (equipment[Equipment_primary].tval > 0) {
          i3 = damroll(equipment[Equipment_primary].damage);
          i3 = tot_dam(&equipment[Equipment_primary], i3, &c_list[a_mptr]);
          is_sharp =
              (equipment[Equipment_primary].tval != bow_crossbow_or_sling) &&
              ((equipment[Equipment_primary].flags2 & Sharp_worn_bit) != 0);
          crit_mult = critical_blow(equipment[Equipment_primary].weight,
                                    tot_tohit, is_sharp, false);
          if (backstab_flag) {
            i3 *= ((player_lev / 7) + 1);
          }
          if (player_pclass == C_WARRIOR) {
            i3 += (player_lev / 3);
          }
          i3 += (i3 + 5) * crit_mult;

        } else { /*{ Bare hands!?  }*/
          if (player_pclass == C_MONK) {
            i3 = randint((4 + 2 * player_lev) / 3);
            crit_mult = critical_blow(12000, 0, false, false);
            if (randint(crit_mult + 2) > 2) {
              do_stun(a_cptr, -10, 2);
            }
            i3 += (i3 + 5) * crit_mult;
          } else {
            i3 = damroll(bare_hands);
            crit_mult = critical_blow(1, 0, false, false);
            i3 += (i3 + 5) * crit_mult;
          }
        }

        i3 += player_ptodam;
        if (i3 < 0) {
          i3 = 0;
        }
        /*{ See if we done it in... }*/
        /* with m_list[a_cptr]. do; */
        if (mon_take_hit(a_cptr, i3) > 0) {
          sprintf(out_val, "You have slain %s.", m_name);
          msg_print(out_val);
          blows = 0;
          return_value = false;
        } else {
          return_value = true; /*{ If creature
                                  hit, but
                                  alive...}*/
        }

        /* with equipment[Equipment_primary]. do; */
        /*{ Use missiles up}*/
        if (is_in(equipment[Equipment_primary].tval, catch_this)) {
          equipment[Equipment_primary].number--;
          if (equipment[Equipment_primary].number <= 0) {
            inven_weight -= equipment[Equipment_primary].weight;
            prt_stat_block();
            equip_ctr--;
            inven_temp.data = equipment[Equipment_primary];
            equipment[Equipment_primary] = blank_treasure;
            py_bonuses(&(inven_temp.data), -1);
          }
        }
      } else {
        sprintf(out_val, "You miss %s.", m_name);
        msg_print(out_val);
      }
      blows--;
    } while (blows >= 1);
  }

  RETURN("py_attack", "", 'b', "hit", &return_value);
  return return_value;
}

long tot_dam(treasure_type *item, long tdam, creature_type *monster) {
  /*{ Special damage due to magical abilities of object     -RAK-   }*/

  obj_set stuff_that_goes_thump = {
      sling_ammo,    bolt,     arrow,  lamp_or_torch, bow_crossbow_or_sling,
      hafted_weapon, pole_arm, dagger, sword,         maul,
      flask_of_oil,  0};

  unsigned long cdefense, flags, flags2;

  cdefense = monster->cdefense;
  flags = item->flags;
  flags2 = item->flags2;

  /* with item do; */
  if (is_in(item->tval, stuff_that_goes_thump)) {
    /* with monster do; */

    /*{ Slay Dragon   }*/
    if (((cdefense & 0x0001) != 0) && ((flags & Slay_Dragon_worn_bit) != 0)) {
      tdam *= 4;

      /*{ Slay Undead   }*/
    } else if (((cdefense & 0x0008) != 0) &&
               ((flags & Slay_Undead_worn_bit) != 0)) {
      tdam *= 3;

      /*{ Demon Bane    }*/
    } else if (((cdefense & 0x0400) != 0) &&
               ((flags2 & Slay_demon_worn_bit) != 0)) {
      tdam *= 3;

      /*{ Slay Regenerative }*/
    } else if (((cdefense & 0x8000) != 0) &&
               ((flags2 & Slay_regen_worn_bit) != 0)) {
      tdam *= 3;

      /*{ Slay Monster  }*/
    } else if (((cdefense & 0x0002) != 0) &&
               ((flags & Slay_Monster_worn_bit) != 0)) {
      tdam *= 2;

      /*{ Slay Evil     }*/
    } else if (((cdefense & 0x0004) != 0) &&
               ((flags & Slay_Evil_worn_bit) != 0)) {
      tdam *= 2;

      /*{ Soul Sword    }*/
    } else if ((!((cdefense & 0x0008) != 0)) &&
               ((flags2 & Soul_Sword_worn_bit) != 0)) {
      tdam *= 2;
    }
    /*{ Frost         }*/
  } else if (((cdefense & 0x0010) != 0) &&
             ((flags & Cold_Brand_worn_bit) != 0)) {
    tdam *= 1.5;

    /*{ Fire          }*/
  } else if (((cdefense & 0x0020) != 0) &&
             ((flags & Flame_Brand_worn_bit) != 0)) {
    tdam *= 1.5;
  }

  return tdam;
}

void get_player_move_rate() {
  long cur_swim;

  /* with player_flags do; */
  if (is_in(cave[char_row][char_col].fval, earth_set)) {
    (player_flags).move_rate = 4;
  } else {
    cur_swim = (((player_flags).swim + randint(5) - 1) / 5);

    if (cur_swim <= -2) {
      (player_flags).move_rate = 0;
    } else if (cur_swim == -1) {
      (player_flags).move_rate = 1;
    } else if (cur_swim == 0) {
      (player_flags).move_rate = 2;
    } else if (cur_swim == 1) {
      (player_flags).move_rate = 4;
    } else {
      (player_flags).move_rate = 8;
    }
  }
}

boolean xor
    (long thing1, long thing2) {
      /* with fake boolean values you cant really do a (bool1 !=
         bool2)
         and expect it to work.  */

      return !((thing1 && thing2) || (!thing1 && !thing2));
    }

    long movement_rate(long cspeed, long mon) {
  /*{ Given speed, returns number of moves this turn.       -RAK-   }*/
  /*{ NOTE: Player must always move at least once per iteration,    }*/
  /*{       a slowed player is handled by moving monsters faster    }*/

  long final_rate;      /*{ final speed as long }*/
  long c_rate, py_rate; /*{ rate (0,1,2,3) = (0,1/4,1/2,1)
                             _                in wrong element }*/
  long return_value;

  /* with m_list[mon] do; */
  /* with c_list[mptr] do; */
  /* with cave[fy,fx] do; */
  if (xor((is_in(cave[MY(mon)][MX(mon)].fval, earth_set) ||
           is_in(cave[MY(mon)][MX(mon)].fval, pwall_set)),
          ((c_list[m_list[mon].mptr].cmove & 0x00000010) == 0))) {
    c_rate = (long)((c_list[m_list[mon].mptr].cmove & 0x00000300) / 256);
  } else {
    c_rate = 3;
  }

  if (c_rate == 3) {
    c_rate = 4; /* I wish I knew why they did this... rounding up? */
  }

  py_rate = player_flags.move_rate;

  if (cspeed > 0) {
    c_rate *= cspeed;
  } else {
    py_rate *= (2 - cspeed);
  }

  final_rate = c_rate / py_rate;

  if (((c_rate * turn) % py_rate) < (c_rate % py_rate)) {
    final_rate++;
  }

  /* { if player resting, max monster move = 1 } */
  if ((final_rate > 1) && (player_flags.rest > 0)) {
    return_value = 1;
  } else {
    return_value = final_rate;
  }

  return return_value;
}

void desc_remain(treas_rec *item_ptr) {
  /*{ Describe amount of item remaining...                  -RAK-   }*/

  char out_val[82];
  char out_val2[120];

  inven_temp.data = item_ptr->data;

  /* with inven_temp->data do; */

  inven_temp.data.number--;
  objdes(out_val, &inven_temp, true);
  sprintf(out_val2, "You have %s.", out_val);
  msg_print(out_val2);
}

void add_food(long num) {
  /*{ Add to the players food time                          -RAK-   }*/

  /* with player_flags do; */
  if ((player_flags).foodc < 0) {
    (player_flags).foodc = 0;
  }
  (player_flags).foodc += num;

  if ((player_flags).foodc > PLAYER_FOOD_FULL) {
    msg_print("You are full.");
  }

  if ((player_flags).foodc > PLAYER_FOOD_MAX) {
    msg_print("You're getting fat from eating so much.");
    (player_flags).foodc = PLAYER_FOOD_MAX;
    player_wt += trunc(player_wt * 0.1);
    if (player_wt > max_allowable_weight()) {
      msg_print("Oh no...  Now you've done it.");
      death = true;
      moria_flag = true;
      total_winner = false;
      strcpy(died_from, "gluttony");
    } else {
      switch (randint(3)) {
      case 1:
        msg_print("Buuurrrppppp !");
        break;
      case 2:
        msg_print("Remember, obesity kills.");
        break;
      case 3:
        msg_print("Your armor doesn't seem to fit too "
                  "well anymore.");
        break;
      }
    }
  }
}

boolean twall(long y, long x, long t1, long t2) {
  /*{ Tunneling through real wall: 10,11,12                 -RAK-   }*/
  /*{ Used by TUNNEL and WALL_TO_MUD                                }*/

  obj_set some_walls = {1, 2, 0};
  boolean return_value = false;

  /* with cave[y][x]. do; */
  if (t1 > t2) {
    if (next_to4(y, x, some_walls) > 0) {
      cave[y][x].fval = corr_floor2.ftval;
      cave[y][x].fopen = corr_floor2.ftopen;
    } else {
      cave[y][x].fval = corr_floor1.ftval;
      cave[y][x].fopen = corr_floor1.ftopen;
    }

    if (test_light(y, x)) {
      if (panel_contains(y, x)) {
        if (cave[y][x].tptr > 0) {
          msg_print("You have found something!");
        }
        lite_spot(y, x);
      }
    }

    cave[y][x].fm = false;
    cave[y][x].pl = false;
    return_value = true;
  }

  return return_value;
}

void desc_charges(treas_rec *item_ptr) {
  /*{ Describe number of remaining charges...               -RAK-   }*/

  char out_val[82];

  if (strstr(item_ptr->data.name, "^") == NULL) {
    sprintf(out_val, "You have %ld charges remaining.", item_ptr->data.p1);
    msg_print(out_val);
  }
}

boolean cast_spell(char prompt[82], treas_rec *item_ptr, long *sn, long *sc,
                   boolean *redraw) {
  /*{ Return spell number and failure chance                -RAK-   }*/

  unsigned long i2, i4;
  long i1, i3, num;
  spl_type aspell;
  boolean flag = false;

  i1 = num = 0;
  i2 = item_ptr->data.flags;
  i4 = item_ptr->data.flags2;

  do {
    i3 = bit_pos64(&i4, &i2) + 1;
    /*{ Avoid the cursed bit like the plague -DMF-   }*/
    if (i3 > 31) {
      i3--;
    }
    if (i3 > 0) {
      i3--;
      if ((C_magic_spell_level(i3) <= player_lev) &&
          (C_player_knows_spell(i3))) {
        aspell[i1++].splnum = i3;
        num = i1;
      } else {
        aspell[i1++].splnum = -1; /* leave gaps for unknown spells */
      }
    }

  } while (!((i2 == 0) && (i4 == 0)));

  if (num > 0) {
    flag = get_spell(aspell, num, sn, sc, prompt, redraw);
  }

  if (*redraw) {
    draw_cave();
  }

  return flag;
}

boolean d__get_dir(char prompt[82], long *dir, long *command_ptr, long *y,
                   long *x) {
  /*{ Prompts for a direction                               -RAK-   }*/

  char command;
  boolean flag = false;

  while (true) {
    if (!get_com(prompt, &command)) {
      reset_flag = true;
      return false;
    }

    switch (command) {
    case '1':
    case '2':
    case '3':
    case '4':
    case '6':
    case '7':
    case '8':
    case '9':
      *command_ptr = (long)command;
      flag = true;
      break;

    case 'b':
      *command_ptr = '1';
      flag = true;
      break;
    case 'j':
      *command_ptr = '2';
      flag = true;
      break;
    case 'n':
      *command_ptr = '3';
      flag = true;
      break;
    case 'h':
      *command_ptr = '4';
      flag = true;
      break;
    case 'l':
      *command_ptr = '6';
      flag = true;
      break;
    case 'y':
      *command_ptr = '7';
      flag = true;
      break;
    case 'k':
      *command_ptr = '8';
      flag = true;
      break;
    case 'u':
      *command_ptr = '9';
      flag = true;
      break;

    default:
      break;
    }

    if (flag) {
      *dir = *command_ptr - '0';
      move_dir(*dir, y, x);
      return true;
    }
  }
}

void d__quit() {
  /* this can be called from signalquit in io.c */

  char command;

  flush();
  if (get_com("Enter 'Q' to quit (and kill this character)", &command)) {
    switch (command) {
    case 'q':
    case 'Q':
      strcpy(died_from, "Ripe Old Age");
      moria_flag = true;
      death = true;
      break;
    default:
      break;
    }
  }

  erase_line(1, 1);
  refresh();
}

void dungeon() {
  ENTER(("dungeon", "d"));

  s1[0] = 0;
  s2[0] = 0;
  s3[0] = 0;
  s4[0] = 0;

  cur_inven = inventory_list;
  i_summ_count = 0;

  /*{ Check light status for setup          }*/
  if ((equipment[Equipment_light].p1 > 0) && (player_flags).light_on) {
    player_light = true;
  } else {
    player_light = false;
  }

  /*{ Check for a maximum level             }*/
  if (dun_level > player_max_lev) {
    player_max_lev = dun_level;
  }

  d__set_coords(&char_row, &char_col);

  /*{ Reset flags and initialize variables  }*/
  moria_flag = false;
  cave_flag = false;
  find_flag = false;
  search_flag = false;
  teleport_flag = false;
  mon_tot_mult = 0;
  cave[char_row][char_col].cptr = 1;
  old_chp = trunc(C_player_current_hp());
  old_cmana = trunc(player_cmana);

  /*{ Light up the area around character    }*/
  move_char(5);

  /*{ Light, but do not move critters       }*/
  creatures(false);

  /*{ Loop until dead, or new level 		}*/
  do {
    /*{ Check for the AST's			-DMF-	}*/
    /*if (want_trap) then dump_ast_mess; XXXX*/

    /*{ Increment turn counter			}*/
    turn++;

    if (((player_flags).speed > 0) ||
        ((turn % (labs((player_flags).speed) + 1)) == 0)) {
      water_move();
      adv_time(true); /*{ Increment game time }*/
    }

    d__sun_rise_or_set();

    if (turn % 10 == 1) {
      kick__kickout_player_if_time();
    }

    d__check_hours();

    /*{ Check for random wandering monster generation
     * }*/
    if (randint(MAX_MALLOC_CHANCE) == 1) {
      alloc_land_monster(floor_set, 1, MAX_SIGHT, false, false);
    }

    /*{ Screen may need updating, used mostly for stats}*/
    d__print_updated_stats();
    prt_equipment();
    d__check_light_status();

    /*{ Update counters and messages			}*/
    /* with player_flags do; */

    d__check_food();
    d__eat_food();
    d__regenerate();
    d__update_blindness();
    d__update_confusion();
    d__update_resist_lightning();
    d__update_monster_protect();
    d__update_fire_ring();
    d__update_frost_ring();
    d__update_blade_barrier();
    d__update_magic_protect();
    d__update_resist_petrfy();
    d__update_stealth();
    d__update_resist_charm();
    d__update_hoarse();
    d__update_fear();
    d__update_poison();
    d__update_fast();
    d__update_slow();
    d__update_resting();
    d__update_hallucinate();
    d__update_petrify();
    d__update_evil_protect();
    d__update_invulnerable();
    d__update_heroism();
    d__update_super_heroism();
    d__update_blessed();
    d__update_resist_heat();
    d__update_resist_cold();
    d__update_detect_invisible();
    d__update_infra_vision();
    d__update_word_of_recall();
    d__update_hit_points();
    C_check_passive_abilities();

    if ((player_flags.paralysis < 1) && /*{ Accept a command? }*/
        (player_flags.rest < 1) && (!(death))) {

      /*{ Accept a command and execute it }*/
      do {
        print_stat = 0;
        reset_flag = false;
        turn_counter++;
        if (turn_counter > 4000000) {
          turn_counter = 100000;
        }

        /*{ Random teleportation  }*/
        if (player_flags.teleport) {
          if (randint(100) == 1) {
            find_flag = false;
            teleport(40);
          }
        }

        if (!find_flag) {
          print_null(char_row, char_col);
          save_msg_flag = msg_flag;
          game_state = GS_GET_COMMAND;
          char command = inkey();
          game_state = GS_IGNORE_CTRL_C;
          if (save_msg_flag) {
            erase_line(msg_line, msg_line);
          }
          com_val = (long)command;
        }

        d__execute_command(&com_val);

      } while (!(!reset_flag || moria_flag)); /* end command do */

    } /* end if (able to do something) */
    /*{ Teleport?                     }*/
    if (teleport_flag) {
      teleport(100);
    }

    /*{ Move the creatures            }*/
    if (!moria_flag) {
      creatures(true);
    }

    /*{ Exit when moria_flag is set   }*/
  } while (!moria_flag);

  if (search_flag) {
    search_off(); /*{ Fixed "SLOW" bug; 06-11-86 RAK     }*/
  }

  LEAVE("dungeon", "d");
}
