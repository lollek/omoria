/* Helper functions to call from ncurses.rs */

#include <ncurses.h>

#define DIM 1

#define GREEN 10
#define YELLOW 11
#define RED 12
#define BLUE 13
#define CYAN 14
#define MAGENTA 15

int chattr(int attr, uint32_t on) {
	switch(attr) {
		case DIM:
			if (on) return attron(A_DIM);
			else	return attroff(A_DIM);
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

int attron_dim() {
	return attron(A_DIM);
}

int attroff_dim() {
	return attroff(A_DIM);
}
