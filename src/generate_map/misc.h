#pragma once

#include "../types.h"

/**
 * -RAK-
 *  place_boundry() Places indestructible rock arounds edges of dungeon
 */
void place_boundry(void);

void try_to_place_stairs(long stairs_type, long number_of_stairs, int number_of_adjacent_walls_to_place_stairs);

/**
 * -RAK-
 *  fill_cave() - Fills in empty spots with desired rock
 *  Note: 9 is a temporary value
 */
void fill_cave(floor_type fill);

/**
 * blank_out_map() - Blanks out entire cave
 */
void blank_out_map(void);

void place_a_staircase(long y, long x, long staircase_type);

/**
 * Place door of random type at coordinates
 */
void place_random_door(long y, long x);
void place_open_door(long y, long x);
void place_broken_door(long y, long x);
void place_closed_door(long y, long x);
void place_locked_door(long y, long x);
void place_stuck_door(long y, long x);
void place_secret_door(long y, long x);
