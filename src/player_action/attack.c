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
                                long const a_mptr, const long tot_tohit, bool const is_backstab) {
  char m_name[82];
  find_monster_name(m_name, a_cptr, false);
  const monster_template_t *monster_template = &monster_templates[a_mptr];

  /*{ Fix for arrows}*/
  const obj_set catch_this = {sling_ammo, bolt, arrow, 0};
  bool const is_missile = is_in(equipment[Equipment_primary].tval, catch_this);

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

extern long C_calculate_number_of_attacks(long *to_hit);
bool player_action_attack(const long y, const long x) {

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

  long const number_of_attacks = C_calculate_number_of_attacks(&tot_tohit);

  if (is_backstab) {
    tot_tohit += player_lev / 4;
  }

  /*{ Adjust weapons for class }*/
  if (player_pclass == C_WARRIOR) {
    tot_tohit += 1 + player_lev / 2;
  }
  tot_tohit += C_calculate_tohit_bonus_for_weapon_type(equipment[Equipment_primary].tval);
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
    monster_is_hit_but_alive = execute_all_attacks(number_of_attacks, a_cptr, a_mptr, tot_tohit, is_backstab);
  }

  RETURN("py_attack", "", 'b', "hit", &monster_is_hit_but_alive);
  return monster_is_hit_but_alive;
}
