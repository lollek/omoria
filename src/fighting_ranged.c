#include <stdio.h>
#include <string.h>
#include <math.h>

#include "boolean.h"
#include "routines.h"
#include "debug.h"
#include "dungeon.h"
#include "player.h"
#include "variables.h"

enum _ranged_attack_t {
	THROW,
	SHOOT
};

/**
 *__calc_base_to_hit() - Calculate missile base to hit something
 */
static long __calc_base_to_hit(enum _ranged_attack_t type) {
	switch (type) {
		case THROW: return trunc(player_bthb * 0.75);
		case SHOOT: return player_bthb;
	}
	MSG(("__calc_base_to_hit fell through switch"));
	return 0;
}

/**
 *__calc_plus_to_hit() - Calculate missile plus to hit something
 */
static long __calc_plus_to_hit(treas_rec const *missile, enum _ranged_attack_t type) {
	switch (type) {
		case THROW:
			return player_ptohit + missile->data.tohit;
		case SHOOT:
			return player_ptohit + missile->data.tohit + equipment[Equipment_primary].tohit;
	}
	MSG(("__calc_plus_to_hit fell through switch"));
	return 0;
}

/**
 *__calc_damage() - Calculate missile damage
 */
static long __calc_damage(treas_rec const *missile, enum _ranged_attack_t type) {
	switch (type) {
		case THROW:
			return damroll(missile->data.damage) + missile->data.todam;
		case SHOOT: {
			long base_damage = damroll(missile->data.damage) + missile->data.todam;
			switch (equipment[Equipment_primary].p1) {
				case 1: /*{ Sling and Bullet  }*/
					return base_damage + 2;
				case 2: /*{ Short Bow and Arrow    }*/
					return base_damage + 2;
				case 3: /*{ Long Bow and Arrow     }*/
					return base_damage + 3;
				case 4: /*{ Composite Bow and Arrow}*/
					return base_damage + 4;
				case 5: /*{ Light Crossbow and Bolt}*/
					return base_damage + 2;
				case 6: /*{ Heavy Crossbow and Bolt}*/
					return base_damage + 4;
			}
			MSG(("__calc_damage fell through weapon switch"));
			return base_damage;
			    }

	}
	MSG(("__calc_damage fell through switch"));
	return 0;
}

/**
 *__calc_distance() - Calculate how long a missile can fly
 */
static long __calc_distance(treas_rec const *missile, enum _ranged_attack_t type) {
	long item_weight;

	if (missile->data.weight < 1) {
		item_weight = 1;
	} else {
		item_weight = missile->data.weight;
	}

	switch (type) {
		case THROW: {
				    long distance = trunc(((C_player_get_stat(STR) * 10) + 100) * 200 / item_weight);
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
			    MSG(("__calc_distance fell through weapon switch"));
			    return 0;

	}
	MSG(("__calc_damage fell through switch"));
	return 0;
}

/**
 *__can_place_missile_in_coordinate() - Can we place a new missile in that spot?
 * @y: Missile y
 * @x: Missile x
 */
static boolean __can_place_missile_in_coordinate(long y, long x) {
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
 *__missile_hit_ground() - Handle a missile hitting the ground
 * @y: y position where hit
 * @x: x position where hit
 */
static void __missile_hit_ground(treas_rec *missile, long y, long x)
{
	long place_missile_y = y;
	long place_missile_x = x;
	boolean keep_missile = false;

	// 90% chance that we try to place the missile on the ground
	if (randint(10) > 1) {
		for (long i = 0; i < 10; ++i) {
			if (__can_place_missile_in_coordinate(place_missile_y, place_missile_x)) {
				keep_missile = true;
				break;
			}

			// Try another spot next the the impact x/y
			place_missile_y = y + randint(3) - 2;
			place_missile_x = x + randint(3) - 2;
		}
	}

	if (keep_missile) {
		long cur_pos;
		popt(&cur_pos);
		cave[place_missile_y][place_missile_x].tptr = cur_pos;
		t_list[cur_pos] = missile->data;
		if (test_light(place_missile_y, place_missile_x)) {
			lite_spot(place_missile_y, place_missile_x);
		}
	} else {
		char out_val[82];
		char out_val2[120];
		objdes(out_val, missile, false);
		sprintf(out_val2, "The %s disappears.", out_val);
		msg_print(out_val2);
	}
}

/**
 *__missile_try_hit_creature() - Handle a missile maybe hitting a creature
 * @type: Type of ranged attack
 * @y: y position where hit
 * @x: x position where hit
 * @travel_distance: travel distance of missile
 *
 * returns true if a creature was hit
 */
static boolean __missile_try_hit_creature(treas_rec *missile, enum _ranged_attack_t type, long y, long x, long travel_distance) {
	long const base_to_hit = __calc_base_to_hit(type) - travel_distance;
	long plus_to_hit = __calc_plus_to_hit(missile, type);
	long damage = __calc_damage(missile, type);

	int16_t monster_ac = c_list[m_list[cave[y][x].cptr].mptr].ac;
	boolean creature_was_hit = player_test_hit(base_to_hit, player_lev, plus_to_hit, monster_ac, true);
	if (!creature_was_hit) {
		return false;
	}

	char missile_text_buf[82];
	char monster_name_buf[82];
	char text_buf[200];
	long monster_index = m_list[cave[y][x].cptr].mptr;
	objdes(missile_text_buf, missile, FALSE);
	find_monster_name(monster_name_buf, cave[y][x].cptr, FALSE);
	sprintf(text_buf, "The %s hits %s.", missile_text_buf, monster_name_buf);
	msg_print(text_buf);
	damage = tot_dam(&(missile->data), damage, &(c_list[monster_index]));
	long crit_mult = critical_blow(missile->data.weight, plus_to_hit, (equipment[Equipment_primary] .flags2 & Sharp_worn_bit) != 0, true);
	damage += (5 + damage) * crit_mult;

	if (mon_take_hit(cave[y][x].cptr, damage) > 0) {
		sprintf(text_buf, "You have killed %s.", monster_name_buf);
		msg_print(text_buf);
	}
	return true;
}


/**
 *__missile_travel() - Make a missile travel and maybe hit something
 * @missile: The thing which travels
 * @type: How the missile was fired
 * @missile_travel_dir
 */
static void __missile_travel(treas_rec *missile, enum _ranged_attack_t type, long missile_travel_dir)
{
	long const max_distance = __calc_distance(missile, type);

	boolean missile_has_stopped = FALSE;
	long missile_y = char_row;
	long missile_x = char_col;
	long prev_missile_y = missile_y;
	long prev_missile_x = missile_x;
	long travel_distance = 0;

	ENTER(("__missile_travel", ""));

	while (!missile_has_stopped) {
		move_dir(missile_travel_dir, &missile_y, &missile_x);
		travel_distance++;

		// Remove drawing of missile
		if (test_light(prev_missile_y, prev_missile_x)) {
			lite_spot(prev_missile_y, prev_missile_x);
		}

		if (travel_distance > max_distance) {
			missile_has_stopped = TRUE;
		}

		// If something is in the way, stop
		if (!cave[missile_y][missile_x].fopen) {
			missile_has_stopped = TRUE;
		}

		if (missile_has_stopped) {
			__missile_hit_ground(missile, prev_missile_y, prev_missile_x);
			LEAVE("__missile_travel", "");
			return;
		}

		// We hit some creature
		if (cave[missile_y][missile_x].cptr > 1) {
			if (__missile_try_hit_creature(missile, type, missile_y, missile_x, travel_distance)) {
				LEAVE("__missile_travel", "");
				return;
			}
		}

		if (panel_contains(missile_y, missile_x) && test_light(missile_y, missile_x)) {
			print(C_item_get_tchar(&missile->data), missile_y,
					missile_x);
		}

		prev_missile_y = missile_y;
		prev_missile_x = missile_x;
	}

	LEAVE("__missile_travel", "");
}


/**
 *__count_things_to_throw() - Return num things we can throw
 */
static long __count_things_to_throw() {
	long things_to_throw = change_all_ok_stats(TRUE, FALSE);
	for (treas_ptr item_ptr = inventory_list; item_ptr != NULL; item_ptr = item_ptr->next) {
		if (((item_ptr->data.flags2 & Holding_bit) != 0) &&
				(item_ptr->insides > 0)) {
			things_to_throw--;
		}
	}
	return things_to_throw;
}

/**
 *__calculate_ammo_type() - Get ammo type used by current weapon
 */
static uint8_t __calculate_ammo_type()
{
	ENTER(("__calculate_ammo_type", "d"));

	if (equipment[Equipment_primary].tval != bow_crossbow_or_sling) {
		LEAVE("__calculate_ammo_type", "d");
		return 0;
	}

	switch (equipment[Equipment_primary].p1) {
		case 1:
			LEAVE("__calculate_ammo_type", "d");
			return sling_ammo;

		case 2:
		case 3:
		case 4:
			LEAVE("__calculate_ammo_type", "d");
			return arrow;

		case 5:
		case 6:
			LEAVE("__calculate_ammo_type", "d");
			return bolt;

		default:
			LEAVE("__calculate_ammo_type", "d");
			return 0;
	}

}

/**
 * __count_things_to_shoot() - Return num things we can shoot
 */
static long __count_things_to_shoot() {
	uint8_t const ammo_type = __calculate_ammo_type();
	long things_to_shoot = 0;

	// Count objects which can be used as ammo
	change_all_ok_stats(FALSE, FALSE);
	for (treas_ptr ptr = inventory_list; ptr != NULL; ptr = ptr->next) {
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
 * __ranged_attack() - Prepare a ranged attack
 */
static void __ranged_attack(enum _ranged_attack_t type) {
	reset_flag = TRUE;

	long num_things_to_attack_with = 0;
	switch (type) {
		case THROW:
			num_things_to_attack_with = __count_things_to_throw();
			break;
		case SHOOT:
			num_things_to_attack_with = __count_things_to_shoot();
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
	boolean redraw = FALSE;
	treas_ptr weapon;
	char unused_char;
	boolean const item_to_use_found = get_item(&weapon, item_query, &redraw, num_things_to_attack_with, &unused_char, FALSE, FALSE);

	if (redraw) {
		draw_cave();
	}

	if (!item_to_use_found) {
		// We changed our mind
		return;
	}

	// Decide where to attack
	long dir_to_attack; // direction to attack
	long dumy; // the keypad number of the direction to attack
	long y_dumy = char_row; // return y position of where the missile will end up
	long x_dumy = char_col; // return x posiiton of where the missile will end up
	if (!d__get_dir("Which direction?", &dir_to_attack, &dumy, &y_dumy, &x_dumy)) {
		// We changed our mind
		return;
	}

	// Past the point of no return, now we will try to attack
	reset_flag = FALSE;

	// TODO: shouldn't this be moved a bit further down?
	desc_remain(weapon);

	if (player_flags.confused > 0) {
		// Being confused causes us to throw in a random direction
		msg_print("You are confused...");
		do {
			dir_to_attack = randint(9);
		} while (dir_to_attack == 5);
	}

	// Remove the missile (used) from weapon (ammo stack)
	treas_rec missile = {
		.data = weapon->data,
		.ok = false,
		.insides = 0,
		.next = NULL,
		.is_in = false
	};
	missile.data.number = 1;

	if ((weapon->data.number > 1) && (weapon->data.subval > 511)) {
		weapon->data.number--;
		inven_weight -= weapon->data.weight; // BUG? Wrong weight?
	} else {
		inven_destroy(weapon);
	}
	prt_stat_block();

	__missile_travel(&missile, type, dir_to_attack);
}

void throw_something() {
	ENTER(("throw_something", ""));
	__ranged_attack(THROW);
	LEAVE("throw_something", "");

}

void shoot_something() {
	ENTER(("shoot_something", ""));
	__ranged_attack(SHOOT);
	LEAVE("shoot_something", "");
}
