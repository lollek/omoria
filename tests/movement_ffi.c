#include <assert.h>
#include <stdio.h>
#include <string.h>

#include "../src/constants.h"
#include "../src/player.h"
#include "../src/player_action.h"
#include "../src/variables.h"

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
  }
  puts("Movement C caller checks passed.");
  return 0;
}