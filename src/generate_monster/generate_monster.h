#pragma once

#include "../variables.h"

/**
 * @brief Spawn a monster at a given location
 *
 * @param y         Coordinate
 * @param x         Coordinate
 * @param template  Monster template to spawn
 * @param is_asleep Should the monster be asleep
 */
void place_monster(long y, long x, long template, bool is_asleep);

/**
 * @brief Spawn new land monster(s) at random location(s)
 *
 * @param alloc_set
 * @param number_of_monsters        Number of monsters to spawn
 * @param min_distance_from_player  How close to player it can spawn
 * @param is_sleeping               Should the monster be asleep
 */
void generate_land_monster(obj_set alloc_set, long number_of_monsters,
                           long min_distance_from_player, bool is_sleeping);

/**
 * @brief Spawn new water monster(s) at random location(s)
 *
 * @param alloc_set
 * @param number_of_monsters        Number of monsters to spawn
 * @param min_distance_from_player  How close to player it can spawn
 * @param is_sleeping               Should the monster be asleep
 */
void generate_water_monster(obj_set alloc_set, long number_of_monsters,
                            long min_distance_from_player, bool is_sleeping);

/**
 * @brief Spawn a land monster adjacent to the given coordinates
 *
 * @param y         Coordinate
 * @param x         Coordinate
 * @param is_asleep Should the monster be asleep
 * @return          Did we successfully spawn a monster?
 */
bool summon_land_monster(int64_t *y, int64_t *x, bool is_asleep);

/**
 * @brief Spawn a water monster adjacent to the given coordinates
 *
 * @param y         Coordinate
 * @param x         Coordinate
 * @param is_asleep Should the monster be asleep
 * @return          Did we successfully spawn a monster?
 */
bool summon_water_monster(int64_t *y, int64_t *x, bool is_asleep);

/**
 * @brief Spawn an undead monster adjacent to the given coordinates
 *
 * @param y         Coordinate
 * @param x         Coordinate
 * @return          Did we successfully spawn a monster?
 */
bool summon_undead(long *y, long *x);

/**
 * @brief Spawn a demon monster adjacent to the given coordinates
 *
 * @param y         Coordinate
 * @param x         Coordinate
 * @return          Did we successfully spawn a monster?
 */
bool summon_demon(long *y, long *x);

/**
 * @brief Spawn a breeder monster adjacent to the given coordinates
 *
 * @param y         Coordinate
 * @param x         Coordinate
 * @return          Did we successfully spawn a monster?
 */
bool summon_breed(long *y, long *x);

/**
 * @brief Summon a monster with a given name
 *
 * @param y         Coordinate
 * @param x         Coordinate
 * @param name      Name of monster to summon
 * @param present   If false, player is prompted to give a name, and variable "name" is ignored
 * @param is_asleep Should the monster be asleep
 */
void monster_summon_by_name(long y, long x, char name[28], bool present,
                            bool is_asleep);

/**
 * @brief Multiplies a breeding monster
 * 
 * @param y         Coordinate
 * @param x         Coordinate
 * @param template  Monster template to use for placing monster
 * @param is_asleep 
 */
void multiply_monster(long y, long x, long template, bool is_asleep);
