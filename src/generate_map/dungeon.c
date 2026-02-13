#include "../debug.h"
#include "../generate_monster/generate_monster.h"
#include "../generate_monster/monster_template.h"
#include "../loot/loot.h"
#include "../misc.h"
#include "../pascal.h"
#include "../player.h"
#include "../random.h"
#include "../screen.h"
#include "../types.h"
#include "../variables.h"
#include "config.h"
#include "misc.h"
#include "river.h"
#include "rooms.h"

#include <curses.h>
#include <stdlib.h>
#include <unistd.h> /* for ftruncate, usleep */

/**
 * gc__correct_dir() - Always picks a correct direction
 */
static void gc__correct_dir(long *rdir, long *cdir, const long y1,
                            const long x1, const long y2, const long x2) {

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
static void gc__rand_dir(long *rdir, long *cdir, const long y1, const long x1,
                         const long y2, const long x2, const long chance) {
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
 * -RAK-
 *  gc__place_streamer() - Places "streamers" of rock through dungeon
 */
static void gc__place_streamer(const floor_type rock, const long treas_chance) {
  /*{ Choose starting point and direction		}*/
  long y = cur_height / 2.0 + 11 - randint(23);
  long x = cur_width / 2.0 + 16 - randint(33);

  long dir = randint(8); /*{ Number 1-4, 6-9	}*/
  if (dir > 4) {
    dir++;
  }

  /*{ Place streamer into dungeon			}*/
  bool is_out_of_bounds = false;
  const long t1 = 2 * DUN_STR_RNG + 1; /*{ Constants	}*/
  const long t2 = DUN_STR_RNG + 1;

  while (!is_out_of_bounds) {
    for (long _ = 1; _ < DUN_STR_DEN; _++) {
      const long ty = y + randint(t1) - t2;
      const long tx = x + randint(t1) - t2;
      if (in_bounds(ty, tx)) {
        if (cave[ty][tx].fval == wall_granite.ftval) {
          cave[ty][tx].fval = rock.ftval;
          cave[ty][tx].fopen = rock.ftopen;
          if (randint(treas_chance) == 1) {
            place_gold(ty, tx);
          }
        }
      }
    }
    if (!move_dir(dir, &y, &x)) {
      is_out_of_bounds = true;
    }
  }
}

/**
 * -RAK-
 *  gc__tunnel() - Constructs a tunnel between two points
 */
static void gc__tunnel(long row1, long col1, const long row2, const long col2,
                       long *doorptr, coords *doorstk) {
  long row_dir;
  long col_dir;
  coords tunstk[1001];
  coords wallstk[1001];

  /*{ Note: 9 is a temporary value		}*/

  bool stop_flag = false;
  bool door_flag = false;
  long tunptr = 0;
  long wallptr = 0;
  gc__correct_dir(&row_dir, &col_dir, row1, col1, row2, col2);

  do {
    if (randint(100) > DUN_TUN_CHG) {
      gc__rand_dir(&row_dir, &col_dir, row1, col1, row2, col2, DUN_TUN_RND);
    }

    long tmp_row = row1 + row_dir;
    long tmp_col = col1 + col_dir;

    while (!in_bounds(tmp_row, tmp_col)) {
      gc__rand_dir(&row_dir, &col_dir, row1, col1, row2, col2, DUN_TUN_RND);
      tmp_row = row1 + row_dir;
      tmp_col = col1 + col_dir;
    }

    if (cave[tmp_row][tmp_col].fval == ft_wall_granite) {
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

    } else if (cave[tmp_row][tmp_col].fval == ft_corr_open_floor) {
      row1 = tmp_row;
      col1 = tmp_col;
      if (!door_flag) {
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
  } while (!((row1 == row2 && col1 == col2) || stop_flag));

  for (long i = 1; i <= tunptr; i++) {
    cave[tunstk[i].y][tunstk[i].x].fval = corr_open_floor.ftval;
    cave[tunstk[i].y][tunstk[i].x].fopen = corr_open_floor.ftopen;
  }

  for (long i = 1; i <= wallptr; i++) {
    if (cave[wallstk[i].y][wallstk[i].x].fval == 9) {
      if (randint(100) < DUN_TUN_PEN) {
        place_random_door(wallstk[i].y, wallstk[i].x);
      } else {
        cave[wallstk[i].y][wallstk[i].x].fval = corr_room_junction.ftval;
        cave[wallstk[i].y][wallstk[i].x].fopen = corr_room_junction.ftopen;
      }
    }
  }
}

static bool gc__next_to(const long y, const long x) {
  obj_set corridors = {4, 5, 6, 0};
  bool next_to = false;

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
static void gc__try_door(const long y, const long x) {
  if (randint(100) > DUN_TUN_JCT) {
    if (cave[y][x].fval == corr_open_floor.ftval) {
      if (gc__next_to(y, x)) {
        place_random_door(y, x);
      }
    }
  }
}

/**
 * -DMF-
 *  gc__place_pool() - Place a pool of water, and rough up the edges
 */
static void gc__place_pool(const floor_type water) {
  (void)water;
  /*
  long   y,x;

  y = (long)(cur_height/2.0) + 11 - randint(23);
  x = (long)(cur_width/2.0)  + 16 - randint(33);
  */

  /* XXXX place_pool does nothing useful */
}

static void gc__place_win_monster(void) {

  long cur_pos;

  if (!total_winner) {
    long x;
    long y;
    popm(&cur_pos);
    /* with m_list[cur_pos] do; */
    do {
      y = randint(cur_height - 2) + 1;
      x = randint(cur_width - 2) + 1;
    } while (!(is_in(cave[y][x].fval, open_dry_floors) &&
               cave[y][x].cptr == 0 && cave[y][x].tptr == 0 &&
               distance(y, x, char_row, char_col) > MAX_SIGHT));

    m_list[cur_pos].fy = y;
    m_list[cur_pos].fx = x;

    m_list[cur_pos].mptr =
        randint(WIN_MON_TOT) + m_level[MAX_MONS_LEVEL] + m_level[0];
    m_list[cur_pos].nptr = muptr;
    muptr = cur_pos;

    if ((monster_templates[m_list[cur_pos].mptr].cdefense & 0x4000) != 0) {
      m_list[cur_pos].hp = max_hp(monster_templates[m_list[cur_pos].mptr].hit_die);
    } else {
      m_list[cur_pos].hp = damroll(monster_templates[m_list[cur_pos].mptr].hit_die);
    }

    m_list[cur_pos].cdis = distance(char_row, char_col, y, x);
    m_list[cur_pos].cspeed =
        monster_templates[m_list[cur_pos].mptr].speed + player_flags.speed;
    m_list[cur_pos].stunned = 0;
    m_list[cur_pos].csleep = 0;
    cave[y][x].cptr = cur_pos;
  }
}

void generate_dungeon(void) {
  coords doorstk[101];
  bool room_map[21][21]; /*: room_type;*/
  long doorptr = 0;
  short yloc[401]; /*: array [1..400] of short;*/
  short xloc[401]; /*: array [1..400] of short;*/


  const long row_rooms = 2 * (cur_height / SCREEN_HEIGHT);
  const long col_rooms = 2 * (cur_width / SCREEN_WIDTH);

  for (long y = 1; y <= row_rooms; y++) {
    for (long x = 1; x <= col_rooms; x++) {
      room_map[y][x] = false;
    }
  }

  for (long i = 1, roomCount = randnor(DUN_ROO_MEA, 2); i <= roomCount; i++) {
    room_map[randint(row_rooms)][randint(col_rooms)] = true;
  }

  long num_rooms_created = 0;
  for (long y = 1; y <= row_rooms; y++) {
    for (long x = 1; x <= col_rooms; x++) {
      if (room_map[y][x]) {
        num_rooms_created++;
        const long current_room = num_rooms_created;
        yloc[current_room] = (y - 1) * (quart_height * 2 + 1) + quart_height + 1;
        xloc[current_room] = (x - 1) * (quart_width * 2 + 1) + quart_width + 1;
        if (dun_level > randint(dun_unusual)) {
          switch (randint(3)) {
          case 1:
            gc__build_type1(yloc[current_room], xloc[current_room]);
            break;
          case 2:
            gc__build_type2(yloc[current_room], xloc[current_room]);
            break;
          case 3:
            gc__build_type3(yloc[current_room], xloc[current_room]);
            break;
          }
        } else {
          gc__build_room(yloc[current_room], xloc[current_room]);
        }
      }
    }
  }

  for (long _ = 1; _ <= num_rooms_created; _++) {
    const long pick1 = randint(num_rooms_created);
    const long pick2 = randint(num_rooms_created);
    const long y1 = yloc[pick1];
    const long x1 = xloc[pick1];
    yloc[pick1] = yloc[pick2];
    xloc[pick1] = xloc[pick2];
    yloc[pick2] = y1;
    xloc[pick2] = x1;
  }

  for (long i = 1; i < num_rooms_created; i++) {
    const long y1 = yloc[i];
    const long x1 = xloc[i];
    const long y2 = yloc[i + 1];
    const long x2 = xloc[i + 1];
    gc__tunnel(y2, x2, y1, x1, &doorptr, doorstk);
  }

  fill_cave(wall_granite);

  for (long _ = 1; _ <= DUN_STR_MAG; _++) {
    gc__place_streamer(wall_magma, dun_str_mc);
  }
  for (long _ = 1; _ <= DUN_STR_QUA; _++) {
    gc__place_streamer(wall_quartz, dun_str_qc);
  }

  place_boundry();
  generate_rivers();

  for (long _ = 1; _ <= DUN_POOLS; _++) {
    gc__place_pool(water_on_floor);
  }

  /*{ Place intersection doors	}*/
  for (long i = 1; i <= doorptr; i++) {
    gc__try_door(doorstk[i].y, doorstk[i].x - 1);
    gc__try_door(doorstk[i].y, doorstk[i].x + 1);
    gc__try_door(doorstk[i].y - 1, doorstk[i].x);
    gc__try_door(doorstk[i].y + 1, doorstk[i].x);
  }

  long alloc_level = dun_level / 3;
  if (alloc_level < 2) {
    alloc_level = 2;
  } else if (alloc_level > 10) {
    alloc_level = 10;
  }

  try_to_place_stairs(up_staircase, randint(2), 3);
  try_to_place_stairs(down_staircase, randint(2) + 2, 3);
  try_to_place_stairs(up_steep_staircase, 1, 3);
  try_to_place_stairs(down_steep_staircase, 1, 3);

  obj_set land_tiles = {ft_dark_open_floor, ft_light_open_floor, 0};       /* land monsters */
  generate_land_monster(land_tiles, randint(8) + MIN_MALLOC_LEVEL + alloc_level,
                     0, true);
  generate_water_monster(water_set,
                     (randint(8) + MIN_MALLOC_LEVEL + alloc_level) / 3, 0, true);

  obj_set allocSet3 = {4, 0};          /* treasure things */
  alloc_object(allocSet3, 3, randint(alloc_level));
  obj_set allocSet4 = {1, 2, 0};       /* treasure things */
  alloc_object(allocSet4, 5, randnor(treas_room_alloc, 3));
  obj_set allocSet5 = {1, 2, 4, 0};    /* treasure things */
  alloc_object(allocSet5, 5, randnor(treas_any_alloc, 3));
  alloc_object(allocSet5, 4, randnor(treas_gold_alloc, 3));
  alloc_object(allocSet5, 1, randint(alloc_level));

  if (dun_level >= WIN_MON_APPEAR) {
    gc__place_win_monster();
  }

  place_boundry(); /* just to make sure */
}

