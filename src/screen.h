#ifndef SCREEN_H
#define SCREEN_H

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
void prt_equipment_args(integer y, integer x, integer start, boolean clear);

/**
 * -RAK-
 * prt_map() - Prints the map of the dungeon
 */
extern void prt_map();

/**
 * -RAK-
 *  prt_stat() - Print character stat in given row, column
 */
extern void prt_stat(vtype stat_name, byteint stat, integer row,
		     integer column);

/**
 * -RAK-
 * prt_stat_attr() - Print character stat in a given row, column
 */
extern void prt_stat_attr(vtype stat_name, byteint stat, byteint loss,
			  integer row, integer column);

/**
 * -RAK-
 *  prt_field() - Print character info in a given row, column
 */
extern void prt_field(vtype info, integer row, integer column);

/**
 * -RAK-
 *  prt_num() - Print number with header at given row, column
 */
extern void prt_num(vtype header, integer num, integer row, integer column);

/**
 * -RAK-
 *  prt_title() - Prints title of character's level
 */
extern void prt_title();

/**
 * prt_a_stat()
 */
extern void prt_a_stat(stat_set tstat);

/**
 * -RAK-
 *  prt_level() - Prints level
 */
extern void prt_level();

/**
 * -DJC-
 *  prt_mana() - Prints player's mana
 */
extern void prt_mana();

/**
 * -DCJ-
 *  prt_hp() - Prints hit points
 */
extern void prt_hp();

/**
 * -RAK-
 *  prt_pac() - Prints current AC
 */
extern void prt_pac();

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
 * prt_6_stats()
 */
extern void prt_6_stats(stat_s_type p, stat_s_type l, byteint row, byteint col);

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