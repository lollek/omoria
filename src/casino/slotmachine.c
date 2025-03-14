/* slots.c */
/* slot machine code for the casino */

#include <curses.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "../io.h"
#include "../random.h"
#include "../term.h"

#include "casino_local.h"
#include "slotmachine.h"

#define xS_JACKPOT 1
#define xS_CHERRY 2
#define xS_ORANGE 3
#define xS_BELL 4
#define xS_BAR 5

#define S_JACKPOT 0
#define S_CHERRY 1
#define S_ORANGE 2
#define S_BELL 3
#define S_BAR 4

typedef long stype;    /* stype = (jackpot,cherry,orange,bell,bar);*/
typedef stype slot[4]; /* slot  = array[1..3] of stype;*/

char *s_name[6] = {"jackpot", "cherry", "orange", "bell", "bar"};
slot slotpos;

static void sm__display_slot_options(void) {
  prt(" -------------------------------------------           ", 2, 11);
  prt("|                                           |   _-_     ", 3, 11);
  prt("|                                           |  /   \\     ", 4, 11);
  prt("| XXXXXXXXXXXXX XXXXXXXXXXXXX XXXXXXXXXXXXX | (     )    ", 5, 11);
  prt("| X           X X           X X           X |  \\ _ /     ", 6, 11);
  prt("| X           X X           X X           X |   | |      ", 7, 11);
  prt("| X           X X           X X           X |   | |      ", 8, 11);
  prt("| XXXXXXXXXXXXX XXXXXXXXXXXXX XXXXXXXXXXXXX |   | |      ", 9, 11);
  prt("|                                           |   | |      ", 10, 11);
  prt("|      X              X             X       |   | |      ", 11, 11);
  prt("|     XXX            XXX           XXX      |__/  |      ", 12, 11);
  prt("|      X              X             X       |     |      ", 13, 11);
  prt("|                                           |____/       ", 14, 11);
  prt("|                                           |             ", 15, 11);
  prt("|                                           |             ", 16, 11);
  prt("|                                           |             ", 17, 11);
  prt(" -------------------------------------------", 18, 11);

  prt("You may:", 21, 1);
  prt(" p) pull lever.                   d) display prizes.", 22, 2);
  prt("^R) Redraw the screen.          Esc) Return to main menu.", 23, 2);
}

static void sm__position_adjust(long *c1, long *c2, long *c3) {
  /*  Centers slots in middle of box */

  *c1 = 15;
  *c2 = 29;
  *c3 = 43;

  /*  if (slotpos[1] > 1) { *c1 = 14; } */
  /*  if (slotpos[2] > 1) { *c2 = 28; } */
  /*  if (slotpos[3] > 1) { *c3 = 42; } */
}

static void sm__display_slots(void) {
  char out_val[82];
  long c1, c2, c3;

  C_clear_screen();
  sm__display_slot_options();
  sm__position_adjust(&c1, &c2, &c3);
  sprintf(out_val, "%s", s_name[slotpos[1]]);
  put_buffer(out_val, 7, c1);
  sprintf(out_val, "%s", s_name[slotpos[2]]);
  put_buffer(out_val, 7, c2);
  sprintf(out_val, "%s", s_name[slotpos[3]]);
  put_buffer(out_val, 7, c3);
  c__display_gold();
}

static void sm__display_prizes(void) {
  char command;

  C_clear_screen();
  prt("                                1     2      5     10     25     "
      "50 ",
      4, 1);
  prt("jackpot  jackpot  jackpot     1000  2000   5000   1000  25000  "
      "50000",
      6, 1);
  prt("  bar      bar      bar         30    60    150    300    750   "
      "1500",
      7, 1);
  prt("  bell     bell     bell        15    30     75    150    375    "
      "750",
      8, 1);
  prt(" orange   orange   orange        8    16     40     80    200    "
      "400",
      9, 1);
  prt(" cherry   cherry   cherry        4     8     20     40    100    "
      "200",
      10, 1);
  prt(" bell      ---      bell         4     8     20     40    100    "
      "200",
      11, 1);
  prt("  ---      bar      bar          4     8     20     40    100    "
      "200",
      12, 1);
  prt("  ---    orange   orange         2     4     10     20     50    "
      "100",
      13, 1);
  prt(" jackpot   ---      ---          2     4     10     20     50    "
      "100",
      14, 1);
  prt(" cherry   cherry    ---          1     2      5     10     25     "
      "50",
      15, 1);
  prt("[hit any key to continue]", 22, 27);
  get_com("", &command);
  sm__display_slots();
}

static void sm__get_slots(void) {

  /*  Wheel one  */
  long c = randint(20);
  if (c >= 20) {
    slotpos[1] = S_JACKPOT;
  } else if (c >= 17) {
    slotpos[1] = S_BAR;
  } else if (c >= 13) {
    slotpos[1] = S_BELL;
  } else if (c >= 8) {
    slotpos[1] = S_ORANGE;
  } else if (c >= 1) {
    slotpos[1] = S_CHERRY;
  }

  /*  Wheel two  */
  c = randint(20);
  if (c >= 20) {
    slotpos[2] = S_JACKPOT;
  } else if (c >= 17) {
    slotpos[2] = S_BAR;
  } else if (c >= 11) {
    slotpos[2] = S_BELL;
  } else if (c >= 7) {
    slotpos[2] = S_ORANGE;
  } else if (c >= 1) {
    slotpos[2] = S_CHERRY;
  }

  /*  Wheel three  */
  c = randint(20);
  if (c >= 20) {
    slotpos[3] = S_JACKPOT;
  } else if (c >= 16) {
    slotpos[3] = S_BAR;
  } else if (c >= 13) {
    slotpos[3] = S_BELL;
  } else if (c >= 7) {
    slotpos[3] = S_ORANGE;
  } else if (c >= 1) {
    slotpos[3] = S_CHERRY;
  }
}

static void sm__clearslots(const long line) {
  /*  clears a line of slots */

  char killpos[82];

  strcpy(killpos, "        ");
  put_buffer(killpos, line, 15);
  put_buffer(killpos, line, 29);
  put_buffer(killpos, line, 43);
}

static void sm__print_slots(void) {
  /* Simulates wheel spinning  */

  long c1, c2, c3;

  sm__get_slots(); /*  {get new slots}  */

  for (long i = 1; i <= 9; i++) {
    char out_val[82];
    sm__clearslots(7);                  /*  {clear middle row}  */
    sm__position_adjust(&c1, &c2, &c3); /*  {center bar and bell} */

    sprintf(out_val, "%s", s_name[slotpos[1]]);
    put_buffer(out_val, 8, c1);
    sprintf(out_val, "%s", s_name[slotpos[2]]); /*{print bottom}*/
    put_buffer(out_val, 8, c2);
    sprintf(out_val, "%s", s_name[slotpos[3]]);
    put_buffer(out_val, 8, c3);

    sm__get_slots(); /*{get new slots}*/
    sm__position_adjust(&c1, &c2, &c3);

    sprintf(out_val, "%s", s_name[slotpos[1]]);
    put_buffer(out_val, 6, c1);
    sprintf(out_val, "%s", s_name[slotpos[2]]); /*{print top row}*/
    put_buffer(out_val, 6, c2);
    sprintf(out_val, "%s", s_name[slotpos[3]]);
    put_buffer(out_val, 6, c3);

    refresh();
    usleep(50);
    sm__clearslots(6); /*{clear top row}*/
    refresh();
    usleep(50);
    sm__clearslots(8); /*{clear bottom row}*/

    sprintf(out_val, "%s", s_name[slotpos[1]]);
    put_buffer(out_val, 7, c1);
    sprintf(out_val, "%s", s_name[slotpos[2]]); /*{print middle row}*/
    put_buffer(out_val, 7, c2);
    sprintf(out_val, "%s", s_name[slotpos[3]]);
    put_buffer(out_val, 7, c3);

    refresh();
    usleep(100);
  }
}

static void sm__winnings(void) {
  /* calculates the amount won */ /* Currently, odds slightly favor  */
                                  /* the user.   Return of 101%      */

  char comment[82];
  char comment1[82];

  strcpy(comment, "You have won ");
  strcpy(comment1, " gold pieces!");

  long winning = 0;
  if (slotpos[1] == slotpos[2] && slotpos[1] == S_CHERRY) {
    winning = bet;
  }
  if (slotpos[1] == S_JACKPOT) {
    winning = bet * 2;
  }
  if (slotpos[2] == slotpos[3] && slotpos[2] == S_ORANGE) {
    winning = 2 * bet;
  }
  if (slotpos[1] == slotpos[3] && slotpos[1] == S_BELL) {
    winning = 4 * bet;
  }
  if (slotpos[2] == slotpos[3] && slotpos[2] == S_BAR) {
    winning = 4 * bet;
  }

  if (slotpos[1] == slotpos[2] && slotpos[1] == slotpos[3]) {
    switch (slotpos[1]) {
    case S_JACKPOT:
      winning = 1000 * bet;
      break;
    case S_CHERRY:
      winning = 4 * bet;
      break;
    case S_ORANGE:
      winning = 8 * bet;
      break;
    case S_BELL:
      winning = 15 * bet;
      break;
    case S_BAR:
      winning = 30 * bet;
      break;
    }
  }

  if (winning == 0) {
    switch (randint(5)) {
    case 1:
      msg_print("You lose.                              ");
      break;
    case 2:
      msg_print("Your money pouch feels a little lighter.");
      break;
    case 3:
      msg_print("Rats!  Lost again!                     ");
      break;
    case 4:
      msg_print("The casino owner becomes richer.       ");
      break;
    case 5:
      msg_print("Apparently you don't know the secret to "
                "winning at slots.");
      break;
    }
  } else {
    if (winning > bet) {
      char out_val[300];
      switch (randint(5)) {
      case 1:
        msg_print("Hmmm...Maybe this system really works...");
        break;
      case 2:
        msg_print("Maybe you should quit while you''re "
                  "ahead.");
        break;
      case 3:
        msg_print("Coins begin to pour out of the machine.");
        break;
      case 4:
        msg_print("You're not cheating, are you?");
        break;
      case 5:
        msg_print("Be sure to report your winnings!");
        break;
      }
      sprintf(out_val, "%s%ld%s", comment, winning, comment1);
      msg_print(out_val);
    } else {
      msg_print("You break even.");
    }
    gld += winning;
  }
}

static void sm__get_slots_bet(void) {
  char comment[82];
  long num;
  bool exit_flag = false;

  strcpy(comment, "Which machine (1 to 10000 gp)? ");

  do {
    if (c__get_response(comment, &num)) {
      bet = num;
      if (bet > 0 && bet < 10001) {
        exit_flag = true;
      } else {
        prt("Improper value.", 1, 1);
      }
    } else {
      exit_flag = true;
      bet = 0;
    }
  } while (!exit_flag);

  if (bet > gld) {
    prt("You have not the gold!", 1, 1);
    bet = 0;
  }
}

static void sm__slot_commands(void) {
  char command;
  bool exit_flag = false;

  bet = 0;

  do {
    if (get_com("", &command)) {
      switch (command) {
      case 112:
        sm__get_slots_bet();
        break;
      case 18:
        sm__display_slots();
        break;
      case 100:
        sm__display_prizes();
        break;
      default:
        prt("Invalid Command.", 1, 1);
        break;
      }
    } else {
      exit_flag = true;
    }

    if (bet > 0) {
      gld -= bet;
      sm__print_slots();
      sm__winnings();
      bet = 0;
      c__display_gold();
    }
    c__check_casino_kickout();
  } while (!exit_flag);
}

void start_slot_machine(void) {
  C_clear_screen();
  slotpos[1] = S_JACKPOT;
  slotpos[2] = S_JACKPOT;
  slotpos[3] = S_JACKPOT;
  sm__display_slots();
  prt("You are standing in front of a row of odd looking machines.", 1, 1);
  sm__slot_commands();
}

/* END FILE slots.c */
