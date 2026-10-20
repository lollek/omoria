#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../src/debug.h"
#include "../src/effects.h"
#include "../src/messages.h"
#include "../src/player.h"
#include "../src/random.h"
#include "../src/screen.h"
#include "../src/variables.h"
#include "support/terminal.h"

static const int armor_slots[] = {
    Equipment_armor, Equipment_shield, Equipment_cloak, Equipment_gloves,
    Equipment_helm, Equipment_boots, Equipment_belt, Equipment_bracers};
static size_t stat_drawing;

static treasure_type armor_fixture(void) {
  treasure_type item = {0};
  strcpy(item.name, "Test armor");
  strcpy(item.damage, "1d2");
  item.tval = hard_armor;
  item.ac = 7;
  item.toac = 2;
  item.number = 1;
  item.weight = 13;
  item.cost = 29;
  item.subval = 1;
  item.p1 = 3;
  item.tohit = 4;
  item.todam = 5;
  item.level = 6;
  item.identified = 1;
  return item;
}

static void assert_item(treasure_type actual, treasure_type expected) {
  assert(strcmp(actual.name, expected.name) == 0);
  assert(strcmp(actual.damage, expected.damage) == 0);
  assert(actual.tval == expected.tval);
  assert(actual.flags == expected.flags && actual.flags2 == expected.flags2);
  assert(actual.p1 == expected.p1 && actual.cost == expected.cost);
  assert(actual.subval == expected.subval && actual.weight == expected.weight);
  assert(actual.number == expected.number && actual.tohit == expected.tohit);
  assert(actual.todam == expected.todam && actual.ac == expected.ac);
  assert(actual.toac == expected.toac && actual.level == expected.level);
  assert(actual.identified == expected.identified);
}

static void assert_message(size_t index, const char *expected) {
  char buffer[256];
  assert(C_message_capture_get(index, buffer, sizeof(buffer)));
  if (strcmp(buffer, expected) != 0)
    fprintf(stderr, "Message %zu: expected [%s], got [%s]\n", index, expected,
            buffer);
  assert(strcmp(buffer, expected) == 0);
}

static void assert_armor_message(treasure_type before, bool resistant) {
  char expected[160];
  snprintf(expected, sizeof(expected),
           "Your metal scale mail (+4) [%d,%+d] (+3) %s!", before.ac,
           before.toac, resistant ? "resists damage" : "is damaged");
  assert_message(0, expected);
}

static void reset_case(void) {
  assert(inventory_list == NULL);
  memset(equipment, 0, sizeof(equipment));
  memset(&inven_temp, 0, sizeof(inven_temp));
  memset(&blank_treasure, 0, sizeof(blank_treasure));
  memset(&player_flags, 0, sizeof(player_flags));
  inven_ctr = inven_weight = equip_ctr = 0;
  cur_inven = NULL;
  player_lev = 0;
  search_flag = find_flag = death = moria_flag = total_winner = false;
  print_stat = 0;
  died_from[0] = '\0';
  msg_flag = msg_terse = false;
  last_printed_message[0] = '\0';
  C_player_reset_current_hp();
  C_player_modify_current_hp(100 - C_player_current_hp());
  assert(C_player_current_hp() == 100);
  headless_terminal_reset();
  headless_terminal_allow_drawing(true);
  C_message_capture_begin();
}

static void finish_case(size_t messages, size_t drawing) {
  assert(C_message_capture_count() == messages);
  const struct headless_terminal_counts counts = headless_terminal_counts();
  assert(counts.input == 0 && counts.erasure == 0);
  assert(counts.drawing == drawing);
  assert(!death && !moria_flag);
  C_message_capture_end();
}

static void assert_seeded_slot_order(bool corrosion) {
  bool seen[8] = {false};
  size_t covered = 0;
  for (unsigned long long seed = 1; seed <= 4096 && covered < 8; seed++) {
    C_seeded_rng_begin(seed);
    const long choice = randint(8) - 1;
    C_seeded_rng_end();
    assert(choice >= 0 && choice < 8);
    if (seen[choice])
      continue;
    reset_case();
    for (size_t index = 0; index < 8; index++) {
      equipment[armor_slots[index]] = armor_fixture();
      equipment[armor_slots[index]].cost += index;
    }
    equipment[Equipment_primary] = armor_fixture();
    equipment[Equipment_amulet] = armor_fixture();
    equipment[Equipment_right_ring] = armor_fixture();
    equipment[Equipment_left_ring] = armor_fixture();
    equipment[Equipment_light] = armor_fixture();
    equipment[Equipment_secondary] = armor_fixture();
    treasure_type before[EQUIP_MAX];
    memcpy(before, equipment, sizeof(before));
    C_seeded_rng_begin(seed);
    if (corrosion)
      corrode_gas("corrosion test");
    else
      acid_dam(19, "acid test");
    C_seeded_rng_end();
    assert_item(inven_temp.data, before[armor_slots[choice]]);
    before[armor_slots[choice]].toac--;
    for (size_t slot = 0; slot < EQUIP_MAX; slot++)
      assert_item(equipment[slot], before[slot]);
    assert_armor_message(inven_temp.data, false);
    assert(C_player_current_hp() == (corrosion ? 100 : 91));
    assert(print_stat == 1);
    finish_case(1, corrosion ? 0 : 1 + stat_drawing);
    seen[choice] = true;
    covered++;
  }
  assert(covered == 8);
}

static void assert_armor_outcomes(bool corrosion) {
  for (int scenario = 0; scenario < 6; scenario++) {
    reset_case();
    treasure_type before = armor_fixture();
    if (scenario == 0 || scenario == 1) {
      before.flags = Resist_Acid_worn_bit;
      if (scenario == 1) {
        before.ac = 0;
        before.toac = -2;
      }
    } else if (scenario == 2) {
      before.toac = -before.ac;
    } else if (scenario == 3) {
      before.toac = -before.ac - 1;
    } else if (scenario == 4) {
      before.tval = 0;
    } else {
      before.toac = -before.ac + 1;
    }
    equipment[Equipment_boots] = before;
    inven_temp.data = armor_fixture();
    inven_temp.data.cost = 999;
    const treasure_type sentinel = inven_temp.data;
    const bool present = before.tval > 0;
    const bool success = scenario < 2 || scenario == 5;
    C_seeded_rng_begin(73);
    if (present)
      (void)randint(1);
    const long hp_loss = corrosion ? (success ? 0 : randint(8))
                                  : (success ? 19 / 2 : 19);
    C_seeded_rng_end();
    C_seeded_rng_begin(73);
    if (corrosion)
      corrode_gas("corrosion test");
    else
      acid_dam(19, "acid test");
    C_seeded_rng_end();
    assert_item(inven_temp.data, present ? before : sentinel);
    if (success)
      assert_armor_message(before, scenario < 2);
    if (scenario == 5)
      before.toac--;
    assert_item(equipment[Equipment_boots], before);
    assert(C_player_current_hp() == 100 - hp_loss);
    assert(print_stat == 1);
    finish_case(success ? 1 : 0,
                corrosion && success ? 0 : 1 + stat_drawing);
  }
}

static void assert_acid_divisors(void) {
  for (int armor = 0; armor < 2; armor++) {
    for (int resistance = 0; resistance < 2; resistance++) {
      for (long damage = 0; damage <= 19; damage++) {
        reset_case();
        if (armor)
          equipment[Equipment_armor] = armor_fixture();
        if (resistance) {
          equipment[Equipment_right_ring] = armor_fixture();
          equipment[Equipment_right_ring].flags = Resist_Acid_worn_bit;
          player_flags.acid_resist = true;
        }
        const long divisor = armor ? (resistance ? 4 : 2)
                                   : (resistance ? 3 : 1);
        C_seeded_rng_begin(91);
        acid_dam(damage, "acid divisor test");
        C_seeded_rng_end();
        assert(C_player_current_hp() == 100 - damage / divisor);
        assert(player_flags.acid_resist == (bool)resistance);
        assert(print_stat == 1);
        if (armor)
          assert_armor_message(armor_fixture(), false);
        finish_case(armor, 1 + stat_drawing);
      }
    }
  }
}

static unsigned long long inventory_seed(bool corrosion, long roll) {
  for (unsigned long long seed = 1; seed <= 65536; seed++) {
    C_seeded_rng_begin(seed);
    if (corrosion)
      (void)randint(8);
    const long actual = randint(100);
    C_seeded_rng_end();
    if (actual == roll)
      return seed;
  }
  assert(false);
  return 0;
}

static void assert_inventory_damage(bool corrosion) {
  const long threshold = corrosion ? 5 : 3;
  for (int scenario = 0; scenario < 7; scenario++) {
    reset_case();
    treas_rec *item = calloc(1, sizeof(*item));
    assert(item != NULL);
    item->data = armor_fixture();
    item->data.tval = corrosion ? sword : soft_armor;
    if (scenario == 1)
      item->data.number = 2;
    if (scenario == 2)
      item->data.tval = corrosion ? soft_armor : sword;
    if (scenario == 3)
      item->is_in = true;
    if (scenario == 4 || scenario == 5) {
      item->data.flags2 = Holding_bit;
      item->insides = scenario == 4 ? 1 : 0;
    }
    inventory_list = cur_inven = item;
    inven_ctr = 1;
    inven_weight = item->data.number * item->data.weight;
    const long initial_weight = inven_weight;
    const treasure_type before = item->data;
    const bool destroyed = scenario == 0 || scenario == 1 || scenario == 5;
    const unsigned long long seed =
        inventory_seed(corrosion, scenario == 6 ? threshold : threshold - 1);
    C_seeded_rng_begin(seed);
    const long damage = corrosion ? randint(8) : 19;
    C_seeded_rng_end();
    C_seeded_rng_begin(seed);
    if (corrosion)
      corrode_gas("inventory corrosion test");
    else
      acid_dam(19, "inventory acid test");
    C_seeded_rng_end();
    assert(C_player_current_hp() == 100 - damage);
    assert(print_stat == 1);
    if (destroyed) {
      assert_message(0, corrosion
                            ? "There is an acrid smell coming from your pack."
                            : "There is an acrid smell coming from your pack!");
      treasure_type snapshot = before;
      snapshot.number = 1;
      assert_item(inven_temp.data, snapshot);
      assert(inven_weight == initial_weight - before.weight);
      if (scenario == 1) {
        assert(inventory_list == item && cur_inven == item && inven_ctr == 1);
        snapshot = before;
        snapshot.number--;
        assert_item(item->data, snapshot);
      } else {
        assert(inventory_list == NULL && cur_inven == NULL && inven_ctr == 0);
      }
    } else {
      assert(inventory_list == item && cur_inven == item && inven_ctr == 1);
      assert_item(item->data, before);
      assert(inven_weight == initial_weight);
    }
    finish_case(destroyed ? 1 : 0,
                1 + stat_drawing * (destroyed ? 2 : 1));
    free(inventory_list);
    inventory_list = cur_inven = NULL;
  }
}

static void assert_take_hit_boundaries(void) {
  reset_case();
  player_flags.invuln = 1;
  acid_dam(101, "invulnerable test");
  assert(C_player_current_hp() == 100 && died_from[0] == '\0');
  assert(print_stat == 1);
  finish_case(0, 1 + stat_drawing);

  reset_case();
  acid_dam(100, "zero hp test");
  assert(C_player_current_hp() == 0 && died_from[0] == '\0');
  assert(print_stat == 1);
  finish_case(0, 1 + stat_drawing);

  reset_case();
  total_winner = true;
  acid_dam(101, "first fatal test");
  assert(C_player_current_hp() == -1 && death && moria_flag && !total_winner);
  assert(strcmp(died_from, "first fatal test") == 0);
  acid_dam(1, "second fatal test");
  assert(C_player_current_hp() == -2 && death && moria_flag && !total_winner);
  assert(strcmp(died_from, "first fatal test") == 0);
  assert(print_stat == 1 && C_message_capture_count() == 0);
  const struct headless_terminal_counts counts = headless_terminal_counts();
  assert(counts.input == 0 && counts.erasure == 0 && counts.drawing == 2);
  C_message_capture_end();

  reset_case();
  player_flags.invuln = 1;
  C_seeded_rng_begin(17);
  corrode_gas("invulnerable corrosion test");
  C_seeded_rng_end();
  assert(C_player_current_hp() == 100 && print_stat == 1);
  finish_case(0, 1 + stat_drawing);
}

int main(void) {
  debug_file = fopen("target/debug/effects-ffi.log", "w");
  assert(debug_file != NULL);
  for (int repeat = 0; repeat < 2; repeat++) {
    reset_case();
    prt_stat_block();
    stat_drawing = headless_terminal_counts().drawing;
    assert(stat_drawing > 0);
    finish_case(0, stat_drawing);
    assert_seeded_slot_order(false);
    assert_seeded_slot_order(true);
    assert_armor_outcomes(false);
    assert_armor_outcomes(true);
    assert_acid_divisors();
    assert_inventory_damage(false);
    assert_inventory_damage(true);
    assert_take_hit_boundaries();
  }
  assert(fclose(debug_file) == 0);
  debug_file = NULL;
  puts("Effects C acceptance checks passed (SE1 armor/acid/corrosion, F-DMG)");
  return 0;
}