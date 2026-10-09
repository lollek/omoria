#include <assert.h>
#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include "../src/constants.h"
#include "../src/floor.h"
#include "../src/main_loop/main_loop.h"
#include "../src/messages.h"
#include "../src/player.h"
#include "../src/random.h"
#include "../src/variables.h"
#include "support/headless_scenario.h"
#include "support/terminal.h"

struct command_script {
  const char *keys;
  size_t count;
  size_t consumed;
  size_t calls;
};

static bool next_command(void *context, char *key) {
  struct command_script *script = context;
  assert(game_state == GS_GET_COMMAND);
  script->calls++;
  if (script->consumed == script->count) {
    return false;
  }
  *key = script->keys[script->consumed++];
  return true;
}

static bool command_with_creature_marker(void *context, char *key) {
  player_flags.move_rate = -1234;
  return next_command(context, key);
}

static bool exhaust_while_searching(void *context, char *key) {
  search_on();
  return next_command(context, key);
}

static bool command_while_running(void *context, char *key) {
  find_flag = true;
  return next_command(context, key);
}

static void begin_scenario(void) {
  headless_terminal_reset();
  headless_terminal_allow_drawing(true);
  C_message_capture_begin();
  C_seeded_rng_begin(1234);
  memset(cave, 0, sizeof(cave));
  memset(t_list, 0, sizeof(t_list));
  memset(equipment, 0, sizeof(equipment));
  memset(&player_flags, 0, sizeof(player_flags));
  memset(&player_cur_age, 0, sizeof(player_cur_age));
  memset(used_line, 0, sizeof(used_line));
  cur_height = cur_width = 5;
  char_row = char_col = 3;
  panel_row_min = panel_col_min = 1;
  panel_row_max = panel_col_max = 5;
  panel_row_prt = panel_col_prt = 0;
  panel_row = panel_col = 0;
  for (long row = 1; row <= 5; row++) {
    for (long col = 1; col <= 5; col++) {
      cave[row][col].fval = ft_dark_open_floor;
      cave[row][col].fopen = true;
    }
  }
  cave[3][3].cptr = 1;
  player_flags.blind = 100;
  player_flags.status = IS_BLIND;
  player_flags.foodc = 10000;
  dun_level = 1;
  player_max_lev = 1;
  player_mana = 0;
  player_cmana = 0;
  muptr = 0;
  inventory_list = NULL;
  turn = 1;
  turn_counter = 100;
  wizard1 = true;
  death = false;
  moria_flag = find_flag = search_flag = teleport_flag = false;
  reset_flag = msg_flag = false;
  print_stat = command_count = 0;
  game_state = GS_IGNORE_CTRL_C;
}

static void end_scenario(void) {
  assert(muptr == 0 && !death && !moria_flag);
  assert(game_state == GS_IGNORE_CTRL_C);
  assert(headless_terminal_counts().input == 0);
  C_seeded_rng_end();
  C_message_capture_end();
  headless_terminal_reset();
  assert(headless_terminal_counts().drawing == 0);
}

static void assert_zero_budget(void) {
  begin_scenario();
  struct command_script script = {" ", 1, 0, 0};
  main_loop_with_commands(next_command, &script, 0);
  main_loop_with_commands(NULL, &script, 10);
  assert(turn == 1 && turn_counter == 100);
  assert(script.calls == 0);
  assert(headless_terminal_counts().drawing == 0);
  end_scenario();
}

static void assert_turn_limit_without_input(void) {
  begin_scenario();
  player_flags.paralysis = 20;
  struct command_script script = {" ", 1, 0, 0};
  main_loop_with_commands(next_command, &script, 2);
  assert(turn == 3 && turn_counter == 100);
  assert(script.calls == 0 && script.consumed == 0);
  assert(player_flags.blind == 98);
  assert(headless_terminal_counts().drawing > 0);
  end_scenario();
}

static void assert_turn_limit_with_commands(void) {
  begin_scenario();
  command_count = 7;
  msg_flag = true;
  struct command_script script = {"SSS", 3, 0, 0};
  main_loop_with_commands(command_with_creature_marker, &script, 2);
  assert(turn == 3 && turn_counter == 102);
  assert(script.calls == 2 && script.consumed == 2);
  assert(command_count == 0);
  assert(player_flags.move_rate != -1234);
  assert(player_flags.blind == 98);
  assert(C_message_capture_count() == 2);
  assert(headless_terminal_counts().erasure > 0);
  end_scenario();
}

static void assert_turn_limit_while_resting(void) {
  begin_scenario();
  player_flags.rest = 10;
  player_flags.status |= IS_RESTING;
  struct command_script script = {"S", 1, 0, 0};
  main_loop_with_commands(next_command, &script, 2);
  assert(turn == 3 && turn_counter == 100);
  assert(script.calls == 0 && player_flags.rest == 8);
  assert(player_flags.blind == 98);
  end_scenario();
}

static void assert_turn_limit_while_running(void) {
  begin_scenario();
  struct command_script script = {"SSS", 3, 0, 0};
  main_loop_with_commands(command_while_running, &script, 2);
  assert(turn == 3 && turn_counter == 102);
  assert(script.calls == 1 && script.consumed == 1);
  assert(find_flag && player_flags.blind == 98);
  assert(C_message_capture_count() == 2);
  end_scenario();
}

static void assert_turn_limit_cleans_up_search(void) {
  begin_scenario();
  struct command_script script = {"l", 1, 0, 0};
  main_loop_with_commands(exhaust_while_searching, &script, 1);
  assert(turn == 2 && turn_counter == 101);
  assert(script.calls == 1 && script.consumed == 1);
  assert(!search_flag && !(player_flags.status & IS_SEARCHING));
  assert(player_flags.speed == 0);
  end_scenario();
}

static void assert_exhaustion_cleans_up_search(void) {
  begin_scenario();
  struct command_script script = {NULL, 0, 0, 0};
  main_loop_with_commands(exhaust_while_searching, &script, 10);
  assert(script.calls == 1);
  assert(!search_flag && !(player_flags.status & IS_SEARCHING));
  assert(player_flags.speed == 0);
  end_scenario();
}

static void assert_exhaustion(size_t count) {
  begin_scenario();
  struct command_script script = {" ", count, 0, 0};
  main_loop_with_commands(command_with_creature_marker, &script, 10);
  assert(script.consumed == count && script.calls == count + 1);
  assert(player_flags.move_rate == -1234);
  assert(turn == 2);
  assert(turn_counter == 101 + (long)count);
  assert(player_flags.blind == 99);
  end_scenario();
}

static void assert_movement_scenarios(void) {
  struct headless_scenario_result moved;
  struct headless_scenario_result moved_again;
  headless_scenario_run("l", 1, false, &moved);
  assert(moved.start_row == 39 && moved.start_col == 140);
  assert(moved.row == 39 && moved.col == 141);
  assert(moved.origin_occupant == 0 && moved.destination_occupant == 1);
  assert(moved.command_calls == 1 && moved.commands_consumed == 1);
  assert(moved.turn == 2 && moved.turn_counter == 101);
  assert(moved.message_count == 0);
  assert(moved.drawing_count > 0 && moved.input_count == 0);

  struct headless_scenario_result blocked;
  struct headless_scenario_result blocked_again;
  headless_scenario_run("h", 1, true, &blocked);
  assert(blocked.start_row == 39 && blocked.start_col == 140);
  assert(blocked.row == 39 && blocked.col == 140);
  assert(blocked.origin_occupant == 1 && blocked.destination_occupant == 0);
  assert(blocked.command_calls == 2 && blocked.commands_consumed == 1);
  assert(blocked.turn == 2 && blocked.turn_counter == 102);
  assert(blocked.message_count == 0);
  assert(blocked.drawing_count > 0 && blocked.input_count == 0);

  headless_scenario_run("l", 1, false, &moved_again);
  headless_scenario_run("h", 1, true, &blocked_again);
  assert(memcmp(&moved, &moved_again, sizeof(moved)) == 0);
  assert(memcmp(&blocked, &blocked_again, sizeof(blocked)) == 0);
}

int main(void) {
  alarm(10);
  for (int repeat = 0; repeat < 2; repeat++) {
    C_seeded_rng_begin(77);
    const long expected = randint(1000000);
    C_seeded_rng_end();
    C_seeded_rng_begin(77);
    C_message_capture_begin();
    assert_zero_budget();
    assert_turn_limit_without_input();
    assert_turn_limit_with_commands();
    assert_turn_limit_while_resting();
    assert_turn_limit_while_running();
    assert_turn_limit_cleans_up_search();
    assert_exhaustion(0);
    assert_exhaustion(1);
    assert_exhaustion_cleans_up_search();
    assert(randint(1000000) == expected);
    assert(C_message_capture_count() == 0);
    msg_print("Outer capture restored.");
    assert(C_message_capture_count() == 1);
    C_message_capture_end();
    C_seeded_rng_end();
  }
  assert_movement_scenarios();
  alarm(0);
  puts("Bounded main-loop C checks passed.");
  return 0;
}