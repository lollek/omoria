#ifndef PLAYER_MOVE_H
#define PLAYER_MOVE_H

/**
 * char_to_dir() - Get a 'dir' from a roguelike command
 * @c: character to translate
 * @return: number [0-9] or -1 on error
 */
int char_to_dir(char c);

/**
 * dir_to_char() - Get roguelike command for a direction
 * @dir: Direction to get command for
 * @return: Roguelike command in char, or '?' on error;
 */
char dir_to_char(int dir);

/**
 * -RAK-
 *  move_char() - Moves player from one space to another
 *  @dir: keypad-direction in which direction to go (8 = up)
 */
void move_char(long dir);

#endif // PLAYER_MOVE_H