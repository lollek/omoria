#include <string.h>

#include "../c.h"
#include "../debug.h"
#include "../floor.h"
#include "../io.h"
#include "../misc.h"
#include "../pascal.h"
#include "../player.h"
#include "../random.h"
#include "../screen.h"
#include "../variables.h"

#include "light.h"

static bool light_flag; /*	{ Used in MOVE_LIGHT  } */

static void ml__radius_bounds(const long row, const long col, long *top,
                              long *bottom, long *left, long *right) {
  *top = row - LIGHT_RADIUS;
  if (*top < 1)
    *top = 1;

  *bottom = row + LIGHT_RADIUS;
  if (*bottom > cur_height)
    *bottom = cur_height;

  *left = col - LIGHT_RADIUS;
  if (*left < 1)
    *left = 1;

  *right = col + LIGHT_RADIUS;
  if (*right > cur_width)
    *right = cur_width;
}

int dungeon_light_is_open(const long row, const long col) {
  return cave[row][col].fopen ? 1 : 0;
}

static void ml__clear_temporary_light_box(const long center_row,
                                          const long center_col) {
  long top, bottom, left, right;
  ml__radius_bounds(center_row, center_col, &top, &bottom, &left, &right);

  for (long row = top; row <= bottom; row++)
    for (long col = left; col <= right; col++)
      cave[row][col].is_temporarily_lit = false;
}

static void ml__set_temporary_light_box_with_los(const long player_row,
                                                 const long player_col) {
  long top, bottom, left, right;
  ml__radius_bounds(player_row, player_col, &top, &bottom, &left, &right);

  for (long row = top; row <= bottom; row++)
    for (long col = left; col <= right; col++)
      if (dungeon_light_should_light_cell(player_row, player_col, row, col,
                                          LIGHT_RADIUS))
        cave[row][col].is_temporarily_lit = true;
}

// Given two sets of points, draw the block
static void ml__draw_block(const long y1, const long x1, const long y2,
                           const long x2) {

  /* Redraw the union of old/new light boxes so larger radii repaint fully. */
  long const topp = max(min(y1, y2) - LIGHT_RADIUS, panel_row_min);
  long const bott = min(max(y1, y2) + LIGHT_RADIUS, panel_row_max);
  long const left = max(min(x1, x2) - LIGHT_RADIUS, panel_col_min);
  long const right = min(max(x1, x2) + LIGHT_RADIUS, panel_col_max);
  long const new_topp = y2 - LIGHT_RADIUS; /*{ Margins for new things to appear}*/
  long const new_bott = y2 + LIGHT_RADIUS;
  long const new_left = x2 - LIGHT_RADIUS;
  long const new_righ = x2 + LIGHT_RADIUS;

  long xmax = 0;

  /*{ From uppermost to bottom most lines player was on...  }*/
  /*{ Points are guaranteed to be on the screen (I hope...) }*/

  for (long y = topp; y <= bott; y++) {
    long xpos = 0;
    chtype floor_str[82] = {0};
    long floor_str_len = 0;

    /*{ Leftmost to rightmost do}*/
    for (long x = left; x <= right; x++) {
      chtype tmp_char = ' ';
      bool flag;

      if (cave[y][x].is_permanently_lit || cave[y][x].fm) {
        flag = (y == y1 && x == x1) || (y == y2 && x == x2);
        /* flag = true; */
      } else {
        flag = true;
        if (y >= new_topp && y <= new_bott &&
            (x >= new_left && x <= new_righ) && cave[y][x].is_temporarily_lit) {
          if (is_in(cave[y][x].fval, pwall_set)) {
            cave[y][x].is_permanently_lit = true;
          } else if (cave[y][x].tptr > 0 &&
                     is_in(t_list[cave[y][x].tptr].tval, light_set) &&
                     !cave[y][x].fm) {
            cave[y][x].fm = true;
          }
        }
      }

      if (cave[y][x].is_permanently_lit || cave[y][x].is_temporarily_lit || cave[y][x].fm)
        tmp_char = loc_symbol(y, x);
      if (player_flags.image > 0 && randint(12) == 1)
        tmp_char = (char)(randint(95) + 31);

      if (flag) {
        if (xpos == 0)
          xpos = x;
        xmax = x;
      }

      if (xpos > 0) {
        if (floor_str_len > 80)
          MSG((": ERROR draw_block floor_str_len "
               "too big: %d",
               floor_str_len));
        floor_str[floor_str_len++] = tmp_char;
      }

    } /* end for  x */

    if (floor_str_len > 80)
      MSG((": ERROR draw_block floor_str_len too big: %d", floor_str_len));

    floor_str[floor_str_len] = 0;

    if (xpos > 0) {
      /*{ Var for PRINT cannot be loop index}*/
      long const ypos = y;
      /*print(substr(floor_str,1,1+xmax-xpos),y2,xpos);*/

      if (1 + xmax - xpos + 1 > 80 || 1 + xmax - xpos + 1 < 0)
        MSG((": ERROR draw_block xmax-xpos is bad\n"));

      floor_str[1 + xmax - xpos + 1] = 0;
      print_chstr(floor_str, ypos, xpos);
    }
  }
}

static void ml__sub1_move_light(const long y1, const long x1, const long y2,
                                const long x2) {
  /*{ Normal movement                                   }*/

  light_flag = true;

  /* Turn off old light box, then light the new one with LOS checks. */
  ml__clear_temporary_light_box(y1, x1);
  ml__set_temporary_light_box_with_los(y2, x2);

  ml__draw_block(y1, x1, y2, x2); /*{ Redraw area           }*/
}

static void ml__sub2_move_light(const long y1, const long x1, const long y2,
                                const long x2) {
  /*{ When FIND_FLAG, light only permanent features     }*/

  long new_top, new_bottom, new_left, new_right;
  ml__radius_bounds(y2, x2, &new_top, &new_bottom, &new_left, &new_right);

  if (light_flag) {
    ml__clear_temporary_light_box(y1, x1);
    ml__draw_block(y1, x1, y1, x1);
    light_flag = false;
  }

  for (long y = new_top; y <= new_bottom; y++) {
    chtype floor_str[82] = {0};
    chtype save_str[82] = {0};
    long floor_str_len = 0;
    long save_str_len = 0;
    long xpos = 0;
    chtype tmp_char;

    for (long x = new_left; x <= new_right; x++) {
      bool flag = false;
      if (!(cave[y][x].fm || cave[y][x].is_permanently_lit)) {
        tmp_char = ' ';
        if (player_light &&
            dungeon_light_should_light_cell(y2, x2, y, x, LIGHT_RADIUS)) {
          if (is_in(cave[y][x].fval, pwall_set)) {
            /* Turn on perm light */
            cave[y][x].is_permanently_lit = true;
            tmp_char = loc_symbol(y, x);
            flag = true;
          } else if (cave[y][x].tptr > 0 &&
                     is_in(t_list[cave[y][x].tptr].tval, light_set)) {
            /* Turn on field marker */
            cave[y][x].fm = true;
            tmp_char = loc_symbol(y, x);
            flag = true;
          }
        }
      } else {
        tmp_char = loc_symbol(y, x);
      }

      if (flag) {
        if (xpos == 0)
          xpos = x;
        if (save_str[0] != 0) {
          for (long i = 0; i < save_str_len; ++i)
            floor_str[floor_str_len++] = save_str[i];
          save_str[0] = 0;
          save_str_len = 0;
        }
        floor_str[floor_str_len++] = tmp_char;
      } else if (xpos > 0) {
        save_str[save_str_len++] = tmp_char;
      }
    } /* end for x */

    if (xpos > 0) {
      long const tmp_y = y;
      floor_str[floor_str_len] = 0;
      print_chstr(floor_str, tmp_y, xpos);
    }
  } /* end for y */
}

static void ml__sub3_move_light(const long y1, const long x1, const long y2,
                                const long x2) {
  /*{ When blinded, move only the player symbol...              }*/

  if (light_flag) {
    ml__clear_temporary_light_box(y1, x1);
    light_flag = false;
  }
  print(' ', y1, x1);
  print('@', y2, x2);
}

static void ml__sub4_move_light(const long y1, const long x1, const long y2,
                                const long x2) {
  /*{ With no light, movement becomes involved...               }*/

  light_flag = true;
  if (cave[y1][x1].is_temporarily_lit) {
    dungeon_light_clear_temporary_light_box_and_redraw(y1, x1);
  } else if (test_light(y1, x1)) {
    lite_spot(y1, x1);
  } else {
    unlite_spot(y1, x1);
  }
  print('@', y2, x2);
}

void dungeon_light_move(const long y1, const long x1, const long y2,
                        const long x2) {

  if (player_flags.blind > 0) {
    ml__sub3_move_light(y1, x1, y2, x2); /* blind */
  } else if (find_flag) {
    ml__sub2_move_light(y1, x1, y2, x2); /* searching */
  } else if (!player_light) {
    ml__sub4_move_light(y1, x1, y2, x2); /* no light */
  } else {
    ml__sub1_move_light(y1, x1, y2, x2); /* normal */
  }
}

static void lr__find_light(const long y1, const long x1, const long y2,
                           const long x2) {
  obj_set room_floors;

  memset(room_floors, 0, sizeof(room_floors));
  room_floors[0] = ft_dark_open_floor;
  room_floors[1] = ft_light_open_floor;
  room_floors[2] = ft_water_on_room_floor;

  for (long i1 = y1; i1 <= y2; i1++) {
    for (long i2 = x1; i2 <= x2; i2++) {
      if (!is_in(cave[i1][i2].fval, room_floors))
        continue;

      for (long i3 = i1 - 1; i3 <= i1 + 1; i3++) {
        for (long i4 = i2 - 1; i4 <= i2 + 1; i4++)
          cave[i3][i4].is_permanently_lit = true;
      }

      if (cave[i1][i2].fval == ft_water_on_room_floor) {
        /* water on room floor */
        cave[i1][i2].fval = water_on_floor_lit.ftval;
        /* lit rm water on floor */
      } else {
        cave[i1][i2].fval = ft_light_open_floor;
        /* lit rm floor */
      }
    }
  }
}

void dungeon_light_room(const long param_y, const long param_x) {

  long const half_height = SCREEN_HEIGHT / 2;
  long const half_width = SCREEN_WIDTH / 2;
  long const start_row = param_y / half_height * half_height + 1;
  long const start_col = param_x / half_width * half_width + 1;
  long const end_row = start_row + half_height - 1;
  long const end_col = start_col + half_width - 1;
  long xpos = 0;

  lr__find_light(start_row, start_col, end_row, end_col);

  for (long y = start_row; y <= end_row; y++) {
    chtype floor_str[82] = {0};
    long floor_str_len = 0;
    long const ypos = y;
    for (long x = start_col; x <= end_col; x++) {
      if (cave[y][x].is_permanently_lit || cave[y][x].fm) {
        if (floor_str_len == 0)
          xpos = x;
        floor_str[floor_str_len++] = loc_symbol(y, x);
      } else {
        if (floor_str_len > 0) {
          floor_str[floor_str_len] = 0;
          print_chstr(floor_str, ypos, xpos);
          floor_str[0] = 0;
          floor_str_len = 0;
        }
      }
    }
    if (floor_str_len > 0) {
      floor_str[floor_str_len] = 0;
      print_chstr(floor_str, ypos, xpos);
    }
  }
}

void dungeon_light_clear_temporary_light_box_and_redraw(const long center_row,
                                                     const long center_col) {
  long top, bottom, left, right;
  ml__radius_bounds(center_row, center_col, &top, &bottom, &left, &right);

  for (long row = top; row <= bottom; row++) {
    for (long col = left; col <= right; col++) {
      cave[row][col].is_temporarily_lit = false;
      if (test_light(row, col))
        lite_spot(row, col);
      else
        unlite_spot(row, col);
    }
  }
}
