#include <assert.h>
#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include "../src/constants.h"
#include "../src/debug.h"
#include "../src/floor.h"
#include "../src/inventory/inven.h"
#include "../src/main_loop/command.h"
#include "../src/main_loop/main_loop.h"
#include "../src/messages.h"
#include "../src/misc.h"
#include "../src/monsters.h"
#include "../src/player.h"
#include "../src/player_action.h"
#include "../src/random.h"
#include "../src/screen.h"
#include "../src/variables.h"
#include "support/terminal.h"

void C_save_test_apply_blank(void);
bool C_save_test_apply_fixture(void);

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
  C_save_test_apply_blank();
  assert(C_save_test_apply_fixture());
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
  C_save_test_apply_blank();
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

static void assert_same_item(const treasure_type *actual,
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

static void assert_close_target_scenario(uint8_t tval, long broken,
                                        uint8_t monster, bool empty,
                                        const char *expected_message) {
  begin_pickup_scenario();
  const long row = char_row;
  const long col = char_col + 1;
  const long slot = ration_slot;
  t_list[slot] = door_list[0];
  strcpy(t_list[slot].name, "old door");
  t_list[slot].tval = tval;
  t_list[slot].p1 = broken;
  t_list[slot].flags2 = 7;
  t_list[slot].flags = 9;
  t_list[slot].cost = 11;
  t_list[slot].subval = 13;
  t_list[slot].weight = 15;
  t_list[slot].number = 17;
  t_list[slot].tohit = -2;
  t_list[slot].todam = -3;
  t_list[slot].ac = 4;
  t_list[slot].toac = 5;
  strcpy(t_list[slot].damage, "2d3");
  t_list[slot].level = 6;
  t_list[slot].identified = true;
  cave[row][col].cptr = monster;
  cave[row][col].fm = true;
  cave[row][col].tptr = empty ? 0 : slot;
  memset(&m_list[2], 0, sizeof(m_list[2]));

  cave_type expected_cell = cave[row][col];
  const treasure_type original_item = t_list[slot];
  const size_t drawing_before = headless_terminal_counts().drawing;
  C_player_action_close_target(row, col);

  if (expected_message == NULL) {
    expected_cell.fopen = false;
    assert_same_item(&t_list[slot], &door_list[1]);
    assert(C_message_capture_count() == 0);
    assert(headless_terminal_counts().drawing > drawing_before);
  } else {
    assert_same_item(&t_list[slot], &original_item);
    assert(C_message_capture_count() == 1);
    char message[120];
    assert(C_message_capture_get(0, message, sizeof(message)));
    assert(strcmp(message, expected_message) == 0);
    assert(headless_terminal_counts().drawing == drawing_before);
  }
  assert(memcmp(&cave[row][col], &expected_cell, sizeof(expected_cell)) == 0);
  assert(turn == 1 && turn_counter == 100 && !reset_flag);
  end_pickup_scenario();
}

static void assert_refill_message(size_t index, const char *expected) {
  char message[120];
  assert(C_message_capture_get(index, message, sizeof(message)));
  assert(strcmp(message, expected) == 0);
}

static treas_rec *add_oil_flasks(uint16_t number) {
  treasure_type oil;
  memset(&oil, 0, sizeof(oil));
  strcpy(oil.name, "& Flask~ of Oil");
  oil.tval = flask_of_oil;
  oil.subval = 257;
  oil.p1 = 7500;
  oil.number = number;
  oil.weight = 10;
  treas_rec *flasks = add_inven_item(oil);
  assert(flasks != NULL);
  return flasks;
}

static void assert_refill_scenario(long subval, long fuel, long expected_fuel,
                                   uint16_t number) {
  begin_pickup_scenario();
  equipment[Equipment_light].subval = subval;
  equipment[Equipment_light].p1 = fuel;
  treasure_type expected_lamp = equipment[Equipment_light];
  expected_lamp.p1 = expected_fuel;
  treas_rec *flasks = add_oil_flasks(number);
  const size_t drawing_before = headless_terminal_counts().drawing;

  player_action_refill_lamp();

  assert_same_item(&equipment[Equipment_light], &expected_lamp);
  assert(inven_weight == (number - 1) * 10);
  if (number > 1) {
    assert(inven_ctr == 1 && inventory_list == flasks);
    assert(flasks->data.number == number - 1 && flasks->next == NULL);
  } else {
    assert(inven_ctr == 0 && inventory_list == NULL);
  }
  assert(C_message_capture_count() == 2);
  assert_refill_message(0, "Your lamp is full.");
  assert_refill_message(1, number > 1 ? "You have flask of oil."
                                     : "You have no more flasks of oil.");
  assert(headless_terminal_counts().drawing > drawing_before);
  assert(turn == 1 && turn_counter == 100 && !reset_flag);
  end_pickup_scenario();
}

static void assert_refill_refused(long subval, bool has_oil, bool contained,
                                const char *expected_message) {
  begin_pickup_scenario();
  equipment[Equipment_light].subval = subval;
  equipment[Equipment_light].p1 = 123;
  const treasure_type original_lamp = equipment[Equipment_light];
  treas_rec *flasks = has_oil ? add_oil_flasks(2) : NULL;
  if (flasks != NULL) {
    flasks->is_in = contained;
  }
  const size_t drawing_before = headless_terminal_counts().drawing;

  player_action_refill_lamp();

  assert_same_item(&equipment[Equipment_light], &original_lamp);
  assert(inventory_list == flasks);
  assert(inven_ctr == (has_oil ? 1 : 0));
  assert(inven_weight == (has_oil ? 20 : 0));
  if (flasks != NULL) {
    assert(flasks->data.number == 2 && flasks->is_in == contained);
  }
  assert(C_message_capture_count() == 1);
  assert_refill_message(0, expected_message);
  assert(headless_terminal_counts().drawing == drawing_before);
  assert(turn == 1 && turn_counter == 100 && !reset_flag);
  end_pickup_scenario();
}

static void assert_toggle_light_scenario(uint8_t tval, long fuel, bool on,
                                         const char *expected_message) {
  begin_pickup_scenario();
  equipment[Equipment_light].tval = tval;
  equipment[Equipment_light].p1 = fuel;
  const treasure_type original_light = equipment[Equipment_light];
  player_flags.light_on = on;
  player_light = !on;
  player_flags.blind = 0;
  player_flags.status = 0;
  for (long row = char_row - 1; row <= char_row + 1; row++) {
    for (long col = char_col - 1; col <= char_col + 1; col++) {
      cave[row][col].is_temporarily_lit = on;
    }
  }
  const size_t drawing_before = headless_terminal_counts().drawing;

  player_action_toggle_light_source();

  const bool toggled = tval > 0 && fuel > 0;
  assert(reset_flag);
  assert(player_flags.light_on == (toggled ? !on : on));
  assert(player_light == !on);
  assert_same_item(&equipment[Equipment_light], &original_light);
  assert(char_row == 39 && char_col == 140);
  assert(turn == 1 && turn_counter == 100);
  for (long row = char_row - 1; row <= char_row + 1; row++) {
    for (long col = char_col - 1; col <= char_col + 1; col++) {
      assert(cave[row][col].is_temporarily_lit == (toggled ? !on : on));
    }
  }
  assert(C_message_capture_count() == 1);
  char message[120];
  assert(C_message_capture_get(0, message, sizeof(message)));
  assert(strcmp(message, expected_message) == 0);
  if (toggled) {
    assert(headless_terminal_counts().drawing > drawing_before);
  } else {
    assert(headless_terminal_counts().drawing == drawing_before);
  }
  end_pickup_scenario();
}

static void assert_toggle_combat_calculations_scenario(void) {
  begin_pickup_scenario();
  show_combat_calculations = false;

  long command_key = 27;
  headless_terminal_script("v", 1);
  command(&command_key);

  assert(command_key == 'v');
  assert(show_combat_calculations);
  assert(reset_flag);
  assert(turn == 1 && turn_counter == 100);
  assert(headless_terminal_counts().input == 1);
  assert(C_message_capture_count() == 1);
  char message[120];
  assert(C_message_capture_get(0, message, sizeof(message)));
  assert(strcmp(message, "Combat calculations enabled.") == 0);

  command_key = 27;
  headless_terminal_script("v", 1);
  command(&command_key);

  assert(command_key == 'v');
  assert(!show_combat_calculations);
  assert(reset_flag);
  assert(turn == 1 && turn_counter == 100);
  assert(headless_terminal_counts().input == 2);
  assert(C_message_capture_count() == 2);
  assert(C_message_capture_get(1, message, sizeof(message)));
  assert(strcmp(message, "Combat calculations disabled.") == 0);

  headless_terminal_reset();
  end_pickup_scenario();
}

static void assert_look_message(size_t index, const char *expected) {
  char message[120];
  assert(C_message_capture_get(index, message, sizeof(message)));
  assert(strcmp(message, expected) == 0);
}

static void assert_look_direction_scenario(bool blind, bool monster,
                                           bool object) {
  begin_pickup_scenario();
  player_flags.blind = blind ? 1 : 0;
  cave[char_row][char_col + 1].tptr = object ? ration_slot : 0;
  cave[char_row][char_col + 1].is_permanently_lit = true;
  t_list[ration_slot].number = 5;
  memset(&m_list[2], 0, sizeof(m_list[2]));
  m_list[2].mptr = 2;
  m_list[2].is_seen = monster;
  cave[char_row][char_col + 1].cptr = 2;
  const cave_type original_cell = cave[char_row][char_col + 1];
  const treasure_type original_item = t_list[ration_slot];
  const size_t drawing_before = headless_terminal_counts().drawing;

  C_player_action_look_direction(6);

  if (blind) {
    assert(C_message_capture_count() == 1);
    assert_look_message(0, "You can't see a damn thing!");
  } else if (monster || object) {
    assert(C_message_capture_count() == (size_t)monster + (size_t)object);
    if (monster) {
      assert_look_message(0, "You see a Town Guard.");
    }
    if (object) {
      assert_look_message(monster ? 1 : 0, "You see a ration of food.");
    }
  } else {
    assert(C_message_capture_count() == 1);
    assert_look_message(0, "You see nothing of interest in that direction.");
  }
  assert_same_item(&t_list[ration_slot], &original_item);
  assert(memcmp(&cave[char_row][char_col + 1], &original_cell,
                sizeof(original_cell)) == 0);
  assert(char_row == 39 && char_col == 140);
  assert(turn == 1 && turn_counter == 100 && !reset_flag);
  assert(headless_terminal_counts().drawing == drawing_before);
  end_pickup_scenario();
}

static void assert_look_prompt_scenario(bool cancel, bool blind) {
  begin_pickup_scenario();
  player_flags.blind = blind ? 1 : 0;
  cave[char_row][char_col + 1].is_permanently_lit = true;
  headless_terminal_script(cancel ? "\033" : "l", 1);
  player_action_look();
  assert(headless_terminal_counts().input == 1);
  assert(C_message_capture_count() == (cancel ? 0 : 1));
  if (!cancel) {
    assert_look_message(0, blind ? "You can't see a damn thing!"
                                : "You see a ration of food.");
  }
  assert(reset_flag == cancel);
  assert(char_row == 39 && char_col == 140);
  assert(turn == 1 && turn_counter == 100);
  headless_terminal_reset();
  end_pickup_scenario();
}

static void assert_look_boundary_scenario(void) {
  begin_pickup_scenario();
  player_flags.blind = 0;
  char_col = 2;
  cave[char_row][1].fval = ft_boundry_wall;
  cave[char_row][1].fopen = false;
  cave[char_row][1].is_permanently_lit = true;
  C_player_action_look_direction(4);
  assert(C_message_capture_count() == 1);
  assert_look_message(0, "You see a granite wall.");
  assert(turn == 1 && turn_counter == 100 && !reset_flag);
  end_pickup_scenario();
}

static void assert_rest_input_scenario(const char *input, long expected_turns,
                                     bool was_until_full, bool searching) {
  begin_pickup_scenario();
  player_flags.rest = 3;
  player_flags.resting_till_full = was_until_full;
  player_flags.status = IS_BLIND | (searching ? IS_SEARCHING : 0);
  player_flags.speed = searching ? 1 : 0;
  search_flag = searching;
  find_flag = searching;
  bool started = C_player_action_rest_input(input);
  assert(started == (expected_turns > 0));
  if (started) {
    assert(player_flags.rest == expected_turns);
    assert(turn_counter == 100 + expected_turns);
    assert(player_flags.status == (IS_BLIND | IS_RESTING));
    assert(player_flags.resting_till_full ==
           (was_until_full || strcmp(input, "*") == 0));
    assert(!search_flag && !find_flag);
    assert(player_flags.speed == 0);
    assert(!reset_flag);
  } else {
    assert(player_flags.rest == 3);
    assert(turn_counter == 100);
    assert(player_flags.status == (IS_BLIND | (searching ? IS_SEARCHING : 0)));
    assert(player_flags.resting_till_full == was_until_full);
    assert(search_flag == searching && find_flag == searching);
    assert(player_flags.speed == (searching ? 1 : 0));
    assert(reset_flag);
  }
  assert(turn == 1);
  assert(C_message_capture_count() == 0);
  end_pickup_scenario();
}

static const char *rest_prompt_text;
static bool rest_prompt_completed;
static size_t rest_prompt_calls;

bool headless_rest_get_string(char *text, int row, int col, int length) {
  assert(row == 0 && col == 27 && length == 10);
  assert(strlen(rest_prompt_text) <= (size_t)length);
  rest_prompt_calls++;
  if (rest_prompt_completed) {
    strcpy(text, rest_prompt_text);
  } else {
    memset(text, '7', length);
    memcpy(text, rest_prompt_text, strlen(rest_prompt_text));
  }
  return rest_prompt_completed;
}

static void assert_rest_prompt_scenario(const char *text, bool completed) {
  begin_pickup_scenario();
  rest_prompt_text = text;
  rest_prompt_completed = completed;
  rest_prompt_calls = 0;
  player_flags.rest = 3;
  player_flags.resting_till_full = false;
  player_flags.status = IS_BLIND | IS_SEARCHING;
  player_flags.speed = 1;
  search_flag = find_flag = true;
  player_action_rest();
  assert(rest_prompt_calls == 1);
  if (completed) {
    const long turns = strcmp(text, "*") == 0 ? 20 : 12;
    assert(player_flags.rest == turns && turn_counter == 100 + turns);
    assert(player_flags.resting_till_full == (strcmp(text, "*") == 0));
    assert(player_flags.status == (IS_BLIND | IS_RESTING));
    assert(player_flags.speed == 0 && !search_flag && !find_flag);
    assert(!reset_flag);
    assert(C_message_capture_count() == 1);
    char message[120];
    assert(C_message_capture_get(0, message, sizeof(message)));
    assert(strcmp(message, "Press any key to wake up...") == 0);
    assert(headless_terminal_counts().drawing > 1);
  } else {
    assert(reset_flag);
    assert(player_flags.rest == 3 && turn_counter == 100);
    assert(!player_flags.resting_till_full);
    assert(player_flags.status == (IS_BLIND | IS_SEARCHING));
    assert(player_flags.speed == 1 && search_flag && find_flag);
    assert(C_message_capture_count() == 0);
    assert(headless_terminal_counts().drawing == 1);
    assert(headless_terminal_counts().erasure == 1);
  }
  assert(turn == 1);
  end_pickup_scenario();
}

static void assert_jam_target_scenario(uint8_t tval, long strength,
                                      uint8_t monster, uint16_t spikes,
                                      bool inside_bag,
                                      const char *expected_message) {
  begin_pickup_scenario();
  const long row = char_row;
  const long col = char_col + 1;
  const long slot = ration_slot;
  t_list[slot] = door_list[1];
  t_list[slot].tval = tval;
  t_list[slot].p1 = strength;
  cave[row][col].fopen = tval == open_door;
  cave[row][col].cptr = monster;
  cave[row][col].tptr = tval == 0 ? 0 : slot;
  memset(&m_list[2], 0, sizeof(m_list[2]));

  treasure_type spike_item;
  memset(&spike_item, 0, sizeof(spike_item));
  strcpy(spike_item.name, "& Iron Spike~");
  spike_item.tval = spike;
  spike_item.subval = 1;
  spike_item.number = spikes;
  spike_item.weight = 3;
  if (spikes > 0) {
    treas_rec *spike_ptr = add_inven_item(spike_item);
    spike_ptr->is_in = inside_bag;
    cur_inven = spike_ptr;
  }

  const cave_type original_cell = cave[row][col];
  treasure_type expected_item = t_list[slot];
  const bool success =
      strcmp(expected_message, "You jam the door with a spike.") == 0;
  const size_t drawing_before = headless_terminal_counts().drawing;
  C_player_action_jam_door_target(row, col);

  if (success) {
    expected_item.p1 = -(strength < 0 ? -strength : strength) - 20;
    assert(headless_terminal_counts().drawing > drawing_before);
    if (spikes == 1) {
      assert(inventory_list == NULL && cur_inven == NULL);
      assert(inven_ctr == 0 && inven_weight == 0);
      assert_same_item(&inven_temp.data, &spike_item);
    } else {
      assert(inventory_list != NULL && inventory_list->next == NULL);
      spike_item.number--;
      assert_same_item(&inventory_list->data, &spike_item);
      assert(inventory_list->ok && !inventory_list->is_in);
      assert(inven_ctr == 1 && inven_weight == spikes * spike_item.weight);
    }
  } else {
    assert(headless_terminal_counts().drawing == drawing_before);
    if (spikes > 0) {
      assert(inventory_list != NULL && inventory_list->next == NULL);
      assert_same_item(&inventory_list->data, &spike_item);
      assert(inventory_list->is_in == inside_bag);
      assert(!inventory_list->ok);
      assert(inven_ctr == 1 && inven_weight == spikes * spike_item.weight);
    } else {
      assert(inventory_list == NULL && inven_ctr == 0 && inven_weight == 0);
    }
  }
  assert_same_item(&t_list[slot], &expected_item);
  assert(memcmp(&cave[row][col], &original_cell, sizeof(original_cell)) == 0);
  assert(C_message_capture_count() == 1);
  char message[120];
  assert(C_message_capture_get(0, message, sizeof(message)));
  assert(strcmp(message, expected_message) == 0);
  assert(turn == 1 && turn_counter == 100 && !reset_flag);
  end_pickup_scenario();
}

static void assert_open_messages(const char *first, const char *second,
                                 const char *third) {
  const char *expected[] = {first, second, third};
  size_t count = 0;
  while (count < 3 && expected[count] != NULL) {
    char message[120];
    assert(C_message_capture_get(count, message, sizeof(message)));
    if (strcmp(message, expected[count]) != 0) {
      fprintf(stderr, "Message %zu: expected '%s', got '%s'\n",
              count, expected[count], message);
    }
    assert(strcmp(message, expected[count]) == 0);
    count++;
  }
  assert(C_message_capture_count() == count);
}

static void seed_for_open_roll(long skill, bool success) {
  assert(skill > 2);
  C_seeded_rng_end();
  for (uint64_t seed = 1; seed < 10000; seed++) {
    C_seeded_rng_begin(seed);
    const long roll = randint(100);
    C_seeded_rng_end();
    if ((skill - 1 > roll) == success) {
      C_seeded_rng_begin(seed);
      return;
    }
  }
  assert(false && "Could not find a deterministic lockpick seed");
}

static void assert_open_door_scenario(long strength, bool confused,
                                      bool success, const char *message) {
  begin_pickup_scenario();
  const long row = char_row;
  const long col = char_col + 1;
  const long slot = ration_slot;
  t_list[slot] = door_list[1];
  t_list[slot].p1 = strength;
  cave[row][col].fopen = false;
  cave[row][col].cptr = 2;
  player_flags.confused = confused;
  if (strength > 0 && !confused) {
    seed_for_open_roll(player_disarm(), success);
  }
  const int64_t exp_before = player_exp;
  const cave_type original_cell = cave[row][col];
  treasure_type expected_item = t_list[slot];
  const size_t drawing_before = headless_terminal_counts().drawing;
  C_player_action_open_target(row, col);
  if (success) {
    expected_item = door_list[0];
    assert(cave[row][col].fopen);
    assert(headless_terminal_counts().drawing > drawing_before);
  } else {
    assert(!cave[row][col].fopen);
    assert(headless_terminal_counts().drawing == drawing_before);
  }
  assert_same_item(&t_list[slot], &expected_item);
  cave_type expected_cell = original_cell;
  expected_cell.fopen = success;
  assert(memcmp(&cave[row][col], &expected_cell, sizeof(expected_cell)) == 0);
  assert(player_exp == exp_before + (success && strength > 0 ? 1 : 0));
  assert_open_messages(message, NULL, NULL);
  assert(dun_level == 1 && turn == 1 && turn_counter == 100 && !reset_flag);
  end_pickup_scenario();
}

static size_t count_chest_drops(long row, long col, long slot) {
  size_t drops = 0;
  for (long drop_row = row - 2; drop_row <= row + 2; drop_row++) {
    for (long drop_col = col - 2; drop_col <= col + 2; drop_col++) {
      if (cave[drop_row][drop_col].tptr > 0 &&
          cave[drop_row][drop_col].tptr != slot) {
        drops++;
      }
    }
  }
  return drops;
}

static void assert_open_chest_scenario(bool locked, bool confused,
                                       bool success, bool trapped,
                                       bool has_treasure) {
  begin_pickup_scenario();
  const long row = char_row;
  const long col = char_col + 1;
  const long slot = ration_slot;
  treasure_type *item = &t_list[slot];
  memset(item, 0, sizeof(*item));
  strcpy(item->name, "& Small^ chest (Locked)");
  item->tval = chest;
  item->number = 1;
  item->level = 1;
  item->p1 = 12;
  item->cost = 50;
  item->flags = (locked ? 1 : 0) | (trapped ? 0x40 : 0) |
                (has_treasure ? 0x11000000 : 0);
  player_flags.confused = confused;
  player_flags.free_act = true;
  if (locked && !confused) {
    seed_for_open_roll(player_disarm() - 1, success);
  }
  const treasure_type original_item = *item;
  const cave_type original_cell = cave[row][col];
  const int64_t exp_before = player_exp;
  const long original_level = dun_level;
  const size_t drawing_before = headless_terminal_counts().drawing;
  C_player_action_open_target(row, col);
  assert(dun_level == original_level);
  assert(cave[row][col].tptr == slot);
  assert(memcmp(&cave[row][col], &original_cell, sizeof(original_cell)) == 0);
  if (success) {
    assert(strcmp(item->name, "& Small chest (Empty)") == 0);
    assert(item->flags == 0 && item->cost == 0 && item->p1 == 12);
    assert(player_exp == exp_before + (locked ? item->level : 0));
    if (locked) {
      assert(headless_terminal_counts().drawing > drawing_before);
    }
    assert_open_messages(locked ? "You have picked the lock." :
                         trapped ? "A puff of yellow gas surrounds you!" : NULL,
                         trapped && locked ? "A puff of yellow gas surrounds you!" :
                         trapped ? "You are unaffected." : NULL,
                         trapped && locked ? "You are unaffected." : NULL);
    const size_t drops = count_chest_drops(row, col, slot);
    assert(has_treasure ? drops > 0 : drops == 0);
    C_player_action_open_target(row, col);
    assert(dun_level == original_level && item->flags == 0);
    assert(count_chest_drops(row, col, slot) == drops);
  } else {
    assert_same_item(item, &original_item);
    assert(player_exp == exp_before);
    assert(headless_terminal_counts().drawing == drawing_before);
    assert_open_messages(confused ? "You are too confused to pick the lock." :
                         "You failed to pick the lock.", NULL, NULL);
  }
  assert(player_flags.paralysis == 0);
  assert(turn == 1 && turn_counter == 100 && !reset_flag);
  end_pickup_scenario();
}

static void assert_open_treasure_matches_c_at_chest_level(void) {
  treasure_type expected_drops[5][5];
  bool has_drop[5][5];
  begin_pickup_scenario();
  const long row = char_row;
  const long col = char_col + 1;
  t_list[ration_slot].tval = chest;
  t_list[ration_slot].p1 = 12;
  t_list[ration_slot].flags = 0x12000000;
  C_player_action_open_target(row, col);
  assert(dun_level == 1);
  assert(count_chest_drops(row, col, ration_slot) > 0);
  for (long offset_row = 0; offset_row < 5; offset_row++) {
    for (long offset_col = 0; offset_col < 5; offset_col++) {
      const long index = cave[row + offset_row - 2][col + offset_col - 2].tptr;
      has_drop[offset_row][offset_col] = index > 0 && index != ration_slot;
      if (has_drop[offset_row][offset_col]) {
        expected_drops[offset_row][offset_col] = t_list[index];
      }
    }
  }
  end_pickup_scenario();

  begin_pickup_scenario();
  t_list[ration_slot].tval = chest;
  dun_level = 12;
  monster_death(row, col, 0x12000000);
  for (long offset_row = 0; offset_row < 5; offset_row++) {
    for (long offset_col = 0; offset_col < 5; offset_col++) {
      const long index = cave[row + offset_row - 2][col + offset_col - 2].tptr;
      const bool actual_drop = index > 0 && index != ration_slot;
      assert(actual_drop == has_drop[offset_row][offset_col]);
      if (actual_drop) {
        assert_same_item(&t_list[index], &expected_drops[offset_row][offset_col]);
      }
    }
  }
  end_pickup_scenario();
}

static void seed_for_disarm_outcome(long skill, long level, int outcome) {
  C_seeded_rng_end();
  for (uint64_t seed = 1; seed < 10000; seed++) {
    C_seeded_rng_begin(seed);
    const int actual = skill - level > randint(100) ? 0 :
                       randint(skill) > 5 ? 1 : 2;
    C_seeded_rng_end();
    if (actual == outcome) {
      C_seeded_rng_begin(seed);
      return;
    }
  }
  assert(false && "Could not find a deterministic disarm seed");
}

static void assert_disarm_scenario(bool is_chest, int outcome,
                                  bool blind, bool confused, bool no_light) {
  begin_pickup_scenario();
  const long row = char_row;
  const long old_col = char_col;
  const long col = old_col + 1;
  const long slot = ration_slot;
  treasure_type *item = &t_list[slot];
  memset(item, 0, sizeof(*item));
  strcpy(item->name, is_chest ? "a chest (Gas Trap)|" : "a trap");
  strcpy(item->damage, "0d1");
  item->tval = is_chest ? chest : seen_trap;
  item->flags = is_chest ? 0x80000041 : 0;
  item->number = 1;
  item->level = 1;
  item->p1 = 12;
  cave[row][col].fm = true;
  player_flags.blind = blind;
  player_flags.confused = confused;
  player_flags.free_act = true;
  cave[row][old_col].is_permanently_lit = !no_light;
  long skill = player_disarm();
  if (blind) {
    skill /= 5;
  } else if (no_light) {
    skill /= 2;
  }
  if (confused) {
    skill /= 3;
  }
  seed_for_disarm_outcome(skill, item->level, outcome);
  const treasure_type original = *item;
  const int64_t exp_before = player_exp;
  C_player_action_disarm_target(row, col, 6);
  if (is_chest) {
    assert(char_row == row && char_col == old_col);
    assert(cave[row][col].tptr == slot && cave[row][col].fm);
    if (outcome == 0) {
      assert(item->flags == 0x80000001);
      assert(strcmp(item->name, "a chest (Locked)") == 0);
      assert(player_exp == exp_before + 1);
      assert_open_messages("You have disarmed the chest.", NULL, NULL);
    } else {
      assert_same_item(item, &original);
      assert(player_exp == exp_before);
      assert_open_messages(outcome == 1 ? "You failed to disarm the chest." :
                           "You set a trap off!",
                           outcome == 2 ? "A puff of yellow gas surrounds you!" : NULL,
                           outcome == 2 ? "You are unaffected." : NULL);
    }
    assert(player_flags.paralysis == 0);
  } else if (outcome == 0) {
    assert(cave[row][col].tptr == 0 && !cave[row][col].fm);
    assert(char_row == row && char_col == col);
    assert(player_exp == exp_before + 12);
    assert_open_messages("You have disarmed the trap.", NULL, NULL);
    long freed = 0;
    popt(&freed);
    assert(freed == slot);
    pusht(freed);
  } else {
    assert(cave[row][col].tptr == slot && cave[row][col].fm);
    assert_same_item(item, &original);
    assert(player_exp == exp_before);
    assert(char_row == row && char_col == (outcome == 1 ? old_col : col));
    assert_open_messages(outcome == 1 ? "You failed to disarm the trap." :
                         "You set the trap off!",
                         outcome == 2 ? "You got lucky: unknown trap value." : NULL,
                         NULL);
  }
  assert(turn == 1 && turn_counter == 100 && !reset_flag);
  end_pickup_scenario();
}

static long tunnel_ability(void) {
  return (C_player_get_stat(STR) * 10 + 20) / 5 + 125;
}

static void equip_tunnel_tool(void) {
  equipment[Equipment_primary].tval = 1;
  equipment[Equipment_primary].flags = Tunneling_worn_bit;
  equipment[Equipment_primary].p1 = 2;
}

static uint64_t seed_for_tunnel(long max, long offset, bool success, int loot) {
  const long ability = tunnel_ability();
  C_seeded_rng_end();
  for (uint64_t seed = 1; seed < 10000; seed++) {
    C_seeded_rng_begin(seed);
    const long difficulty = randint(max) + offset;
    const bool drops = success && max == 180 && randint(10) == 1;
    C_seeded_rng_end();
    if ((ability > difficulty) == success &&
        (loot < 0 || drops == (loot != 0))) {
      C_seeded_rng_begin(seed);
      return seed;
    }
  }
  assert(false && "Could not find a deterministic tunnel seed");
  return 0;
}

static void assert_tunnel_wall_matches_c(uint8_t terrain, bool success) {
  begin_pickup_scenario();
  equip_tunnel_tool();
  const long row = char_row;
  const long col = char_col + 1;
  cave[row][col].fval = terrain;
  cave[row][col].fopen = false;
  cave[row][col].fm = true;
  cave[row][col].is_permanently_lit = true;
  cave[row][col].cptr = 2;
  const cave_type original = cave[row][col];
  const treasure_type original_item = t_list[ration_slot];
  const long max = terrain == 10 ? 1200 : terrain == 11 ? 600 : 400;
  const long offset = terrain == 10 ? 80 : 10;
  const uint64_t seed = seed_for_tunnel(max, offset, success, -1);
  assert(twall(row, col, tunnel_ability(), randint(max) + offset) == success);
  const cave_type expected = cave[row][col];
  cave[row][col] = original;
  C_message_capture_end();
  C_message_capture_begin();
  C_seeded_rng_end();
  C_seeded_rng_begin(seed);
  C_player_action_tunnel_target(row, col);
  assert(memcmp(&cave[row][col], &expected, sizeof(expected)) == 0);
  assert_same_item(&t_list[ration_slot], &original_item);
  const char *message = success ? "You have finished the tunnel." :
      terrain == 10 ? "You tunnel into the granite wall." :
      terrain == 11 ? "You tunnel into the magma intrusion." :
                      "You tunnel into the quartz vein.";
  assert_open_messages(success ? "You have found something!" : message,
                       success ? message : NULL, NULL);
  assert(turn == 1 && turn_counter == 100 && !reset_flag);
  end_pickup_scenario();
}

static void assert_tunnel_rubble_scenario(bool success, bool loot, bool lit) {
  begin_pickup_scenario();
  equip_tunnel_tool();
  const long row = char_row;
  const long col = char_col + 1;
  t_list[ration_slot].tval = rubble;
  cave[row][col].fopen = false;
  cave[row][col].fm = true;
  cave[row][col].is_permanently_lit = lit;
  const cave_type original = cave[row][col];
  const treasure_type original_item = t_list[ration_slot];
  const uint64_t seed = seed_for_tunnel(180, 0, success, loot);
  const size_t drawing = headless_terminal_counts().drawing;
  C_player_action_tunnel_target(row, col);
  if (success) {
    assert(cave[row][col].fopen && !cave[row][col].fm);
    assert((cave[row][col].tptr > 0) == loot);
    assert(headless_terminal_counts().drawing > drawing);
    if (loot) {
      assert(t_list[cave[row][col].tptr].tval != rubble);
    } else {
      long freed = 0;
      popt(&freed);
      assert(freed == ration_slot);
      pusht(freed);
    }
    assert_open_messages("You have removed the rubble.",
                         loot && lit ? "You have found something!" : NULL, NULL);
  } else {
    assert(memcmp(&cave[row][col], &original, sizeof(original)) == 0);
    assert_same_item(&t_list[ration_slot], &original_item);
    assert(headless_terminal_counts().drawing == drawing);
    assert_open_messages("You dig in the rubble...", NULL, NULL);
  }
  assert(turn == 1 && turn_counter == 100 && !reset_flag);
  if (success && loot) {
    const treasure_type generated_item = t_list[cave[row][col].tptr];
    end_pickup_scenario();

    begin_pickup_scenario();
    equip_tunnel_tool();
    const long replay_row = char_row;
    const long replay_col = char_col + 1;
    t_list[ration_slot].tval = rubble;
    cave[replay_row][replay_col].fopen = false;
    cave[replay_row][replay_col].fm = true;
    cave[replay_row][replay_col].is_permanently_lit = lit;
    C_seeded_rng_end();
    C_seeded_rng_begin(seed);
    C_player_action_tunnel_target(replay_row, replay_col);
    assert(cave[replay_row][replay_col].tptr > 0);
    assert_same_item(&t_list[cave[replay_row][replay_col].tptr], &generated_item);
    end_pickup_scenario();
    return;
  }
  end_pickup_scenario();
}

static void assert_tunnel_target_message(uint8_t terrain, uint8_t tval,
                                         uint8_t monster, const char *message) {
  begin_pickup_scenario();
  const long row = char_row;
  const long col = char_col + 1;
  t_list[ration_slot].tval = tval;
  cave[row][col].fval = terrain;
  cave[row][col].cptr = monster;
  if (tval == 0) {
    cave[row][col].tptr = 0;
  }
  const cave_type original = cave[row][col];
  C_player_action_tunnel_target(row, col);
  assert(memcmp(&cave[row][col], &original, sizeof(original)) == 0);
  assert_open_messages(message, tval == secret_door ?
                       "You are incapable of searching while blind." : NULL, NULL);
  assert(turn == 1 && turn_counter == 100 && !reset_flag);
  end_pickup_scenario();
}

static void assert_tunnel_border_messages(void) {
  begin_pickup_scenario();
  const long targets[4][2] = {{1, char_col}, {cur_height, char_col},
                             {char_row, 1}, {char_row, cur_width}};
  for (size_t target = 0; target < 4; target++) {
    const long row = targets[target][0];
    const long col = targets[target][1];
    cave[row][col].fval = 15;
    const cave_type original = cave[row][col];
    C_message_capture_end();
    C_message_capture_begin();
    C_player_action_tunnel_target(row, col);
    assert_open_messages("This seems to be permanent rock.", NULL, NULL);
    assert(memcmp(&cave[row][col], &original, sizeof(original)) == 0);
  }
  assert(turn == 1 && turn_counter == 100 && !reset_flag);
  end_pickup_scenario();
}

static void seed_for_bash_chest(bool destroyed, bool unlocked) {
  C_seeded_rng_end();
  for (uint64_t seed = 1; seed < 10000; seed++) {
    C_seeded_rng_begin(seed);
    const bool first = randint(10) == 1;
    const bool second = randint(10) == 1;
    C_seeded_rng_end();
    if (first == destroyed && second == unlocked) {
      C_seeded_rng_begin(seed);
      return;
    }
  }
  assert(false && "Could not find a deterministic chest bash seed");
}

static void assert_bash_chest_scenario(bool destroyed, bool unlocked,
                                      bool locked) {
  begin_pickup_scenario();
  const long row = char_row;
  const long col = char_col + 1;
  treasure_type *item = &t_list[ration_slot];
  strcpy(item->name, "& Small^ chest (Locked)");
  item->tval = chest;
  item->flags = locked ? UINT64_MAX : 0x11000040;
  item->flags2 = 42;
  item->cost = 50;
  treasure_type expected = *item;
  const cave_type original_cell = cave[row][col];
  const int64_t original_exp = player_exp;
  const size_t drawing_before = headless_terminal_counts().drawing;
  seed_for_bash_chest(destroyed, unlocked);
  C_player_action_bash_target(row, col);
  if (destroyed) {
    strcpy(expected.name, "& ruined chest");
    expected.flags = 0;
    assert_open_messages("You have destroyed the chest...",
                         "and its contents!", NULL);
  } else if (locked && unlocked) {
    expected.flags &= 0xFFFFFFFE;
    assert_open_messages("The lock breaks open!", NULL, NULL);
  } else {
    assert_open_messages(NULL, NULL, NULL);
  }
  assert_same_item(item, &expected);
  assert(memcmp(&cave[row][col], &original_cell, sizeof(original_cell)) == 0);
  assert(headless_terminal_counts().drawing == drawing_before);
  assert(player_flags.paralysis == 0 && player_exp == original_exp);
  assert(dun_level == 1 && turn == 1 && turn_counter == 100 && !reset_flag);
  end_pickup_scenario();
}

static bool run_bash_door_scenario(uint64_t seed) {
  begin_pickup_scenario();
  const long row = char_row;
  const long col = char_col + 1;
  treasure_type *item = &t_list[ration_slot];
  strcpy(item->name, "a closed door");
  item->tval = closed_door;
  item->p1 = 0;
  cave[row][col].fopen = false;

  C_seeded_rng_end();
  C_seeded_rng_begin(seed);
  C_player_action_bash_target(row, col);

  const bool opened = cave[row][col].fopen;
  assert_open_messages(opened ? "You smash into the door! The door crashes open!"
                              : "You smash into the door! The door holds firm.",
                       NULL, NULL);
  assert(turn == 1 && turn_counter == 100 && !reset_flag);
  end_pickup_scenario();
  return opened;
}

static void assert_bash_door_seeded_outcomes(void) {
  uint64_t success_seed = 0;
  uint64_t failure_seed = 0;
  for (uint64_t seed = 1; seed < 256 && (!success_seed || !failure_seed);
       seed++) {
    const bool opened = run_bash_door_scenario(seed);
    if (opened && !success_seed) {
      success_seed = seed;
    } else if (!opened && !failure_seed) {
      failure_seed = seed;
    }
  }
  assert(success_seed && failure_seed);
  assert(run_bash_door_scenario(success_seed));
  assert(!run_bash_door_scenario(failure_seed));
}

static void assert_bash_prompt_scenario(bool cancel, bool afraid) {
  begin_pickup_scenario();
  const long row = char_row;
  const long col = char_col + 1;
  cave[row][col].cptr = afraid ? 2 : 0;
  player_flags.afraid = afraid ? 1 : 0;
  const cave_type original_cell = cave[row][col];
  const treasure_type original_item = t_list[ration_slot];
  const treasure_type original_weapon = equipment[Equipment_primary];
  headless_terminal_script(cancel ? "\033" : "l", 1);
  player_action_bash();
  assert(headless_terminal_counts().input == 1);
  assert_open_messages(cancel ? NULL : afraid ? "You are afraid!" :
                       "I do not see anything you can bash there.", NULL, NULL);
  assert(reset_flag == cancel);
  assert_same_item(&t_list[ration_slot], &original_item);
  assert_same_item(&equipment[Equipment_primary], &original_weapon);
  assert(memcmp(&cave[row][col], &original_cell, sizeof(original_cell)) == 0);
  assert(player_flags.paralysis == 0 && char_row == row && char_col == col - 1);
  assert(turn == 1 && turn_counter == 100);
  headless_terminal_reset();
  end_pickup_scenario();
}

static void assert_search_scenario(bool blind) {
  begin_pickup_scenario();
  player_flags.blind = blind ? 1 : 0;

  const long row = char_row;
  const long trap_col = char_col - 1;
  const long door_col = char_col + 1;
  long door_slot = 0;
  popt(&door_slot);
  assert(door_slot > 0);

  cave[row][char_col + 1].tptr = 0;
  cave[row][trap_col].tptr = ration_slot;
  t_list[ration_slot].tval = unseen_trap;
  t_list[ration_slot].subval = 2;
  strcpy(t_list[ration_slot].name, "an arrow trap");

  cave[row][door_col].tptr = door_slot;
  t_list[door_slot].tval = secret_door;
  t_list[door_slot].subval = 2;
  strcpy(t_list[door_slot].name, "a secret door");

  player_action_search(row, char_col, 1000);

  if (blind) {
    assert_open_messages("You are incapable of searching while blind.",
                         NULL, NULL);
    assert(cave[row][trap_col].tptr == ration_slot);
    assert(cave[row][door_col].tptr == door_slot);
    assert(t_list[ration_slot].tval == unseen_trap);
    assert(t_list[door_slot].tval == secret_door);
  } else {
    assert_open_messages("You have found an arrow trap.",
                         "You have found a secret door.", NULL);
    assert(t_list[cave[row][trap_col].tptr].tval == seen_trap);
    assert(t_list[cave[row][door_col].tptr].tval == seen_trap);
    assert(cave[row][door_col].fval == 5);
  }

  end_pickup_scenario();
}

static void assert_stairs_scenario(bool ascend, uint8_t tile,
                                   long expected_level, bool expected_transition,
                                   const char *first_message,
                                   const char *second_message) {
  begin_pickup_scenario();
  dun_level = 5;
  cave[char_row][char_col].tptr = ration_slot;
  t_list[ration_slot].tval = tile;

  if (ascend) {
    player_action_ascend_stairs();
  } else {
    player_action_descend_stairs();
  }

  assert(dun_level == expected_level);
  assert(moria_flag == expected_transition);
  assert_open_messages(first_message, second_message, NULL);
  moria_flag = false;
  end_pickup_scenario();
}

int main(void) {
  alarm(30);
  debug_file = fopen("target/debug/headless-interaction.log", "w");
  assert(debug_file != NULL);
  for (int repeat = 0; repeat < 2; repeat++) {
    assert_toggle_light_scenario(15, 123, false, "Light On.  123 turns left.");
    assert_toggle_combat_calculations_scenario();
    assert_toggle_light_scenario(15, 123, true, "Light Off.  123 turns left.");
    assert_toggle_light_scenario(0, 123, true, "You are not carrying a light.");
    assert_toggle_light_scenario(0, 0, false, "You are not carrying a light.");
    assert_toggle_light_scenario(15, 0, true, "Your light has gone out!");
    assert_toggle_light_scenario(15, -1, false, "Your light has gone out!");
    assert_ration_pickup_scenario();
    assert_refill_scenario(1, 100, 7600, 2);
    assert_refill_scenario(9, 14000, 15000, 1);
    assert_refill_scenario(1, 15000, 15000, 2);
    assert_refill_refused(0, true, false, "But you are not using a lamp.");
    assert_refill_refused(10, true, false, "But you are not using a lamp.");
    assert_refill_refused(1, false, false, "You have no oil.");
    assert_refill_refused(1, true, true, "You have no oil.");
    assert_rest_prompt_scenario("", false);
    assert_rest_prompt_scenario("12", false);
    assert_rest_prompt_scenario("*", true);
    assert_rest_prompt_scenario("12junk", true);
    assert_rest_input_scenario("*", 20, false, true);
    assert_rest_input_scenario("12junk", 12, false, true);
    assert_rest_input_scenario("12junk", 12, true, true);
    assert_rest_input_scenario(" +7", 7, false, false);
    assert_rest_input_scenario("0", 0, false, true);
    assert_rest_input_scenario("0", 0, true, true);
    assert_rest_input_scenario("-4", -4, true, false);
    assert_rest_input_scenario("junk", 0, true, true);
    assert_rest_input_scenario("", 0, false, false);
    assert_close_target_scenario(open_door, 0, 0, false, NULL);
    assert_close_target_scenario(open_door, 1, 2, false, "It is in your way!");
    assert_close_target_scenario(open_door, 1, 0, false,
                                "The door appears to be broken.");
    assert_close_target_scenario(open_door, -1, 0, false,
                                "The door appears to be broken.");
    assert_close_target_scenario(closed_door, 0, 2, false,
                                "I do not see anything you can close there.");
    assert_close_target_scenario(open_door, 0, 0, true,
                                "I do not see anything you can close there.");
    assert_look_direction_scenario(false, true, true);
    assert_look_direction_scenario(false, true, false);
    assert_look_direction_scenario(false, false, true);
    assert_look_direction_scenario(false, false, false);
    assert_look_direction_scenario(true, true, true);
    assert_look_boundary_scenario();
    assert_look_prompt_scenario(false, false);
    assert_look_prompt_scenario(false, true);
    assert_look_prompt_scenario(true, false);
    assert_look_prompt_scenario(true, true);
    assert_jam_target_scenario(closed_door, 0, 0, 2, false,
                              "You jam the door with a spike.");
    assert_jam_target_scenario(closed_door, 7, 0, 300, false,
                              "You jam the door with a spike.");
    assert_jam_target_scenario(closed_door, -40, 0, 1, false,
                              "You jam the door with a spike.");
    assert_jam_target_scenario(closed_door, 7, 0, 0, false,
                              "But you have no spikes...");
    assert_jam_target_scenario(closed_door, 7, 0, 2, true,
                              "But you have no spikes...");
    assert_jam_target_scenario(open_door, 0, 2, 2, false,
                              "The door must be closed first.");
    assert_jam_target_scenario(closed_door, 0, 2, 2, false,
                              "It is in your way!");
    assert_jam_target_scenario(Food, 0, 2, 2, false, "That isn't a door!");
    assert_jam_target_scenario(0, 0, 0, 2, false, "That isn't a door!");
    assert_open_door_scenario(0, false, true, NULL);
    assert_open_door_scenario(1, false, true, "You have picked the lock.");
    assert_open_door_scenario(1, false, false, "You failed to pick the lock.");
    assert_open_door_scenario(-20, false, false, "It appears to be stuck.");
    assert_open_door_scenario(1, true, false, "You are too confused to pick the lock.");
    assert_open_chest_scenario(true, false, true, false, false);
    assert_open_chest_scenario(true, false, false, true, true);
    assert_open_chest_scenario(true, true, false, true, true);
    assert_open_chest_scenario(true, false, true, true, true);
    assert_open_chest_scenario(false, false, true, true, true);
    assert_open_chest_scenario(false, true, true, false, false);
    assert_open_treasure_matches_c_at_chest_level();
    assert_disarm_scenario(false, 0, false, false, false);
    assert_disarm_scenario(false, 1, false, false, false);
    assert_disarm_scenario(false, 2, false, false, false);
    assert_disarm_scenario(true, 0, false, false, false);
    assert_disarm_scenario(true, 1, false, false, false);
    assert_disarm_scenario(true, 2, false, false, false);
    assert_disarm_scenario(true, 1, true, false, false);
    assert_disarm_scenario(true, 2, true, true, true);
    assert_disarm_scenario(true, 1, false, true, false);
    assert_disarm_scenario(true, 1, false, false, true);
    for (uint8_t terrain = 10; terrain <= 12; terrain++) {
      assert_tunnel_wall_matches_c(terrain, true);
      assert_tunnel_wall_matches_c(terrain, false);
    }
    assert_tunnel_rubble_scenario(false, false, true);
    assert_tunnel_rubble_scenario(true, false, true);
    assert_tunnel_rubble_scenario(true, true, true);
    assert_tunnel_rubble_scenario(true, true, false);
    assert_tunnel_target_message(15, 0, 0, "This seems to be permanent rock.");
    assert_tunnel_target_message(16, 0, 0, "You can't tunnel through water!");
    assert_tunnel_target_message(5, closed_door, 0, "You can't tunnel through that.");
    assert_tunnel_target_message(5, closed_door, 2, "You can't tunnel through that.");
    assert_tunnel_target_message(1, 0, 2, "Tunnel through what?  Empty air???");
    assert_tunnel_target_message(1, 0, 0, "Tunnel through what?  Empty air???");
    assert_tunnel_target_message(1, secret_door, 0, "You tunnel into the granite wall.");
    assert_tunnel_border_messages();
    assert_bash_chest_scenario(true, false, true);
    assert_bash_chest_scenario(true, false, false);
    assert_bash_chest_scenario(false, true, true);
    assert_bash_chest_scenario(false, false, true);
    assert_bash_chest_scenario(false, true, false);
    assert_bash_door_seeded_outcomes();
    assert_bash_prompt_scenario(true, false);
    assert_bash_prompt_scenario(false, false);
    assert_bash_prompt_scenario(false, true);
    assert_search_scenario(false);
    assert_search_scenario(true);
    assert_stairs_scenario(true, up_staircase, 4, true,
                           "You enter a maze of up staircases.",
                           "You pass through a one-way door.");
    assert_stairs_scenario(false, down_staircase, 6, true,
                           "You enter a maze of down staircases.",
                           "You pass through a one-way door.");
    assert_stairs_scenario(true, down_staircase, 5, false,
                           "I see no up staircase here.", NULL);
    assert_stairs_scenario(false, up_staircase, 5, false,
                           "I see no down staircase here.", NULL);
  }
  assert(fclose(debug_file) == 0);
  debug_file = NULL;
  alarm(0);
  puts("Headless interaction checks passed.");
  return 0;
}
