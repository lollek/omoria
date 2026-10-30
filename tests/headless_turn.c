#include <assert.h>
#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include "../src/constants.h"
#include "../src/floor.h"
#include "../src/map_colors.h"
#include "../src/main_loop/main_loop.h"
#include "../src/messages.h"
#include "../src/model_item.h"
#include "../src/player.h"
#include "../src/random.h"
#include "../src/term.h"
#include "../src/variables.h"
#include "support/headless_scenario.h"
#include "support/terminal.h"

struct command_script {
  const char *keys;
  size_t count;
  size_t consumed;
  size_t calls;
};

static size_t recorded_message_views;
static size_t travel_delay_calls;

chtype get_loc_symbol(const long y, const long x);
chtype loc_symbol(const long y, const long x);

void headless_travel_delay(void) { travel_delay_calls++; }

static bool captured_message_contains(const char *expected) {
  char message[128];
  for (size_t index = 0; index < C_message_capture_count(); index++) {
    if (C_message_capture_get(index, message, sizeof(message)) &&
        strcmp(message, expected) == 0) {
      return true;
    }
  }
  return false;
}

void headless_show_recorded_messages(void) { recorded_message_views++; }

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
  C_mouse_travel_cancel();
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
  recorded_message_views = 0;
  travel_delay_calls = 0;
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

static void assert_terrain_symbols_keep_glyphs_and_attributes(void) {
  FILE *input = tmpfile();
  FILE *output = tmpfile();
  assert(input != NULL && output != NULL);
  SCREEN *screen = newterm("xterm-256color", output, input);
  assert(screen != NULL);
  set_term(screen);
  assert(start_color() == OK);
  C_map_color_init(COLORS);

  begin_scenario();
  player_flags.blind = 0;
  player_flags.status &= ~IS_BLIND;

  const uint8_t terrain[] = {
      ft_wall_granite, ft_wall_magma, ft_wall_quartz,
      ft_dark_open_floor, ft_corr_open_floor, ft_boundry_wall,
  };
  const char glyphs[] = {'#', '#', '#', '.', '.', '#'};
    const chtype pairs[] = {
      COLOR_PAIR(17), COLOR_PAIR(18), COLOR_PAIR(19),
      COLOR_PAIR(21), COLOR_PAIR(22), COLOR_PAIR(20),
    };
  for (size_t index = 0; index < sizeof(terrain) / sizeof(terrain[0]); index++) {
    const long row = 1;
    const long col = (long)index + 1;
    cave[row][col].fval = terrain[index];
    cave[row][col].is_permanently_lit = true;

    const chtype symbol = get_loc_symbol(row, col);
    assert((symbol & 0x7F) == glyphs[index]);
    assert((symbol & A_COLOR) == (pairs[index] & A_COLOR));
    assert((symbol & A_COLOR) ==
           (C_map_color_terrain_attr(terrain[index]) & A_COLOR));
  }

  const uint8_t item_types[] = {
      closed_door, down_staircase, potion1, potion1,
      valuable_gems_wear, rubble, seen_trap, entrance_to_store,
  };
  const int64_t item_subvals[] = {0, 0, 272, 258, 1, 0, 2, 101};
  const uint64_t item_flags[] = {
      0, 0, 0, 0, Teleportation_worn_bit, 0, 0, 0,
  };
  const char item_glyphs[] = {'+', '>', '!', '!', '*', ':', '^', 'G'};
  const chtype item_pairs[] = {
      COLOR_PAIR(23), COLOR_PAIR(24), COLOR_PAIR(48), COLOR_PAIR(49),
      COLOR_PAIR(49), COLOR_PAIR(25), COLOR_PAIR(26), COLOR_PAIR(27),
  };
  int healing_color_key = -1;
  int harmful_color_key = -1;
  for (size_t index = 0; index < sizeof(item_types) / sizeof(item_types[0]); index++) {
    const long row = 2;
    const long col = (long)index + 1;
    cave[row][col].tptr = 1;
    cave[row][col].is_permanently_lit = true;
    t_list[1].tval = item_types[index];
    t_list[1].subval = item_subvals[index];
    t_list[1].flags = item_flags[index];

    const chtype plain_glyph = C_item_get_tchar(&t_list[1]);
    const chtype glyph = get_loc_symbol(row, col);
    assert(glyph == C_item_get_glyph(&t_list[1]));
    assert((plain_glyph & 0x7F) == item_glyphs[index]);
    assert((glyph & 0x7F) == item_glyphs[index]);
    assert((glyph & A_COLOR) == (item_pairs[index] & A_COLOR));
    assert((glyph & A_BOLD) == (index == 7 ? A_BOLD : 0));
    if (index == 2) {
      healing_color_key = C_item_color_key(&t_list[1]);
    } else if (index == 3) {
      assert(C_item_color_key(&t_list[1]) != healing_color_key);
      harmful_color_key = C_item_color_key(&t_list[1]);
    } else if (index == 4) {
      assert(C_item_color_key(&t_list[1]) == harmful_color_key);
    }
  }

  cave[2][4].tptr = 1;
  cave[2][4].is_permanently_lit = true;
  t_list[1].tval = unseen_trap;
  t_list[1].subval = 0;
  assert((get_loc_symbol(2, 4) & 0x7F) == '.');
  assert((get_loc_symbol(2, 4) & A_COLOR) == 0);

  cave[2][5].tptr = 1;
  cave[2][5].is_permanently_lit = true;
  t_list[1].tval = secret_door;
  t_list[1].subval = 0;
  assert((get_loc_symbol(2, 5) & 0x7F) == '#');
  assert((get_loc_symbol(2, 5) & A_COLOR) == 0);

  cave[1][1].cptr = 2;
  memset(&m_list[2], 0, sizeof(m_list[2]));
  assert((loc_symbol(1, 1) & 0x7F) == '#');
  assert((loc_symbol(1, 1) & A_COLOR) == (COLOR_PAIR(17) & A_COLOR));
  assert(get_loc_symbol(char_row, char_col) == '@');
  end_scenario();

  endwin();
  delscreen(screen);
  fclose(input);
  fclose(output);
  C_map_color_init(0);
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
  assert(headless_terminal_counts().screen_clears == 0);
  assert(headless_terminal_counts().erasure >= 22);
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

static void assert_message_history_does_not_consume_turn(void) {
  begin_scenario();
  const char keys[] = {CTRL_P, '.', '\0'};
  struct command_script script = {keys, 2, 0, 0};
  main_loop_with_commands(command_with_creature_marker, &script, 1);
  assert(recorded_message_views == 1);
  assert(script.calls == 2 && script.consumed == 2);
  assert(turn == 2 && turn_counter == 102);
  assert(player_flags.move_rate != -1234);
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

static void assert_mouse_travel_stops_on_seen_monster(void) {
  begin_scenario();
  player_flags.blind = 0;
  player_flags.status = 0;
  player_light = false;
  cur_height = cur_width = 7;
  for (long row = 1; row <= 7; row++) {
    for (long col = 1; col <= 7; col++) {
      cave[row][col].fval = ft_dark_open_floor;
      cave[row][col].fopen = true;
      cave[row][col].is_permanently_lit = true;
    }
  }
  cave[2][5].cptr = 2;
  m_list[2].is_seen = true;

  const char travel_key = OMORIA_KEY_TRAVEL;
  struct command_script script = {&travel_key, 1, 0, 0};
  assert(C_mouse_travel_begin(3, 6));
  main_loop_with_commands(next_command, &script, 4);

  assert(script.consumed == 1 && script.calls == 2);
  assert(char_row == 2 && char_col == 4);
  assert(!C_mouse_travel_is_active() && !find_flag);
  end_scenario();
}

static void assert_mouse_travel_reaches_clicked_room_corner(void) {
  begin_scenario();
  cur_height = cur_width = 20;
  panel_row_min = panel_col_min = 1;
  panel_row_max = panel_col_max = 20;
  char_row = char_col = 5;
  player_flags.blind = 0;
  player_flags.status = 0;

  for (long row = 1; row <= 20; row++) {
    for (long col = 1; col <= 20; col++) {
      cave[row][col].fval = ft_wall_granite;
      cave[row][col].fopen = false;
    }
  }
  for (long row = 2; row <= 8; row++) {
    for (long col = 2; col <= 5; col++) {
      cave[row][col].fval = ft_dark_open_floor;
      cave[row][col].fopen = true;
      cave[row][col].is_permanently_lit = true;
    }
    for (long col = 15; col <= 18; col++) {
      cave[row][col].fval = ft_dark_open_floor;
      cave[row][col].fopen = true;
      cave[row][col].is_permanently_lit = true;
    }
  }
  for (long col = 6; col <= 14; col++) {
    cave[5][col].fval = col == 6 || col == 14
                            ? ft_corr_room_junction
                            : ft_corr_open_floor;
    cave[5][col].fopen = true;
    cave[5][col].is_permanently_lit = true;
  }
  cave[5][5].cptr = 1;

  const char travel_key = OMORIA_KEY_TRAVEL;
  struct command_script script = {&travel_key, 1, 0, 0};
  assert(C_mouse_travel_begin(5, 15));
  find_flag = true;
  assert(get_loc_symbol(5, 5) == '@');
  main_loop_with_commands(next_command, &script, 20);

  assert(char_row == 5 && char_col == 15);
  assert(!C_mouse_travel_is_active() && !find_flag);
  assert(cave[5][5].cptr == 0 && cave[5][15].cptr == 1);
  assert(travel_delay_calls == 10);
  assert(captured_message_contains("You have arrived at your destination."));
  assert(get_loc_symbol(5, 15) == '@');
  end_scenario();
}

static void assert_mouse_travel_crosses_walked_dark_cells(void) {
  begin_scenario();
  cur_height = cur_width = 7;
  const char manual_keys[] = {'l', 'l'};
  struct command_script manual = {manual_keys, sizeof(manual_keys), 0, 0};
  main_loop_with_commands(next_command, &manual, 2);

  assert(char_row == 3 && char_col == 5);
  assert(cave[3][3].moved && cave[3][4].moved && cave[3][5].moved);

  const char travel_key = OMORIA_KEY_TRAVEL;
  struct command_script travel = {&travel_key, 1, 0, 0};
  assert(C_mouse_travel_begin(3, 3));
  main_loop_with_commands(next_command, &travel, 5);

  assert(char_row == 3 && char_col == 3);
  assert(!C_mouse_travel_is_active() && !find_flag);
  end_scenario();
}

static void assert_mouse_travel_key_interrupts_without_an_extra_step(void) {
  begin_scenario();
  cur_height = cur_width = 7;
  for (long row = 1; row <= 7; row++) {
    for (long col = 1; col <= 7; col++) {
      cave[row][col].fval = ft_dark_open_floor;
      cave[row][col].fopen = true;
      cave[row][col].is_permanently_lit = true;
    }
  }

  const char keys[] = {OMORIA_KEY_TRAVEL, 'l'};
  const bool pending[] = {false, true};
  struct command_script script = {keys, sizeof(keys), 0, 0};
  assert(C_mouse_travel_begin(3, 6));
  headless_terminal_poll_script(pending, sizeof(pending) / sizeof(pending[0]));
  main_loop_with_commands(next_command, &script, 2);

  assert(script.consumed == 2 && script.calls == 2);
  assert(char_row == 2 && char_col == 5);
  assert(turn == 3 && turn_counter == 103);
  assert(!C_mouse_travel_is_active() && !find_flag);
  assert(captured_message_contains("Travel interrupted."));
  end_scenario();
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
    assert_message_history_does_not_consume_turn();
    assert_turn_limit_while_resting();
    assert_turn_limit_while_running();
    assert_turn_limit_cleans_up_search();
    assert_exhaustion(0);
    assert_exhaustion(1);
    assert_exhaustion_cleans_up_search();
    assert_mouse_travel_crosses_walked_dark_cells();
    assert_mouse_travel_reaches_clicked_room_corner();
    assert(randint(1000000) == expected);
    assert(C_message_capture_count() == 0);
    msg_print("Outer capture restored.");
    assert(C_message_capture_count() == 1);
    C_message_capture_end();
    C_seeded_rng_end();
  }
  assert_terrain_symbols_keep_glyphs_and_attributes();
  assert_movement_scenarios();
  assert_mouse_travel_stops_on_seen_monster();
  assert_mouse_travel_key_interrupts_without_an_extra_step();
  alarm(0);
  puts("Bounded main-loop C checks passed.");
  return 0;
}