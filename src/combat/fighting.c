#include "fighting.h"

#include "../debug.h"
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

  const long max_possible_attack_value = bth + level * 3 + pth * 3;
  MSG(("MonsterToHit: %d + %d*3 + %d*3 = %d", bth, level, pth, max_possible_attack_value));
  const long attack_value = randint(max_possible_attack_value);
  const bool managed_to_hit = (attack_value >= ac) || randint(20) == 1;
  MSG(("MonsterHits? %d (of %d) vs %d - Hit? %d", attack_value, max_possible_attack_value, ac, managed_to_hit));
  return managed_to_hit;
}
