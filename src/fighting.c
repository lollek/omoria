#include "player.h"
#include "random.h"
#include "variables.h"

bool test_hit(const long bth, const long level, const long pth,
                 const long ac) {
  /*{ Attacker's level and pluses, defender's AC            -RAK-   }*/

  if (search_flag) {
    search_off();
  }
  if (player_flags.rest > 0) {
    rest_off();
  }

  const long i1 = bth + level * BTH_LEV_ADJ + pth * BTH_PLUS_ADJ;

  /*{ hits if above ac or 1 in 20.  OOK! }*/
  const bool return_value = randint(i1) > ac || randint(20) == 1;

  return return_value;
}
