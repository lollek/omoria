#include <assert.h>
#include <curses.h>
#include <stdio.h>
#include <string.h>

#include "../src/floor.h"
#include "../src/io.h"
#include "../src/dungeon/light.h"
#include "../src/main_loop/command.h"
#include "../src/messages.h"
#include "../src/player.h"
#include "../src/screen.h"
#include "../src/term.h"
#include "../src/variables.h"

extern bool C_layout_is_split(void);
extern void C_wait_for_usable_terminal_with(
  int rows, int cols, int (*read_key)(void),
  void (*read_size)(int *rows, int *cols));
extern void C_terminal_resize(void (*redraw_screen)(void));

static int resize_reads;
static int redraw_calls;

static int resize_to_split(void) {
  resize_reads++;
  assert(mvinch(0, 0) == 'T');
  assert(resize_term(35, 150) == OK);
  return KEY_RESIZE;
}

static void read_terminal_size(int *rows, int *cols) {
  getmaxyx(stdscr, *rows, *cols);
}

static void record_redraw(void) {
  redraw_calls++;
  assert(panel_row == 3);
  assert(panel_col == 3);
}

static void assert_travel_player_symbol_is_drawn(void) {
  cur_height = cur_width = 7;
  char_row = char_col = 3;
  memset(cave, 0, sizeof(cave));
  for (long row = 1; row <= 7; row++) {
    for (long col = 1; col <= 7; col++) {
      cave[row][col].fval = ft_dark_open_floor;
      cave[row][col].fopen = true;
      cave[row][col].moved = true;
    }
  }
  cave[3][3].cptr = 1;

  const struct viewport_state viewport = C_viewport_get(0, 0, 7, 7);
  panel_row_min = viewport.world_top;
  panel_row_max = viewport.world_bottom;
  panel_col_min = viewport.world_left;
  panel_col_max = viewport.world_right;
  panel_row_prt = viewport.panel_row_prt;
  panel_col_prt = viewport.panel_col_prt;

  player_light = false;
  find_flag = false;
  prt_map();
  refresh();
  assert(mvinch(viewport.screen_row + 3 - viewport.world_top,
                viewport.screen_col + 3 - viewport.world_left) == '@');

  assert(C_mouse_travel_begin(5, 5));
  find_flag = true;
  cave[3][3].cptr = 0;
  cave[4][4].cptr = 1;
  char_row = char_col = 4;
  dungeon_light_move(3, 3, 4, 4);
  refresh();
  assert(mvinch(viewport.screen_row + 3 - viewport.world_top,
                viewport.screen_col + 3 - viewport.world_left) != '@');
  assert(mvinch(viewport.screen_row + 4 - viewport.world_top,
                viewport.screen_col + 4 - viewport.world_left) == '@');

  cave[4][4].cptr = 0;
  cave[5][5].cptr = 1;
  char_row = char_col = 5;
  dungeon_light_move(4, 4, 5, 5);
  refresh();
  assert(C_mouse_travel_arrived(5, 5));
  find_flag = false;
  assert(mvinch(viewport.screen_row + 5 - viewport.world_top,
                viewport.screen_col + 5 - viewport.world_left) == '@');
  assert(mvinch(viewport.screen_row + 4 - viewport.world_top,
                viewport.screen_col + 4 - viewport.world_left) != '@');
  assert(mvinch(viewport.screen_row + 3 - viewport.world_top,
                viewport.screen_col + 3 - viewport.world_left) != '@');
  C_mouse_travel_cancel();
}

static void assert_travel_cursor_follows_player(void) {
  cur_height = cur_width = 7;
  char_row = char_col = 3;
  memset(cave, 0, sizeof(cave));
  memset(&player_flags, 0, sizeof(player_flags));
  player_flags.blind = 1;
  player_light = false;
  for (long row = 1; row <= 7; row++) {
    for (long col = 1; col <= 7; col++) {
      cave[row][col].fopen = true;
      cave[row][col].moved = true;
    }
  }
  cave[3][3].cptr = 1;

  const struct viewport_state viewport = C_viewport_get(0, 0, 7, 7);
  panel_row_min = viewport.world_top;
  panel_row_max = viewport.world_bottom;
  panel_col_min = viewport.world_left;
  panel_col_max = viewport.world_right;
  panel_row_prt = viewport.panel_row_prt;
  panel_col_prt = viewport.panel_col_prt;

  assert(C_mouse_travel_begin(5, 5));
  long travel_command = OMORIA_KEY_TRAVEL;
  C_message_capture_begin();
  command(&travel_command);
  C_message_capture_end();

  int cursor_row;
  int cursor_col;
  getyx(stdscr, cursor_row, cursor_col);
  assert(cursor_row == viewport.screen_row + char_row - viewport.world_top);
  assert(cursor_col == viewport.screen_col + char_col - viewport.world_left);
  assert(mvinch(cursor_row, cursor_col) == '@');
  C_mouse_travel_cancel();
  find_flag = false;
}

int main(void) {
  FILE *input = tmpfile();
  FILE *output = tmpfile();
  assert(input != NULL && output != NULL);

  SCREEN *screen = newterm("xterm", output, input);
  assert(screen != NULL);
  set_term(screen);
  keypad(stdscr, TRUE);
  mousemask(ALL_MOUSE_EVENTS | REPORT_MOUSE_POSITION, NULL);

  MEVENT event = {
    .x = 9, .y = 4, .bstate = BUTTON1_CLICKED | BUTTON_SHIFT
  };
  assert(ungetmouse(&event) == OK);
  assert(ungetch('q') == OK);
  assert(ungetch(KEY_MOUSE) == OK);

  cur_height = 66;
  cur_width = 198;
  assert(inkey() == 'q');
  struct mouse_event mouse = C_mouse_event_pop();
  assert(mouse.kind == 1);
  assert(mouse.button == 1);
  assert(mouse.row == 4);
  assert(mouse.col == 9);
  assert(C_mouse_event_pop().kind == 0);

  cur_height = cur_width = 7;
  char_row = char_col = 3;
  memset(cave, 0, sizeof(cave));
  for (long row = 1; row <= 7; row++) {
    for (long col = 1; col <= 7; col++) {
      cave[row][col].fopen = true;
      cave[row][col].is_permanently_lit = true;
    }
  }
  assert(C_mouse_travel_begin(5, 5));

  MEVENT motion = {.x = 12, .y = 8, .bstate = REPORT_MOUSE_POSITION};
  assert(ungetmouse(&motion) == OK);
  assert(ungetch('k') == OK);
  assert(ungetch(KEY_MOUSE) == OK);
  assert(C_terminal_poll_key() == 0);
  assert(C_mouse_travel_is_active());
  mouse = C_mouse_event_pop();
  assert(mouse.kind == 2 && mouse.row == 8 && mouse.col == 12);

  assert(C_terminal_poll_key() == 1);
  assert(C_mouse_travel_is_active());
  C_mouse_travel_cancel();
  assert(inkey() == 'k');

  cur_height = 0;
  cur_width = 0;
  assert(resize_term(35, 150) == OK);
  assert(ungetch('r') == OK);
  assert(ungetch(KEY_RESIZE) == OK);
  assert(inkey() == 'r');
  assert(C_layout_is_split());

  assert(resize_term(25, 150) == OK);
  assert(ungetch('c') == OK);
  assert(ungetch(KEY_RESIZE) == OK);
  assert(inkey() == 'c');
  assert(!C_layout_is_split());

  assert(resize_term(24, 149) == OK);
  resize_reads = 0;
  C_wait_for_usable_terminal_with(24, 149, resize_to_split,
                                  read_terminal_size);
  assert(resize_reads == 1);
  assert(C_layout_is_split());
  assert(mvinch(0, 0) == ' ');
  assert_travel_player_symbol_is_drawn();
  assert_travel_cursor_follows_player();

  cur_height = 66;
  cur_width = 198;
  char_row = 45;
  char_col = 170;
  panel_row = 0;
  panel_col = 0;
  character_generated = 0;
  redraw_calls = 0;
  C_terminal_resize(record_redraw);
  assert(redraw_calls == 1);
  assert(panel_row == 3);
  assert(panel_col == 3);

  endwin();
  delscreen(screen);
  fclose(input);
  fclose(output);
  puts("Mouse input remains outside the command key stream");
  return 0;
}