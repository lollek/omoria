#include <string.h>

#include "init/monsters.h"
#include "init/stores.h"
#include "init/treasures.h"

#include "configure.h"
#include "debug.h"
#include "kickout.h"
#include "player.h"
#include "routines.h"
#include "save.h"
#include "stores.h"
#include "variables.h"

void C_main_menu();
int main(int argc, char *argv[]) {

  // Initialization
  game_state = GS_LOADING;
  dbg__init();
  MSG(("%s", "Main - Initialization"));

  // Check to see if an update is in progress
  if (kick__should_kickout()) {
    printf("The gates to moria are locked ...\n"
        "Perhaps a new version is being installed.\n"
        "Wait a few minutes and try again.\n");
    exit_game();
  }

  if (!init__monsters()) exit_game();
  if (!init__treasures()) exit_game();
  if (!init__stores()) exit_game();

  bank_init();

  /*
   * Check operating hours
   * If not wizard then No_Control_Y
   * Check or create hours.dat, print message
   */
  intro(argc, argv);

  C_init_curses();
  curses_is_running = true;

  C_main_menu();

  /* Generate a character, or retrieve old one... */
  boolean generate = false;
  if (player_name[0] != '\0') {
    /* Retrieve character */
    game_state = GS_IGNORE_CTRL_C;
    generate = get_char();
    save_char();
    change_name();
    magic_init(randes_seed);

  } else {
    char save_file_name[82];
    /* Create character */
    create_character();
    player_flags.light_on = false;
    strcpy(save_file_name, SAVE_FILE_PATH "/");
    strcat(save_file_name, player_name);
    player_uid = C_master_add_character();

    learn_magic(false);
    player_cmana = player_mana;
    randes_seed = town_seed; // Unsure why the seed needs to be the same
    magic_init(randes_seed);
    generate = true;

  } /* end if (load/create character) */

  if (player_pclass == C_MONK) {
    strcpy(bare_hands, "2d2");
  }

  /* Begin the game */
  replace_name();
  set_gem_values();

  player_max_exp = (long)(exp_per_level[MAX_PLAYER_LEVEL - 1] * player_expfact);
  clear_from(1);
  prt_stat_block();

  /* Turn on message trapping, if requested */
  /*    if (want_trap) set_the_trap(); */

  /* Loop till dead, or exit */
  MSG(("Entering main loop"));
  while (true) {
    if (generate) { /* New level */
      generate_cave();
    }

    /* Dungeon logic */
    dungeon();
    generate = true;

    /* Character gets buried, or respawns */
    if (death) {
      upon_death();
      if (death) {
#if DO_DEBUG
        memory_error(0, "DEBUG_ON_EXIT");
#endif
        return 0;
      }
    }
  }
}
