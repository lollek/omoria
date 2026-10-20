#include <assert.h>
#include <stdio.h>
#include <string.h>

#include "../src/constants.h"
#include "../src/floor.h"
#include "../src/messages.h"
#include "../src/misc.h"
#include "../src/player.h"
#include "../src/player/hunger.h"
#include "../src/random.h"
#include "../src/screen.h"
#include "../src/variables.h"
#include "support/terminal.h"

static void reset_case(long foodc, unsigned long status) {
  memset(&player_flags, 0, sizeof(player_flags));
  memset(cave, 0, sizeof(cave));
  memset(t_list, 0, sizeof(t_list));
  player_flags.foodc = foodc;
  player_flags.status = status;
  player_flags.rest = 10;
  player_flags.resting_till_full = true;
  player_flags.paralysis = 7;
  player_flags.blind = 1;
  player_wt = 200;
  death = moria_flag = false;
  total_winner = true;
  strcpy(died_from, "unchanged");
  msg_flag = true;
  msg_terse = false;
  find_flag = search_flag = false;
  cur_height = cur_width = 3;
  char_row = char_col = 2;
  cave[2][2].cptr = 1;
  cave[2][2].fopen = true;
  cave[2][2].fval = ft_dark_open_floor;
  panel_row = panel_col = max_panel_rows = max_panel_cols = 0;
  panel_row_min = panel_col_min = 1;
  panel_row_max = SCREEN_HEIGHT;
  panel_col_max = SCREEN_WIDTH;
  panel_row_prt = -1;
  panel_col_prt = -14;
  cave_flag = true;
  memset(used_line, 0, sizeof(used_line));
  headless_terminal_reset();
  headless_terminal_allow_drawing(true);
  C_message_capture_begin();
}

static void assert_message(size_t index, const char *expected) {
  char buffer[256];
  assert(C_message_capture_get(index, buffer, sizeof(buffer)));
  assert(strcmp(buffer, expected) == 0);
}

static void finish_case(size_t messages) {
  assert(C_message_capture_count() == messages);
  assert(headless_terminal_counts().input == 0);
  C_message_capture_end();
}

static void assert_transitions_and_stationary_move(void) {
  for (int running = 0; running < 2; running++) {
    reset_case(1999, IS_RESTING | 0x80);
    find_flag = running;
    C_seeded_rng_begin(42);
    player_hunger_recalculate();
    C_seeded_rng_end();
    assert(player_flags.foodc == 1999 && player_wt == 200);
    assert(player_flags.status == (IS_HUNGERY | 0x80));
    assert(player_flags.rest == 0 && !player_flags.resting_till_full);
    assert(!msg_flag && !find_flag);
    assert(char_row == 2 && char_col == 2 && cave[2][2].cptr == 1);
    assert(!death && !moria_flag && total_winner);
    assert_message(0, "You are getting hungry.");
    player_hunger_recalculate();
    finish_case(1);
  }
}

static void assert_weakness_and_starvation(void) {
  const unsigned short minimum = min_allowable_weight();
  assert(minimum > 0);
  for (int starves = 0; starves < 2; starves++) {
    reset_case(999, IS_RESTING | 0x80);
    player_wt = starves ? minimum - 1 : 200;
    const unsigned short expected = player_wt - (unsigned short)(player_wt * 0.015);
    assert((expected < minimum) == starves);
    player_hunger_recalculate();
    assert(player_wt == expected);
    assert(player_flags.foodc == 999);
    assert(player_flags.status == (IS_HUNGERY | IS_WEAK | 0x80));
    assert(player_flags.rest == 0 && msg_flag);
    assert(death == starves && moria_flag == starves);
    assert(total_winner == !starves);
    assert(strcmp(died_from, starves ? "starvation" : "unchanged") == 0);
    assert_message(0, "You are getting weak from hunger.");
    assert_message(1, "Your clothes seem to be getting loose.");
    if (starves)
      assert_message(2, "Oh no...  Now you've done it.");
    player_hunger_recalculate();
    assert(player_wt == expected);
    finish_case(starves ? 3 : 2);
  }
}

static void assert_seeded_fainting(void) {
  bool covered[3] = {false};
  for (unsigned long long seed = 1; seed <= 256; seed++) {
    C_seeded_rng_begin(seed);
    long turns = 0;
    int path = 2;
    if (randint(5) == 1) {
      path = 0;
      turns = randint(3);
    } else if (randint(8) == 1) {
      path = 1;
      turns = randint(5);
    }
    const long next_roll = randint(1000);
    C_seeded_rng_end();
    reset_case(-1, IS_HUNGERY | IS_WEAK | IS_RESTING);
    find_flag = true;
    C_seeded_rng_begin(seed);
    player_hunger_recalculate();
    assert(randint(1000) == next_roll);
    C_seeded_rng_end();
    assert(player_flags.paralysis == 7 + turns);
    assert(player_flags.foodc == -1 && player_wt == 200);
    assert(player_flags.rest == (turns ? 0 : 10));
    assert(find_flag == (turns == 0));
    assert(player_flags.status ==
           (unsigned long)(IS_HUNGERY | IS_WEAK | (turns ? 0 : IS_RESTING)));
    assert(!death && !moria_flag && total_winner);
    assert(char_row == 2 && char_col == 2);
    if (turns)
      assert_message(0, "You faint from the lack of food.");
    finish_case(turns ? 1 : 0);
    covered[path] = true;
  }
  assert(covered[0] && covered[1] && covered[2]);
}

static void assert_hunger_item_and_nonnegative_food(void) {
  reset_case(3000, IS_RESTING);
  player_flags.hunger_item = true;
  player_hunger_recalculate();
  assert(player_flags.foodc == 2015 && player_flags.rest == 10);
  assert(player_flags.status == IS_RESTING);
  finish_case(0);
  reset_case(0, IS_WEAK | IS_HUNGERY | IS_RESTING);
  player_hunger_recalculate();
  assert(player_flags.paralysis == 7 && player_flags.rest == 10);
  assert(player_flags.status == (IS_WEAK | IS_HUNGERY | IS_RESTING));
  finish_case(0);
}

static void assert_sighted_running_can_reenter_rng(void) {
  reset_case(1999, IS_RESTING);
  player_flags.blind = 0;
  player_pclass = 0;
  find_flag = true;
  C_seeded_rng_begin(42);
  player_hunger_recalculate();
  C_seeded_rng_end();
  assert(!find_flag && player_flags.rest == 0);
  assert(char_row == 2 && char_col == 2);
  assert_message(0, "You are getting hungry.");
  finish_case(1);
}

int main(void) {
  player_prace = 0;
  strcpy(player_sex, "Male");
  assert_transitions_and_stationary_move();
  assert_weakness_and_starvation();
  assert_seeded_fainting();
  assert_hunger_item_and_nonnegative_food();
  assert_sighted_running_can_reenter_rng();
  puts("Hunger C caller checks passed.");
  return 0;
}
