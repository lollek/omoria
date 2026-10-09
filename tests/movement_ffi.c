#include <assert.h>
#include <stdio.h>
#include <string.h>

#include "../src/constants.h"
#include "../src/floor.h"
#include "../src/player.h"
#include "../src/player_action.h"
#include "../src/screen.h"
#include "../src/variables.h"
#include "support/terminal.h"

static void reset_movement_state(void) {
  memset(cave, 0, sizeof(cave));
  memset(t_list, 0, sizeof(t_list));
  memset(&player_flags, 0, sizeof(player_flags));
  cur_height = 3;
  cur_width = 3;
  char_row = 2;
  char_col = 2;
  cave[2][2].cptr = 1;
  cave[2][2].fopen = true;
  reset_flag = false;
  find_flag = false;
  turn_counter = 100;
}

static void assert_blocked_wall_keeps_position(void) {
  reset_movement_state();
  player_action_move(6);
  assert(char_row == 2 && char_col == 2);
  assert(cave[2][2].cptr == 1 && cave[2][3].cptr == 0);
  assert(reset_flag);
  assert(turn_counter == 100);
}

static void assert_edge_move_stops_find_without_consuming_turn(void) {
  const long edges[][3] = {{1, 2, 8}, {3, 2, 2}, {2, 1, 4}, {2, 3, 6}};
  for (size_t edge = 0; edge < sizeof(edges) / sizeof(edges[0]); edge++) {
    for (int running = 0; running < 2; running++) {
      reset_movement_state();
      char_row = edges[edge][0];
      char_col = edges[edge][1];
      cave[2][2].cptr = 0;
      cave[char_row][char_col].cptr = 1;
      find_flag = running;
      player_action_move(edges[edge][2]);
      assert(char_row == edges[edge][0] && char_col == edges[edge][1]);
      assert(cave[char_row][char_col].cptr == 1);
      assert(reset_flag && !find_flag);
      assert(turn_counter == 100);
    }
  }
}

/* Blindness skips the search RNG roll and room lighting; an empty target skips
 * carry. Running/search are disabled and the panel is already current, avoiding
 * area checks and map redraw. Real blind lighting still draws twice via HT1.
 * This standalone process serializes the globals; assertion failure ends it. */
static void assert_open_floor_move_updates_position_and_occupancy(void) {
  const long saved_panels[] = {
      panel_row, panel_col, max_panel_rows, max_panel_cols,
      panel_row_min, panel_row_max, panel_col_min, panel_col_max,
      panel_row_prt, panel_col_prt};
  const bool saved_cave_flag = cave_flag;
  const bool saved_search_flag = search_flag;
  bool saved_lines[sizeof(used_line) / sizeof(used_line[0])];
  memcpy(saved_lines, used_line, sizeof(used_line));

  reset_movement_state();
  headless_terminal_reset();
  headless_terminal_allow_drawing(true);
  player_flags.blind = 1;
  search_flag = false;
  cave[2][3].fopen = true;
  cave[2][3].fval = ft_dark_open_floor;
  panel_row = panel_col = 0;
  max_panel_rows = max_panel_cols = 0;
  panel_row_min = panel_col_min = 1;
  panel_row_max = SCREEN_HEIGHT;
  panel_col_max = SCREEN_WIDTH;
  panel_row_prt = -1;
  panel_col_prt = -14;
  cave_flag = true;
  memset(used_line, 0, sizeof(used_line));

  player_action_move(6);

  assert(char_row == 2 && char_col == 3);
  assert(cave[2][2].cptr == 0 && cave[2][3].cptr == 1);
  assert(!reset_flag && !find_flag);
  assert(turn_counter == 100);
  const struct headless_terminal_counts counts = headless_terminal_counts();
  assert(counts.drawing == 2 && counts.erasure == 0 && counts.input == 0);
  assert(used_line[3]);
  headless_terminal_reset();
  reset_movement_state();
  panel_row = saved_panels[0];
  panel_col = saved_panels[1];
  max_panel_rows = saved_panels[2];
  max_panel_cols = saved_panels[3];
  panel_row_min = saved_panels[4];
  panel_row_max = saved_panels[5];
  panel_col_min = saved_panels[6];
  panel_col_max = saved_panels[7];
  panel_row_prt = saved_panels[8];
  panel_col_prt = saved_panels[9];
  cave_flag = saved_cave_flag;
  search_flag = saved_search_flag;
  memcpy(used_line, saved_lines, sizeof(used_line));
}

static void assert_abi_results_match_cave_records(void) {
  reset_movement_state();
  cave[2][3].fopen = true;
  struct player_move_result step = C_player_move_resolve(6, 2, 2);
  assert(step.kind == PLAYER_MOVE_OPEN && step.consumes_turn);
  assert(step.row == 2 && step.col == 3);
  cave[2][3].fopen = false;
  cave[2][3].tptr = 1;
  t_list[1].tval = closed_door;
  step = C_player_move_resolve(6, 2, 2);
  assert(step.kind == PLAYER_MOVE_BLOCKED && !step.consumes_turn);
  assert(step.obstacle == closed_door);
  t_list[1].tval = rubble;
  step = C_player_move_resolve(6, 2, 2);
  assert(step.obstacle == rubble);
  cave[2][3].cptr = 2;
  step = C_player_move_resolve(6, 2, 2);
  assert(step.kind == PLAYER_MOVE_ATTACK && step.consumes_turn);
  const struct player_move_direction direction = C_player_move_direction(6, 0);
  assert(direction.dir == 6 && !direction.scrambled);
}

int main(void) {
  for (int repeat = 0; repeat < 2; repeat++) {
    assert_blocked_wall_keeps_position();
    assert_edge_move_stops_find_without_consuming_turn();
    assert_abi_results_match_cave_records();
    assert_open_floor_move_updates_position_and_occupancy();
  }
  puts("Movement C caller checks passed.");
  return 0;
}