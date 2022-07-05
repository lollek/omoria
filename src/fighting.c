#include "player.h"
#include "random.h"
#include "variables.h"

boolean test_hit(long bth, long level, long pth, long ac) {
  /*{ Attacker's level and pluses, defender's AC            -RAK-   }*/

  long i1;
  boolean return_value;

  if (search_flag) {
    search_off();
  }
  if (player_flags.rest > 0) {
    rest_off();
  }

  i1 = bth + level * BTH_LEV_ADJ + pth * BTH_PLUS_ADJ;

  /*{ hits if above ac or 1 in 20.  OOK! }*/
  return_value = ((randint(i1) > ac) || (randint(20) == 1));

  return return_value;
}
