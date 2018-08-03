/* create.c */
/* Ever want to create the perfect character? Here is where you can cheat. */

#include "imoria.h"

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
