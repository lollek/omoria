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

static void sc__write_inventory(FILE *f1, encrypt_state *cf_state,
				char out_rec[1026])
{
	/*{ Write out the inventory records.	}*/
	treas_ptr curse;

	curse = inventory_list;
	while (curse != NULL) {
		unsigned long const chtype_buf = curse->data.tchar;
		sprintf(out_rec, "%lu %s", chtype_buf, curse->data.name);
		encrypt_write(f1, cf_state, out_rec);

		sprintf(out_rec, "%d %d", (int)curse->is_in,
			(int)curse->insides);
		encrypt_write(f1, cf_state, out_rec);

		sprintf(out_rec, "%s", curse->data.damage);
		encrypt_write(f1, cf_state, out_rec);

		/* with curse->data do; */
		sprintf(out_rec, "%d %d %d %d %d %d %d %d %d %ld %ld %d %ld",
			(int)curse->data.tval, (int)curse->data.subval,
			(int)curse->data.weight, (int)curse->data.number,
			(int)curse->data.tohit, (int)curse->data.todam,
			(int)curse->data.ac, (int)curse->data.toac,
			(int)curse->data.p1, curse->data.flags,
			curse->data.flags2, (int)curse->data.level,
			curse->data.cost);
		encrypt_write(f1, cf_state, out_rec);
		curse = curse->next;

	} /* end while curse */
}

static void sc__write_equipment(FILE *f1, encrypt_state *cf_state,
				char out_rec[1026])
{
	/*{ Write out the equipment records.	}*/

	long i1;
	for (i1 = Equipment_min; i1 < EQUIP_MAX; i1++) {
		unsigned long const chtype_buf = equipment[i1].tchar;
		sprintf(out_rec, "%lu %s", chtype_buf, equipment[i1].name);
		encrypt_write(f1, cf_state, out_rec);

		sprintf(out_rec, "%s", equipment[i1].damage);
		encrypt_write(f1, cf_state, out_rec);

		/* with equipment[i1] do; */
		sprintf(out_rec, "%d %d %d %d %d %d %d %d %d %ld %ld %d %ld",
			(int)equipment[i1].tval, (int)equipment[i1].subval,
			(int)equipment[i1].weight, (int)equipment[i1].number,
			(int)equipment[i1].tohit, (int)equipment[i1].todam,
			(int)equipment[i1].ac, (int)equipment[i1].toac,
			(int)equipment[i1].p1, equipment[i1].flags,
			equipment[i1].flags2, (int)equipment[i1].level,
			equipment[i1].cost);
		encrypt_write(f1, cf_state, out_rec);

	} /* end for i1 */
}

static void sc__write_dungeon(FILE *f1, encrypt_state *cf_state, char out_rec[1026])
{
	/*{ Write the important dungeon info and floor	-RAK-	}*/

	long i1, i2, tot_treasure, tptr, count = 0;
	unsigned long xfloor, prevFloor = 999999;

	sprintf(out_rec, "%ld %ld %ld %ld", cur_height, cur_width,
		max_panel_rows, max_panel_cols);
	encrypt_write(f1, cf_state, out_rec);

	/*{ Save the floor	}*/

	tot_treasure = 0;
	for (i1 = 1; i1 <= cur_height; i1++) {
		/* out_rec = pad(' ',' ',cur_width); */
		for (i2 = 1; i2 <= cur_width; i2++) {
			/* with cave[i1][i2]. do; */
			xfloor = cave[i1][i2].fval;
			if (cave[i1][i2].fopen) {
				xfloor |= 0x20;
			}
			if (cave[i1][i2].pl) {
				xfloor |= 0x40;
			}
			if (cave[i1][i2].fm) {
				xfloor |= 0x80;
			}
			if (cave[i1][i2].tptr > 0) {
				tot_treasure++;
			}

			/* run length encoding the floor cuts about 30k from the
			 * save file */
			if (xfloor == prevFloor) {
				count++;
			} else {
				if (count != 0) {
					if (count == 1) {
						sprintf(out_rec, "%ld",
							prevFloor);
					} else {
						sprintf(out_rec, "%ld %ld",
							prevFloor, count);
					}
					encrypt_write(f1, cf_state, out_rec);
				}
				prevFloor = xfloor;
				count = 1;
			}

		} /* end for i2 */
	}	 /* end for i1 */

	if (count == 1) {
		sprintf(out_rec, "%ld", prevFloor);
	} else {
		sprintf(out_rec, "%ld %ld", prevFloor, count);
	}
	encrypt_write(f1, cf_state, out_rec);

	/*{ Save the Treasure List		}*/
	sprintf(out_rec, "%ld", tot_treasure);
	encrypt_write(f1, cf_state, out_rec);

	for (i1 = 1; i1 <= cur_height; i1++) {
		for (i2 = 1; i2 <= cur_width; i2++) {
			if (cave[i1][i2].tptr > 0) {
				unsigned long chtype_buf;
				tptr = cave[i1][i2].tptr;
				/* with t_list[tptr]. do; */
				sprintf(out_rec, "%ld %ld", i1, i2);
				encrypt_write(f1, cf_state, out_rec);

				chtype_buf = t_list[tptr].tchar;
				sprintf(out_rec, "%lu %s", chtype_buf,
					t_list[tptr].name);
				encrypt_write(f1, cf_state, out_rec);

				sprintf(out_rec, "%s", t_list[tptr].damage);
				encrypt_write(f1, cf_state, out_rec);

				sprintf(out_rec, "%d %ld %d %d %d %d %d %d %ld "
						 "%ld %ld %d %ld",
					t_list[tptr].tval, t_list[tptr].subval,
					t_list[tptr].weight,
					t_list[tptr].number, t_list[tptr].tohit,
					t_list[tptr].todam, t_list[tptr].ac,
					t_list[tptr].toac, t_list[tptr].p1,
					t_list[tptr].flags, t_list[tptr].flags2,
					t_list[tptr].level, t_list[tptr].cost);
				encrypt_write(f1, cf_state, out_rec);
			}
		}
	}
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

static void sc__write_town(FILE *f1, encrypt_state *cf_state, char out_rec[1026])
{
	/*{ Save the town level stores		}*/

	long i1, i2;
	game_time_type st;

	sprintf(out_rec, "%ld", town_seed);
	encrypt_write(f1, cf_state, out_rec);

	sprintf(out_rec, "%ld %ld %ld %ld %ld %ld %ld", bank[0], bank[6],
		bank[5], bank[4], bank[3], bank[2], bank[1]);
	encrypt_write(f1, cf_state, out_rec);

	for (i1 = 0; i1 < MAX_STORES; i1++) {
		/* with stores[i1]. do; */
		/*{ Save items...                 }*/
		sprintf(out_rec, "%d", stores[i1].store_ctr);
		encrypt_write(f1, cf_state, out_rec);

		for (i2 = 1; i2 <= stores[i1].store_ctr; i2++) {
			unsigned long chtype_buf;
			/* with stores[i1].store_inven[i2].sitem do; */
			sprintf(out_rec, "%ld",
				stores[i1].store_inven[i2].scost);
			encrypt_write(f1, cf_state, out_rec);

			chtype_buf = stores[i1].store_inven[i2].sitem.tchar;
			sprintf(out_rec, "%lu %s", chtype_buf,
				stores[i1].store_inven[i2].sitem.name);
			encrypt_write(f1, cf_state, out_rec);

			sprintf(out_rec, "%s",
				stores[i1].store_inven[i2].sitem.damage);
			encrypt_write(f1, cf_state, out_rec);

			sprintf(out_rec,
				"%d %ld %d %d %d %d %d %d %ld %ld %ld %d %ld",
				stores[i1].store_inven[i2].sitem.tval,
				stores[i1].store_inven[i2].sitem.subval,
				stores[i1].store_inven[i2].sitem.weight,
				stores[i1].store_inven[i2].sitem.number,
				stores[i1].store_inven[i2].sitem.tohit,
				stores[i1].store_inven[i2].sitem.todam,
				stores[i1].store_inven[i2].sitem.ac,
				stores[i1].store_inven[i2].sitem.toac,
				stores[i1].store_inven[i2].sitem.p1,
				stores[i1].store_inven[i2].sitem.flags,
				stores[i1].store_inven[i2].sitem.flags2,
				stores[i1].store_inven[i2].sitem.level,
				stores[i1].store_inven[i2].sitem.cost);
			encrypt_write(f1, cf_state, out_rec);
			/* end with store inven; */
		} /* end for i2; */

		/* with stores[i1].store_inven[i2].store_open. do; */
		/* with player_do; */
		st = stores[i1].store_open;
		if ((player_cur_age.year > st.year) ||
		    ((player_cur_age.year == st.year) &&
		     ((player_cur_age.month > st.month) ||
		      ((player_cur_age.month == st.month) &&
		       ((player_cur_age.day > st.day) ||
			((player_cur_age.day == st.day) &&
			 ((player_cur_age.hour > st.hour) ||
			  ((player_cur_age.hour == st.hour) ||
			   ((player_cur_age.secs > st.secs)))))))))) {
			st.year = 0;
			st.month = 0;
			st.day = 0;
			st.hour = 0;
			st.secs = 0;
			stores[i1].store_open = st;
		}

		/* with store_open do; */
		sprintf(out_rec, "%d %d %ld %d %d %d %d", stores[i1].owner,
			stores[i1].insult_cur, st.year, st.month, st.day,
			st.hour, st.secs);
		encrypt_write(f1, cf_state, out_rec);
	} /* end for i1; */
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

static void gc__add_item(treas_ptr *cur_bag)
{
	treas_ptr ptr, curse;

	/* {Extensive clarifications and bug fixes here by Dean}*/
	ptr = (treas_ptr)safe_malloc(sizeof(treas_rec), "gc__add_item");

	if (inventory_list == nil) {
		inventory_list = ptr;
	} else {
		curse = inventory_list;
		while (curse->next != nil) {
			curse = curse->next;
		}
		curse->next = ptr;
	}

	ptr->data = inven_temp->data;
	ptr->is_in = inven_temp->is_in;
	ptr->insides = inven_temp->insides;
	ptr->ok = false;
	ptr->next = nil;

	if (ptr->data.tval == bag_or_sack) {
		*cur_bag = ptr;
	}

	if (ptr->is_in && (*cur_bag != nil)) {
		((*cur_bag)->insides)++;
	}

	/*    printf("\n\tgot item: >>>%s<<<\n", ptr->data.name); */
	/*    fflush(stdout); */
}

static void gc__open_save_file(FILE **f1, char const *fnam, boolean *paniced)
{
	char out_str[82];

	*f1 = (FILE *)fopen(fnam, "r");
	if (*f1 == NULL) {
		sprintf(out_str, "Error Opening> %s", fnam);
		prt(out_str, 1, 1);
		prt("", 2, 1);
		*paniced = true;
	} else {
		rewind(*f1);
	}
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

static void gc__read_inventory(FILE *f1, encrypt_state *cf_state, char in_rec[1026],
			       boolean *paniced, boolean *was_dead)
{
	/* { Read in the inventory records.	}*/

	long lost_inven_count;
	boolean bag_lost;
	treas_ptr ptr, cur_bag;
	long i1;
	int x1, x2, x3, x4, x5, x6, x7, x8, x9, x10;

	inventory_list = nil;
	lost_inven_count = 0;
	bag_lost = false;
	cur_bag = nil;

	for (i1 = 0; i1 < inven_ctr; i1++) {
		unsigned long chtype_buf;
		read_decrypt(f1, cf_state, in_rec, paniced);
		sscanf(in_rec, "%lu %[^\n]", &chtype_buf,
		       inven_temp->data.name);
		inven_temp->data.tchar = chtype_buf;

		read_decrypt(f1, cf_state, in_rec, paniced);
		if (sscanf(in_rec, "%d %d", &x2, &x1) != 2) {
			*paniced = true;
		}
		inven_temp->is_in = x2;
		inven_temp->insides = x1;

		read_decrypt(f1, cf_state, in_rec, paniced);
		strncpy(inven_temp->data.damage, in_rec, sizeof(char[7]));

		read_decrypt(f1, cf_state, in_rec, paniced);
		/* with inven_temp->data do; */
		if (sscanf(in_rec, "%d %d %d %d %d %d %d %d %d %lu %lu %d %ld",
			   &x1, &x2, &x3, &x4, &x5, &x6, &x7, &x8, &x9,
			   &(inven_temp->data.flags),
			   &(inven_temp->data.flags2), &x10,
			   &(inven_temp->data.cost)) != 13) {
			*paniced = true;
		}

		inven_temp->data.tval = x1;
		inven_temp->data.subval = x2;
		inven_temp->data.weight = x3;
		inven_temp->data.number = x4;
		inven_temp->data.tohit = x5;
		inven_temp->data.todam = x6;
		inven_temp->data.ac = x7;
		inven_temp->data.toac = x8;
		inven_temp->data.p1 = x9;
		inven_temp->data.level = x10;

		if ((*was_dead) &&
		    ((inven_temp->data.flags2 & Insured_bit) == 0)) {
			if (inven_temp->data.tval == bag_or_sack) {
				bag_lost = true;
			}
			lost_inven_count++;
			inven_weight -=
			    (inven_temp->data.number * inven_temp->data.weight);
		} else if (bag_lost && inven_temp->is_in) {
			lost_inven_count++;
		} else {
			if (*was_dead) {
				inven_temp->data.flags2 &= ~Insured_bit;
				inven_temp->data.flags2 |= Blackmarket_bit;
			}
			gc__add_item(&cur_bag);
			bag_lost = false;
		}
	} /* end for i1 */

	for (inven_ctr = 0, ptr = inventory_list; ptr != nil;) {
		ptr = ptr->next;
		inven_ctr++;
	}

	if (lost_inven_count == 1) {
		msg_print("You lost an item that wasn't insured.");
	} else if (lost_inven_count > 1) {
		msg_print("You lost several items that weren't insured.");
	}
}

static void gc__read_equipment(FILE *f1, encrypt_state *cf_state, char in_rec[1026],
			       boolean *paniced, boolean *was_dead)
{
	/*{ Read in the equipment records.	}*/

	long i1;
	long lost_equip_count;
	int x1, x2, x3, x4, x5, x6, x7, x8, x9, x10;

	lost_equip_count = 0;

	for (i1 = Equipment_min; i1 < EQUIP_MAX; i1++) {
		unsigned long chtype_buf;

		read_decrypt(f1, cf_state, in_rec, paniced);
		sscanf(in_rec, "%lu %[^\n]", &chtype_buf,
		       inven_temp->data.name);
		inven_temp->data.tchar = chtype_buf;

		read_decrypt(f1, cf_state, in_rec, paniced);
		strncpy(inven_temp->data.damage, in_rec, sizeof(char[7]));

		read_decrypt(f1, cf_state, in_rec, paniced);
		if (sscanf(in_rec, "%d %d %d %d %d %d %d %d %d %lu %lu %d %ld",
			   &x1, &x2, &x3, &x4, &x5, &x6, &x7, &x8, &x9,
			   &(inven_temp->data.flags),
			   &(inven_temp->data.flags2), &x10,
			   &(inven_temp->data.cost)) != 13) {
			*paniced = true;
		}

		inven_temp->data.tval = x1;
		inven_temp->data.subval = x2;
		inven_temp->data.weight = x3;
		inven_temp->data.number = x4;
		inven_temp->data.tohit = x5;
		inven_temp->data.todam = x6;
		inven_temp->data.ac = x7;
		inven_temp->data.toac = x8;
		inven_temp->data.p1 = x9;
		inven_temp->data.level = x10;

		if ((*was_dead) && (inven_temp->data.tval > 0) &&
		    (uand(inven_temp->data.flags2, Insured_bit) == 0)) {
			lost_equip_count++;
			equipment[i1] = blank_treasure;
			inven_weight -=
			    inven_temp->data.number * inven_temp->data.weight;
			if (i1 != EQUIP_MAX - 1) {
				py_bonuses(&(inven_temp->data), -1);
			}
		} else {
			if (*was_dead) {
				inven_temp->data.flags2 =
				    uand(inven_temp->data.flags2, 0xBFFFFFFF);
			}
			equipment[i1] = inven_temp->data;
		}
	} /* end for f1 */

	equip_ctr -= lost_equip_count;
	if (lost_equip_count == 1) {
		msg_print("You lost a piece of equipment that wasn't insured.");
	} else if (lost_equip_count > 1) {
		msg_print("You lost several pieces of equipment that weren't "
			  "insured.");
	}
}

static void gc__read_dungeon(FILE *f1, encrypt_state *cf_state, char in_rec[1026],
			     boolean *paniced)
{
	long i1, i2, i3, i4, tot_treasure;
	long x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13;
	long count = 0;
	unsigned long xfloor;

	read_decrypt(f1, cf_state, in_rec, paniced);
	if (sscanf(in_rec, "%ld %ld %ld %ld", &cur_height, &cur_width,
		   &max_panel_rows, &max_panel_cols) != 4) {
		*paniced = true;
	}

	if (cur_height > MAX_HEIGHT) {
		*paniced = true;
	} else if (cur_width > MAX_WIDTH) {
		*paniced = true;
	}

	/*{ Read the floor	}*/

	if (!(*paniced)) {
		for (i1 = 1; i1 <= cur_height; i1++) {
			/* read_decrypt(f1, cf_state, in_rec, paniced); */
			for (i2 = 1; i2 <= cur_width; i2++) {

				if (count == 0) {
					read_decrypt(f1, cf_state, in_rec,
						     paniced);
					if (sscanf(in_rec, "%lu %ld", &xfloor,
						   &count) == 1) {
						count = 1;
					}
				}
				count--;

				/* with cave[i1][i2]. do; */
				cave[i1][i2].fval = (xfloor & 0x1f);
				if (xfloor & 0x20) {
					cave[i1][i2].fopen = true;
				}
				if (xfloor & 0x40) {
					cave[i1][i2].pl = true;
				}
				if (xfloor & 0x80) {
					cave[i1][i2].fm = true;
				}
				cave[i1][i2].tl = false;
				cave[i1][i2].tptr = 0;
				cave[i1][i2].cptr = 0;

			} /* end for i2 */
		}	 /* end for i1 */
	}

	/*{ Read the Treasure List		}*/
	tlink();
	if (!(*paniced)) {
		read_decrypt(f1, cf_state, in_rec, paniced);
		if (sscanf(in_rec, "%ld", &tot_treasure) != 1) {
			*paniced = true;
		}

		for (i1 = 1; i1 <= tot_treasure; i1++) {
			unsigned long chtype_buf;
			popt(&i2);
			/* with t_list[i2] do; */

			read_decrypt(f1, cf_state, in_rec, paniced);
			if (sscanf(in_rec, "%ld %ld", &i3, &i4) != 2) {
				*paniced = true;
			}
			cave[i3][i4].tptr = i2;

			read_decrypt(f1, cf_state, in_rec, paniced);
			sscanf(in_rec, "%lu %[^\n]", &chtype_buf,
			       t_list[i2].name);
			t_list[i2].tchar = chtype_buf;

			read_decrypt(f1, cf_state, in_rec, paniced);
			strcpy(t_list[i2].damage, in_rec);

			read_decrypt(f1, cf_state, in_rec, paniced);
			if (sscanf(in_rec, "%ld %ld %ld %ld %ld %ld %ld %ld "
					   "%ld %ld %ld %ld %ld",
				   &x1, &x2, &x3, &x4, &x5, &x6, &x7, &x8, &x9,
				   &x10, &x11, &x12, &x13) != 13) {
				*paniced = true;
			}

			t_list[i2].tval = x1;
			t_list[i2].subval = x2;
			t_list[i2].weight = x3;
			t_list[i2].number = x4;
			t_list[i2].tohit = x5;
			t_list[i2].todam = x6;
			t_list[i2].ac = x7;
			t_list[i2].toac = x8;
			t_list[i2].p1 = x9;
			t_list[i2].flags = x10;
			t_list[i2].flags2 = x11;
			t_list[i2].level = x12;
			t_list[i2].cost = x13;
		}
	}
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

static void gc__read_town(FILE *f1, encrypt_state *cf_state, char in_rec[1026],
			  boolean *paniced)
{
	long x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13;
	long i1, i2, i3;
	game_time_type st;

	/*{ Restore the town level stores 	}*/
	read_decrypt(f1, cf_state, in_rec, paniced);
	if (sscanf(in_rec, "%lu", &town_seed) != 1) {
		*paniced = true;
	}

	read_decrypt(f1, cf_state, in_rec, paniced);
	if (sscanf(in_rec, "%ld %ld %ld %ld %ld %ld %ld", &bank[0], &bank[6],
		   &bank[5], &bank[4], &bank[3], &bank[2], &bank[1]) != 7) {
		*paniced = true;
	}

	for (i1 = 0; i1 < MAX_STORES; i1++) {
		/* with stores[i1]. do; */
		read_decrypt(f1, cf_state, in_rec, paniced);
		sscanf(in_rec, "%ld", &i2);
		stores[i1].store_ctr = i2;
		for (i3 = 1; i3 <= i2; i3++) {
			unsigned long chtype_buf;
			/* with stores[i1].store_inven[i3].sitem do; */
			read_decrypt(f1, cf_state, in_rec, paniced);
			if (sscanf(in_rec, "%ld",
				   &stores[i1].store_inven[i3].scost) != 1) {
				*paniced = true;
			}

			read_decrypt(f1, cf_state, in_rec, paniced);
			sscanf(in_rec, "%lu %[^\n]", &chtype_buf,
			       stores[i1].store_inven[i3].sitem.name);
			stores[i1].store_inven[i3].sitem.tchar =
			    (chtype)chtype_buf;

			read_decrypt(f1, cf_state, in_rec, paniced);
			strcpy(stores[i1].store_inven[i3].sitem.damage, in_rec);

			read_decrypt(f1, cf_state, in_rec, paniced);
			if (sscanf(in_rec, "%ld %ld %ld %ld %ld %ld %ld %ld "
					   "%ld %ld %ld %ld %ld",
				   &x1, &x2, &x3, &x4, &x5, &x6, &x7, &x8, &x9,
				   &x10, &x11, &x12, &x13) != 13) {
				*paniced = true;
			}

			stores[i1].store_inven[i3].sitem.tval = x1;
			stores[i1].store_inven[i3].sitem.subval = x2;
			stores[i1].store_inven[i3].sitem.weight = x3;
			stores[i1].store_inven[i3].sitem.number = x4;
			stores[i1].store_inven[i3].sitem.tohit = x5;
			stores[i1].store_inven[i3].sitem.todam = x6;
			stores[i1].store_inven[i3].sitem.ac = x7;
			stores[i1].store_inven[i3].sitem.toac = x8;
			stores[i1].store_inven[i3].sitem.p1 = x9;
			stores[i1].store_inven[i3].sitem.flags = x10;
			stores[i1].store_inven[i3].sitem.flags2 = x11;
			stores[i1].store_inven[i3].sitem.level = x12;
			stores[i1].store_inven[i3].sitem.cost = x13;
		}

		/*{ If not current version then re-outfit the stores      }*/

		read_decrypt(f1, cf_state, in_rec, paniced);
		if (sscanf(in_rec, "%ld %ld %ld %ld %ld %ld %ld", &x1, &x2, &x3,
			   &x4, &x5, &x6, &x7) != 7) {
			*paniced = true;
		}
		stores[i1].owner = x1;
		stores[i1].insult_cur = x2;
		st.year = x3;
		st.month = x4;
		st.day = x5;
		st.hour = x6;
		st.secs = x7;
		stores[i1].store_open = st;
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
