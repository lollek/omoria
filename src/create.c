/* create.c */
/* Ever want to create the perfect character? Here is where you can cheat. */

#include "imoria.h"

int64_t cc__next_best_stats(stat_s_type this, stat_s_type user, stat_s_type best, int64_t best_min);
uint8_t cc__max_de_statp(uint8_t stat);
uint8_t cc__max_in_statp(uint8_t stat);
uint8_t cc__max_stat(uint8_t cur_stat, int8_t amount);
uint8_t cc__new_stat(uint8_t old_guy);
uint8_t cc__old_stat(uint8_t stat);

static uint8_t cc__get_min_stat(char const *prompt, uint8_t max)
{
	uint32_t abil = 0;
	uint32_t perc = 0;
	char tmp_str[134] = "";
	char out_str[134] = "";

	if (max == 250) {
		sprintf(out_str, "Min %s (racial max 18/00) : ", prompt);
	} else if (max > 150) {
		sprintf(out_str, "Min %s (racial max 18/%d) : ", prompt,
			max - 150);
	} else {
		sprintf(out_str, "Min %s (racial max %d) : ", prompt,
			cc__old_stat(max));
	}

	prt(out_str, 1, 1);

	while (tmp_str[0] == '\0')
		get_string(tmp_str, 1, strlen(out_str) + 1, 10);
	if (pindex(tmp_str, '/') > 0) {
		/* player is asking for an 18/xx, 17/00 is left a 0 here, bummer
		 */

		*strchr(tmp_str, '/') = ' ';
		sscanf(tmp_str, "%u %u", &abil, &perc);
		if (abil == 18) {
			if (perc == 0) {
				abil = 250;
			} else {
				abil = 150 + perc;
			}
		}
	} else {
		/* player is asking for something less than 18/xx */

		sscanf(tmp_str, "%u", &abil);
		if (abil < 3) {
			abil = 3;
		} else if (abil > 18) {
			abil = 18;
		}

		abil = cc__new_stat(abil);
	}

	return abil;
}

static void cc__get_minimums(stat_s_type user, boolean *minning, stat_s_type max_r)
{
	/*	{ Get minimum stats the character wants			-DMF-
	 * } */
	if (get_yes_no("Do you wish to try for minimum statistics?")) {
		*minning = true;
		user[STR] = cc__get_min_stat("STR", max_r[STR]);
		user[INT] = cc__get_min_stat("INT", max_r[INT]);
		user[WIS] = cc__get_min_stat("WIS", max_r[WIS]);
		user[DEX] = cc__get_min_stat("DEX", max_r[DEX]);
		user[CON] = cc__get_min_stat("CON", max_r[CON]);
		user[CHR] = cc__get_min_stat("CHR", max_r[CHR]);
		prt_6_stats(user, NULL, 3, 65);
	}
}

static long cc__get_stat()
{
	/*	{ Generates character's stats				-JWT-
	 * } */
	long i;

	i = randint(4) + randint(4) + randint(4) + 5; /* 8..17 */
	return (i - 3) * 10;			      /* 50..140 */
}

static long cc__change_stat(long cur_stat, long amount)
{
	/*	{ Changes stats by given amount				-JWT-
	 * }*/
	long i;

	if (amount < 0) {
		for (i = -1; i >= amount; i--) {
			cur_stat -= squish_stat(de_statp(cur_stat));
		}
	} else {
		for (i = 1; i <= amount; i++) {
			cur_stat += squish_stat(in_statp(cur_stat));
		}
	}

	return cur_stat;
}




static boolean cc__choose_race()
{
	/*	{ Allows player to select a race			-JWT-
	 * }*/

	long i2, i3, i4, i5;
	char s;
	boolean exit_flag;
	char out_val[82];
	boolean return_value = false;

	/*	printf("enter choose_race \n"); fflush(stdout); */

	i2 = 0;
	i3 = 1;
	i4 = 3;
	i5 = 22;
	clear_from(21);
	prt("Choose a race (? for Help):", 21, 3);
	do {
		sprintf(out_val, " %c) %s", (int)i3 + 96, race_name(i2));
		put_buffer(out_val, i5, i4);
		i3++;
		i4 += 15;
		if (i4 > 70) {
			i4 = 3;
			i5++;
		}
		i2++;
	} while (i2 < MAX_RACES);

	/*  printf("    choose_race done with do\n"); fflush(stdout); */

	player_race[0] = 0;
	put_buffer("", 21, 30);
	exit_flag = false;

	do {
		move(3, 14);
		inkey_flush(&s);
		i2 = pindex("abcdefghijklmnopqrstuvwxyz", s);
		if ((i2 <= MAX_RACES) && (i2 >= 1)) {
			player_prace = i2 - 1;
			strcpy(player_race, race_name(i2 - 1));
			exit_flag = true;
			return_value = true;
			put_buffer(player_race, 4, 15);
		} else if (s == '?') {
			moria_help("Character Races");
			exit_flag = true;
			return_value = false;
		}
	} while (!exit_flag);

	/*	printf("exit choose_race \n"); fflush(stdout); */

	return return_value;
}

static void cc__print_try_count(int try_count)
{
	char out_str[134];
	sprintf(out_str, "Try = %10d", try_count);
	put_buffer(out_str, 21, 60);
	put_qio();
}


static boolean cc__satisfied(boolean *minning, boolean *printed_once, long *best_min,
		      long *try_count, stat_s_type best, stat_s_type user)
{
	/*	{ What does it take to satisfy the guy?!		-KRC-
	 * }*/

	char s;
	stat_set tstat;
	boolean return_value = false;

	if (!*minning) {
		/*
		 * Figure out what the current bonuses are
		 * so the player has a clue
		 */
		player_dis_th = tohit_adj();
		player_dis_td = todam_adj();
		player_dis_tac = toac_adj();

		if (!*printed_once) {
			erase_line(1, 1);
			clear_from(21);
			put_misc1();
			put_stats();

			prt("Press 'R' to reroll, <RETURN> to continue:", 21,
			    3);
			*printed_once = true;
		} else {
			upd_misc1();
			upd_stats();
			prt("", 21, 51);
		} /* endif printed_once */

		while (s != 10 && s != 'R') {
			inkey_flush(&s);
			return_value = (s != 'R');
		}

	} else { /* minning */

		if (!*printed_once) {
			clear_from(21);
			prt("Press any key to give up (50000 rolls max): ", 21,
			    3);
			put_qio();
			*printed_once = true;
		}

		*best_min =
		    cc__next_best_stats(player_stats_perm, user, best, *best_min);
		(*try_count)++;
		if ((*try_count % 250) == 0) {
			cc__print_try_count(*try_count);
		}

		inkey_delay(&s, 0);
		if ((s != 0) || (*try_count == 50000)) {
			*minning = false;
			*printed_once = false;
			for (tstat = STR; tstat <= CHR; tstat++) {
				player_stats_perm[(int)tstat] = best[(int)tstat];
				player_stats_curr[(int)tstat] = best[(int)tstat];
			}
			return_value =
			    cc__satisfied(minning, printed_once, best_min,
					  try_count, best, user);
		} else {
			return_value = (*best_min == 0);
			if (*best_min == 0) {
				put_misc1();
				put_stats();
			}
		} /* endif s || try_count */
	}	 /* endif minning */

	return return_value;
}

static void cc__get_stats()
{
	/*	{ Get the statistics for this bozo			-KRC-
	 * }*/

	const int32_t prace = player_prace;
	int tstat;

	for (tstat = STR; tstat <= CHR; tstat++) {
		player_stats_perm[tstat] = cc__change_stat(
		    cc__get_stat(), race_stats(prace)[tstat]);
		player_stats_curr[tstat] = player_stats_perm[tstat];
	}

	player_rep = 0;
	player_srh = race_search_mod(prace);
	player_bth = race_melee_bonus(prace);
	player_bthb = race_ranged_bonus(prace);
	player_fos = race_search_freq(prace);
	player_stl = race_stealth_mod(prace);
	player_save = race_save_mod(prace);
	player_hitdie = race_health_bonus(prace);
	player_lev = 1;
	player_ptodam = todam_adj();
	player_ptohit = tohit_adj();
	player_ptoac = 0;
	player_pac = toac_adj();
	player_expfact = race_expfactor(prace);
	py.flags.see_infra = race_infravision(prace);
	py.flags.swim = race_swim_speed(prace);
}

static void cc__print_history()
{
	/*	{ Will print the history of a character			-JWT-
	 * }*/

	long i1;

	clear_from(14);
	put_buffer("Character Background", 14, 28);
	for (i1 = 0; i1 < 5; i1++) {
		put_buffer(player_history[i1], i1 + 1 + 14, 5);
	}
}

/*
	{ Get the racial history, determines social class	-RAK-	}
	{ Assumtions:	Each race has init history beginning at 	}
	{		(race)*3 + 1					}
	{		All history parts are in ascending order	}
*/
static void cc__get_history()
{

	long hist_ptr, cur_ptr, test_roll;
	long start_pos, end_pos, cur_len;
	long line_ctr, new_start = 0, social_class;
	char history_block[400];
	boolean flag;

	/*	{ Get a block of history text				}*/
	hist_ptr = (player_prace) * 3 + 1;
	history_block[0] = 0;
	social_class = randint(4);
	cur_ptr = -1;

	do {
		flag = false;
		do {
			/* find the chart to use */
			cur_ptr++;
			if (background[cur_ptr].chart == hist_ptr) {
				/* found it, now pick an entry */
				test_roll = randint(100);
				while (test_roll > background[cur_ptr].roll) {
					cur_ptr++;
				}

				strcat(history_block, background[cur_ptr].info);
				social_class += background[cur_ptr].bonus;

				if (hist_ptr > background[cur_ptr].next) {
					/* reset cur_ptr since next chart has
					 * alerady been passed */
					hist_ptr = background[cur_ptr].next;
					cur_ptr = -1;
				} else {
					hist_ptr = background[cur_ptr].next;
				}

				flag = true;
			} /* endif chart == hist_ptr */
		} while (!flag);
	} while (hist_ptr >= 1);

	/* -------------------------------------------------- */

	/*	{ Process block of history text for pretty output	} */

	start_pos = 0;
	end_pos = strlen(history_block) - 1;
	line_ctr = 0;
	flag = false;

	while (history_block[end_pos] == ' ') {
		end_pos--;
	}
	do {
		while (history_block[start_pos] == ' ') {
			start_pos++;
		}
		cur_len = end_pos - start_pos + 1;
		if (cur_len > 70) {
			cur_len = 70;
			while (history_block[start_pos + cur_len - 1] != ' ') {
				cur_len--;
			}
			new_start = start_pos + cur_len;
			while (history_block[start_pos + cur_len - 1] == ' ') {
				cur_len--;
			}
		} else {
			flag = true;
		}
		strncpy(player_history[line_ctr], &(history_block[start_pos]),
			cur_len);
		player_history[line_ctr][cur_len] = 0;
		line_ctr++;
		start_pos = new_start;
	} while (!flag);

	for (; line_ctr < 5; line_ctr++) {
		player_history[line_ctr][0] = 0;
	}

	/*	{ Compute social class for player			}*/
	if (social_class > 100) {
		social_class = 100;
	} else if (social_class < 1) {
		social_class = 1;
	}

	player_rep = 50 - social_class;
	player_sc = social_class;

} /* end cc__get_history */

static boolean cc__get_sex()
{
	/*	{ Gets the character's sex				-JWT-
	 * }*/

	char s;
	boolean exit_flag = false;
	boolean return_value = false;

	if (player_prace == R_DRYAD) {
		strcpy(player_sex, "Female");
		return_value = true;
		exit_flag = true;
		prt(player_sex, 5, 15);
	} else {
		player_sex[0] = 0;
		clear_from(21);
		prt("Choose a sex (? for Help):", 21, 3);
		prt("m) Male       f) Female", 22, 3);
		prt("", 21, 29);
		do {
			move(4, 14);
			inkey_flush(&s);
			if (s == 'f') {
				strcpy(player_sex, "Female");
				prt(player_sex, 5, 15);
				exit_flag = true;
				return_value = true;
			} else if (s == 'm') {
				strcpy(player_sex, "Male");
				prt(player_sex, 5, 15);
				exit_flag = true;
				return_value = true;
			} else if (s == '?') {
				moria_help("Character Sex");
				exit_flag = true;
				return_value = false;
			}
		} while (!exit_flag);
	} /* endif prace */

	return return_value;
} /* end cc__get_sex */

static void cc__get_ahw()
{
	/*	{ Computes character's age, height, and weight		-JWT-
	 * }*/

	long i1;

	i1 = player_prace;
	player_age = race_rand_starting_age(i1);

	player_birth.year = 500 + randint(50);
	player_birth.month = randint(13);
	player_birth.day = randint(28);
	player_birth.hour = randint(24) - 1;
	player_birth.secs = randint(400) - 1;

	player_cur_age.year = player_age + player_birth.year;
	player_cur_age.month = player_birth.month;
	player_cur_age.day = player_birth.day + 1;
	if ((player_cur_age.day % 7) == 0) {
		add_days(&player_cur_age, 2);
	}
	if ((player_cur_age.day % 7) == 1) {
		add_days(&player_cur_age, 1);
	}
	player_cur_age.hour = 7;
	player_cur_age.secs = 300 + randint(99);
	player_ht = race_rand_starting_height(i1, characters_sex() == MALE);
	player_wt = race_rand_starting_weight(i1, characters_sex() == MALE);
	player_disarm = race_disarm_mod(i1) + todis_adj();

} /* end cc__get_ahw */

/*	{ Gets a character class				-JWT-	}*/
static boolean cc__get_class()
{
	long i1, i2, i3, i4, i5;
	long cl[MAX_CLASS + 1];
	int32_t aclass;
	char s;
	boolean exit_flag;
	char out_str[134];
	stat_set tstat;
	boolean return_value = false;

	for (i2 = 0; i2 < MAX_CLASS; i2++) {
		cl[i2] = 0;
	}

	i1 = player_prace;
	i2 = 0;
	i3 = 0;
	i4 = 3;
	i5 = 22;
	clear_from(21);
	prt("Choose a class (? for Help):", 21, 3);

	do {
		if (race_class_field(i1) & bit_array[i2 + 1]) {
			i3++;
			sprintf(out_str, "%c) %s", (int)i3 + 96,
				class_title(i2));
			put_buffer(out_str, i5, i4);
			cl[i3] = i2;
			i4 += 15;
			if (i4 > 70) {
				i4 = 3;
				i5++;
			}
		} /* endif class bit set */
		i2++;
	} while (i2 < MAX_CLASS);

	player_pclass = 0;
	put_buffer("", 21, 31);
	exit_flag = false;

	do {
		move(5, 14);
		inkey_flush(&s);
		i2 = pindex("abcdefghijklmnopqrstuvwxyz", s);
		if ((i2 <= i3) && (i2 >= 1)) {
			strcpy(player_tclass, class_title(cl[i2]));
			player_pclass = cl[i2];
			aclass = player_pclass;
			exit_flag = true;
			return_value = true;
			clear_from(21);
			put_buffer(player_tclass, 6, 15);
			player_hitdie += class_extra_health(aclass);
			player_mhp = con_adj() + player_hitdie;
			player_chp = player_mhp;
			player_bth += class_melee_bonus(aclass) * 5 + 20;
			player_bthb += class_ranged_bonus(aclass) * 5 + 20;
			player_srh += class_search_mod(aclass);
			player_disarm += class_disarm_mod(aclass);
			player_fos += class_search_freq(aclass);
			player_stl += class_stealth_mod(aclass);
			player_save += class_save_mod(aclass);
			player_expfact += class_expfactor(aclass);
			strcpy(player_title, player_titles[aclass][0]);
			player_mr = class_magic_resist(aclass);

			/* { Adjust the stats for the class adjustment
			 * -RAK-	}*/

			for (tstat = STR; tstat <= CHR; tstat++) {
				player_stats_perm[(int)tstat] = cc__change_stat(
				    player_stats_perm[(int)tstat],
				    class_stats(aclass)[(int)tstat]);
				player_stats_curr[(int)tstat] = player_stats_perm[(int)tstat];
			}

			player_ptodam = todam_adj(); /*{ Real values	}*/
			player_ptohit = tohit_adj();
			player_ptoac = toac_adj();
			player_pac = 0;
			player_dis_td =
			    player_ptodam; /*{ Displayed values	}*/
			player_dis_th = player_ptohit;
			player_dis_tac = player_ptoac;
			player_dis_ac = player_pac;

		} else if (s == '?') {
			moria_help("Character Classes");
			exit_flag = true;
			return_value = false;
		}
	} while (!exit_flag);

	return return_value;
} /* end cc__get_class */

static void cc__get_money()
{

	long tmp, i1;
	stat_set tstat;

	tmp = 0;
	for (tstat = STR; tstat <= CHR; tstat++) {
		tmp += cc__old_stat(player_stats_curr[(int)tstat]);
	}

	i1 = player_sc * 6 + randint(25) + 325; /*{ Social Class adj	} */
	i1 -= tmp;				 /*{ Stat adj		} */
	i1 += cc__old_stat(player_stats_curr[CHR]);      /*{ Charisma adj	} */
	if (i1 < 80) {
		i1 = 80;
	} /*{ Minimum		} */
	i1 = (i1 * GOLD_VALUE) + randint(GOLD_VALUE);
	add_money(i1);
}

void create_character()
{
	stat_s_type best, user, max_r;
	boolean minning = false;
	boolean printed_once = false;
	long try_count = 0;
	long best_min = 99999999;
	stat_set tstat;

	/*
	 * This delay may be reduced, but is recomended to keep players
	 *
	 * from continuously rolling up characters, which can be VERY
	 * expensive CPU wise.
	*/

	ENTER(("create_character", ""));

	for (tstat = STR; tstat <= CHR; tstat++) {
		best[(int)tstat] = 3;
	}

	do {
		put_character();
	} while (!cc__choose_race());

	while (!cc__get_sex()) {
		put_character();
	}

	printed_once = false;
	for (tstat = STR; tstat <= CHR; tstat++) {
		max_r[(int)tstat] =
		    cc__max_stat(140, race_stats(player_prace)[(int)tstat]);
	}

	cc__get_minimums(user, &minning, max_r);
	do {
		cc__get_stats();
		cc__get_history();
		cc__get_ahw();
	} while (!cc__satisfied(&minning, &printed_once, &best_min, &try_count,
				best, user));

	cc__print_history();
	while (!cc__get_class()) {
		put_character();
		cc__print_history();
		put_misc1();
		put_stats();
	}

	player_creation_time = time(NULL);
	player_save_count = 0;
	player_claim_check = 0;
	py.flags.light_on = false;

	cc__get_money();
	put_stats();
	put_misc2();
	put_misc3();
	get_name();
	/*	get_ssn(); */
	pause_exit(24, PLAYER_EXIT_PAUSE);

	LEAVE("create_character", "");
}

void put_character()
{
	/*{ Prints the following information on the screen.	-JWT-	}*/

	clear_from(1);
	prt2("Name      : ", player_name, 3, 3);
	prt2("Race      : ", player_race, 4, 3);
	prt2("Sex       : ", player_sex, 5, 3);
	prt2("Class     : ", player_tclass, 6, 3);
}

void put_stats()
{

	/*	{ Prints the following information on the screen.	-JWT-
	 * }*/

	prt_6_stats(player_stats_curr, NULL, 3, 65);
	prt_num("+ To Hit   : ", player_dis_th, 10, 4);
	prt_num("+ To Damage: ", player_dis_td, 11, 4);
	prt_num("+ To AC    : ", player_dis_tac, 12, 4);
	prt_num("  Total AC : ", player_dis_ac, 13, 4);
}

void upd_stats()
{

	/*	{ Updates the following information on the screen. (wow)-KRC-
	 * }*/
	stat_set tstat;

	for (tstat = STR; tstat <= CHR; tstat++) {
		prt_stat("", player_stats_curr[(int)tstat], 3 + tstat, 71);
	}
	prt_num("", player_dis_th, 10, 17);
	prt_num("", player_dis_td, 11, 17);
	prt_num("", player_dis_tac, 12, 17);
	prt_num("", player_dis_ac, 13, 17);
}

void put_misc1()
{
	/*	{ Prints age, height, weight, and SC			-JWT-
	 * }*/

	prt_num("Age          : ", player_age, 3, 40);
	prt_num("Height       : ", player_ht, 4, 40);
	prt_num("Weight       : ", player_wt, 5, 40);
	prt_num("Social Class : ", player_sc, 6, 40);
	prt_num("Difficulty   : ", player_diffic, 7, 40);
}

void put_misc2()
{

	/*	{ Prints the following information on the screen.	-JWT-
	 * }*/

	prt_num("Level      : ", player_lev, 10, 31);
	prt_num("Experience : ", player_exp, 11, 31);
	prt_num("Gold       : ", player_money[TOTAL_], 12, 31);
	prt_num("Account    : ", player_account, 13, 31);
	prt_num("Max Hit Points : ", player_mhp, 10, 54);
	prt_num("Cur Hit Points : ", (long)(player_chp), 11, 54);
	prt_num("Max Mana       : ", player_mana, 12, 54);
	prt_num("Cur Mana       : ", (long)(player_cmana), 13, 54);
}

void put_misc3()
{

	/*	{ Prints ratings on certain abilities			-RAK-
	 * }*/
	long xbth, xbthb, xfos, xsrh, xstl, xdis;
	long xsave, xdev, xswm, xrep;
	char xinfra[82];
	char tmp_str[82], tmp2[82];

	clear_from(14);

	xbth = player_bth + player_lev * BTH_LEV_ADJ +
	       player_ptohit * BTH_PLUS_ADJ;
	xbthb = player_bthb + player_lev * BTH_LEV_ADJ +
		player_ptohit * BTH_PLUS_ADJ;
	xfos = 27 - player_fos;
	if (xfos < 0) {
		xfos = 0;
	}
	xsrh = player_srh + spell_adj(INT);
	xstl = player_stl;
	xdis = player_disarm + player_lev + 2 * todis_adj() + spell_adj(INT);
	xsave = player_save + player_lev + spell_adj(WIS);
	xdev = player_save + player_lev + spell_adj(INT);
	xswm = py.flags.swim + 4;
	xrep = 6 + player_rep div 25;
	sprintf(xinfra, "%ld feet", py.flags.see_infra * 10);

	prt("(Miscellaneous Abilities)", 16, 24);
	sprintf(tmp2, "%s%s", "Fighting    : ", likert(xbth, 12, tmp_str));
	put_buffer(tmp2, 17, 2);
	sprintf(tmp2, "%s%s", "Bows/Throw  : ", likert(xbthb, 12, tmp_str));
	put_buffer(tmp2, 18, 2);
	sprintf(tmp2, "%s%s", "Saving Throw: ", likert(xsave, 6, tmp_str));
	put_buffer(tmp2, 19, 2);
	sprintf(tmp2, "%s%s", "Stealth     : ", likert(xstl, 1, tmp_str));
	put_buffer(tmp2, 17, 27);
	sprintf(tmp2, "%s%s", "Disarming   : ", likert(xdis, 8, tmp_str));
	put_buffer(tmp2, 18, 27);
	sprintf(tmp2, "%s%s", "Magic Device: ", likert(xdev, 7, tmp_str));
	put_buffer(tmp2, 19, 27);
	sprintf(tmp2, "%s%s", "Perception  : ", likert(xfos, 3, tmp_str));
	put_buffer(tmp2, 17, 52);
	sprintf(tmp2, "%s%s", "Searching   : ", likert(xsrh, 6, tmp_str));
	put_buffer(tmp2, 18, 52);
	sprintf(tmp2, "%s%s", "Infra-Vision: ", xinfra);
	put_buffer(tmp2, 19, 52);
	sprintf(tmp2, "%s%s", "Swimming    : ", likert(xswm, 1, tmp_str));
	put_buffer(tmp2, 20, 52);
	sprintf(tmp2, "%s%s", "Reputation  : ", likert(xrep, 1, tmp_str));
	put_buffer(tmp2, 20, 2);
}

void upd_misc1()
{
	/*{ Updates age, height, weight, and SC (amazing, huh?)	-KRC-	}*/

	prt_num("", player_age, 3, 55);
	prt_num("", player_ht, 4, 55);
	prt_num("", player_wt, 5, 55);
	prt_num("", player_sc, 6, 55);
}

void display_char()
{

	/*	{ Used to display the character on the screen.		-RAK-
	 * }*/

	put_character();
	put_misc1();
	put_stats();
	put_misc2();
	put_misc3();
}

void get_name()
{
	/* Gets a name for the character    -JWT- */

	prt("Enter your player's name  [press <RETURN> when finished]", 22, 3);
	player_name[0] = '\0';
	while (player_name[0] == '\0')
		get_string(player_name, 3, 15, 24);
	clear_from(21);
}

void change_name()
{

	/*	{ Chances the name of the character			-JWT-
	 * } */

	char c;
	boolean flag = false;

	display_char();
	do {
		prt("<c>hange character name.     <ESCAPE> to continue.", 22,
		    3);

		c = inkey();

		switch (c) {
		case 99:
			get_name();
			break;
		case 0:
		case 3:
		case 25:
		case 26:
		case 27:
			flag = true;
			break;
		}

	} while (!flag);
}

void set_gem_values()
{
	long count;

	ENTER(("set_gem_values", ""));

	for (count = 1; count <= MAX_OBJECTS; count++) {
		/*with object_list[count] do*/
		if ((strstr(object_list[count].name, "Finely cut") != NULL) &&
		    (strstr(object_list[count].name, "of") != NULL)) {

			if (strstr(object_list[count].name, "Amber") != NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Agate") != NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Alexandrite") !=
			    NULL) {
				object_list[count].cost += 5000;
			}
			/* amethyst was misspelled as "amathyst".  2/15/00 JEB
			 */
			if (strstr(object_list[count].name, "Amethyst") !=
			    NULL) {
				object_list[count].cost += 2000;
			}
			if (strstr(object_list[count].name, "Antlerite") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Aquamarine") !=
			    NULL) {
				object_list[count].cost += 6000;
			}
			if (strstr(object_list[count].name, "Argentite") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Azurite") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Beryl") != NULL) {
				object_list[count].cost += 2000;
			}
			if (strstr(object_list[count].name, "Bloodstone") !=
			    NULL) {
				object_list[count].cost += 3500;
			}
			if (strstr(object_list[count].name, "Calcite") !=
			    NULL) {
				object_list[count].cost += 1500;
			}
			if (strstr(object_list[count].name, "Carnelian") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Coral") != NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Corundum") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Cryolite") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Diamond") !=
			    NULL) {
				object_list[count].cost += 35000;
			}
			if (strstr(object_list[count].name, "Diorite") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Emerald") !=
			    NULL) {
				object_list[count].cost += 20000;
			}
			if (strstr(object_list[count].name, "Flint") != NULL) {
				object_list[count].cost += 5000;
			}
			if (strstr(object_list[count].name, "Fluorite") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Gabbro") != NULL) {
				object_list[count].cost += 5000;
			}
			if (strstr(object_list[count].name, "Garnet") != NULL) {
				object_list[count].cost += 6500;
			}
			if (strstr(object_list[count].name, "Granite") !=
			    NULL) {
				object_list[count].cost += 500;
			}
			if (strstr(object_list[count].name, "Gypsum") != NULL) {
				object_list[count].cost += 3000;
			}
			if (strstr(object_list[count].name, "Hematite") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Jade") != NULL) {
				object_list[count].cost += 12000;
			}
			if (strstr(object_list[count].name, "Jasper") != NULL) {
				object_list[count].cost += 3000;
			}
			if (strstr(object_list[count].name, "Kryptonite") !=
			    NULL) {
				object_list[count].cost += 5000;
			}
			if (strstr(object_list[count].name, "Lapus lazuli") !=
			    NULL) {
				object_list[count].cost += 4500;
			}
			if (strstr(object_list[count].name, "Limestone") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Malachite") !=
			    NULL) {
				object_list[count].cost += 3000;
			}
			if (strstr(object_list[count].name, "Manganite") !=
			    NULL) {
				object_list[count].cost += 5000;
			}
			if (strstr(object_list[count].name, "Marble") != NULL) {
				object_list[count].cost += 5500;
			}
			if (strstr(object_list[count].name, "Mica") != NULL) {
				object_list[count].cost += 1500;
			}
			if (strstr(object_list[count].name, "Moonstone") !=
			    NULL) {
				object_list[count].cost += 3000;
			}
			if (strstr(object_list[count].name, "Neptunite") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Obsidian") !=
			    NULL) {
				object_list[count].cost += 2500;
			}
			if (strstr(object_list[count].name, "Onyx") != NULL) {
				object_list[count].cost += 1500;
			}
			if (strstr(object_list[count].name, "Opal") != NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Pearl") != NULL) {
				object_list[count].cost += 11500;
			}
			if (strstr(object_list[count].name, "Pyrite") != NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Quartz") != NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Quartzite") !=
			    NULL) {
				object_list[count].cost += 1500;
			}
			if (strstr(object_list[count].name, "Rhodonite") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Rhyolite") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Ruby") != NULL) {
				object_list[count].cost += 14500;
			}
			if (strstr(object_list[count].name, "Sapphire") !=
			    NULL) {
				object_list[count].cost += 14500;
			}
			if (strstr(object_list[count].name, "Sphalerite") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Staurolite") !=
			    NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Tiger eye") !=
			    NULL) {
				object_list[count].cost += 8500;
			}
			if (strstr(object_list[count].name, "Topaz") != NULL) {
				object_list[count].cost += 1000;
			}
			if (strstr(object_list[count].name, "Turquoise") !=
			    NULL) {
				object_list[count].cost += 3000;
			}
			if (strstr(object_list[count].name, "Zircon") != NULL) {
				object_list[count].cost += 1000;
			}
		} /* end if (finely cut) */
	}	 /* end for */

	LEAVE("set_gem_values", "");
}
