#include "inventory/inven.h"
#include "io.h"
#include "player.h"
#include "random.h"
#include "screen.h"
#include "text_lines.h"
#include "types.h"
#include "variables.h"

#include <stdbool.h>

static bool minus_ac(const long typ_dam) {
  /*{ AC gets worse                                         -RAK-   }*/
  /*{ Note: This routine affects magical AC bonuse so that stores   }*/
  /*{       can detect the damage.                                  }*/

  long i1 = 0;
  long tmp[9]; /*  : array [1..8] of long;*/
  bool return_value = false;

  if (equipment[Equipment_armor].tval > 0) {
    i1++;
    tmp[i1] = Equipment_armor;
  }
  if (equipment[Equipment_shield].tval > 0) {
    i1++;
    tmp[i1] = Equipment_shield;
  }
  if (equipment[Equipment_cloak].tval > 0) {
    i1++;
    tmp[i1] = Equipment_cloak;
  }
  if (equipment[Equipment_gloves].tval > 0) {
    i1++;
    tmp[i1] = Equipment_gloves;
  }
  if (equipment[Equipment_helm].tval > 0) {
    i1++;
    tmp[i1] = Equipment_helm;
  }
  if (equipment[Equipment_boots].tval > 0) {
    i1++;
    tmp[i1] = Equipment_boots;
  }
  if (equipment[Equipment_belt].tval > 0) {
    i1++;
    tmp[i1] = Equipment_belt;
  }
  if (equipment[Equipment_bracers].tval > 0) {
    i1++;
    tmp[i1] = Equipment_bracers;
  }

  if (i1 > 0) {
    char out_val[82];
    char out_str[120];
    const long i2 = tmp[randint(i1)];
    inven_temp.data = equipment[i2];
    /* with equipment[i2] do; */
    if ((equipment[i2].flags & typ_dam) != 0) {
      item_name(out_val, &inven_temp);
      sprintf(out_str, "Your %s resists damage!", out_val);
      msg_print(out_str);
      return_value = true;
    } else if (equipment[i2].ac + equipment[i2].toac > 0) {
      item_name(out_val, &inven_temp);
      sprintf(out_str, "Your %s is damaged!", out_val);
      msg_print(out_str);
      equipment[i2].toac--;
      py_bonuses(&blank_treasure, 0);
      return_value = true;
    }
  }

  return return_value;
}

void corrode_gas(char const *const kb_str) {

  obj_set things_that_corrode = {dagger, sword,      gem_helm, helm,
                                 shield, hard_armor, wand,     0};

  if (!minus_ac(Resist_Acid_worn_bit)) {
    /* if nothing corrodes then take damage */
    take_hit(randint(8), kb_str);
  }

  print_stat = 1;

  if (inven_damage(things_that_corrode, 5) > 0) {
    msg_print("There is an acrid smell coming from your pack.");
    prt_stat_block();
  }
}

void poison_gas(const long dam, char const *const kb_str) {

  take_hit(dam, kb_str);
  print_stat = 1;
  player_flags.poisoned += 12 + randint(dam);
}

void fire_dam(long dam, char const *const kb_str) {

  obj_set things_that_burn = {arrow,
                              bow_crossbow_or_sling,
                              hafted_weapon,
                              pole_arm,
                              maul,
                              boots,
                              gloves_and_gauntlets,
                              cloak,
                              soft_armor,
                              staff,
                              scroll1,
                              scroll2,
                              0};

  if (player_flags.fire_resist) {
    dam /= 3;
  }
  if (player_flags.resist_heat > 0) {
    dam /= 3;
  }

  take_hit(dam, kb_str);

  print_stat = 1;

  if (inven_damage(things_that_burn, 3) > 0) {
    msg_print("There is smoke coming from your pack!");
    prt_stat_block();
  }
}

void acid_dam(const long dam, char const *const kb_str) {

  long flag = 0;
  obj_set things_that_dilute = {
      miscellaneous_object,  chest,         bolt,       arrow,
      bow_crossbow_or_sling, hafted_weapon, pole_arm,   boots,
      gloves_and_gauntlets,  cloak,         soft_armor, 0};

  if (minus_ac(Resist_Acid_worn_bit)) {
    flag = 1;
  }
  if (player_flags.acid_resist) {
    flag += 2;
  }

  switch (flag) {
  case 0:
    take_hit(dam / 1, kb_str);
    break;
  case 1:
    take_hit(dam / 2, kb_str);
    break;
  case 2:
    take_hit(dam / 3, kb_str);
    break;
  case 3:
    take_hit(dam / 4, kb_str);
    break;
  }

  print_stat = 1;

  if (inven_damage(things_that_dilute, 3) > 0) {
    msg_print("There is an acrid smell coming from your pack!");
    prt_stat_block();
  }
}

void cold_dam(long dam, char const *const kb_str) {
  obj_set things_that_freeze = {potion1, potion2, 0};

  if (player_flags.cold_resist) {
    dam /= 3;
  }
  if (player_flags.resist_cold > 0) {
    dam /= 3;
  }

  take_hit(dam, kb_str);

  print_stat = 1;

  if (inven_damage(things_that_freeze, 5) > 0) {
    msg_print("Something shatters inside your pack!");
    prt_stat_block();
  }
}

void light_dam(long dam, char const *const kb_str) {

  if (player_flags.lght_resist) {
    dam /= 3;
  }
  if (player_flags.resist_lght > 0) {
    dam /= 3;
  }

  take_hit(dam, kb_str);

  print_stat = 1;
}
