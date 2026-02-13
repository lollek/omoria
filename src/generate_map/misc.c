#include "misc.h"
#include "../misc.h"
#include "../pascal.h"
#include "../random.h"
#include "../types.h"
#include "../variables.h"

void place_boundry(void) {
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

void try_to_place_stairs(const long stairs_type, const long number_of_stairs,
                         int number_of_adjacent_walls_to_place_stairs) {
  for (long i = 0; i < number_of_stairs; i++) {
    bool has_placed_staircase = false;
    while (!has_placed_staircase) {
      for (int attempts = 0; attempts < 30 && !has_placed_staircase;
           attempts++) {
        const long initial_y = randint(cur_height - 12);
        const long initial_x = randint(cur_width - 12);
        const long max_y = initial_y + 12;
        const long max_x = initial_x + 12;
        for (long y = initial_y; y <= max_y && !has_placed_staircase; y++) {
          for (long x = initial_x; x <= max_x && !has_placed_staircase; x++) {
            if (is_in(cave[y][x].fval, open_dry_floors)) {
              if (cave[y][x].tptr == 0) {
                if (next_to4(y, x, wall_set) >=
                    number_of_adjacent_walls_to_place_stairs) {
                  has_placed_staircase = true;
                  place_a_staircase(y, x, stairs_type);
                }
              }
            }
          }
        }
      }
      number_of_adjacent_walls_to_place_stairs--;
    }
  }
}

void fill_cave(const floor_type fill) {
  const obj_set blank_floor_set = {8, 9, 0};
  for (long y = 2; y <= cur_height - 1; y++) {
    for (long x = 2; x <= cur_width - 1; x++) {
      const uint8_t fval = cave[y][x].fval;
      if (fval == 0 || is_in(fval, blank_floor_set)) {
        cave[y][x].fval = fill.ftval;
        cave[y][x].fopen = fill.ftopen;
      }
    }
  }
}

/**
 * gc__blank_cave() - Blanks out entire cave
 */
void blank_out_map(void) {
  for (long y = 0; y <= MAX_HEIGHT; y++) {
    for (long x = 0; x <= MAX_WIDTH; x++) {
      cave[y][x] = blank_floor;
    }
  }
}

void place_a_staircase(const long y, const long x, const long staircase_type) {
  /*{ Place an up staircase at given y,x			-RAK-	}*/

  long cur_pos;

  static const treasure_type up_stair = {"an up staircase",
                                         up_staircase,
                                         0x00000000,
                                         0x00000000,
                                         0,
                                         0,
                                         1,
                                         0,
                                         0,
                                         0,
                                         0,
                                         0,
                                         0,
                                         "1d1",
                                         0,
                                         0};
  static const treasure_type down_stair = {"a down staircase",
                                           down_staircase,
                                           0x00000000,
                                           0x00000000,
                                           0,
                                           0,
                                           1,
                                           0,
                                           0,
                                           0,
                                           0,
                                           0,
                                           0,
                                           "1d1",
                                           0,
                                           0};
  static const treasure_type up_steep = {"a steep staircase",
                                         up_steep_staircase,
                                         0x00000000,
                                         0x00000000,
                                         0,
                                         0,
                                         1,
                                         0,
                                         0,
                                         0,
                                         0,
                                         0,
                                         0,
                                         "1d1",
                                         0,
                                         0};
  static const treasure_type down_steep = {"a steep staircase",
                                           down_steep_staircase,
                                           0x00000000,
                                           0x00000000,
                                           0,
                                           0,
                                           1,
                                           0,
                                           0,
                                           0,
                                           0,
                                           0,
                                           0,
                                           "1d1",
                                           0,
                                           0};

  if (cave[y][x].tptr != 0) {
    pusht(cave[y][x].tptr);
    cave[y][x].tptr = 0;
    cave[y][x].fopen = true;
  }

  popt(&cur_pos);
  cave[y][x].tptr = cur_pos;
  switch (staircase_type) {
  case up_staircase:
    t_list[cur_pos] = up_stair;
    break;
  case down_staircase:
    t_list[cur_pos] = down_stair;
    break;
  case up_steep_staircase:
    t_list[cur_pos] = up_steep;
    break;
  case down_steep_staircase:
    t_list[cur_pos] = down_steep;
    break;
  }
}

void place_random_door(const long y, const long x) {

  switch (randint(3)) {
  case 1:
    switch (randint(4)) {
    case 1:
      place_broken_door(y, x);
      break;
    default:
      place_open_door(y, x);
      break;
    }
    break;

  case 2:
    switch (randint(12)) {
    case 1:
    case 2:
      place_locked_door(y, x);
      break;
    case 3:
      place_stuck_door(y, x);
      break;
    default:
      place_closed_door(y, x);
      break;
    }
    break;

  case 3:
    place_secret_door(y, x);
    break;
  }
}

void place_open_door(const long y, const long x) {
  long cur_pos;

  popt(&cur_pos);

  cave[y][x].tptr = cur_pos;
  t_list[cur_pos] = door_list[0];
  cave[y][x].fval = corr_door.ftval;
  cave[y][x].fopen = true;
}

void place_broken_door(const long y, const long x) {
  long cur_pos;

  popt(&cur_pos);

  cave[y][x].tptr = cur_pos;
  t_list[cur_pos] = door_list[0];
  cave[y][x].fval = corr_door.ftval;
  cave[y][x].fopen = true;
  t_list[cur_pos].p1 = 1;
}

void place_closed_door(const long y, const long x) {
  long cur_pos;

  popt(&cur_pos);

  cave[y][x].tptr = cur_pos;
  t_list[cur_pos] = door_list[1];
  cave[y][x].fval = corr_door.ftval;
  cave[y][x].fopen = false;
}

void place_locked_door(const long y, const long x) {
  long cur_pos;

  popt(&cur_pos);

  cave[y][x].tptr = cur_pos;
  t_list[cur_pos] = door_list[1];
  cave[y][x].fval = corr_door.ftval;
  cave[y][x].fopen = false;
  t_list[cur_pos].p1 = randint(10) + 10;
}

void place_stuck_door(const long y, const long x) {
  long cur_pos;

  popt(&cur_pos);

  cave[y][x].tptr = cur_pos;
  t_list[cur_pos] = door_list[1];
  cave[y][x].fval = corr_door.ftval;
  cave[y][x].fopen = false;
  t_list[cur_pos].p1 = -randint(10) - 10;
}

void place_secret_door(const long y, const long x) {
  long cur_pos;

  popt(&cur_pos);

  cave[y][x].tptr = cur_pos;
  t_list[cur_pos] = door_list[2];
  cave[y][x].fval = corr_secret_door.ftval;
  cave[y][x].fopen = false;
}
