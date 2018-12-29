#include "imoria.h"

static long win_equip_x = 1;
static long win_equip_y = 26;

void C_print_equipment_block();
void prt_equipment(void)
{
#if DO_DEBUG
	C_print_equipment_block();
	prt_equipment_args(win_equip_y, win_equip_x, 1, false);
#else
	prt_equipment_args(1, 81, 1, false);
#endif
}

void prt_equipment_args(long y, long x, long start, boolean clear)
{
	long i;
	long counter = 0;
	char tmp_buf[82];

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
	ENTER(("draw_cave", ""));
	C_clear_screen();
	prt_stat_block();
	prt_map();
	prt_search();
	prt_equipment();
	refresh();
	LEAVE("draw_cave", "");
}

void prt_map()
{
	long y;
	long panel_y = 1;	 /* Used for erasing dirty lines */
	long const panel_x0 = 14; /*{ Erasure starts in this column  }*/

	ENTER(("prt_map", ""));

	redraw = false; /*{ Screen has been redrawn	   }*/
	for (y = panel_row_min; y <= panel_row_max; y++) {
		chtype floor_str[82] = {0}; /* string to be printed */
		long floor_str_len = 0;     /* floor_str length counter */
		long isp = 0;		    /* Number of blanks encountered */
		boolean flag = false;       /* False until floor_str <> '' */
		long xpos = 0;
		long x;

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
					long i5;
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

void prt_num(char header[82], long num, long row, long column)
{
	char out_val[82];

	sprintf(out_val, "%s%6ld  ", header, num);
	put_buffer(out_val, row, column);
}

extern void C_print_stat_block();
void prt_stat_block()
{
	ENTER(("prt_stat_block", ""));

	C_print_stat_block();

	prt_hunger();
	prt_blind();
	prt_confused();
	prt_afraid();
	prt_poisoned();
	prt_search();
	prt_rest();
	prt_quested();
	prt_light_on();

	LEAVE("prt_stat_block", "");
}

void prt_field(char info[82], long row, long column)
{
	char out_val1[82];

	sprintf(out_val1, "%-14s", info);
	put_buffer(out_val1, row, column);
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

void prt_hunger()
{
	if ((IS_WEAK & player_flags.status) != 0) {
		put_buffer_attr("Weak    ", STATUS_ROW, HUNGER_COLUMN,
				A_BOLD | A_BLINK);
	} else if ((IS_HUNGERY & player_flags.status) != 0) {
		put_buffer_attr("Hungry  ", STATUS_ROW, HUNGER_COLUMN, A_BOLD);
	} else {
		put_buffer("        ", STATUS_ROW, HUNGER_COLUMN);
	}
}

void prt_blind()
{
	if ((IS_BLIND & player_flags.status) != 0) {
		put_buffer_attr("Blind  ", STATUS_ROW, BLIND_COLUMN, A_BOLD);
	} else {
		put_buffer("       ", STATUS_ROW, BLIND_COLUMN);
	}
}

void prt_confused()
{
	if ((IS_CONFUSED & player_flags.status) != 0) {
		put_buffer_attr("Confused  ", STATUS_ROW, CONFUSED_COLUMN,
				A_BOLD);
	} else {
		put_buffer("          ", STATUS_ROW, CONFUSED_COLUMN);
	}
}

void prt_afraid()
{
	if ((IS_AFRAID & player_flags.status) != 0) {
		put_buffer_attr("Afraid  ", STATUS_ROW, AFRAID_COLUMN, A_BOLD);
	} else {
		put_buffer("        ", STATUS_ROW, AFRAID_COLUMN);
	}
}

void prt_poisoned()
{
	if ((IS_POISONED & player_flags.status) != 0) {
		put_buffer_attr("Poisoned  ", STATUS_ROW, POISONED_COLUMN,
				A_BOLD);
	} else {
		put_buffer("          ", STATUS_ROW, POISONED_COLUMN);
	}
}

void prt_search()
{
	if ((IS_SEARCHING & player_flags.status) != 0) {
		put_buffer("Searching", STATUS_ROW, SEARCHING_COLUMN);
	} else {
		put_buffer("         ", STATUS_ROW, SEARCHING_COLUMN);
	}
}

void prt_rest()
{

	if ((IS_RESTING & player_flags.status) != 0) {
		put_buffer("Resting  ", STATUS_ROW, RESTING_COLUMN);
	} else {
		put_buffer("         ", STATUS_ROW, RESTING_COLUMN);
	}
}

void prt_quested()
{
	if (player_flags.quested) {
		put_buffer(" Quest  ", STATUS_ROW, QUESTED_COLUMN);
	} else if (player_cur_quest > 0) {
		put_buffer("  Done  ", STATUS_ROW, QUESTED_COLUMN);
	} else {
		put_buffer("        ", STATUS_ROW, QUESTED_COLUMN);
	}
}
