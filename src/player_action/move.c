#include <curses.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "../configure.h"
#include "../constants.h"
#include "../creature.h"
#include "../debug.h"
#include "../desc.h"
#include "../inven.h"
#include "../magic.h"
#include "../main_loop.h"
#include "../dungeon/light.h"
#include "../misc.h"
#include "../pascal.h"
#include "../player.h"
#include "../random.h"
#include "../screen.h"
#include "../term.h"
#include "../traps.h"
#include "../types.h"
#include "../variables.h"

#include "search.h"
#include "attack.h"

#include "move.h"

boolean cave_flag = false; /*	{ Used in get_panel   } */

/*{ Player is on an object.  Many things can happen BASED -RAK-   }*/
/*{ on the TVAL of the object.  Traps are set off, money and most }*/
/*{ objects are picked up.  Some objects, such as open doors, just}*/
/*{ sit there...                                                  }*/
static void carry(long y, long x) {

  treas_rec *item_ptr;
  char out_val[82];
  char out2[120];
  char page_char;
  char inv_char;
  treas_rec *tmp_ptr;
  long count;
  boolean money_flag;

  ENTER(("carry", "%d, %d", y, x));

  money_flag = false;
  find_flag = false;

  /* with cave[y][x]. do; */
  inven_temp.data = t_list[cave[y][x].tptr];

  /*{ There's GOLD in them thar hills!      }*/
  /*{ OPPS!                                 }*/

  if (is_in(t_list[cave[y][x].tptr].tval, trap_set)) {
    hit_trap(&y, &x);
  } else if (t_list[cave[y][x].tptr].tval <= valuable_metal) {

    /*{ Attempt to pick up an object.         }*/
    if (inven_check_num()) {      /*{ Too many objects?     }*/
      if (inven_check_weight()) { /*{ Weight limit check }*/

        /*{ Okay, pick it up      }*/
        pusht(cave[y][x].tptr);
        cave[y][x].tptr = 0;

        if (inven_temp.data.tval == valuable_metal) {
          item_ptr = money_carry();
          money_flag = true;
        } else {
          item_ptr = inven_carry();
        }

        prt_stat_block();
        objdes(out_val, item_ptr, true);

        if (money_flag) {

          page_char = '$';
          inv_char = '$';

        } else {

          count = 0;
          tmp_ptr = inventory_list;

          if (tmp_ptr->next == item_ptr->next) {
            count = 0;
          } else {
            do {
              count++;
              tmp_ptr = tmp_ptr->next;
            } while (tmp_ptr->next != item_ptr->next);
          }

          if ((count / 20) > 9) {
            page_char = '*';
            inv_char = '*';
          } else {
            page_char = ((count / 20) + 49);
            inv_char = (count - (count / 20) * 20 + 97);
          }
        }
        sprintf(out2, "You have %s. (%c%c)", out_val, page_char, inv_char);
        msg_print(out2);
      } else {
        msg_print("You can't carry that much weight.");
      }
    } else {
      msg_print("You can't carry that many items.");
    }
  }
  LEAVE("carry", "");
}

/*        { Turns off Find_flag if something interesting appears  -RAK- * }*/
/*        { BUG: Does not handle corridor/room corners, but I didn't * want }*/
/*        {      to add a lot of checking for such a minor detail }*/
static void area_affect(long dir, long y, long x) {
  long z[4]; /*: array [1..3] of long;*/
  long i1, row, col;
  obj_set corridors = {4, 5, 6, 0};
  obj_set some_hidden_stuff = {unseen_trap, secret_door, 0};

  if (cave[y][x].fval == corr_floor1.ftval) {
    if (next_to4(y, x, corridors) > 2) {
      find_flag = false;
    }
  }

  if ((find_flag) && (player_flags.blind < 1)) {

    switch (dir) {
    case 1:
    case 3:
    case 7:
    case 9:
      z[1] = rotate_dir(dir, -1);
      z[2] = dir;
      z[3] = rotate_dir(dir, 1);
      break;

    case 2:
    case 4:
    case 6:
    case 8:
      z[1] = rotate_dir(dir, -2);
      z[2] = dir;
      z[3] = rotate_dir(dir, 2);
      break;
    }

    for (i1 = 1; i1 <= 3; i1++) {
      row = y;
      col = x;
      if (move_dir(z[i1], &row, &col)) {

        /* with cave[row,col] do; */

        /* { Empty doorways        }*/
        if (cave[row][col].fval == corr_floor2.ftval) {
          find_flag = false;
        }

        /*  { Objects player can see}*/
        /*  { Including doors       }*/
        if (find_flag) {
          if (player_light) {
            if (cave[row][col].tptr > 0) {
              if (!(is_in(t_list[cave[row][col].tptr].tval,
                          some_hidden_stuff))) {
                find_flag = false;
              }
            }
          } else if ((cave[row][col].tl) || (cave[row][col].pl) ||
                     (cave[row][col].fm)) {
            if (cave[row][col].tptr > 0) {
              if (!(is_in(t_list[cave[row][col].tptr].tval,
                          some_hidden_stuff))) {
                find_flag = false;
              }
            }
          }
        }

        /*{ Creatures             }*/
        if (find_flag) {
          if ((cave[row][col].tl) || (cave[row][col].pl) || (player_light)) {
            if (cave[row][col].cptr > 1) {
              /* with */
              /* m_list[cave[row][col].cptr]
               */
              /* do; */
              if (m_list[cave[row][col].cptr].ml) {
                find_flag = false;
              }
            }
          }
        }

      } /* end if move_char */
    }   /* end for */
  }     /* end if find and not blind */
}

/**
 * -RAK-
 * s__panel_bounds() - Calculates current boundries
 */
static void s__panel_bounds() {
  panel_row_min = (trunc(panel_row * (SCREEN_HEIGHT / 2)) + 1);
  panel_row_max = panel_row_min + SCREEN_HEIGHT - 1;

  panel_col_min = (trunc(panel_col * (SCREEN_WIDTH / 2)) + 1);
  panel_col_max = panel_col_min + SCREEN_WIDTH - 1;

  panel_row_prt = panel_row_min - 2;
  panel_col_prt = panel_col_min - 15;
}

/*{ Given an row (y) and col (x), this routine detects  -RAK-     }*/
/*{ when a move off the screen has occurred and figures new borders}*/
/* forceit forcses the panel bounds to be recalculated (show_location).
 */
static boolean get_panel(long y, long x, boolean forceit) {

  long prow, pcol;
  boolean return_value;

  prow = panel_row;
  pcol = panel_col;

  if (forceit || (y < panel_row_min + 2) || (y > panel_row_max - 2)) {
    prow = trunc((y - 2) / (SCREEN_HEIGHT / 2));
    if (prow > max_panel_rows) {
      prow = max_panel_rows;
    } else if (prow < 0) {
      prow = 0;
    }
  }

  if (forceit || (x < panel_col_min + 3) || (x > panel_col_max - 3)) {
    pcol = trunc((x - 3) / (SCREEN_WIDTH / 2));
    if (pcol > max_panel_cols) {
      pcol = max_panel_cols;
    } else if (pcol < 0) {
      pcol = 0;
    }
  }

  if ((prow != panel_row) || (pcol != panel_col) || !(cave_flag)) {
    panel_row = prow;
    panel_col = pcol;
    s__panel_bounds();
    cave_flag = true;
    return_value = true;
  } else {
    return_value = false;
  }

  return return_value;
}

/**
 * -RAK-
 *  pick_dir() - Picks new direction when in find mode
 *  @dir: Initial direction
 */
static boolean pick_dir(long dir) {

  long z[2];

  if (!find_flag)
    return false;

  if (next_to4(char_row, char_col, corr_set) != 2)
    return false;

  switch (dir) {
  case 1:
  case 3:
  case 7:
  case 9:
    z[0] = rotate_dir(dir, -1);
    z[1] = rotate_dir(dir, 1);
    break;

  case 2:
  case 4:
  case 6:
  case 8:
    z[0] = rotate_dir(dir, -2);
    z[1] = rotate_dir(dir, 2);
    break;
  }

  boolean return_value = false;
  for (int i = 0; i < 2; i++) {
    long y = char_row;
    long x = char_col;
    if (move_dir(z[i], &y, &x)) {
      if (cave[y][x].fopen) {
        return_value = true;
        com_val = dir_to_char(z[i]);
      }
    }
  }

  return return_value;
}

static void _move_char(long dir) {
  if (dir == 5)
    find_flag = false;

  /* Confused causes random movement 75% of the time */
  if (player_flags.confused > 0 && dir != 5 && randint(4) > 1) {
    dir = randint(9);
    find_flag = false;
  }

  /* Legal move? */
  long test_row = char_row;
  long test_col = char_col;
  if (!move_dir(dir, &test_row, &test_col))
    return;

  /* Creature in the way? Attack! */
  if (cave[test_row][test_col].cptr >= 2) {
    if (find_flag) {
      find_flag = false;
      dungeon_light_move(char_row, char_col, char_row, char_col);
    }
    /* ..if we dare */
    if (player_flags.afraid < 1) {
      player_action_attack(test_row, test_col);
    } else {
      msg_print("You are too afraid to attack!");
    }
    return;
  }

  /* Can't move onto floor space? */
  if (!cave[test_row][test_col].fopen) {
    /* Try a new direction if in find mode */
    if (pick_dir(dir))
      return;

    if (find_flag) {
      find_flag = false;
      player_action_move(5);
      return;
    }

    reset_flag = true;
    if (cave[test_row][test_col].tptr <= 0)
      return;

    if (t_list[cave[test_row][test_col].tptr].tval == rubble)
      msg_print("There is rubble blocking your way.");
    else if (t_list[cave[test_row][test_col].tptr].tval == closed_door)
      msg_print("There is a closed door blocking your way.");
    return;
  }

  /* Open floor spot */
  if (find_flag && (is_in(cave[char_row][char_col].fval, earth_set) ==
                    is_in(cave[test_row][test_col].fval, water_set))) {
    find_flag = false;
    player_action_move(5);
    return;
  }

  /* Move character record (-1) */
  move_rec(char_row, char_col, test_row, test_col);

  /* Check for new panel */
  if (get_panel(test_row, test_col, false))
    prt_map();

  /* Check to see if he should stop */
  if (find_flag)
    area_affect(dir, test_row, test_col);

  /* Check to see if he notices something */
  if (player_flags.blind < 1 && (randint(player_fos) == 1 || search_flag))
    player_action_search(test_row, test_col, C_player_curr_search_skill());

  /* An object is beneath him? */
  if (cave[test_row][test_col].tptr > 0)
    carry(test_row, test_col);

  /* Move the light source */
  dungeon_light_move(char_row, char_col, test_row, test_col);

  /* A room of light should be lit... */
  if (cave[test_row][test_col].fval == lopen_floor.ftval) {
    if (player_flags.blind < 1) {
      if (!(cave[test_row][test_col].pl)) {
        dungeon_light_room(test_row, test_col);
      }
    }

    /* In doorway of light-room? */
  } else if ((cave[test_row][test_col].fval == corr_floor2.ftval ||
              cave[test_row][test_col].fval == corr_floor3.ftval) &&
             player_flags.blind < 1) {
    for (long y = test_row - 1; y <= test_row + 1; y++) {
      for (long x = test_col - 1; x <= test_col + 1; x++) {
        if (in_bounds(y, x) && cave[y][x].fval == lopen_floor.ftval &&
            !cave[y][x].pl) {
          dungeon_light_room(y, x);
        }
      }
    }
  }

  /* Make final assignments of char co-ords */
  char_row = test_row;
  char_col = test_col;
}

int char_to_dir(char c) {
  switch (c) {
  case 'b':
  case 'B':
    return 1;
  case 'j':
  case 'J':
    return 2;
  case 'n':
  case 'N':
    return 3;
  case 'h':
  case 'H':
    return 4;
  case '.':
    return 5;
  case 'l':
  case 'L':
    return 6;
  case 'y':
  case 'Y':
    return 7;
  case 'k':
  case 'K':
    return 8;
  case 'u':
  case 'U':
    return 9;
  default:
    return -1;
  }
}

char dir_to_char(int dir) {
  switch (dir) {
  case 1:
    return 'b';
  case 2:
    return 'j';
  case 3:
    return 'n';
  case 4:
    return 'h';
  case 5:
    return '.';
  case 6:
    return 'l';
  case 7:
    return 'y';
  case 8:
    return 'k';
  case 9:
    return 'u';
  default:
    return '?';
  }
}

void player_action_move(long dir) {
  ENTER(("move_char", "%d", dir));
  _move_char(dir);
  LEAVE("move_char", "m");
}
