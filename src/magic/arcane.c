#include "../player.h"
#include "../variables.h"
#include "../spells.h"
#include "../misc.h"
#include "../random.h"

void arcane_spell_effects(long effect) {
  /*{ Spells...                                     }*/

  long i2;
  long dir;
  long dumy;
  long y_dumy = char_row;
  long x_dumy = char_col;

  switch (effect + 1) {

  case 1: /*{ Magic Missile }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      fire_bolt(0, dir, char_row, char_col, damroll("2d6") + 1,
                "Magic Missile");
    }
    break;

  case 2: /*{ Detect Monster }*/
    detect_creatures(SE_MONSTER);
    break;

  case 3: /*{ Phase Door }*/
    teleport(10);
    break;

  case 4: /*{ Light }*/
    light_area(char_row, char_col);
    break;

  case 5: /*{ Cure Light }*/
    hp_player(damroll("4d4"), "a magic spell.");
    break;

  case 6: /*{ Find Hidden Traps/Door }*/
    detect_sdoor();
    detect_trap();
    msg_print(" ");
    break;

  case 7: /*{ Stinking Cloud }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      fire_ball(SE_GAS, dir, char_row, char_col, 9, "Stinking Cloud");
    }
    break;

  case 8: /*{ Confuse Monster }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      zap_monster(dir, char_row, char_col, 0, SE_CONFUSE);
    }
    break;

  case 9: /*{ Lightning Bolt }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      fire_bolt(SE_LIGHTNING, dir, char_row, char_col, damroll("3d8") + 1,
                "Lightning Bolt");
    }
    break;

  case 10: /*{ Trap/Door Destruction }*/
    td_destroy();
    break;

  case 11: /*{ Sleep I }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      zap_monster(dir, char_row, char_col, 0, SE_SLEEP);
    }
    break;

  case 12: /*{ Cure Poison }*/
    cure_me(&(player_flags.poisoned));
    break;

  case 13: /*{ Shadow Door }*/
    teleport(player_lev * 5);
    break;

  case 14: /*{ Remove Curse }*/
    for (i2 = Equipment_min; i2 <= EQUIP_MAX - 1; i2++) {
      /* with equipment[i2] do; */
      equipment[i2].flags &= 0x7FFFFFFF;
    }
    break;

  case 15: /*{ Frost Bolt }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      fire_bolt(SE_COLD, dir, char_row, char_col, damroll("4d8") + 1,
                "Frost Bolt");
    }
    break;

  case 16: /*{ Create Food }*/
    create_food(7, 7, 0, 0, 0);
    break;

  case 17: /*{ Infravision }*/
    player_flags.tim_infra += 50 + randint(50);
    break;

  case 18: /*{ Invisibility }*/
    player_flags.temp_stealth += randint(15) + 10;
    break;

  case 19: /*{ Turn Stone to Mud }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      wall_to_mud(dir, char_row, char_col);
    }
    break;

  case 20: /*{ Recharge Item I }*/
    recharge(20);
    break;

  case 21: /*{ Sleep II }*/
    sleep_monsters1(char_row, char_col);
    break;

  case 22: /*{ Phantasmal Force }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      fire_bolt(SE_ILLUSION, dir, char_row, char_col, damroll("8d4"),
                "phantasmal force");
    }
    break;

  case 23: /*{ Polymorph Other }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      poly_monster(dir, char_row, char_col);
    }
    break;

  case 24: /*{ Identify }*/
    ident_spell();
    break;

  case 25: /*{ Ring of Frost }*/
    player_flags.ring_ice += 3 + randint(3);
    break;

  case 26: /*{ Sleep III }*/
    zap_area(0, 0, SE_SLEEP);
    break;

  case 27: /*{ Hold Monster }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      zap_monster(dir, char_row, char_col, 4 + randint(4), SE_HOLD);
    }
    break;

  case 28: /*{ Fire Bolt }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      fire_bolt(SE_FIRE, dir, char_row, char_col, damroll("6d8") + 1,
                "Fire Bolt");
    }
    break;

  case 29: /*{ Slow Creature }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      zap_monster(dir, char_row, char_col, -1, SE_SPEED);
    }
    break;

  case 30: /*{ Protection From Magic }*/
    player_flags.magic_prot += 20 + randint(20);
    break;

  case 31: /*{ Frost Ball }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      fire_ball(SE_COLD, dir, char_row, char_col, 40, "Frost Ball");
    }
    break;

  case 32: /*{ Death Spell }*/
    zap_area(0, player_lev / 2 + damroll("4d8"), SE_DRAIN);
    break;

  case 33: /*{ Ring of Fire }*/
    player_flags.ring_fire += 3 + randint(3);
    break;

  case 34: /*{ Recharge Item II }*/
    recharge(50);
    break;

  case 35: /*{ Teleport Other }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      teleport_monster(dir, char_row, char_col);
    }
    break;

  case 36: /*{ Haste Self }*/
    (player_flags).fast += randint(20) + player_lev;
    break;

  case 37: /*{ Fire Ball }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      fire_ball(SE_FIRE, dir, char_row, char_col, 59, "Fire Ball");
    }
    break;

  case 38: /*{ Power Word: Destruction }*/
    destroy_area(char_row, char_col);
    break;

  case 39: /*{ Power Word: Kill }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      fire_bolt(0, dir, char_row, char_col, damroll("24d8"), "Black Bolt");
    }
    break;

  case 40: /*{ Genocide }*/
    genocide();
    break;

  default:
    break;
  }
}
