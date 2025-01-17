#pragma once

#include <stdint.h>

/**
 * sav__save_character() - Saves the currenet character
 *
 * @returns: True on success, false on failure
 */
uint8_t sav__save_character(void);

/**
 * sav__load_character() - Loads the given character
 *
 * @returns: True on success, false on failure
 */
uint8_t sav__load_character(char const *player_name, int64_t player_uid);

void C_delete_character(void);
