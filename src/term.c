/* stuff to print stuff to the screen. This file originaly came from umoria. */

#include <sys/wait.h>

#include <ctype.h>
#include <curses.h>
#include <math.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ioctl.h>
#include <termios.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "io.h"
#include "messages.h"
#include "screen.h"
#include "save.h"
#include "term.h"

#include "debug.h"
#include "unix.h"
#include "variables.h"

int command_count;
int eof_flag;
int character_generated, character_saved;
int sound_beep_flag;
int panic_save;
FILE *highscore_fp;

extern bool C_terminal_layout_update(int rows, int cols);

static int read_terminal_key(void) { return getch(); }

static void read_terminal_size(int *rows, int *cols) {
  getmaxyx(stdscr, *rows, *cols);
}

void C_wait_for_usable_terminal_with(
    int rows, int cols, int (*read_key)(void),
    void (*read_size)(int *rows, int *cols)) {
  bool showed_warning = false;
  while (!C_terminal_layout_update(rows, cols)) {
    static const char warning[] =
        "Terminal too small; resize to at least 150x25 to continue.";
    clear();
    if (rows > 0 && cols > 1) {
      mvaddnstr(0, 0, warning, cols - 1);
    }
    refresh();
    showed_warning = true;

    int input;
    do {
      input = read_key();
    } while (input != KEY_RESIZE);
    read_size(&rows, &cols);
  }

  if (showed_warning) {
    clear();
    refresh();
  }
}

void C_wait_for_usable_terminal(void) {
  int rows, cols;
  getmaxyx(stdscr, rows, cols);
  C_wait_for_usable_terminal_with(rows, cols, read_terminal_key,
                                  read_terminal_size);
}

static void recenter_viewport_on_player(void) {
  const struct viewport_state state = C_viewport_update(
      panel_row, panel_col, cur_height, cur_width, char_row, char_col, true);
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
  cave_flag = true;
}

void C_terminal_resize(void (*redraw_screen)(void)) {
  C_wait_for_usable_terminal();
  if (cur_height > 0 && cur_width > 0) {
    recenter_viewport_on_player();
    redraw_screen();
  }
}

/* source/io.c: terminal I/O code, uses the curses package

   Copyright (c) 1989-94 James E. Wilson, Robert A. Koeneke

   This software may be copied and distributed for educational, research, and
   not for profit purposes provided that this copyright and statement are
   included in all such copies. */

void put_buffer_attr(const char *out_str, const long row, const long col,
                     const int attrs) {
  attr_t old_attr;
  short unused_pair;
  short unused_opts;

  (void)unused_opts;
  attr_get(&old_attr, &unused_pair, &unused_opts);
  attrset(attrs);
  put_buffer(out_str, row, col);
  attrset(old_attr);
}

/* Returns a single character input from the terminal.	This silently -CJS-
   consumes ^R to redraw the screen and reset the terminal, so that this
   operation can always be performed at any input prompt.  inkey() never
   returns ^R.	*/
char inkey(void) {

  C_message_log_input();
  refresh();         /* Dump IO buffer		*/
  command_count = 0; /* Just to be safe -CJS- */
  while (TRUE) {
    const int i = getch();
    if (i == KEY_RESIZE) {
      C_terminal_resize(draw_cave);
      continue;
    }
    /* some machines may not sign extend. */
    if (i == EOF) {
      eof_flag++;
      /* avoid infinite loops while trying to call inkey() for
         a -more-
         prompt. */
      msg_flag = FALSE;

      refresh();
      if (!character_generated || character_saved)
        exit_game();
      /*  disturb(1, 0);*/
      if (eof_flag > 100) {
        /* just in case, to make sure that the process
         * eventually dies */
        panic_save = 1;
        strcpy(died_from, "(end of input: panic saved)");
        if (!sav__save_character()) {
          strcpy(died_from, "panic: unexpected eof");
          death = TRUE;
        }
        exit_game();
      }
      return ESCAPE;
    }
    const int translated = C_translate_terminal_input_code(i);
    if (translated == C_MOUSE_INPUT) {
      MEVENT event;
      if (getmouse(&event) == OK) {
        C_mouse_event_push(event.y, event.x, (unsigned long)event.bstate);
        C_mouse_dispatch_pending_events(panel_row, panel_col, cur_height, cur_width);
      }
      continue;
    }
    if (translated < 0)
      continue;
    if (translated != CTRL_('R')) {
      msg_flag = false;
      return (char)translated;
    }
    refresh();
  }
}

/* Flush the buffer					-RAK-	*/
void flush(void) {
  refresh();
}

/* Clears given line of text				-RAK-	*/
void Erase_Line(const long row, const long col) {
  if (row == MSG_LINE && msg_flag)
    msg_print(CNIL);
  (void)move(row, col);
  clrtoeol();
}

void Clear_From(const int row) {
  (void)move(row, 0);
  clrtobot();
}

/* Outputs a char to a given interpolated y, x position	-RAK-	*/
/* sign bit of a character used to indicate standout mode. -CJS */
void Print(chtype const ch, int row, int col) {
  row -= panel_row_prt; /* Real co-ords convert to screen positions */
  col -= panel_col_prt;

  if (!C_viewport_screen_contains(row, col)) {
    MSG(("error in print, row = %d col = %d\n", row, col));
    abort();
  }
  used_line[row + 1] = true;

  if (mvaddch(row, col, ch) == ERR) {
    MSG(("error in print, row = %d col = %d\n", row, col));
    abort();
  }
}

void prt2(char *str_buff1, char *str_buff2, const int row, const int col) {
  char temp[82];
  sprintf(temp, "%s%s", str_buff1, str_buff2);
  prt(temp, row, col);
}

/* move cursor to a given y, x position */
void move_cursor(const int row, const int col) { (void)move(row, col); }

/* Gets a string terminated by <RETURN>		*/
/* Function returns false if <ESCAPE> is input	*/
bool Get_String(char *in_str, const int row, int column, int slen) {
  int aborted = FALSE;
  int flag = FALSE;
  (void)move(row, column);
  for (int i = slen; i > 0; i--)
    (void)addch(' ');
  (void)move(row, column);
  const int start_col = column;
  int end_col = column + slen - 1;
  if (end_col > 79) {
    /* TODO: slen below is unused. Should it be? */
    slen = 80 - column;
    end_col = 79;
  }
  char *p = in_str;
  do {
    int i = inkey();
    switch (i) {
    case ESCAPE:
      aborted = TRUE;
      break;
    case CTRL_('J'):
    case CTRL_('M'):
      flag = TRUE;
      break;
    case DELETE:
    case CTRL_('H'):
      if (column > start_col) {
        column--;
        put_buffer_(" ", row, column);
        move_cursor(row, column);
        *--p = '\0';
      }
      break;
    default:
      if (!isprint((unsigned char)i) || column > end_col)
        bell();
      else {
        mvaddch(row, column, i);
        *p++ = i;
        column++;
      }
      break;
    }
  } while (!flag && !aborted);
  if (aborted)
    return FALSE;
  /* Remove trailing blanks	*/
  while (p > in_str && p[-1] == ' ')
    p--;
  *p = '\0';
  return TRUE;
}

/* Pauses for user response before returning		-RAK-	*/
void Pause_Line(const int prt_line)
{
  prt_("[Press any key to continue.]", prt_line, 23);
  (void)inkey();
  Erase_Line(prt_line, 0);
}

void bell(void) {
  refresh();

  /* The player can turn off beeps if he/she finds them annoying.  */
  if (!sound_beep_flag)
    return;

  write(1, "\007", 1);
}
