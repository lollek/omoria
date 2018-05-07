#include "imoria.h"
#include "dungeon.h"

/* P_STATS */
uint8_t player_stats_perm[] = {0, 0, 0, 0, 0, 0};
uint8_t player_stats_curr[] = {0, 0, 0, 0, 0, 0};
int8_t  player_stats_mod[] = {0, 0, 0, 0, 0, 0};
uint8_t player_stats_lost[] = {0, 0, 0, 0, 0, 0};

/* P_MISC */
int64_t player_xtr_wgt = 0;
int64_t player_account = 0;
money_type player_money = {0, 0, 0, 0, 0, 0, 0};
game_time_type player_birth = {0, 0, 0, 0, 0};
game_time_type player_cur_age = {0, 0, 0, 0, 0};
time_type player_play_tm = {0, 0, 0, 0, 0, 0, 0};
uint8_t player_diffic = 3;
char player_name[82] = " ";
char player_race[82] = " ";
char player_sex[82] = " ";
char player_title[82] = " ";
char player_tclass[82] = " ";
int64_t player_max_exp = 0;
int64_t player_exp = 0;
int64_t player_rep = 0;
int64_t player_deaths = 0;
int64_t player_premium = 0;
uint16_t player_age = 0;
uint16_t player_ht = 0;
uint16_t player_wt = 0;
uint16_t player_lev = 0;
uint16_t player_max_lev = 0;
int16_t player_srh = 0;
int16_t player_fos = 0;
int16_t player_bth = 0;
int16_t player_bthb = 0;
int16_t player_mana = 0;
int16_t player_mhp = 0;
int16_t player_ptohit = 0;
int16_t player_ptodam = 0;
int16_t player_pac = 0;
int16_t player_ptoac = 0;
int16_t player_dis_th = 0;
int16_t player_dis_td = 0;
int16_t player_dis_ac = 0;
int16_t player_dis_tac = 0;
int16_t player_disarm = 0;
int16_t player_save = 0;
int16_t player_sc = 0;
enum class_t player_pclass = 0;
uint8_t player_prace = 0;
uint8_t player_hitdie = 0;
uint8_t player_stl = 0;
float player_expfact = 0;
float player_cmana = 0;
float player_chp = 0;
char player_history[][82] = {"", "", "", "", ""};
boolean player_cheated = false;
int64_t  player_mr = 0;
uint8_t player_quests = 0;
uint16_t player_cur_quest = 0;
time_t player_creation_time = 0;
int64_t player_save_count = 0;
int64_t player_claim_check = 0;

void search_off()
{
	search_flag = false;
	find_flag = false;
	move_char(5);
	change_speed(-1);
	player_flags.status &= ~IS_SEARCHING;
	prt_search();
}

void search_on()
{
	/*{ Search Mode enhancement                               -RAK-   }*/

	search_flag = true;
	change_speed(+1);
	player_flags.status |= IS_SEARCHING;
	prt_search();
	/* with player_flags do; */
}

void rest_off()
{
	player_flags.rest = 0;
	player_flags.status &= ~IS_RESTING;
	player_flags.resting_till_full = false;
	if (msg_flag) {
		erase_line(1, 1);
	}
	prt_rest();
}

static void _move_char(long dir)
{
	/*{ Moves player from one space to another...		-RAK-	}*/

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
		search(test_row, test_col, player_srh);

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

void regenhp(float percent)
{
	/*{ Regenerate hit points		-RAK-	}*/

	player_chp += player_mhp * percent + PLAYER_REGEN_HPBASE;
}

void regenmana(float percent)
{
	/*{ Regenerate mana points		-RAK-	}*/

	player_cmana += player_mana * percent + PLAYER_REGEN_MNBASE;
}

void take_hit(long damage, char hit_from[82])
{
	/*{ Decreases players hit points and sets death flag if neccessary}*/

	ENTER(("take_hit", "%d, %s", damage, hit_from));

	if (player_flags.invuln > 0) {
		damage = 0;
	}

	player_chp -= damage;

	if (search_flag) {
		search_off();
	}

	if (player_flags.rest > 0) {
		rest_off();
	}

	flush();

	if (player_chp <= -1) {
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
