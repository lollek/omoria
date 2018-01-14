#include "imoria.h"
#include "dungeon.h"

void search_off()
{
	search_flag = false;
	find_flag = false;
	move_char(5);
	change_speed(-1);
	py.flags.status &= ~IS_SEARCHING;
	prt_search();
}

void search_on()
{
	/*{ Search Mode enhancement                               -RAK-   }*/

	search_flag = true;
	change_speed(+1);
	py.flags.status |= IS_SEARCHING;
	prt_search();
	/* with py.flags do; */
}

void rest_off()
{
	py.flags.rest = 0;
	py.flags.status &= ~IS_RESTING;
	py.flags.resting_till_full = false;
	if (msg_flag) {
		erase_line(1, 1);
	}
	prt_rest();
}

static void _move_char(integer dir)
{
	/*{ Moves player from one space to another...		-RAK-	}*/

	integer test_row = char_row;
	integer test_col = char_col;
	integer i1;
	integer i2;

	if (dir == 5)
		find_flag = false;

	/* Confused causes random movement 75% of the time */
	if (py.flags.confused > 0 && dir != 5 && randint(4) > 1) {
		dir = randint(9);
		find_flag = false;
	}

	/* Legal move? */
	if (!move_dir(dir, &test_row, &test_col))
		return;

	/* Creature in the way? Attack! */
	if (cave[test_row][test_col].cptr >= 2) {
		if (find_flag) {
			find_flag = false;
			move_light(char_row, char_col, char_row, char_col);
		}
		/* ..if we dare */
		if (py.flags.afraid < 1) {
			py_attack(test_row, test_col);
		} else {
			msg_print("You are too afraid!");
		}
		return;
	}

	/* Can't move onto floor space? */
	if (!cave[test_row][test_col].fopen) {
		/* Try a new direction if in find mode */
		if (pick_dir(dir))
			return;

		if (find_flag) {
			find_flag = false;
			move_char(5);
			return;
		}

		reset_flag = true;
		if (cave[test_row][test_col].tptr <= 0)
			return;

		if (t_list[cave[test_row][test_col].tptr].tval == rubble)
			msg_print("There is rubble blocking your way.");
		else if (t_list[cave[test_row][test_col].tptr].tval ==
			 closed_door)
			msg_print("There is a closed door blocking your way.");
		return;
	}

	/* Open floor spot */
	if (find_flag && (is_in(cave[char_row][char_col].fval, earth_set) ==
			  is_in(cave[test_row][test_col].fval, water_set))) {
		find_flag = false;
		move_char(5);
		return;
	}

	/* Move character record (-1) */
	move_rec(char_row, char_col, test_row, test_col);

	/* Check for new panel */
	if (get_panel(test_row, test_col, false))
		prt_map();

	/* Check to see if he should stop */
	if (find_flag)
		area_affect(dir, test_row, test_col);

	/* Check to see if he notices something */
	if (py.flags.blind < 1 && (randint(py.misc.fos) == 1 || search_flag))
		search(test_row, test_col, py.misc.srh);

	/* An object is beneath him? */
	if (cave[test_row][test_col].tptr > 0)
		carry(test_row, test_col);

	/* Move the light source */
	move_light(char_row, char_col, test_row, test_col);

	/* A room of light should be lit... */
	if (cave[test_row][test_col].fval == lopen_floor.ftval) {
		if (py.flags.blind < 1) {
			if (!(cave[test_row][test_col].pl)) {
				light_room(test_row, test_col);
			}
		}

		/* In doorway of light-room? */
	} else if ((cave[test_row][test_col].fval == corr_floor2.ftval ||
		    cave[test_row][test_col].fval == corr_floor3.ftval) &&
		   py.flags.blind < 1) {
		for (i1 = test_row - 1; i1 <= test_row + 1; i1++) {
			for (i2 = test_col - 1; i2 <= test_col + 1; i2++) {
				if (in_bounds(i1, i2) &&
				    cave[i1][i2].fval == lopen_floor.ftval &&
				    !cave[i1][i2].pl) {
					light_room(i1, i2);
				}
			}
		}
	}

	/* Make final assignments of char co-ords */
	char_row = test_row;
	char_col = test_col;
}

void move_char(integer dir)
{
	ENTER("move_char", "m");
	_move_char(dir);
	LEAVE("move_char", "m");
}

void regenhp(real percent)
{
	/*{ Regenerate hit points		-RAK-	}*/

	PM.chp += PM.mhp * percent + PLAYER_REGEN_HPBASE;
}

void regenmana(real percent)
{
	/*{ Regenerate mana points		-RAK-	}*/

	PM.cmana += PM.mana * percent + PLAYER_REGEN_MNBASE;
}

void take_hit(integer damage, vtype hit_from)
{
	/*{ Decreases players hit points and sets death flag if neccessary}*/

	ENTER("take_hit", "");

	if (py.flags.invuln > 0) {
		damage = 0;
	}

	py.misc.chp -= damage;

	if (search_flag) {
		search_off();
	}

	if (py.flags.rest > 0) {
		rest_off();
	}

	flush();

	if (py.misc.chp <= -1) {
		if (!death) {
			/*{ Hee, hee... Ain't I mean?     }*/
			death = true;
			strcpy(died_from, hit_from);
			total_winner = false;
		}
		moria_flag = true;
	} else {
		prt_hp();
	}

	LEAVE("take_hit", "");
}
