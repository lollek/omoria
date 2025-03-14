#include "../constants.h"
#include "../io.h"
#include "../magic.h"
#include "../misc.h"
#include "../player.h"
#include "../player/hunger.h"
#include "../random.h"
#include "../screen.h"
#include "../spells.h"
#include "../types.h"
#include "../variables.h"

void divine_spell_effects(const long effect) {

  /*{ Prayers...                                    }*/
  long i2, dir;
  long dumy, y_dumy, x_dumy;

  y_dumy = char_row;
  x_dumy = char_col;

  switch (effect + 1) {

  case 1: /*{ Detect Evil }*/
    detect_creatures(SE_EVIL);
    break;

  case 2: /*{ Cure Light Wounds }*/
    hp_player(damroll("3d3"), "a prayer.");
    break;

  case 3: /*{ Bless }*/
    bless(randint(12) + 12);
    break;

  case 4: /*{ Remove Fear }*/
    cure_player_status_effect(&player_flags.afraid);
    break;

  case 5: /*{ Call Light }*/
    light_area(char_row, char_col);
    break;

  case 6: /*{ Find Traps }*/
    detect_trap();
    break;

  case 7: /*{ Detect Doors/Stairs }*/
    detect_sdoor();
    break;

  case 8: /*{ Slow Poison }*/
    slow_poison();
    break;

  case 9: /*{ Blind Creature }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      zap_monster(dir, char_row, char_col, 0, SE_CONFUSE);
    }
    break;

  case 10: /*{ Portal }*/
    teleport(player_lev * 3);
    break;

  case 11: /*{ Cure Medium Wounds }*/
    hp_player(damroll("4d4"), "a prayer.");
    break;

  case 12: /*{ Ray of Sanctification }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      fire_bolt(SE_GOOD, dir, char_row, char_col, damroll("2d6"), "Purple Ray");
    }
    break;

  case 13: /*{ Heroism }*/
    player_flags.hero += randint(24) + 48;
    break;

  case 14: /*{ Sanctuary }*/
    sleep_monsters1(char_row, char_col);
    break;

  case 15: /*{ Remove Curse }*/
    for (i2 = Equipment_min; i2 <= EQUIP_MAX - 1; i2++) {
      /* with equipment[i2]. do; */
      equipment[i2].flags &= 0x7FFFFFFF;
    }
    break;

  case 16: /*{ Resist Heat and Cold }*/
    /* with player_flags do; */
    player_flags.resist_heat += randint(10) + 10;
    player_flags.resist_cold += randint(10) + 10;
    break;

  case 17: /*{ Silence }*/
    player_flags.temp_stealth += randint(20) + 15;
    break;

  case 18: /*{ Resist Petrification }*/
    player_flags.resist_petri += randint(15) + 10;
    break;

  case 19: /*{ Neutralize Poison }*/
    cure_player_status_effect(&player_flags.poisoned);
    break;

  case 20: /*{ Cure Serious Wounds }*/
    hp_player(damroll("9d4"), "a prayer.");
    break;

  case 21: /*{ Chant }*/
    bless(24 + randint(48));
    break;

  case 22: /*{ Sense Invisible }*/
    detect_inv2(randint(24) + 24);
    break;

  case 23: /*{ Protection from Evil }*/
    protect_evil();
    break;

  case 24: /*{ Earthquake }*/
    earthquake();
    break;

  case 25: /*{ Create food }*/
    create_food(3, 2, 1, 0, 0);
    break;

  case 26: /*{ Sense Surroundings }*/
    map_area();
    break;

  case 27: /*{ Orb of Draining }*/
    if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy, &x_dumy)) {
      fire_ball(SE_GOOD, dir, char_row, char_col, damroll("3d6") + player_lev,
                "Black Sphere");
    }
    break;

  case 28: /*{ Cure Critical Wounds }*/
    hp_player(damroll("20d4"), "a prayer.");
    break;

  case 29: /*{ Turn Undead }*/
    zap_area(0, 0, SE_TURN);
    break;

  case 30:                                 /*{ Prayer }*/
    player_flags.shero = 24 + randint(48); /* XXXX not cumulitive */
    break;

  case 31: /*{ Dispell Undead }*/
    zap_area(0x0008, 3 * player_lev, SE_HP);
    break;

  case 32: /*{ Resist Paralysis }*/
    player_flags.free_time += randint(20) + 15;
    break;

  case 33: /*{ Blade Barrier }*/
    player_flags.blade_ring += 3 + randint(3);
    break;

  case 34: /*{ Dispell Evil }*/
    zap_area(0x0004, 3 * player_lev, SE_HP);
    break;

  case 35: /*{ Heal }*/
    hp_player(200, "a prayer.");
    break;

  case 36: /*{ Resist Magic }*/
    player_flags.magic_prot += 40 + randint(40);
    break;

  case 37: /*{ Holy Thunder }*/
    msg_print("KABOOM!");
    zap_area(0x0004, 4 + randint(4), SE_THUNDER);
    break;

  case 38: /*{ Glyph of Warding }*/
    warding_glyph();
    break;

  case 39: /*{ Hero's Feast }*/
    msg_print("You have a marvelous meal!");
    player_hunger_set_status(FULL);
    prt_hunger();
    hp_player(200, "a prayer.");
    create_food(6, 4, 3, 2, 1);
    player_flags.status &= ~(IS_WEAK | IS_HUNGERY);
    prt_hunger();
    msg_print("You are full.");
    break;

  case 40: /*{ Holy Word }*/
    zap_area(0x0004, 6 * player_lev, SE_HOLY_WORD);
    cure_player_status_effect(&player_flags.afraid);
    cure_player_status_effect(&player_flags.poisoned);
    hp_player(1000, "a prayer.");
    break;

  default:
    break;
  }
  /*{ End of prayers...                             }*/
}
