#include "imoria.h"
#include "dungeon.h"

/**
 * -RAK-
 *  pick_dir() - Picks new direction when in find mode
 *  @dir: Initial direction
 */
static boolean pick_dir(long dir)
{

	long z[2];
	long i;
	boolean return_value = false;

	if (!find_flag)
		return false;

	if (next_to4(char_row, char_col, corr_set) != 2)
		return false;

	switch (dir) {
		case 1:
		case 3:
		case 7:
		case 9:
			z[0] = rotate_dir(dir, -1);
			z[1] = rotate_dir(dir, 1);
			break;

		case 2:
		case 4:
		case 6:
		case 8:
			z[0] = rotate_dir(dir, -2);
			z[1] = rotate_dir(dir, 2);
			break;
	}

	for (i = 0; i < 2; i++) {
		long y = char_row;
		long x = char_col;
		if (move_dir(z[i], &y, &x)) {
			if (cave[y][x].fopen) {
				return_value = true;
				com_val = z[i] + 48;
			}
		}
	}

	return return_value;
}

static void _move_char(long dir)
{
	long test_row = char_row;
	long test_col = char_col;
	long i1;
	long i2;

	if (dir == 5)
		find_flag = false;

	/* Confused causes random movement 75% of the time */
	if (player_flags.confused > 0 && dir != 5 && randint(4) > 1) {
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
		if (player_flags.afraid < 1) {
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
	if (player_flags.blind < 1 && (randint(player_fos) == 1 || search_flag))
		search(test_row, test_col, C_player_get_curr_search_skill());

	/* An object is beneath him? */
	if (cave[test_row][test_col].tptr > 0)
		carry(test_row, test_col);

	/* Move the light source */
	move_light(char_row, char_col, test_row, test_col);

	/* A room of light should be lit... */
	if (cave[test_row][test_col].fval == lopen_floor.ftval) {
		if (player_flags.blind < 1) {
			if (!(cave[test_row][test_col].pl)) {
				light_room(test_row, test_col);
			}
		}

		/* In doorway of light-room? */
	} else if ((cave[test_row][test_col].fval == corr_floor2.ftval ||
		    cave[test_row][test_col].fval == corr_floor3.ftval) &&
		   player_flags.blind < 1) {
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

void move_char(long dir)
{
	ENTER(("move_char", "%d", dir));
	_move_char(dir);
	LEAVE("move_char", "m");
}
