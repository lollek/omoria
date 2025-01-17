#pragma once

/*	{ Screen size parameters					} */
#define SCREEN_HEIGHT 22
#define SCREEN_WIDTH 66
#include <stdbool.h>

/**
 *  Draws equipment screen on the right
 */
void prt_equipment(void);

/**
 *  Helper function for drawing equipment screen
 *  @param y: Y position to start printing
 *  @param x: X position to start printing
 *  @param start: Equpment number to start from (1 = first)
 *  @param clear: Clear the line after last equipment
 */
void prt_equipment_args(long y, long x, long start, bool clear);

/**
 *  Prints the map of the dungeon
 */
void prt_map(void);

/**
 *   Print character stat in given row, column
 */
void prt_stat(const char stat_name[82], unsigned char stat, long row,
              long column);

/**
 *   Print character info in a given row, column
 */
void prt_field(char info[82], long row, long column);

/**
 *  Print number with header at given row, column
 */
void prt_num(char header[82], long num, long row, long column);

/**
 *  Prints current gold
 */
void prt_gold(void);

/**
 *  Prints current inventory weight
 */
void prt_weight(void);

/**
 *  Prints time of game day
 */
void prt_time(void);

/**
 *  Prints depth in stat area
 */
void prt_depth(void);

/**
 */
void prt_light_on(void);

/**
 *  Prints status of hunger
 */
void prt_hunger(void);

/**
 *  Prints blind status
 */
void prt_blind(void);

/**
 *  Prints confusion status
 */
void prt_confused(void);

/**
 *  Prints fear status
 */
void prt_afraid(void);

/**
 *  Prints poisoned status
 */
void prt_poisoned(void);

/**
 *  Prints searching status
 */
void prt_search(void);

/**
 *  Prints resting status
 */
void prt_rest(void);

/**
 *  Prints quested status
 */
void prt_quested(void);

/**
 *  Prints winner status on display
 */
void prt_winner(void);

/**
 */
void prt_experience(void);

/**
 *  Prints character-screen info
 */
void prt_stat_block(void);

/**
 * Draws entire screen
 */
void draw_cave(void);

/**
 * @brief Unveils a black square on the map
 *
 * @param y
 * @param x
 */
void lite_spot(long y, long x);

/**
 * @brief Blackens out a square on the map
 *
 * @param y
 * @param x
 */
void unlite_spot(long y, long x);

/**
 * @brief Tests a given point to see if it is within the screen boundaries
 *
 * @param y
 * @param x
 * @return bool
 */
bool panel_contains(long y, long x);
