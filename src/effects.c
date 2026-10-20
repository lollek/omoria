#include "debug.h"
#include "inventory/inven.h"
#include "io.h"
#include "player.h"
#include "random.h"
#include "screen.h"
#include "text_lines.h"
#include "types.h"
#include "variables.h"

#include <stdbool.h>

void poison_gas(const long dam, char const *const kb_str) {
  MSG(("Damage (Poison Gas): %ld", dam));
  take_hit(dam, kb_str);
  print_stat = 1;
  player_flags.poisoned += 12 + randint(dam);
}

void fire_dam(long dam, char const *const kb_str) {
  const long initial_damage = dam;
  obj_set things_that_burn = {arrow,
                              bow_crossbow_or_sling,
                              hafted_weapon,
                              pole_arm,
                              maul,
                              boots,
                              gloves_and_gauntlets,
                              cloak,
                              soft_armor,
                              staff,
                              scroll1,
                              scroll2,
                              0};

  if (player_flags.fire_resist) {
    dam /= 3;
  }
  if (player_flags.resist_heat > 0) {
    dam /= 3;
  }

  MSG(("Damage (Fire): %ld - %ld = %ld", initial_damage, initial_damage - dam,
       dam));
  take_hit(dam, kb_str);

  print_stat = 1;

  if (inven_damage(things_that_burn, 3) > 0) {
    msg_print("There is smoke coming from your pack!");
    prt_stat_block();
  }
}

void cold_dam(long dam, char const *const kb_str) {
  const long initial_damage = dam;
  obj_set things_that_freeze = {potion1, potion2, 0};

  if (player_flags.cold_resist) {
    dam /= 3;
  }
  if (player_flags.resist_cold > 0) {
    dam /= 3;
  }

  MSG(("Damage (Cold): %ld - %ld = %ld", initial_damage, initial_damage - dam,
       dam));
  take_hit(dam, kb_str);

  print_stat = 1;

  if (inven_damage(things_that_freeze, 5) > 0) {
    msg_print("Something shatters inside your pack!");
    prt_stat_block();
  }
}

void light_dam(long dam, char const *const kb_str) {
  const long initial_damage = dam;

  if (player_flags.lght_resist) {
    dam /= 3;
  }
  if (player_flags.resist_lght > 0) {
    dam /= 3;
  }

  MSG(("Damage (Lightning): %ld - %ld = %ld", initial_damage,
       initial_damage - dam, dam));
  take_hit(dam, kb_str);

  print_stat = 1;
}
