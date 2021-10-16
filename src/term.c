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
#include <time.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "configure.h"
#include "constants.h"
#include "debug.h"
#include "magic.h"
#include "pascal.h"
#include "routines.h"
#include "save.h"
#include "term.h"
#include "types.h"
#include "variables.h"

/* source/io.c: terminal I/O code, uses the curses package

   Copyright (c) 1989-94 James E. Wilson, Robert A. Koeneke

   This software may be copied and distributed for educational, research, and
   not for profit purposes provided that this copyright and statement are
   included in all such copies. */

static void minor_error(char const *error_message) {
  abort();
  /* clear msg_flag to avoid problems with unflushed messages */
  msg_flag = 0;
  prt_(error_message, 0, 0);
  bell();
  /* wait so user can see error */
  sleep(2);
}

void put_buffer_attr(const char *out_str, long row, long col, int attrs) {
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
char inkey() {
  int i;

  refresh();         /* Dump IO buffer		*/
  command_count = 0; /* Just to be safe -CJS- */
  while (TRUE) {
    i = getch();
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
    if (i != CTRL_('R')) {
      msg_flag = false;
      return (char)i;
    }
    refresh();
  }
}

/* Flush the buffer					-RAK-	*/
void flush() {
  /* the code originally used ioctls, TIOCDRAIN, or TIOCGETP/TIOCSETP, or
     TCGETA/TCSETAF, however this occasionally resulted in loss of output,
     the happened especially often when rlogin from BSD to SYS_V machine,
     using check_input makes the desired effect a bit clearer */
  /* wierd things happen on EOF, don't try to flush input in that case */
  if (!eof_flag)
    while (check_input(0))
      ;
  /* used to call refresh() here to drain output, but it is not necessary
   */
}

/* Clears given line of text				-RAK-	*/
void Erase_Line(long row, long col) {
  if (row == MSG_LINE && msg_flag)
    msg_print(CNIL);
  (void)move((int)row, (int)col);
  clrtoeol();
}

void Clear_From(int row) {
  (void)move(row, 0);
  clrtobot();
}

/* Outputs a char to a given interpolated y, x position	-RAK-	*/
/* sign bit of a character used to indicate standout mode. -CJS */
void Print(chtype const ch, int row, int col) {
  char tmp_str[82];

  row -= panel_row_prt; /* Real co-ords convert to screen positions */
  col -= panel_col_prt;

  used_line[row + 1] = true;

  if ((row > 24) || (row < 0) || (col > 79) || (col < 0)) {
    sprintf(tmp_str, "error in print, row = %d col = %d\n", row, col);
    minor_error(tmp_str);
  }

  if (mvaddch(row, col, ch) == ERR) {
    sprintf(tmp_str, "error in print, row = %d col = %d\n", row, col);
    minor_error(tmp_str);
  }
}

void prt2(char *str_buff1, char *str_buff2, int row, int col) {
  char temp[82];
  sprintf(temp, "%s%s", str_buff1, str_buff2);
  prt(temp, row, col);
}

/* move cursor to a given y, x position */
void move_cursor(int row, int col) { (void)move(row, col); }

/* Gets a string terminated by <RETURN>		*/
/* Function returns false if <ESCAPE> is input	*/
boolean Get_String(char *in_str, int row, int column, int slen) {
  register int start_col, end_col, i;
  char *p;
  int flag, aborted;

  aborted = FALSE;
  flag = FALSE;
  (void)move(row, column);
  for (i = slen; i > 0; i--)
    (void)addch(' ');
  (void)move(row, column);
  start_col = column;
  end_col = column + slen - 1;
  if (end_col > 79) {
    /* TODO: slen below is unused. Should it be? */
    slen = 80 - column;
    end_col = 79;
  }
  p = in_str;
  do {
    i = inkey();
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
      if (!isprint(i) || column > end_col)
        bell();
      else {
        mvaddch(row, column, (char)i);
        *p++ = i;
        column++;
      }
      break;
    }
  } while ((!flag) && (!aborted));
  if (aborted)
    return (FALSE);
  /* Remove trailing blanks	*/
  while (p > in_str && p[-1] == ' ')
    p--;
  *p = '\0';
  return (TRUE);
}

/* Pauses for user response before returning		-RAK-	*/
void Pause_Line(prt_line) int prt_line;
{
  prt_("[Press any key to continue.]", prt_line, 23);
  (void)inkey();
  Erase_Line(prt_line, 0);
}

void bell() {
  refresh();

  /* The player can turn off beeps if he/she finds them annoying.  */
  if (!sound_beep_flag)
    return;

  write(1, "\007", 1);
}
