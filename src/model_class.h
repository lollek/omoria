#pragma once

#include "types.h"

signed char C_class_melee_bonus(enum class_t class);
signed char C_class_ranged_bonus(enum class_t class);
bool C_class_can_use_item(enum class_t class, treasure_type const *item);
int8_t C_calculate_tohit_bonus_for_weapon_type(uint8_t item_tval);
