#ifndef MASTER_H
#define MASTER_H

#include <stdint.h>

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


#endif /* MASTER_H */
