#include <assert.h>
#include <curses.h>
#include <stdio.h>
#include <string.h>

#include "../src/io.h"
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

int main(void) {
  FILE *input = tmpfile();
  FILE *output = tmpfile();
  assert(input != NULL && output != NULL);

  SCREEN *screen = newterm("xterm", output, input);
  assert(screen != NULL);
  set_term(screen);
  keypad(stdscr, TRUE);
  mousemask(ALL_MOUSE_EVENTS | REPORT_MOUSE_POSITION, NULL);

  MEVENT event = {.x = 9, .y = 4, .bstate = BUTTON1_CLICKED};
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