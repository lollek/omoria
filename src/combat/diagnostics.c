#include "diagnostics.h"

#include "../io.h"
#include "../variables.h"

void combat_log_player_roll(const long rating, const long d20_roll,
                            const long enemy_ac, const bool did_hit) {
  if (!show_combat_calculations) {
    return;
  }

  const long total = rating + d20_roll;
  msg_printf("Combat: You roll %ld + %ld = %ld vs AC %ld: %s.", rating,
             d20_roll, total, enemy_ac, did_hit ? "HIT" : "MISS");
}

void combat_log_monster_roll(const long attack_value,
                             const long max_attack_value,
                             const long enemy_ac, bool did_hit) {
  if (!show_combat_calculations) {
    return;
  }

  msg_printf("Combat: Monster roll %ld/%ld vs AC %ld: %s.", attack_value,
             max_attack_value, enemy_ac, did_hit ? "HIT" : "MISS");
}

void combat_log_hp_change(const bool player_target, const long damage,
                          const long hp_before, const long hp_after) {
  if (!show_combat_calculations || damage <= 0) {
    return;
  }

  const char *attacker = player_target ? "Monster" : "Player";
  const char *defender = player_target ? "Player" : "Monster";

  msg_printf("Combat: %s hits %s for %ld damage (HP %ld -> %ld).",
             attacker, defender, damage, hp_before, hp_after);
}