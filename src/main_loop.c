/* Here you should find the guts of the game */

#include <curses.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "ability.h"
#include "blow.h"
#include "effects.h"
#include "commands.h"
#include "configure.h"
#include "fighting.h"
#include "constants.h"
#include "creature.h"
#include "death.h"
#include "debug.h"
#include "desc.h"
#include "eat.h"
#include "fighting_ranged.h"
#include "help.h"
#include "inven.h"
#include "io.h"
#include "kickout.h"
#include "magic.h"
#include "menu.h"
#include "misc.h"
#include "model_class.h"
#include "monsters.h"
#include "pascal.h"
#include "dungeon/light.h"
#include "player.h"
#include "player/hunger.h"
#include "player/regeneration.h"
#include "player_action/move.h"
#include "player_action/attack.h"
#include "player_action/search.h"
#include "potions.h"
#include "random.h"
#include "save.h"
#include "screen.h"
#include "scrolls.h"
#include "spells.h"
#include "staffs.h"
#include "stores.h"
#include "term.h"
#include "traps.h"
#include "types.h"
#include "variables.h"
#include "wands.h"
#include "wizard.h"

#include "main_loop.h"

void C_print_known_spells();

#define OBJ_LAMP_MAX 15000 /*{ Maximum amount that lamp can be filled} */

static long old_chp;              /* { Detect change         } */
static long old_cmana;            /* { Detect change         } */
static boolean save_msg_flag;     /* { Msg flag after INKEY  } */
static char s1[70];               /* { Summon item strings   } */
static char s2[70];               /* { Summon item strings   } */
static char s3[70];               /* { Summon item strings   } */
static char s4[70];               /* { Summon item strings   } */
static long i_summ_count;         /* { Summon item count	   } */


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
  if (turn % 100 != 1)
    return;
  if (!kick__should_kickout())
    return;

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
        dungeon_light_move(char_row, char_col, char_row, char_col);
      } else if (equipment[Equipment_light].p1 < 40) {
        if (randint(5) == 1) {
          if (find_flag) {
            find_flag = false;
            dungeon_light_move(char_row, char_col, char_row, char_col);
          }
          msg_print("Your light is growing faint.");
        }
      }
    } else {
      (player_flags).light_on = false;
      player_light = false;
      find_flag = false;
      dungeon_light_move(char_row, char_col, char_row, char_col);
    }
  } else if ((equipment[Equipment_light].p1 > 0) && (player_flags).light_on) {
    equipment[Equipment_light].p1--;
    player_light = true;
    dungeon_light_move(char_row, char_col, char_row, char_col);
  }

  LEAVE("d__check_light_status", "d");
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
  float const regen_amount = player_regeneration_get_amount();
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
      player_action_move(5);
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
        player_action_move(5);
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
          player_action_move(5);
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
        player_action_move(5);
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
        player_action_move(5);
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
        player_action_move(5);
      }
    }
    (player_flags).fast--;
    if ((player_flags).fast == 0) {
      (player_flags).status &= ~IS_HASTED;
      msg_print("You feel yourself slow down.");
      change_speed(+1);
      if (find_flag) {
        player_action_move(5);
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
        player_action_move(5);
      }
    }
    (player_flags).slow--;
    if ((player_flags).slow == 0) {
      (player_flags).status &= ~IS_SLOW;
      msg_print("You feel yourself speed up.");
      change_speed(-1);
      if (find_flag) {
        player_action_move(5);
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
        player_action_move(5);
      }
      msg_print("Your skin turns into steel!");
      player_pac += 100;
      player_dis_ac += 100;
    }
    (player_flags).invuln--;
    if ((player_flags).invuln == 0) {
      (player_flags).status &= ~IS_INVULNERABLE;
      if (find_flag) {
        player_action_move(5);
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
        player_action_move(5);
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
        player_action_move(5);
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
        player_action_move(5);
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
        player_action_move(5);
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
        player_action_move(5);
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
        player_action_move(5);
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

      if (player_action_attack(y, x)) {
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
      } else if (player_has_no_light()) {
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
          player_action_move(tdir);
          lite_spot(y, x);
          prt_stat_block();
        } else if (randint(tot) > 5) {
          msg_print("You failed to disarm the trap.");
        } else {
          msg_print("You set the trap off!");
          player_action_move(tdir);
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
          player_action_search(char_row, char_col, C_player_curr_search_skill());
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
    death_by_quitting();
    reset_flag = true;
    break;

  case CTRL_A:
    reset_flag = C_select_ability();
    draw_cave();
    break;

  case CTRL_B:
    find_flag = true;
    player_action_move(1);
    break;

  case CTRL_H:
    find_flag = true;
    player_action_move(4);
    break;
  case CTRL_I:
    blow();
    break;
  case CTRL_J:
    find_flag = true;
    player_action_move(2);
    break;
  case CTRL_K:
    find_flag = true;
    player_action_move(8);
    break;
  case CTRL_L:
    find_flag = true;
    player_action_move(6);
    break;

  case CTRL_N:
    find_flag = true;
    player_action_move(3);
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
    player_action_move(9);
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
    player_action_move(7);
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
      player_action_move(1);
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
      player_action_move(4);
      break;

    case 'j':
      player_action_move(2);
      break;
    case 'k':
      player_action_move(8);
      break;
    case 'l':
      player_action_move(6);
      break;
    case 'm':
      moria_help("");
      draw_cave();
      reset_flag = true;
      break;
    case 'n':
      player_action_move(3);
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
      player_action_move(9);
      break;

    case 'w':
      moria_help("Adventuring Weapons Weapon_List");
      draw_cave();
      reset_flag = true;
      break;

    case 'y':
      player_action_move(7);
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
    player_action_move(5);
    usleep(10);
    flush();
    break;

  case 'A':
    throw_something();
    break;
  case 'B':
    find_flag = true;
    player_action_move(1);
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
    player_action_move(4);
    break;
  case 'I': /* Selected inv */
    reset_flag = true;
    if (inven_command('I', &trash_ptr, "")) {
      draw_cave();
    }
    break;
  case 'J':
    find_flag = true;
    player_action_move(2);
    break;
  case 'K':
    find_flag = true;
    player_action_move(8);
    break;
  case 'L':
    find_flag = true;
    player_action_move(6);
    break;
  case 'N':
    find_flag = true;
    player_action_move(3);
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
    player_action_move(9);
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
        dungeon_light_move(char_row, char_col, char_row, char_col);
      } else {
        msg_print("Your light has gone out!");
      }
    } else {
      msg_print("You are not carrying a light.");
    }
    break;
  case 'Y':
    find_flag = true;
    player_action_move(7);
    break;
  case 'Z':
    use_staff();
    break;

  case 'a':
    shoot_something();
    break;
  case 'b':
    player_action_move(1);
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
    player_action_move(4);
    break;
  case 'i': /* Inventory */
    reset_flag = true;
    if (inven_command('i', &trash_ptr, "")) {
      draw_cave();
    }
    break;
  case 'j':
    player_action_move(2);
    break;
  case 'k':
    player_action_move(8);
    break;
  case 'l':
    player_action_move(6);
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
    player_action_move(3);
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
      player_action_search(char_row, char_col, C_player_curr_search_skill());
    }
    break;
  case 't': /* take off */
    reset_flag = true;
    if (inven_command('t', &trash_ptr, "")) {
      draw_cave();
    }
    break;
  case 'u':
    player_action_move(9);
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
    player_action_move(7);
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
  player_action_move(5);

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

    player_hunger_recalculate();
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
