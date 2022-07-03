#ifndef MODEL_CLASS_H
#define MODEL_CLASS_H

#include "types.h"

signed char C_class_melee_bonus(enum class_t class);
signed char C_class_ranged_bonus(enum class_t class);
boolean C_class_can_use_item(enum class_t class, treasure_type const *item);

#endif // MODEL_CLASS_H