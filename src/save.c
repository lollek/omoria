/* save.c */
/* code for saving and loading characters */

#include <unistd.h> /* unlink */
#include <string.h> /* strncpy */

#include "imoria.h"
#include "save.h"

boolean save_char(boolean quick)
{
	/* Actual save procedure -RAK- & -JWT- */
	boolean flag = true;

	ENTER(("save_char", "%d", quick));

	/*{ Message to player on what is happening}*/
	if (!player_flags.dead) {
		clear_from(1);
		prt("Saving character...", 1, 1);
		refresh();
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

boolean get_char(boolean prop)
{
	/*{ Restore a saved game				-RAK- & -JWT-
	 * }*/

	boolean paniced = false;

	ENTER(("get_char", "%d", prop));

	prt("Restoring Character...", 1, 1);
	refresh();

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
	clear();

	LEAVE("get_char", "");
	return false;
}
