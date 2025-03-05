#include "pregame/main.h"
#include "debug.h"
#include "init/argv.h"
#include "init/bank.h"
#include "init/death.h"
#include "init/debug.h"
#include "init/graphics.h"
#include "init/kickout.h"
#include "init/monsters.h"
#include "init/stores.h"
#include "init/trade.h"
#include "io.h"
#include "kickout.h"
#include "main_loop/main_loop.h"
#include "master.h"
#include "player.h"
#include "variables.h"

int main(const int argc, char *argv[]) {

  // Initialization
  game_state = GS_LOADING;
  if (!mst__init_masters())
    exit_game();
  if (!init__debug())
    exit_game();
  MSG(("%s", "Main - Initialization"));

  // Check to see if an update is in progress
  if (!init__kickout())
    exit_game();
  if (kick__should_kickout()) {
    printf("The gates to Moria are closed.\n"
           "Either you are outside operating hours, or there is a maintenance "
           "going on.\n\n");
    kick__dump_operating_hours_to_file(stdout);
    exit_game();
  }

  if (!init__monsters())
    exit_game();
  if (!init__stores())
    exit_game();
  if (!init__bank())
    exit_game();
  if (!init__death())
    exit_game();
  if (!init__trade())
    exit_game();
  if (!init__argv(argc, argv))
    exit_game();
  if (!init__graphics())
    exit_game();

  MSG(("%s", "Main - Entering pregame"));
  if (!pregame__main())
    exit_game();

  MSG(("%s", "Main - Entering game"));
  return main_loop();
}
