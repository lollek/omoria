/* save.c */
/* code for saving and loading characters */

#include <unistd.h> /* unlink */
#include <string.h> /* strncpy */

#include "imoria.h"
#include "save.h"

static char filename[82] = "";

static void data_exception()
{
	/* -RAK-
	 * Data Corruption means character is dead, or save file was screwed
	 * with. Keep them guessing as to what is actually wrong
	 */

	clear_from(1);
	prt("Data Corruption Error.", 1, 1);
	prt(" ", 2, 1);

	/* We'll put it in the debug log as well */
	MSG(("Data Corruption Error"));
	exit_game();
}

static void sc__write_identified(FILE *f1, encrypt_state *cf_state,
				 char out_rec[1026])
{
	/*{ Save identified list			}*/
	long i1;

	for (i1 = 1; i1 <= MAX_OBJECTS; i1++) {
		if (object_ident[i1]) {
			out_rec[i1 - 1] = 'T';
		} else {
			out_rec[i1 - 1] = 'F';
		}
	}
	out_rec[MAX_OBJECTS] = 0;
	encrypt_write(f1, cf_state, out_rec);
}

static void sc__write_monsters(FILE *f1, encrypt_state *cf_state, char out_rec[1026])
{
	/*{ Save the Monster List 		}*/
	long i1, tot_monsters;

	for (i1 = muptr, tot_monsters = 0; i1 > 0; i1 = m_list[i1].nptr) {
		tot_monsters++;
	}

	sprintf(out_rec, "%ld", tot_monsters);
	encrypt_write(f1, cf_state, out_rec);

	for (i1 = muptr; i1 > 0; i1 = m_list[i1].nptr) {

		/* with m_list[i1] do; */
		sprintf(out_rec, "%d %d %d %d %d %d %d %d %d", m_list[i1].fy,
			m_list[i1].fx, m_list[i1].mptr, m_list[i1].hp,
			m_list[i1].cspeed, m_list[i1].csleep, m_list[i1].cdis,
			m_list[i1].ml, m_list[i1].confused);
		encrypt_write(f1, cf_state, out_rec);
	}
}

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

static void gc__read_seeds(FILE *f1, encrypt_state *cf_state, char in_rec[1026],
			   boolean *paniced)
{
	/* creation_time, save_count and deaths are in the master file, be sure
	   to fix up char_restore if you move more onto this line */

	unsigned long save_seed = 0;

	read_decrypt(f1, cf_state, in_rec, paniced);
	if (sscanf(in_rec, "%lu %ld %ld %ld", &save_seed,
		   &player_creation_time, &player_save_count,
		   &player_deaths) != 4) {
		*paniced = true;
	}

	/*  strcpy(temp,in_rec+13); */
	/*  player_ssn = temp; */

	/*  set_seed(ENCRYPT_SEED1); */
	/*  coder(temp); */
	/*  temp_id = temp; */

}

static void gc__read_identified(FILE *f1, encrypt_state *cf_state, char in_rec[1026],
				boolean *paniced)
{
	long i1;

	read_decrypt(f1, cf_state, in_rec, paniced);
	for (i1 = 1; i1 <= MAX_OBJECTS; i1++) {
		if (in_rec[i1 - 1] == 'T') {
			identify(&(object_list[i1]));
		} else if (in_rec[i1 - 1] == 'F') {
			object_ident[i1] = false;
		} else {
			*paniced = true;
		}
	}
}

static void gc__read_monsters(FILE *f1, encrypt_state *cf_state, char in_rec[1026],
			      boolean *paniced)
{
	int x1, x2, x3, x4, x5, x6, x7, x8, x9;
	long i1, i2, i3, tot_monsters;

	muptr = 0;
	mlink();

	read_decrypt(f1, cf_state, in_rec, paniced);
	if (sscanf(in_rec, "%ld", &tot_monsters) != 1) {
		*paniced = true;
	}

	i3 = 0;
	for (i1 = 1; i1 <= tot_monsters; i1++) {
		read_decrypt(f1, cf_state, in_rec, paniced);
		popm(&i2);

		/* with m_list[i2] do; */
		if (sscanf(in_rec, "%d %d %d %d %d %d %d %d %d", &x1, &x2, &x3,
			   &x4, &x5, &x6, &x7, &x8, &x9) != 9) {
			*paniced = true;
		}
		m_list[i2].fy = x1;
		m_list[i2].fx = x2;
		m_list[i2].mptr = x3;
		m_list[i2].hp = x4;
		m_list[i2].cspeed = x5;
		m_list[i2].csleep = x6;
		m_list[i2].cdis = x7;
		m_list[i2].ml = x8;
		m_list[i2].confused = x9;

		if (x1 > MAX_HEIGHT) {
			*paniced = true;
		} else if (x2 > MAX_WIDTH) {
			*paniced = true;
		}

		if (!(*paniced)) {
			cave[x1][x2].cptr = i2;
			if (muptr == 0) {
				muptr = i2;
			} else {
				m_list[i3].nptr = i2;
			}
			m_list[i2].nptr = 0;
			i3 = i2;
		}
	}
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

	if (paniced) data_exception();

	LEAVE("get_char", "");
	return false;
}
