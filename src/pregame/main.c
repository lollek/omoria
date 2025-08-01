#include "main.h"
#include "../configure.h"
#include "../generate_map/generate_map.h"
#include "../io.h"
#include "../master.h"
#include "../menu.h"
#include "../misc.h"
#include "../player.h"
#include "../save.h"
#include "../screen.h"
#include "../term.h"
#include "../variables.h"
#include "menu.h"
#include <string.h>

static bool load_existing_character(void) {
    game_state = GS_IGNORE_CTRL_C;
    if (!sav__load_character(player_name, player_uid)) {
      msg_print("Failed to load character. The file is likely corrupt.");
      msg_print(" ");
      return false;
    }
    if (!sav__save_character()) return false;
    change_name();

    return true;
}

static bool create_new_character(void) {
  char save_file_name[82];

  create_character();
  player_flags.light_on = false;
  strcpy(save_file_name, SAVE_FILE_PATH "/");
  strcat(save_file_name, player_name);
  player_uid = mst__add_character();
  if (player_uid == 0) {
    msg_print("ERROR opening file MASTER. Contact your local wizard.");
    msg_print(" ");
    return false;
  }

  learn_magic(false);
  player_cmana = player_mana;
  randes_seed = town_seed; // Unsure why the seed needs to be the same

  return true;
}

bool pregame__main(void) {
  /* Currently, pregame__menu will set player_name if a we loaded a player, else
   * player_name will be unset, so that's what we go on. */
  if (!pregame__menu()) return false;


  bool generate = false;
  if (player_name[0] != '\0') {
    if (!load_existing_character()) return false;
  } else {
    if (!create_new_character()) return false;
    generate = true;
  }

  if (player_pclass == C_MONK) {
    strcpy(bare_hands, "2d2");
  }

  player_max_exp = (long)(exp_per_level[MAX_PLAYER_LEVEL - 1] * player_expfact);
  clear_from(1);
  prt_stat_block();

  if (generate) {
    generate_map();
  }

  return true;
}
