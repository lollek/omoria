#include <curses.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "configure.h"
#include "constants.h"
#include "debug.h"
#include "dungeon.h"
#include "magic.h"
#include "pascal.h"
#include "player.h"
#include "routines.h"
#include "term.h"
#include "types.h"
#include "variables.h"
#include "desc.h"

#define MITHRIL_POS (MAX_GOLD)
#define PLATINUM_POS (MAX_GOLD - 1)
#define GOLD_POS (MAX_GOLD - 2)
#define SILVER_POS 4
#define COPPER_POS 3
#define IRON_POS 2

static void aii__insert(treas_rec *ptr, treas_rec *new_item) {
  treas_rec *cur;

  if (ptr == inventory_list) {
    new_item->next = inventory_list;
    inventory_list = new_item;
  } else {
    cur = inventory_list;
    while (cur->next != ptr) {
      cur = cur->next;
    }
    new_item->next = ptr;
    cur->next = new_item;
  }
}

static boolean is_players_spell_book(long typ) {
  if (C_player_uses_magic(M_ARCANE)) {
    return typ == magic_book;
  } else if (C_player_uses_magic(M_SONG)) {
    return typ == song_book;
  } else if (C_player_uses_magic(M_NATURE)) {
    return typ == instrument;
  } else if (C_player_uses_magic(M_DIVINE)) {
    return typ == prayer_book;
  }

  return false;
}

treas_rec *add_inven_item(treasure_type item) {
  /*	{ Add item to inventory_list				-DMF-
   * }*/

  treas_rec *return_value = NULL;

  if (inventory_list == NULL) {
    inventory_list =
        (treas_rec *)safe_malloc(sizeof(treas_rec), "add_inven_item");

    inventory_list->data = item;
    inventory_list->ok = false;
    inventory_list->insides = 0;
    inventory_list->is_in = false;
    inventory_list->next = NULL;
    inven_weight += item.number * item.weight;
    return_value = inventory_list;
    inven_ctr++;

  } else {
    long item_num = item.number;
    long typ = item.tval;
    long subt = item.subval;
    long wgt = item.number * item.weight;
    boolean flag = false;
    boolean duplicate_spell_book = false;
    treas_rec *curse = inventory_list;
    treas_rec *new_item =
        (treas_rec *)safe_malloc(sizeof(treas_rec), "add_inven_item");

    new_item->data = item;
    new_item->ok = false;
    new_item->insides = 0;
    new_item->is_in = false;
    new_item->next = NULL;

    do {
      if (typ == curse->data.tval) {
        if (subt == curse->data.subval) {
          /* */
          /* Items are of the same type and can be
           */
          /* combined */
          /* */
          if (subt > 255) {
            curse->data.number += item_num;
            inven_weight += wgt;
            return_value = curse;
            flag = true;
          }
        } else if (is_players_spell_book(typ)) {
          /* */
          /* Put the users spell books in subt */
          /* order sending */
          /* duplicate books to the end of the */
          /* list. */
          /* */
          if (subt == curse->data.subval) {
            /* */
            /* I don't think this ever */
            /* happens since the subvals */
            /* are probably > 255 so the */
            /* books get stacked. */
            /* */
            duplicate_spell_book = true;
          }

          if (!duplicate_spell_book && (subt < curse->data.subval)) {
            aii__insert(curse, new_item);
            inven_ctr++;
            inven_weight += wgt;
            return_value = new_item;
            flag = true;
          }
        }
      } else if (curse->data.tval < typ) {
        /* */
        /* This puts new items at the end of the list of
         */
        /* their tval type */
        /* */
        aii__insert(curse, new_item);
        inven_ctr++;
        inven_weight += wgt;
        return_value = new_item;
        flag = true;
      }

      /* */
      /* move to next item and skip over any items */
      /* inside a bag of holding */
      /* */
      curse = curse->next;
      if ((curse != NULL) && (curse->is_in)) {
        while ((curse != NULL) && (curse->is_in)) {
          curse = curse->next;
        }
      }
    } while (!(flag || (curse == NULL)));

    if (!flag) {
      curse = inventory_list;
      while (curse->next != NULL) {
        curse = curse->next;
      }
      curse->next = new_item;
      return_value = new_item;
      inven_ctr++;
      inven_weight += wgt;
    }
  } /* endif inventory_list == NULL */

  /*  count = 0; */
  /*  curse = inventory_list; */
  /*  while (curse != NULL) { */
  /*    curse = curse->next; */
  /*    count++; */
  /*  } */
  /*  printf("    END add_inven count: %d  float: %d\n",inven_ctr, count);
   */
  /*  fflush(stdout); */

  return return_value;
}

treas_rec *inven_carry() {
  /*	{ Add the item in inven_temp to players inventory.  Return the
   * }*/
  /*	{ item position for a description if needed...		-RAK-
   * }*/
  return add_inven_item(inven_temp.data);
}

long change_all_ok_stats(boolean nok, boolean nin) {
  treas_rec *ptr;
  long count = 0;

  ENTER(("change_all_ok_stats", "%d, %d", nok, nin));

  for (ptr = inventory_list; ptr != NULL; ptr = ptr->next) {
    if (ptr->is_in) {
      ptr->ok = nin;
    } else {
      ptr->ok = nok;
    }

    if (ptr->ok) {
      count++;
    }
  }

  RETURN("change_all_ok_stats", "u", 'd', "count", &count);
  return count;
}

void ic__clear_display(treas_rec *cur_display[], long *cur_display_size) {
  long index;

  ENTER(("ic__clear_display", "iu"));

  cur_display_size = 0;
  (void)cur_display_size;
  for (index = 1; index <= DISPLAY_SIZE; index++) {
    cur_display[index] = 0;
  }

  LEAVE("ic__clear_display", "iu");
}

long ic__display_inv(treas_rec *cur_display[], char prompt[82],
                     treas_rec *start, treas_rec **next_start) {
  /*{ start changes into start of next page; returns # items in page}*/

  long count;
  long i1;
  char out_val[82];
  char out_val2[120];
  char out_val3[82];
  char *t;

  for (count = 0; start != NULL && count < DISPLAY_SIZE;) {
    if (start->ok) {
      count++;
      if (cur_display[count] != start) {
        cur_display[count] = start;
        inven_temp.data = start->data;
        objdes(out_val, &inven_temp, true);
        if ((start->data.flags2 & Holding_bit) != 0) {
          if (strstr(start->data.name, "|") == NULL) {
            bag_descrip(start, out_val3);
            strcat(out_val, out_val3);
          }
        }
        if (start->is_in) {
          sprintf(out_val2, "%c%c%c%s%s", cur_insure(), (96 + (int)count),
                  cur_char1(), "     ", out_val);
        } else {
          sprintf(out_val2, "%c%c%c%s%s", cur_insure(), (96 + (int)count),
                  cur_char1(), " ", out_val);
        }
        prt(out_val2, count + 1, 1);
      }
    }
    start = start->next;
  } /* end for */

  for (i1 = count + 1; i1 <= DISPLAY_SIZE; i1++) {
    erase_line(i1 + 1, 1);
    cur_display[i1] = NULL;
  }

  if (start == NULL) {
    *next_start = inventory_list;
  } else {
    *next_start = start;
  }

  if (count > 0) {
    strcpy(out_val, prompt);
    sprintf(out_val2, "%c", ((int)count + 96));
    insert_str(out_val, "%N", out_val2);
  } else if ((t = strstr(prompt, "%N")) != NULL) {
    sprintf(out_val, "No items found%s", t + 2);
  } else {
    sprintf(out_val, "No items found. %s", prompt);
  }
  prt(out_val, 1, 1);

  return count;
}

boolean ic__show_inven(treas_rec **ret_ptr, boolean want_back,
                       boolean clean_flag, long *scr_state, boolean *valid_flag,
                       char prompt[82], treas_rec *cur_display[],
                       long *cur_display_size) {
  /*  { Displays inventory items, returns chosen item if want_back. }*/
  /*{ boolean returns if chosen }*/

  long num_choices;
  boolean exit_flag = false;
  treas_rec *next_inven;
  long count;
  long count2;
  char out_str[82];
  obj_set stuff_to_fire = {sling_ammo, bolt, arrow, 0};
  boolean return_value = false;
  long wgt = 0;

  ENTER(("ic__show_inven", "iu"));

  num_choices = ic__display_inv(cur_display, prompt, cur_inven, &next_inven);

  while (!exit_flag) {
    boolean caps_flag = false;
    char command;
    if (!get_com("", &command)) {
      *valid_flag = false;
      exit_flag = true;
      continue;
    }

    if (command == 22 || command == 32) {
      if (cur_inven == next_inven) {
        prt("Entire inventory displayed.", 1, 1);
        num_choices = 0;
      } else {
        cur_inven = next_inven;
        num_choices =
            ic__display_inv(cur_display, prompt, cur_inven, &next_inven);
      }
      continue;
    }

    if (command == 3 || command == 25 || command == 26 || command == 27) {
      *valid_flag = false;
      exit_flag = true;
      continue;
    }

    if ('1' <= command && command <= '9') {
      cur_inven = inventory_list;
      count = 0;
      if (!(cur_inven->next == NULL || count >= (command - '1') * 20)) {
        do {
          count++;
          cur_inven = cur_inven->next;
          if (cur_inven->next == NULL) {
            exit_flag = true;
          }
        } while (!(cur_inven->next == NULL || count >= (command - '1') * 20));
      }

      if (cur_inven->next == NULL && count != (command - '1') * 20) {
        prt("Entire inventory displayed.", 1, 1);
        cur_inven = inventory_list;
      } else {
        next_inven = cur_inven;
        num_choices =
            ic__display_inv(cur_display, prompt, cur_inven, &next_inven);
      }
      sprintf(out_str, ": %c", (int)command);
      prt(out_str, 1, 75);
      continue;
    }

    if (!want_back) {
      *valid_flag = false;
      exit_flag = true;
      continue;
    }

    if (clean_flag) {
      if (command <= 'Z' && command >= 'A') {
        caps_flag = true;
      }
      command -= caps_flag ? 64 : 96;
    } else {
      command -= 96;
    }

    if (command < 1 || command > num_choices) {
      prt("Invalid selection.", 1, 1);
      *valid_flag = false;
      exit_flag = true;
      continue;
    }

    *ret_ptr = cur_display[(int)command];
    if (!clean_flag) {
      cur_display[(int)command] = NULL;
      exit_flag = true;
      return_value = true;
      continue;
    }

    wgt = 0;
    if ((*ret_ptr)->data.flags2 & Holding_bit) {
      treas_rec *temp_ptr = (*ret_ptr)->next;
      while ((temp_ptr != NULL) && (temp_ptr->is_in)) {
        wgt += temp_ptr->data.weight * temp_ptr->data.number;
        temp_ptr = temp_ptr->next;
      }
    }

    if (!((*ret_ptr)->is_in) && wgt == 0) {
      if (caps_flag && !(is_in((*ret_ptr)->data.tval, stuff_to_fire))) {
        count = (*ret_ptr)->data.number;
      } else {
        count = 1;
      }
      for (count2 = 1; count2 <= count; count2++) {
        inven_destroy(*ret_ptr);
      }
      ic__clear_display(cur_display, cur_display_size);
      num_choices =
          ic__display_inv(cur_display, prompt, cur_inven, &next_inven);
    } else {
      msg_print("You must empty the bag of holding first.");
    }

    if (num_choices == 0) {
      prt("No items in inventory.", 1, 1);
      *valid_flag = false;
      exit_flag = true;
    }
  }

  *scr_state = 1;

  RETURN("ic__show_inven", "iu", 'b', "picked something", &return_value);
  return return_value;
}

/**
 * -OK-
 *  ic__equip_print_prefix() - Returns prefix for printing equipment
 *  @position: Which equipment slot to prefix
 */
static char const *ic__equip_print_prefix(long position) {
  switch (position) {
  case Equipment_primary:
    return " You are wielding   : ";
    break;
  case Equipment_helm:
    return " Worn on head       : ";
    break;
  case Equipment_amulet:
    return " Worn around neck   : ";
    break;
  case Equipment_armor:
    return " Worn on body       : ";
    break;
  case Equipment_belt:
    return " Worn at waist      : ";
    break;
  case Equipment_shield:
    return " Worn on arm        : ";
    break;
  case Equipment_gloves:
    return " Worn on hands      : ";
    break;
  case Equipment_bracers:
    return " Worn on wrists     : ";
    break;
  case Equipment_right_ring:
    return " Worn on right hand : ";
    break;
  case Equipment_left_ring:
    return " Worn on left hand  : ";
    break;
  case Equipment_boots:
    return " Worn on feet       : ";
    break;
  case Equipment_cloak:
    return " Worn about body    : ";
    break;
  case Equipment_light:
    if ((player_flags).light_on && (equipment[position].p1 > 0)) {
      return " Light source (On)  : ";
    } else {
      return " Light source (Off) : ";
    }
    break;
  case Equipment_secondary:
    return " Secondary weapon   : ";
    break;
  default:
    return " Unknown value      : ";
    break;
  }
}

void inv__equip_pos_string(char out_val[82], long equip_pos, long counter) {
  char tmp_buf[82];

  inven_temp.data = equipment[equip_pos];
  objdes(tmp_buf, &inven_temp, true);
  snprintf(out_val, 82, "%c%c%c%s%s", cur_insure(), (char)('a' + counter - 1),
          cur_char2(), ic__equip_print_prefix(equip_pos), tmp_buf);
}

void ic__show_equip(long *scr_state, long r1) {
  /*{ Displays equipment items from r1 to end       -RAK-   }*/

  if (r1 > equip_ctr) { /*{ Last item gone                }*/
    prt("", equip_ctr + 3, 1);
  } else if (r1 > 0) { /*{ R1 = 0 dummy call             }*/
    prt_equipment_args(2, 1, r1, true);
    *scr_state = 2; /*{ Set state of screen   }*/
  }
}

treas_rec *ic__remove(long item_val, boolean show_message) {
  /*{ Remove item from equipment list               -RAK-   }*/

  long typ;
  char out_val[200];
  char prt1[82];
  char prt2[82];

  prt1[0] = 0;

  typ = equipment[item_val].tval;
  inven_temp.data = equipment[item_val];
  add_inven_item(equipment[item_val]);
  inven_weight -= inven_temp.data.number * inven_temp.data.weight;
  equipment[item_val] = blank_treasure;
  equip_ctr--;

  if (show_message) {
    switch (typ) {
    case sling_ammo:
    case bolt:
    case arrow:
    case bow_crossbow_or_sling:
    case hafted_weapon:
    case pole_arm:
    case sword:
    case dagger:
    case maul:
    case pick_or_shovel:
      strcat(prt1, "Was wielding ");
      break;

    case lamp_or_torch:
      strcat(prt1, "Light source was ");
      break;

    default:
      strcat(prt1, "Was wearing ");
      break;
    }

    objdes(prt2, &inven_temp, true);
    sprintf(out_val, "%s%s", prt1, prt2);
    msg_print(out_val);
  }

  if (item_val != Equipment_secondary) { /* Secondary weapon already off */
    py_bonuses(&(inven_temp.data), -1);
  }

  return &inven_temp;
}

void ic__unwear(long *scr_state) {
  /*{ Unwear routine, remove a piece of equipment   -RAK-   }*/

  long i1, i2, l_command;
  boolean exit_flag, test_flag;
  char command;
  char out_val[82];

  if (*scr_state == 1) {
    clear_rc(1, 1);
    ic__show_equip(scr_state, 1);
  }

  exit_flag = false;
  do {
    sprintf(out_val,
            "(a-%c, *,<space> for equipment list, Esc to "
            "exit) Take off which one?",
            ((int)equip_ctr + 96));
    test_flag = false;
    msg_print(out_val);
    do {
      command = inkey();
      l_command = (long)(command);
      switch (l_command) {
      case 0:
      case 3:
      case 25:
      case 26:
      case 27:
        test_flag = true;
        exit_flag = true;
        break;

      case 42:
      case 32:
        clear_rc(2, 1);
        ic__show_equip(scr_state, 1);
        break;

      default:
        l_command -= 96;
        if ((l_command >= 1) && (l_command <= equip_ctr)) {
          test_flag = true;
        }
        break;
      }
    } while (!test_flag);

    if (!exit_flag) {
      reset_flag = false; /*{ Player turn   }*/
      i1 = 0;
      i2 = Equipment_min - 1;

      do {
        i2++;
        if (equipment[i2].tval > 0) {
          i1++;
        }
      } while (i1 != l_command);

      if ((Cursed_worn_bit & equipment[i2].flags) != 0) {
        msg_print("Hmmm, it seems to be cursed...");
        l_command = 0;
      } else {
        ic__remove(i2, true);
      }
    }

    if (*scr_state == 0) {
      exit_flag = true;
    } else if (equip_ctr == 0) {
      exit_flag = true;
    } else if (inven_ctr >= Equipment_min - 1) {
      ic__show_equip(scr_state, l_command);
      exit_flag = true;
    } else if (!exit_flag) {
      ic__show_equip(scr_state, l_command);
    }
  } while (!exit_flag);

  if (*scr_state != 0) {
    if (equip_ctr == 0) {
      clear_rc(1, 1);
    } else {
      prt("You are currently using -", 1, 1);
    }
  }
}

static void ic__wear__gem(treas_rec *gem) {
  long factor;
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

void ic__wear(treas_rec *cur_display[], long *cur_display_size, char prompt[82],
              long *scr_state, boolean *valid_flag) {
  /*{ Wear routine, wear or wield an item           -RAK-   }*/
  boolean exit_flag = false;

  ENTER(("ic__wear", "i2"));

  cur_inven = inventory_list;

  while (!exit_flag) {
    long i1;
    boolean equip_cursed_item;
    char out_val[200];
    treas_rec *selected_item;
    boolean item_was_selected;

    ic__clear_display(cur_display, cur_display_size);
    change_all_ok_stats(true, false);

    { /* Filter item types before we show the list */
      const obj_set wearables = {valuable_gems_wear,
                                 lamp_or_torch,
                                 bow_crossbow_or_sling,
                                 hafted_weapon,
                                 pole_arm,
                                 dagger,
                                 sword,
                                 pick_or_shovel,
                                 maul,
                                 gem_helm,
                                 boots,
                                 gloves_and_gauntlets,
                                 cloak,
                                 helm,
                                 shield,
                                 hard_armor,
                                 soft_armor,
                                 bracers,
                                 belt,
                                 amulet,
                                 ring,
                                 0,
                                 0,
                                 0,
                                 0};
      treas_rec *ptr;
      long count;
      find_range(wearables, false, &ptr, &count);
    }

    sprintf(prompt, "Items a-%%N, space for next page, Esc to "
                    "exit) Wear/Wield which one?");
    clear_rc(2, 1);

    selected_item = inventory_list;
    item_was_selected =
        ic__show_inven(&selected_item, true, false, scr_state, valid_flag,
                       prompt, cur_display, cur_display_size);
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
    switch (selected_item->data.tval) {
    case lamp_or_torch:
      i1 = Equipment_light;
      break;

    case bow_crossbow_or_sling:
    case hafted_weapon:
    case pole_arm:
    case sword:
    case dagger:
    case maul:
    case pick_or_shovel:
      i1 = Equipment_primary;
      break;

    case boots:
      i1 = Equipment_boots;
      break;

    case gloves_and_gauntlets:
      i1 = Equipment_gloves;
      break;

    case cloak:
      i1 = Equipment_cloak;
      break;

    case helm:
    case gem_helm:
      i1 = Equipment_helm;
      break;

    case shield:
      i1 = Equipment_shield;
      break;

    case hard_armor:
    case soft_armor:
      i1 = Equipment_armor;
      break;

    case amulet:
      i1 = Equipment_amulet;
      break;

    case bracers:
      i1 = Equipment_bracers;
      break;

    case belt:
      i1 = Equipment_belt;
      break;

    case ring:
      i1 = equipment[Equipment_right_ring].tval == 0 ? Equipment_right_ring
                                                     : Equipment_left_ring;
      break;

    case valuable_gems_wear:
      ic__wear__gem(selected_item);
      item_was_selected = false;
      break;

    default:
      msg_print("I don't see how you can use that.");
      msg_print("");
      item_was_selected = false;
      i1 = 0;
      break;
    } /* end switch */

    equip_cursed_item = item_was_selected && equipment[i1].tval > 0 &&
                        Cursed_worn_bit & equipment[i1].flags;
    if (equip_cursed_item) {
      char const *const equip_way =
          i1 == Equipment_primary ? "wielding" : "wearing";
      char out_val_tmp[82];
      inven_temp.data = equipment[i1];
      objdes(out_val, &inven_temp, false);
      strcpy(out_val_tmp, out_val);
      sprintf(out_val, "The %s you are %s appears to be cursed", out_val_tmp,
              equip_way);
      item_was_selected = false;
    }

    if (item_was_selected) {
      long i2;
      long i3;
      char prt1[82];
      char prt2[82];
      treasure_type unwear_obj = equipment[i1];
      equipment[i1] = selected_item->data;
      if (i1 == Equipment_light) {
        (player_flags).light_on = true;
      }
      equipment[i1].number = 1;

      /*{ Fix for weight        }*/
      inven_weight += equipment[i1].weight * equipment[i1].number;

      /*{ Subtracts weight      }*/
      inven_destroy(selected_item);
      equip_ctr++;
      py_bonuses(&(equipment[i1]), 1);
      if (unwear_obj.tval > 0) {
        equipment[EQUIP_MAX - 1] = unwear_obj;
        ic__remove(EQUIP_MAX - 1, true);
      }

      switch (i1) {
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
      inven_temp.data = equipment[i1];
      objdes(prt2, &inven_temp, true);
      i2 = 0;
      i3 = Equipment_min - 1;
      do { /*{ Get the right letter of equipment }*/
        i3++;
        if (equipment[i3].tval > 0) {
          i2++;
        }
      } while (!(i3 == i1));
      sprintf(out_val, "%s%s (%c%c", prt1, prt2, (int)i2 + 96,
              (int)cur_char2());
      msg_print(out_val);
    }

    if (*scr_state == 0) {
      exit_flag = true;
    } else if (inven_ctr == 0) {
      exit_flag = true;
    }
  }

  if (*scr_state != 0) {
    prt("You are currently carrying -", 1, 1);
  }

  LEAVE("ic__wear", "i2");
}

void ic__stats(treas_rec *cur_display[], long *cur_display_size,
               char prompt[82], long *scr_state, boolean *valid_flag) {
  /*{ Statistics routine, get wizard info on an item        -DMF-   }*/

  char out_val[82];
  treas_rec *item_ptr;
  boolean exit_flag;

  ENTER(("ic__stats", "i2"));

  do {
    sprintf(prompt, "(Items a-%%N, space for next page, Esc to "
                    "exit) Statistics on which one ?");
    clear_rc(1, 1);
    item_ptr = NULL;
    change_all_ok_stats(true, true);
    ic__clear_display(cur_display, cur_display_size);
    exit_flag = !(ic__show_inven(&item_ptr, true, false, scr_state, valid_flag,
                                 prompt, cur_display, cur_display_size));
    if (item_ptr != NULL) {
      clear_rc(1, 1);
      prt("Name        : ", 1, 1);
      prt("Description : ", 2, 1);
      prt("Value       : ", 3, 1);
      prt("Type        : ", 4, 1);
      prt("Character   : ", 5, 1);
      prt("Flags       : ", 6, 1);
      prt("Flags2      : ", 7, 1);
      prt("P1          : ", 8, 1);
      prt("Cost        : ", 9, 1);
      prt("Subval      : ", 10, 1);
      prt("Weight      : ", 11, 1);
      prt("Number      : ", 12, 1);
      prt("+ To hit    : ", 13, 1);
      prt("+ To Damage : ", 14, 1);
      prt("AC          : ", 15, 1);
      prt("+ To AC     : ", 16, 1);
      prt("Damage      : ", 17, 1);
      prt("Level       : ", 18, 1);
      prt("Blackmarket : ", 19, 1);
      prt("Insured     : ", 20, 1);
      prt(item_ptr->data.name, 1, 15);
      objdes(out_val, item_ptr, true);
      prt(out_val, 2, 15);
      sprintf(out_val, "%ld", (long)item_ptr->data.tval);
      prt(out_val, 3, 15);

      switch (item_ptr->data.tval) {
      case miscellaneous_object:
        strcpy(out_val, "Miscellaneous object");
        break;
      case chest:
        strcpy(out_val, "Chest");
        break;
      case misc_usable:
        strcpy(out_val, "Miscellaneous usable");
        break;
      case valuable_jewelry:
        strcpy(out_val, "Jewelry");
        break;
      case valuable_gems:
        strcpy(out_val, "Gem");
        break;
      case bag_or_sack:
        strcpy(out_val, "Bag or Sack");
        break;
      case valuable_gems_wear:
        strcpy(out_val, "Wearable Gem");
        break;
      case sling_ammo:
        strcpy(out_val, "Sling ammo");
        break;
      case bolt:
        strcpy(out_val, "Bolt");
        break;
      case arrow:
        strcpy(out_val, "Arrow");
        break;
      case spike:
        strcpy(out_val, "Spike");
        break;
      case lamp_or_torch:
        strcpy(out_val, "Lamp or torch");
        break;
      case bow_crossbow_or_sling:
        strcpy(out_val, "Ranged weapon");
        break;
      case hafted_weapon:
        strcpy(out_val, "Hafted weapon");
        break;
      case pole_arm:
        strcpy(out_val, "Pole arm");
        break;
      case sword:
        strcpy(out_val, "Sword");
        break;
      case dagger:
        strcpy(out_val, "Light Weapon");
        break;
      case maul:
        strcpy(out_val, "Blunt Weapon");
        break;
      case pick_or_shovel:
        strcpy(out_val, "Pick or shovel");
        break;
      case gem_helm:
        strcpy(out_val, "Gem Helm");
        break;
      case boots:
        strcpy(out_val, "Boots");
        break;
      case gloves_and_gauntlets:
        strcpy(out_val, "Gloves or gauntlets");
        break;
      case cloak:
        strcpy(out_val, "Cloak");
        break;
      case helm:
        strcpy(out_val, "Helm");
        break;
      case shield:
        strcpy(out_val, "Shield");
        break;
      case hard_armor:
        strcpy(out_val, "Hard armor");
        break;
      case soft_armor:
        strcpy(out_val, "Soft armor");
        break;
      case bracers:
        strcpy(out_val, "Bracers");
        break;
      case belt:
        strcpy(out_val, "Belt");
        break;
      case amulet:
        strcpy(out_val, "Amulet");
        break;
      case ring:
        strcpy(out_val, "Ring");
        break;
      case staff:
        strcpy(out_val, "Staff");
        break;
      case rod:
        strcpy(out_val, "Rod");
        break;
      case wand:
        strcpy(out_val, "Wand");
        break;
      case scroll1:
      case scroll2:
        strcpy(out_val, "Scroll");
        break;
      case potion1:
      case potion2:
        strcpy(out_val, "Potion");
        break;
      case flask_of_oil:
        strcpy(out_val, "Flask of oil");
        break;
      case Food:
        strcpy(out_val, "Food");
        break;
      case junk_food:
        strcpy(out_val, "Junk Food");
        break;
      case chime:
        strcpy(out_val, "Chime");
        break;
      case horn:
        strcpy(out_val, "Horn");
        break;
      case magic_book:
        strcpy(out_val, "Magic book");
        break;
      case prayer_book:
        strcpy(out_val, "Prayer book");
        break;
      case instrument:
        strcpy(out_val, "Instrument");
        break;
      case song_book:
        strcpy(out_val, "Song book");
        break;
      default:
        sprintf(out_val, "Unknown item type: %d", item_ptr->data.tval);
        break;
      }

      prt(out_val, 4, 15);
      sprintf(out_val, "'%u'", C_item_get_tchar(&item_ptr->data));
      prt(out_val, 5, 15);
      print_hex_value((item_ptr->data.flags), 6, 15);
      print_hex_value((item_ptr->data.flags2), 7, 15);
      sprintf(out_val, "%ld", item_ptr->data.p1);
      prt(out_val, 8, 15);
      sprintf(out_val, "%ld iron  (%ld gold)", item_ptr->data.cost,
              item_ptr->data.cost / GOLD_VALUE);
      prt(out_val, 9, 15);
      sprintf(out_val, "%ld", item_ptr->data.subval);
      prt(out_val, 10, 15);
      if (item_ptr->data.weight < 100) {
        sprintf(out_val, "%ld small", (long)item_ptr->data.weight);
      } else {
        sprintf(out_val, "%ld large", (long)(item_ptr->data.weight / 100));
      }
      prt(out_val, 11, 15);
      sprintf(out_val, "%ld", (long)item_ptr->data.number);
      prt(out_val, 12, 15);
      sprintf(out_val, "%ld", (long)item_ptr->data.tohit);
      prt(out_val, 13, 15);
      sprintf(out_val, "%ld", (long)item_ptr->data.todam);
      prt(out_val, 14, 15);
      sprintf(out_val, "%ld", (long)item_ptr->data.ac);
      prt(out_val, 15, 15);
      sprintf(out_val, "%ld", (long)item_ptr->data.toac);
      prt(out_val, 16, 15);
      prt(item_ptr->data.damage, 17, 15);
      sprintf(out_val, "%ld", (long)item_ptr->data.level);
      prt(out_val, 18, 15);
      sprintf(out_val, "%s",
              ((item_ptr->data.flags2 & Blackmarket_bit) != 0) ? "true"
                                                               : "false");
      prt(out_val, 19, 15);
      sprintf(out_val, "%s",
              ((item_ptr->data.flags2 & Insured_bit) != 0) ? "true" : "false");
      prt(out_val, 20, 15);
      prt("Hit any key to continue", 22, 29);
      inkey();
    }
  } while (!exit_flag);

  LEAVE("ic__stats", "i2");
}

void ic__show_money() {
  /*{ Show players money                            -DMF-   }*/

  char prt1[82];

  clear_rc(1, 1);

  prt("You are carrying -", 1, 1);
  sprintf(prt1, "Mithril  : %10ld", player_money[MITHRIL]);
  prt(prt1, 3, 10);
  sprintf(prt1, "Platinum : %10ld", player_money[PLATINUM]);
  prt(prt1, 4, 10);
  sprintf(prt1, "Gold     : %10ld", player_money[GOLD]);
  prt(prt1, 5, 10);
  sprintf(prt1, "Silver   : %10ld", player_money[SILVER]);
  prt(prt1, 6, 10);
  sprintf(prt1, "Copper   : %10ld", player_money[COPPER]);
  prt(prt1, 7, 10);
  sprintf(prt1, "Iron     : %10ld", player_money[IRON]);
  prt(prt1, 8, 10);
  sprintf(prt1, "Total    : %10ld", player_money[TOTAL_]);
  prt(prt1, 10, 10);
}

void ic__destroy_bag(treas_rec *bag) {
  while ((bag->next != NULL) && (bag->next->is_in)) {
    /* this seems odd, wasn't it already subtracted from   XXXX
       inven_weight when it went into the bag?  */
    /* inven_weight -= (bag->next->data.number * */
    /* bag->next->data.weight); */
    delete_inven_item(bag->next);
  }
  inven_weight -= (bag->data.number * bag->data.weight);
  delete_inven_item(bag);
}

void ic__put_inside() {
  /*        { Put an item inside of another item            -DMF-   }*/

  treas_rec *put_ptr;
  boolean redraw;
  boolean blooey = false;
  char trash_char;

  change_all_ok_stats(true, true);

  if (get_item(&put_ptr, "Put which item?", &redraw, inven_ctr, &trash_char,
               false, true)) {
    treas_rec *temp_ptr;
    long count = 0;
    change_all_ok_stats(false, false);
    temp_ptr = inventory_list;

    while (temp_ptr != NULL) {
      if ((temp_ptr->data.flags2 & Holding_bit) != 0) {
        temp_ptr->ok = true;
        count++;
      }
      temp_ptr = temp_ptr->next;
    }

    if (count == 0) {
      msg_print("You have nothing to put it into.");
    } else {
      treas_rec *into_ptr;
      clear_rc(2, 1);
      if (get_item(&into_ptr, "Into which item?", &redraw, inven_ctr,
                   &trash_char, false, true)) {
        if (into_ptr == put_ptr) {
          msg_print("You can't seem to fit it "
                    "inside itself.");
        } else if ((put_ptr->data.flags2 & Holding_bit) != 0) {
          msg_print("Uh oh, now you've done it!");
          msg_print("You lose the items in both bags!");
          ic__destroy_bag(put_ptr);
          ic__destroy_bag(into_ptr);
        } else {
          long wgt;
          treas_rec *curse;
          player_flags.paralysis++;
          reset_flag = false;

          if (put_ptr == inventory_list) {
            inventory_list = put_ptr->next;
          } else {
            curse = inventory_list;
            while (curse->next != put_ptr) {
              curse = curse->next;
            }
            curse->next = put_ptr->next;
          }

          curse = inventory_list;
          while (curse != into_ptr) {
            curse = curse->next;
          }

          put_ptr->next = curse->next;
          curse->next = put_ptr;
          put_ptr->is_in = true;
          (into_ptr->insides)++;

          inven_weight -= (put_ptr->data.weight * put_ptr->data.number);
          msg_print("You stuff it inside");

          if ((put_ptr->data.flags2 & Sharp_bit) != 0) {
            msg_print("You poke a hole in "
                      "the bag!");
            blooey = true;
          }

          wgt = 0;
          temp_ptr = into_ptr->next;
          while ((temp_ptr != NULL) && (temp_ptr->is_in)) {
            wgt += (temp_ptr->data.weight * temp_ptr->data.number);
            temp_ptr = temp_ptr->next;
          }

          if (!blooey && (wgt > into_ptr->data.p1)) {
            msg_print("The sides of the "
                      "bag swell and "
                      "burst!");
            blooey = true;
          }

          if (blooey) {
            ic__destroy_bag(into_ptr);
          }

        } /* end if (have two legal items) */
      }   /* end if (get_item to put into) */
    }     /* end if (count != 0) */
  }       /* end if (get_item to put in bag) */

  cur_inven = inventory_list;
}

void ic__take_out() {
  /*{ Take an item out of another item              -DMF-   }*/

  treas_rec *from_ptr;
  treas_rec *temp_ptr;
  treas_rec *curse;
  boolean redraw;
  boolean flag;
  char trash_char;
  long count = change_all_ok_stats(false, true);

  if (count <= 0) {
    msg_print("You have nothing to remove.");
    return;
  }

  if (get_item(&from_ptr, "Remove which item?", &redraw, count, &trash_char,
               false, true)) {
    player_flags.paralysis += 2;
    reset_flag = false;
    temp_ptr = inventory_list;

    while ((temp_ptr != NULL) && (temp_ptr != from_ptr)) {
      if ((temp_ptr->data.flags2 & Holding_bit) != 0) {
        curse = temp_ptr;
      }
      temp_ptr = temp_ptr->next;
    }

    if ((curse->data.flags2 & Swallowing_bit) != 0) {
      /* bag of devouring */
      flag = (randint(100) < 6);
    } else {
      /* bag of holding */
      flag = true;
    }

    if (flag) {
      (curse->insides)--;
      curse = inventory_list;
      while (curse->next != from_ptr) {
        curse = curse->next;
      }
      curse->next = from_ptr->next;
      inven_temp.data = from_ptr->data;
      inven_carry(); /* XXXX is this a memory leak? */
      /*{change to next line by Dean; used to begin
        with
        if (inven_ctr=old_ctr) then}*/
      inven_ctr--;
      msg_print("You remove the item");

    } else {
      msg_print("You make several attempts, but "
                "cannot seem to get a grip on it.");
      cur_inven = inventory_list;
    }
  }
}

void ic__selective_inven(long *scr_state, boolean *valid_flag, char prompt[82],
                         treas_rec *cur_display[], long *cur_display_size) {
  /*{ Inventory of selective items, picked by character     -DMF-   }*/

  treas_rec *ptr;
  char out[134], out_str[134];
  boolean exit_flag = false;
  char command;
  char *out_pos;

  ptr = inventory_list;
  out_pos = &(out[sizeof(out)]);
  *(--out_pos) = 0;

  while (ptr != NULL) {
    if (strchr(out_pos, (char)C_item_get_tchar(&ptr->data)) == NULL) {
      *(--out_pos) = (char)C_item_get_tchar(&ptr->data);
    }
    ptr = ptr->next;
  }

  do {
    sprintf(out_str, "What type of items to inventory? (%s) ", out_pos);
    prt(out_str, 1, 1);
    if (!(get_com("", &command))) {
      exit_flag = true;
    }
  } while (!(exit_flag || (pindex(out_pos, command) != 0)));

  if (!exit_flag) {
    change_all_ok_stats(false, false);
    ptr = inventory_list;

    while (ptr != NULL) {
      if ((char)C_item_get_tchar(&ptr->data) == command) {
        ptr->ok = true;
      }
      ptr = ptr->next;
    }

    ic__clear_display(cur_display, cur_display_size);
    clear_rc(1, 1);
    strcpy(prompt, "You are currently carrying: space for next page");
    ic__show_inven(&ptr, false, false, scr_state, valid_flag, prompt,
                   cur_display, cur_display_size);
  }
}

void ic__switch_weapon(long *scr_state) {
  /*{ Switch primary and secondary weapons          -RAK-   }*/

  if ((Cursed_worn_bit & equipment[Equipment_primary].flags) != 0) {
    char prt1[82];
    char prt2[200];
    inven_temp.data = equipment[Equipment_primary];
    objdes(prt1, &inven_temp, false);
    sprintf(prt2, "The %s you are wielding appears to be cursed.", prt1);
    msg_print(prt2);
  } else {
    treasure_type tmp_obj;
    /*{ Switch weapons        }*/
    reset_flag = false;
    tmp_obj = equipment[Equipment_secondary];
    equipment[Equipment_secondary] = equipment[Equipment_primary];
    equipment[Equipment_primary] = tmp_obj;

    /*{ Subtract bonuses      }*/
    py_bonuses(&(equipment[Equipment_secondary]), -1);

    /*{ Add bonuses           }*/
    py_bonuses(&(equipment[Equipment_primary]), 1);

    msg_print("Swapped main hand with backup");
  }

  if (*scr_state != 0) {
    msg_print("");
    clear_rc(1, 1);
    prt("You are currently using -", 1, 1);
    ic__show_equip(scr_state, 1);
  }
}

boolean inven_command(char command, treas_rec **item_ptr, char sprompt[82]) {
  /* Comprehensive function block to handle all inventory      -RAK-
   * and equipment routines.  Five kinds of calls can take place.
   * Note that '?' is a special call for other routines to display
   * only a portion of the inventory, and take no other action.
   */

  long scr_state = 0;
  boolean exit_flag, test_flag;
  treas_rec *cur_display[DISPLAY_SIZE + 1];
  long cur_display_size;
  boolean valid_flag = false;
  char prompt[82];
  boolean return_value = false;

  ENTER(("inven_command", "i"));

  strcpy(prompt, sprompt); /* prompt gets modified from time to time,
                              constants get passed to inven_command... */
  exit_flag = false;
  scr_state = 0;
  cur_inven = inventory_list;

  do {
    switch (command) {

    case 'i': /*{ Inventory     }*/
      if (inven_ctr == 0) {
        msg_print("You are not carrying anything.");
      } else {
        clear_rc(1, 1);
        strcpy(prompt, "You are currently carrying: "
                       "space for next page");
        ic__clear_display(cur_display, &cur_display_size);
        change_all_ok_stats(true, true);
        ic__show_inven(item_ptr, false, false, &scr_state, &valid_flag, prompt,
                       cur_display, &cur_display_size);
      }
      break;

    case 'c':
      if (inven_ctr == 0) {
        msg_print("You are not carrying anything.");
      } else {
        clear_rc(1, 1);
        strcpy(prompt, "Warning: a-t/A-T DESTROYS that "
                       "item: space for next page");
        ic__clear_display(cur_display, &cur_display_size);
        change_all_ok_stats(true, true);
        ic__show_inven(item_ptr, true, true, &scr_state, &valid_flag, prompt,
                       cur_display, &cur_display_size);
      }
      break;

    case 'e': /*{ Equipment     }*/
      if (equip_ctr == 0) {
        msg_print("You are not using any equipment.");
      } else if (scr_state != 2) {
        /*{ Sets scr_state to 2           }*/
        clear_rc(1, 1);
        prt("You are currently using -", 1, 1);
        ic__show_equip(&scr_state, 1);
      }
      break;

    case 's': /*{ Statistics of an item }*/
      ic__clear_display(cur_display, &cur_display_size);
      if ((!(wizard1) && !(wizard2)) && 0) {
        msg_print("You *wish*, you sleazy scum-bag!");
      } else {
        if (inven_ctr == 0) {
          msg_print("You are not carrying anything.");
        } else {
          ic__stats(cur_display, &cur_display_size, prompt, &scr_state,
                    &valid_flag);
        }
      }
      break;

    case 't': /*{ Take off      }*/
      if (equip_ctr == 0) {
        msg_print("You are not using any equipment.");
      } else {
        /*{ May set scr_state to 2 }*/
        ic__unwear(&scr_state);
      }
      break;

    case 'w': /*{ Wear/wield    }*/
      if (inven_ctr == 0) {
        msg_print("You are not carrying anything.");
      } else {
        /*{ May set scr_state to 1        }*/
        ic__wear(cur_display, &cur_display_size, prompt, &scr_state,
                 &valid_flag);
      }
      break;

    case 'x':
      if (equipment[Equipment_primary].tval != 0) {
        ic__switch_weapon(&scr_state);
      } else if (equipment[Equipment_secondary].tval != 0) {
        ic__switch_weapon(&scr_state);
      } else {
        msg_print("But you are wielding no weapons.");
      }
      break;

    case 'M':
      if (scr_state != 4) {
        ic__show_money();
        scr_state = 4;
      }
      break;

    case 'p':
      if (inven_ctr == 0) {
        msg_print("You are not carrying anything.");
      } else {
        ic__put_inside();
      }
      break;

    case 'r':
      if (inven_ctr == 0) {
        msg_print("You are not carrying anything.");
      } else {
        ic__take_out();
      }
      break;

    case 'I':
      if (inven_ctr == 0) {
        msg_print("You are not carrying anything.");
      } else {
        ic__selective_inven(&scr_state, &valid_flag, prompt, cur_display,
                            &cur_display_size);
      }
      break;

    /*{ Special function for other routines                   }*/
    case '?': /* { Displays part inven, returns  }*/
      cur_inven = inventory_list;
      ic__clear_display(cur_display, &cur_display_size);
      return_value =
          ic__show_inven(item_ptr, true, false, &scr_state, &valid_flag, prompt,
                         cur_display, &cur_display_size);

      scr_state = 0; /*{ Clear screen state    }*/
      break;

    default:
      /*{ Nonsense command }*/
      break;
    } /* end case */

    if (scr_state > 0) {
      prt("<e>quip, <i>inven, <t>ake-off, <w>ear/wield, "
          "e<x>change, <M>oney, <c>lean.",
          23, 2);
      if (wizard2) {
        prt("<p>ut item into, <r>emove item from, <s> "
            "stats of item, <I>inven selective.",
            24, 2);
      } else {
        prt("<p>ut item into, <r>emove item from, "
            "<I>inven selective, or Esc to exit.",
            24, 2);
      }
      test_flag = false;

      do {
        command = inkey();
        switch (command) {
        case 0:
        case 3:
        case 25:
        case 26:
        case 27:
        case 32:
          /*{ Exit from module      }*/
          exit_flag = true;
          test_flag = true;
          break;

        case 'e':
        case 'i':
        case 'c':
        case 's':
        case 't':
        case 'w':
        case 'x':
        case 'M':
        case 'p':
        case 'r':
        case 'I':
        case 'W':
          /*{ Module commands }*/
          test_flag = true;
          break;

        case '?':
          break;

        default:
          break;
        } /* end switch */

      } while (!test_flag);
      prt("", 23, 1);
      prt("", 24, 1);

    } else {
      exit_flag = true;
    }

  } while (!exit_flag);

  if (scr_state > 0) { /*{ If true, must redraw screen   }*/
    return_value = true;
  }

  RETURN("inven_command", "i", 'b', "need redraw", &return_value);

  return return_value;
}

char cur_char1() {
  /*{ Returns a '*' for cursed items, a ')' for normal ones -RAK-   }*/
  /*{ NOTE: '*' returned only if item has been identified...        }*/

  char return_value;

  /* with inven_temp->data. do; */
  if ((Cursed_worn_bit & inven_temp.data.flags) == 0) {
    return_value = ')'; /*{ Not cursed...                 }*/
  } else if ((Known_cursed_bit & inven_temp.data.flags2) != 0) {
    return_value = '*'; /*{ Cursed and detected by spell }*/
  } else if (pindex(inven_temp.data.name, '^') > 0) {
    return_value = ')'; /*{ Cursed, but not identified    }*/
  } else {
    return_value = '*'; /*{ Cursed and identified...      }*/
  }

  return return_value;
}

char cur_char2() {
  char return_value;

  /*{ Returns a '*' for cursed items, a ')' for normal ones -RAK-   }*/

  if ((Cursed_worn_bit & inven_temp.data.flags) == 0) {
    return_value = ')'; /*{ Not cursed...  }*/
  } else {
    return_value = '*'; /*{ Cursed...      }*/
  }

  return return_value;
}

char cur_insure() {
  /*{ Returns a ' ' for uninsured items, a '(' for insured ones -DMF-}*/

  char return_value;

  if ((inven_temp.data.flags2 & Insured_bit) == 0) {
    return_value = ' ';
  } else {
    return_value = '(';
  }

  return return_value;
}

void inven_destroy(treas_rec *item_ptr) {
  /*{ Destroy an item in the inventory                      -RAK-   }*/

  ENTER(("inven_destroy", "i"));

  inven_temp.data = item_ptr->data;

  /* with item_ptr->data. do; */
  if ((item_ptr->data.number > 1) && (item_ptr->data.subval < 512)) {
    item_ptr->data.number--;
    inven_weight -= item_ptr->data.weight;
    inven_temp.data.number = 1;
  } else {
    inven_weight -= item_ptr->data.weight * item_ptr->data.number;
    delete_inven_item(item_ptr);
  }

  LEAVE("inven_destroy", "i");
}

void delete_inven_item(treas_rec *ptr) {
  /*{ Remove an item from inventory_list                    -DMF-   }*/

  treas_rec *temp_ptr;
  treas_rec *curse;

  ENTER(("delete_inven_item", "i"));

  if (cur_inven == ptr) {
    cur_inven = cur_inven->next;
  }

  if (ptr == inventory_list) {

    temp_ptr = inventory_list;
    inventory_list = ptr->next;
    dispose(temp_ptr, sizeof(treasure_type), "delete_inven_item 1");
    inven_ctr--;

  } else {

    if (cur_inven == NULL) {
      cur_inven = inventory_list;
    }

    for (curse = inventory_list; curse->next != ptr; curse = curse->next) {
    }

    temp_ptr = ptr;
    curse->next = ptr->next;
    dispose(temp_ptr, sizeof(treasure_type), "delete_inven_item 2");
    inven_ctr--;
  }

  LEAVE("delete_inven_item", "i");
}

boolean inven_check_weight() {
  /*{ Check inventory for too much weight                   -RAK-   }*/

  long item_wgt;
  boolean return_value;

  /* with inven_temp^.data do; */
  item_wgt = inven_temp.data.number * inven_temp.data.weight;

  /*{ Current stuff + weight <= max weight }*/

  return_value = (inven_weight + item_wgt) <= (C_player_max_bulk() * 100);

  return return_value;
}

boolean inven_check_num() {
  /*{ Check to see if he will be carrying too many objects  -RAK-   }*/

  return true;
}

long inven_damage(obj_set typ, long perc) {
  /*{ Destroys a type of item on a given percent chance     -RAK-   }*/

  long i2 = 0;
  treas_rec *curse;
  treas_rec *next_curse;

  ENTER(("inven_damage", "i"));

  for (curse = inventory_list; curse != NULL;) {
    next_curse = curse->next; /* get now since we may nuke the entry */

    /* with curse^.data do; */
    if (is_in(curse->data.tval, typ)) {
      if ((randint(100) < perc) && (!curse->is_in)) {
        if ((((curse->data.flags2 & Holding_bit) != 0) &&
             (curse->insides == 0)) ||
            ((curse->data.flags2 & Holding_bit) == 0)) {

          inven_destroy(curse);
          i2++;
        }
      }
    }
    curse = next_curse;
  }

  RETURN("inven_damage", "i", 'd', "destroyed this many: ", &i2);
  return i2;
}

boolean drop_money(treas_rec **ptr, boolean *clr) {
  /*{ Drop money onto ground                                -DMF-   }*/

  char out_val[200];
  char out_val2[82];
  int l_command;
  boolean reset_flag;
  long max;
  char mon_name[82];
  long amt;
  long pos;
  long mon_type;
  boolean return_value = false;

  *ptr = NULL;
  *clr = false;

  if (cave[char_row][char_col].tptr > 0) {
    msg_print("There is something there already.");
    *clr = true;
  } else {
    /* with player_do begin; */
    l_command = get_money_type("Drop ", &reset_flag, false);
    reset_flag = (!reset_flag);
    if (!reset_flag) {

      switch (l_command) {
      case 109:
        strcpy(mon_name, coin_name[MITHRIL]);
        break;
      case 112:
        strcpy(mon_name, coin_name[PLATINUM]);
        break;
      case 103:
        strcpy(mon_name, coin_name[GOLD]);
        break;
      case 115:
        strcpy(mon_name, coin_name[SILVER]);
        break;
      case 99:
        strcpy(mon_name, coin_name[COPPER]);
        break;
      case 105:
        strcpy(mon_name, coin_name[IRON]);
        break;
      }

      coin_stuff((char)l_command, &mon_type);
      max = player_money[mon_type];
      sprintf(out_val2, "%ld", max);
      sprintf(out_val, "Drop how much %s (1-%ld), Esc to exit : ", mon_name,
              max);
      prt(out_val, 1, 1);
      if (get_string(out_val2, 1, strlen(out_val) + 1, 10)) {
        sscanf(out_val2, "%ld", &amt);
        if (amt > max) {
          amt = max;
        }
        if (amt < 1) {
          msg_print("You don't have that much money.");
          *clr = true;
        } else {
          player_money[mon_type] -= amt;

          switch (mon_type) {
          case 1:
            pos = IRON_POS;
            break;
          case 2:
            pos = COPPER_POS;
            break;
          case 3:
            pos = SILVER_POS;
            break;
          case 4:
            pos = GOLD_POS;
            break;
          case 5:
            pos = PLATINUM_POS;
            break;
          case 6:
            pos = MITHRIL_POS;
            break;
          default:
            MSG(("Unknown mon_type in "
                 "drop_money"));
            pos = IRON_POS;
            break;
          }

          inven_temp.data = gold_list[pos - 1];
          inven_temp.data.number = amt;
          *ptr = &inven_temp;
          return_value = true;
          inven_weight -= COIN_WEIGHT * amt;
          reset_total_cash();
          prt_stat_block();
        }

      } else {
        erase_line(msg_line, msg_line);
      }
    }
  }
  return return_value;
}

boolean get_item(treas_rec **com_ptr, char const *pmt, boolean *redraw,
                 long count, char *choice, boolean mon, boolean no_wait) {
  /*{ Get the ID of an item and return the CTR value of it  -RAK-   }*/

  char command;
  char out_val[82];
  char char_str[2];
  boolean test_flag;
  long i1;
  boolean stay;
  boolean only_money;
  boolean return_value = false;

  ENTER(("get_item", "i"));

  only_money = false;
  stay = false;
  com_val = 0;

  if (count < 1) {
    only_money = true;
  }

  if (mon) {
    if (count > 20) {
      sprintf(out_val,
              "(Items a-t,$, <space> for inventory, "
              "Esc to exit) %s",
              pmt);
    } else if (!only_money) {
      sprintf(out_val,
              "(Items a-%c,$, <space> for inventory "
              "list, Esc to exit) %s",
              ((int)count + 96), pmt);
    } else {
      sprintf(out_val, " ");
    }
  } else {
    if (count > 20) {
      sprintf(out_val,
              "(Items a-t, <space> for inventory, "
              "Esc to exit) %s",
              pmt);
    } else {
      sprintf(out_val,
              "(Items a-%c, <space> for inventory "
              "list, Esc to exit) %s",
              ((int)count + 96), pmt);
    }
  }

  test_flag = false;
  if (!no_wait) {
    prt(out_val, 1, 1);
  }

  do {

    if (only_money) {
      command = '$';
    } else {
      if (!no_wait) {
        command = inkey();
      } else {
        command = '*';
      }
    }

    *choice = command;
    com_val = (long)(command);

    switch (com_val) {

    case 0:
    case 3:
    case 25:
    case 26:
    case 27:

      test_flag = true;
      reset_flag = true;
      break;

    case 42:
    case 32:

      clear_rc(2, 1);
      sprintf(out_val,
              "(Items a-%%N, <space> for next page, "
              "Esc to exit) %s",
              pmt);
      return_value = inven_command('?', com_ptr, out_val);
      test_flag = true;
      *redraw = true;
      break;

    case 36:
      if (mon) {

        test_flag = true;
        redraw = false;

        /* with player_do; */
        if (player_money[IRON] + player_money[COPPER] + player_money[SILVER] +
                player_money[GOLD] + player_money[PLATINUM] +
                player_money[MITHRIL] >
            0) {
          return_value = drop_money(com_ptr, &stay);
        } else {
          msg_print("You have no money to drop.");
          return_value = false;
          stay = true;
        }
      }
      break;

    case 49:
    case 50:
    case 51:
    case 52:
    case 53:
    case 54:
    case 55:
    case 56:
    case 57:

      test_flag = true;

      sprintf(char_str, "%c", (int)com_val);
      prt(char_str, 1, strlen(out_val) + 2);
      inkey(choice);
      prt(choice, 1, strlen(out_val) + 3);

      if ((*choice <= 't') && (*choice >= 'a')) {
        *com_ptr = inventory_list;
        count = 0;

        if (!(((*com_ptr)->next == NULL) ||
              (count >= (com_val - 49) * 20 + (long)(*choice) - 97))) {

          do {
            if ((!((*com_ptr)->is_in)) &&
                (((*com_ptr)->data.flags2 & Holding_bit) == 0)) {
              count++;
            }
            (*com_ptr) = (*com_ptr)->next;
          } while (!(((*com_ptr)->next == NULL) ||
                     (count == (com_val - 49) * 20 + (long)(choice)-97)));

          if (((*com_ptr)->next == NULL) &&
              (count != (com_val - 49) * 20 + (long)(choice)-97)) {
            return_value = false;
            stay = true;
            prt("Invalid Selection.", 1, 1);
          } else {
            return_value = true;
          }
        }
      }
      break;

    default:

      com_val -= 96;
      if ((com_val >= 1) && (com_val <= count) && (com_val <= 20)) {

        *com_ptr = inventory_list;
        i1 = 1;

        for (; !((*com_ptr)->ok);) {
          (*com_ptr) = (*com_ptr)->next;
        }

        for (; i1 != com_val;) {
          if ((*com_ptr)->ok) {
            i1++;
          }
          *com_ptr = (*com_ptr)->next;
          for (; !((*com_ptr)->ok);) {
            *com_ptr = (*com_ptr)->next;
          }
        } /* end for */

        test_flag = true;
        return_value = true;
      }      /* end if */
      break; /* end default */

    } /* end switch */

  } while (!test_flag);

  if (!stay) {
    erase_line(msg_line, msg_line);
  }

  RETURN("get_item", "i", 'b', "got an item", &return_value);
  return return_value;
}

void inven_drop(treas_rec *item_ptr, long y, long x, boolean mon) {
  /*{ Drops an item from inventory to given location        -RAK-   }*/

  long i1;
  treas_rec *temp_ptr;

  /* with cave[y][x]. do; */
  if (cave[y][x].tptr > 0) {
    pusht(cave[y][x].tptr);
  }
  temp_ptr = (treas_rec *)safe_malloc(sizeof(treas_rec), "inven_drop");
  temp_ptr->data = item_ptr->data;
  if (mon) {
    inven_temp.data = item_ptr->data;
  } else {
    inven_destroy(item_ptr);
  }
  popt(&i1);
  t_list[i1] = inven_temp.data;
  cave[y][x].tptr = i1;
  dispose(temp_ptr, sizeof(treas_rec), "inven_drop");
}
