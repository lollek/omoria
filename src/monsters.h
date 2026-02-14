#pragma once

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
 * @param cptr 
 */
void delete_monster(long cptr);
