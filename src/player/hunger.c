#include <math.h>
#include <string.h>

#include "../io.h"
#include "../misc.h"
#include "../player.h"
#include "../player_action.h"
#include "../random.h"
#include "../screen.h"
#include "../variables.h"

#include "hunger.h"

#define PLAYER_FOOD_MAX 15000  // Maximum food value, beyond is wasted
#define PLAYER_FOOD_FULL 10000 // Getting full
#define PLAYER_FOOD_ALERT 2000 // Warn player that he is getting low
#define PLAYER_FOOD_WEAK 1000  // Warn player that he is getting very low}
#define PLAYER_FOOD_FAINT 300  // Character begins fainting

void player_hunger_recalculate(void) {

  // Hunger item causes perpetual hunger
  if (player_flags.hunger_item &&
      player_flags.foodc > PLAYER_FOOD_ALERT + 15) {
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
    player_flags.status |= IS_WEAK | IS_HUNGERY;
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
