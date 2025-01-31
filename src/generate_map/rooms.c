#include "../c.h"
#include "../constants.h"
#include "../generate_monster.h"
#include "../io.h"
#include "../loot/loot.h"
#include "../misc.h"
#include "../pascal.h"
#include "../random.h"
#include "../traps.h"
#include "../types.h"
#include "../variables.h"
#include "misc.h"
#include <curses.h>

/*{ Place a trap with a given displacement of point	-RAK-	}*/
static void gc__vault_trap(const int64_t y, const int64_t x, const int64_t yd,
                           const int64_t xd, const int64_t num) {

  for (int64_t i = 1; i <= num; i++) {
    bool flag = false;
    int64_t count = 0;

    do {
      const int64_t y1 = y - yd - 1 + randint(2 * yd + 1);
      const int64_t x1 = x - xd - 1 + randint(2 * xd + 1);
      if (is_in(cave[y1][x1].fval, floor_set)) {
        if (cave[y1][x1].tptr == 0) {
          place_trap(y1, x1, 1, randint(MAX_TRAPA));
          flag = true;
        }
      }
      count++;
    } while (!(flag || count > 5));
  }
}

/*{ Place a trap with a given displacement of point	-RAK-	}*/
static void gc__vault_monster(const int64_t y, const int64_t x, const int64_t num) {
  for (int64_t i1 = 1; i1 <= num; i1++) {
    int64_t y1 = y;
    int64_t x1 = x;
    summon_land_monster(&y1, &x1, true);
  }
}

/*{ Builds a room at a row,column coordinate		-RAK-	}*/
void gc__build_room(const int64_t yval, const int64_t xval) {

  floor_type cur_floor;

  if (dun_level <= randint(25)) {
    cur_floor = lopen_floor; /*{ Floor with light	}*/
  } else {
    cur_floor = dopen_floor; /*{ Dark floor		}*/
  }

  const int64_t y_height = yval - randint(4);
  const int64_t y_depth = yval + randint(3);
  const int64_t x_left = xval - randint(11);
  const int64_t x_right = xval + randint(11);

  for (int64_t i = y_height; i <= y_depth; i++) {
    for (int64_t i2 = x_left; i2 <= x_right; i2++) {
      cave[i][i2].fval = cur_floor.ftval;
      cave[i][i2].fopen = cur_floor.ftopen;
    }
  }

  for (int64_t i = y_height - 1; i <= y_depth + 1; i++) {
    cave[i][x_left - 1].fval = rock_wall1.ftval;
    cave[i][x_left - 1].fopen = rock_wall1.ftopen;
    cave[i][x_right + 1].fval = rock_wall1.ftval;
    cave[i][x_right + 1].fopen = rock_wall1.ftopen;
  }

  for (int64_t i = x_left; i <= x_right; i++) {
    cave[y_height - 1][i].fval = rock_wall1.ftval;
    cave[y_height - 1][i].fopen = rock_wall1.ftopen;
    cave[y_depth + 1][i].fval = rock_wall1.ftval;
    cave[y_depth + 1][i].fopen = rock_wall1.ftopen;
  }
}

/* Builds a room at a row,column coordinate		-RAK-
   Type 1 unusual rooms are several overlapping rectangular ones */
void gc__build_type1(const int64_t yval, const int64_t xval) {

  int64_t i1;
  floor_type cur_floor;

  if (dun_level <= randint(25)) {
    cur_floor = lopen_floor; /*{ Floor with light	}*/
  } else {
    cur_floor = dopen_floor; /*{ Dark floor		}*/
  }

  for (int64_t i0 = 1; i0 <= 1 + randint(2); i0++) {
    const int64_t y_height = yval - randint(4);
    const int64_t y_depth = yval + randint(3);
    const int64_t x_left = xval - randint(11);
    const int64_t x_right = xval + randint(11);

    for (i1 = y_height; i1 <= y_depth; i1++) {
      for (int64_t i2 = x_left; i2 <= x_right; i2++) {
        cave[i1][i2].fval = cur_floor.ftval;
        cave[i1][i2].fopen = cur_floor.ftopen;
      }
    }

    for (i1 = y_height - 1; i1 <= y_depth + 1; i1++) {

      if (cave[i1][x_left - 1].fval != cur_floor.ftval) {
        cave[i1][x_left - 1].fval = rock_wall1.ftval;
        cave[i1][x_left - 1].fopen = rock_wall1.ftopen;
      }

      if (cave[i1][x_right + 1].fval != cur_floor.ftval) {
        cave[i1][x_right + 1].fval = rock_wall1.ftval;
        cave[i1][x_right + 1].fopen = rock_wall1.ftopen;
      }
    }

    for (i1 = x_left; i1 <= x_right; i1++) {

      if (cave[y_height - 1][i1].fval != cur_floor.ftval) {
        cave[y_height - 1][i1].fval = rock_wall1.ftval;
        cave[y_height - 1][i1].fopen = rock_wall1.ftopen;
      }

      if (cave[y_depth + 1][i1].fval != cur_floor.ftval) {
        cave[y_depth + 1][i1].fval = rock_wall1.ftval;
        cave[y_depth + 1][i1].fopen = rock_wall1.ftopen;
      }
    }
  }
}

/* Builds an unusual room at a row,column coordinate	-RAK- 
   Type 2 unusual rooms all have an inner room: 

     1 - Just an inner room with one door 
     2 - An inner room within an inner room 
     3 - An inner room with pillar(s) 
     4 - Inner room has a maze
     5 - A set of four inner rooms }*/
void gc__build_type2(const int64_t yval, const int64_t xval) {

  int64_t i1;
  int64_t i2;
  floor_type cur_floor;

  if (dun_level <= randint(30)) {
    cur_floor = lopen_floor; /*{ Floor with light	}*/
  } else {
    cur_floor = dopen_floor; /*{ Dark floor		}*/
  }

  int64_t y_height = yval - 4;
  int64_t y_depth = yval + 4;
  int64_t x_left = xval - 11;
  int64_t x_right = xval + 11;

  for (i1 = y_height; i1 <= y_depth; i1++) {
    for (i2 = x_left; i2 <= x_right; i2++) {
      cave[i1][i2].fval = cur_floor.ftval;
      cave[i1][i2].fopen = cur_floor.ftopen;
    }
  }

  for (i1 = y_height - 1; i1 <= y_depth + 1; i1++) {
    cave[i1][x_left - 1].fval = rock_wall1.ftval;
    cave[i1][x_left - 1].fopen = rock_wall1.ftopen;
    cave[i1][x_right + 1].fval = rock_wall1.ftval;
    cave[i1][x_right + 1].fopen = rock_wall1.ftopen;
  }

  for (i1 = x_left; i1 <= x_right; i1++) {
    cave[y_height - 1][i1].fval = rock_wall1.ftval;
    cave[y_height - 1][i1].fopen = rock_wall1.ftopen;
    cave[y_depth + 1][i1].fval = rock_wall1.ftval;
    cave[y_depth + 1][i1].fopen = rock_wall1.ftopen;
  }

  /*{ The inner room		}*/
  y_height += 2;
  y_depth -= 2;
  x_left += 2;
  x_right -= 2;

  for (i1 = y_height - 1; i1 <= y_depth + 1; i1++) {
    cave[i1][x_left - 1].fval = 8;
    cave[i1][x_right + 1].fval = 8;
  }

  for (i1 = x_left; i1 <= x_right; i1++) {
    cave[y_height - 1][i1].fval = 8;
    cave[y_depth + 1][i1].fval = 8;
  }

  /*{ Inner room varitions		}*/
  switch (randint(5)) {
  case 1:                 /*	{ Just an inner room...	}*/
    switch (randint(4)) { /*{ Place a door	}*/
    case 1:
      place_secret_door(y_height - 1, xval);
      break;
    case 2:
      place_secret_door(y_depth + 1, xval);
      break;
    case 3:
      place_secret_door(yval, x_left - 1);
      break;
    case 4:
      place_secret_door(yval, x_right + 1);
      break;
    }
    gc__vault_monster(yval, xval, 1);
    break;

  case 2:                 /*	{ Treasure Vault	}*/
    switch (randint(4)) { /*{ Place a door	}*/
    case 1:
      place_secret_door(y_height - 1, xval);
      break;
    case 2:
      place_secret_door(y_depth + 1, xval);
      break;
    case 3:
      place_secret_door(yval, x_left - 1);
      break;
    case 4:
      place_secret_door(yval, x_right + 1);
      break;
    }

    for (i1 = yval - 1; i1 <= yval + 1; i1++) {
      cave[i1][xval - 1].fval = 8;
      cave[i1][xval + 1].fval = 8;
    }

    cave[yval - 1][xval].fval = 8;
    cave[yval + 1][xval].fval = 8;

    switch (randint(4)) { /*{ Place a door	}*/
    case 1:
      place_locked_door(yval - 1, xval);
      break;
    case 2:
      place_locked_door(yval + 1, xval);
      break;
    case 3:
      place_locked_door(yval, xval - 1);
      break;
    case 4:
      place_locked_door(yval, xval + 1);
      break;
    }

    /*{ Place an object in the treasure vault	}*/
    switch (randint(10)) {
    case 1:
      place_a_staircase(yval, xval, up_staircase);
      break;
    case 2:
      place_a_staircase(yval, xval, down_staircase);
      break;
    default:
      place_random_dungeon_item(yval, xval);
      break;
    }

    /*{ Guard the treasure well		}*/
    gc__vault_monster(yval, xval, 2 + randint(3));

    /*{ If the monsters don't get 'em...	}*/
    gc__vault_trap(yval, xval, 4, 10, 2 + randint(3));
    break;

  case 3:                 /*{ Inner pillar(s)...	}*/
    switch (randint(4)) { /*{ Place a door	}*/
    case 1:
      place_secret_door(y_height - 1, xval);
      break;
    case 2:
      place_secret_door(y_depth + 1, xval);
      break;
    case 3:
      place_secret_door(yval, x_left - 1);
      break;
    case 4:
      place_secret_door(yval, x_right + 1);
      break;
    }

    for (i1 = yval - 1; i1 <= yval + 1; i1++) {
      for (i2 = xval - 1; i2 <= xval + 1; i2++) {
        cave[i1][i2].fval = 8;
      }
    }

    if (randint(2) == 1) {
      switch (randint(2)) {
      case 1:
        for (i1 = yval - 1; i1 <= yval + 1; i1++) {
          for (i2 = xval - 6; i2 <= xval - 4; i2++) {
            cave[i1][i2].fval = 8;
          }
        }

        for (i1 = yval - 1; i1 <= yval + 1; i1++) {
          for (i2 = xval + 4; i2 <= xval + 6; i2++) {
            cave[i1][i2].fval = 8;
          }
        }
        break;

      case 2:
        for (i1 = yval - 1; i1 <= yval + 1; i1++) {
          for (i2 = xval - 7; i2 <= xval - 5; i2++) {
            cave[i1][i2].fval = 8;
          }
        }

        for (i1 = yval - 1; i1 <= yval + 1; i1++) {
          for (i2 = xval + 5; i2 <= xval + 7; i2++) {
            cave[i1][i2].fval = 8;
          }
        }
        break;
      }

      if (randint(3) == 1) { /*{ Inner rooms   }*/
        for (i1 = xval - 5; i1 <= xval + 5; i1++) {
          cave[yval - 1][i1].fval = 8;
          cave[yval + 1][i1].fval = 8;
        }

        switch (randint(2)) {
        case 1:
          place_secret_door(yval + 1, xval - 3);
          break;
        case 2:
          place_secret_door(yval - 1, xval - 3);
          break;
        }

        switch (randint(2)) {
        case 1:
          place_secret_door(yval + 1, xval + 3);
          break;
        case 2:
          place_secret_door(yval - 1, xval + 3);
          break;
        }

        if (randint(3) == 1) {
          place_random_dungeon_item(yval, xval - 2);
        }

        if (randint(3) == 1) {
          place_random_dungeon_item(yval, xval + 2);
        }

        gc__vault_monster(yval, xval - 2, randint(2));
        gc__vault_monster(yval, xval + 2, randint(2));
      }
    }
    break;

  case 4:                 /*  { Maze inside...        }*/
    switch (randint(4)) { /*{ Place a door  }*/
    case 1:
      place_secret_door(y_height - 1, xval);
      break;
    case 2:
      place_secret_door(y_depth + 1, xval);
      break;
    case 3:
      place_secret_door(yval, x_left - 1);
      break;
    case 4:
      place_secret_door(yval, x_right + 1);
      break;
    }

    for (i1 = y_height; i1 <= y_depth; i1++) {
      for (i2 = x_left; i2 <= x_right; i2++) {
        if (int_is_odd(i2 + i1)) {
          cave[i1][i2].fval = 8;
        }
      }
    }

    /*{ Monsters just love mazes...           }*/
    gc__vault_monster(yval, xval - 5, randint(3));
    gc__vault_monster(yval, xval + 5, randint(3));

    /*{ Traps make them entertaining...       }*/
    gc__vault_trap(yval, xval - 3, 2, 8, randint(3));
    gc__vault_trap(yval, xval + 3, 2, 8, randint(3));

    /*{ Mazes should have some treasure too.. }*/
    for (i1 = 1; i1 <= 3; i1++) {
      place_random_loot_near(yval, xval, 1);
    }
    break;

  case 5: /*   { Four small rooms...   }*/
    for (i1 = y_height; i1 <= y_depth; i1++) {
      cave[i1][xval].fval = 8;
    }

    for (i1 = x_left; i1 <= x_right; i1++) {
      cave[yval][i1].fval = 8;
    }

    switch (randint(2)) {
    case 1:
      i1 = randint(10);
      place_secret_door(y_height - 1, xval - i1);
      place_secret_door(y_height - 1, xval + i1);
      place_secret_door(y_depth + 1, xval - i1);
      place_secret_door(y_depth + 1, xval + i1);
      break;

    case 2:
      i1 = randint(3);
      place_secret_door(yval + i1, x_left - 1);
      place_secret_door(yval - i1, x_left - 1);
      place_secret_door(yval + i1, x_right + 1);
      place_secret_door(yval - i1, x_right + 1);
      break;
    }

    /*{ Treasure in each one...               }*/
    place_random_loot_near(yval, xval, 2 + randint(2));

    /*{ Gotta have some monsters...           }*/
    gc__vault_monster(yval + 2, xval - 4, randint(2));
    gc__vault_monster(yval + 2, xval + 4, randint(2));
    gc__vault_monster(yval - 2, xval - 4, randint(2));
    gc__vault_monster(yval - 2, xval + 4, randint(2));
    break;
  }
}

/* Builds a room at a row,column coordinate		-RAK-
    Type 3 unusual rooms are cross shaped	*/
void gc__build_type3(const int64_t yval, const int64_t xval) {

  int64_t i1, i2;
  floor_type cur_floor;

  if (dun_level <= randint(25)) {
    cur_floor = lopen_floor; /*{ Floor with light	}*/
  } else {
    cur_floor = dopen_floor; /*{ Dark floor		}*/
  }

  int64_t i0 = 2 + randint(2);
  int64_t y_height = yval - i0;
  int64_t y_depth = yval + i0;
  int64_t x_left = xval - 1;
  int64_t x_right = xval + 1;

  for (i1 = y_height; i1 <= y_depth; i1++) {
    for (i2 = x_left; i2 <= x_right; i2++) {
      cave[i1][i2].fval = cur_floor.ftval;
      cave[i1][i2].fopen = cur_floor.ftopen;
    }
  }

  for (i1 = y_height - 1; i1 <= y_depth + 1; i1++) {
    cave[i1][x_left - 1].fval = rock_wall1.ftval;
    cave[i1][x_left - 1].fopen = rock_wall1.ftopen;

    cave[i1][x_right + 1].fval = rock_wall1.ftval;
    cave[i1][x_right + 1].fopen = rock_wall1.ftopen;
  }

  for (i1 = x_left; i1 <= x_right; i1++) {
    cave[y_height - 1][i1].fval = rock_wall1.ftval;
    cave[y_height - 1][i1].fopen = rock_wall1.ftopen;

    cave[y_depth + 1][i1].fval = rock_wall1.ftval;
    cave[y_depth + 1][i1].fopen = rock_wall1.ftopen;
  }

  i0 = 2 + randint(9);
  y_height = yval - 1;
  y_depth = yval + 1;
  x_left = xval - i0;
  x_right = xval + i0;

  for (i1 = y_height; i1 <= y_depth; i1++) {
    for (i2 = x_left; i2 <= x_right; i2++) {
      cave[i1][i2].fval = cur_floor.ftval;
      cave[i1][i2].fopen = cur_floor.ftopen;
    }
  }

  for (i1 = y_height - 1; i1 <= y_depth + 1; i1++) {
    if (cave[i1][x_left - 1].fval != cur_floor.ftval) {
      cave[i1][x_left - 1].fval = rock_wall1.ftval;
      cave[i1][x_left - 1].fopen = rock_wall1.ftopen;
    }

    if (cave[i1][x_right + 1].fval != cur_floor.ftval) {
      cave[i1][x_right + 1].fval = rock_wall1.ftval;
      cave[i1][x_right + 1].fopen = rock_wall1.ftopen;
    }
  }

  for (i1 = x_left; i1 <= x_right; i1++) {

    if (cave[y_height - 1][i1].fval != cur_floor.ftval) {
      cave[y_height - 1][i1].fval = rock_wall1.ftval;
      cave[y_height - 1][i1].fopen = rock_wall1.ftopen;
    }

    if (cave[y_depth + 1][i1].fval != cur_floor.ftval) {
      cave[y_depth + 1][i1].fval = rock_wall1.ftval;
      cave[y_depth + 1][i1].fopen = rock_wall1.ftopen;
    }
  }

  /*{ Special features...			}*/
  switch (randint(4)) {
  case 1: /*{ Large middle pillar		}*/
    for (i1 = yval - 1; i1 <= yval + 1; i1++) {
      for (i2 = xval - 1; i2 <= xval + 1; i2++) {
        cave[i1][i2].fval = 8;
      }
    }
    break;

  case 2: /*{ Inner treasure vault		}*/
    for (i1 = yval - 1; i1 <= yval + 1; i1++) {
      cave[i1][xval - 1].fval = 8;
      cave[i1][xval + 1].fval = 8;
    }

    cave[yval - 1][xval].fval = 8;
    cave[yval + 1][xval].fval = 8;

    switch (randint(4)) { /*{ Place a door	}*/
    case 1:
      place_secret_door(yval - 1, xval);
      break;
    case 2:
      place_secret_door(yval + 1, xval);
      break;
    case 3:
      place_secret_door(yval, xval - 1);
      break;
    case 4:
      place_secret_door(yval, xval + 1);
      break;
    }

    /*{ Place a treasure in the vault		}*/
    place_random_dungeon_item(yval, xval);

    /*{ Let's gaurd the treasure well...	}*/
    gc__vault_monster(yval, xval, 2 + randint(2));

    /*{ Traps naturally			}*/
    gc__vault_trap(yval, xval, 4, 4, 1 + randint(3));
    break;

  case 3:
    if (randint(3) == 1) {
      cave[yval - 1][xval - 2].fval = 8;
      cave[yval + 1][xval - 2].fval = 8;
      cave[yval - 1][xval + 2].fval = 8;
      cave[yval - 1][xval + 2].fval = 8;
      cave[yval - 2][xval - 1].fval = 8;
      cave[yval - 2][xval + 1].fval = 8;
      cave[yval + 2][xval - 1].fval = 8;
      cave[yval + 2][xval + 1].fval = 8;
      if (randint(3) == 1) {
        place_secret_door(yval, xval - 2);
        place_secret_door(yval, xval + 2);
        place_secret_door(yval - 2, xval);
        place_secret_door(yval + 2, xval);
      }
    } else if (randint(3) == 1) {
      cave[yval][xval].fval = 8;
      cave[yval - 1][xval].fval = 8;
      cave[yval + 1][xval].fval = 8;
      cave[yval][xval - 1].fval = 8;
      cave[yval][xval + 1].fval = 8;
    } else if (randint(3) == 1) {
      cave[yval][xval].fval = 8;
    }
    break;

  case 4:
    break;
  }
}
