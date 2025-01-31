#pragma once

#include <stdint.h>

/**
 * mst__init_masters() - Init masters database
 *
 * @returns: true on success, false on failure
 */
uint8_t mst__init_masters(void);

/**
 * mst__update_character() - Update a character on the character list
 *
 * @returns: true on success, false on failure
 */
uint8_t mst__update_character(int64_t uid);

/**
 * mst__add_character() - Add a character to list of characters
 *
 * @returns: The unique uid for the specific character
 */
int64_t mst__add_character(void);
