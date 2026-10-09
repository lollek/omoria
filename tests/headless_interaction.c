#include <assert.h>
#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include "../src/constants.h"
#include "../src/floor.h"
#include "../src/inventory/inven.h"
#include "../src/main_loop/main_loop.h"
#include "../src/messages.h"
#include "../src/misc.h"
#include "../src/player.h"
#include "../src/random.h"
#include "../src/screen.h"
#include "../src/variables.h"
#include "support/terminal.h"

bool C_save_test_reset(void);
bool C_save_test_apply_fixture_and_verify(void);

#define RATION_SUBVAL 307
#define RATION_WEIGHT 10
#define PICKUP_MESSAGE "You have ration of food. (1a)"

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

static void clear_inventory(void) {
  while (inventory_list != NULL) {
    delete_inven_item(inventory_list);
  }
  inven_weight = 0;
  assert(inven_ctr == 0);
}

static long ration_slot = 0;

static void place_ration_east_of_player(void) {
  long slot = 0;
  popt(&slot);
  assert(slot > 0 && slot <= MAX_TALLOC);
  ration_slot = slot;

  treasure_type ration;
  memset(&ration, 0, sizeof(ration));
  strcpy(ration.name, "& Ration~ of Food");
  ration.tval = Food;
  ration.subval = RATION_SUBVAL;
  ration.number = 1;
  ration.weight = RATION_WEIGHT;
  ration.cost = 3;
  ration.level = 6;
  ration.identified = true;
  t_list[slot] = ration;

  cave[char_row][char_col + 1].tptr = slot;
}

static void begin_pickup_scenario(void) {
  assert(C_save_test_reset());
  assert(C_save_test_apply_fixture_and_verify());
  assert(char_row == 39 && char_col == 140);

  headless_terminal_reset();
  headless_terminal_allow_drawing(true);
  C_message_capture_begin();
  C_seeded_rng_begin(1234);

  clear_inventory();
  memset(cave, 0, sizeof(cave));
  memset(equipment, 0, sizeof(equipment));
  memset(&player_flags, 0, sizeof(player_flags));
  memset(&player_cur_age, 0, sizeof(player_cur_age));
  memset(used_line, 0, sizeof(used_line));
  tlink();

  cur_height = MAX_HEIGHT;
  cur_width = MAX_WIDTH;
  for (long row = char_row - 2; row <= char_row + 2; row++) {
    for (long col = char_col - 2; col <= char_col + 2; col++) {
      cave[row][col].fval = ft_dark_open_floor;
      cave[row][col].fopen = true;
    }
  }
  cave[char_row][char_col].cptr = 1;
  place_ration_east_of_player();

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

static void end_pickup_scenario(void) {
  assert(muptr == 0 && !death && !moria_flag);
  assert(game_state == GS_IGNORE_CTRL_C);
  assert(headless_terminal_counts().input == 0);
  clear_inventory();
  C_seeded_rng_end();
  C_message_capture_end();
  headless_terminal_reset();
  assert(C_save_test_reset());
}

static void assert_ration_pickup_scenario(void) {
  begin_pickup_scenario();
  struct command_script script = {"l", 1, 0, 0};
  main_loop_with_commands(next_command, &script, 1);

  assert(char_row == 39 && char_col == 141);
  assert(cave[39][140].cptr == 0 && cave[39][141].cptr == 1);
  assert(cave[39][141].tptr == 0);

  long freed = 0;
  popt(&freed);
  assert(freed == ration_slot);
  pusht(freed);

  assert(turn == 2 && turn_counter == 101);
  assert(!reset_flag);
  assert(script.calls == 1 && script.consumed == 1);

  assert(inven_ctr == 1);
  assert(inven_weight == RATION_WEIGHT);
  assert(inventory_list != NULL && inventory_list->next == NULL);
  assert(inventory_list->data.tval == Food);
  assert(inventory_list->data.subval == RATION_SUBVAL);
  assert(inventory_list->data.number == 1);

  assert(C_message_capture_count() == 1);
  char message[120];
  assert(C_message_capture_get(0, message, sizeof(message)));
  assert(strcmp(message, PICKUP_MESSAGE) == 0);

  const struct headless_terminal_counts terminal = headless_terminal_counts();
  assert(terminal.drawing > 0 && terminal.input == 0);

  end_pickup_scenario();
}

int main(void) {
  alarm(10);
  for (int repeat = 0; repeat < 2; repeat++) {
    assert_ration_pickup_scenario();
  }
  alarm(0);
  puts("Headless interaction checks passed.");
  return 0;
}
