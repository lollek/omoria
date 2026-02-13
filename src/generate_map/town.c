#include "town.h"
#include "../debug.h"
#include "../generate_monster/generate_monster.h"
#include "../loot/loot.h"
#include "../misc.h"
#include "../player.h"
#include "../random.h"
#include "../screen.h"
#include "../stores.h"
#include "../types.h"
#include "../variables.h"
#include "config.h"
#include "misc.h"

#include <stdlib.h>
#include <unistd.h> /* for ftruncate, usleep */

static void gc__make_door(const long y, const long x, long *cur_pos,
                          const long store_num, const long house_type) {
  cave[y][x].fval = corr_floor3.ftval;
  cave[y][x].fopen = corr_floor3.ftopen;
  popt(cur_pos);
  cave[y][x].tptr = *cur_pos;

  if (store_num < TOT_STORES) {
    t_list[*cur_pos] = store_door[store_num];
  } else {
    t_list[*cur_pos] = store_door[TOT_STORES];
  }

  usleep(5);

  if (store_num >= TOT_STORES) {
    /* with t_list[cur_pos] do; */
    switch (house_type) {
    case 1:
      t_list[*cur_pos].p1 = 8 + randint(4);
      break; /*  9..12  */
    case 2:
      t_list[*cur_pos].p1 = randint(10);
      break; /*  1..10  */
    case 3:
      t_list[*cur_pos].p1 = 3 + randint(6);
      break; /*  4..9   */
    case 4:
      t_list[*cur_pos].p1 = randint(7);
      break; /*  1..7   */
    case 5:
      t_list[*cur_pos].p1 = 1;
      break; /*  1      */
    default:
      break;
    }
  }
}

static void dr_castle(const long yval, const long xval, long dy, long dx,
                      const floor_type ft) {
  /*{ for castle--changes all in both lines of symmetry }*/

  dx = labs(dx);
  dy = labs(dy);

  do {
    dy = -dy;
    if (dy <= 0) {
      dx = -dx;
    }
    cave[yval + dy][xval + dx].fopen = ft.ftopen;
    cave[yval + dy][xval + dx].fval = ft.ftval;
  } while (!(dy >= 0 && dx >= 0));
}

static void gc__blank_square(const long dy, const long dx) {
  cave[dy][dx].fopen = dopen_floor.ftopen;
  cave[dy][dx].fval = dopen_floor.ftval;
}

static void gc__build_store(const enum store_t store_num, const long where) {
  /*{ Builds a building at a row,column coordinate, and	}*/
  /*{ set up the initial contents by setting p1 to	}*/
  /*{ whatever inside type is desired			}*/

  long y_height, y_depth;
  long x_left, x_right;
  long i1, i2, cur_pos, house_type;

  long yval = 10 * (where / 9) + 6;
  long xval = 14 * (where % 9) + 11;

  if (store_num == S_FORTRESS) {
    /* whats this? 5 and 1 ? */
    house_type = 5;
    house_type = 1;
  } else if (store_num == S_BLACK_MARKET) {
    house_type = randint(2);
  } else if (store_num >= TOT_STORES) {
    house_type = store_num - TOT_STORES + 1;
  } else {
    house_type = 0;
  }

  if (house_type == 1 || house_type == 3) {
    y_height = yval - 1 - randint(2);
    y_depth = yval + 1 + randint(3);
    x_left = xval - 1 - randint(4);
    x_right = xval + 2 + randint(3);
  } else if (house_type == 2) {
    yval = yval - 2 + randint(3);
    xval = xval - 3 + randint(4);
    y_height = yval - randint(2);
    y_depth = yval + randint(3);
    x_left = xval - randint(3);
    x_right = xval + randint(4);
  } else if (house_type == 5) {
    yval = yval + 5;
    y_height = yval - 3;
    y_depth = yval + 3;
    x_left = xval - 5;
    x_right = xval + 5;
  } else {
    y_height = yval - 3;
    y_depth = yval + 3;
    x_left = xval - 5;
    x_right = xval + 5;
  }

  for (i1 = y_height; i1 <= y_depth; i1++) {
    for (i2 = x_left; i2 <= x_right; i2++) {
      cave[i1][i2].fval = boundry_wall.ftval;
      cave[i1][i2].fopen = boundry_wall.ftopen;
    }
  }

  if (house_type == 4) {
    for (i2 = x_left; i2 <= x_right; i2++) {
      cave[yval][i2].fval = dopen_floor.ftval;
      cave[yval][i2].fopen = dopen_floor.ftopen;
    }
  }

  if (house_type == 5) {
    dr_castle(yval, xval, 0, 5, water1);
    dr_castle(yval, xval, 1, 5, water1);
    dr_castle(yval, xval, 1, 6, water1);
    dr_castle(yval, xval, 2, 6, water1);
    dr_castle(yval, xval, 2, 7, water1);
    dr_castle(yval, xval, 3, 7, water1);
    dr_castle(yval, xval, 4, 7, water1);
    dr_castle(yval, xval, 4, 6, water1);
    dr_castle(yval, xval, 5, 6, water1);
    dr_castle(yval, xval, 5, 5, water1);
    dr_castle(yval, xval, 5, 4, water1);
    dr_castle(yval, xval, 5, 3, water1);
    dr_castle(yval, xval, 4, 3, water1);
    dr_castle(yval, xval, 4, 2, water1);
    dr_castle(yval, xval, 3, 2, water1);
    dr_castle(yval, xval, 3, 1, water1);
    dr_castle(yval, xval, 3, 0, water1);
    dr_castle(yval, xval, 3, 6, boundry_wall);
    dr_castle(yval, xval, 4, 5, boundry_wall);
    dr_castle(yval, xval, 4, 4, boundry_wall);
  }

  if (house_type == 3) {
    gc__make_door(y_height, xval - 2 + randint(4), &cur_pos, store_num,
                  house_type);
    gc__make_door(y_depth, xval - 2 + randint(4), &cur_pos, store_num,
                  house_type);
    gc__make_door(yval - 2 + randint(3), x_left, &cur_pos, store_num,
                  house_type);
    gc__make_door(yval - 2 + randint(3), x_right, &cur_pos, store_num,
                  house_type);
  } else if (house_type == 4) {
    gc__make_door(yval - 1, xval - 4, &cur_pos, store_num, house_type);
    gc__make_door(yval + 1, xval - 4, &cur_pos, store_num, house_type);
    gc__make_door(yval - 1, xval + 4, &cur_pos, store_num, house_type);
    gc__make_door(yval + 1, xval + 4, &cur_pos, store_num, house_type);
    gc__make_door(yval - 1, xval, &cur_pos, store_num, house_type);
    gc__make_door(yval + 1, xval, &cur_pos, store_num, house_type);
  } else if (house_type == 5) {
    i1 = 2 * randint(2) - 3;
    gc__make_door(yval + 2 * i1, xval - 1, &cur_pos, store_num, house_type);
    gc__make_door(yval + 2 * i1, xval, &cur_pos, store_num, house_type);
    gc__make_door(yval + 2 * i1, xval + 1, &cur_pos, store_num, house_type);
    gc__blank_square(yval + 3 * i1, xval - 1);
    gc__blank_square(yval + 3 * i1, xval + 0);
    gc__blank_square(yval + 3 * i1, xval + 1);
  } else {
    long q1;
    long q2;
    switch (randint(4)) {
    case 1:
      i1 = randint(y_depth - y_height) + y_height - 1;
      i2 = x_left;
      q1 = randint(y_depth - y_height) + y_height - 1;
      q2 = x_right;
      break;
    case 2:
      i1 = randint(y_depth - y_height) + y_height - 1;
      i2 = x_right;
      q1 = randint(y_depth - y_height) + y_height - 1;
      q2 = x_left;
      break;
    case 3:
      i1 = y_depth;
      i2 = randint(x_right - x_left) + x_left - 1;
      q1 = y_height;
      q2 = randint(x_right - x_left) + x_left - 1;
      break;
    case 4:
      i1 = y_height;
      i2 = randint(x_right - x_left) + x_left - 1;
      q1 = y_depth;
      q2 = randint(x_right - x_left) + x_left - 1;
      break;
    default:
      MSG(("randint returned something other than 1-4 in "
           "gc__build_store"));
      i1 = i2 = q1 = q2 = 0;
      break;
    }
    gc__make_door(i1, i2, &cur_pos, store_num, house_type);
    if (store_num == S_BANK) {
      gc__make_door(q1, q2, &cur_pos, S_INSURANCE, house_type);
    }
  }
}

static void gc__build_house(const long house_num, const long where) {
  gc__build_store(house_num + TOT_STORES - 1, where);
}

static void gc__build_fountain(const long where) {
  /*{ Build a fountain at row, column coordinate	-dmf-	}*/

  long flr[36]; /*: array [1..35] of long;*/

  const long yval = 10 * (int)(where / 9) + 4 + randint(3);
  const long xval = 14 * (where % 9) + 9 + randint(3);

  for (long i = 1; i <= 35; i++) {
    flr[i] = 2;
  }
  flr[1] = 1;
  flr[7] = 1;
  for (long i = 10; i <= 12; i++) {
    flr[i] = 3;
  }
  for (long i = 16; i <= 17; i++) {
    flr[i] = 3;
  }
  for (long i = 19; i <= 20; i++) {
    flr[i] = 3;
  }
  for (long i = 24; i <= 26; i++) {
    flr[i] = 3;
  }
  flr[29] = 1;
  flr[35] = 1;

  const long y_height = yval - 2;
  const long y_depth = yval + 2;
  const long x_left = xval - 3;
  const long x_right = xval + 3;

  long count = 0;
  long i1;
  do {
    i1 = randint(35);
  } while (flr[i1] != 2 && i1 != 18);

  flr[i1] = 4;
  for (long y = y_height; y <= y_depth; y++) {
    for (long x = x_left; x <= x_right; x++) {
      count++;
      switch (flr[count]) {
      case 1:
        cave[y][x].fval = dopen_floor.ftval;
        cave[y][x].fopen = dopen_floor.ftopen;
        break;
      case 2:
        cave[y][x].fval = boundry_wall.ftval;
        cave[y][x].fopen = boundry_wall.ftopen;
        break;
      case 3:
        cave[y][x].fval = water1.ftval;
        cave[y][x].fopen = water1.ftopen;
        usleep(5);
        if (randint(12) == 1) {
          place_gold(y, x);
        }
        break;
      case 4:
        cave[y][x].fval = rock_wall2.ftval;
        cave[y][x].fopen = rock_wall2.ftopen;
        break;
      }
    }
  }
}

static void gc__mixem(long rooms[], const long num) {
  for (long i1 = 0; i1 < num; i1++) {
    const long i2 = i1 - 1 + randint(num - i1);
    const long i3 = rooms[i1];
    rooms[i1] = rooms[i2];
    rooms[i2] = i3;
  }
}


void generate_town(void) {
  long rooms[36];
  bool room_is_done[36];

  for (long i = 0; i < 36; i++) {
    room_is_done[i] = false;
  }

  const long center = 10 + randint(5);
  long i3 = 0;
  for (long i = -2; i <= 2; i++) {
    for (long j = -1; j <= 2; j++) {
      if ((i < 2 && i > -2) || (j > -1 && j < 2)) {
        room_is_done[center + i + j * 9] = true;
        if (i != 0 || j == -1 || j == 2) { /*{not castle}*/
          rooms[i3] = center + i + j * 9;
          i3++;
        }
      }
    }
  }

  gc__mixem(rooms, i3);
  gc__build_store(S_TEMPLE, rooms[0]);   /*  3 */
  gc__build_store(S_ALCHEMY, rooms[1]);  /*  4 */
  gc__build_store(S_MAGIC, rooms[2]);    /*  5 */
  gc__build_store(S_LIBRARY, rooms[3]);  /*  7 */
  gc__build_store(S_MUSIC, rooms[4]);    /*  8 */
  gc__build_store(S_GEM, rooms[5]);      /*  9 */
  gc__build_store(S_BANK, rooms[6]);     /* 14 */
  gc__build_store(S_FORTRESS, rooms[7]); /* 17 */
  gc__build_fountain(rooms[8]);
  gc__build_fountain(rooms[9]);
  gc__build_store(S_BLACK_MARKET, rooms[10]); /* 11 */

  for (long i = 2; i <= MAX_HOUSE1; i++) {
    gc__build_house(1, rooms[9 + i]);
  }

  i3 = 0;
  for (long i = 0; i < 36; i++) {
    if (!room_is_done[i]) {
      rooms[i3] = i;
      i3++;
    }
  }

  gc__mixem(rooms, i3);
  gc__build_store(S_GENERAL, rooms[0]);    /*  0 */
  gc__build_store(S_ARMORY, rooms[1]);     /*  1 */
  gc__build_store(S_WEAPONS, rooms[2]);    /*  2 */
  gc__build_store(S_INN, rooms[3]);        /*  6 */
  gc__build_store(S_TRADE_POST, rooms[4]); /* 12 */
  gc__build_store(S_CASINO, rooms[5]);     /* 16 */
  gc__build_store(S_DELI, rooms[6]);       /* 10 */
  gc__build_fountain(rooms[8]);
  gc__build_fountain(rooms[9]);
  gc__build_house(4, rooms[10]);

  for (long i = 1; i <= MAX_HOUSE2; i++) {
    gc__build_house(2, rooms[10 + i]);
  }

  fill_cave(dopen_floor);

  long i1;
  long i2;
  do {
    i1 = randint(4);
    i2 = randint(4);
  } while (i1 == i2);

  place_boundry();

  obj_set floor_tiles = {1, 2, 0};
  obj_set water_tiles = {16, 17, 18, 0};
  if (player_cur_age.hour > 17 || player_cur_age.hour < 6) {
    /*{ Night	}*/

    mugging_chance = NIGHT_MUGGING;

    for (long y = 1; y <= cur_height; y++) {
      for (long x = 1; x <= cur_width; x++) {
        if (cave[y][x].fval != dopen_floor.ftval) {
          cave[y][x].pl = true;
        }
      }
    }

    try_to_place_stairs(down_staircase, 2, 0);
    try_to_place_stairs(down_steep_staircase, 1, 0);

    generate_land_monster(floor_tiles, MIN_MALLOC_TN, 3, true);
    generate_water_monster(water_tiles, 7, 0, true);
    store_maint();

  } else {
    /*{ Day	}*/

    mugging_chance = DAY_MUGGING;
    for (long y = 1; y <= cur_height; y++) {
      for (long x = 1; x <= cur_width; x++) {
        cave[y][x].pl = true;
      }
    }

    try_to_place_stairs(down_staircase, 2, 0);
    try_to_place_stairs(down_steep_staircase, 1, 0);

    generate_land_monster(floor_tiles, MIN_MALLOC_TD, 3, true);
    generate_water_monster(water_tiles, 7, 0, true);
    store_maint();
  }

  place_boundry(); /* just to make sure */
}

