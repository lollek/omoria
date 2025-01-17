#define _GNU_SOURCE

#include <curses.h>
#include <math.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "constants.h"
#include "death.h"
#include "debug.h"
#include "graphics.h"
#include "pascal.h"
#include "save.h"
#include "term.h"
#include "types.h"
#include "unix.h"
#include "variables.h"

#include "io.h"

#define MAX_MESSAGES 50
static char msg_prev[MAX_MESSAGES + 1][82];
static unsigned char record_ctr = 0;

// ReSharper disable once CppParameterNeverUsed
__unused static void signalexit(int unused __unused) {
  ENTER(("signalexit", ""));
  MSG(("Sorry, caught a core-dump signal."));

  msg_print("Sorry, caught a core-dump signal.");
  sav__save_character();
  exit_game();

  LEAVE("signalexit", "");
}

// ReSharper disable once CppParameterNeverUsed
static void signalquit(int unused __unused) {
  ENTER(("signalquit", ""));
  signal(SIGINT, signalquit);
  switch (game_state) {

  case GS_GET_COMMAND:
    death_by_quitting();
    break;

  case GS_IGNORE_CTRL_C:
    break;

  case GS_ALLOW_CTRL_C:
  case GS_LOADING:
    exit_game();
    break;
  }
  LEAVE("signalquit", "");
}

void signalsave(void) {
  sav__save_character();
  exit_game();
}

void no_controly(void) {
  /* { Turn off Control-Y					-RAK-	} */
  /* ok, this is unix not vms, so it turns off ^C and ^Z */

  ENTER(("no_controly", ""));

  signal(SIGINT, signalquit);
#if !DO_DEBUG
  signal(SIGHUP, signalsave);
  signal(SIGTSTP, SIG_IGN);
  signal(SIGQUIT, signalexit);
  signal(SIGILL, signalexit);
  signal(SIGTRAP, signalexit);
  signal(SIGFPE, signalexit);
  signal(SIGSEGV, signalexit);
  signal(SIGIOT, signalexit);
  signal(SIGABRT, signalexit);
  signal(SIGBUS, signalexit);
  signal(SIGSYS, signalexit);
#endif
  LEAVE("no_controly", "");
}

void controly(void) {
  /* { Turn on Control-Y					-RAK-	} */
  /* ok, this is unix not vms, so it turns on ^C and ^Z */
}

void exit_ncurses(void) {
  if (curses_is_running) {
    refresh(); /* { Dump any remaining buffer	} */

    /* clean up the terminal */
    echo();
    nocbreak();
    endwin();
  }
}

void exit_game(void) {
  /*	{ Immediate exit from program } */
  controly(); /* { Turn control-Y back on	} */
  exit_ncurses();
  fflush(stdout);
  exit(0); /* { exit from game		} */
}

void msg_record(char message[82], const bool save) {
  ENTER(("msg_record", "%s, %d", message, save));

  if (save) {
    record_ctr++;
    if (record_ctr > MAX_MESSAGES) {
      record_ctr = 1;
    }
    strcpy(msg_prev[record_ctr], message);
    if (strlen(msg_prev[record_ctr]) > 74) {
      msg_prev[record_ctr][74] = 0;
    }
  } else {
    char ic;
    unsigned char count = 0;
    unsigned char temp_ctr = record_ctr;

    do {
      char fixed_string[134];
      count++;
      /* XXXX pad, dec, what to do? */
      /*prt(pad(msg_prev[temp_ctr],' ',74) + ':' +
       * dec(count,4,3),1,1);*/
      sprintf(fixed_string, "%02d> %s", count, msg_prev[temp_ctr]);
      /* prt(msg_prev[temp_ctr],1,1); */
      prt(fixed_string, 1, 1);
      temp_ctr--;
      if (temp_ctr < 1) {
        temp_ctr = MAX_MESSAGES;
      }
      ic = inkey();
    } while (!(!(ic == 13 || ic == 32 || ic == 86) || count == MAX_MESSAGES));
    msg_print("End of buffer. ");
    /* XXXX another pad, what to do? */
    /*msg_print(pad('End of buffer. ',' ',80));*/
  }

  LEAVE("msg_record", "i");
}

void inkey_delay(char *getchar) {
  /* XXXX check_input consumes the input, so we never actually get data */

  refresh(); /*{ Dump the IO buffer		}*/

  *getchar = 0;
}

void inkey_flush(char *x) {
  refresh(); /*{ Dup the IO buffer	}*/
  if (!wizard1) {
    flush();
  }
  *x = inkey();
}

void clear_rc(long row, long col) {
  /*	{ Clears screen at given row, column }*/

  row--;
  col--;

  for (long i1 = 2; i1 <= 23; i1++) {
    used_line[i1] = false;
  }
  move(row, col);
  clrtobot();
  /*  put_buffer(cursor_erp, row, col); */
  refresh(); /* dump the clear sequence */
}

bool msg_print_pass_one(char *str_buff) /* : varying[a] of char; */
{
  bool return_value = false;
  char ic = 0;

  ENTER(("msg_print", "%s", str_buff));

  if (msg_flag && !msg_terse) {
    long old_len = 0;
    old_len = strlen(old_msg) + 1;
    put_buffer(" -more-", msg_line, old_len);
    do {
      ic = inkey();
    } while (ic != 10 && ic != 13 && ic != 25 && ic != 26 && ic != 27 &&
             ic != 32);
    /* isn't this nicer: until (ord(in_char) in [3,13,25,26,27,32])
     * ? */
  }

  if (str_buff && str_buff[0]) {

    /* put_buffer(cursor_erl+str_buff,msg_line,msg_line);*/
    erase_line(msg_line, msg_line);
    put_buffer(str_buff, msg_line, msg_line);
    strncpy(old_msg, str_buff, sizeof(char[82]));
    msg_record(str_buff, true);

    if (ic == 3 || ic == 25 || ic == 26 || ic == 27) {
      return_value = true;
    } else {
      return_value = false;
    }

    msg_flag = true;
  } else {

    msg_flag = false;
  }

  RETURN("msg_print", "m", 'b', "msg", &return_value);
  return return_value;
}

void msg_printf(char const *const fmt, ...) {
  va_list ap;

  va_start(ap, fmt);
  char *buffer;
  const int result = vasprintf(&buffer, fmt, ap);
  if (result == -1) {
    MSG(("ERROR: Failed to allocate a line for vasprintf!"));
    msg_print("<<Failed to allocate data for this line>>");
    return;
  }

  msg_print(buffer);
  free(buffer);
  va_end(ap);
}

bool msg_print(char *str_buff) /* : varying[a] of char; */
{
  /*{ Outputs message to top line of screen }*/

  char in_char = 0;
  const obj_set big_set = {3, 10, 13, 25, 26, 27, 32, 0};
  const obj_set small_set = {3, 25, 26, 27, 0};
  bool flag;

  if (msg_flag && !msg_terse) {
    const long old_len = strlen(old_msg) + 1;
    put_buffer(" -more-", msg_line, old_len);
    do {
      in_char = inkey();
    } while (!is_in(in_char, big_set));
  }

  erase_line(msg_line, msg_line);
  put_buffer(str_buff, msg_line, msg_line);

  strcpy(old_msg, str_buff);
  msg_record(str_buff, true);

  msg_flag = true;

  if (is_in(in_char, small_set)) {
    flag = true;
  } else {
    flag = false;
  }

  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */

bool get_com(char const *prompt, char *command) {
  if (strlen(prompt) > 1) {
    prt(prompt, 1, 1);
  }
  *command = inkey();

  const bool return_value = !(*command == 3 || *command == 25 || *command == 27);

  erase_line(msg_line, msg_line);
  msg_flag = false;

  return return_value;
}

/*//////////////////////////////////////////////////////////////////// */
void print_str(char const *str_buff, int row, int col) {
  row -= panel_row_prt; /*{ Real co-ords convert to screen positions }*/
  col -= panel_col_prt;
  used_line[row] = true;
  put_buffer(str_buff, row, col);
}

void print_chstr(chtype const *str_buff, int row, int col) {
  /* Real co-ords convert to screen positions */
  row -= panel_row_prt;
  col -= panel_col_prt;
  used_line[row] = true;

  /* Remove 1 like put_buffer does */
  mvaddchstr(row - 1, col - 1, str_buff);
}

bool get_yes_no(char const *prompt) {
  /*{ Gets response to a  Y/N question				}*/
  char const *question_suffix = " (y/n) ";
  char *buf = alloca(strlen(prompt) + strlen(question_suffix) + 1);
  sprintf(buf, "%s%s", prompt, question_suffix);

  for (;;) {
    char command;
    msg_print(" ");
    get_com(buf, &command);
    switch (command) {
    case 'y':
    case 'Y':
      return true;
    case 'n':
    case 'N':
      return false;
    }
  }
}

long get_hex_value(const long row, const long col, const long slen) {
  long return_value = 0;
  char tmp_str[82];

  if (get_string(tmp_str, row, col, slen)) {
    if (strlen(tmp_str) <= 8) {
      sscanf(tmp_str, "%ld", &return_value);
    }
  }

  return return_value;
}

void print_hex_value(const long num, const long row, const long col) {
  char out_val[82];
  sprintf(out_val, "0x%08lx", num);
  prt(out_val, row, col);
}

void pause_game(const long prt_line) { pause_line(prt_line); }
