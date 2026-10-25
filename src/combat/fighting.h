#pragma once

#include <stdbool.h>

typedef struct managed_hit_result {
	long base_to_hit;
	long level;
	long plus_to_hit;
	long max_attack_value;
	long attack_value;
	long enemy_ac;
	bool was_rolled;
	bool hit;
} managed_hit_result;

/*{ Attacker's level and pluses, defender's AC            -RAK-   }*/
bool managed_to_hit(long base_to_hit, long level, long plus_to_hit, long enemy_ac);
managed_hit_result managed_to_hit_detailed(long base_to_hit, long level,
																					 long plus_to_hit, long enemy_ac);
