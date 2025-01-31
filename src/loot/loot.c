#include "../constants.h"
#include "../pascal.h"
#include "../generate_item/generate_item.h"
#include "../misc.h"
#include "../random.h"
#include "../types.h"
#include "../variables.h"
#include "config.h"
#include "treasures.h"

void place_gold(const long y, const long x) {
  /*{ Places a treasure (Gold or Gems) at given row, column -RAK-	}*/

  long cur_pos;

  popt(&cur_pos);
  long i1 = (2 + randint(dun_level + 4) + randint(dun_level + 4)) / 4;
  if (randint(obj_great) == 1) {
    i1 += randint(dun_level);
  }
  if (i1 > MAX_GOLD) {
    i1 = MAX_GOLD + 1 - randint(randint(3));
  }

  cave[y][x].tptr = cur_pos;
  t_list[cur_pos] = gold_list[i1 - 1];

  if (t_list[cur_pos].tval == valuable_metal) {
    t_list[cur_pos].number =
        randint(t_list[cur_pos].number) + t_list[cur_pos].number / 2;
  }
}

void place_random_dungeon_item(const long y, const long x) {
  long cur_pos;

  popt(&cur_pos);
  cave[y][x].tptr = cur_pos;
  t_list[cur_pos] = generate_item_for_dungeon_level(dun_level);
  magic_treasure(cur_pos, dun_level, false);
}

void place_random_loot_near(const long y, const long x, long max_items_to_place) {
  while (max_items_to_place != 0) {
    for (long i = 0; i <= 10; i++) {
      const long attempted_y = y - 3 + randint(5);
      const long attempted_x = x - 4 + randint(7);

      if (!is_in(cave[attempted_y][attempted_x].fval, floor_set))
        continue;
      if (cave[attempted_y][attempted_x].tptr != 0)
        continue;
      if (randint(100) < 75) {
        place_random_dungeon_item(attempted_y, attempted_x);
      } else {
        place_gold(attempted_y, attempted_x);
      }
      goto placed_item;
    }
  placed_item:
    max_items_to_place--;
  }
}
