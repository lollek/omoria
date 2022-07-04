#ifndef LOGIC_GENERATE_ITEM_H
#define LOGIC_GENERATE_ITEM_H

#include <stdint.h>

uint8_t generate_item_level_for_dungeon_level(uint8_t dungeon_level, uint8_t tries);

treasure_type generate_item_for_dungeon_level(uint8_t dungeon_level);
treasure_type generate_item_for_item_level(uint8_t item_level);
treasure_type generate_item_for_general_store(void);
treasure_type generate_item_for_armorsmith(void);
treasure_type generate_item_for_weaponsmith(void);
treasure_type generate_item_for_temple(void);
treasure_type generate_item_for_alchemist_store(void);
treasure_type generate_item_for_magic_store(void);
treasure_type generate_item_for_inn(void);
treasure_type generate_item_for_library(void);
treasure_type generate_item_for_music_store(void);
treasure_type generate_item_for_gem_store(void);
treasure_type generate_item_for_all_night_deli(void);
treasure_type generate_item_for_black_market(void);

#endif /* LOGIC_GENERATE_ITEM_H */
