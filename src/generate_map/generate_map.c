#include "../debug.h"
#include "../generate_monster/generate_monster.h"
#include "../misc.h"
#include "../pascal.h"
#include "../player.h"
#include "../random.h"
#include "../screen.h"
#include "../stores.h"
#include "../types.h"
#include "../variables.h"
#include "config.h"
#include "dungeon.h"
#include "misc.h"
#include "river.h"
#include "rooms.h"
#include "town.h"

void generate_map(void) {
  panel_row_min = 0;
  panel_row_max = 0;
  panel_col_min = 0;
  panel_col_max = 0;
  char_row = -1;
  char_col = -1;

  tlink();
  mlink();

  blank_out_map();

  /*  for (i1 = 0; i1 < MAX_MONS_LEVEL+1; i1++) { */
  /*    printf ("\n m_level[%d] : %d",i1,m_level[i1]);  fflush(stdout); */
  /*  } */

  if (dun_level == 0) {
    cur_height = SCREEN_HEIGHT * 2;
    cur_width = SCREEN_WIDTH * 2;
    max_panel_rows = cur_height / SCREEN_HEIGHT * 2 - 2;
    max_panel_cols = cur_width / SCREEN_WIDTH * 2 - 2;
    panel_row = 0;
    panel_col = 0;
    generate_town();
  } else {
    cur_height = MAX_HEIGHT;
    cur_width = MAX_WIDTH;
    max_panel_rows = cur_height / SCREEN_HEIGHT * 2 - 2;
    max_panel_cols = cur_width / SCREEN_WIDTH * 2 - 2;
    panel_row = max_panel_rows;
    panel_col = max_panel_cols;
    generate_dungeon();
  }
}
