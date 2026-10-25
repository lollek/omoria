#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../src/debug.h"
#include "../src/combat/diagnostics.h"
#include "../src/combat/fighting.h"
#include "../src/io.h"
#include "../src/messages.h"
#include "../src/player.h"
#include "../src/random.h"
#include "../src/variables.h"
#include "support/terminal.h"

bool msg_print_pass_one(char *message);

void assert_headless_terminal(void);
void C_terminal_test_layout(int rows, int cols);
static void assert_message(size_t index, const char *expected);

static void assert_player_hit_roll_diagnostic(void) {
  char expected[120];
  show_combat_calculations = false;
  C_seeded_rng_begin(42);
  const long roll = randint(20);
  C_seeded_rng_end();

  show_combat_calculations = true;
  C_message_capture_begin();
  C_seeded_rng_begin(42);
  assert(player_test_hit(30, 20));
  C_seeded_rng_end();
  assert(C_message_capture_count() == 1);
  snprintf(expected, sizeof(expected),
           "Combat: You roll 30 + %ld = %ld vs AC 20: HIT.", roll,
           30 + roll);
  assert_message(0, expected);
  C_message_capture_end();

  show_combat_calculations = false;
  C_message_capture_begin();
  C_seeded_rng_begin(42);
  assert(player_test_hit(30, 20));
  C_seeded_rng_end();
  assert(C_message_capture_count() == 0);
  C_message_capture_end();
}

static void assert_player_natural_20_diagnostic(bool expected_hit) {
  unsigned long long seed = 1;
  long attack_roll = 0;
  for (; seed < 10000; seed++) {
    C_seeded_rng_begin(seed);
    attack_roll = randint(20);
    C_seeded_rng_end();
    if ((attack_roll == 20) == expected_hit) {
      break;
    }
  }
  assert(seed < 10000);

  C_seeded_rng_begin(seed);
  (void)randint(20);
  const long expected_next_roll = randint(100);
  C_seeded_rng_end();

  show_combat_calculations = true;
  C_message_capture_begin();
  C_seeded_rng_begin(seed);
  assert(player_test_hit(1, 1000) == expected_hit);
  const long actual_next_roll = randint(100);
  C_seeded_rng_end();
  assert(actual_next_roll == expected_next_roll);

  char expected[120];
  snprintf(expected, sizeof(expected),
           "Combat: You roll 1 + %ld = %ld vs AC 1000: %s.",
           attack_roll, 1 + attack_roll, expected_hit ? "HIT" : "MISS");
  assert(C_message_capture_count() == 1);
  assert_message(0, expected);
  C_message_capture_end();
  show_combat_calculations = false;
}

static void assert_combat_diagnostic_messages(void) {
  show_combat_calculations = true;
  C_message_capture_begin();
  combat_log_player_roll(30, 13, 20, true);
  combat_log_player_roll(10, 7, 30, true);
  combat_log_monster_roll(41, 69, 50, false);
  combat_log_monster_roll(1, 69, 50, true);
  combat_log_hp_change(false, 9, 20, 11);
  combat_log_hp_change(true, 4, 10, 6);
  combat_log_hp_change(false, 0, 20, 20);
  assert(C_message_capture_count() == 6);
  assert_message(0, "Combat: You roll 30 + 13 = 43 vs AC 20: HIT.");
  assert_message(1, "Combat: You roll 10 + 7 = 17 vs AC 30: HIT.");
  assert_message(2, "Combat: Monster roll 41/69 vs AC 50: MISS.");
  assert_message(3, "Combat: Monster roll 1/69 vs AC 50: HIT.");
  assert_message(4, "Combat: Player hits Monster for 9 damage (HP 20 -> 11).");
  assert_message(5, "Combat: Monster hits Player for 4 damage (HP 10 -> 6).");
  C_message_capture_end();

  show_combat_calculations = false;
  C_message_capture_begin();
  combat_log_player_roll(30, 13, 20, true);
  combat_log_monster_roll(41, 69, 50, false);
  combat_log_hp_change(false, 9, 20, 11);
  assert(C_message_capture_count() == 0);
  C_message_capture_end();
}

static void assert_detailed_monster_hit_ffi(void) {
  C_seeded_rng_begin(42);
  const managed_hit_result result = managed_to_hit_detailed(50, 10, 10, 1);
  C_seeded_rng_end();

  assert(result.base_to_hit == 50);
  assert(result.level == 10);
  assert(result.plus_to_hit == 10);
  assert(result.max_attack_value == 110);
  assert(result.attack_value >= 1 && result.attack_value <= 110);
  assert(result.enemy_ac == 1);
  assert(result.was_rolled && result.hit);
}

static bool log_dim[10];

static void observe_log_line(const char *text, int row, int col, bool dim) {
  assert(strlen(text) == 81 && col == 0 && row >= 25 && row < 35);
  log_dim[row - 25] = dim;
}

static bool unexpected_more(void) {
  assert(0 && "Unexpected -more- in command brightness scenario");
  return false;
}

static void assert_split_command_cycle_brightness(void) {
  C_terminal_test_layout(35, 150);
  C_message_log_command();
  turn = 1000;
  assert(!C_message_log_print("Player action", observe_log_line, unexpected_more));
  turn++;
  assert(!C_message_log_print("Monster action", observe_log_line, unexpected_more));
  turn++;
  assert(!C_message_log_print("Status update", observe_log_line, unexpected_more));
  assert(!log_dim[7] && !log_dim[8] && !log_dim[9]);

  C_message_log_input();
  turn++;
  assert(!C_message_log_print("More feedback", observe_log_line, unexpected_more));
  assert(!log_dim[6] && !log_dim[7] && !log_dim[8] && !log_dim[9]);

  C_message_log_command();
  assert(!C_message_log_print("Next action", observe_log_line, unexpected_more));
  assert(log_dim[5] && log_dim[6] && log_dim[7] && log_dim[8]);
  assert(!log_dim[9]);
  C_terminal_test_layout(25, 150);
}

static void assert_split_log_overflow_and_capture(void) {
  C_terminal_test_layout(35, 150);
  turn = 100;
  msg_flag = false;
  msg_terse = false;
  headless_terminal_reset();
  headless_terminal_allow_drawing(true);
  for (int i = 0; i < 10; i++) {
    char message[40];
    snprintf(message, sizeof(message), "Split message %d", i);
    assert(!msg_print(message));
  }
  assert(headless_terminal_counts().input == 0);
  assert(headless_terminal_counts().more == 0);
  const struct headless_attributed_draw line = headless_terminal_last_attributed_draw();
  assert(line.row == 35 && line.col == 1 && line.attrs == 0);
  assert(strlen(line.text) == 81 && strncmp(line.text, "Split message 9", 15) == 0);
  /* Empty output still enters raw history but must not page or add a log row. */
  assert(!msg_print(""));
  assert(!msg_print_pass_one(""));
  assert(!msg_print_pass_one(NULL));
  assert(headless_terminal_counts().more == 0);
  headless_terminal_script("\033", 1);
  assert(msg_print_pass_one("Overflow"));
  assert(headless_terminal_counts().input == 1 && headless_terminal_counts().more == 1);

  /* A normal command acknowledges pending lines without advancing the turn. */
  headless_terminal_script("a", 1);
  assert(headless_inkey() == 'a');
  assert(!msg_print("After command"));
  for (int i = 0; i < 20; i++)
    assert(!msg_print("After command"));
  assert(headless_terminal_counts().more == 1);
  assert(strstr(headless_terminal_last_attributed_draw().text, "(x21)") != NULL);

  /* A later turn starts its own unseen budget, even without command input. */
  turn++;
  for (int i = 0; i < 10; i++) {
    char message[40];
    snprintf(message, sizeof(message), "New turn %d", i);
    assert(!msg_print_pass_one(message));
  }
  headless_terminal_script("\033", 1);
  assert(msg_print("Primary overflow"));
  assert(headless_terminal_counts().more == 2);

  /* One oversized message pages twice, then retains its final line. */
  turn++;
  char long_message[81 * 21 + 1];
  memset(long_message, 'L', sizeof(long_message) - 1);
  long_message[sizeof(long_message) - 1] = '\0';
  headless_terminal_script("  ", 2);
  assert(!msg_print(long_message));
  assert(headless_terminal_counts().more == 4);
  assert(headless_terminal_last_attributed_draw().text[80] == 'L');
  assert(last_printed_message[81] == '\0');

  turn++;
  msg_terse = true;
  assert(!msg_print_pass_one(long_message));
  assert(headless_terminal_counts().more == 4);
  msg_terse = false;

  C_message_capture_begin();
  const struct headless_terminal_counts before = headless_terminal_counts();
  for (int i = 0; i < 12; i++) {
    msg_printf("Raw %s", "repeat");
    assert(!msg_print_pass_one("Raw repeat"));
  }
  assert(C_message_capture_count() == 24);
  for (int i = 0; i < 24; i++)
    assert_message(i, "Raw repeat");
  assert(headless_terminal_counts().drawing == before.drawing);
  assert(headless_terminal_counts().input == before.input);
  assert(headless_terminal_counts().erasure == before.erasure);
  C_message_capture_end();
  C_terminal_test_layout(25, 150);
  headless_terminal_reset();
}

static void assert_message(size_t index, const char *expected) {
  char buffer[256];
  assert(C_message_capture_get(index, buffer, sizeof(buffer)));
  assert(strcmp(buffer, expected) == 0);
}

static void assert_captured_messages_skip_terminal(void) {
  assert(!message_capture_active());
  msg_flag = false;
  msg_terse = false;
  last_printed_message[0] = '\0';
  C_message_capture_begin();
  assert(message_capture_active());
  assert(C_message_capture_count() == 0);
  assert(!msg_print("First"));
  assert(!msg_print("Second"));
  assert(C_message_capture_count() == 2);
  assert_message(0, "First");
  assert_message(1, "Second");
  assert(msg_flag && strcmp(last_printed_message, "Second") == 0);
  assert(!msg_print(""));
  assert_message(2, "");
  assert(msg_flag);
  assert(!msg_print_pass_one("Pass one"));
  assert_message(3, "Pass one");
  assert(!msg_print_pass_one(""));
  assert(!msg_flag && C_message_capture_count() == 4);
  assert(!msg_print_pass_one(NULL));
  assert(!msg_flag && C_message_capture_count() == 4);
  assert(!msg_print_pass_one(" "));
  assert_message(4, " ");
  assert(msg_flag);
  C_message_capture_end();
  assert(!message_capture_active());
  assert(C_message_capture_count() == 0);
}

static void assert_nested_captures_restore_outer(void) {
  C_message_capture_begin();
  assert(!msg_print("Outer"));
  C_message_capture_begin();
  assert(C_message_capture_count() == 0);
  assert(!msg_print("Inner"));
  assert_message(0, "Inner");
  C_message_capture_end();
  assert(message_capture_active());
  assert_message(0, "Outer");
  assert(!msg_print("Restored"));
  assert_message(1, "Restored");
  C_message_capture_end();
  assert(!message_capture_active());
}

static void assert_capture_copy_boundaries(void) {
  char buffer[8] = "Marker";
  assert(!C_message_capture_get(0, buffer, sizeof(buffer)));
  C_message_capture_end();
  C_message_capture_begin();
  assert(!C_message_capture_get(0, buffer, sizeof(buffer)));
  assert(!msg_print("Long message"));
  assert(!C_message_capture_get(1, buffer, sizeof(buffer)));
  assert(!C_message_capture_get(0, NULL, sizeof(buffer)));
  assert(!C_message_capture_get(0, buffer, 0));
  assert(strcmp(buffer, "Marker") == 0);
  assert(C_message_capture_get(0, buffer, 1));
  assert(buffer[0] == '\0' && buffer[1] == 'a');
  assert(C_message_capture_get(0, buffer, 5));
  assert(strcmp(buffer, "Long") == 0);
  C_message_capture_end();
}

static void assert_last_drop_restores_interactive_path(void) {
  assert(!message_capture_active());
  msg_flag = false;
  msg_terse = false;
  headless_terminal_reset();
  headless_terminal_allow_drawing(true);
  headless_terminal_script("\033\033", 2);
  assert(!msg_print("First interactive"));
  assert(msg_print("Second interactive"));
  assert(msg_print_pass_one("Pass one interactive"));
  const struct headless_terminal_counts counts = headless_terminal_counts();
  assert(counts.input == 2 && counts.drawing == 5 && counts.erasure == 3);
  assert(msg_flag && strcmp(last_printed_message, "Pass one interactive") == 0);
  headless_terminal_reset();
}

int main(void) {
  debug_file = fopen("target/debug/message-ffi.log", "w");
  assert(debug_file != NULL);
  assert_headless_terminal();
  assert_split_command_cycle_brightness();
  assert_split_log_overflow_and_capture();
  assert_player_hit_roll_diagnostic();
  assert_player_natural_20_diagnostic(true);
  assert_player_natural_20_diagnostic(false);
  assert_combat_diagnostic_messages();
  assert_detailed_monster_hit_ffi();
  for (int repeat = 0; repeat < 2; repeat++) {
    assert_captured_messages_skip_terminal();
    assert_nested_captures_restore_outer();
    assert_capture_copy_boundaries();
    assert_last_drop_restores_interactive_path();
  }
  puts("Message C boundary checks passed");
  fclose(debug_file);
  debug_file = NULL;
  return 0;
}