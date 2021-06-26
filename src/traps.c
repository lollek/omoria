/* traps.c */
/* all sorts of nasty traps (stores too)! */

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
#include "trade.h"
#include "types.h"
#include "variables.h"

/*	{ Traps are just Nasty treasures...				} */
static treasure_type trap_lista[MAX_TRAPA + 1] = {
    {"bogus trap a", seen_trap, 0x00000000, 0x00000000, 0, 0, 1, 0, 0, 0, 0, 0,
     0, "2d6", -50, 0},
    {"an open pit", seen_trap, 0x00000000, 0x00000000, 0, 0, 1, 0, 0, 0, 0, 0,
     0, "2d6", -50, 0},
    {"an arrow trap", unseen_trap, 0x00000000, 0x00000000, 0, 0, 2, 0, 0, 0, 0,
     0, 0, "1d8", 0, 0},
    {"a covered pit", unseen_trap, 0x00000000, 0x00000000, 0, 0, 3, 0, 0, 0, 0,
     0, 0, "2d6", 0, 0},
    {"a trap door", unseen_trap, 0x00000000, 0x00000000, 0, 0, 4, 0, 0, 0, 0, 0,
     0, "2d8", 0, 0},
    {"a gas trap", unseen_trap, 0x00000000, 0x00000000, 0, 0, 5, 0, 0, 0, 0, 0,
     0, "1d4", 0, 0},
    {"a loose rock", unseen_trap, 0x00000000, 0x00000000, 0, 0, 6, 0, 0, 0, 0,
     0, 0, "0d0", 0, 0},
    {"a dart trap", unseen_trap, 0x00000000, 0x00000000, 0, 0, 7, 0, 0, 0, 0, 0,
     0, "1d4", 0, 0},
    {"a strange rune", unseen_trap, 0x00000000, 0x00000000, 0, 0, 8, 0, 0, 0, 0,
     0, 0, "0d0", 0, 0},
    {"some loose rock", unseen_trap, 0x00000000, 0x00000000, 0, 0, 9, 0, 0, 0,
     0, 0, 0, "2d6", 0, 0},
    {"a gas trap", unseen_trap, 0x00000000, 0x00000000, 0, 0, 10, 0, 0, 0, 0, 0,
     0, "1d4", 0, 0},
    {"a strange rune", unseen_trap, 0x00000000, 0x00000000, 0, 0, 11, 0, 0, 0,
     0, 0, 0, "0d0", 0, 0},
    {"a blackened spot", unseen_trap, 0x00000000, 0x00000000, 0, 0, 12, 0, 0, 0,
     0, 0, 0, "4d6", 0, 0},
    {"some corroded rock", unseen_trap, 0x00000000, 0x00000000, 0, 0, 13, 0, 0,
     0, 0, 0, 0, "4d6", 0, 0},
    {"a gas trap", unseen_trap, 0x00000000, 0x00000000, 0, 0, 14, 0, 0, 0, 0, 0,
     0, "2d6", 0, 0},
    {"a gas trap", unseen_trap, 0x00000000, 0x00000000, 5, 0, 15, 0, 0, 0, 0, 0,
     0, "1d4", 10, 0},
    {"a gas trap", unseen_trap, 0x00000000, 0x00000000, 5, 0, 16, 0, 0, 0, 0, 0,
     0, "1d8", 5, 0},
    {"a dart trap", unseen_trap, 0x00000000, 0x00000000, 5, 0, 17, 0, 0, 0, 0,
     0, 0, "1d8", 10, 0},
    {"a dart trap", unseen_trap, 0x00000000, 0x00000000, 5, 0, 18, 0, 0, 0, 0,
     0, 0, "1d8", 10, 0},
    {"a chute", unseen_trap, 0x00000000, 0x00000000, 5, 0, 20, 0, 0, 0, 0, 0, 0,
     "4d8", 20, 0}};

/*	{ Traps: Level represents the difficulty of disarming;	} */
/*	{ and P1 represents the experienced gained when disarmed} */
static treasure_type trap_listb[MAX_TRAPB + 1] = {
    {"bogus trap b", seen_trap, 0x00000000, 0x00000000, 0, 0, 1, 0, 0, 0, 0, 0,
     0, "2d6", -50, 0},
    {"an open pit", seen_trap, 0x00000000, 0x00000000, 1, 0, 1, 0, 0, 0, 0, 0,
     0, "2d6", -50, 0},
    {"an arrow trap", seen_trap, 0x00000000, 0x00000000, 3, 0, 2, 0, 0, 0, 0, 0,
     0, "1d8", -10, 0},
    {"a covered pit", seen_trap, 0x00000000, 0x00000000, 2, 0, 3, 0, 0, 0, 0, 0,
     0, "2d6", -40, 0},
    {"a trap door", seen_trap, 0x00000000, 0x00000000, 5, 0, 4, 0, 0, 0, 0, 0,
     0, "2d8", -25, 0},
    {"a gas trap", seen_trap, 0x00000000, 0x00000000, 3, 0, 5, 0, 0, 0, 0, 0, 0,
     "1d4", 5, 0},
    {"a loose rock", seen_trap, 0x00000000, 0x00000000, 0, 0, 6, 0, 0, 0, 0, 0,
     0, "0d0", -90, 0},
    {"a dart trap", seen_trap, 0x00000000, 0x00000000, 5, 0, 7, 0, 0, 0, 0, 0,
     0, "1d4", 10, 0},
    {"a strange rune", seen_trap, 0x00000000, 0x00000000, 5, 0, 8, 0, 0, 0, 0,
     0, 0, "0d0", -10, 0},
    {"some loose rock", seen_trap, 0x00000000, 0x00000000, 5, 0, 9, 0, 0, 0, 0,
     0, 0, "2d6", -10, 0},
    {"a gas trap", seen_trap, 0x00000000, 0x00000000, 10, 0, 10, 0, 0, 0, 0, 0,
     0, "1d4", 5, 0},
    {"a strange rune", seen_trap, 0x00000000, 0x00000000, 5, 0, 11, 0, 0, 0, 0,
     0, 0, "0d0", -10, 0},
    {"a blackened spot", seen_trap, 0x00000000, 0x00000000, 10, 0, 12, 0, 0, 0,
     0, 0, 0, "4d6", 10, 0},
    {"some corroded rock", seen_trap, 0x00000000, 0x00000000, 10, 0, 13, 0, 0,
     0, 0, 0, 0, "4d6", 10, 0},
    {"a gas trap", seen_trap, 0x00000000, 0x00000000, 5, 0, 14, 0, 0, 0, 0, 0,
     0, "2d6", 5, 0},
    {"a gas trap", seen_trap, 0x00000000, 0x00000000, 5, 0, 15, 0, 0, 0, 0, 0,
     0, "1d4", 10, 0},
    {"a gas trap", seen_trap, 0x00000000, 0x00000000, 5, 0, 16, 0, 0, 0, 0, 0,
     0, "1d8", 5, 0},
    {"a dart trap", seen_trap, 0x00000000, 0x00000000, 5, 0, 17, 0, 0, 0, 0, 0,
     0, "1d8", 10, 0},
    {"a dart trap", seen_trap, 0x00000000, 0x00000000, 5, 0, 18, 0, 0, 0, 0, 0,
     0, "1d8", 10, 0},
    /*	{ Special case, see DOOR_LIST below (subvals must agree)	} */
    {"a closed door", closed_door, 0x00000000, 0x00000000, 0, 0, 19, 0, 0, 0, 0,
     0, 0, "1d1", 0, 0},
    {"a chute", seen_trap, 0x00000000, 0x00000000, 5, 0, 20, 0, 0, 0, 0, 0, 0,
     "4d8", 20, 0}};

static treasure_type some_rubble = {
    "some rubble", rubble, 0x00000000, 0x00000000, 0, 0, 1, 0, 0, 0, 0, 0, 0,
    "0d0",         0,      0};

/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void place_trap(long y, long x, long typ, long subval) {
  /*{ Places a particular trap at location y,x		-RAK-	}*/

  long cur_pos;
  treasure_type cur_trap;

  if (typ == 1) {
    cur_trap = trap_lista[subval];
  } else {
    cur_trap = trap_listb[subval];
  }

  popt(&cur_pos);
  cave[y][x].tptr = cur_pos;
  t_list[cur_pos] = cur_trap;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void change_trap(long y, long x) {
  /*{ Change a trap from invisible to visible               -RAK-   }*/
  /*{ Note: Secret doors are handled here                           }*/

  long i3;
  obj_set little_things = {unseen_trap, secret_door, 0};

  /* with cave[y,x] do; */
  if (is_in(t_list[cave[y][x].tptr].tval, little_things)) {
    i3 = cave[y][x].tptr;
    place_trap(y, x, 2, t_list[i3].subval);
    pusht(i3);
    lite_spot(y, x);
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void place_rubble(long y, long x) {
  /*{ Places rubble at location y,x				-RAK-
   * }*/

  long cur_pos;

  popt(&cur_pos);

  cave[y][x].tptr = cur_pos;
  cave[y][x].fopen = false;

  t_list[cur_pos] = some_rubble;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
static void ht__open_pit(long dam) {
  msg_print("You fell into a pit!");
  if (player_flags.ffall) {
    msg_print("You gently float down.");
  } else {
    take_hit(dam, "an open pit");
  }
}
/*//////////////////////////////////////////////////////////////////// */
static void ht__arrow(long dam) {
  if (test_hit(125, 0, 0, player_pac + player_ptoac)) {
    take_hit(dam, "an arrow trap");
    msg_print("An arrow hits you.");
  } else {
    msg_print("An arrow barely misses you.");
  }
}
/*//////////////////////////////////////////////////////////////////// */
static void ht__covered_pit(long dam, long y, long x) {
  msg_print("You fell into a covered pit.");
  if (player_flags.ffall) {
    msg_print("You gently float down.");
  } else {
    take_hit(dam, "a covered pit");
    place_trap(y, x, 2, 1);
  }
}
/*//////////////////////////////////////////////////////////////////// */
static void ht__trap_door(long dam) {
  msg_print("You fell through a trap door!");
  msg_print(" ");
  moria_flag = true;
  dun_level++;
  if (player_flags.ffall) {
    msg_print("You gently float down.");
  } else {
    take_hit(dam, "a trap door");
  }
}
/*//////////////////////////////////////////////////////////////////// */
static void ht__sleep_gas(void) {

  if (player_flags.paralysis == 0) {
    msg_print("A strange white mist surrounds you!");
    if (player_flags.free_act) {
      msg_print("You are unaffected.");
    } else {
      msg_print("You fall asleep.");
      player_flags.paralysis += randint(10) + 4;
    }
  }
}
/*//////////////////////////////////////////////////////////////////// */
static void ht__hidden_object(long y, long x) {
  cave[y][x].fm = false;
  pusht(cave[y][x].tptr);
  place_object(y, x);
  msg_print("Hmmm, there was something under this rock.");
}
/*//////////////////////////////////////////////////////////////////// */
static void ht__str_dart(long dam) {
  if (test_hit(125, 0, 0, player_pac + player_ptoac)) {
    if (lose_stat(STR, "", "A small dart hits you.")) {
      take_hit(dam, "a dart trap");
      print_stat |= 0x0001;
      msg_print("A small dart weakens you!");
    }
  } else {
    msg_print("A small dart barely misses you.");
  }
}
/*//////////////////////////////////////////////////////////////////// */
static void ht__teleport(void) {
  teleport_flag = true;
  msg_print("You hit a teleport trap!");
}
/*//////////////////////////////////////////////////////////////////// */
static void ht__rockfall(long dam, long y, long x) {
  take_hit(dam, "falling rock");
  pusht(cave[y][x].tptr);
  place_rubble(y, x);
  msg_print("You are hit by falling rock");
}
/*//////////////////////////////////////////////////////////////////// */
static void ht__corrode_gas(void) {
  corrode_gas("corrosion gas.");
  msg_print("A strange red gas surrounds you.");
}
/*//////////////////////////////////////////////////////////////////// */
static void ht__summon_monster(long y, long x) {
  long i1, ty, tx;

  cave[y][x].fm = false; /*{ Rune disappears...    }*/
  pusht(cave[y][x].tptr);
  cave[y][x].tptr = 0;
  for (i1 = 1; i1 <= (2 + randint(3)); i1++) {
    ty = char_row;
    tx = char_col;
    if (is_in(cave[ty][tx].fval, water_set)) {
      summon_water_monster(&ty, &tx, false);
    } else {
      summon_land_monster(&ty, &tx, false);
    }
  }
}
/*//////////////////////////////////////////////////////////////////// */
static void ht__fire(long dam) {
  fire_dam(dam, "a fire trap.");
  msg_print("You are enveloped in flames!");
}
/*//////////////////////////////////////////////////////////////////// */
static void ht__acid(long dam) {
  acid_dam(dam, "an acid trap.");
  msg_print("You are splashed with acid!");
}
/*//////////////////////////////////////////////////////////////////// */
static void ht__poison_gas(long dam) {
  poison_gas(dam, "a poison gas trap.");
  msg_print("A pungent green gas surrounds you!");
}
/*//////////////////////////////////////////////////////////////////// */
static void ht__blind_gas(void) {
  msg_print("A black gas surrounds you!");
  (player_flags).blind += randint(50) + 50;
}
/*//////////////////////////////////////////////////////////////////// */
static void ht__confuse_gas(void) {
  msg_print("A gas of scintillating colors surrounds you!");
  (player_flags).confused += randint(15) + 15;
}
/*//////////////////////////////////////////////////////////////////// */
static void ht__slow_dart(long dam) {
  if (test_hit(125, 0, 0, player_pac + player_ptoac)) {
    take_hit(dam, "a dart trap");
    msg_print("A small dart hits you!");
    (player_flags).slow += randint(20) + 10;
  } else {
    msg_print("A small dart barely misses you.");
  }
}
/*//////////////////////////////////////////////////////////////////// */
static void ht__con_dart(long dam) {
  if (test_hit(125, 0, 0, player_pac + player_ptoac)) {
    if (lose_stat(CON, "", "A small dart hits you.")) {
      take_hit(dam, "a dart trap");
      print_stat |= 0x0004;
      msg_print("A small dart weakens you!");
    }
  } else {
    msg_print("A small dart barely misses you.");
  }
}
/*//////////////////////////////////////////////////////////////////// */
static void ht__secret_door(void) {}
/*//////////////////////////////////////////////////////////////////// */
static void ht__chute(long dam) {
  msg_print("You fell down a chute!");
  msg_print(" ");
  moria_flag = true;
  dun_level += randint(6);
  if (player_flags.ffall) {
    msg_print("You gently slide down.");
  } else {
    take_hit(dam, "chute landing");
  }
}
/*//////////////////////////////////////////////////////////////////// */
static void ht__scare_monster(void) {}
/*//////////////////////////////////////////////////////////////////// */
static void ht__whirlpool(long dam) {
  msg_print("You are swept into a whirlpool!");
  msg_print(" ");
  moria_flag = true;
  do {
    dun_level++;
    if (!(player_flags.ffall)) { /*{XXX...swimming_worn}*/
      msg_print("You are drowning!");
      take_hit(dam, "drowning");
    }
  } while (randint(2) != 1);
}
/*//////////////////////////////////////////////////////////////////// */
static void ht__house(long y, long x) {
  switch (t_list[cave[y][x].tptr].p1) {
  case 1:
    msg_print("The building is empty.");
    if (react(10) == 0) {
      msg_print("The building is being guarded!");
      call_guards("Magic Mouth spell");
    }
    break;

  case 2:
    msg_print("There is a Thieves' Guild meeting here.");
    switch (react(6)) {
    case 0:
      call_guards("Guildmaster");
      break;

    case 1:
    case 2:
    case 3:
    case 4:
    case 5:
    case 6:
    case 7:
      kicked_out();
      break;

    case 8:
    case 9:
    case 10:
      thief_games();
      break;
    }
    break;

  case 3:
    msg_print("This is a town brothel.  Some young prostitutes are "
              "here.");
    switch (react(10)) {
    case 0:
      call_guards("prostitutes");
      break;

    case 1:
    case 2:
    case 3:
    case 4:
    case 5:
    case 6:
      kicked_out();
      break;

    default:
      if (characters_sex() != FEMALE) {
        msg_print("The girls invite you to prove your "
                  "abilities.");
        battle_game(C_player_mod_from_stat(CHR), "some playful prostitutes");
      } else {
        msg_print("The girls invite you to work with them.");
        brothel_game();
      }
    }
    break;

  case 4:
    msg_print("Some drunken fighters are telling tales here.");
    switch (react(8)) {
    case 0:
      call_guards("group of fighters");
      break;

    case 1:
    case 2:
    case 3:
    case 4:
    case 5:
    case 6:
      kicked_out();
      break;

    default:
      msg_print("They ask you to demonstrate your fighting skill.");
      battle_game(player_ptohit, "some drunken fighters");
      break;
    }
    break;

  case 5:
    msg_print("There is a party in progress here.");
    switch (react(8)) {
    case 0:
      call_guards("party's host");
      break;

    case 1:
    case 2:
    case 3:
    case 4:
    case 5:
      kicked_out();
      break;

    default:
      party();
    }
    break;

  case 6:
    switch (randint(2)) {
    case 1:
      msg_print("The building is a poorhouse.");
      break;
    case 2:
      msg_print("This is an orphanage.");
      break;
    }

    switch (react(12)) {
    case 0:
      call_guards("beggars");
      break;

    case 1:
    case 2:
    case 3:
    case 4:
      kicked_out();
      break;

    default:
      switch (randint(2)) {
      case 1: /*beg_food();  break;*/ /* beg_food is
                                         unfinished XXXX */
      case 2:
        beg_money();
        break;
      }
    }
    break;

  case 7:
  case 8:
    switch (randint(3)) {
    case 1:
      msg_print("This is the home of a peasant family.");
      break;
    case 2:
      msg_print("These are the quarters of a humble laborer.");
      break;
    case 3:
      msg_print(" This is the home of several poor families.");
      break;
    }

    switch (react(8)) {
    case 0:
      call_guards("peasant(s)");
      break;

    case 1:
    case 2:
    case 3:
      kicked_out();
      break;

    case 4:
    case 5:
    case 6:
    case 7:
      invite_for_meal();
      break;

    case 8:
    case 9:
    case 10:
      spend_the_night("peasant(s)");
      break;
    }
    break;

  case 9:
    switch (randint(3)) {
    case 1:
    case 2:
      msg_print("This is the home of a merchant.");
      break;
    case 3:
      msg_print("This is the house of an accomplished craftsman.");
      break;
    }

    switch (react(5)) {
    case 0:
      call_guards("owner");
      break;

    case 1:
    case 2:
    case 3:
    case 4:
      kicked_out();
      break;

    case 5:
    case 6:
    case 7:
    case 8:
    case 9:
      invite_for_meal();
      break;

    case 10:
      spend_the_night("gentleman");
      break;
    }
    break;

  case 10:
    msg_print("There is a religious service in progress here.");
    switch (react(8)) {
    case 0:
      call_guards("High Priest");
      break;

    case 1:
    case 2:
    case 3:
    case 4:
    case 5:
      kicked_out();
      break;

    default:
      worship();
      break;
    }
    break;

  case 11:
    switch (randint(3)) {
    case 1:
      msg_print("This is the house of a wealthy shopkeeper.");
      break;
    case 2:
      msg_print("This is the mansion of a affluent noble.");
      break;
    case 3:
      msg_print("This is the estate of an rich guildsman.");
      break;
    }

    switch (react(2)) {
    case 0:
      call_guards("master of the house");
      break;

    case 1:
    case 2:
    case 3:
      kicked_out();
      break;

    case 4:
    case 5:
    case 6:
    case 7:
    case 8:
    case 9:
      invite_for_meal();
      break;

    case 10:
      spend_the_night("master of the house");
      break;
    }
    break;

  case 12:
    msg_print("This is the home of a powerful mage.");
    switch (react(5)) {
    case 0:
      call_wizards();
      break;

    case 1:
    case 2:
    case 3:
      call_guards("mage");
      break;

    case 4:
    case 5:
    case 6:
    case 7:
    case 8:
    case 9:
      kicked_out();
      break;

    case 10:
      invite_for_meal();
      break;
    }
  }

  t_list[cave[y][x].tptr].p1 = 1;

  prt_stat_block();
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void hit_trap(long *y, long *x) {
  /*{ Player hit a trap...  (Chuckle)                       -RAK-   }*/

  long dam;

  ENTER(("hit_trap", ""));

  change_trap(*y, *x);
  lite_spot(char_row, char_col);
  find_flag = false;

  /* with cave[*y][*x]. do; */
  /* with player_do; */

  dam = damroll(t_list[cave[*y][*x].tptr].damage);

  switch (t_list[cave[*y][*x].tptr].subval) {

  case 1:
    ht__open_pit(dam);
    break;

  case 2:
    ht__arrow(dam);
    break;

  case 3:
    ht__covered_pit(dam, *y, *x);
    break;

  case 4:
    ht__trap_door(dam);
    break;

  case 5:
    ht__sleep_gas();
    break;

  case 6:
    ht__hidden_object(*y, *x);
    break;

  case 7:
    ht__str_dart(dam);
    break;

  case 8:
    ht__teleport();
    break;

  case 9:
    ht__rockfall(dam, *y, *x);
    break;

  case 10:
    ht__corrode_gas();
    break;

  case 11:
    ht__summon_monster(*y, *x);
    break;

  case 12:
    ht__fire(dam);
    break;

  case 13:
    ht__acid(dam);
    break;

  case 14:
    ht__poison_gas(dam);
    break;

  case 15:
    ht__blind_gas();
    break;

  case 16:
    ht__confuse_gas();
    break;

  case 17:
    ht__slow_dart(dam);
    break;

  case 18:
    ht__con_dart(dam);
    break;

  case 19:
    ht__secret_door();
    break;

  case 20:
    ht__chute(dam);
    break;

  case 99:
    ht__scare_monster();
    break;

  /*{ Town level traps are special, the stores...   }*/
  case 101: /* { General    } */
    check_store_hours_and_enter(S_GENERAL, S_GENERAL, S_GENERAL);
    break;

  case 102: /* { Armory     } */
    check_store_hours_and_enter(S_ARMORY, S_ARMORY, S_ARMORY);
    break;

  case 103: /* { Weaponsmith} */
    check_store_hours_and_enter(S_WEAPONS, S_WEAPONS, S_WEAPONS);
    break;

  case 104: /* { Temple     } */
    check_store_hours_and_enter(S_TEMPLE, S_TEMPLE, S_TEMPLE);
    break;

  case 105: /* { Alchemy    } */
    check_store_hours_and_enter(S_ALCHEMY, S_ALCHEMY, S_ALCHEMY);
    break;

  case 106: /* { Magic-User } */
    check_store_hours_and_enter(S_MAGIC, S_MAGIC, S_MAGIC);
    break;

  case 107: /* { Inn        } */
    check_store_hours_and_enter(S_INN, S_INN, S_INN);
    break;

  case 108: /* { Trade Post } */
    if (check_store_hours(S_TRADE_POST, -1)) {
      enter_trading_post();
    }
    break;

  case 109: /* { Library    } */
    check_store_hours_and_enter(S_LIBRARY, S_LIBRARY, S_LIBRARY);
    break;

  case 110: /* { Music Shop } */
    check_store_hours_and_enter(S_MUSIC, S_MUSIC, S_MUSIC);
    break;

  case 111: /* { Insurance  } */
    msg_print("The insurance shop has gone out of business.");
    break;

  case 112: /* { Bank       } */
    if (check_store_hours(S_BANK, -1)) {
      enter_bank();
    }
    break;

  case 113: /* { Gem Shop   } */
    check_store_hours_and_enter(S_GEM, S_GEM, S_GEM);
    break;

  case 114: /* { $ Changer  } */
    if (check_store_hours(S_CHANGER, -1)) {
      msg_print("Oh, just go to the bloody bank!");
    }
    break;

  case 115: /* { Casino     } */
    if (check_store_hours(S_CASINO, -1)) {
      enter_casino();
    }
    break;

  case 116: /* { Deli       } */
    check_store_hours_and_enter(S_DELI, S_DELI, S_DELI);
    break;

  case 117: /* { Fortress   } */
    enter_fortress();
    break;

  case 118: /* { Black Market } */
    check_store_hours_and_enter(S_BLACK_MARKET, S_BLACK_MARKET, S_BLACK_MARKET);
    break;

  case 120:
  case 121:
  case 122: /* { House      } */
    ht__house(*y, *x);
    break;

  case 123:
    ht__whirlpool(dam);
    break;

  default:
    msg_print("You got lucky: unknown trap value.");
    break;
  }

  LEAVE("hit_trap", "");
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
