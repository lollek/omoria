#include "../player.h"
#include "../types.h"
#include "../variables.h"
#include "../debug.h"
#include "../misc.h"
#include "../random.h"
#include "../pascal.h"
#include "../spells.h"
#include "../monsters.h"
#include "../screen.h"

boolean player_action_attack(long y, long x) {

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
