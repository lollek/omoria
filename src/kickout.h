#pragma once

#include <stdbool.h>
#include <stdio.h>

extern char operating_hours[7][30];

/**
 * kick__should_kickout() - If all players should be disconnected from the game
 *
 * This function can be used for either kicking players out for updates, or when
 * they are playing past operating hours.
 *
 * If this function returns true, and the game has started, you must do any saves
 * necessary and then call kick__kickout_player_if_time()
 */
bool kick__should_kickout(void);

/**
 * kick__kickout_player_if_time() - Check if it's time to kickout, and if so -
 * do so.
 */
void kick__kickout_player_if_time(void);

/**
 * kick__dump_operating_hours_to_file()
 *
 * Used for printin operating hours, or creating a new file
 */
void kick__dump_operating_hours_to_file(FILE *file);
