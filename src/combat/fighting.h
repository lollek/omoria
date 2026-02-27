#pragma once

#include <stdbool.h>

/*{ Attacker's level and pluses, defender's AC            -RAK-   }*/
bool managed_to_hit(long base_to_hit, long level, long plus_to_hit, long enemy_ac);
