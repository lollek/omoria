#include <string.h>

#include "../debug.h"
#include "../io.h"
#include "../misc.h"
#include "../pascal.h"
#include "../player.h"
#include "../random.h"
#include "../screen.h"
#include "../variables.h"

#include "light.h"

static bool light_flag; /*	{ Used in MOVE_LIGHT  } */

static void ml__draw_block(const long y1, const long x1, const long y2,
                           const long x2) {
  /*{ Given two sets of points, draw the block		}*/

  long const topp = maxmin(y1, y2, panel_row_min);
  long const bott = minmax(y1, y2, panel_row_max);
  long const left = maxmin(x1, x2, panel_col_min);
  long const right = minmax(x1, x2, panel_col_max);
  long const new_topp = y2 - 1; /*{ Margins for new things to appear}*/
  long const new_bott = y2 + 1;
  long const new_left = x2 - 1;
  long const new_righ = x2 + 1;

  long xmax = 0;

  ENTER(("ml__draw_block", "%d, %d, %d, %d", y1, x2, y2, x2));

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

      if (cave[y][x].pl || cave[y][x].fm) {
        flag = (y == y1 && x == x1) || (y == y2 && x == x2);
        /* flag = true; */
      } else {
        flag = true;
        if (y >= new_topp && y <= new_bott &&
            (x >= new_left && x <= new_righ) && cave[y][x].tl) {
          if (is_in(cave[y][x].fval, pwall_set)) {
            cave[y][x].pl = true;
          } else if (cave[y][x].tptr > 0 &&
                     is_in(t_list[cave[y][x].tptr].tval, light_set) &&
                     !cave[y][x].fm) {
            cave[y][x].fm = true;
          }
        }
      }

      if (cave[y][x].pl || cave[y][x].tl || cave[y][x].fm)
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

  LEAVE("ml__draw_block", "m");
}

static void ml__sub1_move_light(const long y1, const long x1, const long y2,
                                const long x2) {
  /*{ Normal movement                                   }*/

  ENTER(("ml__sub1_move_light", "%d, %d, %d, %d", y1, x1, y2, x2));

  light_flag = true;

  /* Turn off lamp light */
  for (long i = y1 - 1; i <= y1 + 1; i++)
    for (long j = x1 - 1; j <= x1 + 1; j++)
      cave[i][j].tl = false;

  for (long i = y2 - 1; i <= y2 + 1; i++)
    for (long j = x2 - 1; j <= x2 + 1; j++)
      cave[i][j].tl = true;

  ml__draw_block(y1, x1, y2, x2); /*{ Redraw area           }*/

  LEAVE("ml__sub1_move_light", "m");
}

static void ml__sub2_move_light(const long y1, const long x1, const long y2,
                                const long x2) {
  /*{ When FIND_FLAG, light only permanent features     }*/

  ENTER(("ml__sub2_move_light", "%d, %d, %d, %d", y1, x1, y2, x1));

  if (light_flag) {
    for (long y = y1 - 1; y <= y1 + 1; y++)
      for (long x = x1 - 1; x <= x1 + 1; x++)
        cave[y][x].tl = false;
    ml__draw_block(y1, x1, y1, x1);
    light_flag = false;
  }

  for (long y = y2 - 1; y <= y2 + 1; y++) {
    chtype floor_str[82] = {0};
    chtype save_str[82] = {0};
    long floor_str_len = 0;
    long save_str_len = 0;
    long xpos = 0;
    chtype tmp_char;

    for (long x = x2 - 1; x <= x2 + 1; x++) {
      bool flag = false;
      if (!(cave[y][x].fm || cave[y][x].pl)) {
        tmp_char = ' ';
        if (player_light) {
          if (is_in(cave[y][x].fval, pwall_set)) {
            /* Turn on perm light */
            cave[y][x].pl = true;
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

  LEAVE("ml__sub2_move_light", "m");
}

static void ml__sub3_move_light(const long y1, const long x1, const long y2,
                                const long x2) {
  /*{ When blinded, move only the player symbol...              }*/

  ENTER(("ml__sub3_move_light", "%d, %d, %d, %d", y1, x1, y2, x1));

  if (light_flag) {
    for (long i1 = y1 - 1; i1 <= y1 + 1; i1++) {
      for (long i2 = x1 - 1; i2 <= x1 + 1; i2++) {
        cave[i1][i2].tl = false;
      }
    }
    light_flag = false;
  }
  vms_print(' ', y1, x1);
  vms_print('@', y2, x2);

  LEAVE("ml__sub3_move_light", "m");
}

static void ml__sub4_move_light(const long y1, const long x1, const long y2,
                                const long x2) {
  /*{ With no light, movement becomes involved...               }*/

  ENTER(("ml__sub4_move_light", "%d, %d, %d, %d", y1, x1, y2, x2));

  light_flag = true;
  if (cave[y1][x1].tl) {
    for (long i1 = y1 - 1; i1 <= y1 + 1; i1++) {
      for (long i2 = x1 - 1; i2 <= x1 + 1; i2++) {
        cave[i1][i2].tl = false;
        if (test_light(i1, i2))
          lite_spot(i1, i2);
        else
          unlite_spot(i1, i2);
      }
    }
  } else if (test_light(y1, x1)) {
    lite_spot(y1, x1);
  } else {
    unlite_spot(y1, x1);
  }
  vms_print('@', y2, x2);

  LEAVE("ml__sub4_move_light", "m");
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
  room_floors[0] = dopen_floor.ftval;
  room_floors[1] = lopen_floor.ftval;
  room_floors[2] = water2.ftval;

  for (long i1 = y1; i1 <= y2; i1++) {
    for (long i2 = x1; i2 <= x2; i2++) {
      if (!is_in(cave[i1][i2].fval, room_floors))
        continue;

      for (long i3 = i1 - 1; i3 <= i1 + 1; i3++) {
        for (long i4 = i2 - 1; i4 <= i2 + 1; i4++)
          cave[i3][i4].pl = true;
      }

      if (cave[i1][i2].fval == water2.ftval) {
        /* water on room floor */
        cave[i1][i2].fval = water3.ftval;
        /* lit rm water on floor */
      } else {
        cave[i1][i2].fval = lopen_floor.ftval;
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

  ENTER(("light_room", "%d, %d", param_y, param_x));

  lr__find_light(start_row, start_col, end_row, end_col);

  for (long y = start_row; y <= end_row; y++) {
    chtype floor_str[82] = {0};
    long floor_str_len = 0;
    long const ypos = y;
    for (long x = start_col; x <= end_col; x++) {
      if (cave[y][x].pl || cave[y][x].fm) {
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

  LEAVE("light_room", "");
}
