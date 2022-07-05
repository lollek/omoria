#ifndef PLAYER_ACTION
#define PLAYER_ACTION

#include "boolean.h"
#include "magic.h"

boolean player_action_attack(long y, long x);

/**
 *  Moves player from one space to another
 *
 *  dir: keypad-direction in which direction to go (8 = up)
 */
void player_action_move(long dir);

/**
 * @brief Searches for hidden things
 *
 * @param y
 * @param x
 * @param chance
 */
void player_action_search(long y, long x, long chance);

/**
 * @brief Jam a closed door
 *
 */
void player_action_jam_door(void);

/**
 * @brief Look at something
 *
 */
void player_action_look(void);

/**
 * @brief Open a closed door or chest
 *
 */
void player_action_open(void);

void player_action_aim_wand(void);

void player_action_use_staff(void);

void player_action_read_scroll(void);

void player_action_quaff_potion(void);

/**
 *  Cast a magic spell or use other magic ability
 *
 *  @param magic: Which type of magic is to be cast
 */
void player_action_use_magic(enum magic_t magic_type);

/**
 * @brief Bash open door or chest
 *
 */
void player_action_bash(void);

/**
 * @brief Drop something being carried
 *
 */
void player_action_drop(void);

/**
 * @brief Close an open door
 *
 */
void player_action_close(void);

/**
 * @brief Toggles active light source on or off
 * 
 */
void player_action_toggle_light_source(void);

/**
 * @brief Tunnels through rubble and walls
 * 
 */
void player_action_tunnel(void);

void player_action_rest(void);

void player_action_refill_lamp(void);

void player_action_eat(void);

void player_action_disarm_trap(void);

void player_action_ascend_stairs(void);

void player_action_descend_stairs(void);

#endif // PLAYER_ACTION