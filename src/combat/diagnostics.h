#pragma once

#include <stdbool.h>

/**
 * @brief Add combat calculation messages to game history when enabled.
 */

/**
 * @brief Log the player's attack check.
 *
 * @param rating Attack value before the d20 roll
 * @param d20_roll Primary d20 result
 * @param enemy_ac Defender's armor class
 * @param did_hit Whether the attack hit
 */
void combat_log_player_roll(long rating, long d20_roll, long enemy_ac, bool did_hit);

/**
 * @brief Log a monster's attack check.
 *
 * @param attack_value Rolled attack value
 * @param max_attack_value Upper bound for the attack value
 * @param enemy_ac Defender's armor class
 * @param did_hit Whether the attack hit
 */
void combat_log_monster_roll(long attack_value, long max_attack_value,
                             long enemy_ac, bool did_hit);

/**
 * @brief Log a target's HP transition when damage is positive.
 *
 * @param player_target True for the player; false for a monster
 * @param damage Applied damage; no message is emitted when not positive
 * @param hp_before Target's HP before the hit
 * @param hp_after Target's HP after the hit
 */
void combat_log_hp_change(bool player_target, long damage, long hp_before,
                          long hp_after);