#include "imoria.h"
#include "save.h"

void C_main_menu();
int main(int argc, char *argv[])
{
	/* SYSPRV stays off except when needed */
	game_state = GS_LOADING;
	init_priv_switch();
	priv_switch(0);

#if DO_DEBUG
	init_debug();
#endif
	ENTER(("main", ""));

	/* Get the directory location of the image */
	get_paths();

	/* Here comes the monsters.... */
	load_monsters();

	/* Check to see if an update is in progress -DMF- */
	if (check_kickout()) {
		writeln("Imoria is locked . . . Try playing conquest.");
		writeln("Who knows *how* long this might take?");
		exit_game();
	}

	/* Some necessary initializations */
	msg_line = 1;
	quart_height = SCREEN_HEIGHT / 4;
	quart_width = SCREEN_WIDTH / 4;
	dun_level = 0;
	inven_temp = safe_malloc(sizeof(treas_rec), "inven_temp");
	inven_temp->data = blank_treasure;
	inven_temp->ok = false;
	inven_temp->insides = 0;
	inven_temp->next = nil;
	inven_temp->is_in = false;
	old_message = nil;
	turn_counter = 100000;

	/* Sort the objects by level */
	sort_objects();

	/* Init monster and treasure levels for allocate */
	init_m_level();
	init_t_level();

	/* Init the store inventories */
	store_init();
	if (COST_ADJ != 1.00) {
		price_adjust();
	}
	if (WEIGHT_ADJ != 1) {
		item_weight_adjust();
	}
	bank_init();

	/* Build the secret wizard and god passwords */
	bpswd();

	/*
	 * Check operating hours
	 * If not wizard then No_Control_Y
	 * Check or create hours.dat, print message
	 */
	intro(argc, argv);

	C_init_curses();
	curses_is_running = true;

	C_main_menu();

	/* Generate a character, or retrieve old one... */
	if (player_name[0] != '\0') {
		/* Retrieve character */
		game_state = GS_IGNORE_CTRL_C;
		generate = get_char(true);
		is_from_file = true;
		save_char();
		change_name();
		magic_init(randes_seed);

	} else {
		char save_file_name[82];
		/* Create character */
		is_from_file = false;
		create_character();
		player_flags.light_on = false;
		strcpy(save_file_name, SAVE_FILE_PATH "/");
		strcat(save_file_name, player_name);
		player_uid = C_master_add_character();

		learn_magic(false);
		player_cmana = player_mana;
		randes_seed = seed; /* Description seed */
		town_seed = seed;   /* Town generation seed */
		magic_init(randes_seed);
		generate = true;

	} /* end if (load/create character) */

	if (player_pclass == C_MONK) {
		strcpy(bare_hands, "2d2");
	}

	/* Begin the game */
	replace_name();
	set_gem_values();

	player_max_exp =
	    (long)(exp_per_level[MAX_PLAYER_LEVEL - 1] * player_expfact);
	clear_from(1);
	prt_stat_block();

	/* Turn on message trapping, if requested */
	/*    if (want_trap) set_the_trap(); */

	/* Loop till dead, or exit */
	MSG(("Entering main loop"));
	while (true) {
		if (generate) { /* New level */
			generate_cave();
		}

		/* Dungeon logic */
		dungeon();
		generate = true;

		/* Character gets buried, or respawns */
		if (death) {
			upon_death();
			if (death) {
				LEAVE("main", "");
#if DO_DEBUG
				memory_error(0, "DEBUG_ON_EXIT");
#endif
				return 0;
			}
		}
	}
}
