/* stuff to print stuff to the screen. This file originaly came from umoria. */

#include <sys/wait.h>

#include "imoria.h"
#include "save.h"

/* source/io.c: terminal I/O code, uses the curses package

   Copyright (c) 1989-94 James E. Wilson, Robert A. Koeneke

   This software may be copied and distributed for educational, research, and
   not for profit purposes provided that this copyright and statement are
   included in all such copies. */

#include <stdio.h>

#include "config.h"

#include <curses.h>
#include <ctype.h>
#include <sys/ioctl.h>
#include <signal.h>
#include <string.h>
#include <termios.h>

static void minor_error(char const *error_message)
{
	abort();
	/* clear msg_flag to avoid problems with unflushed messages */
	msg_flag = 0;
	prt_(error_message, 0, 0);
	bell();
	/* wait so user can see error */
	sleep(2);
}

/* initializes curses routines */
void init_curses()
{
	printf("Attempting to start curses...\n");
	fflush(stdout);

	if (initscr() == NULL) {
		fprintf(stderr, "Error allocating screen in curses package.\n\r");
		exit(1);
	}

	if (LINES < 24 || COLS < 80) {
		fprintf(stderr, "Screen too small for moria.\n\r");
		exit(1);
	}

	clear();
	refresh();

	start_color();
	init_pair(COLOR_RED, COLOR_RED, COLOR_BLACK);
	init_pair(COLOR_GREEN, COLOR_GREEN, COLOR_BLACK);
	init_pair(COLOR_YELLOW, COLOR_YELLOW, COLOR_BLACK);
	init_pair(COLOR_BLUE, COLOR_BLUE, COLOR_BLACK);
	init_pair(COLOR_MAGENTA, COLOR_MAGENTA, COLOR_BLACK);
	init_pair(COLOR_CYAN, COLOR_CYAN, COLOR_BLACK);

	crmode();
	noecho();
	nonl();
}

void highlite_on() { attron(A_DIM); }
void highlite_off() { attroff(A_DIM); }

void put_buffer_attr(const char *out_str, long row, long col, int attrs)
{
	attr_t old_attr;
	short unused_pair;
	short unused_opts;

	(void)unused_opts;
	attr_get(&old_attr, &unused_pair, &unused_opts);
	attrset(attrs);
	put_buffer(out_str, row, col);
	attrset(old_attr);
}

/* Returns a single character input from the terminal.	This silently -CJS-
   consumes ^R to redraw the screen and reset the terminal, so that this
   operation can always be performed at any input prompt.  inkey() never
   returns ^R.	*/
char inkey()
{
	int i;

	refresh();	 /* Dump IO buffer		*/
	command_count = 0; /* Just to be safe -CJS- */
	while (TRUE) {
		i = getch();
		/* some machines may not sign extend. */
		if (i == EOF) {
			eof_flag++;
			/* avoid infinite loops while trying to call inkey() for
			   a -more-
			   prompt. */
			msg_flag = FALSE;

			refresh();
			if (!character_generated || character_saved)
				exit_game();
			/*  disturb(1, 0);*/
			if (eof_flag > 100) {
				/* just in case, to make sure that the process
				 * eventually dies */
				panic_save = 1;
				strcpy(died_from, "(end of input: panic saved)");
				if (!save_char(TRUE)) {
					strcpy(died_from, "panic: unexpected eof");
					death = TRUE;
				}
				exit_game();
			}
			return ESCAPE;
		}
		if (i != CTRL_('R')) {
			msg_flag = false;
			return (char)i;
		}
		refresh();
	}
}

/* Flush the buffer					-RAK-	*/
void flush()
{
	/* the code originally used ioctls, TIOCDRAIN, or TIOCGETP/TIOCSETP, or
	   TCGETA/TCSETAF, however this occasionally resulted in loss of output,
	   the happened especially often when rlogin from BSD to SYS_V machine,
	   using check_input makes the desired effect a bit clearer */
	/* wierd things happen on EOF, don't try to flush input in that case */
	if (!eof_flag)
		while (check_input(0))
			;
	/* used to call refresh() here to drain output, but it is not necessary
	 */
}

/* Clears given line of text				-RAK-	*/
void Erase_Line(long row, long col)
{
	if (row == MSG_LINE && msg_flag)
		msg_print(CNIL);
	(void)move((int)row, (int)col);
	clrtoeol();
}

void Clear_From(int row)
{
	(void)move(row, 0);
	clrtobot();
}

/* Outputs a char to a given interpolated y, x position	-RAK-	*/
/* sign bit of a character used to indicate standout mode. -CJS */
void Print(chtype const ch, int row, int col)
{
	char tmp_str[82];

	row -= panel_row_prt; /* Real co-ords convert to screen positions */
	col -= panel_col_prt;

	used_line[row + 1] = true;

	if ((row > 24) || (row < 0) || (col > 79) || (col < 0)) {
		sprintf(tmp_str, "error in print, row = %d col = %d\n", row,
			col);
		minor_error(tmp_str);
	}

	if (mvaddch(row, col, ch) == ERR) {
		sprintf(tmp_str, "error in print, row = %d col = %d\n", row,
			col);
		minor_error(tmp_str);
	}
}

/* Moves the cursor to a given interpolated y, x position	-RAK-	*/
void move_cursor_relative(int row, int col)
{
	char tmp_str[82];

	row -= panel_row_prt; /* Real co-ords convert to screen positions */
	col -= panel_col_prt;
	if (move(row, col) == ERR) {
		abort();
		/* clear msg_flag to avoid problems with unflushed messages */
		msg_flag = 0;
		(void)sprintf(
		    tmp_str,
		    "error in move_cursor_relative, row = %d col = %d\n", row,
		    col);
		prt_(tmp_str, 0, 0);
		bell();
		/* wait so user can see error */
		(void)sleep(2);
	}
}

/* Print a message so as not to interrupt a counted command. -CJS- */
void count_msg_print(p) char *p;
{
	int i = command_count;
	msg_print(p);
	command_count = i;
}

void prt2(char *str_buff1, char *str_buff2, int row, int col)
{
	char temp[82];
	sprintf(temp, "%s%s", str_buff1, str_buff2);
	prt(temp, row, col);
}

/* move cursor to a given y, x position */
void move_cursor(int row, int col) { (void)move(row, col); }

/* Gets a string terminated by <RETURN>		*/
/* Function returns false if <ESCAPE> is input	*/
boolean Get_String(char *in_str, int row, int column, int slen)
{
	register int start_col, end_col, i;
	char *p;
	int flag, aborted;

	aborted = FALSE;
	flag = FALSE;
	(void)move(row, column);
	for (i = slen; i > 0; i--)
		(void)addch(' ');
	(void)move(row, column);
	start_col = column;
	end_col = column + slen - 1;
	if (end_col > 79) {
		slen = 80 - column;
		end_col = 79;
	}
	p = in_str;
	do {
		i = inkey();
		switch (i) {
		case ESCAPE:
			aborted = TRUE;
			break;
		case CTRL_('J'):
		case CTRL_('M'):
			flag = TRUE;
			break;
		case DELETE:
		case CTRL_('H'):
			if (column > start_col) {
				column--;
				put_buffer_(" ", row, column);
				move_cursor(row, column);
				*--p = '\0';
			}
			break;
		default:
			if (!isprint(i) || column > end_col)
				bell();
			else {
				mvaddch(row, column, (char)i);
				*p++ = i;
				column++;
			}
			break;
		}
	} while ((!flag) && (!aborted));
	if (aborted)
		return (FALSE);
	/* Remove trailing blanks	*/
	while (p > in_str && p[-1] == ' ')
		p--;
	*p = '\0';
	return (TRUE);
}

/* Pauses for user response before returning		-RAK-	*/
void Pause_Line(prt_line) int prt_line;
{
	prt_("[Press any key to continue.]", prt_line, 23);
	(void)inkey();
	Erase_Line(prt_line, 0);
}

void bell()
{
	refresh();

	/* The player can turn off beeps if he/she finds them annoying.  */
	if (!sound_beep_flag)
		return;

	write(1, "\007", 1);
}

/* screen_map code taken from umoria 5.5 */

/* Display highest priority object in the RATIO by RATIO area */
#define RATIO 3

void screen_map()
{
	int i;
	chtype map[MAX_WIDTH / RATIO + 1];
	int priority[256];
	int orow = -1;
	int myrow = 0;
	int mycol = 0;

	memset(priority, 0, sizeof(priority));
	priority['@'] = 10;
	priority['<'] = 5;
	priority['>'] = 5;
	priority['\''] = -3;
	priority['#'] = -5;
	priority['.'] = -10;
	priority[' '] = -15;

	clear_rc(1, 1);

	/* Add top border */
	mvaddch(0, 0, '+');
	for (i = 0; i < MAX_WIDTH / RATIO; i++)
		addch('-');
	addch('+');

	map[MAX_WIDTH / RATIO] = '\0';
	for (i = 0; i < MAX_HEIGHT; i++) {
		int const row = i / RATIO;
		int j;

		if (row != orow) {
			if (orow >= 0) {
				mvaddch(orow + 1, 0, '|');
				mvaddchstr(orow + 1, 1, map);
				mvaddch(orow + 1, MAX_WIDTH / RATIO + 1, '|');
			}
			for (j = 0; j < MAX_WIDTH / RATIO; j++)
				map[j] = ' ';
			orow = row;
		}
		for (j = 0; j < MAX_WIDTH; j++) {
			int const col = j / RATIO;
			chtype const tmp = get_loc_symbol(i + 1, j + 1);

			if (priority[(unsigned char)map[col]] <
			    priority[(unsigned char)tmp]) {
				map[col] = tmp;
			}
			if (map[col] == '@') {
				mycol = col + 1; /* account for border */
				myrow = row + 1;
			}
		}
	}
	if (orow >= 0) {
		mvaddch(orow + 1, 0, '|');
		mvaddchstr(orow + 1, 1, map);
		mvaddch(orow + 1, MAX_WIDTH / RATIO + 1, '|');
	}
	mvaddch(orow + 2, 0, '+');
	for (i = 0; i < MAX_WIDTH / RATIO; i++) {
		addch('-');
	}
	addch('+');

	mvaddstr(23, 23, "Hit any key to continue");
	if (mycol > 0)
		move(myrow, mycol);
	inkey();
	draw_cave();
}

boolean sl__get_dir(char *prompt, long *dir)
{
	char command;
	boolean return_value = false;

	for (;;) {
		if (get_com(prompt, &command)) {
			if (command >= '1' && command <= '9') {
				*dir = command - '0';
				return_value = true;
				break;
			}
		} else {
			break;
		}
	}

	return return_value;
}

void show_location()
{
/* taken from umoria 5.5 */

	/* (W)here are we on the map	(L)ocate on map */

	int cy, cx, p_y, p_x, y, x;
	long dir_val;
	char tmp_str[1026], out_val[1026];

	if ((player_flags.blind > 0) || no_light()) {
		msg_print("You can't see your map.");
	} else {
		y = char_row;
		x = char_col;
		if (get_panel(y, x, true)) {
			prt_map();
		}
		cy = panel_row;
		cx = panel_col;

		for (;;) {
			p_y = panel_row;
			p_x = panel_col;
			if (p_y == cy && p_x == cx) {
				tmp_str[0] = '\0';
			} else {
				sprintf(tmp_str, "%s%s of",
					p_y < cy ? " North"
						 : p_y > cy ? " South" : "",
					p_x < cx ? " West" : p_x > cx ? " East"
								      : "");
			}
			sprintf(out_val, "Map sector [%d,%d], which is%s your "
					 "sector. Look which direction?",
				p_y + 1, p_x + 1, tmp_str);
			if (!sl__get_dir(out_val, &dir_val)) {
				break;
			}
			/** -CJS-
			  * Should really use the move function, but what the
			  * hell. This is nicer, as it moves exactly to the
			  * same place in another section. The direction
			  * calculation is not  intuitive. Sorry.
			  **/
			for (;;) {
				x += ((dir_val - 1) % 3 - 1) * SCREEN_WIDTH / 2;
				y -=
				    ((dir_val - 1) / 3 - 1) * SCREEN_HEIGHT / 2;
				if (x < 1 || y < 1 || x >= cur_width ||
				    y >= cur_width) {
					msg_print("You've gone past the end of "
						  "your map.");
					msg_print("");
					x -= ((dir_val - 1) % 3 - 1) *
					     SCREEN_WIDTH / 2;
					y += ((dir_val - 1) / 3 - 1) *
					     SCREEN_HEIGHT / 2;
					break;
				}
				if (get_panel(y, x, true)) {
					prt_map();
					break;
				}
			}
		}
		/* Move to a new panel - but only if really necessary. */
		if (get_panel(char_row, char_col, false)) {
			prt_map();
		}
	}
}
