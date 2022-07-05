#include <math.h>
#include <string.h>
#include <time.h>

#include "../main_loop.h"
#include "../misc.h"
#include "../player.h"
#include "../player_action/move.h"
#include "../random.h"
#include "../screen.h"
#include "../types.h"
#include "../variables.h"

#include "hunger.h"

#define PLAYER_FOOD_MAX 15000  // Maximum food value, beyond is wasted
#define PLAYER_FOOD_FULL 10000 // Getting full
#define PLAYER_FOOD_ALERT 2000 // Warn player that he is getting low
#define PLAYER_FOOD_WEAK 1000  // Warn player that he is getting very low}
#define PLAYER_FOOD_FAINT 300  // Character begins fainting

enum hunger_status_t player_hunger_status(void) {
  if (player_flags.foodc > PLAYER_FOOD_FULL) {
    return BLOATED;
  } else if (player_flags.foodc > PLAYER_FOOD_ALERT) {
    return FULL;
  } else if (player_flags.foodc > PLAYER_FOOD_WEAK) {
    return HUNGRY;
  } else if (player_flags.foodc > PLAYER_FOOD_FAINT) {
    return WEAK;
  } else {
    return DYING;
  }
}

void player_hunger_recalculate(void) {

  // Hunger item causes perpetual hunger
  if (player_flags.hunger_item &&
      player_flags.foodc > (PLAYER_FOOD_ALERT + 15)) {
    player_flags.foodc = PLAYER_FOOD_ALERT + 15;
  }

  // Player is full
  if (player_flags.foodc >= PLAYER_FOOD_ALERT) {
    return;
  }

  // Player is hungry but not weak
  if (player_flags.foodc >= PLAYER_FOOD_WEAK) {
    if ((player_flags.status & IS_HUNGERY) == 0) {
      player_flags.status |= IS_HUNGERY;
      msg_print("You are getting hungry.");
      msg_flag = 0;
      rest_off();
      if (find_flag) {
        player_action_move(5);
      }
      prt_hunger();
    }
    return;
  }

  // Player is getting weak from hunger
  if ((player_flags.status & IS_WEAK) == 0) {
    player_flags.status |= (IS_WEAK | IS_HUNGERY);
    msg_print("You are getting weak from hunger.");
    msg_flag = 0;
    rest_off();
    if (find_flag) {
      player_action_move(5);
    }
    prt_hunger();
    player_wt -= trunc(player_wt * 0.015);
    msg_print("Your clothes seem to be getting loose.");
    if (player_wt < min_allowable_weight()) {
      msg_print("Oh no...  Now you've done it.");
      death = true;
      moria_flag = true;
      total_winner = false;
      strcpy(died_from, "starvation");
    }
  }

  if (player_flags.foodc < 0) {
    if (randint(5) == 1) {
      player_flags.paralysis += randint(3);
      msg_print("You faint from the lack of food.");
      msg_flag = 0;
      rest_off();
      if (find_flag) {
        player_action_move(5);
      }
    } else if (player_flags.foodc < PLAYER_FOOD_FAINT) {
      if (randint(8) == 1) {
        player_flags.paralysis += randint(5);
        msg_print("You faint from the lack of food.");
        msg_flag = 0;
        rest_off();
        if (find_flag) {
          player_action_move(5);
        }
      }
    }
  }
}

void player_hunger_set_status(enum hunger_status_t status) {
  switch (status) {
  case DYING:
    player_flags.foodc = PLAYER_FOOD_FAINT;
    player_flags.status |= (IS_HUNGERY | IS_WEAK);
    break;
  case WEAK:
    player_flags.foodc = PLAYER_FOOD_WEAK;
    player_flags.status |= (IS_HUNGERY | IS_WEAK);
    break;
  case HUNGRY:
    player_flags.foodc = PLAYER_FOOD_ALERT;
    player_flags.status |= IS_HUNGERY;
    player_flags.status &= ~IS_WEAK;
    break;
  case FULL:
    player_flags.foodc = PLAYER_FOOD_FULL;
    player_flags.status &= ~(IS_WEAK | IS_HUNGERY);
    break;
  case BLOATED:
    player_flags.foodc = PLAYER_FOOD_MAX;
    player_flags.status &= ~(IS_WEAK | IS_HUNGERY);
    break;
  }
}

void player_hunger_eat(long amount) {
  player_flags.status &= ~(IS_HUNGERY | IS_WEAK);

  if (player_flags.foodc < 0) {
    player_flags.foodc = 0;
  }

  player_flags.foodc += amount;

  enum hunger_status_t hunger_status = player_hunger_status();
  if (hunger_status >= BLOATED) {
    msg_print("You're getting fat from eating so much.");
    (player_flags).foodc = PLAYER_FOOD_MAX;
    player_wt += trunc(player_wt * 0.1);
    if (player_wt > max_allowable_weight()) {
      msg_print("Oh no...  Now you've done it.");
      death = true;
      moria_flag = true;
      total_winner = false;
      strcpy(died_from, "gluttony");
      return;
    }

    switch (randint(3)) {
    case 1:
      msg_print("Buuurrrppppp !");
      break;
    case 2:
      msg_print("Remember, obesity kills.");
      break;
    case 3:
      msg_print("Your armor doesn't seem to fit too "
                "well anymore.");
      break;
    }
    prt_hunger();
    return;
  }

  if (hunger_status >= FULL) {
    msg_print("You are full.");
  }

  prt_hunger();
}