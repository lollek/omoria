/* save.c */
/* code for saving and loading characters */

#include <unistd.h> /* unlink */
#include <string.h> /* strncpy */

#include "imoria.h"
#include "save.h"

static char filename[82] = "";

void C_delete_character(void);
void save_file_remove(void)
{
	C_delete_character();
}

void save_file_name_set(char path[82]) { strncpy(filename, path, sizeof(char[82])); }

boolean save_file_name_is_set(void) { return filename[0] != '\0'; }

boolean save_char(boolean quick)
{
	/* Actual save procedure -RAK- & -JWT- */
	boolean flag = true;

	ENTER(("save_char", "%d", quick));

	/*{ Message to player on what is happening}*/
	if (!player_flags.dead) {
		clear_from(1);
		prt("Saving character...", 1, 1);
		put_qio();
	}

	if (flag) flag = C_master_update_character(player_uid);
	if (flag) flag = C_save_character();

	if (flag && !player_flags.dead) {
		char out_rec[1026];
		sprintf(out_rec, "Character saved. [Moria Version %s]\n",
			omoria_version());
		prt(out_rec, 2, 1);
		exit_game();
	}

	LEAVE("save_char", "");
	return flag;
}

boolean parse_filename() {
	const char *pname;
	const char *puid;
	char *ptr;
	
	if (player_name[0] != '\0')
		return true; /* already parsed */

	ptr = strchr(filename, '-');
	if (ptr == NULL) {
		MSG(("Malformed filename (Error 1)"));
		return false;
	}
	*ptr = '\0';

	pname = filename;
	puid = &ptr[1];

	ptr = strchr(&ptr[1], '.');
	if (ptr == NULL) {
		MSG(("Malformed filename (Error 2)"));
		return false;
	}
	*ptr = '\0';

	strncpy(player_name, pname, sizeof(player_name));
	player_uid = atol(puid);

	return true;
}

boolean get_char(boolean prop)
{
	/*{ Restore a saved game				-RAK- & -JWT-
	 * }*/

	boolean paniced = false;

	ENTER(("get_char", "%d", prop));

	prt("Restoring Character...", 1, 1);
	put_qio();

	if (!paniced) paniced = !parse_filename();

	MSG(("name: %s, uid: %ld", player_name, player_uid));

	if (!paniced && !C_master_character_exists(player_uid)) {
		MSG(("Character does not exist in master!"));
		paniced = true;
	}
	if (!paniced) paniced = !C_load_character();

	if (paniced) {
		clear_from(1);
		prt("Data Corruption Error.", 1, 1);
		prt(" ", 2, 1);

		/* We'll put it in the debug log as well */
		MSG(("Data Corruption Error"));
		exit_game();
	}

	LEAVE("get_char", "");
	return false;
}
