#include <assert.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "../src/constants.h"
#include "../src/floor.h"
#include "../src/main_loop/main_loop.h"
#include "../src/messages.h"
#include "../src/player.h"
#include "../src/random.h"
#include "../src/screen.h"
#include "../src/variables.h"
#include "support/terminal.h"

bool C_save_test_reset(void);
bool C_save_test_apply_fixture_and_verify(void);
bool C_save_test_memory_begin(void);
bool C_save_test_memory_save_character(void);
bool C_save_test_memory_load_character(void);
bool C_save_test_memory_end(void);

struct command_script {
  const char *keys;
  size_t count;
  size_t consumed;
  size_t calls;
};

struct inventory_snapshot {
  treasure_type data;
  bool ok;
  uint16_t insides;
  bool is_in;
};

struct scenario_result {
  long row;
  long col;
  int16_t hp;
  long inventory_count;
  struct inventory_snapshot *inventory;
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

static void assert_item_data_equal(const treasure_type *actual,
                                   const treasure_type *expected) {
  assert(memcmp(actual->name, expected->name, sizeof(actual->name)) == 0);
  assert(actual->tval == expected->tval);
  assert(actual->flags2 == expected->flags2);
  assert(actual->flags == expected->flags);
  assert(actual->p1 == expected->p1);
  assert(actual->cost == expected->cost);
  assert(actual->subval == expected->subval);
  assert(actual->weight == expected->weight);
  assert(actual->number == expected->number);
  assert(actual->tohit == expected->tohit);
  assert(actual->todam == expected->todam);
  assert(actual->ac == expected->ac);
  assert(actual->toac == expected->toac);
  assert(memcmp(actual->damage, expected->damage, sizeof(actual->damage)) == 0);
  assert(actual->level == expected->level);
  assert(actual->identified == expected->identified);
}

static void assert_scenario_results_equal(const struct scenario_result *actual,
                                          const struct scenario_result *expected) {
  assert(actual->row == expected->row);
  assert(actual->col == expected->col);
  assert(actual->hp == expected->hp);
  assert(actual->inventory_count == expected->inventory_count);
  for (size_t i = 0; i < (size_t)actual->inventory_count; i++) {
    assert_item_data_equal(&actual->inventory[i].data,
                           &expected->inventory[i].data);
    assert(actual->inventory[i].ok == expected->inventory[i].ok);
    assert(actual->inventory[i].insides == expected->inventory[i].insides);
    assert(actual->inventory[i].is_in == expected->inventory[i].is_in);
  }
}

static void begin_scenario(void) {
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

static void assert_move_save_reload(struct scenario_result *result) {
  begin_scenario();

  struct command_script script = {"l", 1, 0, 0};
  main_loop_with_commands(next_command, &script, 1);
  assert(char_row == 39 && char_col == 141);
  assert(script.calls == 1 && script.consumed == 1);
  assert(turn == 2 && turn_counter == 101);
  assert(muptr == 0 && !death && !moria_flag);

  C_player_modify_current_hp(10.0f);
  const long saved_row = char_row;
  const long saved_col = char_col;
  const int16_t saved_hp = C_player_current_hp();
  const long saved_inventory_count = inven_ctr;
  assert(saved_hp > 0);
  assert(inventory_list != NULL && saved_inventory_count > 0);
  struct inventory_snapshot *saved_inventory =
      calloc((size_t)saved_inventory_count, sizeof(*saved_inventory));
  assert(saved_inventory != NULL);
  const treas_rec *item = inventory_list;
  for (size_t i = 0; i < (size_t)saved_inventory_count; i++) {
    assert(item != NULL);
    saved_inventory[i].data = item->data;
    saved_inventory[i].ok = item->ok;
    saved_inventory[i].insides = item->insides;
    saved_inventory[i].is_in = item->is_in;
    item = item->next;
  }
  assert(item == NULL);

  assert(C_save_test_memory_begin());
  assert(C_save_test_memory_save_character());

  char_row = 39;
  char_col = 140;
  C_player_modify_current_hp(1.0f);
  inventory_list->data.number++;
  assert(C_player_current_hp() != saved_hp);
  assert(inventory_list->data.number != saved_inventory[0].data.number);

  assert(C_save_test_memory_load_character());
  assert(char_row == saved_row && char_col == saved_col);
  assert(C_player_current_hp() == saved_hp);
  assert(inven_ctr == saved_inventory_count && inventory_list != NULL);
  item = inventory_list;
  for (size_t i = 0; i < (size_t)saved_inventory_count; i++) {
    assert(item != NULL);
    assert_item_data_equal(&item->data, &saved_inventory[i].data);
    assert(item->ok == saved_inventory[i].ok);
    assert(item->insides == saved_inventory[i].insides);
    assert(item->is_in == saved_inventory[i].is_in);
    item = item->next;
  }
  assert(item == NULL);
  result->row = saved_row;
  result->col = saved_col;
  result->hp = saved_hp;
  result->inventory_count = saved_inventory_count;
  result->inventory = saved_inventory;
  assert(headless_terminal_counts().input == 0);
  assert(C_save_test_memory_end());

  C_seeded_rng_end();
  C_message_capture_end();
  headless_terminal_reset();
  assert(C_save_test_reset());
}

int main(void) {
  alarm(10);
  struct scenario_result first;
  struct scenario_result second;
  assert_move_save_reload(&first);
  assert_move_save_reload(&second);
  assert_scenario_results_equal(&second, &first);
  free(first.inventory);
  free(second.inventory);
  alarm(0);
  puts("Headless save/reload C checks passed.");
  return 0;
}
