#include "../constants.h"
#include "../inventory/inven.h"
#include "../io.h"
#include "../player_action.h"
#include "../screen.h"
#include "../types.h"
#include "../variables.h"

void player_action_eat(void) {
  reset_flag = true;
  if (inven_ctr <= 0) {
    msg_print("But you are not carrying anything.");
    return;
  }

  const obj_set things_to_eat = {Food, junk_food, 0};
  treas_rec *first;
  long count;
  if (!inventory_find_range(things_to_eat, false, &first, &count)) {
    msg_print("You are not carrying any food.");
    return;
  }

  bool redraw = false;
  treas_rec *item_ptr = NULL;
  char trash_char;
  const bool select_ok =
      get_item(&item_ptr, "Eat what?", &redraw, count, &trash_char, false, false);
  if (redraw) {
    draw_cave();
  }
  if (select_ok) {
    reset_flag = false;
    C_player_action_eat_selected(item_ptr);
  }
}
