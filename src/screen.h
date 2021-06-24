#ifndef SCREEN_H
#define SCREEN_H

#include "types.h"

/**
 * -OK-
 *  prt_equipment() - Draws equipment screen on the right
 */
void prt_equipment(void);

/**
 * -OK-
 *  prt_equipment_args() - Helper function for drawing equipment screen
 *  @y: Y position to start printing
 *  @x: X position to start printing
 *  @start: Equpment number to start from (1 = first)
 *  @clear: Clear the line after last equipment
 */
void prt_equipment_args(long y, long x, long start, boolean clear);

/**
 * -RAK-
 * prt_map() - Prints the map of the dungeon
 */
extern void prt_map();

/**
 * -RAK-
 *  prt_stat() - Print character stat in given row, column
 */
extern void prt_stat(const char stat_name[82], unsigned char stat, long row,
		     long column);

/**
 * -RAK-
 *  prt_field() - Print character info in a given row, column
 */
extern void prt_field(char info[82], long row, long column);

/**
 * -RAK-
 *  prt_num() - Print number with header at given row, column
 */
extern void prt_num(char header[82], long num, long row, long column);

/**
 * -RAK-
 *  prt_gold() - Prints current gold
 */
extern void prt_gold();

/**
 * -DCJ-
 *  prt_weight() - Prints current inventory weight
 */
extern void prt_weight();

/**
 * -DMF-
 *  prt_time() - Prints time of game day
 */
extern void prt_time();

/**
 * -RAK-
 *  prt_depth() - Prints depth in stat area
 */
extern void prt_depth();

/**
 * prt_light_on()
 */
extern void prt_light_on();

/**
 * -RAK-
 *  prt_hunger() - Prints status of hunger
 */
extern void prt_hunger();

/**
 * -RAK-
 *  prt_blind() - Prints blind status
 */
extern void prt_blind();

/**
 * -RAK-
 *  prt_confused() - Prints confusion status
 */
extern void prt_confused();

/**
 * -RAK-
 *  prt_afraid() - Prints fear status
 */
extern void prt_afraid();

/**
 * -RAK-
 *  prt_poisoned() - Prints poisoned status
 */
extern void prt_poisoned();

/**
 * -RAK-
 *  prt_search() - Prints searching status
 */
extern void prt_search();

/**
 * -RAK-
 *  prt_rest() - Prints resting status
 */
extern void prt_rest();

/**
 * -RAD-
 *  prt_quested() - Prints quested status
 */
extern void prt_quested();

/**
 * -RAK-
 *  prt_winner() - Prints winner status on display
 */
extern void prt_winner();

/**
 * prt_experience()
 */
extern void prt_experience();

/**
 * -RAK-
 *  prt_stat_block() - Prints character-screen info
 */
extern void prt_stat_block();

/**
 * -RAK-
 * draw_cave() - Draws entire screen
 */
extern void draw_cave();

#endif /* SCREEN_H */
