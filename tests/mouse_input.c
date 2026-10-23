#include <assert.h>
#include <curses.h>
#include <stdio.h>

#include "../src/io.h"
#include "../src/term.h"
#include "../src/variables.h"

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

  endwin();
  delscreen(screen);
  fclose(input);
  fclose(output);
  puts("Mouse input remains outside the command key stream");
  return 0;
}