#pragma once

#include "types.h"
#include <stdbool.h>
#include <stdint.h>

uint8_t get_creature_at_location(coords const *coord);
bool is_spot_empty_of_creatures(coords const *coord);
bool is_monster_at_location(coords const *coord);

/* Returns number of monster in list specified by virtual_name */
long find_mon(const char *virtual_name);

/* Makes sure new creature gets lit up */
void check_mon_lite(long y, long x);

void creatures(bool attack);

/* Moves creature record from one space to another.
 *
 * NOTE! (x1,y1) might equal (x2,y2)
 */
void move_creature(long from_y, long from_x, long to_y, long to_x);
