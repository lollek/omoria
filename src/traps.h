#pragma once

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
 * Null pointers are rejected. Non-null pointers must each point to a readable,
 * aligned long. Call on the game thread with initialized globals.
 *
 * @param y
 * @param x
 */
void hit_trap(const long *y, const long *x);

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

/* Temporary C effect entry points forwarded by Rust until remaining groups are ported. */
void ht__covered_pit(long dam, long y, long x);
void ht__trap_door(long dam);
void ht__hidden_object(long y, long x);
void ht__teleport(void);
void ht__rockfall(long dam, long y, long x);
void ht__corrode_gas(void);
void ht__summon_monster(long y, long x);
void ht__fire(long dam);
void ht__acid(long dam);
void ht__poison_gas(long dam);
void ht__chute(long dam);
void ht__whirlpool(long dam);
