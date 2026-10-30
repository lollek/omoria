#include <curses.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "../constants.h"
#include "../creature.h"
#include "../debug.h"
#include "../floor.h"
#include "../dungeon/light.h"
#include "../inventory/inven.h"
#include "../io.h"
#include "../misc.h"
#include "../pascal.h"
#include "../player.h"
#include "../player_action.h"
#include "../random.h"
#include "../screen.h"
#include "../term.h"
#include "../text_lines.h"
#include "../traps.h"
#include "../types.h"
#include "../variables.h"

bool cave_flag = false; /*	{ Used in get_panel   } */

static void apply_viewport_state(const struct viewport_state state) {
  panel_row = state.panel_row;
  panel_col = state.panel_col;
  panel_row_min = state.world_top;
  panel_row_max = state.world_bottom;
  panel_col_min = state.world_left;
  panel_col_max = state.world_right;
  panel_row_prt = state.panel_row_prt;
  panel_col_prt = state.panel_col_prt;
  max_panel_rows = state.max_panel_rows;
  max_panel_cols = state.max_panel_cols;
}

void C_init_viewport(const bool at_end) {
  const struct viewport_state initial =
      C_viewport_get(0, 0, cur_height, cur_width);
  panel_row = at_end ? initial.max_panel_rows : 0;
  panel_col = at_end ? initial.max_panel_cols : 0;
  apply_viewport_state(
      C_viewport_get(panel_row, panel_col, cur_height, cur_width));
  cave_flag = true;
}

/* Traps are triggered and money is collected on movement. */
static void carry(const long y, const long x, const bool pickup_objects) {

  ENTER(("carry", "%d, %d", y, x));

  find_flag = false;

  /* with cave[y][x]. do; */
  inven_temp.data = t_list[cave[y][x].tptr];

  /*{ There's GOLD in them thar hills!      }*/
  /*{ OPPS!                                 }*/

  if (is_in(t_list[cave[y][x].tptr].tval, trap_set)) {
    hit_trap(&y, &x);
  } else if (t_list[cave[y][x].tptr].tval == valuable_metal ||
             (pickup_objects &&
              t_list[cave[y][x].tptr].tval < valuable_metal)) {

    /*{ Attempt to pick up an object.         }*/
    if (inven_check_num()) {      /*{ Too many objects?     }*/
      if (inven_check_weight()) {
        bool money_flag = false;
        char inv_char;
        char page_char;
        char out2[120];
        char out_val[82];
        treas_rec *item_ptr;
        /*{ Weight limit check }*/

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
        item_name(out_val, item_ptr);

        if (money_flag) {

          page_char = '$';
          inv_char = '$';

        } else {

          long count = 0;
          const treas_rec *tmp_ptr = inventory_list;

          if (tmp_ptr->next == item_ptr->next) {
            count = 0;
          } else {
            do {
              count++;
              tmp_ptr = tmp_ptr->next;
            } while (tmp_ptr->next != item_ptr->next);
          }

          if (count / 20 > 9) {
            page_char = '*';
            inv_char = '*';
          } else {
            page_char = count / 20 + 49;
            inv_char = count - count / 20 * 20 + 97;
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

void player_action_pickup(void) {
  const long y = char_row;
  const long x = char_col;
  const long item_index = cave[y][x].tptr;
  find_flag = false;

  if (item_index == 0 || is_in(t_list[item_index].tval, trap_set) ||
      t_list[item_index].tval > valuable_metal) {
    msg_print("There is nothing here to pick up.");
    return;
  }

  carry(y, x, true);
  draw_cave();
}

/*        { Turns off Find_flag if something interesting appears  -RAK- * }*/
/*        { BUG: Does not handle corridor/room corners, but I didn't * want }*/
/*        {      to add a lot of checking for such a minor detail }*/
static void area_affect(const long dir, const long y, const long x) {
  long row, col;
  const bool travel_active = C_mouse_travel_is_active();
  obj_set corridors = {4, 5, 6, 0};
  const obj_set some_hidden_stuff = {unseen_trap, secret_door, 0};

  if (!travel_active && cave[y][x].fval == corr_open_floor.ftval) {
    if (next_to4(y, x, corridors) > 2) {
      find_flag = false;
    }
  }

  if (find_flag && player_flags.blind < 1) {
    long z[4];

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

    for (long i1 = 1; i1 <= 3; i1++) {
      row = y;
      col = x;
      if (move_dir(z[i1], &row, &col)) {

        /* with cave[row,col] do; */

        /* { Empty doorways        }*/
        if (!travel_active && cave[row][col].fval == corr_room_junction.ftval) {
          find_flag = false;
        }

        /*  { Objects player can see}*/
        /*  { Including doors       }*/
        if (find_flag) {
          if (player_light) {
            if (cave[row][col].tptr > 0) {
              if (!is_in(t_list[cave[row][col].tptr].tval, some_hidden_stuff)) {
                find_flag = false;
              }
            }
          } else if (cave[row][col].is_temporarily_lit || cave[row][col].is_permanently_lit ||
                     cave[row][col].fm) {
            if (cave[row][col].tptr > 0) {
              if (!is_in(t_list[cave[row][col].tptr].tval, some_hidden_stuff)) {
                find_flag = false;
              }
            }
          }
        }

        /*{ Creatures             }*/
        if (find_flag) {
          if (cave[row][col].is_temporarily_lit || cave[row][col].is_permanently_lit || player_light) {
            if (cave[row][col].cptr > 1) {
              /* with */
              /* m_list[cave[row][col].cptr]
               */
              /* do; */
              if (m_list[cave[row][col].cptr].is_seen) {
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
/*{ Given an row (y) and col (x), this routine detects  -RAK-     }*/
/*{ when a move off the screen has occurred and figures new borders}*/
/* forceit forcses the panel bounds to be recalculated (show_location).
 */
static bool get_panel(const long y, const long x, const bool forceit) {
  const bool uninitialized = !cave_flag;
  const struct viewport_state state = C_viewport_update(
      panel_row, panel_col, cur_height, cur_width, y, x, forceit || uninitialized);
  const bool changed = state.changed || uninitialized;
  if (changed) {
    apply_viewport_state(state);
    cave_flag = true;
  }
  return changed;
}

/**
 * -RAK-
 *  pick_dir() - Picks new direction when in find mode
 *  @dir: Initial direction
 */
static bool pick_dir(const long dir) {

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

  bool return_value = false;
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

  const struct player_move_direction direction =
      C_player_move_direction(dir, player_flags.confused);
  dir = direction.dir;
  if (direction.scrambled) {
    find_flag = false;
  }

  const struct player_move_result step =
      C_player_move_resolve(dir, char_row, char_col);
  if (step.kind == PLAYER_MOVE_OUT_OF_BOUNDS) {
    find_flag = false;
    reset_flag = !step.consumes_turn;
    return;
  }
  const long test_row = step.row;
  const long test_col = step.col;

  /* Creature in the way? Attack! */
  if (step.kind == PLAYER_MOVE_ATTACK) {
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
  if (step.kind == PLAYER_MOVE_BLOCKED) {
    /* Try a new direction if in find mode */
    if (pick_dir(dir))
      return;

    if (find_flag) {
      find_flag = false;
      player_action_move(5);
      return;
    }

    reset_flag = !step.consumes_turn;
    if (step.obstacle == rubble)
      msg_print("There is rubble blocking your way.");
    else if (step.obstacle == closed_door)
      msg_print("There is a closed door blocking your way.");
    return;
  }

  /* Open floor spot */
  if (find_flag && is_in(cave[char_row][char_col].fval, earth_set) ==
                       is_in(cave[test_row][test_col].fval, water_set)) {
    find_flag = false;
    player_action_move(5);
    return;
  }

  /* Move character record (-1) */
  move_creature(char_row, char_col, test_row, test_col);
  cave[test_row][test_col].moved = true;

  /* Check for new panel */
  if (get_panel(test_row, test_col, false))
    prt_map();

  /* Check to see if he should stop */
  if (find_flag)
    area_affect(dir, test_row, test_col);

  /* Check to see if he notices something */
  if (player_flags.blind < 1 && (randint(player_fos()) == 1 || search_flag))
    player_action_search(test_row, test_col, C_player_curr_search_skill());

  /* An object is beneath him? */
  if (cave[test_row][test_col].tptr > 0)
    carry(test_row, test_col, false);

  /* Move the light source */
  dungeon_light_move(char_row, char_col, test_row, test_col);

  /* A room of light should be lit... */
  if (cave[test_row][test_col].fval == ft_light_open_floor) {
    if (player_flags.blind < 1) {
      if (!cave[test_row][test_col].is_permanently_lit) {
        dungeon_light_room(test_row, test_col);
      }
    }

    /* In doorway of light-room? */
  } else if ((cave[test_row][test_col].fval == corr_room_junction.ftval ||
              cave[test_row][test_col].fval == corr_door.ftval) &&
             player_flags.blind < 1) {
    for (long y = test_row - 1; y <= test_row + 1; y++) {
      for (long x = test_col - 1; x <= test_col + 1; x++) {
        if (in_bounds(y, x) && cave[y][x].fval == ft_light_open_floor &&
            !cave[y][x].is_permanently_lit) {
          dungeon_light_room(y, x);
        }
      }
    }
  }

  /* Make final assignments of char co-ords */
  char_row = test_row;
  char_col = test_col;
}

void player_action_move(const long dir) {
  ENTER(("move_char", "%d", dir));
  _move_char(dir);
  LEAVE("move_char", "m");
}
