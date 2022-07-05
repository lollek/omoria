#ifndef TRAPS_H
#define TRAPS_H

/**
 * @brief
 * Change a trap from invisible to visible.
 * Note: Secret doors are handled here
 * 
 * @param y coordinate
 * @param x coordinate
 */
void change_trap(long y, long x);

/**
 * @brief Player hit a trap
 * 
 * @param y 
 * @param x 
 */
void hit_trap(long *y, long *x);

/**
 * @brief Player triggered a trap by opening a chest or failing to disarm
 * 
 * @param y 
 * @param x 
 */
void trigger_trap(long y, long x);

/**
 * @brief Places a particular trap at location
 * 
 * @param y coordinate
 * @param x coordinate
 * @param typ 
 * @param subval 
 */
void place_trap(long y, long x, long typ, long subval);

/**
 * @brief Places rubble at location
 * 
 * @param y coordinate
 * @param x coordinate
 */
void place_rubble(long y, long x);

#endif // TRAPS_H