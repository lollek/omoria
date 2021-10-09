#ifndef KICKOUT_H
#define KICKOUT_H

#include <stdbool.h>

/**
 * kick__should_kickout() - If all players should be disconnected from the game
 */
bool kick__should_kickout(void);

/**
 * kick__kickout_player_if_time() - Check if it's time to kickout, and if so -
 * do so
 */
void kick__kickout_player_if_time(void);

#endif /* KICKOUT_H */
