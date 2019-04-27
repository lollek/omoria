/* wizard.c */
/**/

#include "imoria.h"
#include "dungeon.h"

#define LOW_NUM -98765432

#define PB(x, y, z) put_buffer((x), (y), (z))

void game_version()
{
	/*{ Print Moria credits					-RAK-	}*/

	/* why is this in the wizard code? */

	char tmp_str[82];

	clear_from(1);

	sprintf(tmp_str, "              VMS Moria Version 4.8");

	PB(tmp_str, 1, 1);
	PB("Version 0.1  : 03/25/83", 2, 1);
	PB("Version 1.0  : 05/01/84", 3, 1);
	PB("Version 2.0  : 07/10/84", 4, 1);
	PB("Version 3.0  : 11/20/84", 5, 1);
	PB("Version 4.0  : 01/20/85", 6, 1);
	PB("Modules :", 8, 1);
	PB("     V1.0  Dungeon Generator      - RAK", 9, 1);
	PB("           Character Generator    - RAK & JWT", 10, 1);
	PB("           Moria Module           - RAK", 11, 1);
	PB("           Miscellaneous          - RAK & JWT", 12, 1);
	PB("     V2.0  Town Level & Misc      - RAK", 13, 1);
	PB("     V3.0  Internal Help & Misc   - RAK", 14, 1);
	PB("     V4.0  Source Release Version - RAK", 15, 1);
	PB("Robert Alan Koeneke               Jimmey Wayne Todd Jr.", 17, 1);
	PB("Student/University of Oklahoma    Student/University of Oklahoma",
	   18, 1);
	PB("119 Crystal Bend                  1912 Tiffany Dr.", 19, 1);
	PB("Norman, OK 73069                  Norman, OK  73071", 20, 1);
	PB("(405)-321-2925                    (405) 360-6792", 21, 1);

	pause_game(24);
	clear_rc(1, 1);

	sprintf(tmp_str, "              VMS Imoria Version 4.85");
	PB(tmp_str, 1, 1);
	PB("Programers : Robert Alan Koeneke / University of Oklahoma", 3, 1);
	PB("             Jimmey Wayne Todd   / University of Oklahoma", 4, 1);
	PB(" ", 5, 1);
	PB("UW Modifications by : Kenneth Case, Mary Conner,", 6, 1);
	PB("                      Robert DeLoura, Dan Flye,", 7, 1);
	PB("                      Todd Gardiner, Dave Jungck,", 8, 1);
	PB("                      Andy Walker, Dean Yasuda.", 9, 1);
	PB(" ", 10, 1);
	PB("University of Washington version 4.8", 11, 1);

	pause_game(24);
	clear_rc(1, 1);

	sprintf(tmp_str, "          Linux Imoria Version %s", imoria_version());
	PB(tmp_str, 1, 1);
	PB("Version 4.85 : 06/25/98        Finished porting from pascal", 3, 1);
	PB(" ", 4, 1);
	PB("Linux port by Stephen Kertes, 1997-98.", 5, 1);
	PB(" ", 6, 1);
	PB("I fixed all the cheats I used while at UW, thanks Paul.", 7, 1);
	PB(" ", 8, 1);
	PB("Terminal, screen_map and (L)ocate taken from umoria 5.5", 9, 1);
	PB("                                        ", 10, 1);
	pause_game(24);
	draw_cave();
}
#undef PB

void bpswd()
{
	/*{ Builds passwords                                      -RAK-   }*/
	/*  Do remember that the passwords are max 12 chars plus a null. */

	/* lesser op password */
	strcpy(password1, "fragrance");

	/* full op password */
	strcpy(password2, "mopwillow");
}

boolean check_pswd(char passwd[134], boolean present)
{
	long i1;
	char x;
	char tpw[134]; /*  : packed array [1..12] of char;*/
	/* account_type   account; */
	boolean checked_out = false;

	/* perhaps crpyt() should be used?? */
	ENTER(("check_pswd", ""));

	if (present) {
		strcpy(tpw, passwd);
	} else {
		i1 = 0;
		prt("Password : ", 1, 1);
		do {
			x = inkey();
			switch (x) {
			case 10:
			case 13:
				break;
			default:
				tpw[i1++] = x;
				break;
			}
		} while (!((i1 == 12) || (x == 13) || (x == 10)));

		tpw[i1] = 0;
	}

	MSG(("Login attempt: %s", tpw));
	if (!(strcmp(tpw, password1))) {
		checked_out = true;
		wizard1 = true;
	} else if (!strcmp(tpw, password2)) {
		checked_out = true;
		wizard1 = true;
		wizard2 = true;
	}
	/*
	      if ( uw$id ) then
		begin
		  get_account(account.body);
		  account.length := index( account.body, ' ' )-1;
		  if index( wizards, ':'+account+':' ) = 0 then
		     begin
		       wizard1 := false;
		       wizard2 := false;
		       checked_out := false;
		     end;
		end;
	*/

	msg_flag = false;
	if (!present) {
		erase_line(msg_line, msg_line);
	}

	player_cheated |= checked_out;
	became_wizard = checked_out;

	LEAVE("check_pswd", "");

	return checked_out;
}

void wizard_light()
{
	/*{ Light up the dungeon					-RAK-
	 * }*/

	long i1, i2, i3, i4;
	boolean flag;

	if (cave[char_row][char_col].pl) {
		flag = false;
	} else {
		flag = true;
	}

	for (i1 = 0; i1 < cur_height; i1++) {
		for (i2 = 0; i2 < cur_width; i2++) {
			if (is_in(cave[i1][i2].fval, floor_set)) {
				for (i3 = i1 - 1; i3 <= i1 + 1; i3++) {
					for (i4 = i2 - 1; i4 <= i2 + 1; i4++) {
						cave[i3][i4].pl = flag;
						if (!(flag)) {
							cave[i3][i4].fm = false;
						}
					}
				}
			}
		}
	}

	prt_map();
	detect_trap();
	detect_sdoor();
}

void monster_summon_by_name(long y, long x, char name[28], boolean present,
			    boolean sleepy)
{
	/*{ Wizard routine for summoning a specific monster       -RAD-   }*/

	long i1 = 0, i2, i3, i4;
	char monster[28];
	boolean junk;

	if (!present) {
		prt("Monster desired:  ", 1, 1);
		junk = (get_string(monster, 1, 19, 26));
	} else {
		strcpy(monster, name);
		junk = true;
	}

	if (junk) {
		i2 = 0;
		sscanf(monster, "%ld", &i2);
		if (i2 < 0) {
			i2 = 1;
		}
		if (i2 > MAX_CREATURES) {
			i2 = MAX_CREATURES;
		}

		if ((i2 > 0) && (i2 <= MAX_CREATURES)) {
			/* summon by number */
			i1 = 0;
			do {
				i3 = y - 2 + randint(3);
				i4 = x - 2 + randint(3);
				if (in_bounds(i3, i4)) {
					/*with cave[i3,i4] do*/
					if (is_in(cave[i3][i4].fval,
						  floor_set)) {
						if (cave[i3][i4].fopen) {
							place_monster(
							    i3, i4, i2, sleepy);
							i1 = 8;
							y = i3;
							x = i4;
						}
					}
				}
				i1++;
			} while (i1 <= 8);
		} else {
			/* find by name, then summon */
			for (i2 = 1; i2 <= MAX_CREATURES; i2++) {
				if ((strstr(c_list[i2].name, monster) !=
				     NULL) &&
				    (i1 != 10)) {
					i1 = 0;
					do {
						i3 = y - 2 + randint(3);
						i4 = x - 2 + randint(3);
						if (in_bounds(i3, i4)) {
							/*with cave[i3,i4] do*/
							if (is_in(cave[i3][i4]
								      .fval,
								  floor_set)) {
								if (cave[i3][i4]
									.cptr ==
								    0) {
									if (cave[i3][i4]
										.fopen) {
										place_monster(
										    i3,
										    i4,
										    i2,
										    sleepy);
										i1 =
										    9;
										y = i3;
										x = i4;
									}
								}
							}
						}
						i1++;
					} while (i1 <= 9);
				}
			}
		} /* end else */
	}	 /* end if junk */

	if (!present) {
		erase_line(msg_line, msg_line);
	}
}

void wmi__init_data_list(list_elem_ptr *data_list)
{
	/*{ Code streamlined a bit by Dean Yasuda, to eliminate the
	  inefficient quicksort/shell-sort combination.  Exact duplicate
	  items eliminated from output list.              --MAV }*/

	list_elem_ptr curse;
	treasure_type temp_ray[MAX_OBJECTS + 1];
	long i1, i2, i3, gap;
	treasure_type tmp;

	for (i1 = 1; i1 <= MAX_OBJECTS; i1++) {
		temp_ray[i1] = object_list[i1];
	}

	for (gap = MAX_OBJECTS / 2; gap > 0; gap /= 2) {
		for (i1 = gap + 1; i1 <= MAX_OBJECTS; i1++) {
			for (i2 = i1 - gap; i2 > 0; i2 -= gap) {
				i3 = i2 + gap;
				if ((temp_ray[i2].tval > temp_ray[i3].tval) ||
				    ((temp_ray[i2].tval == temp_ray[i3].tval) &&
				     (temp_ray[i2].subval >
				      temp_ray[i3].subval))) {
					tmp = temp_ray[i2];
					temp_ray[i2] = temp_ray[i3];
					temp_ray[i3] = tmp;
				} else {
					i2 = 0;
				}
			}
		}
	}

	*data_list =
	    (list_elem_ptr)safe_malloc(sizeof(list_elem), "init_data_list");
	curse = *data_list;
	curse->data = temp_ray[1];

	for (i1 = 2; i1 <= MAX_OBJECTS; i1++) {
		if ((temp_ray[i1].tval != temp_ray[i1 - 1].tval) ||
		    (temp_ray[i1].subval != temp_ray[i1 - 1].subval)) {
			curse->next = (list_elem_ptr)safe_malloc(
			    sizeof(list_elem), "init_data_list");
			curse = curse->next;
			curse->data = temp_ray[i1];
			curse->next = nil;
		}
	}
}

void wmi__display_commands()
{
	prt("You may:", 22, 1);
	prt(" p) Pick an item.              b) Browse to next page.", 23, 1);
	prt("^R) Redraw screen.           Esc) Exit.", 24, 1);
}

void wmi__display_list(list_elem_ptr start, long *cur_display_size,
		       list_elem_ptr cur_display[], list_elem_ptr *blegga,
		       list_elem_ptr *data_list)
{
	long count, old_display_size;
	char temp[134];

	old_display_size = *cur_display_size;
	count = 0;

	for (; (start != nil) && (count < MOO_DISPLAY_SIZE);) {
		count++;
		if (cur_display[count] != start) {
			cur_display[count] = start;
			sprintf(temp, "%c) %s", 96 + (int)count,
				start->data.name);
			prt(temp, count + 1, 1);
		}
		start = start->next;
	}

	*cur_display_size = count;

	for (; old_display_size > *cur_display_size; old_display_size--) {
		erase_line(old_display_size + 3, 1);
		cur_display[old_display_size] = nil;
	}

	if (start == nil) {
		*blegga = *data_list;
	} else {
		*blegga = start;
	}
}

void wmi__clear_display(long *cur_display_size, list_elem_ptr cur_display[])
{
	long i4;

	*cur_display_size = 0;
	for (i4 = 1; i4 <= MOO_DISPLAY_SIZE; i4++) {
		cur_display[i4] = nil;
	}
}

void wmi__display_screen(long *cur_display_size, list_elem_ptr cur_display[],
			 list_elem_ptr cur_top, list_elem_ptr *blegga,
			 list_elem_ptr *data_list)
{
	clear_rc(1, 1);
	wmi__clear_display(cur_display_size, cur_display);
	wmi__display_list(cur_top, cur_display_size, cur_display, blegga,
			  data_list);
	wmi__display_commands();
}

boolean wmi__get_list_entry(long *com_val, char pmt[82], long i1, long i2)
{
	char command;
	char temp[134];
	boolean flag = true;

	*com_val = 0;

	sprintf(temp, "(Entries %c-%c, ^Z to exit) %s", (int)i1 + 96,
		(int)i2 + 96, pmt);
	for (; (((*com_val < i1) || (*com_val > i2)) && (flag));) {
		prt(temp, 1, 1);
		command = inkey();
		*com_val = (long)command;
		switch (*com_val) {
		case 3:
		case 25:
		case 26:
		case 27:
			flag = false;
			break;
		default:
			*com_val -= 96;
			break;
		}
	}

	erase_line(1, 1);

	return flag;
}

boolean wmi__parse_command(list_elem_ptr *blegga, list_elem_ptr *cur_top,
			   long *cur_display_size, boolean *exit_flag,
			   treasure_type *back, list_elem_ptr cur_display[],
			   list_elem_ptr *data_list)

{
	char command;
	long com_val, which;
	boolean flag = false;

	if (get_com("", &command)) {
		com_val = (long)command;
		switch (com_val) {

		case 18: /*{^R}*/
			wmi__display_screen(cur_display_size, cur_display,
					    *cur_top, blegga, data_list);
			break;

		case 32: /*{ }*/
		case 98: /*{b}*/
			if (cur_top == blegga) {
				prt("Entire list is displayed.", 1, 1);
			} else {
				cur_top = blegga;
				wmi__display_list(*cur_top, cur_display_size,
						  cur_display, blegga,
						  data_list);
			}
			break;

		case 112: /*{p}*/
			if (*cur_display_size > 0) {
				if (wmi__get_list_entry(&which,
							" Pick which one?", 1,
							*cur_display_size)) {
					*exit_flag = true;
					flag = true;
					*back = cur_display[which]->data;
				}
			}
			break;

		default:
			prt("Invalid command", 1, 1);
			break;
		}
	} else {
		*exit_flag = true;
	}

	return flag;
}

boolean wizard_moo_item(treasure_type *back)
{
	/*{ Wizard routine to pick an item from the entire list, and
	  magic it until satisfied                              -DMF-   }*/

	list_elem_ptr data_list;
	list_elem_ptr cur_top;
	list_elem_ptr blegga;
	list_elem_ptr cur_display[MOO_DISPLAY_SIZE + 1];
	long cur_display_size;
	boolean exit_flag = false;
	boolean flag = false;

	*back = blank_treasure;
	wmi__init_data_list(&data_list);
	cur_top = data_list;
	wmi__display_screen(&cur_display_size, cur_display, cur_top, &blegga,
			    &data_list);

	do {
		flag = wmi__parse_command(&blegga, &cur_top, &cur_display_size,
					  &exit_flag, back, cur_display,
					  &data_list);
	} while (!exit_flag);

	return flag;
}

boolean si__get_new_ttype(char s[70], char str[82], char out_str[134])
{
	/*{prompts for new string, <CR> leaves old value}*/
	char os[70];
	boolean flag = false;

	if (strlen(s) > 0) {
		sprintf(out_str, "%s [%s] : ", str, s);
	} else {
		sprintf(out_str, "%s : ", str);
	}

	prt(out_str, 1, 1);
	strcpy(os, s);

	if (get_string(s, 1, strlen(out_str) + 1, 40)) {
		flag = true;
		if ((strlen(os) > 0) && (strlen(s) == 0)) {
			strcpy(s, os);
		}
	}

	return flag;
}

boolean si__get_item_descriptions(char out_str[134], treasure_type moo_item[],
				  boolean *found, long *num_found)
{
	/*{ask wizard for item information/Moo!, Moo./Moo?}*/
	boolean ook;
	boolean flag = false;

	if (si__get_new_ttype(s1, "Item string", out_str)) {
		ook = true;

		if (strstr(s1, "Moo!") == s1) {
			moo_item[1] = blank_treasure;
			ook = wizard_moo_item(&(moo_item[1]));
			if (ook) {
				*found = true;
				*num_found = 1;
			}
			draw_cave();
		}

		if (ook) {
			if (si__get_new_ttype(s2, "More stuff #1", out_str)) {
				if (si__get_new_ttype(s3, "More stuff #2",
						      out_str)) {
					if (si__get_new_ttype(s4, "Special",
							      out_str)) {

						if (i_summ_count > 0) {
							sprintf(
							    out_str,
							    "Maximum number of "
							    "tries: [%ld] : ",
							    i_summ_count);
						} else {
							strcpy(out_str,
							       "Maximum number "
							       "of tries: ");
						}
						prt(out_str, 1, 1);
						if (get_string(out_str, 1,
							       strlen(out_str) +
								   1,
							       60)) {
							flag = true;
						}
					}
				}
			}
		}
	}
	return flag;
}

boolean si__narrow(char s[70], long *num_found, long moo_cursor[])
{
	/*{ eliminate all items without string s from array moo_cursor }*/
	long i1, i2;
	boolean flag = false;

	i2 = 1;
	if (strlen(s) > 0) {
		for (i1 = 1; i1 <= *num_found; i1++) {
			if (strstr(object_list[moo_cursor[i1]].name, s) !=
			    NULL) {
				moo_cursor[i2] = moo_cursor[i1];
				i2++;
			}
		}
	}

	if (i2 > 1) {
		flag = true; /*{at least one feasible substring found}*/
		*num_found = i2 - 1;
	}

	return flag;
}

boolean si__narrow_choices(long *num_found, long moo_cursor[],
			   treasure_type moo_item[])
{
	/*{ use 3 substrings to narrow down specify possible items }*/

	long i1;
	boolean flag = false;

	ENTER(("si__narrow_choices", "w"));

	for (i1 = 1; i1 <= MAX_OBJECTS; i1++) {
		moo_cursor[i1] = i1;
	}
	*num_found = MAX_OBJECTS;

	if (si__narrow(s1, num_found, moo_cursor)) {
		flag = true;
		if (si__narrow(s2, num_found, moo_cursor)) {
			si__narrow(s3, num_found, moo_cursor);
		}
		for (i1 = 1; i1 <= *num_found; i1++) {
			moo_item[i1] = object_list[moo_cursor[i1]];
		}
	}

	RETURN("si__narrow_choices", "w", 'b', "found:", &flag);
	return flag;
}

void si__pesky_stuff(long *best_value, long *good_value,
		     treasure_type *best_pick, treasure_type *good_pick,
		     long *optimize, char out_str[134], long *i_summ_count,
		     long *cur_pos, long x, long y)
{
	/*{init variables, see if optimizing (1=best, -1= worst); find # of
	 * tries }*/
	long omax;

	*best_value = LOW_NUM;
	*good_value = LOW_NUM;
	*best_pick = yums[5]; /*{rice-a-roni}*/
	*good_pick = yums[5];

	if (strstr(s4, "Moo.") != NULL) {
		*optimize = 1;
	} else if (strstr(s4, "Moo?") != NULL) {
		*optimize = -1;
	} else {
		*optimize = 0;
	}

	omax = *i_summ_count;
	sscanf(out_str, "%ld", i_summ_count);

	if (*i_summ_count == 0) {
		*i_summ_count = omax;
	}
	if (*i_summ_count <= 0) {
		*i_summ_count = 1;
	}
	popt(cur_pos);
	cave[y][x].tptr = *cur_pos;
	t_list[*cur_pos] = blank_treasure;
	sprintf(t_list[*cur_pos].name, "& bogus summoned item %ld", *cur_pos);
}

boolean si__optimize_item(treasure_type *pick, long *value, long optimize,
			  long cur_pos)
{
	/*{ formula for comparing value of items}*/
	long i1;
	boolean flag = false;

	/* with t_list[cur_pos]. do; */
	i1 = optimize * (t_list[cur_pos].cost + t_list[cur_pos].tohit +
			 t_list[cur_pos].todam + t_list[cur_pos].toac);
	if (i1 > *value) {
		*value = i1;
		*pick = t_list[cur_pos];
		flag = true;
	}

	return flag;
}

boolean summon_item(long y, long x, char name1[70], char name2[70], long count,
		    boolean present)
{
	/*{ Wizard routine to summon a random item by substring(s) of its }*/
	/*{ name, with a maximum # of tries			-DMF-	    }*/

	long i1, i2, num_found;
	long optimize;
	long best_value, good_value;
	treasure_type best_pick, good_pick;
	boolean flag, done, found;
	char out_str[134];
	long cur_pos;
	char command;
	treasure_type moo_item[MAX_OBJECTS + 1];
	long moo_cursor[MAX_OBJECTS + 1];
	boolean return_value = false;

	ENTER(("summon_item", "w"));

	found = false;
	done = false;

	if (present) {
		flag = (strlen(name1) != 0);
		strcpy(s1, name1);
		strcpy(s2, name2);
		strcpy(s3, "");
		strcpy(s4, "Moo.");
		sprintf(out_str, "%ld", count);
	} else {
		/*{found := true iff successful Moo!}*/
		flag = si__get_item_descriptions(out_str, moo_item, &found,
						 &num_found);
	}

	if (flag) {
		si__pesky_stuff(&best_value, &good_value, &best_pick,
				&good_pick, &optimize, out_str, &i_summ_count,
				&cur_pos, x, y);

		if (!found) {
			/*{create array of all ok choices}*/
			found = si__narrow_choices(&num_found, moo_cursor,
						   moo_item);
		}

		if (found) {
			if (!present) {
				msg_print("Press any key to abort...");
				refresh();
			}

			for (i1 = 0; (i1 < i_summ_count) && (!done); i1++) {
				t_list[cur_pos] =
				    moo_item[((num_found * i1) / i_summ_count) +
					     1];

				if (!present) {
					inkey_delay(&command, 0);
					done = (command != 0);
				}

				magic_treasure(cur_pos, 1000, false);

				if (((strlen(s2) == 0) ||
				     (strstr(t_list[cur_pos].name, s2) !=
				      NULL)) &&
				    ((strlen(s3) == 0) ||
				     (strstr(t_list[cur_pos].name, s3) !=
				      NULL))) {
					if (si__optimize_item(
						&best_pick, &best_value,
						optimize, cur_pos)) {
						/*{ leave loop prematurely if
						 * not optimizing and item is
						 * found }*/
						if (optimize == 0) {
							done = true;
						}
					}

					/*{ while no correct pick, get best
					 * non-correct item }*/
				} else if ((optimize != 0) &&
					   (best_value == LOW_NUM)) {
					si__optimize_item(&good_pick,
							  &good_value, optimize,
							  cur_pos);
				}

			} /* end for */
		}	 /* end if found */

		if (best_value > LOW_NUM) {
			msg_print("Allocated.");
			t_list[cur_pos] = best_pick;
			/* with t_list[cur_pos]. do; */
			if (t_list[cur_pos].subval > 255) {
				i2 = t_list[cur_pos].cost;
				if (i2 < 3) {
					i2 = 3;
				}
				t_list[cur_pos].number =
				    i_summ_count / sqrt(100 * i2 / GOLD_VALUE);
				if (t_list[cur_pos].number < 1) {
					t_list[cur_pos].number = 1;
				} else if (t_list[cur_pos].number > 100) {
					t_list[cur_pos].number = 100;
				}
			}
		} else if (good_value > LOW_NUM) {
			msg_print("Found, but not perfect match.");
			t_list[cur_pos] = good_pick;
		} else {
			msg_print("Unfortunately your wish did not come true.");
			msg_print("You have, however, been awarded a valuable "
				  "consolation gift!");
			t_list[cur_pos] = yums[5]; /*{rice}*/
			t_list[cur_pos].number = 12;
		}
		return_value = true;
	} else { /*{ if flag }*/
		msg_print("Invalid input");
	}

	RETURN("summon_item", "w", 'b', "allocated:", &return_value);
	return return_value;
}

void enter_wizard_mode(boolean ask_for_pass)
{
	if (wizard1) {
		msg_print("Wizard mode off.");
		wizard1 = false;
		wizard2 = false;
		reset_flag = true;
		no_controly();
	} else {
		if (became_wizard || !ask_for_pass) {
			/*if (check_pswd("doublespeak",true)) {*/
			if (check_pswd(password1, true)) {
				msg_print("Wizard mode on.");
				controly();
			}
		} else {
			if (check_pswd("", false)) {
				msg_print("Wizard mode on.");
				controly();
			}
		}
	}
}

void esf__display_commands()
{
	prt("You may:", 21, 1);
	prt(" d) Delete an entry.              b) Browse to next page.", 22, 1);
	prt(" c) Change an entry.", 23, 1);
	prt(" q) Quit and save changes       Esc) Exit without saving.", 24, 1);
}

void esf__display_list(int start, char list[][134], int n1, int *blegga,
		       int *cur_display_size)
{
	long count, old_display_size;
	char out_val[134];

	old_display_size = *cur_display_size;

	for (count = 0; (start <= n1) && (count < 15); start++) {
		count++;

		sprintf(out_val, "%c)%s", (char)(96 + count), list[start]);
		if (strlen(out_val) > 80) {
			out_val[79] = 0;
		}
		prt(out_val, count + 3, 1);
	}

	*cur_display_size = count;
	for (; (old_display_size > *cur_display_size); old_display_size--) {
		erase_line(old_display_size + 3, 1);
	}

	if (start > n1) {
		*blegga = 1;
	} else {
		*blegga = start;
	}
}

void esf__display_screen(int cur_top, char list[][134], int n1, int *blegga,
			 int *cur_display_size)
{
	C_clear_screen();
	*cur_display_size = 0;
	put_buffer("  Username     Points  Diff    Character name    Level  "
		   "Race         Class",
		   2, 1);
	put_buffer("  ____________ ________ _ ________________________ __ "
		   "__________ ______________",
		   3, 1);
	esf__display_list(cur_top, list, n1, blegga, cur_display_size);
	esf__display_commands();
}

boolean esf__get_list_entry(int *com_val, char pmt[82], int cur_top, int i1,
			    int i2)
{
	char out_val[82];
	boolean flag = true;

	*com_val = 0;

	sprintf(out_val, "(Entries %c-%c, Esc to exit) %s", (char)(i1 + 96),
		(char)(i2 + 96), pmt);

	for (; (((*com_val < i1) || (*com_val > i2)) && (flag));) {
		prt(out_val, 1, 1);
		*com_val = inkey();

		switch (*com_val) {
		case 3:
		case 25:
		case 26:
		case 27:
			flag = false;
			break;
		default:
			(*com_val) += (cur_top - 97);
			break;
		}
	}

	erase_line(1, 1);
	return flag;
}

void esf__delete_entry(int cur_top, char list[][134], int *n1,
		       int cur_display_size)
{
	int which, i1;

	if (cur_display_size > 0) {
		if (esf__get_list_entry(&which, " Delete which one?", cur_top,
					1, cur_top + cur_display_size - 1)) {

			for (i1 = which; i1 < *n1; i1++) {
				strcpy(list[i1], list[i1 + 1]);
			}
			(*n1)--;
		}
	}
}

void esf__parse_command(char list[][134], int *cur_top, int *n1, int *blegga,
			int *cur_display_size, boolean *exit_flag,
			boolean *want_save)
{
	char command;

	if (get_com("", &command)) {

		switch (command) {

		case 18: /*{^R}*/
			esf__display_screen(*cur_top, list, *n1, blegga,
					    cur_display_size);
			break;

		case 32: /*{ }*/
		case 98: /*{b}*/
			if (*cur_top == *blegga) {
				prt("Entire list is displayed.", 1, 1);
			} else {
				*cur_top = *blegga;
				esf__display_list(*cur_top, list, *n1, blegga,
						  cur_display_size);
			}
			break;

		case 100: /*{d}*/
			esf__delete_entry(*cur_top, list, n1,
					  *cur_display_size);
			esf__display_list(*cur_top, list, *n1, blegga,
					  cur_display_size);
			break;

		case 113: /*{q}*/
			*exit_flag = true;
			*want_save = true;
			break;

		default:
			prt("Invalid command", 1, 1);
			break;
		}
	} else {
		*exit_flag = true;
	}
}

boolean cc__input_field(char prompt[134], long *num, long min, long max,
			boolean *ok)
{
	char out_val[134];
	long len;
	boolean return_value = false;

	sprintf(out_val, "Current = %ld, %s", *num, prompt);
	len = strlen(out_val);
	prt(out_val, 1, 1);

	if (get_string(out_val, 1, len + 1, 10)) {
		len = -999;
		sscanf(out_val, "%ld", &len);
		if ((len >= min) && (len <= max)) {
			*ok = true;
			*num = len;
		} else {
			*ok = false;
		}
		return_value = true;
	}

	return return_value;
}

void change_character()
{
	/*{ Wizard routine for gaining on stats                   -RAK-   }*/

	long tmp_val;
	char tmp_str[82];
	stat_set tstat;
	boolean flag = false;
	boolean abort = false;

	/* with py.stat do; */
	for (tstat = STR; (tstat <= CHR) && !abort; tstat++) {
		switch (tstat) {
		case STR:
			prt("(0 - 250) Strength     = ", 1, 1);
			break;
		case INT:
			prt("(0 - 250) Intelligence = ", 1, 1);
			break;
		case WIS:
			prt("(0 - 250) Wisdom       = ", 1, 1);
			break;
		case DEX:
			prt("(0 - 250) Dexterity    = ", 1, 1);
			break;
		case CON:
			prt("(0 - 250) Constitution = ", 1, 1);
			break;
		case CHR:
			prt("(0 - 250) Charisma     = ", 1, 1);
			break;
		}

		if (!get_string(tmp_str, 1, 26, 10)) {
			abort = true;
		}

		if (!abort) {
			tmp_val = -999;
			sscanf(tmp_str, "%ld", &tmp_val);
			if (tmp_val != -999) {
				C_player_mod_perm_stat((int16_t)tstat, tmp_val);
				C_player_recalc_stats();
				prt_stat_block();
			}
		}
	}

	/* with player_do; */
	if (!abort) {
		tmp_val = player_mhp;
		if (cc__input_field("(1-32767) Hit points = ", &tmp_val, 1,
				    32767, &flag)) {
			if (flag) {
				player_mhp = tmp_val;
				player_chp = player_mhp;
				prt_stat_block();
			}
		} else {
			abort = true;
		}
	}

	if (!abort) {
		tmp_val = player_mana;
		if (cc__input_field("(0-32767) Mana = ", &tmp_val, 0,
					32767, &flag)) {
			if (flag) {
				player_mana = tmp_val;
				player_cmana = player_mana;
				prt_stat_block();
			}
		} else {
			abort = true;
		}
	}

	if (!abort) {
		tmp_val = 0;
		if (cc__input_field("(0-200) Searching = ", &tmp_val, 0, 200,
				    &flag)) {
			C_player_mod_search_skill(tmp_val);
		} else {
			abort = true;
		}
	}

	if (!abort) {
		tmp_val = player_stl;
		if (cc__input_field("(0-10) Stealth = ", &tmp_val, 0, 10,
				    &flag)) {
			player_stl = tmp_val;
		} else {
			abort = true;
		}
	}

	if (!abort) {
		tmp_val = player_disarm;
		if (cc__input_field("(0-200) Disarming = ", &tmp_val, 0, 200,
				    &flag)) {
			player_disarm = tmp_val;
		} else {
			abort = true;
		}
	}

	if (!abort) {
		tmp_val = player_save;
		if (cc__input_field("(0-100) Save = ", &tmp_val, 0, 100,
				    &flag)) {
			player_save = tmp_val;
		} else {
			abort = true;
		}
	}

	if (!abort) {
		tmp_val = player_bth;
		if (cc__input_field("(0-200) Base to hit = ", &tmp_val, 0, 200,
				    &flag)) {
			player_bth = tmp_val;
		} else {
			abort = true;
		}
	}

	if (!abort) {
		tmp_val = player_bthb;
		if (cc__input_field("(0-200) Bows/Throwing = ", &tmp_val, 0,
				    200, &flag)) {
			player_bthb = tmp_val;
		} else {
			abort = true;
		}
	}

	if (!abort) {
		tmp_val = player_money[TOTAL_];
		if (cc__input_field("Player Gold = ", &tmp_val, 0, 100000000,
				    &flag)) {
			if (flag) {
				tmp_val =
				    (tmp_val - player_money[TOTAL_]) * GOLD_VALUE;
				if (tmp_val > 0) {
					add_money(tmp_val);
				} else {
					subtract_money(-tmp_val, true);
				}
				prt_stat_block();
			}
		} else {
			abort = true;
		}
	}

	if (!abort) {
		if (!cc__input_field("Account Gold = ", &player_account, 0,
				     1000000000, &flag)) {
			abort = true;
		}
	}

	if (!abort) {
		tmp_val = inven_weight;
		if (cc__input_field("Current Weight (100/unit weight) = ",
				    &tmp_val, 0, 900000, &flag)) {
			inven_weight = tmp_val;
			prt_stat_block();
		}
	}

	erase_line(msg_line, msg_line);
	py_bonuses(&blank_treasure, 0);
}

void wizard_create()
{
	/*{ Wizard routine for creating objects                   -RAK-   }*/

	long tmp_val;
	char tmp_str[82];
	boolean flag;

	msg_print("Warning: This routine can cause fatal error.");
	msg_print(" ");
	msg_flag = false;

	/* with inven_temp->data do; */
	prt("Name   : ", 1, 1);
	if (get_string(tmp_str, 1, 10, 40)) {
		strcpy(inven_temp->data.name, tmp_str);
	} else {
		strcpy(inven_temp->data.name, "& Wizard Object!");
	}

	do {
		prt("Tval   : ", 1, 1);
		get_string(tmp_str, 1, 10, 10);
		tmp_val = 0;
		sscanf(tmp_str, "%ld", &tmp_val);
		flag = true;

	} while (!flag);

	inven_temp->data.tval = tmp_val;

	prt("Flags  (In HEX): ", 1, 1);
	inven_temp->data.flags = get_hex_value(1, 18, 8);

	prt("Flags2 (In HEX): ", 1, 1);
	inven_temp->data.flags2 = get_hex_value(1, 18, 8);

	prt("P1     : ", 1, 1);
	get_string(tmp_str, 1, 10, 10);
	tmp_val = 0;
	sscanf(tmp_str, "%ld", &tmp_val);
	inven_temp->data.p1 = tmp_val;

	prt("Cost : ", 1, 1);
	get_string(tmp_str, 1, 10, 10);
	tmp_val = 0;
	sscanf(tmp_str, "%ld", &tmp_val);
	inven_temp->data.cost = tmp_val;

	prt("Subval : ", 1, 1);
	get_string(tmp_str, 1, 10, 10);
	tmp_val = 1;
	sscanf(tmp_str, "%ld", &tmp_val);
	inven_temp->data.subval = tmp_val;

	prt("Weight : ", 1, 1);
	get_string(tmp_str, 1, 10, 10);
	tmp_val = 1;
	sscanf(tmp_str, "%ld", &tmp_val);
	inven_temp->data.weight = tmp_val;

	prt("Number : ", 1, 1);
	get_string(tmp_str, 1, 10, 10);
	tmp_val = 1;
	sscanf(tmp_str, "%ld", &tmp_val);
	inven_temp->data.number = tmp_val;

	prt("+To hit: ", 1, 1);
	get_string(tmp_str, 1, 10, 10);
	tmp_val = 0;
	sscanf(tmp_str, "%ld", &tmp_val);
	inven_temp->data.tohit = tmp_val;

	prt("+To dam: ", 1, 1);
	get_string(tmp_str, 1, 10, 10);
	tmp_val = 0;
	sscanf(tmp_str, "%ld", &tmp_val);
	inven_temp->data.todam = tmp_val;

	prt("AC     : ", 1, 1);
	get_string(tmp_str, 1, 10, 10);
	tmp_val = 0;
	sscanf(tmp_str, "%ld", &tmp_val);
	inven_temp->data.ac = tmp_val;

	prt("+To AC : ", 1, 1);
	get_string(tmp_str, 1, 10, 10);
	tmp_val = 0;
	sscanf(tmp_str, "%ld", &tmp_val);
	inven_temp->data.toac = tmp_val;

	prt("Damage : ", 1, 1);
	get_string(tmp_str, 1, 10, 5);
	strcpy(inven_temp->data.damage, tmp_str);

	prt("Level  : ", 1, 1);		/* added code to specify item's */
	get_string(tmp_str, 1, 10, 10); /* level.  --jb 2/5/00 */
	tmp_val = 0;
	sscanf(tmp_str, "%ld", &tmp_val);
	if (tmp_val < 0)
		tmp_val = 0;
	inven_temp->data.level = tmp_val;

	if (get_yes_no("Allocate?")) {
		popt(&tmp_val);
		t_list[tmp_val] = inven_temp->data;
		/* with cave[char_row][char_col]. do; */
		if (cave[char_row][char_col].tptr > 0) {
			delete_object(char_row, char_col);
		}
		cave[char_row][char_col].tptr = tmp_val;
		msg_print("Allocated...");
	} else {
		msg_print("Aborted...");
	}

	inven_temp->data = blank_treasure;
	move_char(5);
	creatures(false);
}

void wizard_help()
{
	/*{ Help for available wizard commands                            }*/

	C_clear_screen();
	prt(" ? -  Wizard Help.", 1, 1);
	prt(" a -  Remove Curse and Cure all maladies.", 2, 1);
	prt(" b -  Print random objects sample.", 3, 1);
	prt(" d -  Down/Up n levels.", 4, 1);
	prt(" e - *Change character.", 5, 1);
	prt(" f - *Delete monsters.", 6, 1);
	prt(" g - *Allocate treasures.", 7, 1);
	prt(" i -  Identify.", 8, 1);
	prt(" j - *Gain experience.", 9, 1);
	prt(" k - *Summon monster.", 10, 1);
	prt(" l -  Wizard light.", 11, 1);
	prt(" n -  Print monster dictionary.", 12, 1);
	prt(" o - *Summon monster by its name.", 13, 1);
	prt(" p -  Wizard password on/off.", 14, 1);
	prt(" s - *Statistics on item (in inventory screen).", 15, 1);
	prt(" t -  Teleport player.", 16, 1);
	prt(" u - *Roll up an item.", 17, 1);
	prt(" v -  Restore lost character.", 18, 1);
	prt(" w - *Create any object *CAN CAUSE FATAL ERROR*", 19, 1);
	prt(" x - *Edit high score file", 20, 1);
	pause_game(24);
	draw_cave();
}

void wizard_command(void)
{
	char tmp_str[82];
	stat_set tstat;
	treas_ptr trash_ptr;
	long y, x;

	prt("Wizard command: ", 1, 1);
	switch (inkey()) {
	case 'a':
		hp_player(1000, "cheating");
		player_cmana = player_mana;
		prt_stat_block();
		remove_curse();
		cure_me(&(PF.blind));
		cure_me(&(PF.hoarse));
		cure_me(&(PF.afraid));
		cure_me(&(PF.poisoned));
		cure_me(&(PF.confused));
		for (tstat = STR; tstat <= CHR; tstat++)
			restore_stat(tstat, "");
		if (PF.slow > 1)
			PF.slow = 1;
		if (PF.image > 1)
			PF.image = 1;
		break;
	case 'b':
		print_objects();
		break;

	case 'd': /* Change dungeon level */
		prt("Go to which level (0 -1200) ? ", 1, 1);
		if (get_string(tmp_str, 1, 31, 10)) {
			long i1 = -1;
			sscanf(tmp_str, "%ld", &i1);
			if (i1 > -1 || !strcmp(tmp_str, "*")) {
				dun_level = i1;
				if (dun_level > 1200) {
					dun_level = 1200;
				} else if (dun_level < 0) {
					dun_level = player_max_lev;
				}
				moria_flag = true;
			} else {
				erase_line(msg_line, msg_line);
			}
		} else {
			erase_line(msg_line, msg_line);
		}
		break;
	case 'e':
		change_character();
		break;
	case 'f':
		mass_genocide();
		break;
	case 'g': /* Treasure */
		alloc_object(floor_set, 5, 25);
		prt_map();
		break;
	case 'i':
		msg_print("Poof!  Your items are all identifed!!!");
		for (trash_ptr = inventory_list; trash_ptr != NULL;) {
			identify(&(trash_ptr->data));
			known2(trash_ptr->data.name);
			trash_ptr = trash_ptr->next;
		}
		break;
	case 'j': /* Gain exp */
		if (player_exp == 0) {
			C_player_add_exp(1);
		} else {
			C_player_add_exp(player_exp);
		}
		prt_stat_block();
		break;
	case 'k': /* Summon monster */
		y = char_row;
		x = char_col;
		if (is_in(cave[y][x].fval, water_set)) {
			summon_water_monster(&y, &x, true);
		} else {
			summon_land_monster(&y, &x, true);
		}
		creatures(false);
		break;
	case 'l':
		wizard_light();
		break;
	case 'n':
		print_monsters();
		break;

	case 'o':
		monster_summon_by_name(char_row, char_col, "", false, true);
		creatures(false);
		break;
	case 't':
		teleport(100);
		break;

	case 'u': /* Summon item */
		if (cave[char_row][char_col].tptr == 0) {
			summon_item(char_row, char_col, "", "", 0, false);
		} else {
			msg_print("You are "
				  "standing on "
				  "something!");
		}
		break;

	case 'w': /* create */
		if (cave[char_row][char_col].tptr == 0) {
			wizard_create();
		} else {
			msg_print("You are "
				  "standing on "
				  "something!");
		}
		break;
	case 27: /* ^3  Run store_maint */
		store_maint();
		msg_print("Stores updated.");
		break;
	case 31: /* ^_  Can you say security through obscurity?
		    */
		if (wizard1 && search_flag && player_cheated) {
			player_cheated = false;
			msg_print("Cheat flag turned off.");
		}
		break;

	case '?':
		wizard_help();
		break;
	}
}
