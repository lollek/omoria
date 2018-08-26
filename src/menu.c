#include "imoria.h"
#include "menu.h"
#include "save.h"

#include <stdio.h>
#include <dirent.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <errno.h>
#include <string.h>

static char **get_saved_games(int current_row)
{
	char **games_list = safe_malloc(sizeof(char *) * 10, "games_list");

	int counter = 0;

	DIR *dir = opendir(SAVE_FILE_PATH);
	if (dir == NULL) {
		prt("Cannot open " SAVE_FILE_PATH, current_row++, 1);
		prt(strerror(errno), current_row++, 1);
	}

	for (;;) {
		int string_size;
		struct dirent *dir_p = readdir(dir);
		if (dir_p == NULL) {
			break;
		}

		if (dir_p->d_name[0] == '.') {
			continue;
		}

		string_size = strlen(dir_p->d_name) + sizeof('\0');
		games_list[counter] = safe_malloc(string_size, "games_list");
		strcpy(games_list[counter], dir_p->d_name);
		counter++;

		/* Currently no support for more than 9 characters */
		if (counter == 9) {
			break;
		}
	}

	games_list[counter] = NULL;
	return games_list;
}

void main_menu()
{
	int current_row = 1;
	char **saved_games = get_saved_games(current_row);
	int counter;

	ENTER(("main_menu", ""));

	prt("Existing characters:", current_row++, 1);
	for (counter = 0; saved_games[counter] != NULL; ++counter) {
		char counter_str[6] = "* 0: ";
		counter_str[2] = '0' + counter;
		prt2(counter_str, saved_games[counter], current_row++, 3);
	}
	current_row++;

	prt("Select character to load it. ", current_row++, 1);
	prt("Press 'N' to create a new character", current_row++, 1);

	for (;;) {
		unsigned char selection;

		if (counter == 0)
			break; /* No characters to load */

		inkey_flush((char *)&selection);
		if (selection == 'N')
			break;

		selection -= '0';
		if (selection <= counter) {
			char file_name[82];
			strncpy(file_name, saved_games[selection], sizeof(file_name));
			save_file_name_set(file_name);
			break;
		}
	}

	for (counter = 0; saved_games[counter] != NULL; ++counter) {
		dispose(saved_games[counter], strlen(saved_games[counter]) + 1,
			"games_list");
	}
	dispose(saved_games, sizeof(char *) * 10, "games_list");
	LEAVE("main_menu", "");
}
