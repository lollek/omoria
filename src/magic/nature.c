#include <curses.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "../configure.h"
#include "../constants.h"
#include "../debug.h"
#include "../main_loop.h"
#include "../magic.h"
#include "../pascal.h"
#include "../player.h"
#include "../term.h"
#include "../types.h"
#include "../variables.h"
#include "../spells.h"
#include "../misc.h"
#include "../random.h"

void nature_spell_effects(long effect) {
  long i;
  long dir;
  long dumy;
  long y_dumy = char_row;
  long x_dumy = char_col;

  switch (effect + 1) {

  case 1: /*{ Moon Beam }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      msg_print("A line of light appears!");
      light_line(dir, char_row, char_col, 1);
    }
    break;

  case 2: /*{ Detect Monster }*/
    detect_creatures(SE_MONSTER);
    break;

  case 3: /*{ Battle Song }*/
    bless(randint(12) + 12);
    break;

  case 4: /*{ Light }*/
    light_area(char_row, char_col);
    break;

  case 5: /*{ Minor Cure }*/
    hp_player(damroll("5d3"), "a magic spell.");
    break;

  case 6: /*{ Find Safe Path }*/
    detect_sdoor();
    detect_trap();
    break;

  case 7: /*{ Magical Jig }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      zap_monster(dir, char_row, char_col, 0, SE_CONFUSE);
    }
    break;

  case 8: /*{ Warp Wood }*/
    msg_print("The wood around you bends and warps...");
    td_destroy();
    break;

  case 9: /*{ Battle Dance }*/
    player_flags.hero += (randint(10) + 5);
    bless(randint(20) + 20);
    break;

  case 10: /*{ Cure Poison }*/
    cure_me(&player_flags.poisoned);
    break;

  case 11: /*{ Charm }*/
    sleep_monsters1(char_row, char_col);
    break;

  case 12: /*{ Detect Curse }*/
    detect_curse();
    break;

  case 13: /*{ Summon Insects }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      fire_bolt(0, dir, char_row, char_col, damroll("1d6") + player_lev / 3 * 2,
                "Insect Swarm");
    }
    break;

  case 14: /*{ Call Lightning }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      fire_bolt(1, dir, char_row, char_col, damroll("2d8") + player_lev / 2,
                "Lightning Bolt");
    }
    break;

  case 15:                                      /*{ Magic Res. }*/
    player_flags.magic_prot = randint(15) + 10; /* not cumulitive */
    break;

  case 16: /*{ Starlight }*/
    msg_print("A twinkling light appears.");
    starlite(char_row, char_col);
    break;

  case 17: /*{ Create Food }*/
    create_food(10, 8, 0, 0, 0);
    break;

  case 18: /*{ Remove Curse }*/
    for (i = Equipment_min; i <= EQUIP_MAX - 1; i++) {
      equipment[i].flags &= 0x7FFFFFFF;
    }
    break;

  case 19: /*{ Infravision }*/
    player_flags.tim_infra += randint(30) + 30;
    break;

  case 20: /*{ Major Cure }*/
    hp_player(damroll("10d4"), "a magic spell. ");
    break;

  case 21: /*{ Resist Petrification }*/
    player_flags.resist_petri += randint(15) + 10;
    break;

  case 22: /*{ Transplant }*/
    msg_print("You step into a nearby patch of fungus...");
    teleport(player_lev * 6);
    break;

  case 23: /*{ Sunray }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      fire_ball(10, dir, char_row, char_col, damroll("2d8"),
                "flash of sunlight");
    }
    break;

  case 24: /*{ Dispel Magic }*/
    /* with player_flags do; */
    cure_me(&(player_flags).blind);
    cure_me(&(player_flags).poisoned);
    cure_me(&(player_flags).afraid);
    break;

  case 25: /*{ Fire Stream }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      fire_line(5, dir, char_row, char_col, damroll("3d4") + player_lev / 2,
                "Stream of Fire");
    }
    break;

  case 26: /*{ Protection from Nature }*/
    /* with player_flags do; */
    (player_flags).resist_heat += randint(15) + 10;
    (player_flags).resist_cold += randint(15) + 10;
    (player_flags).resist_lght += randint(15) + 10;
    break;

  case 27: /*{ Turn Stone to Mud }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      wall_to_mud(dir, char_row, char_col);
    }
    break;

  case 28: /*{ Goodberry }*/
    create_food(11, 11, 8, 10, 10);
    break;

  case 29: /*{ Creeping Doom }   */
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      creeping_doom(dir, char_row, char_col, player_lev * 3, player_lev / 4,
                    "Creeping Doom");
    }
    break;

  case 30: /*{ Pillar of Fire }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      fire_bolt(5, dir, char_row, char_col, damroll("5d8") + player_lev / 3,
                "Pillar of Fire");
    }
    break;

  case 31: /*{ Word of Recall }*/
    player_flags.word_recall = randint(20) + 20;
    break;

  case 32: /*{ Lightning Ball }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      fire_ball(1, dir, char_row, char_col, player_lev, "Lightning Ball");
    }
    break;

  case 33: /*{ Word of Blindness }*/
    if (zap_area(0, 0, SE_CONFUSE)) {
      msg_print("You blind the creatures!");
    }
    break;

  case 34: /*{ Protection from Monsters }*/
    player_flags.protmon = (randint(20) + player_lev);
    break;

  case 35: /*{ Control Temperature }*/
    do {
      if (d__get_dir("Hotter(1) || Colder(2)?", &dir, &dumy, &y_dumy,
                     &x_dumy)) {
        if (dir == 1) {
          zap_area(0x0020, 30, SE_HP);
        } else if (dir == 2) {
          zap_area(0x0010, 30, SE_HP);
        }
      } else {
        dir = 1;
      }
    } while (!((dir == 1) || (dir == 2)));
    break;

  case 36: /*{ Ring of Fire }*/
    player_flags.ring_fire += randint(4) + 1;
    break;

  case 37: /*{ Resist Charm }*/
    /* with player_flags do; */
    (player_flags).free_time += randint(10) + player_lev;
    (player_flags).magic_prot += randint(10) + player_lev;
    break;

  case 38: /*{ Battle Frenzy }*/
    bless(randint(30) + 30);
    player_flags.shero = (randint(20) + 20); /* not cumulitive */
    break;

  case 39: /*{ Dispel Monster }*/
    zap_area(0x0002, 3 * player_lev, SE_HP);
    break;

  case 40: /*{ Note of Destruction }*/
    destroy_area(char_row, char_col);
    break;

  default:
    break;
  }
  /*{ End of songs...			       }*/
}
