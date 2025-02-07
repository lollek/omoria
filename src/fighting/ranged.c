#include "../debug.h"
#include "../inven.h"
#include "../io.h"
#include "../misc.h"
#include "../model_item.h"
#include "../monsters.h"
#include "../player.h"
#include "../random.h"
#include "../screen.h"
#include "../text_lines.h"
#include "../variables.h"
#include <math.h>
#include <stdio.h>
#include <string.h>

enum ranged_attack_t { THROW, SHOOT };

/**
 *calc_base_to_hit() - Calculate missile base to hit something
 */
static long calc_base_to_hit(const enum ranged_attack_t type) {
  switch (type) {
  case THROW:
    return trunc(player_bthb * 0.75);
  case SHOOT:
    return player_bthb;
  }
  MSG(("calc_base_to_hit fell through switch"));
  return 0;
}

/**
 *calc_plus_to_hit() - Calculate missile plus to hit something
 */
static long calc_plus_to_hit(treas_rec const *missile,
                               const enum ranged_attack_t type) {
  switch (type) {
  case THROW:
    return player_ptohit + missile->data.tohit;
  case SHOOT:
    return player_ptohit + missile->data.tohit +
           equipment[Equipment_primary].tohit;
  }
  MSG(("calc_plus_to_hit fell through switch"));
  return 0;
}

/**
 *calc_damage() - Calculate missile damage
 */
static long calc_damage(treas_rec const *missile,
                          const enum ranged_attack_t type) {
  if (type == THROW) {
    return damroll(missile->data.damage) + missile->data.todam;
  }

  // type == SHOOT, missile.tval should be 20 (ranged weapon)
  const long base_damage = damroll(missile->data.damage) + missile->data.todam;
  switch (missile->data.subval) {
    case 1: /*{ Short Bow and Arrow    }*/
      return base_damage + 2;
    case 2: /*{ Hunters Bow and Arrow     }*/
      return base_damage + 3;
    case 3: /*{ Composite Bow and Arrow}*/
      return base_damage + 4;
    case 4: /*{ War Bow and Arrow}*/
      return base_damage + 5;
    case 5: /*{ Double Bow and Arrow}*/
      return base_damage + 6;
    case 6: /*{ Siege Bow and Arrow}*/
      return base_damage + 7;
    case 7: /*{ Warded Bow and Arrow}*/
      return base_damage + 8;
    case 10: /*{ Light Crossbow and Bolt}*/
      return base_damage + 2;
    case 11: /*{ Heavy Crossbow and Bolt}*/
      return base_damage + 4;
    case 12: /*{ Siege Crossbow and Bolt}*/
      return base_damage + 6;
    case 13: /*{ Ballista and Bolt}*/
      return base_damage + 8;
    case 20: /*{ Sling and Bullet  }*/
      return base_damage + 2;
  }

  MSG(("calc_damage fell through switch"));
  return 0;
}

/**
 *calc_distance() - Calculate how long a missile can fly
 */
static long calc_distance(treas_rec const *missile,
                            const enum ranged_attack_t type) {
  long item_weight;

  if (missile->data.weight < 1) {
    item_weight = 1;
  } else {
    item_weight = missile->data.weight;
  }

  switch (type) {
  case THROW: {
    const long distance =
        trunc((C_player_get_stat(STR) * 10 + 100) * 200 / item_weight);
    return distance > 10 ? 10 : distance;
  }
  case SHOOT:
    switch (equipment[Equipment_primary].p1) {
    case 1: /*{ Sling and Bullet  }*/
      return 20;
    case 2: /*{ Short Bow and Arrow    }*/
      return 25;
    case 3: /*{ Long Bow and Arrow     }*/
      return 30;
    case 4: /*{ Composite Bow and Arrow}*/
      return 35;
    case 5: /*{ Light Crossbow and Bolt}*/
      return 25;
    case 6: /*{ Heavy Crossbow and Bolt}*/
      return 35;
    }
    MSG(("calc_distance fell through weapon switch"));
    return 0;
  }
  MSG(("calc_damage fell through switch"));
  return 0;
}

/**
 *can_place_missile_in_coordinate() - Can we place a new missile in that spot?
 * @y: Missile y
 * @x: Missile x
 */
static bool can_place_missile_in_coordinate(const long y, const long x) {
  if (!in_bounds(y, x)) {
    return false;
  }
  if (!cave[y][x].fopen) {
    return false;
  }
  if (cave[y][x].tptr != 0) {
    return false;
  }
  return true;
}

/**
 * Takes a coordinate coord and checks nearby if we can put a missile there.
 * If we can, coord will be updated with the suitable coordinates.
 *
 * @param coord in and out coordinates
 * @return if we found a spot to put missile
 */
static bool find_missile_spot_on_ground(coords *coord) {
  long place_missile_y = coord->y;
  long place_missile_x = coord->x;

  for (long attempt = 0; attempt < 10; ++attempt) {
    if (can_place_missile_in_coordinate(place_missile_y, place_missile_x)) {
      coord->y = place_missile_y;
      coord->x = place_missile_x;
      return true;
    }

    // try another spot next the the impact x/y
    place_missile_y = coord->y + randint(3) - 2;
    place_missile_x = coord->x + randint(3) - 2;
  }
  return false;
}

/**
 *missile_hit_ground() - Handle a missile hitting the ground
 * @y: y position where hit
 * @x: x position where hit
 */
static void missile_hit_ground(const treas_rec *missile, const long y,
                                 const long x) {
  bool keep_missile = false;
  coords place_missile_coords = {
      .y = y,
      .x = x,
  };

  // 90% chance that we try to place the missile on the ground
  if (randint(10) > 1) {
    keep_missile = find_missile_spot_on_ground(&place_missile_coords);
  }

  if (keep_missile) {
    long cur_pos;
    popt(&cur_pos);
    cave[place_missile_coords.y][place_missile_coords.x].tptr = cur_pos;
    t_list[cur_pos] = missile->data;
    if (test_light(place_missile_coords.y, place_missile_coords.x)) {
      lite_spot(place_missile_coords.y, place_missile_coords.x);
    }
  } else {
    char out_val[82];
    char out_val2[120];
    objdes(out_val, missile, false);
    sprintf(out_val2, "The %s breaks.", out_val);
    msg_print(out_val2);
  }
}

/**
 *missile_try_hit_creature() - Handle a missile maybe hitting a creature
 * @type: Type of ranged attack
 * @y: y position where hit
 * @x: x position where hit
 * @travel_distance: travel distance of missile
 *
 * returns true if a creature was hit
 */
static bool missile_try_hit_creature(const treas_rec *missile,
                                          const enum ranged_attack_t type,
                                          const long y, const long x,
                                          const long travel_distance) {
  long const base_to_hit = calc_base_to_hit(type) - travel_distance;
  const long plus_to_hit = calc_plus_to_hit(missile, type);
  long damage = calc_damage(missile, type);

  const int16_t monster_ac = monster_templates[m_list[cave[y][x].cptr].mptr].ac;
  const bool creature_was_hit =
      player_test_hit(base_to_hit, player_lev, plus_to_hit, monster_ac, true);
  char monster_name_buf[82];
  find_monster_name(monster_name_buf, cave[y][x].cptr, FALSE);
  if (!creature_was_hit) {
    char text_buf[200];
    sprintf(text_buf, "You miss %s.", monster_name_buf);
    msg_print(text_buf);
    return false;
  }

  char missile_text_buf[82];
  char text_buf[200];
  const long monster_index = m_list[cave[y][x].cptr].mptr;
  objdes(missile_text_buf, missile, FALSE);
  sprintf(text_buf, "The %s hits %s.", missile_text_buf, monster_name_buf);
  msg_print(text_buf);
  damage = tot_dam(&missile->data, damage, &monster_templates[monster_index]);
  const long crit_mult = critical_blow(
      missile->data.weight, plus_to_hit,
      (equipment[Equipment_primary].flags2 & Sharp_worn_bit) != 0, true);
  damage += (5 + damage) * crit_mult;

  if (mon_take_hit(cave[y][x].cptr, damage) > 0) {
    sprintf(text_buf, "You have killed %s.", monster_name_buf);
    msg_print(text_buf);
  }
  return true;
}

/**
 *missile_travel() - Make a missile travel and maybe hit something
 * @missile: The thing which travels
 * @type: How the missile was fired
 * @missile_travel_dir
 */
static void missile_travel(const treas_rec *missile,
                             const enum ranged_attack_t type,
                             const long missile_travel_dir) {
  long const max_distance = calc_distance(missile, type);

  long missile_y = char_row;
  long missile_x = char_col;
  long prev_missile_y = missile_y;
  long prev_missile_x = missile_x;
  long travel_distance = 0;

  ENTER(("__missile_travel", ""));

  while (true) {
    move_dir(missile_travel_dir, &missile_y, &missile_x);
    travel_distance++;

    // Remove drawing of missile
    if (test_light(prev_missile_y, prev_missile_x)) {
      lite_spot(prev_missile_y, prev_missile_x);
    }

    bool missile_has_stopped = false;
    if (travel_distance > max_distance) {
      missile_has_stopped = true;
    }

    // If something is in the way, stop
    if (!cave[missile_y][missile_x].fopen) {
      missile_has_stopped = true;
    }

    if (missile_has_stopped) {
      missile_hit_ground(missile, prev_missile_y, prev_missile_x);
      LEAVE("missile_travel", "");
      return;
    }

    // We hit some creature
    if (cave[missile_y][missile_x].cptr > 1) {
      if (missile_try_hit_creature(missile, type, missile_y, missile_x,
                                     travel_distance)) {
        if (type == THROW) {
          missile_hit_ground(missile, missile_y, missile_x);
        }
        LEAVE("missile_travel", "");
        return;
      }
    }

    if (panel_contains(missile_y, missile_x) &&
        test_light(missile_y, missile_x)) {
      print(C_item_get_tchar(&missile->data), missile_y, missile_x);
    }

    prev_missile_y = missile_y;
    prev_missile_x = missile_x;
  }
}

/**
 *__count_things_to_throw() - Return num things we can throw
 */
static long count_things_to_throw(void) {
  long things_to_throw = inventory_change_all_ok_stats(TRUE, FALSE);
  for (const treas_rec *item_ptr = inventory_list; item_ptr != NULL;
       item_ptr = item_ptr->next) {
    if ((item_ptr->data.flags2 & Holding_bit) != 0 &&
        item_ptr->insides > 0) {
      things_to_throw--;
    }
  }
  return things_to_throw;
}

/**
 *_calculate_ammo_type() - Get ammo type used by current weapon
 */
enum ammo_t {
  AMMO_T_NONE = 0,
  AMMO_T_SLING_AMMO = sling_ammo,
  AMMO_T_ARROW = arrow,
  AMMO_T_BOLT = bolt,
};
static enum ammo_t calculate_ammo_type(void) {

  if (equipment[Equipment_primary].tval != bow_crossbow_or_sling) {
    return AMMO_T_NONE;
  }

  switch (equipment[Equipment_primary].p1) {
  case 1:
    return AMMO_T_SLING_AMMO;

  case 2:
  case 3:
  case 4:
    return AMMO_T_ARROW;

  case 5:
  case 6:
    return AMMO_T_BOLT;

  default:
    return AMMO_T_NONE;
  }
}

/**
 * count_things_to_shoot() - Return num things we can shoot
 */
static long count_things_to_shoot(void) {
  enum ammo_t const ammo_type = calculate_ammo_type();
  long things_to_shoot = 0;

  // Count objects which can be used as ammo
  inventory_change_all_ok_stats(FALSE, FALSE);
  for (treas_rec *ptr = inventory_list; ptr != NULL; ptr = ptr->next) {
    if (ptr->data.tval != ammo_type) {
      continue;
    }
    if (ptr->is_in) {
      continue;
    }
    if (ptr->data.tval == bag_or_sack && ptr->insides > 0) {
      continue;
    }

    ptr->ok = true;
    things_to_shoot++;
  }

  return things_to_shoot;
}

/**
 * ranged_attack() - Prepare a ranged attack
 */
static void ranged_attack(const enum ranged_attack_t type) {
  reset_flag = TRUE;

  long num_things_to_attack_with = 0;
  switch (type) {
  case THROW:
    num_things_to_attack_with = count_things_to_throw();
    break;
  case SHOOT:
    num_things_to_attack_with = count_things_to_shoot();
    break;
  }

  if (num_things_to_attack_with == 0) {
    switch (type) {
    case THROW:
      msg_print("But you have nothing to throw.");
      return;
    case SHOOT:
      msg_print("You have nothing to shoot!");
      return;
    }
  }

  char const *item_query = NULL;
  switch (type) {
  case THROW:
    item_query = "Hurl which item?";
    break;
  case SHOOT:
    item_query = "Fire which one?";
    break;
  }

  // Decide what to attack with
  bool redraw = FALSE;
  treas_rec *weapon;
  char unused_char;
  bool const item_to_use_found =
      get_item(&weapon, item_query, &redraw, num_things_to_attack_with,
               &unused_char, FALSE, FALSE);

  if (redraw) {
    draw_cave();
  }

  if (!item_to_use_found) {
    // We changed our mind
    return;
  }

  // Decide where to attack
  long dir_to_attack;     // direction to attack
  long dumy;              // the keypad number of the direction to attack
  long y_dumy = char_row; // return y position of where the missile will end up
  long x_dumy = char_col; // return x posiiton of where the missile will end up
  if (!d__get_dir("Which direction?", &dir_to_attack, &dumy, &y_dumy,
                  &x_dumy)) {
    // We changed our mind
    return;
  }

  // Past the point of no return, now we will try to attack
  reset_flag = FALSE;

  // TODO: shouldn't this be moved a bit further down?
  msg_remaining_of_item(weapon);

  if (player_flags.confused > 0) {
    // Being confused causes us to throw in a random direction
    msg_print("You are confused...");
    do {
      dir_to_attack = randint(9);
    } while (dir_to_attack == 5);
  }

  // Remove the missile (used) from weapon (ammo stack)
  treas_rec missile = {.data = weapon->data,
                       .ok = false,
                       .insides = 0,
                       .next = NULL,
                       .is_in = false};
  missile.data.number = 1;

  if (weapon->data.number > 1 && weapon->data.subval > 511) {
    weapon->data.number--;
    inven_weight -= weapon->data.weight; // BUG? Wrong weight?
  } else {
    inven_destroy(weapon);
  }
  prt_stat_block();

  missile_travel(&missile, type, dir_to_attack);
}

void throw_something(void) {
  ENTER(("throw_something", ""));
  ranged_attack(THROW);
  LEAVE("throw_something", "");
}

void shoot_something(void) {
  ENTER(("shoot_something", ""));
  ranged_attack(SHOOT);
  LEAVE("shoot_something", "");
}
