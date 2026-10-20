#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../src/constants.h"
#include "../src/debug.h"
#include "../src/messages.h"
#include "../src/misc.h"
#include "../src/player.h"
#include "../src/player_action.h"
#include "../src/random.h"
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
  player_lev = 10;
  player_exp = player_max_exp = 100;
  player_wt = 150;
  player_flags.foodc = 1000;
  player_flags.status = IS_HUNGERY | IS_WEAK;
  search_flag = find_flag = death = moria_flag = total_winner = false;
  reset_flag = true;
  msg_flag = msg_terse = false;
  print_stat = 0;
  last_printed_message[0] = '\0';
  headless_terminal_reset();
  headless_terminal_allow_drawing(true);
  C_message_capture_begin();
}

static treas_rec *potion(long subval, unsigned long flags, unsigned long flags2,
                         int number) {
  treas_rec *item = calloc(1, sizeof(*item));
  assert(item != NULL);
  strcpy(item->data.name, "Test| potion");
  strcpy(item->data.damage, "1d1");
  item->data.tval = potion1;
  item->data.subval = subval;
  item->data.flags = flags;
  item->data.flags2 = flags2;
  item->data.number = number;
  item->data.weight = 3;
  item->data.level = 20;
  item->data.p1 = 100;
  inventory_list = cur_inven = item;
  inven_ctr = 1;
  inven_weight = number * item->data.weight;
  return item;
}

static void finish_case(size_t messages, size_t input) {
  assert(C_message_capture_count() == messages);
  assert(headless_terminal_counts().input == input);
  assert(!death && !moria_flag);
  C_message_capture_end();
  while (inventory_list != NULL) {
    treas_rec *next = inventory_list->next;
    free(inventory_list);
    inventory_list = next;
  }
  cur_inven = NULL;
}

static void assert_consumed_last(void) {
  assert(inventory_list == NULL && cur_inven == NULL);
  assert(inven_ctr == 0 && inven_weight == 0);
  assert(inven_temp.data.number == 1);
}

static void assert_refusal_and_cancel(void) {
  reset_case();
  C_player_action_quaff_selected(NULL);
  assert(reset_flag && inventory_list == NULL && inven_ctr == 0);
  assert(player_exp == 100 && player_flags.foodc == 1000);
  assert(C_message_capture_count() == 0);
  player_action_quaff_potion();
  assert(reset_flag && player_exp == 100 && player_flags.foodc == 1000);
  assert_message(0, "But you are not carrying anything.");
  finish_case(1, 0);

  reset_case();
  treas_rec *item = potion(283, 0, 0, 1);
  item->data.tval = sword;
  player_action_quaff_potion();
  assert(reset_flag && inventory_list == item && item->data.number == 1);
  assert(inven_ctr == 1 && inven_weight == 3);
  assert(player_exp == 100 && player_flags.foodc == 1000);
  assert_message(0, "You are not carrying any potions.");
  finish_case(1, 0);

  reset_case();
  item = potion(275, 1UL << 18, 0, 2);
  headless_terminal_script("\033", 1);
  player_action_quaff_potion();
  assert(reset_flag && inventory_list == item && cur_inven == item);
  assert(item->data.number == 2 && inven_ctr == 1 && inven_weight == 6);
  assert(player_exp == 100 && player_flags.foodc == 1000);
  assert(player_flags.paralysis == 0);
  assert(strcmp(item->data.name, "Test| potion") == 0);
  finish_case(0, 1);
}

static void assert_prompt_consumes_stack_and_identifies(void) {
  C_seeded_rng_begin(42);
  const long duration = randint(4) + 4;
  const long next = randint(1000);
  C_seeded_rng_end();
  reset_case();
  treas_rec *item = potion(275, 1UL << 18, 0, 3);
  t_list[1] = equipment[Equipment_light] = item->data;
  player_flags.paralysis = 7;
  headless_terminal_script("a", 1);
  C_seeded_rng_begin(42);
  player_action_quaff_potion();
  assert(randint(1000) == next);
  C_seeded_rng_end();
  assert(!reset_flag && player_flags.paralysis == 7 + duration);
  assert(inventory_list == item && cur_inven == item && inven_ctr == 1);
  assert(item->data.number == 2 && inven_weight == 6);
  assert(item->data.flags == (1UL << 18) && item->data.flags2 == 0);
  assert(item->data.identified == 0);
  assert(strcmp(item->data.name, "Test potion") == 0);
  assert(strcmp(t_list[1].name, "Test potion") == 0);
  assert(strcmp(equipment[Equipment_light].name, "Test potion") == 0);
  assert(player_exp == 102 && player_flags.foodc == 1100);
  assert((player_flags.status & (IS_HUNGERY | IS_WEAK)) == 0);
  assert_message(0, "You fall asleep.");
  assert_message(1, "You have 2 potions of sleep.");
  finish_case(2, 1);
}

static void assert_real_healing_and_last_item(void) {
  C_seeded_rng_begin(73);
  const long healing = damroll("2d7");
  const long next = randint(1000);
  C_seeded_rng_end();
  reset_case();
  potion(269, 1UL << 12, 0, 1);
  C_player_modify_max_hp(200);
  assert(C_player_max_hp() > 64);
  C_player_modify_current_hp(50 - C_player_current_hp());
  assert(C_player_current_hp() == 50);
  C_seeded_rng_begin(73);
  C_player_action_quaff_selected(inventory_list);
  assert(randint(1000) == next);
  C_seeded_rng_end();
  assert(C_player_current_hp() == 50 + healing);
  assert_consumed_last();
  assert(strcmp(inven_temp.data.name, "Test potion") == 0);
  assert(player_exp == 102 && player_flags.foodc == 1100);
  assert_message(0, healing < 5 ? "You feel a little better."
                               : "You feel better.");
  assert_message(1, "You have no more potions of cure light wounds.");
  finish_case(2, 0);
}

static void assert_last_effect_controls_identification(void) {
  C_seeded_rng_begin(91);
  const long duration = randint(25) + 15;
  const long next = randint(1000);
  C_seeded_rng_end();
  reset_case();
  treas_rec *item = potion(283, (1UL << 22) | (1UL << 28), 0, 2);
  player_flags.blind = 9;
  C_seeded_rng_begin(91);
  C_player_action_quaff_selected(item);
  assert(randint(1000) == next);
  C_seeded_rng_end();
  assert(player_flags.fast == duration && player_flags.blind == 1);
  assert(strcmp(item->data.name, "Test| potion") == 0);
  assert(item->data.number == 1 && inven_weight == 3);
  assert(player_exp == 102 && player_flags.foodc == 1100);
  assert_message(0, "You have unknown potion.");
  finish_case(1, 0);
}

static void assert_flags2_offset_and_real_cure(void) {
  reset_case();
  treas_rec *item = potion(298, 1UL << 28, 1UL << 10, 2);
  player_flags.blind = 9;
  player_flags.poisoned = 9;
  C_player_action_quaff_selected(item);
  assert(player_flags.blind == 1 && player_flags.poisoned == 4);
  assert(item->data.number == 1 && inven_weight == 3 && inven_ctr == 1);
  assert(strcmp(item->data.name, "Test potion") == 0);
  assert(item->data.flags == (1UL << 28) &&
         item->data.flags2 == (1UL << 10));
  assert(player_exp == 102 && player_flags.foodc == 1100);
  assert_message(0, "The effects of the poison has been reduced.");
  assert_message(1, "You have potion of slow poison.");
  finish_case(2, 0);
}

static void assert_resistance_does_not_identify(void) {
  C_seeded_rng_begin(123);
  const long duration = randint(10) + 10;
  const long next = randint(1000);
  C_seeded_rng_end();
  reset_case();
  treas_rec *item = potion(295, 0, 1UL << 7, 2);
  player_flags.resist_heat = 5;
  C_seeded_rng_begin(123);
  C_player_action_quaff_selected(item);
  assert(randint(1000) == next);
  C_seeded_rng_end();
  assert(player_flags.resist_heat == 5 + duration);
  assert(strcmp(item->data.name, "Test| potion") == 0);
  assert(item->data.number == 1 && inven_weight == 3);
  assert(player_exp == 100 && player_flags.foodc == 1100);
  assert_message(0, "You have unknown potion.");
  finish_case(1, 0);
}

static void assert_salt_water_before_nutrition(void) {
  reset_case();
  potion(289, 0, 1UL << 1, 1);
  player_flags.poisoned = 20;
  player_flags.status |= IS_POISONED;
  player_flags.paralysis = 9;
  C_player_action_quaff_selected(inventory_list);
  assert_consumed_last();
  assert(player_flags.poisoned == 0 && player_flags.paralysis == 4);
  assert((player_flags.status & (IS_POISONED | IS_HUNGERY | IS_WEAK)) == 0);
  assert(player_flags.foodc == 250 && player_exp == 100);
  assert_message(0, "The potion makes you vomit!");
  assert_message(1, "You have no more potions of salt water.");
  finish_case(2, 0);
}

int main(void) {
  debug_file = fopen("target/debug/quaff-ffi.log", "w");
  assert(debug_file != NULL);
  player_prace = player_pclass = 0;
  strcpy(player_sex, "Male");
  assert_refusal_and_cancel();
  assert_prompt_consumes_stack_and_identifies();
  assert_real_healing_and_last_item();
  assert_last_effect_controls_identification();
  assert_flags2_offset_and_real_cure();
  assert_resistance_does_not_identify();
  assert_salt_water_before_nutrition();
  assert(fclose(debug_file) == 0);
  debug_file = NULL;
  puts("Quaff C caller checks passed.");
  return 0;
}
