#include <curses.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "../constants.h"
#include "../io.h"
#include "../kickout.h"
#include "../misc.h"
#include "../player.h"
#include "../random.h"
#include "../screen.h"
#include "../term.h"
#include "../variables.h"

#include "blackjack.h"
#include "horseracing.h"
#include "slotmachine.h"
#include "casino_local.h"
#include "casino.h"

long bet;
long gld;
long tics;
bool c_closed;

void c__display_gold(void) {
  char out_val[82];

  sprintf(out_val, "gold remaining : %ld   ", gld);
  prt(out_val, 19, 22);
}

bool c__get_response(char comment[82], long *num) {
  long i1;

  bool flag = true;
  i1 = 0;
  const long clen = strlen(comment) + 2;

  do {
    char out_val[82];
    prt(comment, 1, 1);
    msg_flag = false;
    if (!get_string(out_val, 1, clen, 40)) {
      flag = false;
      erase_line(msg_line, msg_line);
    }
    sscanf(out_val, "%ld", &i1);
  } while (!(i1 != 0 || !flag));

  if (flag) {
    *num = i1;
  }

  return flag;
}

void c__change_money(void) {

  const int amount = labs(player_money[TOTAL_] - gld) * GOLD_VALUE;
  if (gld > player_money[TOTAL_]) {
    add_money(amount);
  } else {
    subtract_money(amount, true);
  }
}

void c__check_casino_kickout(void) {
  if (tics % 2 == 1) {
    if (kick__should_kickout()) {
      c__change_money();
      kick__kickout_player_if_time();
    }
  }
  tics++;
}

void c__display_casino(void) {
  char shop_owner[82];

  C_clear_screen();
  strcpy(shop_owner, "Darkon           (Master-Hacker)            Casino");
  prt(shop_owner, 4, 10);
  prt("Game:                                                  Max Bet", 6, 4);
  prt("a) slots                                                  10000", 7, 1);
  prt("b) blackjack                                               1000", 8, 1);
  prt("c) horse racing                                            1000", 9, 1);
  c__display_gold();
  prt("You may:", 21, 1);
  prt(" p) Play a game.                  h) Help on game rules.", 22, 2);
  prt("^R) Redraw the screen.          Esc) Exit from building.", 23, 2);
}

void c__play_game(char game) {
  bool exit_flag = false;

  do {
    if (game == 0) {
      msg_print("Which game do you want to play?      ");
      exit_flag = !get_com("", &game);
    }

    if (!exit_flag) {
      switch (game) {
      case 97:
        start_slot_machine();
        exit_flag = true;
        c__display_casino();
        break;

      case 98:
        start_blackjack();
        exit_flag = true;
        c__display_casino();
        break;

      case 99:
        start_horse_racing();
        exit_flag = true;
        c__display_casino();
        break;

      default:
        prt("That game does not exist, try again.", 1, 1);
        break;
      }
    } else {
      exit_flag = true;
    }
  } while (!exit_flag);
}

void c__parse_command(void) {
  char command;
  bool exit_flag = false;

  do {
    if (get_com("", &command)) {
      switch (command) {
      case 97:
      case 98:
      case 99:
        c__play_game(command);
        break;

      case 112:
        c__play_game(0);
        break;
      case 18:
        c__display_casino();
        break;
      default:
        prt("Invalid Command.", 1, 1);
        break;
      }
    } else {
      exit_flag = true;
    }
  } while (!(exit_flag || c_closed));
}

void c__exit_messages(void) {
  if (gld > 2 * player_money[TOTAL_] + 1000) {
    switch (randint(3)) {
    case 1:
      msg_print("Quitting while you're ahead, huh?");
      break;
    case 2:
      msg_print("Lady luck must be on you side.");
      break;
    case 3:
      msg_print("A pair of heavily armed thugs show you to "
                "the door.");
      break;
    }
  } else if (gld < player_money[TOTAL_] - 1000) {
    switch (randint(4)) {
    case 1:
      msg_print("KC thanks you for your patronage.");
      break;
    case 2:
      msg_print("KC personally escorts you to the door.");
      break;
    case 3:
      msg_print("Better luck next time.");
      break;
    case 4:
      msg_print("You leave a sadder and wiser man.");
      break;
    }
  } else {
    msg_print("Bye.");
  }
  /* msg_print(""); */
}

void enter_casino(void) {
  c_closed = false;
  tics = 1;
  gld = player_money[TOTAL_];
  msg_line = 1;
  c__display_casino();
  c__parse_command();
  c__exit_messages();
  c__change_money();
  draw_cave();
}

/* END FILE  casino.c */
