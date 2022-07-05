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
#include "screen.h"
#include "player_action/move.h"

/* P_MISC */
int64_t player_xtr_wgt = 0;
int64_t player_account = 0;
money_type player_money = {0, 0, 0, 0, 0, 0, 0};
game_time_type player_birth = {0, 0, 0, 0, 0};
game_time_type player_cur_age = {0, 0, 0, 0, 0};
time_type player_play_tm = {0, 0, 0, 0, 0, 0, 0};
char player_name[82] = "";
char player_race[82] = "";
char player_sex[82] = "";
char player_tclass[82] = "";
int64_t player_max_exp = 0;
int64_t player_exp = 0;
int64_t player_rep = 0;
uint16_t player_age = 0;
uint16_t player_ht = 0;
uint16_t player_wt = 0;
uint16_t player_lev = 0;
uint16_t player_max_lev = 0;
int16_t player_fos = 0;
int16_t player_bth = 0;
int16_t player_bthb = 0;
int16_t player_mana = 0;
int16_t player_ptohit = 0;
int16_t player_ptodam = 0;
int16_t player_pac = 0;
int16_t player_ptoac = 0;
int16_t player_dis_th = 0;
int16_t player_dis_td = 0;
int16_t player_dis_ac = 0;
int16_t player_dis_tac = 0;
int16_t player_disarm = 0;
int16_t player_save = 0;
int16_t player_sc = 0;
enum class_t player_pclass = 0;
uint8_t player_prace = 0;
int16_t player_stl = 0;
float player_expfact = 0;
float player_cmana = 0;
char player_history[][82] = {"", "", "", "", ""};
boolean player_cheated = false;
int64_t player_mr = 0;
uint8_t player_quests = 0;
uint16_t player_cur_quest = 0;
time_t player_creation_time = 0;
int64_t player_claim_check = 0;
int64_t player_uid = 0;

/*	{ Following are player variables				} */
p_flags player_flags = {
    false, false,
    0,     0,
    0,     0,
    0,     7500,
    2,     0,
    0,     0,
    false, 0,
    0,     0,
    0,     0,
    0,     0,
    0,     0,
    0,     0,
    0,     0,
    0,     0,
    0,     0,
    0,     0,
    0,     false,
    false, false,
    false, false,
    false, false,
    false, false,
    false, false,
    false, {false, false, false, false, false, false}, /* sustain */
    false, 0,
    0,     0,
    0,     0,
    0,     0,
    0,     0,
    0,     false,
    false, false,
    false};

void search_off() {
  search_flag = false;
  find_flag = false;
  player_action_move(5);
  change_speed(-1);
  player_flags.status &= ~IS_SEARCHING;
  prt_search();
}

void search_on() {
  /*{ Search Mode enhancement                               -RAK-   }*/

  search_flag = true;
  change_speed(+1);
  player_flags.status |= IS_SEARCHING;
  prt_search();
  /* with player_flags do; */
}

void rest_off() {
  player_flags.rest = 0;
  player_flags.status &= ~IS_RESTING;
  player_flags.resting_till_full = false;
  if (msg_flag) {
    erase_line(1, 1);
  }
  prt_rest();
}

void regenmana(float percent) {
  /*{ Regenerate mana points		-RAK-	}*/

  player_cmana += player_mana * percent + 0.0080;
}

void take_hit(long damage, char hit_from[82]) {
  /*{ Decreases players hit points and sets death flag if neccessary}*/

  ENTER(("take_hit", "%d, %s", damage, hit_from));

  if (player_flags.invuln > 0) {
    damage = 0;
  }

  C_player_modify_current_hp(-damage);

  if (search_flag) {
    search_off();
  }

  if (player_flags.rest > 0) {
    rest_off();
  }

  flush();

  if (C_player_current_hp() <= -1) {
    if (!death) {
      /*{ Hee, hee... Ain't I mean?     }*/
      death = true;
      strcpy(died_from, hit_from);
      total_winner = false;
    }
    moria_flag = true;
  } else {
    prt_stat_block();
  }

  LEAVE("take_hit", "");
}

void change_speed(long num) {
  long i1;

  player_flags.speed += num;

  for (i1 = muptr; i1 != 0; i1 = m_list[i1].nptr) {
    m_list[i1].cspeed += num;
  }
}

void py_bonuses(treasure_type *tobj, long factor) {
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