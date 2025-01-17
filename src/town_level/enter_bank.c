#include <curses.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "../constants.h"
#include "../currency.h"
#include "../inven.h"
#include "../io.h"
#include "../kickout.h"
#include "../misc.h"
#include "../player.h"
#include "../port.h"
#include "../random.h"
#include "../screen.h"
#include "../stores.h"
#include "../variables.h"

void eb__display_money(void);
void eb__display_store(const char *shop_owner);

#define BANK_SKIM 0.95 /*{ Percent of money that really gets deposited} */

static bool eb__get_entry(char comment[82], long *num) {
  /*
   * Returns true if a number >= 0 is entered, false if escaped,
   * negative numbers not permitted.                      -MKC-
   * */

  bool return_value = false;
  bool valid;

  *num = -1;
  do {
    char in_val[82];
    valid = true;
    msg_print(comment);
    msg_flag = false;
    if (get_string(in_val, 1, strlen(comment) + 2, 40)) {
      *num = atoi(in_val);
      if (*num >= 0) {
        return_value = true;
      } else {
        msg_print("Invalid entry.");
        valid = false;
      }
    } else {
      /* user escaped out */
      erase_line(msg_line, msg_line);
    }
  } while (!valid);

  return return_value;
}

static void eb__dep_munny(const long mon_type) {
  long deposit;

  if (player_money[mon_type] <= 0) {
    return;
  }
  do {
    char out_val[120];
    sprintf(out_val, "How much %s to deposit?", coin_name[mon_type]);
    if (eb__get_entry(out_val, &deposit)) {
      if (deposit > player_money[mon_type]) {
        sprintf(out_val, "You do not have that much %s!", coin_name[mon_type]);
        msg_print(out_val);
      }
    }
  } while (deposit > player_money[mon_type]);

  if (deposit > 0) {
    bank[mon_type] += deposit;
    player_money[mon_type] -= deposit;
    inven_weight -= COIN_WEIGHT * deposit;
    player_account +=
        trunc(deposit * BANK_SKIM * coin_value(mon_type)) / GOLD_VALUE;
    eb__display_money();
  }
}

static void eb__deposit_money(void) {
  /*
   * Deposit a given number of mithril, platinum, and gold in the bank,
   * but the bank takes its percentage                      -ADW-MKC-
   */

  eb__dep_munny(MITHRIL);
  eb__dep_munny(PLATINUM);
  eb__dep_munny(GOLD);
  reset_total_cash();
  eb__display_money();
}

static void eb__withdraw_money(void) {
  /*
   * Withdraw a given amount, in gold pieces, teller makes change.
   * Bank may not have enough!                              -MKC-
   */

  long withdraw;

  /* get amount to withdraw */
  do {
    if (eb__get_entry("How much money to withdraw (in gold)?", &withdraw)) {
      if (withdraw > player_account) {
        msg_print("You do not have that much!");
        msg_print("You should keep better track of "
                  "your account!");
      }
    }
  } while (withdraw > player_account);

  /*
   * The amount actually given is the minimum of how much the user wants,
   * how much the bank has, and how much the user can carry.
   */
  if (withdraw > 0) {
    long mon_type;
    long amt_given[MITHRIL + 1];
    long weight_left = C_player_max_bulk() * 100 - inven_weight;
    for (mon_type = MITHRIL; mon_type >= GOLD; mon_type--) {
      amt_given[mon_type] = min3(withdraw * GOLD_VALUE / coin_value(mon_type),
                                 bank[mon_type], weight_left / COIN_WEIGHT);
      weight_left -= amt_given[mon_type] * COIN_WEIGHT;
      withdraw -= amt_given[mon_type] * (coin_value(mon_type) / GOLD_VALUE);
    } /* end for mon_type */

    bool deliver = true;
    const bool is_some =
        amt_given[MITHRIL] + amt_given[PLATINUM] + amt_given[GOLD] > 0;

    if (withdraw > 0) {
      /*{ if unable to give the entire amount }*/

      if (weight_left <= 0) { /*{ they can't carry it all }*/
        if (is_some) {
          msg_print("You cannot carry it all.");
          deliver = get_yes_no("Do you want as much as "
                               "you can carry?");
        } else {
          msg_print("You cannot carry any more money.");
          deliver = false;
        }
      } else { /* { the bank doesn't have enough money }*/
        if (is_some) {
          msg_print("We are rather short of cash "
                    "today.");
          deliver = get_yes_no("Do you want as much as "
                               "you can get?");
        } else {
          msg_print("We do not have the cash to "
                    "give you.");
          msg_print("Try again tomorrow.");
          deliver = false;
        }
      } /* endif weight_left */
    }   /* withdraw */

    if (deliver) {
      for (mon_type = MITHRIL; mon_type >= GOLD; mon_type--) {
        if (amt_given[mon_type] > 0) {
          char out_val[134];
          sprintf(out_val,
                  "The teller gives you "
                  "%ld %s piece%s.",
                  amt_given[mon_type], coin_name[mon_type],
                  amt_given[mon_type] == 1 ? "" : "s");
          msg_print(out_val);
          player_money[mon_type] += amt_given[mon_type];
          bank[mon_type] -= amt_given[mon_type];
          player_account -=
              amt_given[mon_type] * coin_value(mon_type) / GOLD_VALUE;
        }
      } /* end for */
      inven_weight = C_player_max_bulk() * 100 - weight_left;
      reset_total_cash();
      eb__display_money();
    } /* end if deliver */
  }   /* end if withdraw */
}

static void eb__safe_deposit(__attribute__((unused)) bool deposit) {
  /* XXXX major work. I don't think it ever worked at the U */
  prt("The dwarves are still installing it, sorry.", 1, 1);
}

static void eb__change_money(void) {
  /* Changes money of one type to money of another type.        -JPS- */

  bool change_flag;   /*{ Did they enter a valid entry? }*/
  long amount_from;      /*{ Amount before changing. }*/
                         /*{ Amount remaining after changing. }*/
                         /*{ input character }*/
  long typ_from, typ_to; /*   { Types of money }*/

  char key_in = get_money_type("Change what coin? ", &change_flag, false);
  if (change_flag) {
    coin_stuff(key_in, &typ_from);
    key_in = (char)get_money_type("Change to? ", &change_flag, true);
  }
  if (change_flag) {
    char prompt[134];
    coin_stuff(key_in, &typ_to);
    sprintf(prompt, "Number of coins to change? (1-%ld)",
            player_money[typ_from]);
    change_flag = eb__get_entry(prompt, &amount_from);
  }
  if (change_flag) {
    const long amount_to = amount_from * coin_value(typ_from) /
                     coin_value(typ_to); /*{NO surcharge}*/
    if (amount_to == 0) {
      msg_print("You don't have enough to trade for that "
                "type of coin!");
    } else if (amount_to > bank[typ_to]) {
      msg_print("The bank doesn't have enough of that kind "
                "of coin!");
    } else if (player_money[typ_from] < amount_from) {
      msg_print("You don't have enough of that coin!");
    } else if (inven_weight + COIN_WEIGHT * (amount_to - amount_from) >
               C_player_max_bulk() * 100) {
      msg_print("You can't carry that much weight.");
    } else {
      player_money[typ_from] -= amount_from;
      bank[typ_from] += amount_from;
      player_money[typ_to] += amount_to;
      bank[typ_to] -= amount_to;
      inven_weight += COIN_WEIGHT * (amount_to - amount_from);
      msg_print("The money changer hands you your money.");
      eb__display_money();
    }
  } /* endif change_flag */
}

static void eb__parse_command(bool *exit_flag, char shop_owner[82]) {
  char command;

  *exit_flag = false;

  if (get_com(" ", &command)) {
    switch (command) {
    case CTRL_R:
      eb__display_store(shop_owner);
      break;
    case 'd':
      eb__deposit_money();
      break;
    case 'w':
      eb__withdraw_money();
      break;
    case 'c':
      eb__change_money();
      break;
    case 'i':
      prt("The insurance shop has gone out of business.", 1, 1);
      break;
    case 'p':
      eb__safe_deposit(true);
      break;
    case 'r':
      eb__safe_deposit(false);
      break;
    default:
      prt("Invalid Command.", 1, 1);
      break;

    } /* end switch */
  } else {
    *exit_flag = true;
  }
}

void enter_bank(void) {
  bool exit_flag = false;
  long tics = 1;
  char shop_owner[82];

  switch (randint(7)) {
  case 1:
    strcpy(shop_owner, "Milton Drysdale      (tightwad)      Bank");
    break;
  case 2:
    strcpy(shop_owner, "Mr. Potter           (slumlord)      Bank");
    break;
  case 3:
    strcpy(shop_owner, "Ebeneezer Scrooge    (broker)        Bank");
    break;
  case 4:
    strcpy(shop_owner, "Scrooge McDuck       (avian)         Bank");
    break;
  case 5:
    strcpy(shop_owner, "Andrew Mellon        (treasury)      Bank");
    break;
  case 6:
    strcpy(shop_owner, "Loony Looby          (pizza!)        Bank");
    break;
  case 7:
    strcpy(shop_owner, "Ram the Booger Eater (Nosepicker)    Bank");
    break;
  }
  eb__display_store(shop_owner);

  do {
    eb__parse_command(&exit_flag, shop_owner);
    adv_time(false);
    tics++;

    if (tics % 2 == 1) {
      kick__kickout_player_if_time();
    }
  } while (!exit_flag);
  draw_cave();
}
