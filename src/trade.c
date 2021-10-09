/* trade.c */
/* routines to handle the trading post */

#include <curses.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/file.h> /* for flock     */
#include <time.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "configure.h"
#include "constants.h"
#include "debug.h"
#include "kickout.h"
#include "magic.h"
#include "pascal.h"
#include "player.h"
#include "routines.h"
#include "store.h"
#include "term.h"
#include "trade.h"
#include "types.h"
#include "variables.h"

#define ROUND(x) ((long)((x) + .5))

#define T_DISPLAY_SIZE 12
#define T_ACCEPTABLE_ITEM_PRICE 50
#define T_PROFIT_FROM_BID 0.05
#define T_PROFIT_FROM_SALE 0.25
#define T_REFUND_ON_BID (1.00 - T_PROFIT_FROM_BID)
#define T_REFUND_ON_SALE (1.00 - T_PROFIT_FROM_SALE)
#define T_BID_INCREMENT_FACTOR 1.05
#define T_TAKE_THE_MONEY_AND_RUN 0.90
#define T_BID_WAIT_DAYS 0
#define T_BID_WAIT_HOURS 6
#define T_EXPIRE_TIME_DAYS 4
#define T_EXPIRE_TIME_HOURS 0

#define TT_PROFIT 0
#define TT_FOR_SALE 1
#define TT_CASH 2

typedef struct trade_account_type {
  uid_t uid;
  char username[13];
  long master_id;
  long claim_check;
} trade_account_type;

typedef struct profit_record {
  long trade_type;
  long time;
  long money;
} profit_record;

typedef struct for_sale_record {
  long trade_type;
  long time;
  treasure_type object;
  trade_account_type seller;
  long bid_time;
  long best_bid;
  trade_account_type best_bidder;
} for_sale_record;

typedef struct cash_record {
  long trade_type;
  long time;
  long amount;
  trade_account_type owner;
} cash_record;

typedef union trade_record_type {
  struct profit_record pr;
  struct for_sale_record fsr;
  struct cash_record cr;
} trade_record_type;

typedef struct pinven_record {
  struct pinven_record *prev;
  struct pinven_record *next;
  trade_record_type data;
} pinven_record;

typedef pinven_record *pinven_ptr;

/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
#if 0
void check_list(pinven_ptr *inv, pinven_ptr *item)
{
  int          c1,c2;
  pinven_ptr   x, oldx = NULL;

  x = *inv;
  for (c1 = 0; x != NULL; c1++ ) {
    oldx  =  x;
    x     =  x->next;
  }  

  x = oldx;
  for (c2 = 0; x != NULL; c2++ ) {
    x = x->prev;
  }  

  if (item != NULL) {
    x = *item;
    for (c1 = 0; x != NULL; c1++ ) {
      oldx  =  x;
      x     =  x->next;
    }  
    
    x = oldx;
    for (c2 = 0; x != NULL; c2++ ) {
      x = x->prev;
    }  
  }

}
#endif
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
boolean trade_file_open(FILE **tf, boolean *busy) {
  /* open the tradeing post db, returns true if file could be opened.
     if busy is true then the flock failed (tf will be null) */

  int trys;
  boolean return_value = false;

  *tf = fopen(TRADE_FILE, "r+");
  *busy = true;

  if (*tf != NULL) {
    return_value = true;

    for (trys = 0; (trys < 10) && *busy; trys++) {
      if (flock((int)fileno(*tf), LOCK_EX | LOCK_NB) == 0) {
        *busy = false;
      } else {
        sleep(1);
      }
    }

    if (*busy) {
      fclose(*tf);
      *tf = NULL;
    }
  }

  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void trade_file_close(FILE **tf) {
  /* save changes and close the tradeing post db */

  if (*tf != NULL) {
    flock((int)fileno(*tf), LOCK_UN);
    fclose(*tf);
    *tf = NULL;
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void tp__open_trade_file(FILE **sales, boolean *exit_flag) {
  boolean busy;

  *exit_flag = true;

  msg_print("You knock on the door to the Trading Post. . . ");
  refresh();

  if (!trade_file_open(sales, &busy)) {
    msg_print("and the doors are locked. Only a moria "
              "wizard can open them.");
  } else if (busy) {
    msg_print("but the storekeeper is helping someone else.");
  } else {
    if (player_max_exp < 30 + randint(30)) {
      trade_file_close(sales);
      msg_print("`Hmmmm...we don't need no novice "
                "adventurers hanging around here...'");
      msg_print("`Now GET LOST!!!'");
      msg_print("A couple of huge fighters appear from "
                "behind a curtain...");
      take_hit(damroll("2d6"), "some burly barbarians.");
      msg_print("They heave you unceremoniously outside.");
    } else {
      *exit_flag = false;
    }
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void tp__set_player(trade_account_type *cur_player) {
  char thename[82];

  user_name(thename);
  strncpy(cur_player->username, thename, 12);

  cur_player->uid = getuid();
  cur_player->master_id = player_creation_time;
  cur_player->claim_check = player_claim_check;

  if (!cur_player->claim_check) {
    sleep(2);
    cur_player->claim_check = player_claim_check = time(NULL);
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void tp__display_gold() {
  char out_val[82];
  sprintf(out_val, "Gold Remaining : %ld", player_money[TOTAL_]);
  prt(out_val, 19, 18);
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void tp__display_commands() {
  prt("You may:", 21, 1);
  prt(" p) Bid on an item.            <space> browse store's inventory.", 22,
      1);
  prt(" s) Put an item up for bid.    i) Inventory and Equipment Lists.", 23,
      1);
  prt("^R) Redraw the screen.       Esc) Exit from Building.", 24, 1);
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void tp__clear_display(long *cur_display_size, pinven_ptr cur_display[]) {
  long index;

  *cur_display_size = 0;
  for (index = 1; index <= T_DISPLAY_SIZE; index++) {
    cur_display[index] = NULL;
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void tp__read_inv(FILE *sales, pinven_ptr *inv, pinven_ptr *cur_top,
                  trade_record_type *profits) {
  int got;
  char out_val[134];
  pinven_ptr item = NULL;
  boolean first, done;

  rewind(sales);

  *inv = NULL;
  *cur_top = NULL;
  first = true;
  done = false;

  profits->pr.trade_type = TT_PROFIT;
  profits->pr.money = 0;
  profits->pr.time = 0;

  while (!feof(sales) && !done) {
    item = (pinven_ptr)safe_malloc(sizeof(pinven_record), "tp__read_inv");
    got = read((int)fileno(sales), &(item->data), sizeof(trade_record_type));
    if (got == 0) {
      done = true;
      dispose(item, sizeof(inven_record), "tp__read_inv: read 0");
    } else if (got != sizeof(trade_record_type)) {
      sprintf(out_val, "Error reading inventory: %d. %s", got,
              "Please report this!");
      msg_print(out_val);
      msg_print("");
      dispose(item, sizeof(inven_record), "tp__read_inv: bad read");
    } else {
      if (item->data.pr.trade_type == TT_PROFIT) {
        profits->pr.money = item->data.pr.money;
        dispose(item, sizeof(trade_record_type), "tp__read_inv: profit type");
      } else if (first) {
        item->prev = NULL;
        *inv = item;
        *cur_top = item;
        first = false;
      } else {
        (*cur_top)->next = item;
        item->prev = *cur_top;
        *cur_top = item;
      }
    }
  }

  if (*cur_top != NULL) {
    (*cur_top)->next = NULL;
  }

  *cur_top = *inv;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void tp__write_inv(FILE *sales, pinven_ptr *inv, pinven_ptr *cur_top,
                   pinven_ptr *blegga, trade_record_type *profits,
                   long *cur_display_size, pinven_ptr cur_display[]) {
  boolean panic = false;
  char out_val[82];
  pinven_ptr dead = NULL, item = NULL;

  if (ftruncate((int)fileno(sales), 0) != 0) {
    msg_print("ftruncate failed");
  }
  fflush(sales);
  rewind(sales);

  fwrite(profits, sizeof(trade_record_type), 1, sales);
  item = *inv;
  *inv = NULL;
  *cur_top = NULL;
  *blegga = NULL;
  tp__clear_display(cur_display_size, cur_display);

  while (item != NULL) {
    if (!panic) {
      if (fwrite(&(item->data), sizeof(trade_record_type), 1, sales) != 1) {
        panic = true;
        sprintf(out_val, "Error writing post "
                         "inventory. Please report "
                         "this!");
        msg_print(out_val);
        msg_print("");
      }
    }

    dead = item;
    item = item->next;
    dead->next = NULL;

    if (item != NULL) {
      item->prev = NULL;
    }
    dispose(dead, sizeof(inven_record), "tp__write_inv: dead");
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void tp__display_inv(pinven_ptr start, pinven_ptr *inv, pinven_ptr *blegga,
                     long *cur_display_size, pinven_ptr cur_display[]) {
  long count = 0;
  long old_display_size = *cur_display_size;

  while ((start != NULL) && (count < T_DISPLAY_SIZE)) {
    if (start->data.fsr.trade_type == TT_FOR_SALE) {
      count++;

      if (cur_display[count] != start) {
        char out_val1[82];
        char out_val2[85];
        cur_display[count] = start;
        inven_temp->data = start->data.fsr.object;
        objdes(out_val1, inven_temp, true);
        sprintf(out_val2, "%c) %s", (char)(96 + count), out_val1);
        prt(out_val2, count + 5, 1);
        sprintf(out_val2, "%ld", start->data.fsr.best_bid);
        prt(out_val2, count + 5, 60);

        if (wizard2) {
          sprintf(out_val2, "%9ld", item_value(&(start->data.fsr.object)));
          prt(out_val2, count + 5, 71);
        } else if (start->data.fsr.seller.claim_check == player_claim_check) {
          prt("your sale!", count + 5, 71);
        } else if (start->data.fsr.best_bidder.claim_check ==
                   player_claim_check) {
          prt("your bid!", count + 5, 71);
        }
      }
    }
    start = start->next;
  }

  *cur_display_size = count;
  while (old_display_size > *cur_display_size) {
    erase_line(old_display_size + 5, 1);
    cur_display[old_display_size] = NULL;
    old_display_size--;
  }

  if (start == NULL) {
    *blegga = *inv;
  } else {
    *blegga = start;
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void tp__display_store(char shop_owner[82], pinven_ptr *inv, pinven_ptr *blegga,
                       pinven_ptr *cur_top, long *cur_display_size,
                       pinven_ptr cur_display[])

{
  clear_rc(1, 1);
  prt(shop_owner, 4, 10);
  prt("   Item", 5, 1);
  prt("Top bid", 5, 60);
  if (wizard2) {
    prt("Value", 5, 75);
  }
  tp__display_gold();
  tp__display_commands();
  tp__clear_display(cur_display_size, cur_display);
  tp__display_inv(*cur_top, inv, blegga, cur_display_size, cur_display);
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
boolean tp__find_money_order(trade_account_type *owner, pinven_ptr *item,
                             pinven_ptr *inv) {
  boolean looking = true;

  *item = *inv;

  while (looking) {
    if (*item == NULL) {
      looking = FALSE;
    } else if (((*item)->data.cr.trade_type == TT_CASH) &&
               ((*item)->data.cr.owner.claim_check == owner->claim_check)) {
      looking = false;
    } else {
      *item = (*item)->next;
    }
  }

  return (*item != NULL);
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void tp__send_money(trade_account_type *owner, long amount, pinven_ptr *inv) {
  pinven_ptr item = NULL;

  if (tp__find_money_order(owner, &item, inv)) {
    item->data.cr.amount += amount;
  } else {
    item = (pinven_ptr)safe_malloc(sizeof(pinven_record), "tp__send_money");
    item->prev = NULL;
    item->next = *inv;
    item->data.cr.trade_type = TT_CASH;
    item->data.cr.owner = *owner;
    item->data.cr.amount = amount;
    (*inv)->prev = item;
    *inv = item;
  }

  item->data.cr.time = time(0);
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void tp__made_profit(long amount, trade_record_type *profits) {
  /*{ Try to trap so there isn't long overflow }*/

  if ((999999999 - profits->pr.money) < amount) {
    profits->pr.money = 999999999;
  } else {
    profits->pr.money += amount;
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void tp__delete_item(pinven_ptr *item, pinven_ptr *inv, pinven_ptr *cur_top) {
  pinven_ptr next = NULL;

  next = (*item)->next;
  if ((*item)->prev != NULL) {
    (*item)->prev->next = next;
  } else if (*inv == (*item)) {
    *inv = next;
  } else {
    msg_print("Something truly bizarre happened in delete_item.");
    msg_print("Please report (via MAIL) to a wizard.  Thanks.");
    msg_print("");
  }
  if (next != NULL) {
    next->prev = (*item)->prev;
  }
  if (*cur_top == (*item)) {
    *cur_top = next;
  }
  (*item)->prev = NULL;
  (*item)->next = NULL;
  dispose(*item, sizeof(inven_record), "tp__delete_item");
  *item = next;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
boolean tp__enough_time(long sale_time, long waiting_days, long waiting_hours) {
  long cur_time;
  double delta_time;
  double wait_time;
  boolean return_value;

  cur_time = time(NULL);
  delta_time = cur_time - sale_time;

  wait_time = (waiting_days * 24 * 60 * 60) + (waiting_hours * 60 * 60);

  return_value = delta_time > wait_time;

  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
void tpd__post_outbids(pinven_ptr *item, pinven_ptr *inv, pinven_ptr *cur_top,
                       trade_record_type *profits, boolean *redisplay) {
  /*   The best bidder bid less than 90% of the value of the object, so
       the storekeeper will bid 5% more than the best_bidder, sell the
       object to a "store", and make a nice profit. */

  tp__send_money(&((*item)->data.fsr.best_bidder),
                 ROUND(T_REFUND_ON_BID * (*item)->data.fsr.best_bid), inv);
  (*item)->data.fsr.best_bid = ROUND((*item)->data.fsr.best_bid *
                                     T_BID_INCREMENT_FACTOR * T_REFUND_ON_SALE);
  tp__send_money(&((*item)->data.fsr.seller),
                 ROUND(T_REFUND_ON_SALE * (*item)->data.fsr.best_bid), inv);
  tp__made_profit(item_value(&((*item)->data.fsr.object)) -
                      (*item)->data.fsr.best_bid,
                  profits);
  *redisplay = true;
  tp__delete_item(item, inv, cur_top); /* XXXX safe to do? make item a **? */
}
/*//////////////////////////////////////////////////////////////////// */
void tpd__player_wins_bid(pinven_ptr *item, pinven_ptr *inv,
                          pinven_ptr *cur_top, trade_record_type *profits,
                          boolean *redisplay, boolean *weight_changed,
                          boolean *exit_flag) {
  char out_val1[82];
  char out_val2[83];
  treas_rec *temp_ptr = NULL;

  msg_print("Hmm, you're supposed to get something.");
  inven_temp->data = (*item)->data.fsr.object;
  if (inven_check_num() && inven_check_weight()) {
    temp_ptr = inven_carry();
    msg_print("You are now the proud owner of");
    objdes(out_val1, temp_ptr, true);
    sprintf(out_val2, "%s.", out_val1);
    msg_print(out_val2);
    tp__send_money(&((*item)->data.fsr.seller),
                   ROUND(T_REFUND_ON_SALE * (*item)->data.fsr.best_bid), inv);
    tp__made_profit(ROUND(T_PROFIT_FROM_SALE * (*item)->data.fsr.best_bid),
                    profits);
    tp__delete_item(item, inv, cur_top); /* XXXX safe to do? */
    *redisplay = true;
    *weight_changed = true;
  } else {
    msg_print("The shopkeeper had something to give you, but");
    msg_print("you couldn't carry it.  Come back when you can.");
    *exit_flag = true;
  }
}
/*//////////////////////////////////////////////////////////////////// */
void tp__deliver(pinven_ptr *inv, pinven_ptr *cur_top, boolean *exit_flag,
                 trade_record_type *profits, trade_account_type *cur_player) {
  boolean weight_changed, gold_changed, redisplay;
  pinven_ptr item = NULL; /*, next;*/
  char out_val2[82];
  /*  treas_rec * temp_ptr; */

  weight_changed = false;
  gold_changed = false;
  redisplay = false;
  item = *inv;

  while ((!*exit_flag) && (item != NULL)) {

    if (tp__enough_time(item->data.fsr.time, T_EXPIRE_TIME_DAYS,
                        T_EXPIRE_TIME_HOURS)) {

      if (item->data.fsr.trade_type == TT_FOR_SALE) {
        tp__send_money(
            &(item->data.fsr.seller),
            ROUND(item_value(&(item->data.fsr.object)) * T_REFUND_ON_SALE),
            inv);
        tp__made_profit(
            ROUND(item_value(&(item->data.fsr.object)) * T_PROFIT_FROM_SALE),
            profits);
        redisplay = true;
      }
      tp__delete_item(&item, inv, cur_top); /* XXXX safe to do? */
    } else if (item->data.fsr.trade_type == TT_FOR_SALE) {

      if ((item->data.fsr.best_bidder.claim_check == player_claim_check) &&
          (tp__enough_time(item->data.fsr.time, T_BID_WAIT_DAYS,
                           T_BID_WAIT_HOURS))) {

        if (item->data.fsr.best_bid <
            ROUND(T_TAKE_THE_MONEY_AND_RUN *
                  item_value(&(item->data.fsr.object)))) {
          tpd__post_outbids(&item, inv, cur_top, profits, &redisplay);
        } else {
          tpd__player_wins_bid(&item, inv, cur_top, profits, &redisplay,
                               &weight_changed, exit_flag);
        }

      } else {
        item = item->next;
      }
    } else {
      item = item->next;
    }
  } /* end while */

  while (tp__find_money_order(cur_player, &item, inv)) {
    add_money(item->data.cr.amount * GOLD_VALUE);
    sprintf(out_val2, "The shopkeeper gave you %ld gold pieces.",
            item->data.cr.amount);
    msg_print(out_val2);
    tp__delete_item(&item, inv, cur_top);
    gold_changed = true;
  }

  if (*exit_flag) {
    if (weight_changed || gold_changed)
      prt_stat_block();
  } else {
    if (redisplay) {
      *cur_top = *inv;
    }
    if (gold_changed || weight_changed) {
      msg_print(" ");
    }
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
boolean tp__get_store_item(long *command, char pmt[82], long i1, long i2) {
  /*{ Get the ID of a store item and return it's value      -RAK-   }*/

  char out_val[82];
  boolean flag;

  *command = 0;
  flag = true;

  sprintf(out_val, "(Items %c-%c, Esc to exit) %s", (char)(i1 + 96),
          (char)(i2 + 96), pmt);

  while (((*command < i1) || (*command > i2)) && (flag)) {
    prt(out_val, 1, 1);
    *command = inkey();
    switch (*command) {
    case 3:
    case 25:
    case 26:
    case 27:
      flag = false;
      break;
    default:
      (*command) -= 96;
      break;
    }
  }

  msg_flag = false;
  erase_line(msg_line, msg_line);

  return flag;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void tp__dump(char filename[82], pinven_ptr *inv) {
  FILE *dump;
  pinven_ptr item = NULL;
  char out_val[82];

  dump = (FILE *)fopen(filename, "w");
  if (dump == NULL) {
    sprintf(out_val, "Error opening %s", filename);
    msg_print(out_val);
  } else {
    if (strcmp(filename, "NL:")) {
      sprintf(out_val, "Dumping to %s", filename);
      msg_print(out_val);
    }
    item = *inv;
    while (item != NULL) {
      /* WITH item->data. DO BEGIN; */
      switch (item->data.cr.trade_type) {
      case TT_FOR_SALE:
        fprintf(dump, "for sale:\n");
        sprintf(out_val, "%ld", item->data.fsr.time);
        fprintf(dump, "  time:        %s\n", out_val);
        inven_temp->data = item->data.fsr.object;
        objdes(out_val, inven_temp, true);
        fprintf(dump, "  object:      %s\n", out_val);
        fprintf(dump, "  seller:      %s/%ld\n", item->data.fsr.seller.username,
                item->data.fsr.seller.claim_check);
        sprintf(out_val, "%ld", item->data.fsr.bid_time);
        fprintf(dump, "  bid time:    %s\n", out_val);
        fprintf(dump, "  best bid:    %ld\n", item->data.fsr.best_bid);
        fprintf(dump, "  best bidder: %s/%ld\n",
                item->data.fsr.best_bidder.username,
                item->data.fsr.best_bidder.claim_check);
        break;

      case TT_CASH:
        fprintf(dump, "cash:");
        sprintf(out_val, "%ld", item->data.cr.time);
        fprintf(dump, "  time:   %s\n", out_val);
        fprintf(dump, "  amount: %ld\n", item->data.cr.amount);
        fprintf(dump, "  owner:  %s/%ld\n", item->data.cr.owner.username,
                item->data.cr.owner.claim_check);
        break;
      }
      item = item->next;
    } /* end while */
    fclose(dump);
  }

  if (strcmp(filename, "NL:")) {
    msg_print("");
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void tp__bid(long *cur_display_size, trade_account_type *cur_player,
             pinven_ptr *inv, pinven_ptr *cur_top, pinven_ptr *blegga,
             pinven_ptr cur_display[], trade_record_type *profits,
             boolean *exit_flag) {
  long offer, to_bank, which;
  char out_val[82];
  pinven_ptr item = NULL;
  boolean flag;

  if (*cur_display_size > 0) {
    if (tp__get_store_item(&which, "Which one?", 1, *cur_display_size)) {
      msg_print("How much do you offer? ");
      if (!get_string(out_val, 1, 24, 40)) {
        erase_line(1, 1);
      } else {
        offer = 0;
        sscanf(out_val, "%ld", &offer);
        if (offer <=
            (cur_display[which]->data.fsr.best_bid * T_BID_INCREMENT_FACTOR)) {
          msg_print("You'll have to do better "
                    "than that!");
        } else {
          if (player_money[TOTAL_] >= offer) {
            subtract_money(offer * GOLD_VALUE, true);
            flag = true;
          } else {
            to_bank = offer - player_money[TOTAL_];
            flag = send_page(to_bank);
          }

          if (flag) {
            item = cur_display[which];
            if (item->data.fsr.best_bid > 0) {
              tp__send_money(&(item->data.fsr.best_bidder),
                             ROUND(T_REFUND_ON_BID * item->data.fsr.best_bid),
                             inv);
              /* tp__dump("NL:", inv);
               */
              tp__made_profit(
                  ROUND(T_PROFIT_FROM_BID * item->data.fsr.best_bid), profits);
            }

            item->data.fsr.best_bidder = *cur_player;
            item->data.fsr.best_bid = offer;
            item->data.fsr.bid_time = time(0);
            cur_display[which] = NULL;
            item = NULL;
            tp__deliver(inv, cur_top, exit_flag, profits, cur_player);
            tp__display_inv(*cur_top, inv, blegga, cur_display_size,
                            cur_display);
            tp__display_gold();
          }
        }
      }
    }
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void tp__sell(pinven_ptr *inv, pinven_ptr *cur_top, pinven_ptr *blegga,
              trade_account_type *cur_player, long *cur_display_size,
              pinven_ptr cur_display[], char shop_owner[82]) {
  treas_rec *item_ptr = NULL;
  boolean redraw;
  pinven_ptr item = NULL;
  char response[82];
  long wgt;
  treas_rec *temp_ptr = NULL;
  char trash_char;

  redraw = false;
  response[0] = 0;
  change_all_ok_stats(true, true);
  if (get_item(&item_ptr, "Which one? ", &redraw, inven_ctr, &trash_char, false,
               false)) {
    wgt = 0;
    temp_ptr = item_ptr->next;
    if ((item_ptr->data.flags2 & Holding_bit) != 0) {
      while ((temp_ptr != NULL) && (temp_ptr->is_in)) {
        wgt += temp_ptr->data.weight * temp_ptr->data.number;
        temp_ptr = temp_ptr->next;
      }
    }

    if ((strstr(item_ptr->data.name, "|") != NULL) ||
        (strstr(item_ptr->data.name, "^") != NULL)) {
      strcpy(response, "I can't sell that!  Identify it first!");
    } else if (wgt != 0) {
      strcpy(response, "Hey that bag is full of items! Empty it first.");
    } else if (item_ptr->is_in) {
      strcpy(response, "You can't sell an item *IN* a bag of holding.");
    } else if (item_value(&(item_ptr->data)) < T_ACCEPTABLE_ITEM_PRICE) {
      strcpy(response, "What is THAT?  I won't have that in my shop!");
    } else if ((item_ptr->data.flags2 & Blackmarket_bit) != 0) {
      strcpy(response, "Hmmm, I don't think I want that on the shelves.");
    } else {
      item = (pinven_ptr)safe_malloc(sizeof(pinven_record), "tp__sell");
      item->next = *inv;
      item->prev = NULL;
      item->data.fsr.trade_type = TT_FOR_SALE;
      item->data.fsr.seller = *cur_player;
      item->data.fsr.object = item_ptr->data;
      item->data.fsr.object.number = 1;
      item->data.fsr.best_bid = 0;
      item->data.fsr.time = time(NULL);

      item->data.fsr.best_bidder.uid = 0;
      item->data.fsr.best_bidder.username[0] = 0;
      item->data.fsr.best_bidder.claim_check = 0;
      item->data.fsr.best_bidder.master_id = 0;

      if ((*inv) != NULL) {
        (*inv)->prev = item;
      }
      *inv = item;
      *cur_top = *inv;
      inven_weight -= item_ptr->data.weight;
      item_ptr->data.number--;
      if (item_ptr->data.number <= 0) {
        delete_inven_item(item_ptr);
      }
      strcpy(response, "Remember to come pick up your cash "
                       "when it sells.");
    }
  }

  if (redraw) {
    tp__display_store(shop_owner, inv, blegga, cur_top, cur_display_size,
                      cur_display);
  } else {
    tp__display_inv(*cur_top, inv, blegga, cur_display_size, cur_display);
  }

  msg_print(response);
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void tp__nuke_item(pinven_ptr *inv, pinven_ptr *cur_top, pinven_ptr *blegga,
                   long *cur_display_size, pinven_ptr cur_display[],
                   trade_record_type *profits) {
  long which;
  char command;

  if (*cur_display_size > 0) {
    if (tp__get_store_item(&which, "Delete which one?", 1, *cur_display_size)) {
      if (get_com("Refund money? (Y/N)", &command)) {
        switch (command) {

        case 'y':
        case 'Y':
          /* with cur_display[which]->data.fsr.;
           */
          tp__send_money(
              &(cur_display[which]->data.fsr.best_bidder),
              ROUND(T_REFUND_ON_BID * cur_display[which]->data.fsr.best_bid),
              inv);
          tp__made_profit(
              ROUND(T_PROFIT_FROM_BID * cur_display[which]->data.fsr.best_bid),
              profits);
          tp__send_money(
              &(cur_display[which]->data.fsr.seller),
              ROUND(item_value(&(cur_display[which]->data.fsr.object)) *
                    T_REFUND_ON_SALE),
              inv);
          tp__made_profit(
              -ROUND(item_value(&(cur_display[which]->data.fsr.object)) *
                     T_REFUND_ON_SALE),
              profits);
          break;

        default:
          break;
        }
        tp__delete_item(&(cur_display[which]), inv, cur_top);
        cur_display[which] = NULL;
        tp__display_inv(*cur_top, inv, blegga, cur_display_size, cur_display);
      }
    }
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void tp__get_info(pinven_ptr *inv, pinven_ptr *blegga, pinven_ptr *cur_top,
                  long *cur_display_size, pinven_ptr cur_display[]) {
  char out_val[82];
  long which;

  if (*cur_display_size > 0) {
    if (tp__get_store_item(&which, "Info on which?", 1, *cur_display_size)) {
      /* with cur_display[which]->data.fsr. do; */
      erase_line(8 + 6, 1);
      erase_line(9 + 6, 1);
      erase_line(10 + 6, 1);
      erase_line(11 + 6, 1);
      erase_line(12 + 6, 1);
      sprintf(out_val, "%ld", cur_display[which]->data.fsr.time);
      prt2("Sale time : ", out_val, 9 + 6, 1);
      sprintf(out_val, "%ld",
              item_value(&(cur_display[which]->data.fsr.object)));
      prt2("Item value : ", out_val, 9 + 6, 60);
      sprintf(out_val, "%s/%ld", cur_display[which]->data.fsr.seller.username,
              cur_display[which]->data.fsr.seller.claim_check);
      prt2("Seller : ", out_val, 10 + 6, 1);
      sprintf(out_val, "%s/%ld",
              cur_display[which]->data.fsr.best_bidder.username,
              cur_display[which]->data.fsr.best_bidder.claim_check);
      prt2("Bidder : ", out_val, 11 + 6, 1);
      msg_print("Press <space> to continue");
      msg_print("");
      *cur_display_size = 12;
      cur_display[8] = NULL;
      cur_display[9] = NULL;
      cur_display[10] = NULL;
      cur_display[11] = NULL;
      cur_display[12] = NULL;
      tp__display_inv(*cur_top, inv, blegga, cur_display_size, cur_display);
    }
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void tp__parse_command(pinven_ptr *inv, pinven_ptr *cur_top, pinven_ptr *blegga,
                       long *cur_display_size, pinven_ptr cur_display[],
                       trade_record_type *profits, char shop_owner[82],
                       trade_account_type *cur_player, boolean *exit_flag) {
  char command;
  char out_val[82];
  treas_rec *trash_ptr = NULL;

  if (get_com("", &command)) {
    switch (command) {

    case 4: /* ^d */
      if (wizard2) {
        tp__nuke_item(inv, cur_top, blegga, cur_display_size, cur_display,
                      profits);
      }
      break;

    case 5: /* ^e */
      if (wizard2) {
        tp__dump("TRADE.DUMP", inv);
      }
      break;

    case 9: /* ^i */
      if (wizard2) {
        tp__get_info(inv, blegga, cur_top, cur_display_size, cur_display);
      }
      break;

    case 16: /* ^p */
      if (wizard2) {
        sprintf(out_val, "Profits made to date: %ld", profits->pr.money);
        msg_print(out_val);
        msg_print("");
        if (get_com("Reset? (Y/N)", &command)) {
          switch (command) {
          case 'y':
          case 'Y':
            profits->pr.money = 0;
            break;
          }
        }
        erase_line(1, 1);
      }
      break;

    case 18: /* ^r */
      tp__display_store(shop_owner, inv, blegga, cur_top, cur_display_size,
                        cur_display);
      break;

    case 32:
      if (*cur_top == *blegga) {
        prt("Entire inventory is displayed.", 1, 1);
      } else {
        *cur_top = *blegga;
        tp__display_inv(*cur_top, inv, blegga, cur_display_size, cur_display);
      }
      break;

    case 101:
      if (inven_command('e', &trash_ptr, "")) {
        tp__display_store(shop_owner, inv, blegga, cur_top, cur_display_size,
                          cur_display);
      }
      break;

    case 105:
      if (inven_command('i', &trash_ptr, "")) {
        tp__display_store(shop_owner, inv, blegga, cur_top, cur_display_size,
                          cur_display);
      }
      break;

    case 116:
      if (inven_command('t', &trash_ptr, "")) {
        tp__display_store(shop_owner, inv, blegga, cur_top, cur_display_size,
                          cur_display);
      }
      break;

    case 119:
      if (inven_command('w', &trash_ptr, "")) {
        tp__display_store(shop_owner, inv, blegga, cur_top, cur_display_size,
                          cur_display);
      }
      break;

    case 120:
      if (inven_command('x', &trash_ptr, "")) {
        tp__display_store(shop_owner, inv, blegga, cur_top, cur_display_size,
                          cur_display);
      }
      break;

    case 112:
      if (player_cheated) {
        msg_print("Cheaters are not allowed to buy "
                  "things here!");
      } else if (total_winner) {
        msg_print("Winners are not allowed to buy "
                  "things here!");
      } else {
        tp__bid(cur_display_size, cur_player, inv, cur_top, blegga, cur_display,
                profits, exit_flag);
      }
      break;

    case 115:
      if (player_cheated) {
        msg_print("Cheaters are not allowed to sell "
                  "things here!");
      } else if (total_winner) {
        msg_print("Winners are not allowed to sell "
                  "things here!");
      } else {
        tp__sell(inv, cur_top, blegga, cur_player, cur_display_size,
                 cur_display, shop_owner);
      }
      break;

    default:
      prt("Invalid Command.", 1, 1);
      break;
    }
  } else {
    *exit_flag = true;
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void tp__set_shop_owner(char shop_owner[82]) {
  switch (randint(4)) {
  case 1:
    strcpy(shop_owner,
           "Ollie North          (arms)                Trading Post");
    break;
  case 2:
    strcpy(shop_owner,
           "Uncle Sam            (wasp)                Trading Post");
    break;
  case 3:
    strcpy(shop_owner,
           "Jimmy Hoffa          (missing link)        Trading Post");
    break;
  default:
    strcpy(shop_owner,
           "Gary Hart            (presidential)        Trading Post");
    break;
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void enter_trading_post() {
  FILE *sales;
  trade_record_type profits;
  trade_account_type cur_player;
  pinven_ptr inv = NULL;
  pinven_ptr cur_top = NULL;
  pinven_ptr blegga = NULL;
  pinven_ptr cur_display[T_DISPLAY_SIZE + 1];
  long tics = 1;
  long cur_display_size;
  char shop_owner[82];
  boolean exit_flag, entered = false;

  tp__set_shop_owner(shop_owner);

  tp__open_trade_file(&sales, &exit_flag);

  if (!exit_flag) {
    tp__set_player(&cur_player);

    tp__read_inv(sales, &inv, &cur_top, &profits);
    tp__deliver(&inv, &cur_top, &exit_flag, &profits, &cur_player);
    if (!exit_flag) {
      entered = true;
      tp__display_store(shop_owner, &inv, &blegga, &cur_top, &cur_display_size,
                        cur_display);
      for (; !exit_flag;) {
        tp__parse_command(&inv, &cur_top, &blegga, &cur_display_size,
                          cur_display, &profits, shop_owner, &cur_player,
                          &exit_flag);
        adv_time(false);
        tics++;

        if (tics % 2 == 1) {
          kick__kickout_player_if_time();
        }
      }
    }
    tp__write_inv(sales, &inv, &cur_top, &blegga, &profits, &cur_display_size,
                  cur_display);
    msg_print("The storekeeper says `Come again. . .'");
    refresh();
    trade_file_close(&sales);
    if (entered) {
      draw_cave();
    }
  }
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/* END FILE  trade.c */
