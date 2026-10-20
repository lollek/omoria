#include "../constants.h"
#include "../inventory/inven.h"
#include "../io.h"
#include "../player_action.h"
#include "../screen.h"
#include "../types.h"
#include "../variables.h"

void player_action_quaff_potion(void) {
  long count;
  treas_rec *first;
  treas_rec *item;
  char trash_char;
  bool redraw = false;
  const obj_set stuff_to_drink = {potion1, potion2, 0};

  reset_flag = true;
  if (inven_ctr <= 0) {
    msg_print("But you are not carrying anything.");
    return;
  }
  if (!inventory_find_range(stuff_to_drink, false, &first, &count)) {
    msg_print("You are not carrying any potions.");
    return;
  }
  if (get_item(&item, "Quaff which potion?", &redraw, count, &trash_char,
               false, false)) {
    if (redraw) {
      draw_cave();
    }
    reset_flag = false;
    C_player_action_quaff_selected(item);
  } else if (redraw) {
    draw_cave();
  }
}
