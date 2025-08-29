#include "monsters.h"

#include "../generate_monster/monster_template.h"
#include "../debug.h"

#include <stdbool.h>

static bool init__m_level(void) {
  long *mutable_m_level = m_level;
  const int m_level_size = sizeof(m_level) / sizeof(m_level[0]);

  int8_t current_level = 0;
  long current_count_for_level = 0;
  for (int i = 0; i < monster_template_size; i++) {
    monster_template_t const *monster = &monster_templates[i];
    while (current_level < monster->level) {
      const bool current_level_was_town_level = current_level == 0;
      mutable_m_level[current_level] = current_count_for_level;
      current_level++;
      if (current_level >= m_level_size) {
        MSG(("FATAL: init__m_level: current_level >= m_level_size\n"));
        return false;
      }
      if (current_level_was_town_level) {
        current_count_for_level = 0;
      }
    }
    current_count_for_level++;
  }

  if (current_level != MAX_MONS_LEVEL) {
    MSG(("FATAL: init__m_level: current_level should be MAX_MONS_LEVEL after initialization, was %ld\n", current_level));
    return false;
  }
  return true;
}

bool init__monsters(void) {
  return init__m_level();
}

