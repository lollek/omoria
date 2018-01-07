#include "imoria.h"

static integer win_equip_x = 81;
static integer win_equip_y = 1;

void prt_equipment(void)
{
	prt_equipment_args(win_equip_y, win_equip_x, 1, false);
}

void prt_equipment_args(integer y, integer x, integer start, boolean clear)
{
	integer i;
	integer counter = 0;
	vtype tmp_buf;

	for (i = Equipment_min; i < EQUIP_MAX - 1; ++i) {
		if (!equipment[i].tval)
			continue;
		++counter;
		if (counter < start)
			continue;
		inv__equip_pos_string(tmp_buf, i, counter);
		prt(tmp_buf, y + counter, x);
	}
	if (clear) {
		prt("", y + counter + 1, x);
	}
}

void draw_cave()
{
	ENTER("draw_cave", "");
	clear_screen();
	prt_stat_block();
	prt_map();
	prt_depth();
	prt_search();
	prt_equipment();
	put_qio();
	LEAVE("draw_cave", "");
}

void prt_map()
{
	integer y;
	integer panel_y = 1;	 /* Used for erasing dirty lines */
	integer const panel_x0 = 14; /*{ Erasure starts in this column  }*/

	ENTER("prt_map", "");

	redraw = false; /*{ Screen has been redrawn	   }*/
	for (y = panel_row_min; y <= panel_row_max; y++) {
		chtype floor_str[82] = {0}; /* string to be printed */
		integer floor_str_len = 0;  /* floor_str length counter */
		integer isp = 0;	    /* Number of blanks encountered */
		boolean flag = false;       /* False until floor_str <> '' */
		integer xpos = 0;
		integer x;

		/* Clean line if dirty */
		panel_y++;
		if (used_line[panel_y]) {
			erase_line(panel_y, panel_x0);
			used_line[panel_y] = false;
		}

		for (x = panel_col_min; x <= panel_col_max; x++) {
			chtype tmp_char = ' ';
			if (test_light(y, x))
				tmp_char = loc_symbol(y, x);
			else if ((cave[y][x].cptr == 1) && (!find_flag))
				tmp_char = '@';
			else if (cave[y][x].cptr > 1 &&
				 m_list[cave[y][x].cptr].ml)
				tmp_char = loc_symbol(y, x);

			if (tmp_char == ' ') {
				if (flag) {
					isp++;
					if (isp > 3) {
						floor_str[floor_str_len] = 0;
						print_chstr(floor_str, y, xpos);
						flag = false;
						isp = 0;
					}
				}

			} else if (flag) {
				if (isp > 0) {
					integer i5;
					for (i5 = 0; i5 < isp; i5++)
						floor_str[floor_str_len++] =
						    ' ';
					isp = 0;
				}
				floor_str[floor_str_len++] = tmp_char;

			} else {
				xpos = x;
				flag = true;
				floor_str_len = 0;
				floor_str[floor_str_len++] = tmp_char;
			}
		}
		if (flag) {
			floor_str[floor_str_len] = 0;
			print_chstr(floor_str, y, xpos);
		}
	}
	LEAVE("prt_map", "");
}

void prt_6_stats(stat_s_type p, stat_s_type l, byteint row, byteint col)
{
	ENTER("prt_6_stats", "");
	if (l != NULL) {
		prt_stat_attr("STR : ", p[STR], l[STR], row, col);
		prt_stat_attr("INT : ", p[INT], l[INT], row + 1, col);
		prt_stat_attr("WIS : ", p[WIS], l[WIS], row + 2, col);
		prt_stat_attr("DEX : ", p[DEX], l[DEX], row + 3, col);
		prt_stat_attr("CON : ", p[CON], l[CON], row + 4, col);
		prt_stat_attr("CHR : ", p[CHR], l[CHR], row + 5, col);
	} else {
		prt_stat("STR : ", p[STR], row, col);
		prt_stat("INT : ", p[INT], row + 1, col);
		prt_stat("WIS : ", p[WIS], row + 2, col);
		prt_stat("DEX : ", p[DEX], row + 3, col);
		prt_stat("CON : ", p[CON], row + 4, col);
		prt_stat("CHR : ", p[CHR], row + 5, col);
	}
	LEAVE("prt_6_stats", "");
}

void prt_stat_attr(vtype stat_name, byteint stat, byteint loss, integer row,
		   integer column)
{
	stat_s_type out_val1;

	ENTER("prt_stat_attr", "");

	if (loss == 0) {
		prt_stat(stat_name, stat, row, column);
	} else {
		cnv_stat(stat, out_val1);
		put_buffer_attr(stat_name, row, column, A_DIM);
		put_buffer(out_val1, row, column + strlen(stat_name));
	}

	LEAVE("prt_stat_attr", "");
}

void prt_stat(vtype stat_name, byteint stat, integer row, integer column)
{
	stat_s_type out_val1;
	vtype out_val2;

	ENTER("prt_stat", "");

	cnv_stat(stat, out_val1);
	sprintf(out_val2, "%s%s", stat_name, out_val1);
	put_buffer(out_val2, row, column);

	LEAVE("prt_stat", "");
}

void cnv_stat(byteint stat, stat_type out_val)
{
	integer part1, part2;

	sprintf(out_val, "      ");

	if (stat > 150) {
		part1 = 18;
		part2 = stat - 150;
		sprintf(out_val, "%2ld/%-2ld", part1, part2);
	} else {
		sprintf(out_val, "%2d   ", 3 + (stat div 10));
	}
}

void prt_num(vtype header, integer num, integer row, integer column)
{
	vtype out_val;

	sprintf(out_val, "%s%6ld  ", header, num);
	put_buffer(out_val, row, column);
}

void prt_stat_block()
{
	ENTER("prt_stat_block", "");
	prt_field(py.misc.race, RACE_ROW, STAT_COLUMN);
	prt_field(py.misc.tclass, CLASS_ROW, STAT_COLUMN);
	prt_title();
	prt_6_stats(py.stat.c, py.stat.l, STR_ROW, STAT_COLUMN);
	prt_num("LVL : ", py.misc.lev, LEVEL_ROW, STAT_COLUMN);
	prt_num("EXP : ", py.misc.exp, EXP_ROW, STAT_COLUMN);
	if (is_magii) {
		prt_field("MANA: ", MANA_ROW, STAT_COLUMN);
		prt_mana();
	}
	prt_field("HP  : ", HP_ROW, STAT_COLUMN);
	prt_hp();
	prt_num("QST : ", py.misc.quests, QUEST_ROW, STAT_COLUMN);
	prt_num("AC  : ", py.misc.dis_ac, AC_ROW, STAT_COLUMN);
	prt_num("GOLD: ", py.misc.money[TOTAL_], GOLD_ROW, STAT_COLUMN);
	prt_field("WGHT:", WEIGHT_ROW, STAT_COLUMN);
	prt_field("M_WT:", WEIGHT_ROW + 1, STAT_COLUMN);
	prt_weight();
	prt_time();
	if (total_winner) {
		prt_winner();
	}
	prt_hunger(); /*{'If' statements here redundant and unnecessary, so}*/
	prt_blind();  /*{ removed per Dean's suggestion                -MAV}*/
	prt_confused();
	prt_afraid();
	prt_poisoned();
	prt_search();
	prt_rest();
	prt_quested();
	prt_light_on();
	LEAVE("prt_stat_block", "");
}

void prt_field(vtype info, integer row, integer column)
{
	vtype out_val1;

	sprintf(out_val1, "%-14s", info);
	put_buffer(out_val1, row, column);
}

void prt_title() { prt_field(py.misc.title, TITLE_ROW, STAT_COLUMN); }

void prt_hp()
{
	vtype buf;

	sprintf(buf, "%6d  ", (int)(py.misc.chp));
	if (py.misc.chp == py.misc.mhp) {
		attron(COLOR_PAIR(COLOR_GREEN));
		put_buffer(buf, HP_ROW, STAT_COLUMN + 6);
		attroff(COLOR_PAIR(COLOR_GREEN));
	} else if (py.misc.chp >= py.misc.mhp / 3) {
		attron(COLOR_PAIR(COLOR_YELLOW));
		put_buffer(buf, HP_ROW, STAT_COLUMN + 6);
		attroff(COLOR_PAIR(COLOR_YELLOW));
	} else {
		attron(COLOR_PAIR(COLOR_RED));
		put_buffer(buf, HP_ROW, STAT_COLUMN + 6);
		attroff(COLOR_PAIR(COLOR_RED));
	}
}

void prt_pac() { prt_num("", py.misc.dis_ac, AC_ROW, STAT_COLUMN + 6); }

void prt_gold()
{
	prt_num("", py.misc.money[TOTAL_], GOLD_ROW, STAT_COLUMN + 6);
}

void prt_weight()
{
	prt_num("", inven_weight div 100, WEIGHT_ROW, STAT_COLUMN + 6);
	prt_num("", weight_limit(), WEIGHT_ROW + 1, STAT_COLUMN + 6);
}

void prt_time()
{
	vtype s1, s2, s3;
	vtype out_val;

	sprintf(out_val, "%s %s %s",
		time_string(py.misc.cur_age.hour, py.misc.cur_age.secs, s1),
		day_of_week_string(py.misc.cur_age.day, 2, s2),
		place_string(py.misc.cur_age.day, s3));

	put_buffer(out_val, TIME_ROW, STAT_COLUMN);
}

void prt_light_on()
{
	if (PF.light_on) {
		prt("         ", STATUS_ROW + 1, LIGHT_ON_COLUMN);
	} else {
		put_buffer_attr("Light Off", STATUS_ROW + 1, LIGHT_ON_COLUMN,
				A_DIM);
	}
}

void prt_depth()
{
	vtype depths;
	integer depth;

	depth = dun_level * 50;
	if (depth == 0) {
		strcpy(depths, "Town level");
	} else if (depth < 10000) {
		sprintf(depths, "Depth: %ld (feet)", depth);
	} else {
		sprintf(depths, "Depth: %ld   ", depth);
	}

	prt(depths, STATUS_ROW, DEPTH_COLUMN);
}

void prt_hunger()
{
	if ((IS_WEAK & py.flags.status) != 0) {
		put_buffer_attr("Weak    ", STATUS_ROW, HUNGER_COLUMN,
				A_BOLD | A_BLINK);
	} else if ((IS_HUNGERY & py.flags.status) != 0) {
		put_buffer_attr("Hungry  ", STATUS_ROW, HUNGER_COLUMN, A_BOLD);
	} else {
		put_buffer("        ", STATUS_ROW, HUNGER_COLUMN);
	}
}

void prt_blind()
{
	if ((IS_BLIND & py.flags.status) != 0) {
		put_buffer_attr("Blind  ", STATUS_ROW, BLIND_COLUMN, A_BOLD);
	} else {
		put_buffer("       ", STATUS_ROW, BLIND_COLUMN);
	}
}

void prt_confused()
{
	if ((IS_CONFUSED & py.flags.status) != 0) {
		put_buffer_attr("Confused  ", STATUS_ROW, CONFUSED_COLUMN,
				A_BOLD);
	} else {
		put_buffer("          ", STATUS_ROW, CONFUSED_COLUMN);
	}
}

void prt_afraid()
{
	if ((IS_AFRAID & py.flags.status) != 0) {
		put_buffer_attr("Afraid  ", STATUS_ROW, AFRAID_COLUMN, A_BOLD);
	} else {
		put_buffer("        ", STATUS_ROW, AFRAID_COLUMN);
	}
}

void prt_poisoned()
{
	if ((IS_POISONED & py.flags.status) != 0) {
		put_buffer_attr("Poisoned  ", STATUS_ROW, POISONED_COLUMN,
				A_BOLD);
	} else {
		put_buffer("          ", STATUS_ROW, POISONED_COLUMN);
	}
}

void prt_search()
{
	if ((IS_SEARCHING & py.flags.status) != 0) {
		put_buffer("Searching", STATUS_ROW, SEARCHING_COLUMN);
	} else {
		put_buffer("         ", STATUS_ROW, SEARCHING_COLUMN);
	}
}

void prt_rest()
{

	if ((IS_RESTING & py.flags.status) != 0) {
		put_buffer("Resting  ", STATUS_ROW, RESTING_COLUMN);
	} else {
		put_buffer("         ", STATUS_ROW, RESTING_COLUMN);
	}
}

void prt_quested()
{
	if (py.flags.quested) {
		put_buffer(" Quest  ", STATUS_ROW, QUESTED_COLUMN);
	} else if (py.misc.cur_quest > 0) {
		put_buffer("  Done  ", STATUS_ROW, QUESTED_COLUMN);
	} else {
		put_buffer("        ", STATUS_ROW, QUESTED_COLUMN);
	}
}

void prt_winner() { put_buffer("*Winner*", WINNER_ROW, WINNER_COLUMN); }

void prt_experience()
{
	/*      with py.misc do*/
	if (py.misc.exp > player_max_exp) {
		py.misc.exp = player_max_exp;
	}

	if (py.misc.lev < MAX_PLAYER_LEVEL) {
		while ((player_exp[py.misc.lev] * py.misc.expfact) <=
		       py.misc.exp) {
			gain_level();
		}

		if (py.misc.exp > py.misc.max_exp) {
			py.misc.max_exp = py.misc.exp;
		}
	}

	prt_num("", py.misc.exp, EXP_ROW, STAT_COLUMN + 6);
}

void prt_mana()
{
	vtype buf;

	sprintf(buf, "%6d  ", (int)(py.misc.cmana));
	if (py.misc.cmana == py.misc.mana) {
		attron(COLOR_PAIR(COLOR_BLUE));
		put_buffer(buf, MANA_ROW, STAT_COLUMN + 6);
		attroff(COLOR_PAIR(COLOR_BLUE));
	} else if (py.misc.cmana >= py.misc.mana / 3) {
		attron(COLOR_PAIR(COLOR_CYAN));
		put_buffer(buf, MANA_ROW, STAT_COLUMN + 6);
		attroff(COLOR_PAIR(COLOR_CYAN));
	} else {
		attron(COLOR_PAIR(COLOR_MAGENTA));
		put_buffer(buf, MANA_ROW, STAT_COLUMN + 6);
		attroff(COLOR_PAIR(COLOR_MAGENTA));
	}
}

void prt_level() { prt_num("", py.misc.lev, LEVEL_ROW, STAT_COLUMN + 6); }

void prt_a_stat(stat_set tstat)
{
	char *stat_names[STAT_SET_MAX + 1] = {"STR : ", "INT : ", "WIS : ",
					      "DEX : ", "CON : ", "CHR : "};

	prt_stat_attr(stat_names[(int)tstat], py.stat.c[(int)tstat],
		      py.stat.l[(int)tstat], STR_ROW + tstat, STAT_COLUMN);
}
