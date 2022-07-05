#ifndef PLAYER_ACTION_MOVE_H
#define PLAYER_ACTION_MOVE_H

#include "../boolean.h"

extern boolean cave_flag;

/**
 * Get a 'dir' from a roguelike command
 * 
 * c: character to translate
 * return: number [0-9] or -1 on error
 */
int char_to_dir(char c);

/**
 * Get roguelike command for a direction
 * 
 * dir: Direction to get command for
 * return: Roguelike command in char, or '?' on error;
 */
char dir_to_char(int dir);

/**
 *  Moves player from one space to another
 * 
 *  dir: keypad-direction in which direction to go (8 = up)
 */
void player_action_move(long dir);

#endif // PLAYER_ACTION_MOVE_H