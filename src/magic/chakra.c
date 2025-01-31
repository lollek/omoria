#include "../io.h"
#include "../magic.h"
#include "../misc.h"
#include "../player.h"
#include "../player/hunger.h"
#include "../random.h"
#include "../screen.h"
#include "../spells.h"
#include "../types.h"

void chakra_spell_effects(const long effect) {
  /*{ Disciplines....}*/
  switch (effect + 1) {

  case 1: /*{ Self-Healing }*/
    hp_player(damroll("4d4"), "a magic spell.");
    break;

  case 2: /*{ Courage } */
    cure_player_status_effect(&player_flags.afraid);
    break;

  case 3: /*{ Slow Poison } */
    slow_poison();
    break;

  case 4: /*{ Negate Hunger } */
    player_hunger_set_status(FULL);
    prt_hunger();
    msg_print("You are full.");
    break;

  case 5: /*{ Sense Enemies }*/
    detect_creatures(SE_CREATURE);
    break;

  case 6: /*{ Self-Healing II }                 */
    hp_player(damroll("8d4"), "a prayer.");
    break;

  case 7: /*{ Night Vision }*/
    player_flags.tim_infra += randint(25) + 25;
    break;

  case 8: /*{ Poison Immunity }*/
    cure_player_status_effect(&player_flags.poisoned);
    break;

  case 9: /*{ See Invisible } */
    detect_inv2(randint(24) + 24);
    break;

  case 10: /*{ Advanced Self-Healing } */
    hp_player(damroll("16d4"), "a prayer.");
    break;

  case 11: /*{ Resist Petrification }*/
    player_flags.resist_petri += randint(15) + 10;
    break;

  case 12: /*{ Stealth }*/
    player_flags.temp_stealth += randint(15) + 10;
    break;

  case 13: /*{ Free Action } */
    player_flags.free_time += randint(10) + player_lev;
    break;

  case 14: /*{ Improved Speed }*/
    player_flags.fast += randint(20) + player_lev;
    break;

  default:
    break;
  }
  /*{ End of Disciplines...}*/
}
