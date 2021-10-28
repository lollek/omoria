#ifndef LOGIC_GENERATE_ITEM_H
#define LOGIC_GENERATE_ITEM_H

#include <stdint.h>

uint8_t generate_item_level_for_dungeon_level(uint8_t dungeon_level, uint8_t tries);
treasure_type generate_item_for_dungeon_level(uint8_t dungeon_level);

#endif /* LOGIC_GENERATE_ITEM_H */
