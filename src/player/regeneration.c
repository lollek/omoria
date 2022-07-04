#include "../player.h"

#include "hunger.h"

static float get_base(void) {
  switch (player_hunger_status()) {
  case DYING:
    return 0.0005;
  case WEAK:
    return 0.0015;
  case HUNGRY:
  case FULL:
  case BLOATED:
  default:
    return 0.0030;
  }
}

float player_regeneration_get_amount(void) {
  float regen_amount = get_base();
  if (player_flags.regenerate) {
    regen_amount *= 1.5;
  }
  if (player_flags.rest > 0) {
    regen_amount *= 2;
  }
  return regen_amount;
}