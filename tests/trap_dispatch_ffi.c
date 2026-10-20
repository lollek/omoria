#include <assert.h>
#include <limits.h>
#include <stdio.h>
#include <string.h>

#if defined(__linux__) && defined(TRAP_TEST_GNU_WRAP)

#include "../src/constants.h"
#include "../src/combat/fighting.h"
#include "../src/floor.h"
#include "../src/generate_monster/generate_monster.h"
#include "../src/io.h"
#include "../src/loot/loot.h"
#include "../src/messages.h"
#include "../src/misc.h"
#include "../src/player.h"
#include "../src/random.h"
#include "../src/spells.h"
#include "../src/stores.h"
#include "../src/traps.h"
#include "../src/variables.h"

/* The default exercises the migrated dispatcher, C ABI, and effect boundary. */
#ifndef TRAP_TEST_MIGRATED
#define TRAP_TEST_MIGRATED 1
#endif

enum event {
  SEARCH,
  REST,
  CHANGE,
  LIGHT,
  ROLL,
  ROUTE,
  LOSE_STAT,
  DAMAGE,
  PUSH,
  ITEM,
  LAND_SUMMON,
  WATER_SUMMON
};

static enum event events[128];
static size_t event_count;
static long trap_y = 7, trap_x = 9;
static const long rolled_damage = 23;
static bool store_open;
static enum store_t expected_store;
static long expected_subval;
static long subval_after_roll;
static const char *expected_cause;
static long expected_damage = rolled_damage;
static long expected_movement_level;
static size_t pushed_count;
static long pushed_index;
static size_t dungeon_item_count;
static long dungeon_item_y, dungeon_item_x;
static size_t land_summon_count, water_summon_count;
static bool chest_trap_mode;
static long chest_damage_values[3];
static size_t chest_damage_count;

static void record(enum event event) {
  assert(event_count < sizeof(events) / sizeof(events[0]));
  events[event_count++] = event;
}

static void handler(void) {
  assert(event_count > 0 && events[event_count - 1] == ROLL);
  assert(!find_flag && !search_flag && player_flags.rest == 0);
  record(ROUTE);
}

void __wrap_search_off(void) {
  assert(event_count == 0 && search_flag && find_flag);
  record(SEARCH);
  search_flag = false;
}

void __wrap_rest_off(void) {
  assert(!search_flag && player_flags.rest > 0 && find_flag);
  record(REST);
  player_flags.rest = 0;
}

void __wrap_change_trap(long y, long x) {
  assert(!search_flag && player_flags.rest == 0 && find_flag);
  assert(y == trap_y && x == trap_x);
  record(CHANGE);
  /* Dispatch and damage must read the tile after revealing the trap. */
  cave[y][x].tptr = 2;
}

void __wrap_lite_spot(long y, long x) {
  assert(event_count > 0 && events[event_count - 1] == CHANGE);
  assert(y == char_row && x == char_col);
  assert(y != trap_y && x != trap_x && find_flag);
  record(LIGHT);
}

long __wrap_damroll(const char *dice) {
  assert(event_count > 0 && events[event_count - 1] == LIGHT);
  assert(!find_flag);
  assert(strcmp(dice, "3d7") == 0);
  record(ROLL);
  /* Keep arrow hit checks sensitive to damage-roll ordering. */
  if (expected_subval == 2)
    (void)randint(1000000);
  if (subval_after_roll != 0) {
    t_list[3] = t_list[2];
    t_list[3].subval = subval_after_roll;
    cave[trap_y][trap_x].tptr = 3;
  }
  return rolled_damage;
}

void __wrap_check_store_hours_and_enter(enum store_t type, long visual) {
  assert(type == expected_store && visual == expected_store);
  handler();
}

bool __wrap_check_store_hours(enum store_t type, long visual) {
  assert(type == expected_store && visual == -1);
  handler();
  return store_open;
}

static void special_entry(long subval, enum store_t store) {
  assert(expected_subval == subval && expected_store == store && store_open);
  assert(event_count > 0 && events[event_count - 1] == ROUTE);
  record(ROUTE);
}

void __wrap_enter_trading_post(void) { special_entry(108, S_TRADE_POST); }
void __wrap_enter_bank(void) { special_entry(112, S_BANK); }
void __wrap_enter_casino(void) { special_entry(115, S_CASINO); }

void __wrap_enter_fortress(void) {
  assert(expected_subval == 117);
  handler();
}

void __wrap_enter_house(long y, long x) {
  assert(expected_subval >= 120 && expected_subval <= 122);
  assert(y == trap_y && x == trap_x);
  handler();
}

void __wrap_take_hit(long damage, const char *cause) {
  if (chest_trap_mode) {
    if (strcmp(cause, "a poison needle") == 0)
      assert(damage >= 1 && damage <= 4);
    else if (strcmp(cause, "a poison needle.") == 0)
      assert(damage >= 1 && damage <= 6);
    else {
      assert(strcmp(cause, "an exploding chest") == 0);
      assert(damage >= 5 && damage <= 40);
      assert(cave[trap_y][trap_x].tptr == 0);
    }
    assert(chest_damage_count < sizeof(chest_damage_values) /
                                    sizeof(chest_damage_values[0]));
    chest_damage_values[chest_damage_count++] = damage;
    if (event_count > 0)
      assert(events[event_count - 1] == LOSE_STAT ||
             events[event_count - 1] == DAMAGE || events[event_count - 1] == PUSH);
    record(DAMAGE);
    return;
  }
  assert(damage == expected_damage);
  assert(expected_cause != NULL && strcmp(cause, expected_cause) == 0);
  assert(event_count > 0);
  assert(events[event_count - 1] == ROLL ||
         events[event_count - 1] == LOSE_STAT ||
         (expected_subval == 123 && events[event_count - 1] == DAMAGE));
  if (expected_subval == 3) {
    assert(!moria_flag && !teleport_flag && dun_level == 12);
    assert(cave[trap_y][trap_x].tptr == 2);
    assert(t_list[MAX_TALLOC].p1 == MAX_TALLOC - 1);
    assert(C_message_capture_count() == 1);
  } else if (expected_subval == 4 || expected_subval == 20) {
    assert(moria_flag && !teleport_flag);
    assert(dun_level == expected_movement_level);
    assert(C_message_capture_count() == 2);
  } else if (expected_subval == 123) {
    assert(moria_flag && !teleport_flag);
    assert(dun_level == 12 + (long)event_count - 4);
    assert(C_message_capture_count() == event_count - 2);
  }
  record(DAMAGE);
}

void __real_pusht(long index);
void __real_place_random_dungeon_item(long y, long x);

void __wrap_pusht(long index) {
  record(PUSH);
  pushed_count++;
  pushed_index = index;
  __real_pusht(index);
}

void __wrap_place_random_dungeon_item(long y, long x) {
  record(ITEM);
  dungeon_item_count++;
  dungeon_item_y = y;
  dungeon_item_x = x;
  __real_place_random_dungeon_item(y, x);
}

bool __wrap_summon_land_monster(int64_t *y, int64_t *x, bool asleep) {
  assert(*y == (chest_trap_mode ? trap_y : char_row));
  assert(*x == (chest_trap_mode ? trap_x : char_col) && !asleep);
  record(LAND_SUMMON);
  land_summon_count++;
  return true;
}

bool __wrap_summon_water_monster(int64_t *y, int64_t *x, bool asleep) {
  assert(*y == (chest_trap_mode ? trap_y : char_row));
  assert(*x == (chest_trap_mode ? trap_x : char_col) && !asleep);
  record(WATER_SUMMON);
  water_summon_count++;
  return true;
}

bool __wrap_lose_stat(enum stat_t stat, const char *message,
                      const char *sustain_message) {
  if (chest_trap_mode) {
    assert(stat == STR);
    assert(message != NULL && strcmp(message, "") == 0);
    assert(sustain_message != NULL &&
           strcmp(sustain_message, "You are unaffected.") == 0);
    record(LOSE_STAT);
    if (player_flags.sustain[stat])
      msg_print(sustain_message);
    return true;
  }
  assert(event_count > 0 && events[event_count - 1] == ROLL);
  assert(message != NULL && strcmp(message, "") == 0);
  assert(sustain_message != NULL &&
         strcmp(sustain_message, "A small dart hits you.") == 0);
  assert(stat == STR || stat == CON);
  record(LOSE_STAT);
  if (player_flags.sustain[stat])
    msg_print(sustain_message);
  return true;
}

static void reset_case(long subval, bool searching, bool resting) {
  memset(cave, 0, sizeof(cave));
  tlink();
  memset(&player_flags, 0, sizeof(player_flags));
  memset(equipment, 0, sizeof(equipment));
  /* Rust-owned accessors cannot reliably be intercepted with GNU --wrap. */
  equipment[Equipment_armor].ac = 17 - C_player_ac_from_dex();
  equipment[Equipment_armor].toac = 5;
  assert(player_pac() == 17 && player_ptoac() == 5);
  char_row = 3;
  char_col = 4;
  cur_height = 20;
  cur_width = 30;
  search_flag = searching;
  player_flags.rest = resting ? 12 : 0;
  find_flag = true;
  cave[trap_y][trap_x].tptr = 1;
  t_list[1].subval = 119;
  t_list[1].tval = seen_trap;
  strcpy(t_list[1].damage, "1d1");
  t_list[2].subval = subval;
  t_list[2].tval = seen_trap;
  strcpy(t_list[2].damage, "3d7");
  expected_subval = subval;
  subval_after_roll = 0;
  expected_cause = NULL;
  expected_damage = rolled_damage;
  dun_level = 12;
  moria_flag = false;
  teleport_flag = false;
  print_stat = 0;
  event_count = 0;
  pushed_count = 0;
  pushed_index = 0;
  dungeon_item_count = 0;
  dungeon_item_y = 0;
  dungeon_item_x = 0;
  land_summon_count = 0;
  water_summon_count = 0;
  chest_trap_mode = false;
  chest_damage_count = 0;
  C_message_capture_begin();
}

static void assert_events(const enum event *expected, size_t count) {
  if (event_count != count)
    fprintf(stderr, "Trap %ld: expected %zu events, got %zu\n",
            expected_subval, count, event_count);
  assert(event_count == count);
  for (size_t i = 0; i < count; i++) {
    if (events[i] != expected[i])
      fprintf(stderr, "Trap %ld, event %zu: expected %d, got %d\n",
              expected_subval, i, expected[i], events[i]);
    assert(events[i] == expected[i]);
  }
}

static void assert_message(const char *expected) {
  char message[256];
  assert(C_message_capture_count() == 1);
  assert(C_message_capture_get(0, message, sizeof(message)));
  assert(strcmp(message, expected) == 0);
}

static void assert_messages(const char *const *expected, size_t count) {
  char message[256];
  assert(C_message_capture_count() == count);
  for (size_t i = 0; i < count; i++) {
    assert(C_message_capture_get(i, message, sizeof(message)));
    assert(strcmp(message, expected[i]) == 0);
  }
}

static void finish_case(void) {
  assert(!find_flag && !search_flag && player_flags.rest == 0);
  C_message_capture_end();
}

static void assert_prelude_conditionals(void) {
  for (int searching = 0; searching < 2; searching++) {
    for (int resting = 0; resting < 2; resting++) {
      reset_case(19, searching, resting);
      hit_trap(&trap_y, &trap_x);
      enum event expected[5];
      size_t count = 0;
      if (searching)
        expected[count++] = SEARCH;
      if (resting)
        expected[count++] = REST;
      expected[count++] = CHANGE;
      expected[count++] = LIGHT;
      expected[count++] = ROLL;
      assert_events(expected, count);
      assert(C_message_capture_count() == 0);
      finish_case();
    }
  }
}

static void assert_regular_stores(void) {
  const struct {
    long subval;
    enum store_t store;
  } stores[] = {{101, S_GENERAL}, {102, S_ARMORY}, {103, S_WEAPONS},
                {104, S_TEMPLE},  {105, S_ALCHEMY}, {106, S_MAGIC},
                {107, S_INN},     {109, S_LIBRARY}, {110, S_MUSIC},
                {113, S_GEM},     {116, S_DELI},    {118, S_BLACK_MARKET}};
  const enum event expected[] = {SEARCH, REST, CHANGE, LIGHT, ROLL, ROUTE};
  for (size_t i = 0; i < sizeof(stores) / sizeof(stores[0]); i++) {
    reset_case(stores[i].subval, true, true);
    expected_store = stores[i].store;
    hit_trap(&trap_y, &trap_x);
    assert_events(expected, sizeof(expected) / sizeof(expected[0]));
    assert(C_message_capture_count() == 0);
    finish_case();
  }
}

static void assert_special_hours(void) {
  const struct {
    long subval;
    enum store_t store;
  } stores[] = {{108, S_TRADE_POST}, {112, S_BANK},
                {114, S_CHANGER}, {115, S_CASINO}};
  const enum event expected[] = {SEARCH, REST, CHANGE, LIGHT,
                                 ROLL, ROUTE, ROUTE};
  for (size_t i = 0; i < sizeof(stores) / sizeof(stores[0]); i++) {
    for (int open = 0; open < 2; open++) {
      reset_case(stores[i].subval, true, true);
      expected_store = stores[i].store;
      store_open = open;
      hit_trap(&trap_y, &trap_x);
      assert_events(expected, open && stores[i].subval != 114 ? 7 : 6);
      if (open && stores[i].subval == 114)
        assert_message("Oh, just go to the bloody bank!");
      else
        assert(C_message_capture_count() == 0);
      finish_case();
    }
  }
}

static void assert_special_tiles(void) {
  const long subvals[] = {111, 117, 119, 120, 121, 122, 0,
                          124, 999, -1, LONG_MIN, LONG_MAX};
  const enum event expected[] = {SEARCH, REST, CHANGE, LIGHT, ROLL, ROUTE};
  for (size_t i = 0; i < sizeof(subvals) / sizeof(subvals[0]); i++) {
    const long subval = subvals[i];
    reset_case(subval, true, true);
    hit_trap(&trap_y, &trap_x);
    const bool enters = subval == 117 || (subval >= 120 && subval <= 122);
    assert_events(expected, enters ? 6 : 5);
    if (enters)
      assert(C_message_capture_count() == 0);
    else if (subval == 111)
      assert_message("The insurance shop has gone out of business.");
    else
      assert_message("You got lucky: unknown trap value.");
    finish_case();
  }
}

static void assert_open_pit(void) {
  const enum event expected[] = {SEARCH, REST, CHANGE, LIGHT, ROLL, DAMAGE};
  for (int feather = 0; feather < 2; feather++) {
    reset_case(1, true, true);
    player_flags.ffall = feather;
    expected_cause = "an open pit";
    hit_trap(&trap_y, &trap_x);
    assert_events(expected, feather ? 5 : 6);
    assert(C_message_capture_count() == (feather ? 2 : 1));
    char message[128];
    assert(C_message_capture_get(0, message, sizeof(message)));
    assert(strcmp(message, "You fell into a pit!") == 0);
    if (feather) {
      assert(C_message_capture_get(1, message, sizeof(message)));
      assert(strcmp(message, "You gently float down.") == 0);
    }
    finish_case();
  }
}

static void assert_arrow(void) {
  const enum event expected[] = {SEARCH, REST, CHANGE, LIGHT, ROLL, DAMAGE};
  size_t hits = 0, misses = 0;
  for (unsigned long long seed = 0; seed < 64; seed++) {
    reset_case(2, true, true);
    C_seeded_rng_begin(seed);
    (void)randint(1000000);
    const bool hit = managed_to_hit(125, 0, 0, 22);
    const long next_roll = randint(1000000);
    C_seeded_rng_end();
    expected_cause = "an arrow trap";
    C_seeded_rng_begin(seed);
    hit_trap(&trap_y, &trap_x);
    assert(randint(1000000) == next_roll);
    C_seeded_rng_end();
    assert_events(expected, hit ? 6 : 5);
    assert_message(hit ? "An arrow hits you." : "An arrow barely misses you.");
    if (hit)
      hits++;
    else
      misses++;
    finish_case();
  }
  assert(hits > 0 && misses > 0);
}

static void force_trap_hit(void) {
  equipment[Equipment_armor].ac = -C_player_ac_from_dex();
  equipment[Equipment_armor].toac = 0;
}

static void assert_status_traps(void) {
  const enum event prelude[] = {SEARCH, REST, CHANGE, LIGHT, ROLL};
  const char *sleep_resisted[] = {
      "A strange white mist surrounds you!", "You are unaffected."};
  const char *sleep_unresisted[] = {
      "A strange white mist surrounds you!", "You fall asleep."};
  const char *blind[] = {"A black gas surrounds you!"};
  const char *confused[] = {"A gas of scintillating colors surrounds you!"};
  const char *dart_hit[] = {"A small dart hits you!"};
  const char *dart_weakened[] = {
      "A small dart hits you.", "A small dart weakens you!"};

  reset_case(5, true, true);
  player_flags.free_act = true;
  C_seeded_rng_begin(42);
  hit_trap(&trap_y, &trap_x);
  C_seeded_rng_end();
  assert_events(prelude, sizeof(prelude) / sizeof(prelude[0]));
  assert(player_flags.paralysis == 0);
  assert_messages(sleep_resisted, 2);
  finish_case();

  reset_case(5, true, true);
  C_seeded_rng_begin(42);
  hit_trap(&trap_y, &trap_x);
  C_seeded_rng_end();
  assert_events(prelude, sizeof(prelude) / sizeof(prelude[0]));
  assert(player_flags.paralysis >= 5 && player_flags.paralysis <= 14);
  assert_messages(sleep_unresisted, 2);
  finish_case();

  reset_case(5, true, true);
  player_flags.paralysis = 3;
  C_seeded_rng_begin(42);
  hit_trap(&trap_y, &trap_x);
  C_seeded_rng_end();
  assert_events(prelude, sizeof(prelude) / sizeof(prelude[0]));
  assert(player_flags.paralysis == 3);
  assert(C_message_capture_count() == 0);
  finish_case();

  reset_case(15, true, true);
  C_seeded_rng_begin(42);
  hit_trap(&trap_y, &trap_x);
  C_seeded_rng_end();
  assert_events(prelude, sizeof(prelude) / sizeof(prelude[0]));
  assert(player_flags.blind >= 51 && player_flags.blind <= 100);
  assert_messages(blind, 1);
  finish_case();

  reset_case(16, true, true);
  C_seeded_rng_begin(42);
  hit_trap(&trap_y, &trap_x);
  C_seeded_rng_end();
  assert_events(prelude, sizeof(prelude) / sizeof(prelude[0]));
  assert(player_flags.confused >= 16 && player_flags.confused <= 30);
  assert_messages(confused, 1);
  finish_case();

  reset_case(17, true, true);
  force_trap_hit();
  expected_cause = "a dart trap";
  C_seeded_rng_begin(42);
  hit_trap(&trap_y, &trap_x);
  C_seeded_rng_end();
  const enum event slow_events[] = {SEARCH, REST, CHANGE, LIGHT, ROLL, DAMAGE};
  assert_events(slow_events, sizeof(slow_events) / sizeof(slow_events[0]));
  assert(player_flags.slow >= 11 && player_flags.slow <= 30);
  assert_messages(dart_hit, 1);
  finish_case();

  reset_case(7, true, true);
  force_trap_hit();
  player_flags.sustain[STR] = true;
  expected_cause = "a dart trap";
  C_seeded_rng_begin(42);
  hit_trap(&trap_y, &trap_x);
  C_seeded_rng_end();
  const enum event strength_events[] = {
      SEARCH, REST, CHANGE, LIGHT, ROLL, LOSE_STAT, DAMAGE};
  assert_events(strength_events,
                sizeof(strength_events) / sizeof(strength_events[0]));
  assert((print_stat & 0x0001) != 0);
  assert_messages(dart_weakened, 2);
  finish_case();

  reset_case(18, true, true);
  force_trap_hit();
  player_flags.sustain[CON] = true;
  expected_cause = "a dart trap";
  C_seeded_rng_begin(42);
  hit_trap(&trap_y, &trap_x);
  C_seeded_rng_end();
  const enum event constitution_events[] = {
      SEARCH, REST, CHANGE, LIGHT, ROLL, LOSE_STAT, DAMAGE};
  assert_events(constitution_events,
                sizeof(constitution_events) / sizeof(constitution_events[0]));
  assert((print_stat & 0x0004) != 0);
  assert_messages(dart_weakened, 2);
  finish_case();
}

static void assert_element_traps(void) {
  const enum event expected[] = {SEARCH, REST, CHANGE, LIGHT, ROLL, DAMAGE};
  const struct {
    long subval;
    const char *cause;
    const char *message;
  } traps[] = {
      {10, "corrosion gas.", "A strange red gas surrounds you."},
      {12, "a fire trap.", "You are enveloped in flames!"},
      {13, "an acid trap.", "You are splashed with acid!"},
      {14, "a poison gas trap.", "A pungent green gas surrounds you!"},
  };

  for (size_t i = 0; i < sizeof(traps) / sizeof(traps[0]); i++) {
    reset_case(traps[i].subval, true, true);

    expected_cause = traps[i].cause;
    if (traps[i].subval == 10) {
      C_seeded_rng_begin(42);
      expected_damage = randint(8);
      C_seeded_rng_end();
    }

    C_seeded_rng_begin(42);
    hit_trap(&trap_y, &trap_x);
    C_seeded_rng_end();
    assert_events(expected, sizeof(expected) / sizeof(expected[0]));
    assert(print_stat == 1);
    if (traps[i].subval == 14)
      assert(player_flags.poisoned >= 13 && player_flags.poisoned <= 35);
    assert_message(traps[i].message);
    finish_case();
  }
}

static void assert_hidden_object_trap(void) {
  reset_case(6, true, true);
  cave[trap_y][trap_x].fm = true;
  C_seeded_rng_begin(42);
  hit_trap(&trap_y, &trap_x);
  C_seeded_rng_end();
  const enum event expected[] = {
      SEARCH, REST, CHANGE, LIGHT, ROLL, PUSH, ITEM};
  assert_events(expected, sizeof(expected) / sizeof(expected[0]));
  assert(!cave[trap_y][trap_x].fm);
  assert(pushed_count == 1 && pushed_index == 2);
  assert(dungeon_item_count == 1);
  assert(dungeon_item_y == trap_y && dungeon_item_x == trap_x);
  assert(cave[trap_y][trap_x].tptr == 2);
  assert(t_list[2].tval != 0);
  assert_message("Hmmm, there was something under this rock.");
  finish_case();
}

static void assert_rockfall_trap(void) {
  reset_case(9, true, true);
  expected_cause = "falling rock";
  cave[trap_y][trap_x].fopen = true;
  hit_trap(&trap_y, &trap_x);
  const enum event expected[] = {
      SEARCH, REST, CHANGE, LIGHT, ROLL, DAMAGE, PUSH};
  assert_events(expected, sizeof(expected) / sizeof(expected[0]));
  assert(pushed_count == 1 && pushed_index == 2);
  assert(cave[trap_y][trap_x].tptr != 0);
  assert(t_list[cave[trap_y][trap_x].tptr].tval == rubble);
  assert(!cave[trap_y][trap_x].fopen);
  assert_message("You are hit by falling rock");
  finish_case();
}

static void assert_summon_monster_trap(void) {
  bool saw_single = false, saw_multiple = false;
  for (int water = 0; water < 2; water++) {
    for (unsigned long long seed = 0; seed < 64; seed++) {
      reset_case(11, true, true);
      cave[char_row][char_col].fval =
          water ? water_set[0] : ft_dark_open_floor;

      C_seeded_rng_begin(seed);
      long expected_summons = 0;
      for (long i = 1; i <= 2 + randint(3); i++)
        expected_summons++;
      const long next_roll = randint(1000000);
      C_seeded_rng_end();
      saw_single |= expected_summons == 3;
      saw_multiple |= expected_summons > 1;

      C_seeded_rng_begin(seed);
      hit_trap(&trap_y, &trap_x);
      assert(randint(1000000) == next_roll);
      C_seeded_rng_end();

      const size_t expected_count = (size_t)expected_summons;
      assert(land_summon_count == (water ? 0 : expected_count));
      assert(water_summon_count == (water ? expected_count : 0));
      assert(pushed_count == 1 && pushed_index == 2);
      assert(cave[trap_y][trap_x].tptr == 0);
      assert(!cave[trap_y][trap_x].fm);
      const enum event prelude[] = {SEARCH, REST, CHANGE, LIGHT, ROLL, PUSH};
      for (size_t i = 0; i < sizeof(prelude) / sizeof(prelude[0]); i++)
        assert(events[i] == prelude[i]);
      for (size_t i = sizeof(prelude) / sizeof(prelude[0]);
           i < event_count; i++)
        assert(events[i] == (water ? WATER_SUMMON : LAND_SUMMON));
      finish_case();
    }
  }
  assert(saw_single && saw_multiple);
}

static void assert_migrated_null_rejection(void) {
  reset_case(119, true, true);
  hit_trap(NULL, &trap_x);
  hit_trap(&trap_y, NULL);
  hit_trap(NULL, NULL);
  assert(event_count == 0);
  assert(search_flag && player_flags.rest == 12 && find_flag);
  assert(cave[trap_y][trap_x].tptr == 1);
  assert(C_message_capture_count() == 0);
  C_message_capture_end();
}

static void assert_migrated_dispatch_reads_after_roll(void) {
  reset_case(19, true, true);
  subval_after_roll = expected_subval = 3;
  player_flags.ffall = true;
  hit_trap(&trap_y, &trap_x);
  const enum event expected[] = {SEARCH, REST, CHANGE, LIGHT, ROLL};
  assert_events(expected, sizeof(expected) / sizeof(expected[0]));
  assert(cave[trap_y][trap_x].tptr == 3);
  const char *messages[] = {
      "You fell into a covered pit.", "You gently float down."};
  assert_messages(messages, 2);
  finish_case();
}

static void assert_chest_traps(void) {
  const struct {
    unsigned long flag;
    const char *const *messages;
    size_t message_count;
  } cases[] = {
      {0x10, (const char *const[]){"A small needle has pricked you!",
                                   "You feel weakened!"}, 2},
      {0x20, (const char *const[]){"A small needle has pricked you!"}, 1},
      {0x40, (const char *const[]){"A puff of yellow gas surrounds you!",
                                   "You choke and pass out."}, 2},
      {0x80, (const char *const[]){"There is a sudden explosion!"}, 1},
      {0x100, NULL, 0},
  };
  for (size_t i = 0; i < sizeof(cases) / sizeof(cases[0]); i++) {
    for (int water = 0; water < 2; water++) {
      long expected_damage[3] = {0};
      size_t expected_damage_count = 0;
      long expected_poison = 0, expected_paralysis = 0;
      C_seeded_rng_begin(42);
      if (cases[i].flag == 0x10)
        expected_damage[expected_damage_count++] = randint(4);
      if (cases[i].flag == 0x20) {
        expected_damage[expected_damage_count++] = randint(6);
        expected_poison = 10 + randint(20);
      }
      if (cases[i].flag == 0x40)
        expected_paralysis = 10 + randint(20);
      if (cases[i].flag == 0x80) {
        long damage = 0;
        for (int die = 0; die < 5; die++)
          damage += randint(8);
        expected_damage[expected_damage_count++] = damage;
      }
      const long expected_next_rng = randint(1000000);
      C_seeded_rng_end();

      reset_case(119, false, false);
      chest_trap_mode = true;
      find_flag = false;
      cave[trap_y][trap_x].tptr = 2;
      cave[trap_y][trap_x].fval = water ? water_set[0] : 0;
      t_list[2].flags = cases[i].flag;
      C_seeded_rng_begin(42);
      trigger_trap(trap_y, trap_x);
      assert(randint(1000000) == expected_next_rng);
      C_seeded_rng_end();
      assert(chest_damage_count == expected_damage_count);
      for (size_t damage = 0; damage < expected_damage_count; damage++)
        assert(chest_damage_values[damage] == expected_damage[damage]);
      assert_messages(cases[i].messages, cases[i].message_count);
      if (cases[i].flag == 0x10)
        assert(player_flags.sustain[STR] == 0 && print_stat == 1);
      if (cases[i].flag == 0x20)
        assert(player_flags.poisoned == expected_poison);
      if (cases[i].flag == 0x40)
        assert(player_flags.paralysis == expected_paralysis);
      if (cases[i].flag == 0x80)
        assert(cave[trap_y][trap_x].tptr == 0);
      if (cases[i].flag == 0x100) {
        assert(water ? water_summon_count == 3 : land_summon_count == 3);
        assert(C_message_capture_count() == 0);
      }
      if (cases[i].flag == 0x10)
        assert(event_count == 2 && events[0] == LOSE_STAT &&
               events[1] == DAMAGE);
      if (cases[i].flag == 0x20)
        assert(event_count == 1 && events[0] == DAMAGE);
      if (cases[i].flag == 0x80)
        assert(event_count == 1 && events[0] == DAMAGE);
      finish_case();
    }
  }

  reset_case(119, false, false);
  chest_trap_mode = true;
  find_flag = false;
  cave[trap_y][trap_x].tptr = 2;
  t_list[2].flags = 0x1f0;
  long combined_damage[3];
  C_seeded_rng_begin(42);
  combined_damage[0] = randint(4);
  combined_damage[1] = randint(6);
  const long combined_poison = 10 + randint(20);
  const long combined_paralysis = 10 + randint(20);
  combined_damage[2] = 0;
  for (int die = 0; die < 5; die++)
    combined_damage[2] += randint(8);
  const long combined_next_rng = randint(1000000);
  C_seeded_rng_end();
  C_seeded_rng_begin(42);
  trigger_trap(trap_y, trap_x);
  assert(randint(1000000) == combined_next_rng);
  C_seeded_rng_end();
  const char *combined_messages[] = {
      "A small needle has pricked you!", "You feel weakened!",
      "A small needle has pricked you!", "A puff of yellow gas surrounds you!",
      "You choke and pass out.", "There is a sudden explosion!"};
  assert_messages(combined_messages,
                  sizeof(combined_messages) / sizeof(combined_messages[0]));
  assert(player_flags.poisoned == combined_poison);
  assert(player_flags.paralysis == combined_paralysis);
  assert(chest_damage_count == 3);
  for (size_t damage = 0; damage < chest_damage_count; damage++)
    assert(chest_damage_values[damage] == combined_damage[damage]);
  assert(print_stat == 1 && cave[trap_y][trap_x].tptr == 0);
  assert(land_summon_count == 3 && water_summon_count == 0);
  assert(event_count == 7);
  assert(events[0] == LOSE_STAT && events[1] == DAMAGE);
  assert(events[2] == DAMAGE && events[3] == DAMAGE);
  assert(events[4] == LAND_SUMMON && events[5] == LAND_SUMMON &&
         events[6] == LAND_SUMMON);
  finish_case();

  reset_case(119, false, false);
  chest_trap_mode = true;
  find_flag = false;
  trigger_trap(-1, trap_x);
  assert(C_message_capture_count() == 0 && event_count == 0);
  assert(player_flags.poisoned == 0 && cave[trap_y][trap_x].tptr == 1);
  C_message_capture_end();

  reset_case(119, false, false);
  chest_trap_mode = true;
  find_flag = false;
  cave[trap_y][trap_x].tptr = UCHAR_MAX;
  trigger_trap(trap_y, trap_x);
  assert(C_message_capture_count() == 0 && event_count == 0);
  assert(player_flags.poisoned == 0 && cave[trap_y][trap_x].tptr == UCHAR_MAX);
  C_message_capture_end();
}

static void assert_guard_rejected(void) {
  hit_trap(&trap_y, &trap_x);
  assert(event_count == 0);
  assert(search_flag && player_flags.rest == 12 && find_flag);
  assert(C_message_capture_count() == 0);
  C_message_capture_end();
}

static void assert_migrated_input_guards(void) {
  const long invalid_y[] = {-1, 0, 1, 20, 21};
  const long invalid_x[] = {-1, 0, 1, 30, 31};
  for (size_t i = 0; i < sizeof(invalid_y) / sizeof(invalid_y[0]); i++) {
    reset_case(119, true, true);
    trap_y = invalid_y[i];
    assert_guard_rejected();
    trap_y = 7;
  }
  for (size_t i = 0; i < sizeof(invalid_x) / sizeof(invalid_x[0]); i++) {
    reset_case(119, true, true);
    trap_x = invalid_x[i];
    assert_guard_rejected();
    trap_x = 9;
  }
  reset_case(119, true, true);
  cave[trap_y][trap_x].tptr = 0;
  assert_guard_rejected();

  const long invalid_hidden_subvals[] = {0, 21, 119};
  for (size_t i = 0;
       i < sizeof(invalid_hidden_subvals) / sizeof(invalid_hidden_subvals[0]);
       i++) {
    reset_case(119, true, true);
    t_list[1].tval = unseen_trap;
    t_list[1].subval = invalid_hidden_subvals[i];
    assert_guard_rejected();
  }

  reset_case(119, true, true);
  memset(t_list[2].damage, '1', sizeof(t_list[2].damage));
  hit_trap(&trap_y, &trap_x);
  const enum event expected[] = {SEARCH, REST, CHANGE, LIGHT};
  assert_events(expected, sizeof(expected) / sizeof(expected[0]));
  assert(!find_flag && !search_flag && player_flags.rest == 0);
  assert(C_message_capture_count() == 0);
  C_message_capture_end();
}

static void assert_movement_traps(void) {
  const struct {
    long subval;
    const char *message;
    const char *cause;
  } traps[] = {
      {3, "You fell into a covered pit.", "a covered pit"},
      {4, "You fell through a trap door!", "a trap door"},
      {8, "You hit a teleport trap!", NULL},
      {20, "You fell down a chute!", "chute landing"},
      {123, "You are swept into a whirlpool!", "drowning"},
  };
  bool saw_single = false, saw_multiple = false;
  bool saw_shallow_chute = false, saw_deep_chute = false;
  for (size_t i = 0; i < sizeof(traps) / sizeof(traps[0]); i++) {
    const long subval = traps[i].subval;
    for (int feather = 0; feather < 2; feather++) {
      for (unsigned long long seed = 0; seed < 64; seed++) {
        reset_case(subval, true, true);
        player_flags.ffall = feather;
        expected_cause = traps[i].cause;
        long levels = subval == 4 ? 1 : 0;
        C_seeded_rng_begin(seed);
        if (subval == 20) {
          levels = randint(6);
          saw_shallow_chute |= levels == 1;
          saw_deep_chute |= levels == 6;
        } else if (subval == 123) {
          do {
            levels++;
          } while (randint(2) != 1);
          saw_single |= levels == 1;
          saw_multiple |= levels > 1;
        }
        const long next_roll = randint(1000000);
        C_seeded_rng_end();
        expected_movement_level = 12 + levels;
        C_seeded_rng_begin(seed);
        hit_trap(&trap_y, &trap_x);
        assert(randint(1000000) == next_roll);
        C_seeded_rng_end();
        assert(dun_level == expected_movement_level);
        assert(moria_flag == (subval == 4 || subval == 20 || subval == 123));
        assert(teleport_flag == (subval == 8));
        assert(char_row == 3 && char_col == 4);
        assert(cave[char_row][char_col].tptr == 0);
        const bool replaces = subval == 3 && !feather;
        assert(cave[trap_y][trap_x].tptr == (replaces ? MAX_TALLOC : 2));
        assert(t_list[cave[trap_y][trap_x].tptr].subval ==
               (replaces ? 1 : subval));
        if (replaces) {
          assert(t_list[MAX_TALLOC].tval == seen_trap);
          assert(strcmp(t_list[MAX_TALLOC].damage, "2d6") == 0);
        }
        const long damages = feather || subval == 8 ? 0 :
                             subval == 123 ? levels : 1;
        assert(event_count == 5 + (size_t)damages);
        const enum event prelude[] = {SEARCH, REST, CHANGE, LIGHT, ROLL};
        for (size_t j = 0; j < 5; j++)
          assert(events[j] == prelude[j]);
        for (size_t j = 5; j < event_count; j++)
          assert(events[j] == DAMAGE);
        const char *messages[128];
        size_t count = 0;
        messages[count++] = traps[i].message;
        if (subval == 4 || subval == 20 || subval == 123)
          messages[count++] = " ";
        if (feather && subval != 8 && subval != 123)
          messages[count++] = subval == 20 ? "You gently slide down." :
                                            "You gently float down.";
        if (subval == 123 && !feather)
          for (long j = 0; j < levels; j++) {
            assert(count < sizeof(messages) / sizeof(messages[0]));
            messages[count++] = "You are drowning!";
          }
        assert_messages(messages, count);
        finish_case();
      }
    }
  }
  assert(saw_single && saw_multiple);
  assert(saw_shallow_chute && saw_deep_chute);
}

int main(void) {
  for (int repeat = 0; repeat < 2; repeat++) {
    assert_prelude_conditionals();
    assert_regular_stores();
    assert_special_hours();
    assert_special_tiles();
    assert_open_pit();
    assert_arrow();
    if (TRAP_TEST_MIGRATED) {
      assert_status_traps();
      assert_element_traps();
      assert_movement_traps();
      assert_hidden_object_trap();
      assert_rockfall_trap();
      assert_summon_monster_trap();
      assert_chest_traps();
      assert_migrated_null_rejection();
      assert_migrated_dispatch_reads_after_roll();
      assert_migrated_input_guards();
    }
  }
  puts(TRAP_TEST_MIGRATED ? "Migrated trap C caller checks passed."
                         : "Legacy trap C caller characterization passed.");
  return 0;
}

#else

int main(void) {
  puts("SKIP trap C caller checks: Linux GNU ld --wrap is required.");
  return 0;
}

#endif
