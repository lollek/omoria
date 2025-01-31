#include "../c.h"
#include "../debug.h"
#include "../io.h"
#include "../misc.h"
#include "../model_class.h"
#include "../monsters.h"
#include "../pascal.h"
#include "../player.h"
#include "../random.h"
#include "../screen.h"
#include "../spells.h"
#include "../types.h"
#include "../variables.h"
#include <math.h>

static long attack_blows(const long weight, long *wtohit) {
  /*{ Weapon weight VS strength and dexterity               -RAK-   }*/

  const long max_wield = C_player_max_bulk() / 10;
  const int dex_mod = C_player_mod_from_stat(DEX);
  const int approx_str_stat = (10 + C_player_mod_from_stat(STR) * 2) * 10;

  long blows = 1;

  *wtohit = 0;

  /*{ make to-hit drop off gradually instead of being so abrupt -DCJ- }*/
  if (max_wield < weight / 100) {
    *wtohit = max_wield - weight / 100;
    return blows;
  }

  blows = 5 + dex_mod;
  blows = min(12, blows);
  blows = max(3, blows);

  const long lev_skill = C_class_melee_bonus(player_pclass) * (player_lev + 10);

  /*{warriors 100-500, paladin 80-400, priest 60-300, mage
   * 40-200}*/
  blows = trunc(0.8 + (float)blows / 3.0 + (float)lev_skill / 350.0);

  /*{usually 3 for 18+ dex, 5 max except 6 for high level
   * warriors}*/
  const long adj_weight =
      (long)((float)approx_str_stat / (float)(weight / 100) * 2.5);

  if (adj_weight < 1) {
    blows = 1;
  } else if (adj_weight < 2) {
    blows = blows / 3.00;
  } else if (adj_weight < 3) {
    blows = blows / 2.50;
  } else if (adj_weight < 5) {
    blows = blows / 2.00;
  } else if (adj_weight < 10) {
    blows = blows / 1.66;
  } else {
    blows = blows / 1.50;
  }

  return blows;
}

bool player_action_attack(const long y, const long x) {

  long blows;
  long tot_tohit;
  char m_name[82];
  bool mean_jerk_flag;
  bool backstab_flag;

  const obj_set mages_suck = {hafted_weapon, pole_arm, sword, maul, 0};
  const obj_set priests_suck = {hafted_weapon, pole_arm, dagger, sword, 0};
  const obj_set druids_suck = {hafted_weapon, pole_arm, sword, 0};
  const obj_set monks_suck = {hafted_weapon, pole_arm, maul, 0};
  const obj_set catch_this = {sling_ammo, bolt, arrow, 0};

  bool return_value = false;

  ENTER(("py_attack", "%d, %d", y, x));

  const long a_cptr = cave[y][x].cptr;
  const long a_mptr = m_list[a_cptr].mptr;

  if (player_pclass == C_ROGUE && m_list[a_cptr].csleep != 0) {
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
    tot_tohit += player_lev / 4;
  }

  /*{ Adjust weapons for class }*/
  if (player_pclass == C_WARRIOR) {
    tot_tohit += 1 + player_lev / 2;

  } else if (player_pclass == C_MAGE &&
             is_in(equipment[Equipment_primary].tval, mages_suck)) {
    tot_tohit -= 5;

  } else if (player_pclass == C_PRIEST &&
             is_in(equipment[Equipment_primary].tval, priests_suck)) {
    tot_tohit -= 4;

  } else if (player_pclass == C_DRUID &&
             is_in(equipment[Equipment_primary].tval, druids_suck)) {
    tot_tohit -= 4;

  } else if (player_pclass == C_MONK &&
             is_in(equipment[Equipment_primary].tval, monks_suck)) {
    tot_tohit -= 3;
  }

  /*{ Fix for arrows}*/
  if (is_in(equipment[Equipment_primary].tval, catch_this)) {
    blows = 1;
  }
  tot_tohit += player_ptohit;

  /*{ stopped from killing town creatures?? }*/
  if ((monster_templates[a_mptr].cmove & 0x00004000) == 0 ||
      randint(100) < -player_rep) {
    mean_jerk_flag = true;
  } else {
    mean_jerk_flag = get_yes_no("Are you sure you want to?");
  }

  /*{ Loop for number of blows, trying to hit the critter...        }*/
  if (mean_jerk_flag) {
    long crit_mult;
    long i3;
    /* with player_do; */
    do {
      char out_val[120];
      if (player_test_hit(player_bth, player_lev, tot_tohit, monster_templates[a_mptr].ac,
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
          i3 = tot_dam(&equipment[Equipment_primary], i3,
                       &monster_templates[a_mptr]);
          const bool is_sharp =
              equipment[Equipment_primary].tval != bow_crossbow_or_sling &&
              (equipment[Equipment_primary].flags2 & Sharp_worn_bit) != 0;
          crit_mult = critical_blow(equipment[Equipment_primary].weight,
                                    tot_tohit, is_sharp, false);
          if (backstab_flag) {
            i3 *= player_lev / 7 + 1;
          }
          if (player_pclass == C_WARRIOR) {
            i3 += player_lev / 3;
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
            py_bonuses(&inven_temp.data, -1);
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
