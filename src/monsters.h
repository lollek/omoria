#ifndef MONSTERS_H
#define MONSTERS_H

#include "types.h"

void place_monster(long y, long x, long z, boolean slp);
void alloc_land_monster(obj_set alloc_set, long num, long dis, boolean slp,
                        boolean water);

/**
 * @brief Spawn loot on monster death
 * 
 * @param y 
 * @param x 
 * @param flags 
 */
void monster_death(long y, long x, unsigned long flags);

/**
 * @brief Decreases monsters hit points and deletes monster if needed.
 * 
 * @param monptr 
 * @param dam 
 * @return long 
 */
long mon_take_hit(long monptr, long dam);

/**
 * @brief Deletes a monster entry from the level
 * 
 * @param i2 
 */
void delete_monster(long i2);

#endif // MONSTERS_H