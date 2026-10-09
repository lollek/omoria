#include "headless_scenario.h"

#include <assert.h>
#include <string.h>

#include "../../src/constants.h"
#include "../../src/floor.h"
#include "../../src/main_loop/main_loop.h"
#include "../../src/messages.h"
#include "../../src/player.h"
#include "../../src/random.h"
#include "../../src/screen.h"
#include "../../src/variables.h"
#include "terminal.h"

bool C_save_test_reset(void);
bool C_save_test_apply_fixture_and_verify(void);

struct command_script {
  const char *commands;
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
  *key = script->commands[script->consumed++];
  return true;
}

static void begin_movement_scenario(bool block_west) {
  assert(C_save_test_reset());
  assert(C_save_test_apply_fixture_and_verify());
  assert(char_row == 39 && char_col == 140);

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

  cur_height = MAX_HEIGHT;
  cur_width = MAX_WIDTH;
  for (long row = char_row - 2; row <= char_row + 2; row++) {
    for (long col = char_col - 2; col <= char_col + 2; col++) {
      cave[row][col].fval = ft_dark_open_floor;
      cave[row][col].fopen = true;
    }
  }
  cave[char_row][char_col].cptr = 1;
  if (block_west) {
    cave[char_row][char_col - 1].fval = ft_wall_granite;
    cave[char_row][char_col - 1].fopen = false;
  }

  max_panel_rows = (MAX_HEIGHT - 1) / (SCREEN_HEIGHT / 2);
  max_panel_cols = (MAX_WIDTH - 1) / (SCREEN_WIDTH / 2);
  panel_row = (char_row - 2) / (SCREEN_HEIGHT / 2);
  panel_col = (char_col - 3) / (SCREEN_WIDTH / 2);
  panel_row_min = panel_row * (SCREEN_HEIGHT / 2) + 1;
  panel_row_max = panel_row_min + SCREEN_HEIGHT - 1;
  panel_col_min = panel_col * (SCREEN_WIDTH / 2) + 1;
  panel_col_max = panel_col_min + SCREEN_WIDTH - 1;
  panel_row_prt = panel_row_min - 2;
  panel_col_prt = panel_col_min - 15;

  player_flags.blind = 100;
  player_flags.status = IS_BLIND;
  player_flags.foodc = 10000;
  player_flags.light_on = false;
  equipment[Equipment_light].p1 = 0;
  player_light = false;
  dun_level = 1;
  player_max_lev = 1;
  player_mana = 0;
  player_cmana = 0;
  muptr = 0;
  turn = 1;
  turn_counter = 100;
  wizard1 = true;
  death = false;
  moria_flag = find_flag = search_flag = teleport_flag = false;
  reset_flag = msg_flag = false;
  print_stat = command_count = 0;
  game_state = GS_IGNORE_CTRL_C;
}

void headless_scenario_run(const char *commands, size_t command_count,
                           bool block_west, struct headless_scenario_result *result) {
  assert(commands != NULL && result != NULL);
  memset(result, 0, sizeof(*result));
  begin_movement_scenario(block_west);

  struct command_script script = {commands, command_count, 0, 0};
  main_loop_with_commands(next_command, &script, 1);

  result->start_row = 39;
  result->start_col = 140;
  result->row = char_row;
  result->col = char_col;
  result->turn = turn;
  result->turn_counter = turn_counter;
  const long destination_col = block_west ? result->start_col - 1 : result->start_col + 1;
  result->origin_occupant = cave[result->start_row][result->start_col].cptr;
  result->destination_occupant = cave[result->start_row][destination_col].cptr;
  result->command_calls = script.calls;
  result->commands_consumed = script.consumed;
  result->message_count = C_message_capture_count();

  const struct headless_terminal_counts terminal = headless_terminal_counts();
  result->drawing_count = terminal.drawing;
  result->input_count = terminal.input;
  assert(muptr == 0 && !death && !moria_flag);
  assert(game_state == GS_IGNORE_CTRL_C);
  assert(terminal.input == 0);
  C_seeded_rng_end();
  C_message_capture_end();
  headless_terminal_reset();
  assert(C_save_test_reset());
}