#include "equip.h"

#include "../constants.h"
#include "../debug.h"
#include "../io.h"
#include "../model_class.h"
#include "../player.h"
#include "../random.h"
#include "../text_lines.h"
#include "../types.h"
#include "../variables.h"
#include "inven.h"

#include <curses.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h> /* for ftruncate, usleep */

static void ic__wear__gem(treas_rec *gem) {
  treasure_type *const worn_helm = &equipment[Equipment_helm];

  if (worn_helm->tval != gem_helm) {
    msg_print("I don't see how you can use that.");
    msg_print("");
    return;
  }

  if (worn_helm->p1 <= 0) {
    msg_print("There is no more room on the helm.");
    if (randint(2) == 1) {
      msg_print("You lose your grip and the gem "
                "falls to the floor.");
      msg_print("The gem shatters!");
      msg_print("");
      inven_destroy(gem);
    } else {
      msg_print("You catch the gem in mid air");
      msg_print("");
    }
    return;
  }

  msg_print("The gem adheres itself to your helm!");
  py_bonuses(worn_helm, -1);
  long factor;
  if (gem->data.flags2 & Negative_gem_bit) {
    gem->data.flags2 &= 0xFF7FFFFF;
    worn_helm->flags ^= gem->data.flags;
    worn_helm->flags2 ^= gem->data.flags2;
    factor = -1;
  } else {
    worn_helm->flags |= gem->data.flags;
    worn_helm->flags2 |= gem->data.flags2;
    factor = 1;
  }

  worn_helm->cost += factor * gem->data.cost;
  worn_helm->weight += factor * gem->data.weight;
  worn_helm->tohit += factor * gem->data.tohit;
  worn_helm->todam += factor * gem->data.todam;
  worn_helm->ac += factor * gem->data.ac;
  worn_helm->toac += factor * gem->data.toac;
  worn_helm->p1--;
  inven_destroy(gem);
  py_bonuses(worn_helm, 1);
}

static long equipment_slot_from_tval(treas_rec *selected_item) {
  switch (selected_item->data.tval) {
  case lamp_or_torch:
    return Equipment_light;

  case bow_crossbow_or_sling:
  case hafted_weapon:
  case pole_arm:
  case sword:
  case dagger:
  case maul:
  case pick_or_shovel:
    return Equipment_primary;

  case boots:
    return Equipment_boots;

  case gloves_and_gauntlets:
    return Equipment_gloves;

  case cloak:
    return Equipment_cloak;

  case helm:
  case gem_helm:
    return Equipment_helm;

  case shield:
    return Equipment_shield;

  case hard_armor:
  case soft_armor:
    return Equipment_armor;

  case amulet:
    return Equipment_amulet;

  case bracers:
    return Equipment_bracers;

  case belt:
    return Equipment_belt;

  case ring:
    return equipment[Equipment_right_ring].tval == 0 ? Equipment_right_ring
                                                     : Equipment_left_ring;

  case valuable_gems_wear:
    ic__wear__gem(selected_item);
    return Equipment_none;

  default:
    msg_print("I don't see how you can use that.");
    msg_print("");
    return Equipment_none;
  }
}

/**
 *
 * @return true if we can continue unequipping the item, false if we should
 * abort
 */
static bool check_if_previous_worn_item_was_cursed(const long equipment_slot) {
  const bool is_currently_wearing_something =
      equipment[equipment_slot].tval > 0;
  const bool currently_worn_item_is_cursed =
      is_currently_wearing_something &&
      Cursed_worn_bit & equipment[equipment_slot].flags;
  if (!currently_worn_item_is_cursed)
    return true;

  char const *const equip_way =
      equipment_slot == Equipment_primary ? "wielding" : "wearing";
  char out_val_tmp[82];
  inven_temp.data = equipment[equipment_slot];
  objdes(out_val_tmp, &inven_temp, false);
  msg_printf("The %s you are %s appears to be cursed!", out_val_tmp, equip_way);
  msg_print("");
  return false;
}

static void unequip_old_item(const long equipment_slot) {
  const treasure_type previously_worn_item = equipment[equipment_slot];
  if (previously_worn_item.tval > 0) {
    equipment[EQUIP_MAX - 1] = previously_worn_item;
    ic__remove(EQUIP_MAX - 1, true);
  }
}

static void equip_new_item(const long equipment_slot,
                           treas_rec *selected_item) {
  equipment[equipment_slot] = selected_item->data;
  if (equipment_slot == Equipment_light) {
    player_flags.light_on = true;
  }
  equipment[equipment_slot].number = 1;

  /*{ Fix for weight        }*/
  inven_weight +=
      equipment[equipment_slot].weight * equipment[equipment_slot].number;

  /*{ Subtracts weight      }*/
  inven_destroy(selected_item);
  equip_ctr++;
  py_bonuses(&equipment[equipment_slot], 1);

  char prt1[82];
  switch (equipment_slot) {
  case Equipment_primary:
    strcpy(prt1, "You are wielding ");
    break;
  case Equipment_light:
    strcpy(prt1, "Your light source is ");
    break;
  default:
    strcpy(prt1, "You are wearing ");
    break;
  }

  inven_temp.data = equipment[equipment_slot];
  char prt2[82];
  objdes(prt2, &inven_temp, true);
  long i2 = 0;
  long i3 = Equipment_min - 1;
  do { /*{ Get the right letter of equipment }*/
    i3++;
    if (equipment[i3].tval > 0) {
      i2++;
    }
  } while (i3 != equipment_slot);
  msg_printf("%s%s (%c%c", prt1, prt2, (int)i2 + 96, (int)cur_char2());
  msg_print("");
}

void equip_item_screen(long *scr_state, bool *valid_flag) {
  ENTER(("equip_item_screen", "i2"));

  cur_inven = inventory_list;

  while (true) {
    inventory_find_wearables();

    treas_rec *selected_item = inventory_list;
    treas_rec *cur_display[INVEN_DISPLAY_SIZE + 1];
    ic__clear_display(cur_display);
    clear_rc(2, 1);
    bool item_was_selected = ic__show_inven(
        &selected_item, true, false, scr_state, valid_flag,
        "Items a-%%N, space for next page, Esc to exit) Wear/Wield which one?",
        cur_display);
    if (!item_was_selected) {
      break;
    }

    if (!C_class_can_use_item(player_pclass, &selected_item->data)) {
      msg_print("You cannot wear that item type!");
      return;
    }

    /*{ Player turn   }*/
    reset_flag = false;

    /*{ Slot for equipment    }*/
    const long equipment_slot = equipment_slot_from_tval(selected_item);
    if (equipment_slot == Equipment_none) {
      item_was_selected = false;
    }

    if (item_was_selected) {
      item_was_selected =
          check_if_previous_worn_item_was_cursed(equipment_slot);
    }

    if (item_was_selected) {
      unequip_old_item(equipment_slot);
      equip_new_item(equipment_slot, selected_item);
    }

    if (*scr_state == 0 || inven_ctr == 0) {
      break;
    }
  }

  if (*scr_state != 0) {
    prt("You are currently carrying -", 1, 1);
  }

  LEAVE("equip_item_screen", "i2");
}
