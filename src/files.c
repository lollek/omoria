#include <sys/file.h> /* for flock     */

#include "imoria.h"
#include "save.h"

FILE *priv_fopen(char *path, char *mode)
{
	FILE *f1;
	/* call fopen bracketed by priv_switch calls */

	priv_switch(1);

	f1 = (FILE *)fopen(path, mode);

	priv_switch(0);

	return f1;
}

void intro_qualifier_help()
{
	printf(		"\n\r"
			"Invalid Moria option!  Valid qualifiers are:\n\r"
			"  -w         Warn about hearing things in water.\n\r"
			"  -s         List top 20 high scores.\n\r"
			"  -t num     List <num> high scores after death or for -s.\n\r"
			"  -V         Print version info.\n\r"
			"\n\r"
			"\n\r");
	printf(		"Wizard commands:\n\r"
			"  -Wpassword Enter wizard mode password.\n\r"
			"  -Rfile     Restore character in master file.\n\r"
			"  -Ufile     Change the dead flag in save file.\n\r"
			"  -Efile     Encrpyt a character file.\n\r"
			"  -Dfile     Decrypt a character file.\n\r"
			"\n\r");
}

void print_version_info()
{
	printf(
			"\n\r"
			"Linux Omoria Version %s\n\r"
			"\n\r"
			, omoria_version());
}

static void intro_parse_switches(int argc, char *argv[])
{
	boolean exit_flag = false;
	boolean print_usage = false;
	boolean show_highs = false;
	boolean show_version = false;

	/* so far there is no filename to open */
	max_score = 20;

	/* parse the command line arguments */
	for (--argc, ++argv; (argc > 0) && (argv[0][0] == '-');
	     --argc, ++argv) {
		switch (argv[0][1]) {
		case 'V':
			/* version info */
			show_version = true;
			break;

		/* print the high scores */
		case 's':
			show_highs = true;
			break;

		/* number of scores to show */
		case 't':
			if (--argc) {
				sscanf((++argv)[0], "%ld", &max_score);
			} else {
				printf("Missing <num> for -t\n\r");
				print_usage = true;
			}
			break;

		/* warn about things in the water */
		case 'w':
			want_warn = true;
			break;

		/* kick into wizard mode */
		case 'W':
			if (argv[0][2] == 0) {
				check_pswd("", false);
			} else {
				check_pswd(&(argv[0][2]), true);
			}
			break;

		default:
			printf("Unknown switch \"%s\"\n\r", argv[0]);
			print_usage = true;
			break;
		}
	}

	if (show_version) {
		print_version_info(); /* not the same as game_version in
					 wizard.c */
		exit_game();
	}

	if (show_highs) {
		C_highscore(max_score);
		exit_game();
	}

	if (print_usage) {
		intro_qualifier_help();
		exit_game();
	}

	if (exit_flag) {
		exit_game();
	}
}

boolean intro_do_hours_file(boolean already_exiting, char *the_file)
{
	/*      { Attempt to read hours.dat.  If it does not exist,     } */
	/*      { then create a standard one.                           } */

	FILE *file1;
	char in_line[82];
	long i1;
	boolean exit_flag = false;

	file1 = priv_fopen(the_file, "r");
	if (file1 != NULL) {
		while (!feof(file1)) {
			in_line[0] = 0;
			fgets(in_line, sizeof(in_line), file1);

			chomp(in_line);
			if (strlen(in_line) > 3) {
				if (!(strncmp("SUN:", in_line, 4))) {
					strncpy(days[0], in_line,
						sizeof(days[0]));
				} else if (!(strncmp("MON:", in_line, 4))) {
					strncpy(days[1], in_line,
						sizeof(days[1]));
				} else if (!(strncmp("TUE:", in_line, 4))) {
					strncpy(days[2], in_line,
						sizeof(days[2]));
				} else if (!(strncmp("WED:", in_line, 4))) {
					strncpy(days[3], in_line,
						sizeof(days[3]));
				} else if (!(strncmp("THR:", in_line, 4))) {
					strncpy(days[4], in_line,
						sizeof(days[4]));
				} else if (!(strncmp("FRI:", in_line, 4))) {
					strncpy(days[5], in_line,
						sizeof(days[5]));
				} else if (!(strncmp("SAT:", in_line, 4))) {
					strncpy(days[6], in_line,
						sizeof(days[6]));
				}
			} /* end if */

		} /* end while */
		fclose(file1);

	} else {
		/* try to create the default hours.dat file */
		priv_switch(0);
		file1 = (FILE *)fopen(the_file, "w");
		if (file1 != NULL) {
			fprintf(file1, "    Moria operating hours are:\n");
			fprintf(file1, "    |    AM     |    player_    |\n");
			fprintf(file1, "    1         111         111\n");
			fprintf(file1, "    2123456789012123456789012\n");
			for (i1 = 0; i1 < 7; i1++) {
				fprintf(file1, "%s\n", days[i1]);
			}
			fprintf(file1, "       (X=Open; .=Closed)\n");
			fclose(file1);
			printf("Created %s\n", the_file);
			exit_flag = true;
		} else {
			printf("Error in creating %s for output.\n\r",
			       the_file);
			exit(1);
		} /* end if file1 (writing) */

	} /* end if file1 (reading) */

	return (exit_flag || already_exiting);
}

boolean intro_do_death_file(boolean already_exiting, char *the_file)
{
	FILE *file1;
	boolean exit_flag = false;

	file1 = priv_fopen(the_file, "r");
	if (file1 != NULL) {
		/* file already exists and can be opened */
		fclose(file1);
	} else {
		/* create a new file */
		file1 = (FILE *)fopen(the_file, "w");
		if (file1 == NULL) {
			printf("Error creating %s\n\r", the_file);
			exit_game();
		} else {
			fprintf(file1, "Moria death log file\n");
			fprintf(file1, "\n");
			fprintf(file1, "Key to abbreviations:\n");
			fprintf(file1, "\n");
			fprintf(file1, "For Race:          For Class:\n");
			fprintf(file1, "  1 = Human          1 = Warrior\n");
			fprintf(file1, "  2 = Half-Elf       2 = Mage\n");
			fprintf(file1, "  3 = Elf            3 = Priest\n");
			fprintf(file1, "  4 = Halfling       4 = Rogue\n");
			fprintf(file1, "  5 = Gnome          5 = Ranger\n");
			fprintf(file1, "  6 = Dwarf          6 = Paladin\n");
			fprintf(file1, "  7 = Half-Orc       7 = Druid\n");
			fprintf(file1, "  8 = Half-Troll     8 = Bard\n");
			fprintf(file1, "  9 = Phraint        9 = Adventurer\n");
			fprintf(file1, " 10 = Dryad         10 = Monk\n");
			fprintf(file1, "\n");
			fprintf(file1, "             Dif Class   Dung Dung\n");
			fprintf(
			    file1,
			    " Username      Race  Lvl Cur  Max  Died from\n");
			fprintf(file1, " ------------ - -- -- -- ---- ---- "
				       "---------------------------------------"
				       "-----\n");

			printf("Created %s\n\r", the_file);
			fclose(file1);
			exit_flag = true;
		} /* end if file1 (writing) */
	}	 /* end if file1 (reading) */

	return (exit_flag || already_exiting);
}

boolean intro_ensure_file_exists(boolean already_exiting, char *the_file)
{
	FILE *file1;
	boolean exit_flag = false;

	file1 = priv_fopen(the_file, "r");
	if (file1 != NULL) {
		/* file already exists and can be opened */
		fclose(file1);
	} else {
		/* create a new file */
		file1 = (FILE *)fopen(the_file, "w");
		if (file1 == NULL) {
			printf("Error creating %s\n\r", the_file);
			exit_game();
		} else {
			printf("Created %s\n\r", the_file);
			fclose(file1);
			exit_flag = true;
		} /* end if file1 (writing) */
	}	 /* end if file1 (reading) */

	return (exit_flag || already_exiting);
}

void intro(int argc, char *argv[])
{
	/* Attempt to open the intro file                        -RAK- */

	char in_line[82];
	FILE *file1;
	boolean exit_flag = false;

	ENTER(("intro", ""));

	/* make sure that various files exist */
	exit_flag = intro_do_hours_file(exit_flag, MORIA_HOU);
	exit_flag = intro_do_death_file(exit_flag, MORIA_DTH);
	exit_flag = intro_ensure_file_exists(exit_flag, MORIA_GCST);

	if (exit_flag) {
		intro_ensure_file_exists(exit_flag, MORIA_TRD);
		printf("\r\n"
			"Notice: System IMORIA wizard should set the protection\r\n"
			"        on  files  just created.  See the README file for\r\n"
			"        help on setting protection on the files.\r\n"
			"        Hint: make privs\r\n"
			"\r\n"
			"Notice: File hours.dat may be edited to set operating\r\n"
			"        hours for IMORIA.\r\n"
			"\r\n"
			"Notice: File moria.dat may be edited to contain  news\r\n"
			"        items, etc...\r\n"
			"\r\n");
		exit_game();
	}

	intro_parse_switches(argc, argv);

	if (!wizard1) {
		no_controly();
	}

	if (!check_time() && !wizard1) {
		/* print out the hours file and exit the game */

		file1 = priv_fopen(MORIA_HOU, "r");
		if (file1 == NULL) {
			printf("Unable to open %s for reading\n\r", MORIA_HOU);
		} else {
			while (!feof(file1)) {
				in_line[0] = 0;
				fgets(in_line, sizeof(in_line), file1);
				printf("%s\r", in_line);
			}
			fclose(file1);
		}
                exit_game();
	}

	LEAVE("intro", "");
}

void file_character()
{
	/*{ Print the character to a file or device               -RAK-   }*/

	long i1, i2, xbth, xbthb, xfos, xsrh, xstl, xdis;
	long xsave, xdev, xswm, xrep;
	char xinfra[82];
	FILE *file1;
	char out_val[82], filename1[82], prt1[82], prt2[82];
	char new_page = 12;
	treas_ptr curse;
	char s1[82], s2[82], s3[82];

	prt("File name: ", 1, 1);
	if (get_string(filename1, 1, 12, 64)) {
		if (filename1[0] == 0) {
			strcpy(filename1, "MORIACHR.DAT");
		}
		file1 = (FILE *)fopen(filename1, "w");
		if (file1 != NULL) {
			prt("Writing character sheet...", 1, 1);
			refresh();

			fprintf(file1, " \n \n \n");
			fprintf(file1, "  Name  : %24s  Age         :%4d     "
				       "Strength     : %d\n",
				player_name, player_age, C_player_get_stat(STR));

			fprintf(file1, "  Race  : %24s  Height      :%4d     "
				       "Intelligence : %d\n",
				player_race, player_ht, C_player_get_stat(INT));

			fprintf(file1, "  Sex   : %24s  Weight      :%4d     "
				       "Wisdom       : %d\n",
				player_sex, player_wt, C_player_get_stat(WIS));

			fprintf(file1, "  Class : %24s  Social Class:%4d     "
				       "Dexterity    : %d\n",
				player_tclass, player_sc, C_player_get_stat(DEX));

			fprintf(file1, "          %24s                       "
				       "Constitution : %d\n",
				"", C_player_get_stat(CON));

			fprintf(file1, "          %24s              %4s      "
				       "Charisma     : %d\n",
				"", "", C_player_get_stat(CHR));

			fprintf(file1, " \n \n \n \n");

			fprintf(file1, "  + To Hit    :%3d        Level      "
				       ":%9d     Max Hit Points :%4d\n",
				player_dis_th, player_lev, C_player_max_hp());

			fprintf(file1, "  + To Damage :%3d        Experience "
				       ":%9ld     Cur Hit Points :%4d\n",
				player_dis_td, player_exp, C_player_current_hp());

			fprintf(file1, "  + To AC     :%3d        Gold       "
				       ":%9ld     Max Mana       :%4d\n",
				player_dis_tac, player_money[TOTAL_],
				player_mana);

			fprintf(file1, "    Total AC  :%3d        Account    "
				       ":%9ld     Cur Mana       :%4d\n",
				player_dis_ac, player_account, player_mana);

			fprintf(file1, " \n \n");

			xbth = player_bth + player_lev * BTH_LEV_ADJ +
			       player_ptohit * BTH_PLUS_ADJ;
			xbthb = player_bthb + player_lev * BTH_LEV_ADJ +
				player_ptohit * BTH_PLUS_ADJ;
			xfos = 27 - player_fos;
			if (xfos < 0) {
				xfos = 0;
			}
			xsrh = C_player_curr_search_skill();
			xstl = player_stl;
			xdis = player_disarm + player_lev + 2 * C_player_disarm_from_dex() +
			       C_player_mod_from_stat(INT);
			xsave = player_save + player_lev + C_player_mod_from_stat(WIS);
			xdev = player_save + player_lev + C_player_mod_from_stat(INT);
			xswm = PF.swim + 4;
			xrep = 6 + player_rep / 25;
			sprintf(xinfra, "%ld feet", PF.see_infra);

			fprintf(file1, "\n");

			fprintf(file1, "%50s\n \n",
				"(Miscellaneous Abilities)");

			fprintf(file1, "  Fighting    : %10s  Stealth     : "
				       "%10s  Perception  : %10s\n",
				likert(xbth, 12, s1), likert(xstl, 1, s2),
				likert(xfos, 3, s3));

			fprintf(file1, "  Throw/Bows  : %10s  Disarming   : "
				       "%10s  Searching   : %10s\n",
				likert(xbthb, 12, s1), likert(xdis, 8, s2),
				likert(xsrh, 6, s3));

			fprintf(file1, "  Saving Throw: %10s  Magic Device: "
				       "%10s  Infra-Vision: %10s\n",
				likert(xsave, 6, s1), likert(xdev, 7, s2),
				xinfra);

			fprintf(file1, "  Reputation  : %10s                   "
				       "         Swimming    : %10s\n",
				likert(xrep, 1, s1), likert(xswm, 1, s3));

			/*{ Write out the character's history     }*/

			fprintf(file1, " \n \n%45s\n", "Character Background");
			for (i1 = 0; i1 < 5; i1++) {
				fprintf(file1, "   %s\n", player_history[i1]);
			}

			/*{ Write out the time stats              }*/

			fprintf(file1, " \n \n");

			/* with player_birth. do; */
			day_of_week_string(player_birth.day, 10, out_val);

			for (i1 = 0; out_val[i1];) {
				if (out_val[i1] == ' ') {
					out_val[i1] = 0;
				} else {
					i1++;
				}
			}

			fprintf(
			    file1,
			    "  You were born at %s on %s, %s the %s, %ld AH.\n",
			    time_string(player_birth.hour, player_birth.secs, s1),
			    out_val, month_string(player_birth.month, s2),
			    place_string(player_birth.day, s3), player_birth.year);

			fprintf(file1, "  %s\n", show_char_age(s1));
			fprintf(file1, "  The current time is %s.\n",
				full_date_string(player_cur_age, s1));
			if (player_flags.dead) {
				fprintf(
				    file1,
				    "  You were killed by %s on level %ld.\n",
				    died_from, dun_level);
			}
			fprintf(file1, "  Maximum depth was %d feet.\n",
				player_max_lev * 50);

			/*{ Write out the equipment list...       }*/
			fprintf(file1,
				" \n \n  [Character's Equipment List]\n \n \n");
			i2 = 0;

			if (equip_ctr == 0) {
				fprintf(
				    file1,
				    "  Character has no equipment in use.\n");
			} else {
				for (i1 = Equipment_min; i1 < EQUIP_MAX; i1++) {
					/* with equipment[i1]. do; */
					if (equipment[i1].tval > 0) {
						switch (i1) {
						case Equipment_primary:
							strcpy(prt1,
							       ") You are "
							       "wielding   : ");
							break;
						case Equipment_helm:
							strcpy(prt1, ") Worn "
								     "on head  "
								     "     : ");
							break;
						case Equipment_amulet:
							strcpy(prt1,
							       ") Worn around "
							       "neck   : ");
							break;
						case Equipment_armor:
							strcpy(prt1, ") Worn "
								     "on body  "
								     "     : ");
							break;
						case Equipment_belt:
							strcpy(prt1,
							       ") Worn around "
							       "body   : ");
							break;
						case Equipment_shield:
							strcpy(prt1,
							       ") Worn on "
							       "shield arm : ");
							break;
						case Equipment_gloves:
							strcpy(prt1, ") Worn "
								     "on hands "
								     "     : ");
							break;
						case Equipment_bracers:
							strcpy(prt1,
							       ") Worn on "
							       "wrists     : ");
							break;
						case Equipment_right_ring:
							strcpy(prt1,
							       ") Right ring "
							       "finger  : ");
							break;
						case Equipment_left_ring:
							strcpy(prt1,
							       ") Left  ring "
							       "finger  : ");
							break;
						case Equipment_boots:
							strcpy(prt1, ") Worn "
								     "on feet  "
								     "     : ");
							break;
						case Equipment_cloak:
							strcpy(prt1,
							       ") Worn about "
							       "body    : ");
							break;
						case Equipment_light:
							strcpy(prt1,
							       ") Light source "
							       "is    : ");
							break;
						case Equipment_secondary:
							strcpy(prt1,
							       ") Secondary "
							       "weapon   : ");
							break;
						default:
							strcpy(prt1,
							       ") *Unknown "
							       "equipment*: ");
							break;
						}
						i2++;
						inven_temp->data =
						    equipment[i1];
						objdes(prt2, inven_temp, true);
						if ((inven_temp->data.flags2 &
						     Insured_bit) == 0) {
							sprintf(out_val,
								"  %c%s%s",
								(char)(i2 + 96),
								prt1, prt2);
						} else {
							sprintf(out_val,
								" (%c%s%s",
								(char)(i2 + 96),
								prt1, prt2);
						}
						fprintf(file1, "%s\n", out_val);
					}
				} /* end for equipment */
			}	 /* end else has equipment */

			/*{ Write out the character's inventory...        }*/

			fprintf(file1, "%c \n \n \n  %s\n \n \n", new_page,
				"[General Inventory List]");

			if (inven_ctr == 0) {
				fprintf(file1, "  Character has no objects in "
					       "inventory.\n");
			} else {
				i1 = 1;
				for (curse = inventory_list; curse != nil;
				     curse = curse->next) {
					if ((i1 % 50) == 0) {
						fprintf(file1, "%c\n \n \n \n  "
							       "%s %ld]\n \n",
							new_page, "[General "
								  "Inventory "
								  "List, Page",
							(i1 / 50) + 1);
					}
					inven_temp->data = curse->data;
					objdes(s1, inven_temp, true);
					if (curse->is_in) {
						sprintf(prt1, "    %s", s1);
					} else {
						sprintf(prt1, "%s", s1);
					}

					if (i1 < 27) {
						if ((inven_temp->data.flags2 &
						     Insured_bit) == 0) {
							sprintf(out_val,
								" %c) %s",
								(char)(i1 + 96),
								prt1);
						} else {
							sprintf(out_val,
								"(%c) %s",
								(char)(i1 + 96),
								prt1);
						}
					} else {
						if ((inven_temp->data.flags2 &
						     Insured_bit) == 0) {
							sprintf(out_val,
								" *) %s", prt1);
						} else {
							sprintf(out_val,
								"(*) %s", prt1);
						}
					}

					if ((inven_temp->data.flags2 &
					     Holding_bit) != 0) {
						bag_descrip(curse, s1);
						strcat(out_val, s1);
					}

					fprintf(file1, "%s\n", out_val);
					i1++;
				} /* end for inventory */
			}	 /* end else inventory not null */

			fclose(file1);
			prt("Completed.", 1, 1);
		} /* end if file !NULL */
	}	 /* end get_string */
}

void print_map()
{
	/*{ Prints dungeon map to external file                       -RAK- }*/

	long i1, i2, i3, i4, i5, i6, i7, i8;
	char dun_line[OUTPAGE_WIDTH + 10];
	char filename1[81];
	char tmp;
	FILE *file1;

	prt("File name: ", 1, 1);
	if (get_string(filename1, 1, 12, 64)) {
		if (strlen(filename1) == 0) {
			strcpy(filename1, "MORIAMAP.DAT");
		}
		file1 = (FILE *)fopen(filename1, "w");
		if (file1 != NULL) {
			prt("Writing Moria Dungeon Map...", 1, 1);
			refresh();
			i1 = 1;
			i7 = 0;
			do {
				i2 = 1;
				i3 = i1 + OUTPAGE_HEIGHT - 1;
				if (i3 > cur_height) {
					i3 = cur_height;
				}
				i7++;
				i8 = 0;
				do {
					i4 = i2 + OUTPAGE_WIDTH - 1;
					if (i4 > cur_width) {
						i4 = cur_width;
					}
					i8++;
					fprintf(file1, "%c", 12);
					fprintf(file1, "Section[%ld,%ld];     ",
						i7, i8);
					fprintf(file1, "Depth : %ld (feet)\n",
						(dun_level * 50));
					fprintf(file1, " \n");
					fprintf(file1, "   ");
					for (i5 = i2; i5 <= i4; i5++) {
						i6 = i5 / 100;
						fprintf(file1, "%ld", i6);
					}
					fprintf(file1, "\n");
					fprintf(file1, "   ");
					for (i5 = i2; i5 <= i4; i5++) {
						i6 = (int)(i5 / 10) -
						     (int)(i5 / 100) * 10;
						fprintf(file1, "%ld", i6);
					}
					fprintf(file1, "\n");
					fprintf(file1, "   ");
					for (i5 = i2; i5 <= i4; i5++) {
						i6 = i5 - (int)(i5 / 10) * 10;
						fprintf(file1, "%ld", i6);
					}
					fprintf(file1, "\n");
					for (i5 = i1; i5 < i3; i5++) {
						sprintf(dun_line, "%3ld", i5);
						for (i6 = i2; i6 <= i4; i6++) {
							if (test_light(i5,
								       i6)) {
								tmp =
								    loc_symbol(
									i5,
									i6) &
								    0x7F;
							} else {
								tmp = ' ';
							}
							strncat(dun_line, &tmp,
								1);
						}
						fprintf(file1, "%s\n",
							dun_line);
					}
					i2 += OUTPAGE_WIDTH;
				} while (i2 < cur_width);
				i1 += OUTPAGE_HEIGHT;
			} while (i1 < cur_height);
			fclose(file1);
			prt("Completed.", 1, 1);
		}
	}
}

void print_objects()
{
	/*{ Prints a list of random objects to a file.  Note that   -RAK-  }*/
	/*{ the objects produced is a sampling of objects which            }*/
	/*{ be expected to appear on that level.                           }*/

	long nobj, i1, i2, level;
	char filename1[82], tmp_str[82];
	FILE *file1;

	level = 0;
	nobj = 0;

	prt("Produce objects on what level?: ", 1, 1);
	if (get_string(tmp_str, 1, 33, 10)) {
		sscanf(tmp_str, "%ld", &level);
	}
	prt("Produce how many objects?: ", 1, 1);
	if (get_string(tmp_str, 1, 28, 10)) {
		sscanf(tmp_str, "%ld", &nobj);
	}
	if ((nobj > 0) && (level > -1) && (level < 1201)) {
		if (nobj > 99999) {
			nobj = 99999;
		}
		prt("File name: ", 1, 1);
		if (get_string(filename1, 1, 12, 64)) {
			if (filename1[0] == 0) {
				strcpy(filename1, "MORIAOBJ.DAT");
			}
			file1 = (FILE *)fopen(filename1, "w");
			if (file1 != NULL) {
				sprintf(tmp_str,
					"%ld random objects being produced...",
					nobj);
				prt(tmp_str, 1, 1);
				refresh();

				fprintf(file1, "*** Random Object Sampling:\n");
				fprintf(file1, "*** %ld objects\n", nobj);
				fprintf(file1, "*** For Level %ld\n\n\n",
					level);
				popt(&i2);

				for (i1 = 1; i1 <= nobj; i1++) {
					t_list[i2] = object_list[get_obj_num(
					    level, PLACE_OBJECT_TRIES)];
					magic_treasure(i2, level, false);
					inven_temp->data = t_list[i2];
					/* with inven_temp->data. do; */
					unquote(inven_temp->data.name);
					known1(inven_temp->data.name);
					known2(inven_temp->data.name);
					objdes(tmp_str, inven_temp, true);
					fprintf(file1, "%s\n", tmp_str);
				}
				pusht(i2);
				fclose(file1);
				prt("Completed.", 1, 1);
			} else {
				prt("File could not be opened.", 1, 1);
			}
		}
	}
}

void print_monsters()
{
	/*{ Prints a listing of monsters                              -RAK- }*/

	long i1, atype, adesc, acount, i5;
	char out_val[82], filename1[82];
	char attstr[82], attx[82], s1[82];
	char *achar;
	char damstr[36];
	FILE *file1;
	unsigned long cmove, cdefense, spells;

	prt("File name for monsters: ", 1, 1);
	if (get_string(filename1, 1, 25, 64)) {
		if (filename1[0] == 0) {
			strcpy(filename1, "MORIAMON.DAT");
		}
		file1 = (FILE *)fopen(filename1, "w");
		if (file1 != NULL) {
			prt("Writing Monster Dictionary...", 1, 1);
			refresh();
			for (i1 = 1; i1 <= MAX_CREATURES; i1++) {
				/* with c_list[i1]. do; */

				cmove = c_list[i1].cmove;
				cdefense = c_list[i1].cdefense;
				spells = c_list[i1].spells;

				/*{ Begin writing to file }*/
				fprintf(file1, "-------------------------------"
					       "-------------\n");

				fprintf(file1, "%3ld  %30s     (%c)\n", i1,
					c_list[i1].name, c_list[i1].cchar);

				fprintf(file1, "     Speed =%2d  Level     "
					       "=%2d  Exp =%5ld\n",
					c_list[i1].speed, c_list[i1].level,
					c_list[i1].mexp);

				fprintf(file1, "     AC    =%2d  Eye-sight "
					       "=%2d  HD  =%5s\n",
					c_list[i1].ac, c_list[i1].aaf,
					c_list[i1].hd);

				if ((0x80000000 & cmove) != 0) {
					fprintf(file1, "     Creature is a "
						       "***Win Creature***\n");
				}
				if ((0x00080000 & cmove) != 0) {
					fprintf(file1, "     Creature "
						       "Eats/kills other "
						       "creatures.\n");
				}
				if ((0x00004000 & cmove) != 0) {
					fprintf(file1, "     Creature is good "
						       "(negative "
						       "experience)\n");
				}
				if ((0x00008000 & cmove) != 0) {
					fprintf(file1, "     Creature will not "
						       "normally appear.\n");
				}
				if ((0x0001 & cdefense) != 0) {
					fprintf(file1,
						"     Creature is a dragon.\n");
				}
				if ((0x0002 & cdefense) != 0) {
					fprintf(
					    file1,
					    "     Creature is a monster.\n");
				}
				if ((0x0400 & cdefense) != 0) {
					fprintf(file1,
						"     Creature is a demon.\n");
				}
				if ((0x0004 & cdefense) != 0) {
					fprintf(file1,
						"     Creature is evil.\n");
				}
				if ((0x0008 & cdefense) != 0) {
					fprintf(file1,
						"     Creature is undead.\n");
				}
				if ((0x0010 & cdefense) != 0) {
					fprintf(
					    file1,
					    "     Creature harmed by cold.\n");
				}
				if ((0x0020 & cdefense) != 0) {
					fprintf(
					    file1,
					    "     Creature harmed by fire.\n");
				}
				if ((0x0040 & cdefense) != 0) {
					fprintf(file1, "     Creature harmed "
						       "by poison.\n");
				}
				if ((0x0080 & cdefense) != 0) {
					fprintf(
					    file1,
					    "     Creature harmed by acid.\n");
				}
				if ((0x0100 & cdefense) != 0) {
					fprintf(file1, "     Creature harmed "
						       "by blue light.\n");
				}
				if ((0x0200 & cdefense) != 0) {
					fprintf(file1, "     Creature harmed "
						       "by Stone-to-Mud.\n");
				}
				if ((0x1000 & cdefense) != 0) {
					fprintf(file1, "     Creature cannot "
						       "be charmed or "
						       "slept.\n");
				}
				if ((0x2000 & cdefense) != 0) {
					fprintf(file1, "     Creature seen "
						       "with Infra-Vision.\n");
				}
				if ((0x4000 & cdefense) != 0) {
					fprintf(file1, "     Creature has MAX "
						       "hit points.\n");
				}
				if ((0x8000 & cdefense) != 0) {
					fprintf(file1,
						"     Creature regenerates.\n");
				}
				if ((0x00010000 & cmove) != 0) {
					fprintf(
					    file1,
					    "     Creature is invisible.\n");
				}
				if ((0x00100000 & cmove) != 0) {
					fprintf(file1, "     Creature picks up "
						       "objects.\n");
				}
				if ((0x00200000 & cmove) != 0) {
					fprintf(file1,
						"     Creature multiplies.\n");
				}
				if ((0x01000000 & cmove) != 0) {
					fprintf(file1,
						"     Carries object(s).\n");
				}
				if ((0x02000000 & cmove) != 0) {
					fprintf(
					    file1,
					    "     Carries gold, gems, ect.\n");
				}
				if ((0x04000000 & cmove) != 0) {
					fprintf(file1, "       Has object/gold "
						       "60%% of time.\n");
				}
				if ((0x08000000 & cmove) != 0) {
					fprintf(file1, "       Has object/gold "
						       "90%% of time.\n");
				}
				if ((0x10000000 & cmove) != 0) {
					fprintf(
					    file1,
					    "       Has 1d2 object(s)/gold.\n");
				}
				if ((0x20000000 & cmove) != 0) {
					fprintf(
					    file1,
					    "       Has 2d2 object(s)/gold.\n");
				}
				if ((0x40000000 & cmove) != 0) {
					fprintf(
					    file1,
					    "       Has 4d2 object(s)/gold.\n");
				}

				/*{ Creature casts spells / Breaths Dragon
				 * breath...      }*/
				if (spells > 0) {

					fprintf(
					    file1,
					    "   --Spells/Dragon Breath =\n");
					if ((spells & 0x80000000) != 0) {
						fprintf(file1, "       Doesn't "
							       "cast spells 1 "
							       "out of %ld "
							       "turns.\n",
							(0xF & spells));
					} else {
						fprintf(file1,
							"       Casts spells 1 "
							"out of %ld turns.\n",
							(0xF & spells));
					}

					if ((0x00000010 & spells) != 0) {
						fprintf(file1, "       Can "
							       "teleport "
							       "short.\n");
					}
					if ((0x00000020 & spells) != 0) {
						fprintf(file1, "       Can "
							       "teleport "
							       "long.\n");
					}
					if ((0x00000040 & spells) != 0) {
						fprintf(file1,
							"       Teleport "
							"player to itself.\n");
					}
					if ((0x00000080 & spells) != 0) {
						fprintf(file1, "       Cause "
							       "light "
							       "wounds.\n");
					}
					if ((0x00000100 & spells) != 0) {
						fprintf(file1, "       Cause "
							       "serious "
							       "wounds.\n");
					}
					if ((0x00000200 & spells) != 0) {
						fprintf(
						    file1,
						    "       Hold person.\n");
					}
					if ((0x00000400 & spells) != 0) {
						fprintf(file1, "       Cause "
							       "blindness.\n");
					}
					if ((0x00000800 & spells) != 0) {
						fprintf(file1, "       Cause "
							       "confusion.\n");
					}
					if ((0x00001000 & spells) != 0) {
						fprintf(file1,
							"       Cause fear.\n");
					}
					if ((0x00002000 & spells) != 0) {
						fprintf(file1, "       Summon "
							       "a monster.\n");
					}
					if ((0x00004000 & spells) != 0) {
						fprintf(file1, "       Summon "
							       "an undead.\n");
					}
					if ((0x00008000 & spells) != 0) {
						fprintf(
						    file1,
						    "       Slow person.\n");
					}
					if ((0x00010000 & spells) != 0) {
						fprintf(file1, "       Drains "
							       "mana for "
							       "healing.\n");
					}
					if ((0x00020000 & spells) != 0) {
						fprintf(file1, "       Shadow "
							       "Breath/Orb of "
							       "draining.\n");
					}
					if ((0x00040000 & spells) != 0) {
						fprintf(file1, "       Breaths "
							       "Petrifying "
							       "Gas\n");
					}
					if ((0x00080000 & spells) != 0) {
						fprintf(file1, "       Breaths "
							       "Lightning "
							       "Dragon "
							       "Breath.\n");
					}
					if ((0x00100000 & spells) != 0) {
						fprintf(file1, "       Breaths "
							       "Gas Dragon "
							       "Breath.\n");
					}
					if ((0x00200000 & spells) != 0) {
						fprintf(file1, "       Breaths "
							       "Acid Dragon "
							       "Breath.\n");
					}
					if ((0x00400000 & spells) != 0) {
						fprintf(file1, "       Breaths "
							       "Frost Dragon "
							       "Breath.\n");
					}
					if ((0x00800000 & spells) != 0) {
						fprintf(file1, "       Breaths "
							       "Fire Dragon "
							       "Breath.\n");
					}
					if ((0x01000000 & spells) != 0) {
						fprintf(
						    file1,
						    "       Casts Illusion.\n");
					}
					if ((0x02000000 & spells) != 0) {
						fprintf(
						    file1,
						    "       Summon a demon.\n");
					}
					if ((0x04000000 & spells) != 0) {
						fprintf(file1, "       Summon "
							       "multiplying "
							       "monster.\n");
					}
					if ((0x08000000 & spells) != 0) {
						fprintf(file1,
							"       Gaze for "
							"petrification.\n");
					}
				} /* end if spells > 0 */

				/*{ Movement for creature }*/
				fprintf(file1, "   --Movement =\n");
				if ((0x00000001 & cmove) != 0) {
					fprintf(
					    file1,
					    "       Move only to attack.\n");
				}
				if ((0x00000002 & cmove) != 0) {
					fprintf(
					    file1,
					    "       20%% random movement.\n");
				}
				if ((0x00000004 & cmove) != 0) {
					fprintf(
					    file1,
					    "       40%% random movement.\n");
				}
				if ((0x00000008 & cmove) != 0) {
					fprintf(
					    file1,
					    "       75%% random movement.\n");
				}
				if ((0x00400000 & cmove) != 0) {
					fprintf(file1, "      Creature can "
						       "anchor in water.\n");
				}
				if ((0x00800000 & cmove) != 0) {
					fprintf(file1,
						"       Creature flies.\n");
				}
				if ((0x00000010 & cmove) != 0) {
					fprintf(file1, "       Creature is "
						       "water based.\n");
				}
				if ((0x00000040 & cmove) == 0) {
					fprintf(file1, "       Survives in "
						       "land and water.\n");
				}
				if ((0x00020000 & cmove) != 0) {
					fprintf(file1,
						"       Can open doors.\n");
				}
				if ((0x00040000 & cmove) != 0) {
					fprintf(file1, "       Can phase "
						       "through walls.\n");
				}

				fprintf(file1, "   --Creature attacks =\n");
				attstr[0] = 0;
				attx[0] = 0;
				strcpy(attstr, c_list[i1].damage);

				for (; attstr[0] != 0;) {

					/* attstr looks like this: "1 32 4d4|2
					 * 21 0d0" */

					achar = strstr(attstr, "|");
					if (achar != NULL) {
						strcpy(attx, attstr);
						achar = strstr(attx, "|");
						(*achar) = 0;
						achar++;
						strcpy(attstr, achar);
					} else {
						strcpy(attx, attstr);
						attstr[0] = 0;
					}

					sscanf(attx, "%ld %ld %s", &atype,
					       &adesc, damstr);
					out_val[0] = 0;

					if ((achar = strstr(damstr, "-")) !=
					    NULL) {
						(*achar) = ' ';
						sscanf(damstr, "%ld %s",
						       &acount, s1);
						strcpy(damstr, s1);
					} else {
						acount = 1;
					}

					for (i5 = 1; i5 <= acount; i5++) {
						switch (adesc) {
						case 0:
							strcpy(out_val,
							       "       < No "
							       "Print > for ");
							break;
						case 1:
							strcpy(
							    out_val,
							    "       Hits for ");
							break;
						case 2:
							strcpy(out_val,
							       "       Bites "
							       "for ");
							break;
						case 3:
							strcpy(out_val,
							       "       Claws "
							       "for ");
							break;
						case 4:
							strcpy(out_val,
							       "       Stings "
							       "for ");
							break;
						case 5:
							strcpy(out_val,
							       "       Touches "
							       "for ");
							break;
						case 6:
							strcpy(out_val,
							       "       Kicks "
							       "for ");
							break;
						case 7:
							strcpy(out_val,
							       "       Gazes "
							       "for ");
							break;
						case 8:
							strcpy(out_val,
							       "       "
							       "Breathes for ");
							break;
						case 9:
							strcpy(out_val,
							       "       Spits "
							       "for ");
							break;
						case 10:
							strcpy(out_val,
							       "       Wails "
							       "for ");
							break;
						case 11:
							strcpy(out_val,
							       "       "
							       "Embraces for ");
							break;
						case 12:
							strcpy(out_val,
							       "       Crawls "
							       "on you for ");
							break;
						case 13:
							strcpy(out_val,
							       "       Shoots "
							       "spores for ");
							break;
						case 14:
							strcpy(out_val,
							       "       Begs "
							       "for money "
							       "for ");
							break;
						case 15:
							strcpy(out_val,
							       "       Slimes "
							       "you for ");
							break;
						case 16:
							strcpy(out_val,
							       "       Crushes "
							       "you for ");
							break;
						case 17:
							strcpy(out_val,
							       "       "
							       "Tramples you "
							       "for ");
							break;
						case 18:
							strcpy(out_val,
							       "       Drools "
							       "on you for ");
							break;
						case 19:
							strcpy(out_val,
							       "       Insults "
							       "you for ");
							break;
						case 20:
							strcpy(out_val,
							       "       UW's "
							       "you for ");
							break;
						case 21:
							strcpy(out_val,
							       "       DMF's "
							       "you for ");
							break;
						case 22:
							strcpy(out_val,
							       "       "
							       "Cultivates you "
							       "for ");
							break;
						case 23:
							strcpy(out_val,
							       "       Charms "
							       "you for ");
							break;
						case 24:
							strcpy(out_val,
							       "       Kisses "
							       "you for ");
							break;
						case 25:
							strcpy(out_val,
							       "       Gores "
							       "you for ");
							break;
						case 26:
							strcpy(out_val,
							       "       Moo's "
							       "you for ");
							break;
						case 27:
							strcpy(out_val,
							       "       "
							       "Electrocutes "
							       "you for ");
							break;
						case 28:
							strcpy(out_val,
							       "       Inks "
							       "you for ");
							break;
						case 29:
							strcpy(out_val, "      "
									" Entan"
									"gles "
									"for ");
							break;
						case 30:
							strcpy(out_val,
							       "       Sucks "
							       "blood for ");
							break;
						case 31:
							strcpy(out_val,
							       "       Throat "
							       "attacks for ");
							break;
						case 32:
							strcpy(out_val,
							       "       Blows "
							       "bubbles for ");
							break;
						case 33:
							strcpy(out_val,
							       "       Squawks "
							       "for ");
							break;
						case 34:
							strcpy(out_val,
							       "       Pecks "
							       "for ");
							break;
						case 35:
							strcpy(out_val,
							       "       Barks "
							       "for ");
							break;
						case 36:
							strcpy(out_val,
							       "       Rubs "
							       "leg for ");
							break;
						case 37:
							strcpy(out_val,
							       "       Follows "
							       "for ");
							break;

						case 99:
							strcpy(out_val,
							       "       Is "
							       "repelled...");
							break;
						default:
							sprintf(out_val,
								"   ** Unknown "
								"attack: %ld "
								"** ",
								adesc);
							break;
						} /* end switch */

						switch (atype) {
						case 1:
							strcat(
							    out_val,
							    "normal damage.");
							break;
						case 2:
							strcat(out_val,
							       "lowering "
							       "strength.");
							break;
						case 3:
							strcat(out_val,
							       "confusion.");
							break;
						case 4:
							strcat(out_val,
							       "fear.");
							break;
						case 5:
							strcat(out_val,
							       "fire damage.");
							break;
						case 6:
							strcat(out_val,
							       "acid damage.");
							break;
						case 7:
							strcat(out_val,
							       "cold damage.");
							break;
						case 8:
							strcat(out_val,
							       "lightning "
							       "damage.");
							break;
						case 9:
							strcat(out_val,
							       "corrosion "
							       "damage.");
							break;
						case 10:
							strcat(out_val,
							       "blindness.");
							break;
						case 11:
							strcat(out_val,
							       "paralyzation.");
							break;
						case 12:
							strcat(
							    out_val,
							    "stealing money.");
							break;
						case 13:
							strcat(
							    out_val,
							    "stealing object.");
							break;
						case 14:
							strcat(
							    out_val,
							    "poison damage.");
							break;
						case 15:
							strcat(
							    out_val,
							    "lose dexterity.");
							break;
						case 16:
							strcat(out_val,
							       "lose "
							       "constitution.");
							break;
						case 17:
							strcat(out_val,
							       "lose "
							       "intelligence.");
							break;
						case 18:
							strcat(out_val,
							       "lose wisdom.");
							break;
						case 19:
							strcat(
							    out_val,
							    "lose experience.");
							break;
						case 20:
							strcat(out_val,
							       "aggravates "
							       "monsters.");
							break;
						case 21:
							strcat(out_val,
							       "disenchants "
							       "objects.");
							break;
						case 22:
							strcat(out_val,
							       "eating food.");
							break;
						case 23:
							strcat(out_val,
							       "eating light "
							       "source.");
							break;
						case 24:
							strcat(out_val,
							       "absorbing "
							       "charges.");
							break;
						case 25:
							strcat(
							    out_val,
							    "lose charisma.");
							break;
						case 26:
							strcat(
							    out_val,
							    "petrification.");
							break;
						case 27:
							strcat(out_val,
							       "poision.");
							break;
						case 99:
							strcat(
							    out_val,
							    "blank message.");
							break;
						default:
							sprintf(out_val,
								"** Unknown "
								"damage: %ld**",
								atype);
							break;
						} /* end switch */

						fprintf(file1, "%s (%s)\n",
							out_val, damstr);

					} /* end acount */
				}	 /* end for attstr */

				fprintf(file1, "   --Magic Resistance : ");
				if (c_list[i1].mr == 0) {
					fprintf(file1, "None\n");
				} else if (c_list[i1].mr < 20) {
					fprintf(file1, "Very Low\n");
				} else if (c_list[i1].mr < 50) {
					fprintf(file1, "Low\n");
				} else if (c_list[i1].mr < 80) {
					fprintf(file1, "Medium\n");
				} else if (c_list[i1].mr < 110) {
					fprintf(file1, "High\n");
				} else if (c_list[i1].mr < 140) {
					fprintf(file1, "Very High\n");
				} else {
					fprintf(file1, "Extreme\n");
				}

				fprintf(file1, " \n \n");

			} /* end for i1 */

			fclose(file1);
			prt("Completed.", 1, 1);

		} /* end file1 != NULL */
	}	 /* end get filename */
}

boolean read_top_scores(FILE **f1, char *fnam, char list[][134], int max_high,
			int *n1, char *openerr)
{
	/* list[] should be declared string list[max_high+1] */
	/* be sure to unlock the file when finished          */

	boolean file_flag = false;
	boolean paniced;
	char temp[1026];
	int i1, trys;

	encrypt_state hs_state;

	for (i1 = 1; i1 <= max_high; i1++) {
		list[i1][0] = 0;
	}

	openerr[0] = 0;
	*n1 = 0;

	*f1 = priv_fopen(fnam, "r+");

	if (*f1 == NULL) {
		sprintf(openerr, "Error opening> %s", fnam);
	} else {

		/* get exclusive access to the high score file */

		for (trys = 0; (trys < 5) && !file_flag; trys++) {
			if (flock((int)fileno(*f1), LOCK_EX | LOCK_NB) == 0) {
				file_flag = true;
			} else {
				sleep(1);
			}
		}

		if (!file_flag) {
			fclose(*f1);
			*f1 = NULL;
			strcpy(openerr, "Error gaining lock for score file.");
		} else {
			for (paniced = false, *n1 = 1;
			     !paniced && (*n1 <= max_high);) {
				read_decrypt(*f1, &hs_state, temp, &paniced);
				if (!paniced) {
					if (strlen(temp) > 80) {
						temp[80] = 0;
					}
					strcpy(list[*n1], temp);
					(*n1)++;
				}
			}
		}
	}

	(*n1)--;
	return file_flag;
}

boolean write_top_scores(FILE **f1, char list[][134], int max_high)
{
	int i1;
	char temp[82];
	encrypt_state hs_state;

	rewind(*f1);
	ftruncate((int)fileno(*f1), 0);

	for (i1 = 1; i1 <= max_high; i1++) {
		strcpy(temp, list[i1]);
		encrypt_write(*f1, &hs_state, temp);
	}

	encrypt_flush(*f1, &hs_state);
	return true;
}

boolean close_top_scores(FILE **f1)
{
	boolean return_value = true;

	if (flock((int)fileno(*f1), LOCK_UN) != 0) {
		return_value = false;
	}
	fclose(*f1);

	return return_value;
}

char *center(char *in_str, long str_len, char *out_str)
{
	int i, j;

	i = strlen(in_str);
	j = (str_len - i) / 2;

	if (i >= str_len) {
		strncpy(out_str, in_str, str_len);
	} else {
		sprintf(out_str, "%*s%s%*s", j, "", in_str,
			(int)str_len - i - j, "");
	}

	return out_str;
}

boolean open_crypt_file(char prompt[82], char fnam1[82], char fnam2[82], FILE **f1,
			FILE **f2)
{
	char out_str[1026];
	boolean flag = true;

	if (fnam1[0] == 0) {
		prt(prompt, 1, 1);
		flag = get_string(fnam1, 1, strlen(prompt) + 1, 80);
	}

	if (flag) {

		*f1 = (FILE *)fopen(fnam1, "r");
		if (*f1 == NULL) {
			sprintf(out_str, "Error Opening> %s", fnam1);
			prt(out_str, 1, 1);
			prt(" ", 2, 1);
			flag = false;
		} else {
			prt("Name of output file: ", 1, 1);
			flag = get_string(fnam2, 1, 21, 80);

			if (flag) {
				if (strcmp(fnam1, fnam2) != 0) {
					*f2 = (FILE *)fopen(fnam2, "w");
					if (*f2 == NULL) {
						sprintf(out_str,
							"Error Opening> %s",
							fnam2);
						prt(out_str, 1, 1);
						prt(" ", 2, 1);
						flag = false;
					}
				} else {
					sprintf(out_str,
						"Use differnt names> %s",
						fnam2);
					prt(out_str, 1, 1);
					prt(" ", 2, 1);
					flag = false;
				}
			}
			if (!flag) {
				fclose(*f1);
			}
		}
	}
	refresh();
	return flag;
}
