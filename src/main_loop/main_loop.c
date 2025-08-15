#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <stdbool.h>

#include "../ability.h"
#include "../c.h"
#include "../constants.h"
#include "../creature.h"
#include "../death.h"
#include "../debug.h"
#include "../dungeon/light.h"
#include "../generate_map/generate_map.h"
#include "../generate_monster/generate_monster.h"
#include "../io.h"
#include "../kickout.h"
#include "../misc.h"
#include "../pascal.h"
#include "../player.h"
#include "../player/hunger.h"
#include "../player/regeneration.h"
#include "../player_action.h"
#include "../random.h"
#include "../screen.h"
#include "../spells.h"
#include "../stores.h"
#include "../term.h"
#include "../types.h"
#include "../variables.h"
#include "command.h"

#include "main_loop.h"

static long old_chp;          /* { Detect change         } */
static long old_cmana;        /* { Detect change         } */
static bool save_msg_flag;    /* { Msg flag after INKEY  } */

static void d__set_coords(long *c_row, long *c_col) {
  /*{ Set up the character co-ords          }*/
  if (*c_row == -1 || *c_col == -1) {
    do {
      *c_row = randint(cur_height);
      *c_col = randint(cur_width);

      /*      *c_row = 8;*/
      /*      *c_col = 20;*/
    } while (!(cave[*c_row][*c_col].fopen &&
               cave[*c_row][*c_col].cptr == 0 &&
               cave[*c_row][*c_col].tptr == 0 &&
               !is_in(cave[*c_row][*c_col].fval, water_set)));
  }
}

static void d__sun_rise_or_set(void) {

  /*{ Sunrise and Sunset on town level	  -KRC-	}*/
  /* with player_cur_age do; */
  if (dun_level == 0) {
    if (player_cur_age.hour == 6 && player_cur_age.secs == 0) {
      for (long i1 = 1; i1 <= cur_height; i1++) {
        for (long i2 = 1; i2 <= cur_width; i2++) {
          cave[i1][i2].pl = true;
        }
      }
      store_maint();
      draw_cave();
    } else if (player_cur_age.hour == 18 && player_cur_age.secs == 0) {
      for (long i1 = 1; i1 <= cur_height; i1++) {
        for (long i2 = 1; i2 <= cur_width; i2++) {
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

static void d__check_hours(void) {
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

static void d__print_updated_stats(void) {
  if (print_stat != 0) {
    prt_stat_block();
  }
}

static void d__check_light_status(void) {
  /*{ Check light status                            }*/
  /* with equipment[Equipment_light] do; */
  ENTER(("d__check_light_status", "d"));
  if (player_light) {
    if (equipment[Equipment_light].p1 > 0 && player_flags.light_on) {
      equipment[Equipment_light].p1--;
      if (equipment[Equipment_light].p1 == 0) {
        msg_print("Your light has gone out!");
        player_flags.light_on = false;
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
      player_flags.light_on = false;
      player_light = false;
      find_flag = false;
      dungeon_light_move(char_row, char_col, char_row, char_col);
    }
  } else if (equipment[Equipment_light].p1 > 0 && player_flags.light_on) {
    equipment[Equipment_light].p1--;
    player_light = true;
    dungeon_light_move(char_row, char_col, char_row, char_col);
  }

  LEAVE("d__check_light_status", "d");
}

static void d__eat_food(void) {
  /*{ Food consumtion       }*/
  /*{ Note: Speeded up characters really burn up the food!  }*/

  player_flags.food_digested = BASE_FOOD_DIGESTED;

  if (player_flags.status & IS_RESTING) {
    player_flags.food_digested -= 1;
  }
  if (player_flags.slow_digest) {
    player_flags.food_digested -= 1;
  }
  if (player_flags.status & IS_SEARCHING) {
    player_flags.food_digested += 1;
  }
  if (player_flags.regenerate) {
    player_flags.food_digested += 3;
  }

  if (player_flags.food_digested < 0) {
    player_flags.food_digested = 0;
  }

  if (player_flags.speed < 0) {
    player_flags.foodc -= player_flags.speed * player_flags.speed +
                            player_flags.food_digested;
  } else {
    player_flags.foodc -= player_flags.food_digested;
  }
}

static void d__regenerate(void) {
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

static void d__update_blindness(void) {
  /*{ Blindness             }*/
  if (player_flags.blind > 0) {
    if ((IS_BLIND & player_flags.status) == 0) {
      player_flags.status |= IS_BLIND;
      prt_map();
      prt_blind();
      if (search_flag) {
        search_off();
      }
    }
    player_flags.blind--;
    if (player_flags.blind == 0) {
      player_flags.status &= ~IS_BLIND;
      prt_blind();
      prt_map();
      msg_print("The veil of darkness lifts.");
      player_action_move(5);
    }
  }
}

static void d__update_confusion(void) {
  /*{ Confusion             }*/
  if (player_flags.confused > 0) {
    if ((IS_CONFUSED & player_flags.status) == 0) {
      player_flags.status |= IS_CONFUSED;
      prt_confused();
    }
    player_flags.confused--;
    if (player_flags.confused == 0) {
      player_flags.status &= ~IS_CONFUSED;
      prt_confused();
      msg_print("You feel less confused now.");
      if (find_flag) {
        player_action_move(5);
      }
    }
  }
}

static void d__update_resist_lightning(void) {
  /*{ Resist Lightning }*/
  if (player_flags.resist_lght > 0) {
    player_flags.resist_lght--;
  }
}

static void d__update_monster_protect(void) {
  /*{ Protection from Monsters }*/
  if (player_flags.protmon > 0) {
    player_flags.protmon--;
  }
}

static void d__update_fire_ring(void) {
  /*{ Ring of Fire }*/
  if (player_flags.ring_fire > 0) {
    msg_print("Flames arise!");
    explode(SE_FIRE, char_row, char_col, 20 + randint(20), "Ring of Fire");
    player_flags.ring_fire--;
  }
}

static void d__update_frost_ring(void) {

  /*{ Ring of Frost }*/
  if (player_flags.ring_ice > 0) {
    explode(SE_COLD, char_row, char_col, 10 + randint(20), "Ring of Frost");
    player_flags.ring_ice--;
  }
}

static void d__update_blade_barrier(void) {

  /*{ Blade Barrier }*/
  if (player_flags.blade_ring > 0) {
    explode(SE_NULL, char_row, char_col, 12 + randint(player_lev),
            "Blade Barrier");
    player_flags.blade_ring--;
  }
}

static void d__update_magic_protect(void) {
  /*{ Magic protection }*/
  if (player_flags.magic_prot > 0) {
    if ((IS_MAGIC_PROTECED & player_flags.status) == 0) {
      player_flags.status |= IS_MAGIC_PROTECED;
      player_save += 25;
    }
    player_flags.magic_prot--;
    if (player_flags.magic_prot == 0) {
      player_save -= 25;
      player_flags.status &= ~IS_MAGIC_PROTECED;
    }
  }
}

static void d__update_resist_petrfy(void) {
  /*{Timed resist Petrification}*/
  if (player_flags.resist_petri > 0) {
    player_flags.resist_petri--;
  }
}

static void d__update_stealth(void) {
  /*{ Timed Stealth    }*/
  if (player_flags.temp_stealth > 0) {
    if ((IS_STEALTHY & player_flags.status) == 0) {
      player_flags.status |= IS_STEALTHY;
      player_stl += 3;
    }
    player_flags.temp_stealth--;
    if (player_flags.temp_stealth == 0) {
      player_flags.status &= ~IS_STEALTHY;
      player_stl -= 3;
      msg_print("The monsters can once again detect you with "
                "ease.");
    }
  }
}

static void d__update_resist_charm(void) {
  /*{ Resist Charm }*/
  if (player_flags.free_time > 0) {
    if ((IS_CHARM_PROOF & player_flags.status) == 0) {
      player_flags.status |= IS_CHARM_PROOF;
      player_flags.free_time--;
      if (player_flags.free_time == 0) {
        player_flags.status &= ~IS_CHARM_PROOF;
        if (find_flag) {
          player_action_move(5);
        }
      }
    }
  }
}

static void d__update_hoarse(void) {
  /*{ Hoarse                }*/
  if (player_flags.hoarse > 0) {
    player_flags.hoarse--;
    if (player_flags.hoarse == 0) {
      msg_print("You feel your voice returning.");
    }
  }
}

static void d__update_fear(void) {
  /*{ Afraid                }*/
  if (player_flags.afraid > 0) {
    if ((IS_AFRAID & player_flags.status) == 0) {
      if (player_flags.shero + player_flags.hero > 0) {
        player_flags.afraid = 0;
      } else {
        player_flags.status |= IS_AFRAID;
        prt_afraid();
      }
    } else if (player_flags.shero + player_flags.hero > 0) {
      player_flags.afraid = 1;
    }

    player_flags.afraid--;
    if (player_flags.afraid == 0) {
      player_flags.status &= ~IS_AFRAID;
      prt_afraid();
      msg_print("You feel bolder now.");
      if (find_flag) {
        player_action_move(5);
      }
    }
  }
  if (player_flags.afraid < 0) {
    player_flags.afraid =
        0; /* fix when getting hit with fear while shero or hero */
  }
}

static void d__update_poison(void) {
  /*{ Poisoned              }*/
  if (player_flags.poisoned > 0) {
    if ((IS_POISONED & player_flags.status) == 0) {
      player_flags.status |= IS_POISONED;
      prt_poisoned();
    }
    player_flags.poisoned--;
    if (player_flags.poisoned == 0) {
      player_flags.status &= ~IS_POISONED;
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
        if (turn % 2 == 0) {
          take_hit(1, "poison");
        }
        break;
      case 4:
      case 5:
        if (turn % 3 == 0) {
          take_hit(1, "poison");
        }
        break;
      case 6:
        if (turn % 4 == 0) {
          take_hit(1, "poison.");
        }
        break;
      } /* end switch */
    }
  }
}

static void d__update_fast(void) {

  /*{ Fast                  }*/
  if (player_flags.fast > 0) {
    if ((IS_HASTED & player_flags.status) == 0) {
      player_flags.status |= IS_HASTED;
      msg_print("You feel yourself moving faster.");
      change_speed(-1);
      if (find_flag) {
        player_action_move(5);
      }
    }
    player_flags.fast--;
    if (player_flags.fast == 0) {
      player_flags.status &= ~IS_HASTED;
      msg_print("You feel yourself slow down.");
      change_speed(+1);
      if (find_flag) {
        player_action_move(5);
      }
    }
  }
}

static void d__update_slow(void) {

  /*{ Slow                  }*/
  if (player_flags.slow > 0) {
    if ((IS_SLOW & player_flags.status) == 0) {
      player_flags.status |= IS_SLOW;
      msg_print("You feel yourself moving slower.");
      change_speed(+1);
      if (find_flag) {
        player_action_move(5);
      }
    }
    player_flags.slow--;
    if (player_flags.slow == 0) {
      player_flags.status &= ~IS_SLOW;
      msg_print("You feel yourself speed up.");
      change_speed(-1);
      if (find_flag) {
        player_action_move(5);
      }
    }
  }
}

static void bother(const long num) {
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

static void d__update_resting(void) {
  /*{ Resting is over?      }*/
  if (player_flags.rest > 0) {
    /*{ Hibernate every 20 iterations so that process does  }*/
    /*{ not eat up system...                                }*/
    /*{ NOTE: Remove comments for VMS version 4.0 or greater}*/
    /*{       INKEY_DELAY takes care of hibernation for     }*/
    /*{       VMS 3.7 or less                               }*/
    if (player_flags.rest % 20 == 1) {
      usleep(500);
      if ((equipment[Equipment_primary].flags2 & Soul_Sword_worn_bit) != 0) {
        bother(randint(10));
        player_flags.rest = 1;
        player_flags.resting_till_full = false;
      }
    }
    player_flags.rest--;
    /*{ Test for any key being hit to abort rest.  Also,    }*/
    /*{ this will do a PUT_QIO which updates the screen...  }*/
    /*{ One more side benifit; since inkey_delay hibernates }*/
    /*{ small amount before executing, this makes resting   }*/
    /*{ less CPU intensive...                               }*/
    char command;
    inkey_delay(&command);
    /*if (want_trap) { dump_ast_mess; XXXX}*/
    if (player_flags.rest == 0) { /*{ Resting over          }*/
      if (player_flags.resting_till_full &&
          (player_cmana < player_mana ||
           C_player_current_hp() < C_player_max_hp())) {
        player_flags.rest = 20;
        turn_counter += player_flags.rest;
      } else {
        rest_off();
      }
    } else if (command != 0) { /*{ Resting aborted       }*/
      rest_off();
    }
  }
}

static void d__update_hallucinate(void) {
  /*{ Hallucinating?  (Random characters appear!)}*/
  if (player_flags.image > 0) {
    player_flags.image--;
    if (player_flags.image == 0) {
      draw_cave();
    }
  }
}

static void d__update_petrify(void) {
  /*{  Petrification wears off slowly  } */
  if (turn % 100 == 0) {
    /* with player_flags do; */
    if (player_flags.petrification > 100) {
      player_flags.petrification--;
    }
  }

  /* not sure what this did, but it was commented out... */
  /*
  if ((player_flags).speed > 0) and (paral_init = speed_paral) then
                   paralysis := paralysis + paral_init + 1;
  */

  /*{ Paralysis             }*/
  if (player_flags.paralysis > 0) {
    player_flags.paralysis--;
    if (player_flags.rest > 0) {
      rest_off();
    }
    if (search_flag &&
        player_flags.paralysis > player_flags.paral_init) {
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

static void d__update_evil_protect(void) {
  /*{ Protection from evil counter}*/
  if (player_flags.protevil > 0) {
    player_flags.protevil--;
  }
}

static void d__update_invulnerable(void) {
  /*{ Invulnerability        }*/
  if (player_flags.invuln > 0) {
    if ((IS_INVULNERABLE & player_flags.status) == 0) {
      player_flags.status |= IS_INVULNERABLE;
      if (find_flag) {
        player_action_move(5);
      }
      msg_print("Your skin turns into steel!");
      player_pac += 100;
      player_dis_ac += 100;
    }
    player_flags.invuln--;
    if (player_flags.invuln == 0) {
      player_flags.status &= ~IS_INVULNERABLE;
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

static void d__update_heroism(void) {
  /*{ Heroism       }*/
  if (player_flags.hero > 0) {
    if ((IS_HERO & player_flags.status) == 0) {
      player_flags.status |= IS_HERO;
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
    player_flags.hero--;
    if (player_flags.hero == 0) {
      player_flags.status &= ~IS_HERO;
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

static void d__update_super_heroism(void) {
  /*{ Super Heroism }*/
  if (player_flags.shero > 0) {
    if ((IS_SUPER_HERO & player_flags.status) == 0) {
      player_flags.status |= IS_SUPER_HERO;
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
    player_flags.shero--;
    if (player_flags.shero == 0) {
      player_flags.status &= ~IS_SUPER_HERO;
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

static void d__update_blessed(void) {
  /*{ Blessed       }*/
  if (player_flags.blessed > 0) {
    if ((IS_BLESSED & player_flags.status) == 0) {
      player_flags.status |= IS_BLESSED;
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
    player_flags.blessed--;
    if (player_flags.blessed == 0) {
      player_flags.status &= ~IS_BLESSED;
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

static void d__update_resist_heat(void) {
  /*{ Resist Heat   }*/
  if (player_flags.resist_heat > 0) {
    player_flags.resist_heat--;
  }
}

static void d__update_resist_cold(void) {
  /*{ Resist Cold   }*/
  if (player_flags.resist_cold > 0) {
    player_flags.resist_cold--;
  }
}

static void d__update_detect_invisible(void) {
  /*{ Detect Invisible      }*/
  if (player_flags.detect_inv > 0) {
    if ((IS_ABLE_TO_SEE_INVIS & player_flags.status) == 0) {
      player_flags.status |= IS_ABLE_TO_SEE_INVIS;
      player_flags.see_inv = true;
    }
    player_flags.detect_inv--;
    if (player_flags.detect_inv == 0) {
      player_flags.status &= ~IS_ABLE_TO_SEE_INVIS;
      player_flags.see_inv = false;
      py_bonuses(&blank_treasure, 0);
    }
  }
}

static void d__update_infra_vision(void) {
  /*{ Timed infra-vision    }*/
  if (player_flags.tim_infra > 0) {
    if ((IS_ABLE_TO_SEE_HEAT & player_flags.status) == 0) {
      player_flags.status |= IS_ABLE_TO_SEE_HEAT;
      player_flags.see_infra++;
    }
    player_flags.tim_infra--;
    if (player_flags.tim_infra == 0) {
      player_flags.status &= ~IS_ABLE_TO_SEE_HEAT;
      player_flags.see_infra--;
      msg_print("Your eyes stop tingling.");
    }
  }
}

static void d__update_word_of_recall(void) {
  /*{ Word-of-Recall  Note: Word-of-Recall is a delayed action      }*/
  if (player_flags.word_recall > 0) {
    if (player_flags.word_recall == 1) {
      if (dun_level > 0) {
        msg_print("You feel yourself yanked upwards!");
        dun_level = 0;
      } else if (player_max_lev > 0) {
        msg_print("You feel yourself yanked downwards!");
        dun_level = player_max_lev;
      }
      moria_flag = true;
      player_flags.paralysis++;
      player_flags.word_recall = 0;
    } else {
      player_flags.word_recall--;
    }
  }
}

static void d__update_hit_points(void) {
  /*{ Check hit points for adjusting...                     }*/
  /* with player_do; */
  ENTER(("d__update_hit_points", "d"));

  if (!find_flag) {
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


/**
 * water_move_item() - I sense a patter about water moves...
 */
bool water_move_item(__attribute__((unused)) long row,
                        __attribute__((unused)) long col,
                        __attribute__((unused)) long num) {
  return true;
}

bool water_move(void) {
  const bool flag = false;

  // water_move_player();

  for (long i = muptr; i != 0; i = m_list[i].nptr) {
    m_list[i].moved = false;
  }

  for (long i = muptr; i != 0; i = m_list[i].nptr) {
    // flag = water_move_creature(i);
  }

  return flag;
}

void main_loop__0(void) {
  ENTER(("main_loop", "d"));

  cur_inven = inventory_list;

  /*{ Check light status for setup          }*/
  if (equipment[Equipment_light].p1 > 0 && player_flags.light_on) {
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
    turn++;

    if (player_flags.speed > 0 ||
        turn % (labs(player_flags.speed) + 1) == 0) {
      water_move();
      adv_time(true); /*{ Increment game time }*/
    }

    d__sun_rise_or_set();

    if (turn % 10 == 1) {
      kick__kickout_player_if_time();
    }

    d__check_hours();

    // Maybe spawn a new monster
    if (randint(MAX_MALLOC_CHANCE) == 1) {
      generate_land_monster(floor_set, 1, MAX_SIGHT, false);
    }

    //{ Screen may need updating, used mostly for stats
    d__print_updated_stats();
    prt_equipment();
    d__check_light_status();

    // Update counters and messages
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

    const bool player_is_able_to_move_this_round =
        player_flags.paralysis < 1 && player_flags.rest < 1 && !death;
    if (player_is_able_to_move_this_round) {

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
          const char command = inkey();
          game_state = GS_IGNORE_CTRL_C;
          if (save_msg_flag) {
            erase_line(msg_line, msg_line);
          }
          com_val = (long)command;
        }

        command(&com_val);

      } while (!(!reset_flag || moria_flag)); /* end command do */
    }
    if (teleport_flag) {
      teleport(100);
    }

    /*{ Move the creatures            }*/
    if (!moria_flag) {
      creatures(true);
    }
  } while (!moria_flag);

  if (search_flag) {
    search_off(); /*{ Fixed "SLOW" bug; 06-11-86 RAK     }*/
  }

  LEAVE("main_loop", "d");
}

int main_loop(void) {
  /* Loop till dead, or exit */
  while (true) {
    /* Dungeon logic */
    main_loop__0();
    /* Character gets buried, or respawns */
    if (death) {
      upon_death();
      if (death) {
#if DO_DEBUG
        memory_error(0, "DEBUG_ON_EXIT");
#endif
        return 0;
      }
    }

    generate_map();
  }
}
