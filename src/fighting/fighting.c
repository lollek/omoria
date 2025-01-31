#include "fighting.h"

#include "../player.h"
#include "../random.h"
#include "../variables.h"

bool managed_to_hit(const long bth, const long level, const long pth, const long ac) {

  if (search_flag) {
    search_off();
  }
  if (player_flags.rest > 0) {
    rest_off();
  }

  const long attack_value = bth + level * BTH_LEV_ADJ + pth * BTH_PLUS_ADJ;
  const bool managed_to_hit = randint(attack_value) > ac;
  if (managed_to_hit) {
    return true;
  }

  // 1 in 20 for a random lucky hit
  return randint(20) == 1;
}
