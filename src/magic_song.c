#include "imoria.h"
#include "dungeon.h"

void song_spell_effects(long effect)
{
	/*{ Songs....					}*/
	long i2, i3, dir;
	long dumy, y_dumy, x_dumy;

	y_dumy = char_row;
	x_dumy = char_col;

	switch (effect + 1) {

	case 1: /*{ Detect Monster }*/
		detect_creatures(c_monster);
		break;

	case 2: /*{ Battle Song }*/
		bless(randint(12) + 12);
		break;

	case 3: /*{ Blink }*/
		teleport(10);
		break;

	case 4: /*{ Light }*/
		light_area(char_row, char_col);
		break;

	case 5: /*{ Detect Doors	}*/
		detect_sdoor();
		detect_trap();
		break;

	case 6: /*{ Magical Jig }*/
		if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy,
			       &x_dumy)) {
			zap_monster(dir, char_row, char_col, 0, c_confuse);
		}
		break;

	case 7: /*{ Detect Magic }*/
		detect_magic();
		break;

	case 8: /*{ Minor Cure }*/
		hp_player(damroll("5d3"), "a magic spell.");
		break;

	case 9: /*{ Battle Dance }*/
		py.flags.hero += (randint(10) + 5);
		bless(randint(20) + 20);
		break;

	case 10: /*{ Charm Monsters }*/
		sleep_monsters1(char_row, char_col);
		break;

	case 11: /*{ Detect Curse }*/
		detect_curse();
		break;

	case 12: /*{ Detect Invisible }*/
		detect_creatures(c_invisible);
		break;

	case 13: /*{ Cure Poison }*/
		cure_me(&(py.flags.poisoned));
		break;

	case 14: /*{ Invisibility }*/
		py.flags.temp_stealth += randint(15) + 10;
		break;

	case 15: /*{ Teleport Self }*/
		teleport(player_lev * 6);
		break;

	case 16: /*{ Infravision }*/
		py.flags.tim_infra += randint(50) + 50;
		break;

	case 17: /*{ Physical Humor }*/
		if (d__get_dir("Which diretion?", &dir, &dumy, &y_dumy,
			       &x_dumy)) {
			fire_bolt(c_joke, dir, char_row, char_col,
				  damroll("3d8"), "punch line");
		}
		break;

	case 18: /*{ Recharge Item }*/
		recharge(20);
		break;

	case 19: /*{ Remove Curse }*/
		for (i2 = Equipment_min; i2 <= EQUIP_MAX - 1; i2++) {
			/* with equipment[i2] do; */
			equipment[i2].flags &= 0x7FFFFFFF;
		}
		break;

	case 20: /*{ Legend Lore }*/
		ident_spell();
		break;

	case 21: /*{ Mass Charm }*/
		zap_area(0, 0, c_sleep);
		break;

	case 22: /*{ Detect Treasure }*/
		detect_item(c_treasure);
		break;

	case 23: /*{ Detect Object }*/
		detect_item(c_object);
		break;

	case 24: /*{ Resist Petrification }*/
		py.flags.resist_petri += randint(15) + 10;
		break;

	case 25: /*{ Create Food and Drink }*/
		create_food(3, 9, 1, 0, 0);
		break;

	case 26: /*{ Panic }*/
		msg_print("You scare the creatures!");
		zap_area(0, 0, c_confuse);
		break;

	case 27: /*{ Word of Recall }*/
		py.flags.word_recall = randint(20) + 20;
		break;

	case 28: /*{ Protection from Nature }*/
		/* with py.flags do; */
		PF.resist_heat += randint(15) + 10;
		PF.resist_cold += randint(15) + 10;
		PF.resist_lght += randint(15) + 10;
		break;

	case 29: /*{ See Invisible }*/
		detect_inv2(randint(24) + 24);
		break;

	case 30: /*{ Magic Mapping }*/
		map_area();
		break;

	case 31: /*{ Joke of Death }*/
		if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy,
			       &x_dumy)) {
			fire_ball(c_joke, dir, char_row, char_col, 30,
				  "terrible joke");
		}
		break;

	case 32: /*{ Battle Frenzy }*/
		bless(randint(30) + 30);
		py.flags.hero = randint(30) + 30;
		py.flags.shero = randint(30) + 30;
		break;

	case 33: /*{ Slow Creature }*/
		if (d__get_dir("Which direction?", &dir, &dumy, &y_dumy,
			       &x_dumy)) {
			zap_monster(dir, char_row, char_col, -1, c_speed);
		}
		break;

	case 34: /*{ Resist Charm }*/
		/* with py.flags do; */
		PF.free_time += randint(10) + player_lev;
		PF.magic_prot += randint(10) + player_lev;
		break;

	case 35: /*{ Item Lore }*/
		lore_spell();
		break;

	case 36: /*{ Song of Protection }*/
		py.flags.protmon = (randint(20) + player_lev);
		protect_evil();
		bless(randint(24) + 24);
		break;

	case 37: /*{ Last Laugh }*/
		zap_area(0, 50, c_joke);
		break;

	case 38: /*{ Teleport Level }*/
		dun_level += 2 * randint(2) - 3;
		if (dun_level < 1) {
			dun_level = 1;
		}
		moria_flag = true;
		break;

	case 39: /*{ Clairvoyance }*/
		redraw = true;
		wizard_light();
		for (i2 = (char_col + 1); i2 <= (char_col - 1); i2++) {
			for (i3 = (char_row + 1); i3 <= (char_row - 1); i3++) {
				if (test_light(i3, i2)) {
					redraw = false;
				}
			}
		}
		if (redraw) {
			wizard_light();
		}
		break;

	case 40: /*{ Song of Power }*/
		zap_area(0x0006, 4 * player_lev, c_hp);
		cure_me(&(py.flags.poisoned));
		hp_player(300, "a spell");
		cure_me(&py.flags.blind);
		break;

	default:
		break;
	}
	/*{ End of songs...			       }*/
}
