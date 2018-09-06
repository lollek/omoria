/* Helper functions to call from ncurses.rs */

#include <ncurses.h>

#define STANDOUT 1
#define UNDERLINE 2
#define REVERSE 3
#define BLINK 4
#define DIM 5
#define BOLD 6

#define GREEN 10
#define YELLOW 11
#define RED 12
#define BLUE 13
#define CYAN 14
#define MAGENTA 15

int C_chattr(int attr, uint32_t on) {
	switch(attr) {
		case STANDOUT:
			if (on) return attron(A_STANDOUT);
			else	return attroff(A_STANDOUT);
		case UNDERLINE:
			if (on) return attron(A_UNDERLINE);
			else	return attroff(A_UNDERLINE);
		case REVERSE:
			if (on) return attron(A_REVERSE);
			else	return attroff(A_REVERSE);
		case BLINK:
			if (on) return attron(A_BLINK);
			else	return attroff(A_BLINK);
		case DIM:
			if (on) return attron(A_DIM);
			else	return attroff(A_DIM);
		case BOLD:
			if (on) return attron(A_BOLD);
			else	return attroff(A_BOLD);
		case GREEN:
			if (on) return attron(COLOR_PAIR(COLOR_GREEN));
			else	return attroff(COLOR_PAIR(COLOR_GREEN));
		case YELLOW:
			if (on) return attron(COLOR_PAIR(COLOR_YELLOW));
			else	return attroff(COLOR_PAIR(COLOR_YELLOW));
		case RED:
			if (on) return attron(COLOR_PAIR(COLOR_RED));
			else	return attroff(COLOR_PAIR(COLOR_RED));
		case BLUE:
			if (on) return attron(COLOR_PAIR(COLOR_BLUE));
			else	return attroff(COLOR_PAIR(COLOR_BLUE));
		case CYAN:
			if (on) return attron(COLOR_PAIR(COLOR_CYAN));
			else	return attroff(COLOR_PAIR(COLOR_CYAN));
		case MAGENTA:
			if (on) return attron(COLOR_PAIR(COLOR_MAGENTA));
			else	return attroff(COLOR_PAIR(COLOR_MAGENTA));
	}
	return ERR;
}
