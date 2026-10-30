#include <curses.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/wait.h>
#include <unistd.h>

#include "configure.h"
#include "debug.h"
#include "help.h"
#include "io.h"
#include "player.h"
#include "screen.h"
#include "term.h"
#include "variables.h"

void help(void) {
  int line;

  C_clear_screen();
  for (line = 0; line < C_help_page_line_count(HELP_PAGE_COMMANDS); ++line) {
    prt(C_help_page_line_text(HELP_PAGE_COMMANDS, line),
        C_help_page_line_row(HELP_PAGE_COMMANDS, line), 1);
  }
  pause_game(24);

  C_clear_screen();
  for (line = 0; line < C_help_page_line_count(HELP_PAGE_SHOPS); ++line) {
    prt(C_help_page_line_text(HELP_PAGE_SHOPS, line),
        C_help_page_line_row(HELP_PAGE_SHOPS, line), 1);
  }
  pause_game(24);
  draw_cave();
}

void moria_help(char const *const help_level) {
  char help_string[1026];
  pid_t help_pid;
  int status;

  game_state = GS_HELP;

  sprintf(help_string, "Imoria %s", help_level);

  if ((help_pid = fork()) == 0) {
    /* child process */
    echo();
    nocbreak();
    C_clear_screen();
    refresh();
    C_mouse_motion_fallback_stop();
    endwin();
    execlp(HELP_FILE_PATH, "mhelp.pl", help_string, (char *)0);
    printf("\n\rUnable to execute: %s\n\r", HELP_FILE_PATH);
    printf("Press return continue.\n\r");
    fflush(stdout);
    getc(stdin);
    exit(1);
  } else if (help_pid == -1) {
    /* parent process on failure */
    msg_print("Fork failed.");
  } else {
    /* parent process on success */

    MSG(("child: %d  watiting...\n", help_pid));

    /* mhelp.pl catches ^Z and exits */
    waitpid(help_pid, &status, 0);

    MSG(("child: %d  status 1: %d\n", help_pid, status));

    /* mhelp.pl catches ^Z and exits */
    waitpid(help_pid, &status, 0);

    MSG(("child: %d  status 2: %d\n", help_pid, status));

    C_mouse_motion_fallback_start();
    cbreak();
    noecho();
    C_clear_screen();
    draw_cave();
  }

  /* printf("\nDisplay help for %s\n\r", help_level); */
}

void ident_char(void) {
  char environment;
  char character;
  bool exit_flag, bleah_flag;

  do {
    exit_flag = false;
    bleah_flag = true;
    msg_print("Identify letter for what environ? (<t>own, "
              "<d>ungeon, <w>ater) ");
    if (get_com("", &environment)) {
      switch (environment) {
      case 't':
      case 'd':
      case 'w':
        exit_flag = true;
        break;
      }
    } else {
      bleah_flag = false;
      exit_flag = true;
    }
  } while (!exit_flag);

  if (bleah_flag && get_com("Enter character to be identified : ", &character)) {
    if (character == '@') {
      prt(player_name, 1, 1);
    } else {
      prt(C_ident_char_description(environment, character), 1, 1);
    }
  }
}
