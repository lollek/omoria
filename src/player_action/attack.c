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

static bool attack_lands_on_monster(const long a_cptr, const monster_template_t* const monster_template,
                                    const long tot_tohit, bool const is_backstab, bool const is_missile) {
  bool monster_is_alive = false;
  long damage;

  if (equipment[Equipment_primary].tval > 0) {
    damage = damroll(equipment[Equipment_primary].damage);
    damage = tot_dam(&equipment[Equipment_primary], damage, monster_template);
    const bool is_sharp =
        equipment[Equipment_primary].tval != bow_crossbow_or_sling &&
        (equipment[Equipment_primary].flags2 & Sharp_worn_bit) != 0;
    const long crit_mult = critical_blow(equipment[Equipment_primary].weight,
                                         tot_tohit, is_sharp, false);
    if (is_backstab) {
      damage *= player_lev / 7 + 1;
    }
    if (player_pclass == C_WARRIOR) {
      damage += player_lev / 3;
    }
    damage += (damage + 5) * crit_mult;

  } else { /*{ Bare hands!?  }*/
    if (player_pclass == C_MONK) {
      damage = randint((4 + 2 * player_lev) / 3);
      const long crit_mult = critical_blow(12000, 0, false, false);
      if (randint(crit_mult + 2) > 2) {
        do_stun(a_cptr, -10, 2);
      }
      damage += (damage + 5) * crit_mult;
    } else {
      damage = damroll(bare_hands);
      const long crit_mult = critical_blow(1, 0, false, false);
      damage += (damage + 5) * crit_mult;
    }
  }

  damage += player_ptodam;
  if (damage < 0) {
    damage = 0;
  }
  /*{ See if we done it in... }*/
  if (mon_take_hit(a_cptr, damage) > 0) {
    monster_is_alive = false;
  } else {
    monster_is_alive = true;
  }

  if (is_missile) {
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

  return monster_is_alive;
}

static bool execute_all_attacks(long number_of_attacks, long const a_cptr,
                                long const a_mptr, const long tot_tohit, bool const is_backstab,
                                bool const is_missile) {
  char m_name[82];
  find_monster_name(m_name, a_cptr, false);
  const monster_template_t *monster_template = &monster_templates[a_mptr];

  bool monster_is_hit_but_alive = false;
  for (; number_of_attacks >= 1; number_of_attacks--) {
    bool const did_hit = player_test_hit(player_bth, player_lev, tot_tohit,
                                         monster_template->ac, false);
    if (!did_hit) {
      switch (randint(10)) {
      case 1:
        msg_printf("The %s evades your attack.", m_name);
      case 2:
        msg_printf("You barely miss %s.", m_name);
      default:
        msg_printf("You miss %s.", m_name);
        break;
      }
      continue;
    }

    if (is_backstab) {
      msg_printf("You backstab %s!", m_name);
    } else {
      msg_printf("You hit %s.", m_name);
    }

    bool const monster_is_alive = attack_lands_on_monster(
        a_cptr, monster_template, tot_tohit, is_backstab, is_missile);
    if (!monster_is_alive) {
      msg_printf("You have slain %s.", m_name);
      return false;
    }
    monster_is_hit_but_alive = true;
  }
  return monster_is_hit_but_alive;
}

bool player_action_attack(const long y, const long x) {

  long blows;
  long tot_tohit;
  bool player_is_mean_jerk;
  bool is_backstab;

  ENTER(("py_attack", "%d, %d", y, x));

  const long a_cptr = cave[y][x].cptr;
  const long a_mptr = m_list[a_cptr].mptr;

  if (player_pclass == C_ROGUE && m_list[a_cptr].csleep != 0) {
    is_backstab = true;
  } else {
    is_backstab = false;
  }
  m_list[a_cptr].csleep = 0;


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

  if (is_backstab) {
    tot_tohit += player_lev / 4;
  }

  /*{ Adjust weapons for class }*/
  if (player_pclass == C_WARRIOR) {
    tot_tohit += 1 + player_lev / 2;
  }
  tot_tohit += C_calculate_tohit_bonus_for_weapon_type(equipment[Equipment_primary].tval);


  /*{ Fix for arrows}*/
  const obj_set catch_this = {sling_ammo, bolt, arrow, 0};
  bool const is_missile = is_in(equipment[Equipment_primary].tval, catch_this);
  if (is_missile) {
    blows = 1;
  }
  tot_tohit += player_ptohit;

  /*{ stopped from killing town creatures?? }*/
  if ((monster_templates[a_mptr].cmove & 0x00004000) == 0 ||
      randint(100) < -player_rep) {
    player_is_mean_jerk = true;
  } else {
    player_is_mean_jerk = get_yes_no("Are you sure you want to?");
  }

  /*{ Loop for number of blows, trying to hit the critter...        }*/
  bool monster_is_hit_but_alive = false;
  if (player_is_mean_jerk) {
    monster_is_hit_but_alive = execute_all_attacks(blows, a_cptr, a_mptr, tot_tohit, is_backstab, is_missile);
  }

  RETURN("py_attack", "", 'b', "hit", &monster_is_hit_but_alive);
  return monster_is_hit_but_alive;
}
