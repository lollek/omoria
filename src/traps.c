/* traps.c */
/* all sorts of nasty traps (stores too)! */

#include <math.h>
#include <stdio.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "casino/casino.h"
#include "combat/fighting.h"
#include "constants.h"
#include "debug.h"
#include "generate_monster/generate_monster.h"
#include "io.h"
#include "loot/loot.h"
#include "magic.h"
#include "misc.h"
#include "pascal.h"
#include "player.h"
#include "quest.h"
#include "random.h"
#include "screen.h"
#include "spells.h"
#include "stores.h"
#include "town_level/enter_bank.h"
#include "town_level/enter_house.h"
#include "trade.h"
#include "traps.h"
#include "types.h"
#include "variables.h"

/* These functions are now provided by Rust (see src/dungeon/trap/interop.rs). */
void place_trap(long y, long x, long typ, long subval);
void change_trap(long y, long x);
void place_rubble(long y, long x);

void ht__covered_pit(const long dam, const long y, const long x) {
  msg_print("You fell into a covered pit.");
  if (player_flags.ffall) {
    msg_print("You gently float down.");
  } else {
    take_hit(dam, "a covered pit");
    place_trap(y, x, 2, 1);
  }
}

void ht__trap_door(const long dam) {
  msg_print("You fell through a trap door!");
  msg_print(" ");
  moria_flag = true;
  dun_level++;
  if (player_flags.ffall) {
    msg_print("You gently float down.");
  } else {
    take_hit(dam, "a trap door");
  }
}

void ht__hidden_object(const long y, const long x) {
  cave[y][x].fm = false;
  pusht(cave[y][x].tptr);
  place_random_dungeon_item(y, x);
  msg_print("Hmmm, there was something under this rock.");
}

void ht__teleport(void) {
  teleport_flag = true;
  msg_print("You hit a teleport trap!");
}

void ht__rockfall(const long dam, const long y, const long x) {
  take_hit(dam, "falling rock");
  pusht(cave[y][x].tptr);
  place_rubble(y, x);
  msg_print("You are hit by falling rock");
}

void ht__summon_monster(const long y, const long x) {

  cave[y][x].fm = false; /*{ Rune disappears...    }*/
  pusht(cave[y][x].tptr);
  cave[y][x].tptr = 0;
  for (long i1 = 1; i1 <= 2 + randint(3); i1++) {
    int64_t ty = char_row;
    int64_t tx = char_col;
    if (is_in(cave[ty][tx].fval, water_set)) {
      summon_water_monster(&ty, &tx, false);
    } else {
      summon_land_monster(&ty, &tx, false);
    }
  }
}

void ht__chute(const long dam) {
  msg_print("You fell down a chute!");
  msg_print(" ");
  moria_flag = true;
  dun_level += randint(6);
  if (player_flags.ffall) {
    msg_print("You gently slide down.");
  } else {
    take_hit(dam, "chute landing");
  }
}

void ht__whirlpool(const long dam) {
  msg_print("You are swept into a whirlpool!");
  msg_print(" ");
  moria_flag = true;
  do {
    dun_level++;
    if (!player_flags.ffall) { /*{XXX...swimming_worn}*/
      msg_print("You are drowning!");
      take_hit(dam, "drowning");
    }
  } while (randint(2) != 1);
}

void trigger_trap(const long y, const long x) {
  /*{ Chests have traps too...                              -RAK-   }*/
  /*{ Note: Chest traps are based on the FLAGS value                }*/

  const unsigned long flags = t_list[cave[y][x].tptr].flags;

  /* with t_list[cave[y][x].tptr]. do; */

  if ((0x00000010 & flags) != 0) {
    msg_print("A small needle has pricked you!");
    if (lose_stat(STR, "", "You are unaffected.")) {
      take_hit(damroll("1d4"), "a poison needle");
      print_stat = 1;
      msg_print("You feel weakened!");
    }
  }

  if ((0x00000020 & flags) != 0) {
    msg_print("A small needle has pricked you!");
    take_hit(damroll("1d6"), "a poison needle.");
    player_flags.poisoned += 10 + randint(20);
  }

  if ((0x00000040 & flags) != 0) {
    msg_print("A puff of yellow gas surrounds you!");
    if (player_flags.free_act) {
      msg_print("You are unaffected.");
    } else {
      msg_print("You choke and pass out.");
      player_flags.paralysis = 10 + randint(20);
    }
  }

  if ((0x00000080 & flags) != 0) {
    msg_print("There is a sudden explosion!");
    delete_object(y, x);
    take_hit(damroll("5d8"), "an exploding chest");
  }

  if ((0x00000100 & flags) != 0) {
    for (long i1 = 0; i1 < 3; i1++) {
      int64_t i2 = y;
      int64_t i3 = x;
      if (is_in(cave[i2][i3].fval, water_set)) {
        summon_water_monster(&i2, &i3, false);
      } else {
        summon_land_monster(&i2, &i3, false);
      }
    }
  }
}
