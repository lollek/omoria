/* create.c */
/* Ever want to create the perfect character? Here is where you can cheat. */

#include "imoria.h"

void cc__put_misc3()
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
	xswm = player_flags.swim + 4;
	xrep = 6 + player_rep div 25;
	sprintf(xinfra, "%ld feet", player_flags.see_infra * 10);

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


void cc__print_history()
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
void cc__get_history()
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

void cc__get_ahw()
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
