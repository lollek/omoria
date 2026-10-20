#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../src/constants.h"
#include "../src/debug.h"
#include "../src/messages.h"
#include "../src/player.h"
#include "../src/player_action.h"
#include "../src/random.h"
#include "../src/screen.h"
#include "../src/variables.h"
#include "support/terminal.h"

static void assert_message(size_t index, const char *expected) {
  char buffer[256];
  assert(C_message_capture_get(index, buffer, sizeof(buffer)));
  if (strcmp(buffer, expected) != 0)
    fprintf(stderr, "Message %zu: expected [%s], got [%s]\n", index, expected,
            buffer);
  assert(strcmp(buffer, expected) == 0);
}

static void reset_case(void) {
  assert(inventory_list == NULL);
  memset(equipment, 0, sizeof(equipment));
  memset(t_list, 0, sizeof(t_list));
  memset(&inven_temp, 0, sizeof(inven_temp));
  memset(&blank_treasure, 0, sizeof(blank_treasure));
  memset(&player_flags, 0, sizeof(player_flags));
  inven_ctr = inven_weight = equip_ctr = 0;
  cur_inven = NULL;
  player_lev = 2;
  player_exp = player_max_exp = 100;
  player_wt = 150;
  player_prace = player_pclass = 0;
  strcpy(player_sex, "Male");
  player_flags.foodc = 1000;
  player_flags.status = IS_HUNGERY | IS_WEAK;
  death = moria_flag = false;
  total_winner = true;
  reset_flag = false;
  msg_flag = msg_terse = false;
  print_stat = 0;
  last_printed_message[0] = '\0';
  cur_height = cur_width = 3;
  char_row = char_col = 2;
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

static treas_rec *food(long subval, unsigned long flags, int number,
                       long nutrition, int level) {
  treas_rec *item = calloc(1, sizeof(*item));
  assert(item != NULL);
  strcpy(item->data.name, "& Mushroom~| of Poison");
  strcpy(item->data.damage, "0d0");
  item->data.tval = Food;
  item->data.subval = subval;
  item->data.flags = flags;
  item->data.number = number;
  item->data.weight = 3;
  item->data.level = level;
  item->data.p1 = nutrition;
  inventory_list = cur_inven = item;
  inven_ctr = 1;
  inven_weight = number * item->data.weight;
  return item;
}

static void finish_case(size_t messages) {
  assert(C_message_capture_count() == messages);
  assert(headless_terminal_counts().input == 0);
  assert(!death && !moria_flag);
  C_message_capture_end();
  while (inventory_list != NULL) {
    treas_rec *next = inventory_list->next;
    free(inventory_list);
    inventory_list = next;
  }
  cur_inven = NULL;
}

static void assert_stack_consumption_updates_hunger_and_inventory(void) {
  reset_case();
  treas_rec *item = food(307, 0, 2, 50, 0);

  C_player_action_eat_selected(item);

  assert(player_flags.foodc == 1050);
  assert((player_flags.status & (IS_HUNGERY | IS_WEAK)) == 0);
  assert(inventory_list == item && cur_inven == item);
  assert(item->data.number == 1 && inven_ctr == 1 && inven_weight == 3);
  assert(inven_temp.data.number == 1);
  assert_message(0, "You have ration of food.");
  finish_case(1);
}

static void assert_poison_food_consumes_last_item_through_c_boundary(void) {
  C_seeded_rng_begin(73);
  const long poison = randint(10) + 3;
  const long next = randint(1000);
  C_seeded_rng_end();
  reset_case();
  treas_rec *item = food(257, 1, 1, 50, 3);

  C_seeded_rng_begin(73);
  C_player_action_eat_selected(item);
  assert(randint(1000) == next);
  C_seeded_rng_end();

  assert(player_flags.poisoned == poison);
  assert(player_flags.foodc == 1050 && player_exp == 102);
  assert(inventory_list == NULL && cur_inven == NULL);
  assert(inven_ctr == 0 && inven_weight == 0);
  assert(inven_temp.data.number == 1);
  assert_message(0, "You have no more mushrooms.");
  finish_case(1);
}

static void assert_status_updates_preserve_other_player_flags(void) {
  C_seeded_rng_begin(73);
  const long poison = randint(10) + 3;
  const long fear = randint(10) + 3;
  const long confusion = randint(10) + 3;
  const long hallucination = randint(200) + 25 * 3 + 200;
  const long next = randint(1000);
  C_seeded_rng_end();
  reset_case();
  treas_rec *item = food(257, 1 | (1UL << 2) | (1UL << 3) | (1UL << 4),
                        2, 50, 3);
  player_flags.poisoned = 5;
  player_flags.afraid = 7;
  player_flags.confused = 9;
  player_flags.image = 11;
  player_flags.blind = 13;
  player_flags.hero = 15;
  player_flags.paralysis = 17;

  C_seeded_rng_begin(73);
  C_player_action_eat_selected(item);
  assert(randint(1000) == next);
  C_seeded_rng_end();

  assert(player_flags.poisoned == 5 + poison);
  assert(player_flags.afraid == 7 + fear);
  assert(player_flags.confused == 9 + confusion);
  assert(player_flags.image == 11 + hallucination);
  assert(player_flags.blind == 13 && player_flags.hero == 15);
  assert(player_flags.paralysis == 17);
  assert_message(0, "You feel terrified!");
  assert_message(1, "You feel drugged.");
  assert_message(2, "You have mushroom.");
  finish_case(3);
}

static void assert_flavor_clears_only_the_selected_secondary_flag(void) {
  static const char *messages[] = {"*munch* *munch*", "*crunch* *crunch*",
                                   "*munch* *crunch*", "*crunch* *munch*"};
  C_seeded_rng_begin(73);
  const long flavor = randint(4);
  const long next = randint(1000);
  C_seeded_rng_end();
  reset_case();
  treas_rec *item = food(307, 1UL << 30, 2, 50, 0);
  item->data.flags2 = (1UL << 63) | 3;

  C_seeded_rng_begin(73);
  C_player_action_eat_selected(item);
  assert(randint(1000) == next);
  C_seeded_rng_end();

  assert(item->data.flags2 == ((1UL << 63) | 2));
  assert(item->data.flags == (1UL << 30));
  assert(item->data.number == 1 && inven_weight == 3);
  assert_message(0, messages[flavor - 1]);
  assert_message(1, "You have ration of food.");
  finish_case(2);
}

int main(void) {
  assert(debug_file == NULL);
  assert_stack_consumption_updates_hunger_and_inventory();
  assert_poison_food_consumes_last_item_through_c_boundary();
  assert_status_updates_preserve_other_player_flags();
  assert_flavor_clears_only_the_selected_secondary_flag();
  puts("Eat C caller checks passed.");
  return 0;
}
