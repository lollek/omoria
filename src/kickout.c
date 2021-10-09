#include <stdbool.h>
#include <stdio.h>

#include "configure.h"
#include "save.h"
#include "routines.h"
#include "variables.h"

#include "kickout.h"

void kick__kickout_player_if_time() {
  if (!kick__should_kickout()) return;

  find_flag = false;

  msg_print("A new version of IMORIA is being installed.");
  msg_print("After your character is saved, wait a few "
      "minutes,");
  msg_print("And then try to run the game.");
  msg_print("");

  do {
    save_and_quit();
  } while (true);
}

bool kick__should_kickout(void) {

  FILE *kick = fopen(MORIA_KICKOUT_FILE, "r");

  if (kick != NULL) {
    fclose(kick);
    return true;
  }

  return false;
}

