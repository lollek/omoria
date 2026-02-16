#pragma once

#include <stdint.h>
#include <stdbool.h>

/**
 * sav__save_character() - Saves the currenet character
 *
 * @returns: True on success, false on failure
 */
bool sav__save_character(void);

/**
 * sav__load_character() - Loads the given character
 *
 * @returns: True on success, false on failure
 */
bool sav__load_character(char const *player_name, int64_t player_uid);

void C_delete_character(void);
