/* generate.c */
/**/
#include <curses.h>
#include <stdlib.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "../debug.h"
#include "../misc.h"
#include "../monsters.h"
#include "../pascal.h"
#include "../player.h"
#include "../random.h"
#include "../screen.h"
#include "../stores.h"
#include "../types.h"
#include "../variables.h"

#include "river.h"
#include "rooms.h"

#include "models.h"

/*
 *	{ Dungeon generation values					}
 *	{ Note: The entire design of dungeon can be changed by only	}
 *	{	slight adjustments here.				}
 */
#define DUN_TUN_RND 36  /*{ Random direction (4 is min)		} */
#define DUN_TUN_CHG 70  /*{ Chance of changing direction (99 max)} */
#define DUN_TUN_FND 12  /*{ Distance for auto find to kick in	} */
#define DUN_TUN_CON 15  /*{ Chance of extra tunneling		} */
#define DUN_ROO_MEA 32  /*{ Mean of # of rooms, standard dev=2	} */
#define DUN_TUN_PEN 25  /*{ % chance of room doors		} */
#define DUN_TUN_JCT 15  /*{ % chance of doors at tunnel junctons	} */
#define DUN_STR_DEN 5   /*{ Density of streamers			} */
#define DUN_STR_RNG 2   /*{ Width of streamers			} */
#define DUN_STR_MAG 3   /*{ Number of magma streamers		} */
#define DUN_STR_QUA 2   /*{ Number of quartz streamers		} */
#define DUN_WTR_DEN 5   /*{ Density of water			} */
#define DUN_WTR_WIDTH 4 /*{ Width of river			} */
#define DUN_RIVERS 3    /*{ Number of rivers			} */
#define DUN_RIV_LEN 35  /*{ Maximum river length			} */
#define DUN_POOLS 3     /*{ Number of pools			} */

// Town level
#define TOT_STORES (MAX_STORES + MAX_UNNAMED)
#define MAX_HOUSE1 4     /*{ # of generic houses in town	} */
#define MAX_HOUSE2 8     /*{ # of small houses in town } */
#define DAY_MUGGING 50   /*{ 1/x chance that page gets mugged (day)} */
#define NIGHT_MUGGING 15 /*{ 1/x chance that page gets mugged (night)} */

/**
 * gc__correct_dir() - Always picks a correct direction
 */
static void gc__correct_dir(long *rdir, long *cdir, long y1, long x1, long y2,
                            long x2) {

  if (y1 < y2)
    *rdir = 1;
  else if (y1 == y2)
    *rdir = 0;
  else
    *rdir = -1;

  if (x1 < x2)
    *cdir = 1;
  else if (x1 == x2)
    *cdir = 0;
  else
    *cdir = -1;

  if (rdir != 0 && cdir != 0) {
    if (randint(2) == 1) {
      *rdir = 0;
    } else {
      *cdir = 0;
    }
  }
}

/**
 * gc__rand_dir() - Chance of wandering direction
 */
static void gc__rand_dir(long *rdir, long *cdir, long y1, long x1, long y2,
                         long x2, long chance) {
  switch (randint(chance)) {
  case 1:
    *rdir = -1;
    *cdir = 0;
    break;
  case 2:
    *rdir = 1;
    *cdir = 0;
    break;
  case 3:
    *rdir = 0;
    *cdir = -1;
    break;
  case 4:
    *rdir = 0;
    *cdir = 1;
    break;
  default:
    gc__correct_dir(rdir, cdir, y1, x1, y2, x2);
    break;
  }
}

/**
 * gc__blank_cave() - Blanks out entire cave
 */
static void gc__blank_cave() {
  for (long y = 0; y <= MAX_HEIGHT; y++) {
    for (long x = 0; x <= MAX_WIDTH; x++) {
      cave[y][x] = blank_floor;
    }
  }
}

/**
 * -RAK-
 *  gc__fill_cave() - Fills in empty spots with desired rock
 *  Note: 9 is a temporary value
 */
static void gc__fill_cave(floor_type fill) {
  obj_set blank_floor_set = {0, 8, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};
  for (long y = 2; y <= cur_height - 1; y++) {
    for (long x = 2; x <= cur_width - 1; x++) {
      if (is_in(cave[y][x].fval, blank_floor_set)) {
        cave[y][x].fval = fill.ftval;
        cave[y][x].fopen = fill.ftopen;
      }
    }
  }
}

/**
 * -RAK-
 *  gc__place_boundry() Places indestructible rock arounds edges of dungeon
 */
static void gc__place_boundry() {
  for (long y = 1; y <= cur_height; y++) {
    cave[y][1].fval = boundry_wall.ftval;
    cave[y][1].fopen = boundry_wall.ftopen;
    cave[y][cur_width].fval = boundry_wall.ftval;
    cave[y][cur_width].fopen = boundry_wall.ftopen;
  }

  for (long x = 1; x <= cur_width; x++) {
    cave[1][x].fval = boundry_wall.ftval;
    cave[1][x].fopen = boundry_wall.ftopen;
    cave[cur_height][x].fval = boundry_wall.ftval;
    cave[cur_height][x].fopen = boundry_wall.ftopen;
  }
}

/**
 * -RAK-
 *  gc__place_streamer() - Places "streamers" of rock through dungeon
 */
static void gc__place_streamer(floor_type rock, long treas_chance) {
  /*{ Choose starting point and direction		}*/
  long y = (cur_height / 2.0) + 11 - randint(23);
  long x = (cur_width / 2.0) + 16 - randint(33);

  long dir = randint(8); /*{ Number 1-4, 6-9	}*/
  if (dir > 4) {
    dir++;
  }

  /*{ Place streamer into dungeon			}*/
  boolean flag = false; /*{ Set to true when y,x are out-of-bounds}*/
  const long t1 = 2 * DUN_STR_RNG + 1; /*{ Constants	}*/
  const long t2 = DUN_STR_RNG + 1;

  do {
    for (long _ = 1; _ < DUN_STR_DEN; _++) {
      long ty = y + randint(t1) - t2;
      long tx = x + randint(t1) - t2;
      if (in_bounds(ty, tx)) {
        if (cave[ty][tx].fval == rock_wall1.ftval) {
          cave[ty][tx].fval = rock.ftval;
          cave[ty][tx].fopen = rock.ftopen;
          if (randint(treas_chance) == 1) {
            place_gold(ty, tx);
          }
        }
      }
    }
    if (!(move_dir(dir, &y, &x))) {
      flag = true;
    }
  } while (!flag);
}

/**
 * -RAK-
 *  gc__tunnel() - Constructs a tunnel between two points
 */
static void gc__tunnel(long row1, long col1, long row2, long col2,
                       long *doorptr, coords *doorstk) {
  long row_dir;
  long col_dir;
  coords tunstk[1001];
  coords wallstk[1001];

  /*{ Note: 9 is a temporary value		}*/

  boolean stop_flag = false;
  boolean door_flag = false;
  long tunptr = 0;
  long wallptr = 0;
  gc__correct_dir(&row_dir, &col_dir, row1, col1, row2, col2);

  do {
    if (randint(100) > DUN_TUN_CHG) {
      gc__rand_dir(&row_dir, &col_dir, row1, col1, row2, col2, DUN_TUN_RND);
    }

    long tmp_row = row1 + row_dir;
    long tmp_col = col1 + col_dir;

    for (; !(in_bounds(tmp_row, tmp_col));) {
      gc__rand_dir(&row_dir, &col_dir, row1, col1, row2, col2, DUN_TUN_RND);
      tmp_row = row1 + row_dir;
      tmp_col = col1 + col_dir;
    }

    /* with cave[tmp_row][tmp_col]. do; */
    if (cave[tmp_row][tmp_col].fval == rock_wall1.ftval) {
      row1 = tmp_row;
      col1 = tmp_col;
      if (wallptr < 1000) {
        wallptr++;
      }
      wallstk[wallptr].y = row1;
      wallstk[wallptr].x = col1;
      for (long y = row1 - 1; y <= row1 + 1; y++) {
        for (long x = col1 - 1; x <= col1 + 1; x++) {
          if (!in_bounds(y, x))
            continue;
          if (!is_in(cave[y][x].fval, wall_set))
            continue;
          cave[y][x].fval = 9;
        }
      }

    } else if (cave[tmp_row][tmp_col].fval == corr_floor1.ftval) {
      row1 = tmp_row;
      col1 = tmp_col;
      if (!(door_flag)) {
        if (*doorptr <= 100) {
          (*doorptr)++;
          doorstk[*doorptr].y = row1;
          doorstk[*doorptr].x = col1;
        }
        door_flag = true;
      }
      if (randint(100) > DUN_TUN_CON) {
        stop_flag = true;
      }

    } else if (cave[tmp_row][tmp_col].fval == 0) {
      row1 = tmp_row;
      col1 = tmp_col;
      if (tunptr < 1000) {
        tunptr++;
      }
      tunstk[tunptr].y = row1;
      tunstk[tunptr].x = col1;
      door_flag = false;

    } else if (cave[tmp_row][tmp_col].fval != 9) {
      row1 = tmp_row;
      col1 = tmp_col;
    }
  } while (!(((row1 == row2) && (col1 == col2)) || (stop_flag)));

  for (long i = 1; i <= tunptr; i++) {
    cave[tunstk[i].y][tunstk[i].x].fval = corr_floor1.ftval;
    cave[tunstk[i].y][tunstk[i].x].fopen = corr_floor1.ftopen;
  }

  for (long i = 1; i <= wallptr; i++) {
    if (cave[wallstk[i].y][wallstk[i].x].fval == 9) {
      if (randint(100) < DUN_TUN_PEN) {
        place_door(wallstk[i].y, wallstk[i].x);
      } else {
        cave[wallstk[i].y][wallstk[i].x].fval = corr_floor2.ftval;
        cave[wallstk[i].y][wallstk[i].x].fopen = corr_floor2.ftopen;
      }
    }
  }
}

static boolean gc__next_to(long y, long x) {
  obj_set corridors = {4, 5, 6, 0};
  boolean next_to = false;

  if (next_to8(y, x, corridors) > 2) {
    if (is_in(cave[y - 1][x].fval, wall_set) &&
        is_in(cave[y + 1][x].fval, wall_set)) {
      next_to = true;
    } else if (is_in(cave[y][x - 1].fval, wall_set) &&
               is_in(cave[y][x + 1].fval, wall_set)) {
      next_to = true;
    } else {
      next_to = false;
    }
  } else {
    next_to = false;
  }

  return next_to;
}

/**
 * gc__try_door() - Places door at y,x position if at least 2 walls found
 */
static void gc__try_door(long y, long x) {
  if (randint(100) > DUN_TUN_JCT) {
    if (cave[y][x].fval == corr_floor1.ftval) {
      if (gc__next_to(y, x)) {
        place_door(y, x);
      }
    }
  }
}

/**
 * -DMF-
 *  gc__place_pool() - Place a pool of water, and rough up the edges
 */
static void gc__place_pool(floor_type water) {
  (void)water;
  /*
  long   y,x;

  y = (long)(cur_height/2.0) + 11 - randint(23);
  x = (long)(cur_width/2.0)  + 16 - randint(33);
  */

  /* XXXX place_pool does nothing useful */
}

static void gc__all_the_river_stuff() {
  all_the_river_stuff(); /* in river.c */
}

static void gc__cave_gen() {
  /*{ Cave logic flow for generation of new dungeon		}*/

  /*{ Following are variables that change with level of difficulty } */
  const long dun_str_mc = 95;      // 1/x chance of treasure per magma
  const long dun_str_qc = 55;      // 1/x chance of treasure per quartz
  const long dun_unusual = 100;    // Level/x chance of unusual room
  const long treas_room_alloc = 7; // Amount of objects for rooms
  const long treas_any_alloc = 2;  // Amount of objects for corridors
  const long treas_gold_alloc = 2; // Amount of gold (and gems)

  coords doorstk[101];
  boolean room_map[21][21]; /*: room_type;*/
  long doorptr = 0;
  short yloc[401]; /*: array [1..400] of short;*/
  short xloc[401]; /*: array [1..400] of short;*/

  obj_set allocSet1 = {1, 2, 0};       /* land mosnters */
  obj_set allocSet2 = {16, 17, 18, 0}; /* land mosnters */
  obj_set allocSet3 = {4, 0};          /* treasure things */
  obj_set allocSet4 = {1, 2, 0};       /* treasure things */
  obj_set allocSet5 = {1, 2, 4, 0};    /* treasure things */

  long i3 = 0;

  long row_rooms = 2 * (long)(cur_height / SCREEN_HEIGHT);
  long col_rooms = 2 * (long)(cur_width / SCREEN_WIDTH);

  for (long y = 1; y <= row_rooms; y++) {
    for (long x = 1; x <= col_rooms; x++) {
      room_map[y][x] = false;
    }
  }

  if (1) {
    for (long i = 1, roomCount = randnor(DUN_ROO_MEA, 2); i <= roomCount; i++) {
      room_map[randint(row_rooms)][randint(col_rooms)] = true;
    }
  } else {
    char_row = 17;
    char_col = 50;
    room_map[2][2] = true;
    room_map[2][3] = true;
  }

  for (long y = 1; y <= row_rooms; y++) {
    for (long i2 = 1; i2 <= col_rooms; i2++) {
      if (room_map[y][i2]) {
        i3++;
        yloc[i3] = (y - 1) * (quart_height * 2 + 1) + quart_height + 1;
        xloc[i3] = (i2 - 1) * (quart_width * 2 + 1) + quart_width + 1;
        if (dun_level > randint(dun_unusual)) {
          switch (randint(3)) {
          case 1:
            gc__build_type1(yloc[i3], xloc[i3]);
            break;
          case 2:
            gc__build_type2(yloc[i3], xloc[i3]);
            break;
          case 3:
            gc__build_type3(yloc[i3], xloc[i3]);
            break;
          }
        } else {
          gc__build_room(yloc[i3], xloc[i3]);
        }
      }
    }
  }

  for (long _ = 1; _ <= i3; _++) {
    long pick1 = randint(i3);
    long pick2 = randint(i3);
    long y1 = yloc[pick1];
    long x1 = xloc[pick1];
    yloc[pick1] = yloc[pick2];
    xloc[pick1] = xloc[pick2];
    yloc[pick2] = y1;
    xloc[pick2] = x1;
  }

  for (long i = 1; i < i3; i++) {
    long y1 = yloc[i];
    long x1 = xloc[i];
    long y2 = yloc[i + 1];
    long x2 = xloc[i + 1];
    gc__tunnel(y2, x2, y1, x1, &doorptr, doorstk);
  }

  gc__fill_cave(rock_wall1);

  for (long _ = 1; _ <= DUN_STR_MAG; _++) {
    gc__place_streamer(rock_wall2, dun_str_mc);
  }
  for (long _ = 1; _ <= DUN_STR_QUA; _++) {
    gc__place_streamer(rock_wall3, dun_str_qc);
  }

  gc__place_boundry();
  gc__all_the_river_stuff();

  for (long _ = 1; _ <= DUN_POOLS; _++) {
    gc__place_pool(water1);
  }

  /*{ Place intersection doors	}*/
  for (long i = 1; i <= doorptr; i++) {
    gc__try_door(doorstk[i].y, doorstk[i].x - 1);
    gc__try_door(doorstk[i].y, doorstk[i].x + 1);
    gc__try_door(doorstk[i].y - 1, doorstk[i].x);
    gc__try_door(doorstk[i].y + 1, doorstk[i].x);
  }

  long alloc_level = (dun_level / 3);
  if (alloc_level < 2) {
    alloc_level = 2;
  } else if (alloc_level > 10) {
    alloc_level = 10;
  }

  place_stairs(up_staircase, randint(2), 3);
  place_stairs(down_staircase, randint(2) + 2, 3);
  place_stairs(up_steep_staircase, 1, 3);
  place_stairs(down_steep_staircase, 1, 3);

  alloc_land_monster(allocSet1, (randint(8) + MIN_MALLOC_LEVEL + alloc_level),
                     0, true, false);
  alloc_land_monster(allocSet2,
                     (randint(8) + MIN_MALLOC_LEVEL + alloc_level) / 3, 0, true,
                     true);

  alloc_object(allocSet3, 3, randint(alloc_level));
  alloc_object(allocSet4, 5, randnor(treas_room_alloc, 3));
  alloc_object(allocSet5, 5, randnor(treas_any_alloc, 3));
  alloc_object(allocSet5, 4, randnor(treas_gold_alloc, 3));
  alloc_object(allocSet5, 1, randint(alloc_level));

  if (dun_level >= WIN_MON_APPEAR) {
    place_win_monster();
  }

  gc__place_boundry(); /* just to make sure */
}

static void gc__make_door(long y, long x, long *cur_pos, long store_num,
                          long house_type) {
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

static void dr_castle(long yval, long xval, long dy, long dx, floor_type ft) {
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
  } while (!((dy >= 0) && (dx >= 0)));
}

static void gc__blank_square(long dy, long dx) {
  cave[dy][dx].fopen = dopen_floor.ftopen;
  cave[dy][dx].fval = dopen_floor.ftval;
}

static void gc__build_store(enum store_t store_num, long where) {
  /*{ Builds a building at a row,column coordinate, and	}*/
  /*{ set up the initial contents by setting p1 to	}*/
  /*{ whatever inside type is desired			}*/

  long yval, y_height, y_depth;
  long xval, x_left, x_right;
  long i1, i2, q1, q2, cur_pos, house_type;

  yval = 10 * (where / 9) + 6;
  xval = 14 * (where % 9) + 11;

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

  if ((house_type == 1) || (house_type == 3)) {
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

static void gc__build_house(long house_num, long where) {
  gc__build_store(house_num + TOT_STORES - 1, where);
}

static void gc__build_fountain(long where) {
  /*{ Build a fountain at row, column coordinate	-dmf-	}*/

  long flr[36]; /*: array [1..35] of long;*/

  const long yval = (10 * (int)(where / 9)) + 4 + randint(3);
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
  } while (!(flr[i1] == 2) && (i1 != 18));

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

static void gc__mixem(long rooms[], long num) {
  for (long i1 = 0; i1 < num; i1++) {
    long i2 = i1 - 1 + randint(num - i1);
    long i3 = rooms[i1];
    rooms[i1] = rooms[i2];
    rooms[i2] = i3;
  }
}

static void gc__town_gen() {
  long rooms[36];       /* array [0..35] of long;*/
  boolean roomdone[36]; /* array [0..35] of boolean;*/

  obj_set allocSet1 = {1, 2, 0};
  obj_set allocSet2 = {16, 17, 18, 0};

  /*  for (i1 = 0; i1 < MAX_MONS_LEVEL+1; i1++) { */
  /*    printf ("\n m_level[%d] : %d",i1,m_level[i1]);  fflush(stdout); */
  /*  } */

  for (long i = 0; i < 36; i++) {
    roomdone[i] = false;
  }

  long center = 10 + randint(5);
  long i3 = 0;
  for (long i = -2; i <= 2; i++) {
    for (long j = -1; j <= 2; j++) {
      if (((i < 2) && (i > -2)) || ((j > -1) && (j < 2))) {
        roomdone[center + i + j * 9] = true;
        if ((i != 0) || (j == -1) || (j == 2)) { /*{not castle}*/
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
    if (!roomdone[i]) {
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

  gc__fill_cave(dopen_floor);

  long i1;
  long i2;
  do {
    i1 = randint(4);
    i2 = randint(4);
  } while (i1 == i2);

  gc__place_boundry();

  if ((player_cur_age.hour > 17) || (player_cur_age.hour < 6)) {
    /*{ Night	}*/

    mugging_chance = NIGHT_MUGGING;

    for (long y = 1; y <= cur_height; y++) {
      for (long x = 1; x <= cur_width; x++) {
        if (cave[y][x].fval != dopen_floor.ftval) {
          cave[y][x].pl = true;
        }
      }
    }

    place_stairs(down_staircase, 2, 0);
    place_stairs(down_steep_staircase, 1, 0);

    alloc_land_monster(allocSet1, MIN_MALLOC_TN, 3, true, false);
    alloc_land_monster(allocSet2, 7, 0, true, true);
    store_maint();

  } else {
    /*{ Day	}*/

    mugging_chance = DAY_MUGGING;
    for (long y = 1; y <= cur_height; y++) {
      for (long x = 1; x <= cur_width; x++) {
        cave[y][x].pl = true;
      }
    }

    place_stairs(down_staircase, 2, 0);
    place_stairs(down_steep_staircase, 1, 0);

    alloc_land_monster(allocSet1, MIN_MALLOC_TD, 3, true, false);
    alloc_land_monster(allocSet2, 7, 0, true, true);
    store_maint();
  }

  gc__place_boundry(); /* just to make sure */
}

void generate_cave() {
  /*{ Generates a random dungeon level			-RAK-	}*/

  panel_row_min = 0;
  panel_row_max = 0;
  panel_col_min = 0;
  panel_col_max = 0;
  char_row = -1;
  char_col = -1;

  tlink();
  mlink();

  gc__blank_cave();

  /*  for (i1 = 0; i1 < MAX_MONS_LEVEL+1; i1++) { */
  /*    printf ("\n m_level[%d] : %d",i1,m_level[i1]);  fflush(stdout); */
  /*  } */

  if (dun_level == 0) {
    cur_height = SCREEN_HEIGHT * 2;
    cur_width = SCREEN_WIDTH * 2;
    max_panel_rows = (cur_height / SCREEN_HEIGHT) * 2 - 2;
    max_panel_cols = (cur_width / SCREEN_WIDTH) * 2 - 2;
    panel_row = 0;
    panel_col = 0;
    gc__town_gen();
  } else {
    cur_height = MAX_HEIGHT;
    cur_width = MAX_WIDTH;
    max_panel_rows = (cur_height / SCREEN_HEIGHT) * 2 - 2;
    max_panel_cols = (cur_width / SCREEN_WIDTH) * 2 - 2;
    panel_row = max_panel_rows;
    panel_col = max_panel_cols;
    gc__cave_gen();
  }
}
