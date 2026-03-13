#include "monster_template.h"

#include <stddef.h>

long m_level[MAX_MONS_LEVEL + 1] = {0};

bool monster_template_has_attributes_at(
    long index, monster_attribute const *const *monster_attributes) {
  for (monster_attribute const *attribute = *monster_attributes;
       attribute != NULL; attribute++) {
    if (!monster_template_has_attribute_at(index, *attribute)) {
      return false;
    }
  }
  return true;
}
