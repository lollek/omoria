#include <assert.h>
#include <stdio.h>
#include <string.h>

#if defined(__linux__) && defined(TRAP_TEST_GNU_WRAP)

#include "../src/constants.h"
#include "../src/messages.h"
#include "../src/player.h"
#include "../src/stores.h"
#include "../src/traps.h"
#include "../src/variables.h"

/* Legacy hit_trap calls static handlers in its own object, bypassing --wrap.
 * The default exercises the migrated forwarding boundary and checked ABI. */
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
  TO_HIT,
  DAMAGE
};

static enum event events[16];
static size_t event_count;
static long trap_y = 7, trap_x = 9;
static const long rolled_damage = 23;
static bool store_open, arrow_hits;
static enum store_t expected_store;
static long expected_subval;
static long subval_after_roll;
static const char *expected_cause;

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
  assert(damage == rolled_damage);
  assert(expected_cause != NULL && strcmp(cause, expected_cause) == 0);
  assert(event_count > 0);
  assert(events[event_count - 1] == ROLL ||
         events[event_count - 1] == TO_HIT);
  record(DAMAGE);
}

int16_t __wrap_player_pac(void) { return 17; }
int16_t __wrap_player_ptoac(void) { return 5; }

bool __wrap_managed_to_hit(long base, long level, long plus, long ac) {
  assert(base == 125 && level == 0 && plus == 0 && ac == 22);
  assert(event_count > 0 && events[event_count - 1] == ROLL);
  record(TO_HIT);
  return arrow_hits;
}

/* These doubles verify the exact surviving C handler signatures, including
 * damage and coordinates. They are unreachable in the legacy static object. */
#define FORWARD_DAMAGE(name, subval)                                             \
  void __wrap_ht__##name(long damage) {                                         \
    assert(expected_subval == subval && damage == rolled_damage);               \
    handler();                                                                \
  }
#define FORWARD_NONE(name, subval)                                               \
  void __wrap_ht__##name(void) {                                                \
    assert(expected_subval == subval);                                         \
    handler();                                                                \
  }
#define FORWARD_COORDS(name, subval)                                             \
  void __wrap_ht__##name(long y, long x) {                                      \
    assert(expected_subval == subval && y == trap_y && x == trap_x);             \
    handler();                                                                \
  }
#define FORWARD_ALL(name, subval)                                                \
  void __wrap_ht__##name(long damage, long y, long x) {                         \
    assert(expected_subval == subval && damage == rolled_damage);               \
    assert(y == trap_y && x == trap_x);                                         \
    handler();                                                                \
  }

FORWARD_ALL(covered_pit, 3)
FORWARD_DAMAGE(trap_door, 4)
FORWARD_NONE(sleep_gas, 5)
FORWARD_COORDS(hidden_object, 6)
FORWARD_DAMAGE(str_dart, 7)
FORWARD_NONE(teleport, 8)
FORWARD_ALL(rockfall, 9)
FORWARD_NONE(corrode_gas, 10)
FORWARD_COORDS(summon_monster, 11)
FORWARD_DAMAGE(fire, 12)
FORWARD_DAMAGE(acid, 13)
FORWARD_DAMAGE(poison_gas, 14)
FORWARD_NONE(blind_gas, 15)
FORWARD_NONE(confuse_gas, 16)
FORWARD_DAMAGE(slow_dart, 17)
FORWARD_DAMAGE(con_dart, 18)
FORWARD_DAMAGE(chute, 20)
FORWARD_DAMAGE(whirlpool, 123)

static void reset_case(long subval, bool searching, bool resting) {
  memset(cave, 0, sizeof(cave));
  memset(t_list, 0, sizeof(t_list));
  memset(&player_flags, 0, sizeof(player_flags));
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
  event_count = 0;
  C_message_capture_begin();
}

static void assert_events(const enum event *expected, size_t count) {
  assert(event_count == count);
  for (size_t i = 0; i < count; i++)
    assert(events[i] == expected[i]);
}

static void assert_message(const char *expected) {
  char message[256];
  assert(C_message_capture_count() == 1);
  assert(C_message_capture_get(0, message, sizeof(message)));
  assert(strcmp(message, expected) == 0);
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
  const long subvals[] = {111, 117, 119, 120, 121, 122, 0, 124, 999};
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
  const enum event expected[] = {SEARCH, REST, CHANGE, LIGHT,
                                 ROLL, TO_HIT, DAMAGE};
  for (int hit = 0; hit < 2; hit++) {
    reset_case(2, true, true);
    arrow_hits = hit;
    expected_cause = "an arrow trap";
    hit_trap(&trap_y, &trap_x);
    assert_events(expected, hit ? 7 : 6);
    assert_message(hit ? "An arrow hits you." : "An arrow barely misses you.");
    finish_case();
  }
}

static void assert_migrated_forwarding(void) {
  const long subvals[] = {3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
                          13, 14, 15, 16, 17, 18, 20, 123};
  const enum event expected[] = {SEARCH, REST, CHANGE, LIGHT, ROLL, ROUTE};
  for (size_t i = 0; i < sizeof(subvals) / sizeof(subvals[0]); i++) {
    reset_case(subvals[i], true, true);
    hit_trap(&trap_y, &trap_x);
    assert_events(expected, sizeof(expected) / sizeof(expected[0]));
    assert(C_message_capture_count() == 0);
    finish_case();
  }
  reset_case(99, true, true);
  hit_trap(&trap_y, &trap_x);
  assert_events(expected, 5);
  assert(C_message_capture_count() == 0);
  finish_case();
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
  hit_trap(&trap_y, &trap_x);
  const enum event expected[] = {SEARCH, REST, CHANGE, LIGHT, ROLL, ROUTE};
  assert_events(expected, sizeof(expected) / sizeof(expected[0]));
  assert(cave[trap_y][trap_x].tptr == 3);
  assert(C_message_capture_count() == 0);
  finish_case();
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

int main(void) {
  for (int repeat = 0; repeat < 2; repeat++) {
    assert_prelude_conditionals();
    assert_regular_stores();
    assert_special_hours();
    assert_special_tiles();
    assert_open_pit();
    assert_arrow();
    if (TRAP_TEST_MIGRATED) {
      assert_migrated_forwarding();
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
