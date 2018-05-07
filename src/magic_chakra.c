#include "imoria.h"
#include "dungeon.h"

void chakra_spell_effects(long effect)
{
	/*{ Disciplines....}*/
	switch (effect + 1) {

	case 1: /*{ Self-Healing }*/
		hp_player(damroll("4d4"), "a magic spell.");
		break;

	case 2: /*{ Courage } */
		cure_me(&player_flags.afraid);
		break;

	case 3: /*{ Slow Poison } */
		slow_poison();
		break;

	case 4: /*{ Negate Hunger } */
		player_flags.foodc = PLAYER_FOOD_FULL + 4000;
		player_flags.status &= ~(IS_HUNGERY | IS_WEAK);
		prt_hunger();
		msg_print("You are full.");
		break;

	case 5: /*{ Sense Enemies }*/
		detect_creatures(c_creature);
		break;

	case 6: /*{ Self-Healing II }                 */
		hp_player(damroll("8d4"), "a prayer.");
		break;

	case 7: /*{ Night Vision }*/
		player_flags.tim_infra += randint(25) + 25;
		break;

	case 8: /*{ Poison Immunity }*/
		cure_me(&(player_flags.poisoned));
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
		player_flags.free_time += (randint(10) + player_lev);
		break;

	case 14: /*{ Improved Speed }*/
		PF.fast += randint(20) + player_lev;
		break;

	default:
		break;
	}
	/*{ End of Disciplines...}*/
}
