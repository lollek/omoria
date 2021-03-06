/* creature.c */
/**/

#include "imoria.h"
#include "dungeon.h"

void lm__read_custom(FILE *file)
{
	long count;
	char a[28];

	if (!feof(file)) {
		do {
			a[0] = 0;
			fgets(a, sizeof(char[28]), file); /* new name */
			if ((count = strlen(a)) > 1) {
				a[count - 1] = 0; /* strip newline */
				fscanf(file, "%ld\n", &count);
				if ((count <= MAX_CREATURES) && (count > 0)) {
					strncpy(c_list[count].name, a,
						sizeof(char[28]) - 1);
				}
			}
		} while (!feof(file));
	}
}

void load_monsters()
{
	/*
	  load all them monsters from monsters.dat

	  file format is like this:  see monsters.info for details

	  --- 001 ---
	  10
	  1
	  Town Wizard
	  0010C000
	  00009F52
	  3000
	  25000
	  50
	  1
	  p
	  20d8
	  1 1 3d3
	  0
	  20
	  */

	long count;
	char a[28];
	int value;
	FILE *file;

	/*////////////////////////////////////////////////////////////////////
	 */
	ENTER(("load_monsters", ""));

	file = priv_fopen(MORIA_MON, "r");
	if (file == NULL) {
		printf("\nUnable to open monster file for reading: %s\n\r",
		       MORIA_MON);
		exit_game();
	}

	for (count = 1; count <= MAX_CREATURES; count++) {
		/* --- 001 --- */
		if (fgets(a, sizeof(char[28]), file) == NULL) {
			count--;
			break;
		}

		/* aaf */
		fscanf(file, "%d\n", &value);
		c_list[count].aaf = (unsigned char)value;

		/* ac */
		fscanf(file, "%d\n", &value); 
		c_list[count].ac = (unsigned char)value;

		/* name */
		fgets(c_list[count].name, sizeof(char[28]), file);
		/* strip newline */
		c_list[count].name[strlen(c_list[count].name) - 1] = 0;

		/* cmove */
		fscanf(file, "%lx", &(c_list[count].cmove));

		/* spells */
		fscanf(file, "%lx", &(c_list[count].spells));

		/* cdefense */
		fscanf(file, "%lx", &(c_list[count].cdefense));

		/* sleep */
		fscanf(file, "%d\n", &value);
		c_list[count].sleep = (short)value;

		/* mexp */
		fscanf(file, "%d\n", &value);
		c_list[count].mexp = (long)value;

		/* speed */
		fscanf(file, "%d\n", &value);
		c_list[count].speed = (signed char)value;

		/* cchar */
		fscanf(file, "%s\n", a);
		c_list[count].cchar = a[0];

		/* hd */
		fscanf(file, "%s\n", c_list[count].hd);

		/* damage */
		fgets(c_list[count].damage, sizeof(char[36]), file);
		/* strip newline */
		c_list[count].damage[strlen(c_list[count].damage) - 1] = 0;

		/* level */
		fscanf(file, "%d\n", &value);
		c_list[count].level = (signed char)value;

		/* mr */
		fscanf(file, "%d\n", &value);
		c_list[count].mr = (unsigned char)value;

		/* print_creature(&(c_list[count]),count,1); */
	} /* end while loop */
	MSG(("load_monsters: loaded %d of max %d", count, MAX_CREATURES));

	fclose(file);

	/*////////////////////////////////////////////////////////////////////
	 */

	/* try to open the custom creature names files */
	/* if they exist, then read in new names for creatures */

	/* first load the global custom names */
	file = priv_fopen(MORIA_GCST, "r");
	if (file != NULL) {
		lm__read_custom(file);
		fclose(file);
	} else {
		printf(
		    "\n\rUnable to open custom name file for reading: %s\n\r",
		    MORIA_GCST);
		/* pause_game(24); */
	}

	/* now get the users custom names */
	file = priv_fopen(MORIA_CST, "r");
	if (file != NULL) {
		lm__read_custom(file);
		fclose(file);
	}

	/*  for (count = 1; count < 5; count++) { */
	/*    print_creature(&(c_list[count]),count,1); */
	/*  } */
	LEAVE("load_monsters", "");
}

void print_creature(creature_type *c, int c_num, int style)
{
	printf("--- %03d ---\n\r", c_num);

	if (style) {
		printf("%d\n\r%d\n\r%s\n\r%08lx\n\r%08lx\n\r", c->aaf, c->ac,
		       c->name, c->cmove, c->spells);

		printf("%04lx\n\r%d\n\r%ld\n\r%d\n\r%c\n\r", c->cdefense,
		       c->sleep, c->mexp, c->speed, c->cchar);

		printf("%s\n\r%s\n\r%d\n\r%d\n\r", c->hd, c->damage, c->level,
		       c->mr);

	} else {
		printf("aaf: %d\n\rac: %d\n\rname: %s\n\rcmove: "
		       "%08lx\n\rspells: %08lx\n\r",
		       c->aaf, c->ac, c->name, c->cmove, c->spells);

		printf("cdefense: %08lx\n\rsleep: %d\n\rmexp: %ld\n\rspeed: "
		       "%d\n\rcchar: %c\n\r",
		       c->cdefense, c->sleep, c->mexp, c->speed, c->cchar);

		printf("hd: %s\n\rdamage: %s\n\rlevel: %d\n\rmr: %d\n\r", c->hd,
		       c->damage, c->level, c->mr);
	}
}

void replace_name()
{
	/*{ replace <gp> for game players name }*/

	long count;
	char t_str[82];
	char *s;

	ENTER(("replace_name", ""));

	strcpy(t_str, player_name);
	if (t_str[0] == 0) {
		strcpy(t_str, "Dead Guy");
	}
	if (strlen(t_str) > 15) {
		t_str[15] = 0;
	}

	for (count = 1; count <= MAX_CREATURES; count++) {
		s = strstr(c_list[count].name, "<gp>");
		if (s != NULL) {
			/*      printf ("\n\nOldName = */
			/*      >>%s<<\n",c_list[count].name); */
			insert_str(c_list[count].name, "<gp>", t_str);
			/*      printf ("NewName = */
			/*      >>%s<<\n",c_list[count].name); */
		} /* end if */
	}	 /* end for */

	LEAVE("replace_name", "");
}

void check_mon_lite(long y, long x)
{
	/*{ Makes sure new creature gets lit up                   -RAK-   }*/

	/* with cave[y][x]. do; */
	if (cave[y][x].cptr > 1) {
		if (!(m_list[cave[y][x].cptr].ml)) {
			if ((cave[y][x].tl) || (cave[y][x].pl)) {
				if (los(char_row, char_col, y, x)) {
					m_list[cave[y][x].cptr].ml = true;
					lite_spot(y, x);
				}
			}
		}
	}
}

void multiply_monster(long y, long x, long z, boolean slp)
{
	/*{ Places creature adjacent to given location            -RAK-   }*/
	/*{ Rats and Flies are fun!                                       }*/

	long i1, i2, i3;

	i1 = 0;

	do {
		i2 = y - 2 + randint(3);
		i3 = x - 2 + randint(3);

		if (in_bounds(i2, i3)) {
			/* with cave[i2,i3] do; */
			if (is_in(cave[i2][i3].fval, floor_set)) {
				if ((cave[i2][i3].tptr == 0) &&
				    (cave[i2][i3].cptr != 1)) {
					if (cave[i2][i3].cptr >
					    1) { /* { Creature there already?
						    }*/
						/*{ Some critters are
						 * canabalistic!       }*/
						if (uand(c_list[z].cmove,
							 0x00080000) != 0) {
							delete_monster(
							    cave[i2][i3].cptr);
							place_monster(i2, i3, z,
								      slp);
							check_mon_lite(i2, i3);
							mon_tot_mult++;
						}
					} else {
						/*{ All clear, place a monster
						 * }*/
						place_monster(i2, i3, z, slp);
						check_mon_lite(i2, i3);
						mon_tot_mult++;
					}
					i1 = 18;
				}
				i1++;
			}
		}
	} while (i1 <= 18);
}

/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*///////////           Begin the insanity               ///////////// */

void c__update_mon(long monptr, long *hear_count)
{
	boolean flag;
	long h_range, s_range;

	/*  ENTER("c__update_mon", "c") */

	/* with m_list[monptr]. do; */
	/* with cave[MY(monptr)][MX(monptr)]. do; */
	flag = false;

	if ((is_in(cave[MY(monptr)][MX(monptr)].fval, water_set)) &&
	    (uand(c_list[ML(monptr).mptr].cmove, 0x00800000) == 0)) {
		/*{in water, not flying}*/
		h_range = 10;
		s_range = 5;
	} else {
		h_range = -1;
		s_range = MAX_SIGHT;
	}

	if ((player_flags.blind < 1) && panel_contains(MY(monptr), MX(monptr))) {
		if (wizard2) {
			flag = true;
		} else if ((ML(monptr).cdis <= s_range)) {
			if (los(char_row, char_col, MY(monptr), MX(monptr))) {
				/* with c_list[mptr] do; */
				if ((cave[MY(monptr)][MX(monptr)].pl) ||
				    (cave[MY(monptr)][MX(monptr)]
					 .tl)) { /*{can see creature?}*/
					flag = (player_flags.see_inv ||
						(uand(0x10000,
						      c_list[ML(monptr).mptr]
							  .cmove) == 0));
				} else if (player_flags.see_infra >
					   0) { /*{infravision?}*/
					flag = ((ML(monptr).cdis <=
						 player_flags.see_infra) &&
						(uand(0x2000,
						      c_list[ML(monptr).mptr]
							  .cdefense) != 0));
				}
			}
		}
	}

	if ((ML(monptr).cdis <= h_range) && /*{noise in water?}*/
	    los(char_row, char_col, MY(monptr), MX(monptr)) && (!flag)) {
		(*hear_count)++;
	}

	if (flag) {
		/*{ Light it up...        }*/
		if (!(ML(monptr).ml)) {
			print(c_list[ML(monptr).mptr].cchar, MY(monptr),
			      MX(monptr));
			ML(monptr).ml = true;
			if (search_flag) {
				search_off();
			}
			if (player_flags.rest > 0) {
				rest_off();
			}
			flush();
			if (find_flag) {
				find_flag = false;
				move_char(5);
			}
		}
	} else if (ML(monptr).ml) {
		/*{ Turn it off...        }*/
		ML(monptr).ml = false;
		if ((cave[MY(monptr)][MX(monptr)].tl) ||
		    (cave[MY(monptr)][MX(monptr)].pl)) {
			lite_spot(MY(monptr), MX(monptr));
		} else {
			unlite_spot(MY(monptr), MX(monptr));
		}
	}

	/*  LEAVE("c__update_mon", "c") */
}

void c__monster_eaten_message(char *squash, char *doesit, long cptr)
{
	char out_val[1026];

	ENTER(("c__monster_eaten_message", "c"))

	switch (randint(10)) {
	case 1:
		sprintf(out_val,
			"Uh oh...it looks like the %s is in need of first aid.",
			squash);
		break;

	case 2:
		sprintf(out_val, "*splat* *crunch* *gobble* *BUUUUUUURP*");
		break;

	case 3:
		sprintf(out_val,
			"Look out!  The %s is going to-- Eeeeew...never mind.",
			doesit);
		break;

	case 4:
		sprintf(out_val, "Ick...the %s has %s all over his toes.",
			doesit, squash);

		break;

	case 5:
		sprintf(out_val, "The nice %s took out the %s for you.", doesit,
			squash);
		break;

	case 6:
		sprintf(out_val,
			"WoWEE, Auggie Ben-Doggie!  The %s just got blatted!",
			squash);
		break;

	case 7:
		sprintf(out_val, "The %s Society will not appreciate this. . .",
			squash);
		break;

	case 8:
		sprintf(out_val, "The %s is not amused.", squash);
		break;

	case 9:
		sprintf(out_val, "The %s pauses to clean the %s off.", doesit,
			squash);
		break;

	case 10:
		sprintf(out_val, "Aw, darn.  There goes %ld experience!",
			(c_list[m_list[cptr].mptr].mexp));
		break;
	}

	msg_print(out_val);

	LEAVE("c__monster_eaten_message", "c")
}

boolean c__check_for_hit(long monptr, long atype)
{
	long level, armor_stuff;
	boolean flag = false;

	ENTER(("c__check_for_hit", "c"))

	level = c_list[m_list[monptr].mptr].level;
	armor_stuff = player_pac + player_ptoac;

	switch (atype) {
	case 1: /*{Normal attack  }*/
		flag = test_hit(60, level, 0, armor_stuff);
		break;

	case 2: /*{Poison Strength}*/
		flag = test_hit(-3, level, 0, armor_stuff);
		break;

	case 3: /*{Confusion attack}*/
		flag = test_hit(10, level, 0, armor_stuff);
		break;

	case 4: /*{Fear attack    }*/
		flag = test_hit(10, level, 0, armor_stuff);
		break;

	case 5: /*{Fire attack    }*/
		flag = test_hit(10, level, 0, armor_stuff);
		break;

	case 6: /*{Acid attack    }*/
		flag = test_hit(0, level, 0, armor_stuff);
		break;

	case 7: /*{Cold attack    }*/
		flag = test_hit(10, level, 0, armor_stuff);
		break;

	case 8: /*{Lightning attack}*/
		flag = test_hit(10, level, 0, armor_stuff);
		break;

	case 9: /*{Corrosion attack}*/
		flag = test_hit(0, level, 0, armor_stuff);
		break;

	case 10: /*{Blindness attack}*/
		flag = test_hit(2, level, 0, armor_stuff);
		break;

	case 11: /*{Paralysis attack}*/
		flag = test_hit(2, level, 0, armor_stuff);
		break;

	case 12: /*{Steal Money    }*/
		flag = test_hit(5, level, 0, player_lev) &&
		       (player_money[TOTAL_] > 0);
		break;

	case 13: /*{Steal Object   }*/
		flag = test_hit(2, level, 0, player_lev) && (inven_ctr > 0);
		break;

	case 14: /*{Poison         }*/
		flag = test_hit(5, level, 0, armor_stuff);
		break;

	case 15: /*{Lose dexterity}*/
		flag = test_hit(0, level, 0, armor_stuff);
		break;

	case 16: /*{Lose constitution}*/
		flag = test_hit(0, level, 0, armor_stuff);
		break;

	case 17: /*{Lose intelligence}*/
		flag = test_hit(2, level, 0, armor_stuff);
		break;

	case 18: /*{Lose wisdom}*/
		flag = test_hit(0, level, 0, armor_stuff);
		break;

	case 19: /*{Lose experience}*/
		flag = test_hit(5, level, 0, armor_stuff);
		break;

	case 20: /*{Aggravate monsters}*/
		flag = true;
		break;

	case 21: /*{Disenchant        }*/
		flag = test_hit(20, level, 0, armor_stuff);
		break;

	case 22: /*{Eat food          }*/
		flag = test_hit(5, level, 0, armor_stuff);
		break;

	case 23: /*{Eat light         }*/
		flag = test_hit(5, level, 0, armor_stuff);
		break;

	case 24: /*{Eat charges       }*/
		flag = test_hit(15, level, 0, armor_stuff);
		break;

	case 25: /*{Lose charisma     }*/
		flag = test_hit(2, level, 0, armor_stuff);
		break;

	case 26: /*{Petrification     }*/
		flag = test_hit(10, level, 0, armor_stuff);
		break;

	case 27: /*{POISON poison     }*/
		flag = test_hit(5, level, 0, armor_stuff);
		break;

	case 99: /* protevil or protmon repelled the attack */
		flag = true;
		break;

	default:
		flag = false;
		break;

	} /* end switch */

	RETURN("c__check_for_hit", "c", 'b', "test hit:", &flag);
	return flag;
}

void c__print_attack(long monptr, long adesc, char *cdesc)
{
	char the_attack[134];
	boolean no_print = false;

	ENTER(("c__print_attack", "%ld, %ld,len: %d >%s<", monptr, adesc,
	       strlen(cdesc), cdesc));
	strcpy(the_attack, cdesc);

	switch (adesc) {
	case 1:
		strcat(the_attack, "hits you.");
		break;
	case 2:
		strcat(the_attack, "bites you.");
		break;
	case 3:
		strcat(the_attack, "claws you.");
		break;
	case 4:
		strcat(the_attack, "stings you.");
		break;
	case 5:
		strcat(the_attack, "touches you.");
		break;
	case 6:
		strcat(the_attack, "kicks you.");
		break;
	case 7:
		strcat(the_attack, "gazes at you.");
		break;
	case 8:
		strcat(the_attack, "breathes on you.");
		break;
	case 9:
		strcat(the_attack, "spits on you.");
		break;
	case 10:
		strcat(the_attack, "makes a horrible wail.");
		break;
	case 11:
		strcat(the_attack, "embraces you.");
		break;
	case 12:
		strcat(the_attack, "crawls on you.");
		break;
	case 13:
		strcat(the_attack, "releases a cloud of spores.");
		break;
	case 14:
		strcat(the_attack, "begs you for money.");
		break;
	case 15:
		strcat(the_attack, "You've been slimed!");
		break;
	case 16:
		strcat(the_attack, "crushes you.");
		break;
	case 17:
		strcat(the_attack, "tramples you.");
		break;
	case 18:
		strcat(the_attack, "drools on you.");
		break;
	case 19:
		switch (randint(9)) {
		case 1:
			strcat(the_attack, "insults you!");
			break;
		case 2:
			strcat(the_attack, "insults your mother!");
			break;
		case 3:
			strcat(the_attack, "gives you the finger!");
			break;
		case 4:
			strcat(the_attack, "humiliates you!");
			break;
		case 5:
			strcat(the_attack, "wets on your leg!");
			break;
		case 6:
			strcat(the_attack, "defiles you!");
			break;
		case 7:
			strcat(the_attack, "dances around you!");
			break;
		case 8:
			strcat(the_attack, "makes obscene gestures!");
			break;
		case 9:
			strcat(the_attack, "moons you!!!");
			break;
		}
		break;
	case 23:
		strcat(the_attack, "sings a charming song");
		break;
	case 24:
		strcat(the_attack, "kisses you");
		break;
	case 25:
		strcat(the_attack, "gores you");
		break;
	case 26:
		switch (randint(2)) {
		case 1:
			strcat(the_attack, "moos forlornly");
			break;
		case 2:
			strcat(the_attack, "questioningly looks at you");
			break;
		}
		break;
	case 27:
		strcat(the_attack, "shocks you");
		break;
	case 28:
		strcat(the_attack, "squirts ink at you");
		break;
	case 29:
		strcat(the_attack, "entangles you");
		break;
	case 30:
		strcat(the_attack, "sucks your blood");
		break;
	case 31:
		strcat(the_attack, "goes for your throat!");
		break;
	case 32:
		strcat(the_attack, "blows bubbles at you");
		break;
	case 33:
		strcat(the_attack, "squawks at you");
		break;
	case 34:
		strcat(the_attack, "pecks at you");
		break;
	case 35:
		strcat(the_attack, "barks at you");
		break;
	case 36:
		strcat(the_attack, "rubs against your leg");
		break;
	case 37:
		strcat(the_attack, "follows you around");
		break;
	case 99:
		strcat(the_attack, "is repelled.");
		break;

	case 0:
	default:
		no_print = true;
		break;
	} /* end switch */

	if (!no_print) {
		msg_print(the_attack);
	}

	LEAVE("c__print_attack", "c")
}

void c__apply_attack(long monptr, long atype, char ddesc[82], char *damstr)
{
	long dam, level, aning;
	long i1, i2, i4;
	boolean flag;
	treas_ptr item_ptr;
	obj_set food_stuffs = {Food, 0};
	obj_set staff_rod_or_wand = {staff, rod, wand, 0};

	ENTER(("c__apply_attack", "c"))

	level = c_list[m_list[monptr].mptr].level;

	switch (atype) {
	case 1: /*{Normal attack  }*/
		dam = damroll(damstr);
		/* with player_do; */
		dam -= (long)((((player_pac + player_ptoac) / 200.0) * dam) + .5);
		take_hit(dam, ddesc);
		prt_stat_block();
		break;

	case 2: /*{Poison Strength}*/
		take_hit(damroll(damstr), ddesc);
		lose_stat(STR, "You feel weaker.",
			  "You feel weaker for a moment, then it passes.");
		prt_stat_block();
		break;

	case 3: /*{Confusion attack}*/
		/* with player_flags do; */
		take_hit(damroll(damstr), ddesc);
		if (randint(2) == 1) {
			if (PF.confused < 1) {
				msg_print("You feel confused.");
				PF.confused += randint(level);
			}
			PF.confused += 3;
		}
		prt_stat_block();
		break;

	case 4: /*{Fear attack    }*/
		/* with player_flags do; */
		take_hit(damroll(damstr), ddesc);
		if (player_spell_saves()) {
			msg_print("You resist the effects!");
		} else if (PF.afraid < 1) {
			msg_print("You are suddenly afraid!");
			PF.afraid += 3 + randint(level);
		} else {
			PF.afraid += 3;
		}
		prt_stat_block();
		break;

	case 5: /*{Fire attack    }*/
		msg_print("You are enveloped in flames!");
		fire_dam(damroll(damstr), ddesc);
		break;

	case 6: /*{Acid attack    }*/
		msg_print("You are covered in acid!");
		acid_dam(damroll(damstr), ddesc);
		break;

	case 7: /*{Cold attack    }*/
		msg_print("You are covered with frost!");
		cold_dam(damroll(damstr), ddesc);
		break;

	case 8: /*{Lightning attack}*/
		msg_print("Lightning strikes you!");
		light_dam(damroll(damstr), ddesc);
		break;

	case 9: /*{Corrosion attack}*/
		msg_print("A stinging red gas swirls about you.");
		corrode_gas(ddesc);
		take_hit(damroll(damstr), ddesc);
		prt_stat_block();
		break;

	case 10: /*{Blindness attack}*/
		/* with player_flags do; */
		take_hit(damroll(damstr), ddesc);
		if (PF.blind < 1) {
			PF.blind += 10 + randint(level);
			msg_print("Your eyes begin to sting.");
			msg_print(" ");
		}
		PF.blind +=
		    5; /* blind the first time is worse than cumulitave blind */
		prt_stat_block();
		break;

	case 11: /*{Paralysis attack}*/
		/* with player_flags do; */
		take_hit(damroll(damstr), ddesc);
		if (player_spell_saves()) {
			msg_print("You resist the effects!");
		} else if (PF.paralysis < 1) {
			if (PF.free_act || (PF.free_time > 0)) {
				msg_print("You are unaffected.");
			} else {
				/* new paralysis overwrites old one, otherwise
				 * you become dead fast */
				PF.paralysis = randint(level) + 3;
				msg_print("You are paralyzed.");
			}
		}
		prt_stat_block();
		break;

	case 12: /*{Steal Money     }*/
		/* with player_do; */
		if ((randint(256) < (C_player_get_stat(DEX) * 10)) &&
		    (player_flags.paralysis < 1)) {
			msg_print("You quickly protect your money pouch!");
		} else {
			if (player_money[TOTAL_] > 0) {
				subtract_money(randint(5) * (player_money[TOTAL_] *
							     GOLD_VALUE)/ 100,
					       false);
				msg_print("Your purse feels lighter.");
				prt_stat_block();
			}
		}
		if (randint(2) == 1) {

			msg_print("There is a puff of smoke!");
			teleport_away(monptr, MAX_SIGHT);
		}

		break;

	case 13: /*{Steal Object   }*/
		/* with py.stat do; */
		if ((randint(256) < (C_player_get_stat(DEX) * 10)) &&
		    (player_flags.paralysis < 1)) {
			msg_print("You grab hold of your backpack!");
		} else {
			item_ptr = inventory_list;
			for (i1 = randint(inven_ctr) - 1; i1 > 0; --i1) {
				item_ptr = item_ptr->next;
				if (item_ptr != NULL) {
					break;
				}
			}
			if (item_ptr != NULL) {
				if (!item_ptr->is_in) {
					if (uand(item_ptr->data.flags2, Holding_bit) !=
							0) {
						if (item_ptr->insides == 0) {
							inven_destroy(item_ptr);
						}
					} else {
						inven_destroy(item_ptr);
					}
				} else {
					inven_destroy(item_ptr);
				}
				prt_stat_block();
				msg_print("Your backpack feels lighter.");
			}
		}

		if (randint(2) == 1) {
			msg_print("There is a puff of smoke!");
			teleport_away(monptr, MAX_SIGHT);
		}

		break;

	case 14: /*{Poison         }*/
		/* with player_flags do ; */
		take_hit(damroll(damstr), ddesc);
		prt_stat_block();
		msg_print("You feel very sick.");
		PF.poisoned += randint(level) + 5;
		break;

	case 15: /*{Lose dexterity }*/
		/* with player_flags do; */
		take_hit(damroll(damstr), ddesc);
		lose_stat(DEX, "You feel more clumsy",
			  "You feel clumsy for a moment, then it passes.");
		prt_stat_block();
		break;

	case 16: /*{Lose constitution }*/
		/* with player_flags do; */
		take_hit(damroll(damstr), ddesc);
		lose_stat(CON, "Your health is damaged!",
			  "Your body resists the effects of the disease.");
		prt_stat_block();
		break;

	case 17: /*{Lose intelligence }*/
		/* with player_flags do; */
		take_hit(damroll(damstr), ddesc);
		lose_stat(
		    INT, "You feel your memories fading.",
		    "You feel your memories fade, then they are restored!");
		prt_stat_block();
		break;

	case 18: /*{Lose wisdom      }*/
		/* with player_flags do; */
		take_hit(damroll(damstr), ddesc);
		lose_stat(WIS, "Your wisdom is drained.",
			  "Your wisdom is sustained.");
		prt_stat_block();
		break;

	case 19: /*{Lose experience  }*/
		msg_print("You feel your life draining away!");
		i1 = damroll(damstr) + (player_exp / 100) * MON_DRAIN_LIFE;
		lose_exp(i1);
		break;

	case 20: /*{Aggravate monster}*/
		aggravate_monster(5);
		break;

	case 21: /*{Disenchant       }*/
		flag = false;
		switch (randint(8)) {
		case 1:
			i1 = Equipment_primary;
			break;
		case 2:
			i1 = Equipment_armor;
			break;
		case 3:
			i1 = Equipment_belt;
			break;
		case 4:
			i1 = Equipment_shield;
			break;
		case 5:
			i1 = Equipment_cloak;
			break;
		case 6:
			i1 = Equipment_gloves;
			break;
		case 7:
			i1 = Equipment_bracers;
			break;
		case 8:
			i1 = Equipment_helm;
			break;
		default:
			MSG(("ERROR: randint returned an out of range value in "
			     "c__apply_attack"));
			i1 = Equipment_primary;
			break;
		}

		/* with equipment[i1] do; */
		if (equipment[i1].tohit > 0) {
			equipment[i1].tohit -=
			    randint((equipment[i1].tohit == 1) ? 1 : 2);
			flag = true;
		}
		if (equipment[i1].todam > 0) {
			equipment[i1].todam -=
			    randint((equipment[i1].todam == 1) ? 1 : 2);
			flag = true;
		}
		if (equipment[i1].toac > 0) {
			equipment[i1].toac -=
			    randint((equipment[i1].toac == 1) ? 1 : 2);
			flag = true;
		}

		if (flag) {
			msg_print("There is a static feeling in the air...");
			py_bonuses(&blank_treasure, 1);
		}
		break;

	case 22: /*{Eat food         }*/
		if (find_range(food_stuffs, false, &item_ptr, &i2)) {
			inven_destroy(item_ptr);
			prt_stat_block();
		}
		break;

	case 23: /*{Eat light        }*/
		/* with equipment[Equipment_light] do; */
		if (equipment[Equipment_light].p1 > 0) {
			equipment[Equipment_light].p1 -= 250 + randint(250);
			if (equipment[Equipment_light].p1 < 1) {
				equipment[Equipment_light].p1 = 1;
			}
			msg_print("Your light dims...");
		}
		break;

	case 24: /*{Eat charges     }*/
		if (inven_ctr > 0) {
			item_ptr = inventory_list;
			aning = randint(inven_ctr) - 1;
			for (i1 = 1; i1 <= aning; i1++) {
				item_ptr = item_ptr->next;
			}
			i4 = level;
			/* with item_ptr^.data do; */
			if (is_in(item_ptr->data.tval, staff_rod_or_wand)) {
				if (item_ptr->data.p1 > 0) {
					m_list[monptr].hp +=
					    i4 * item_ptr->data.p1;
					item_ptr->data.p1 = 0;
				}
			}
			msg_print("Energy drains from your pack!");
		}
		break;

	case 25: /*{Lose charisma   }*/
		/* with player_flags do; */
		take_hit(damroll(damstr), ddesc);
		lose_stat(CHR, "Your skin starts to itch.",
			  "Your skin starts to itch, but feels better now.");
		prt_stat_block();
		break;

	case 26: /*{Petrification  }*/
		/* with player_flags do; */
		petrify(m_list[monptr].hp);
		break;

	case 27: /*{POISON Poison  }*/
		/* with player_flags do; */
		PF.poisoned += damroll(damstr);
		msg_print("You feel very sick.");
		break;

	case 99:
		break;

	default:
		break;
	} /* end switch */

	LEAVE("c__apply_attack", "c")
}

void c__make_attack(long monptr)
{
	/*{ Make an attack on the player (chuckle...)             -RAK-   }*/

	long atype, adesc; /*,dam;*/
	long acount;
	long i5;
	char attstr[82], attx[82];
	char cdesc[82], ddesc[82], s1[82], theattack[82];
	boolean flag;
	char damstr[36];
	char *achar;

	ENTER(("c__make_attack", "c"))
	/* with m_list[monptr] do; */
	/* with c_list[m_list[monptr].mptr]. do; */

	attstr[0] = 0;
	attx[0] = 0;

	strcpy(attstr, c_list[m_list[monptr].mptr].damage);
	find_monster_name(cdesc, monptr, true);
	strcat(cdesc, " ");

	/*{ For "DIED_FROM" string        }*/
	if (uand(0x80000000, c_list[m_list[monptr].mptr].cmove) != 0) {
		sprintf(ddesc, "The %s", c_list[m_list[monptr].mptr].name);
	} else {
		sprintf(ddesc, "& %s", c_list[m_list[monptr].mptr].name);
	}
	strcpy(inven_temp->data.name, ddesc);
	inven_temp->data.number = 1;
	objdes(ddesc, inven_temp, true);
	strcpy(died_from, ddesc);
	/*{ End DIED_FROM                 }*/

	for (; attstr[0] != 0;) {

		/* attstr looks like this: "1 32 4d4|2 21 0d0" */
		achar = strchr(attstr, '|');
		if (achar != NULL) {
			strcpy(attx, attstr);
			achar = strchr(attx, '|');
			(*achar) = 0;
			achar++;
			strcpy(attstr, achar);
		} else {
			strcpy(attx, attstr);
			attstr[0] = 0;
		}

		sscanf(attx, "%ld %ld %s", &atype, &adesc, damstr);

		if (player_flags.protevil > 0) {
			if (uand(c_list[m_list[monptr].mptr].cdefense,
				 0x0004) != 0) {
				if ((player_lev + 1) >
				    c_list[m_list[monptr].mptr].level) {
					atype = 99;
					adesc = 99;
				}
			}
		}

		if (player_flags.protmon > 0) {
			if (uand(c_list[m_list[monptr].mptr].cdefense,
				 0x0002) != 0) {
				if ((player_lev + 1) >
				    c_list[m_list[monptr].mptr].level) {
					atype = 99;
					adesc = 99;
				}
			}
		}

		if ((achar = strstr(damstr, "-")) != NULL) {
			(*achar) = ' ';
			sscanf(damstr, "%ld %s", &acount, s1);
			strcpy(damstr, s1);
		} else {
			acount = 1;
		}

		/* with player_do; */
		for (i5 = 1; i5 <= acount; i5++) {

			flag = c__check_for_hit(monptr, atype);

			if (flag) {
				c__print_attack(monptr, adesc, cdesc);
				c__apply_attack(monptr, atype, ddesc, damstr);
			} else {
				switch (adesc) {
				case 1:
				case 2:
				case 3:
				case 6:
					/* normal, poison strength, confusion
					 * attack, acid attack */
					sprintf(theattack, "%smisses you.",
						cdesc);
					msg_print(theattack);
					break;
				default:
					break;
				}
			}
		} /* end for i5 */
	}	 /* end for attstr != null */

	LEAVE("c__make_attack", "c")
}

boolean c__make_move(long monptr, mm_type mm, long *hear_count)
{
	/*{ Make the move if possible, five choices               -RAK-   }*/

	long i1, i2, newy, newx;
	unsigned long movebits;
	boolean flag, tflag;
	char out_val[82];
	boolean return_value = false;

	ENTER(("c__make_move", "c"))

	i1 = 1;
	flag = false;
	movebits = c_list[m_list[monptr].mptr].cmove;

	do {
		/*{ Get new positon               }*/
		newy = m_list[monptr].fy;
		newx = m_list[monptr].fx;
		move_dir(mm[i1], &newy, &newx);
		/* with cave[newy][newx]. do; */
		if (cave[newy][newx].fval != boundry_wall.ftval) {
			tflag = false;
			if (cave[newy][newx].cptr == 1) {
				tflag = true;
			} else if (cave[newy][newx].fopen) {
				if (is_in(cave[newy][newx].fval, floor_set)) {
					if (uand(movebits, 0x00000040) == 0) {
						tflag = true;
					} else if (! xor
						   ((is_in(
							cave[newy][newx].fval,
							earth_set)),
						    (uand(movebits,
							  0x00000010) == 0))) {
						tflag = true;
					}
				}
				/*{ Creature moves through walls? }*/
			} else if (uand(movebits, 0x40000) != 0) {
				tflag = true;
				/*{ Creature can open doors?      }*/
			} else if (cave[newy][newx].tptr > 0) {

				/* with t_list[cave[newy][newx].tptr]. do; */
				/* with m_list[monptr]. do; */
				if (uand(movebits, 0x20000) != 0) {
					/*{ Creature can open doors... }*/
					switch (t_list[cave[newy][newx].tptr]
						    .tval) {
					case closed_door: /*{ Closed doors...
							     }*/
						if (t_list[cave[newy][newx]
							       .tptr].p1 ==
						    0) { /*{ Closed doors  }*/
							tflag = true;
							if (cave[newy][newx]
								.fm) {
								if (los(char_row,
									char_col,
									newy,
									newx)) {
									t_list
									    [cave[newy][newx]
										 .tptr] = door_list
										[DL_OPEN];
									cave[newy]
									    [newx]
										.fopen =
									    true;
									lite_spot(
									    newy,
									    newx);
									tflag =
									    false;
								}
							}
						} else if (t_list
							       [cave[newy][newx]
								    .tptr].p1 >
							   0) {
							/*{ Locked doors  }*/
							if (randint(
								100 -
								c_list
								    [m_list[monptr]
									 .mptr]
									.level) <
							    5) {
								t_list
								    [cave[newy]
									 [newx]
									     .tptr]
									.p1 = 0;
							}
						} else if (t_list
							       [cave[newy][newx]
								    .tptr].p1 <
							   0) {
							/*{ Stuck doors   }*/
							if (randint(
								m_list[monptr]
								    .hp) >
							    (10 +
							     labs(
								 t_list
								     [cave[newy]
									  [newx]
									      .tptr]
									 .p1))) {
								t_list
								    [cave[newy]
									 [newx]
									     .tptr]
									.p1 = 0;
							}
						}
						break;

					case secret_door: /*{ Secret doors...
							     }*/
						tflag = true;
						if (cave[newy][newx].fm) {
							if (los(char_row,
								char_col, newy,
								newx)) {
								t_list[cave[newy]
									   [newx]
									       .tptr] =
								    door_list
									[DL_OPEN];
								cave[newy][newx]
								    .fopen =
								    true;
								lite_spot(newy,
									  newx);
								tflag = false;
							}
						}
						break;

					default:
						break;
					}
				} else {
					/*  { Creature can not open doors, must
					 * bash them   }*/
					switch (t_list[cave[newy][newx].tptr]
						    .tval) {
					case closed_door: /* { Closed doors...
							     }*/
						i2 =
						    labs(t_list[cave[newy][newx]
								    .tptr].p1) +
						    20;
						if (randint(m_list[monptr].hp) >
						    i2) {
							tflag = true;
							if (cave[newy][newx]
								.fm) {
								if (los(char_row,
									char_col,
									newy,
									newx)) {
									t_list
									    [cave[newy][newx]
										 .tptr] = door_list
										[DL_OPEN];
									t_list
									    [cave[newy]
										 [newx]
										     .tptr]
										.p1 =
									    randint(
										2) -
									    1;
									cave[newy]
									    [newx]
										.fopen =
									    true;
									lite_spot(
									    newy,
									    newx);
									tflag =
									    false;
								}
							}
						}
						break;

					case secret_door: /* { Secret doors...
							     }*/
						break;

					default:
						break;
					}
				}
			}

			/* { Glyph of warding present?     }*/
			if (tflag) {
				if (cave[newy][newx].tptr > 0) {
					if (t_list[cave[newy][newx].tptr]
						.tval == seen_trap) {
						if (t_list[cave[newy][newx]
							       .tptr].subval ==
						    99) {
							if (randint(
								OBJ_RUNE_PROT) <
							    c_list
								[m_list[monptr]
								     .mptr]
								    .level) {
								if ((newy ==
								     char_row) &&
								    (newx ==
								     char_col)) {
									msg_print(
									    "Th"
									    "e "
									    "ru"
									    "ne"
									    " o"
									    "f "
									    "pr"
									    "ot"
									    "ec"
									    "ti"
									    "on"
									    " i"
									    "s "
									    "br"
									    "ok"
									    "en"
									    "!");
								}
								delete_object(
								    newy, newx);
							} else {
								tflag = false;
							}
						}
					}
				}
			}

			/* { Creature has attempted to move on player?     }*/
			if (tflag) {
				if (cave[newy][newx].cptr == 1) {

					if (!(m_list[monptr].ml)) {
						c__update_mon(monptr,
							      hear_count);
					}

					if (find_flag) {
						find_flag = false;
						move_char(5);
					}

					c__make_attack(monptr);
					/* { Player has read a Confuse Monster?
					 * }*/
					/* { Monster gets a saving throw... }*/
					if (player_flags.confuse_monster) {
						/* with m_list[monptr] do; */
						/* with */
						/* c_list[m_list[monptr].mptr].
						 */
						/* do; */
						msg_print(
						    "Your hands stop glowing.");
						player_flags.confuse_monster =
						    false;
						if (mon_save(monptr, 0,
							     c_sc_mental)) {
							sprintf(
							    out_val,
							    "The %s is "
							    "unaffected.",
							    c_list
								[m_list[monptr]
								     .mptr]
								    .name);
						} else {
							sprintf(
							    out_val,
							    "The %s appears "
							    "confused.",
							    c_list
								[m_list[monptr]
								     .mptr]
								    .name);
							m_list[monptr]
							    .confused = true;
						}
						msg_print(out_val);
					}
					tflag = false;
					flag = true;

				} else { /* cptr != 1 */

					/*{ Creature is attempting to move on
					 * other creature?     }*/
					if ((cave[newy][newx].cptr > 1) &&
					    ((newy != m_list[monptr].fy) ||
					     (newx != m_list[monptr].fx))) {

						/*{ Creature eats other
						 * creatures?        }*/
						if (uand(movebits, 0x80000) !=
						    0) {
							if (m_list
								[cave[newy]
								     [newx]
									 .cptr]
								    .ml) {
								/*squash =
								 * c_list[m_list[cptr].mptr].name;*/
								/*doesit =
								 * c_list[m_list[monptr].mptr].name;*/
								c__monster_eaten_message(
								    c_list
									[m_list
									     [cave[newy]
										  [newx]
										      .cptr]
										 .mptr]
									    .name,
								    c_list
									[m_list[monptr]
									     .mptr]
									    .name,
								    cave[newy]
									[newx]
									    .cptr);
							}
							delete_monster(
							    cave[newy][newx]
								.cptr);
						} else {
							tflag = false;
						}
					}
				}
			} /* end if tflag */

			/*{ Creature has been allowed move...     }*/
			if (tflag) {
				/* with m_list[monptr] do */

				/*{ Pick up or eat an object              }*/
				if (uand(movebits, 0x100000) != 0) {
					/* with cave[newy,newx] do; */
					if (cave[newy][newx].tptr > 0) {
						if (t_list[cave[newy][newx]
							       .tptr].tval <
						    valuable_metal) {
							delete_object(newy,
								      newx);
						}
					}
				}

				/*{ Move creature record                  }*/
				move_rec(m_list[monptr].fy, m_list[monptr].fx,
					 newy, newx);
				m_list[monptr].fy = newy;
				m_list[monptr].fx = newx;
				flag = true;
				return_value = true;
			}
		}
		i1++;
		/*{ Up to 5 attempts at moving, then give up...   }*/
	} while (!(flag || (i1 > 5)));

	RETURN("c__make_move", "c", 'b', "moved", &return_value);
	return return_value;
}

boolean c__move_confused(long monptr, mm_type mm, long *hear_count)
{
	boolean return_value;

	ENTER(("c__move_confused", "c"))

	mm[1] = randint(9);
	mm[2] = randint(9);
	mm[3] = randint(9);
	mm[4] = randint(9);
	mm[5] = randint(9);
	return_value = c__make_move(monptr, mm, hear_count);

	RETURN("c__move_confused", "c", 'b', "moved", &return_value);
	return return_value;
}

void c__get_moves(long monptr, mm_type *mm)
{
	/*{ Choose correct directions for monster movement        -RAK-   }*/

	long y, x;
	long move_val, octant_side;

	ENTER(("c__get_moves", "c"))
	/*{ octant_side = +/-1 }*/

	if (m_list[monptr].csleep != 0) {
		m_list[monptr].csleep = 0;
	}

	y = char_row - m_list[monptr].fy;
	x = char_col - m_list[monptr].fx;

	move_val = get_hexdecant(y, x);
	octant_side = 2 * (move_val % 2) - 1;

	(*mm)[1] = key_of[move_val / 2];
	(*mm)[2] = rotate_dir((*mm)[1], octant_side);
	(*mm)[3] = rotate_dir((*mm)[1], -octant_side);
	(*mm)[4] = rotate_dir((*mm)[2], octant_side);
	(*mm)[5] = rotate_dir((*mm)[3], -octant_side);

	LEAVE("c__get_moves", "c")
}

boolean c__cast_spell(long monptr, boolean *took_turn)
{
	/*{ Creatures can cast spells too.  (Dragon Breath)       -RAK-   }*/
	/*{ cast_spell := true if creature changes position       }*/
	/*{ took_turn  := true if creature casts a spell          }*/

	unsigned long i1;
	long i2, i3, y, x, chance2;
	long chance, thrown_spell;
	float r1;
	long spell_choice[32]; /* [1..31] of long;*/
	char cdesc[82], ddesc[82], outval[82];
	boolean stop_player;
	boolean return_value;

	ENTER(("c__cast_spell", "c"))
	/* with m_list[monptr] do; */
	/* with c_list[m_list[monptr].mptr] do; */
	chance = (uand(c_list[m_list[monptr].mptr].spells, 0x0000000F));
	chance2 = (uand(c_list[m_list[monptr].mptr].spells, 0x80000000));

	/*{ 1 in x chance of casting spell                }*/
	/*{ if chance2 is true then 1 in x of not casting }*/
	if (((chance2 == 0) && (randint(chance) != 1)) ||
	    ((chance2 != 0) && (randint(chance) == 1))) {

		return_value = false;
		*took_turn = false;

		/*{ Must be within certain range                  }*/
	} else if (m_list[monptr].cdis > MAX_SPELL_DIS) {

		return_value = false;
		*took_turn = false;

		/*{ Must have unobstructed Line-Of-Sight          }*/
	} else if (!(los(char_row, char_col, m_list[monptr].fy,
			 m_list[monptr].fx))) {

		return_value = false;
		*took_turn = false;

	} else { /*{ Creature is going to cast a spell     }*/

		*took_turn = true;
		return_value = false;

		/*{ Describe the attack                           }*/
		find_monster_name(cdesc, monptr, true);
		strcat(cdesc, " ");
		/*{ For "DIED_FROM" string  }*/
		if (uand(0x80000000, c_list[m_list[monptr].mptr].cmove) != 0) {
			sprintf(ddesc, "The %s",
				c_list[m_list[monptr].mptr].name);
		} else {
			sprintf(ddesc, "& %s",
				c_list[m_list[monptr].mptr].name);
		}
		strcpy(inven_temp->data.name, ddesc);
		inven_temp->data.number = 1;
		objdes(ddesc, inven_temp, true);
		/*{ End DIED_FROM                 }*/

		/*{ Extract all possible spells into spell_choice }*/
		i1 = uand(c_list[m_list[monptr].mptr].spells, 0x0FFFFFF0);
		i3 = 0;
		for (; i1 != 0;) {
			i2 = bit_pos(&i1) + 1;
			i3++;
			spell_choice[i3] = i2;
		}

		/*{ Choose a spell to cast                        }*/
		thrown_spell = spell_choice[randint(i3)];
		stop_player = false;

		/*{ Cast the spell...                             }*/
		switch (thrown_spell) {
		case 5: /*{Teleport Short}*/
			teleport_away(monptr, 5);
			return_value = true;
			break;

		case 6: /*{Teleport Long }*/
			teleport_away(monptr, MAX_SIGHT);
			return_value = true;
			break;

		case 7: /*{Teleport To   }*/
			stop_player = true;
			strcat(cdesc, "casts a spell.");
			msg_print(cdesc);
			msg_print(" ");
			teleport_to(m_list[monptr].fy, m_list[monptr].fx);
			break;

		case 8: /*{Light Wound   }*/
			stop_player = true;
			if ((strstr(cdesc, "Bard") != NULL) ||
			    (strstr(cdesc, "Ranger") != NULL) ||
			    (strstr(cdesc, "Master Bard") != NULL)) {
				strcat(cdesc, "shoots an arrow.");
			} else {
				strcat(cdesc, "casts a spell.");
			}
			msg_print(cdesc);
			if (player_spell_saves()) {
				msg_print(
				    "You resist the effects of the spell.");
			} else {
				take_hit(damroll("3d8"), ddesc);
			}
			break;

		case 9: /*{Serious Wound }*/
			stop_player = true;
			strcat(cdesc, "casts a spell.");
			msg_print(cdesc);
			if (player_spell_saves()) {
				msg_print(
				    "You resist the affects of the spell.");
			} else {
				take_hit(damroll("8d8"), ddesc);
			}
			break;

		case 10: /*{Hold Person   }*/
			stop_player = true;
			strcat(cdesc, "casts a spell.");
			msg_print(cdesc);
			if (PF.free_act || (PF.free_time > 0)) {
				msg_print("You are unaffected...");
			} else if (player_spell_saves()) {
				msg_print(
				    "You resist the affects of the spell.");
			} else {
				msg_print("You can't move!");
				if (PF.paralysis > 0) {
					PF.paralysis += 2;
				} else {
					PF.paralysis = randint(5) + 4;
				}
			}
			break;

		case 11: /*{Cause Blindnes}*/
			stop_player = true;
			strcat(cdesc, "casts a spell.");
			msg_print(cdesc);
			if (player_spell_saves()) {
				msg_print(
				    "You resist the affects of the spell.");
			} else if (player_flags.blind > 0) {
				player_flags.blind += 6;
			} else {
				player_flags.blind = 12 + randint(3);
				msg_print(" ");
			}
			break;

		case 12: /*{Cause Confuse }*/
			stop_player = true;
			strcat(cdesc, "casts a spell.");
			msg_print(cdesc);
			if (player_spell_saves()) {
				msg_print(
				    "You resist the affects of the spell.");
			} else if (player_flags.confused > 0) {
				player_flags.confused += 2;
			} else {
				player_flags.confused = randint(5) + 3;
			}
			break;

		case 13: /*{Cause Fear    }*/
			stop_player = true;
			strcat(cdesc, "casts a spell.");
			msg_print(cdesc);
			if (player_spell_saves()) {
				msg_print(
				    "You resist the affects of the spell.");
			} else if (player_flags.afraid > 0) {
				player_flags.afraid += 2;
			} else {
				player_flags.afraid = randint(5) + 3;
			}
			break;

		case 14: /*{Summon Monster}*/
			stop_player = true;
			strcat(cdesc, "magically summons a monster!");
			msg_print(cdesc);
			y = char_row;
			x = char_col;
			if (is_in(cave[y][x].fval, water_set)) {
				summon_water_monster(&y, &x, false);
			} else {
				summon_land_monster(&y, &x, false);
			}
			check_mon_lite(y, x);
			break;

		case 15: /*{Summon Undead}*/
			stop_player = true;
			strcat(cdesc, "magically summons an undead!");
			msg_print(cdesc);
			y = char_row;
			x = char_col;
			summon_undead(&y, &x);
			check_mon_lite(y, x);
			break;

		case 16: /*{Slow Person  }*/
			/* with player_flags do; */
			stop_player = true;
			strcat(cdesc, "casts a spell.");
			msg_print(cdesc);
			if (PF.free_act || (PF.free_time > 0)) {
				msg_print("You are unaffected...");
			} else if (player_spell_saves()) {
				msg_print(
				    "You resist the affects of the spell.");
			} else if (PF.slow > 0) {
				PF.slow += 2;
			} else {
				PF.slow = randint(5) + 3;
			}
			break;

		case 17:
			if (trunc(player_cmana) > 0) { /*{Drain Mana   }*/
				stop_player = true;
				sprintf(outval,
					"%sdraws psychic energy from you!",
					cdesc);
				msg_print(outval);
				sprintf(outval, "%sappears healthier...",
					cdesc);
				msg_print(outval);
				r1 = (randint(c_list[m_list[monptr].mptr].level)
				      / 2) +
				     1;
				if (r1 > player_cmana) {
					r1 = player_cmana;
				}
				player_cmana -= r1;
				m_list[monptr].hp += 6 * trunc(r1);
			}
			break;

		case 18: /*{Breath Evil  }*/
			stop_player = true;
			if (strstr(cdesc, "High Priest") != NULL) {
				strcat(
				    cdesc,
				    "throws a cloud of black vapors at you!");
			} else {
				strcat(cdesc,
				       "breathes black vapors around you!");
			}
			msg_print(cdesc);
			i1 = (player_exp / 100) * MON_DRAIN_LIFE;
			breath(c_evil, char_row, char_col, 1, ddesc);

			break;

		case 19: /*{Breath Petrify }*/
			stop_player = true;
			strcat(cdesc, "breathes petrifying gas at you!");
			msg_print(cdesc);
			breath(c_petrify, char_row, char_col, 1, ddesc);
			break;

		case 20: /*{Breath Light }*/
			stop_player = true;
			if ((strstr(cdesc, "Druid") != NULL) ||
			    (strstr(cdesc, "Titan") != NULL)) {
				strcat(cdesc, "casts a spell.");
			} else {
				strcat(cdesc, "breathes lightning.");
			}
			msg_print(cdesc);
			if ((strstr(cdesc, "Druid") != NULL) ||
			    (strstr(cdesc, "Titan") != NULL)) {
				breath(c_lightning, char_row, char_col, 32,
				       ddesc);
			} else {
				breath(c_lightning, char_row, char_col,
				       (long)(m_list[monptr].hp / 4.0), ddesc);
			}
			break;

		case 21: /*{Breath Gas   }*/
			stop_player = true;
			strcat(cdesc, "breathes gas.");
			msg_print(cdesc);
			breath(c_gas, char_row, char_col,
			       (long)(m_list[monptr].hp / 3.0), ddesc);
			break;

		case 22: /*{Breath Acid  }*/
			stop_player = true;
			strcat(cdesc, "breathes acid.");
			msg_print(cdesc);
			breath(c_acid, char_row, char_col,
			       (long)(m_list[monptr].hp / 3.0), ddesc);
			break;

		case 23: /*{Breath Frost }*/
			stop_player = true;
			strcat(cdesc, "breathes frost.");
			msg_print(cdesc);
			breath(c_cold, char_row, char_col,
			       (long)(m_list[monptr].hp / 3.0), ddesc);
			break;

		case 24: /*{Breath Fire  }*/
			stop_player = true;
			if (strstr(cdesc, "Heirophant Druid") != NULL) {
				strcat(cdesc, "casts a spell.");
			} else {
				strcat(cdesc, "breathes fire.");
			}
			msg_print(cdesc);
			if (strstr(cdesc, "Heirophant Druid") != NULL) {
				breath(c_fire, char_row, char_col, 48, ddesc);
			} else {
				breath(c_fire, char_row, char_col,
				       (long)(m_list[monptr].hp / 3.0), ddesc);
			}
			break;

		case 25: /*{Cast Illusion }*/
			stop_player = true;
			strcat(cdesc, "casts a spell.");
			msg_print(cdesc);
			if (player_spell_saves()) {
				msg_print(
				    "You resist the affects of the spell.");
			} else if (player_flags.image > 0) {
				player_flags.image += 2;
			} else {
				player_flags.image = randint(20) + 10;
			}
			break;

		case 26: /*{Summon Demon}*/
			stop_player = true;
			strcat(cdesc, "magically summons a demon!");
			msg_print(cdesc);
			y = char_row;
			x = char_col;
			summon_demon(&y, &x);
			check_mon_lite(y, x);
			break;

		case 27: /*{Summon Breed }*/
			stop_player = true;
			strcat(cdesc, "magically summons a monster!");
			msg_print(cdesc);
			y = char_row;
			x = char_col;
			summon_breed(&y, &x);
			check_mon_lite(y, x);
			break;

		case 28: /*{Stoning Gaze}*/
			stop_player = true;
			strcat(cdesc, "gazes at you!");
			msg_print(cdesc);
			if (player_spell_saves()) {
				msg_print("You resist the affects!");
			} else {
				/* XXXX hit points of monster sets petrify?? */
				petrify(m_list[monptr].hp);
			}
			break;

		default:
			stop_player = true;
			msg_print("Lucky you, creature casts unknown spell.");
			cdesc[0] = 0;

			MSG(("ERROR: cast bad spell: i3 = %ld "
			     "spell_choice[i3] = %ld\n       "
			     "monster = >%s<\n",
			     i3, spell_choice[i3],
			     c_list[m_list[monptr].mptr].name));
			break;
		}

		/*{ End of spells                                 }*/

		/*{ Stop player if in find mode   -DCJ/KRC-       }*/
		if (stop_player) {
			if (find_flag) {
				find_flag = false;
				move_char(5);
			}
			if (PF.rest > 0) {
				rest_off();
			}
		}
	}
	RETURN("c__cast_spell", "c", 'b', "moved", &return_value);
	return return_value;
}

boolean mon_move(long monptr, long *hear_count)
{
	/*{ Move the critters about the dungeon                   -RAK-   }*/

	long i1, i2, i3;
	mm_type mm;
	boolean move_test;
	boolean return_value = false;

	ENTER(("mon_move", "c"))
	/* with c_list[m_list[monptr].mptr] do; */

	/*{ Does the creature regenerate?                         }*/
	if (uand(c_list[ML(monptr).mptr].cdefense, 0x8000) != 0) {
		m_list[monptr].hp += randint(4);
	}

	if (m_list[monptr].hp > max_hp(c_list[ML(monptr).mptr].hd)) {
		m_list[monptr].hp = max_hp(c_list[ML(monptr).mptr].hd);
	}

	/*{ Does the critter multiply?                            }*/
	if (uand(c_list[ML(monptr).mptr].cmove, 0x00200000) != 0) {
		if (MAX_MON_MULT >= mon_tot_mult) {
			if ((player_flags.rest % mon_mult_adj) == 0) {
				/* with m_list[monptr] do; */
				i3 = 0;

				for (i1 = MY(monptr) - 1; i1 <= MY(monptr) + 1;
				     i1++) {
					for (i2 = MX(monptr) - 1;
					     i2 <= MX(monptr) + 1; i2++) {
						if (in_bounds(i1, i2)) {
							if (cave[i1][i2].cptr >
							    1) {
								i3++;
							}
						}
					}
				}

				if (i3 < 4) {
					if (randint(i3 * mon_mult_adj) == 1) {
						multiply_monster(
						    MY(monptr), MX(monptr),
						    ML(monptr).mptr, false);
					}
				}
			}
		}
	}

	/*{ Creature is confused?  Chance it becomes un-confused  }*/
	move_test = false;

	if (m_list[monptr].confused) {
		return_value = c__move_confused(monptr, mm, hear_count);
		m_list[monptr].confused = (randint(8) != 1);
		move_test = true;
	} else if (c_list[ML(monptr).mptr].spells > 0) {
		/*{ Creature may cast a spell                             }*/
		return_value = c__cast_spell(monptr, &move_test);
	}

	if (!(move_test)) {

		/*{ 75% random movement                                   }*/
		if ((randint(100) <= 75) &&
		    (uand(c_list[ML(monptr).mptr].cmove, 0x00000008) != 0)) {
			return_value = c__move_confused(monptr, mm, hear_count);

			/*{ 40% random movement }*/
		} else if ((randint(100) <= 40) &&
			   (uand(c_list[ML(monptr).mptr].cmove, 0x00000004) !=
			    0)) {
			return_value = c__move_confused(monptr, mm, hear_count);

			/*{ 20% random movement }*/
		} else if ((randint(100) <= 20) &&
			   (uand(c_list[ML(monptr).mptr].cmove, 0x00000002) !=
			    0)) {
			return_value = c__move_confused(monptr, mm, hear_count);

			/*{ Normal movement }*/
		} else if (uand(c_list[ML(monptr).mptr].cmove, 0x00000001) ==
			   0) {
			if (randint(200) == 1) {
				return_value =
				    c__move_confused(monptr, mm, hear_count);
			} else {
				c__get_moves(monptr, &mm);
				return_value =
				    c__make_move(monptr, mm, hear_count);
			}

			/*{ Attack, but don't move }*/
		} else {
			if (m_list[monptr].cdis < 2) {
				c__get_moves(monptr, &mm);
				return_value =
				    c__make_move(monptr, mm, hear_count);
			}
		}
	}

	RETURN("mon_move", "c", 'b', "moved", &return_value);
	return return_value;
}

void c__splash(long monptr)
{
	long i1, mon_swimming, drown_dam;

	ENTER(("c__splash", "c"))
	/* with m_list[monptr]. do; */
	/* with c_list[m_list[monptr].mptr]. do begin; */
	mon_swimming =
	    (long)((uand(c_list[m_list[monptr].mptr].cmove, 0x00000700)) / 256);
	drown_dam = randint(OUT_OF_ENV_DAM);

	/*{ here will also be modifiers due to waterspeed,depth }*/
	/*{ divide damage by 2 for each mon_swimming level, random
	  rounding procedure }*/
	for (i1 = 1; i1 <= mon_swimming; i1++) {
		drown_dam = (drown_dam +
			     (randint(3) - 1))/ 3; /* kinda large for a 2 */
	}

	m_list[monptr].hp -= drown_dam;
	m_list[monptr].csleep = 0;

	if (m_list[monptr].hp < 0) {
		monster_death(m_list[monptr].fy, m_list[monptr].fx,
			      c_list[m_list[monptr].mptr].cmove);
		/* with cave[fy,fx] do; */
		delete_monster(cave[m_list[monptr].fy][m_list[monptr].fx].cptr);
	}

	LEAVE("c__splash", "c")
}

void creatures(boolean attack)
{
	long i1, i2, i3, moldy, moldx;
	long hear_count;

	ENTER(("creatures", "c"))
	/*{ Main procedure for creatures                              -RAK- }*/
	if (1) {
		get_player_move_rate();

		if (muptr > 0) {

			/*{ Process the monsters  }*/
			hear_count = 0;
			i1 = muptr;
			do {
				/* with m_list[i1]. do; */
				m_list[i1].cdis =
				    distance(char_row, char_col, m_list[i1].fy,
					     m_list[i1].fx);
				if (attack) { /*{ Attack is argument passed to
						 CREATURE}*/
					i3 = movement_rate(m_list[i1].cspeed,
							   i1);
					if (i3 > 0) {
						for (i2 = 1; i2 <= i3; i2++) {
							if ((m_list[i1].cdis <=
							     c_list[m_list[i1]
									.mptr]
								 .aaf) ||
							    (m_list[i1].ml)) {
								if (m_list[i1]
									.csleep >
								    0) {
									if (player_flags
										.aggravate) {
										m_list[i1]
										    .csleep =
										    0;
									} else if (player_flags
										       .rest <
										   1) {
										if (randint(
											10) >
										    player_stl) {
											m_list[i1]
											    .csleep -=
											    (long)(75.0 /
												   m_list[i1]
												       .cdis);
										}
									}
								}
								if (m_list[i1]
									.stunned >
								    0) {
									m_list[i1]
									    .stunned--;
								}
								if ((m_list[i1]
									 .csleep <=
								     0) &&
								    (m_list[i1]
									 .stunned <=
								     0)) {
									moldy =
									    m_list[i1]
										.fy;
									moldx =
									    m_list[i1]
										.fx;
									if (mon_move(
										i1,
										&hear_count)) {
										if (m_list[i1]
											.ml) {
											m_list[i1]
											    .ml =
											    false;
											if (test_light(
												moldy,
												moldx)) {
												lite_spot(
												    moldy,
												    moldx);
											} else {
												unlite_spot(
												    moldy,
												    moldx);
											}
										}
									}
								}
							} /* end if (cdis < aaf)
							     || ml */
							c__update_mon(
							    i1, &hear_count);
						} /*{end for 1 to i3 loop}*/
					} else {
						c__update_mon(i1, &hear_count);
					}
				} else { /*{if attacking}*/
					c__update_mon(i1, &hear_count);
				}

				if (is_in(
					cave[m_list[i1].fy][m_list[i1].fx].fval,
					floor_set)) {
					if (((is_in(cave[m_list[i1].fy]
							[m_list[i1].fx].fval,
						    water_set)) !=
					     (uand(
						  c_list[m_list[i1].mptr].cmove,
						  0x00000010) != 0)) &&
					    (uand(c_list[m_list[i1].mptr].cmove,
						  0x00000040) != 0)) {
						c__splash(i1);
					}
				}

				i1 = m_list[i1].nptr;
			} while (!((i1 == 0) || (moria_flag)));

			if (want_warn) {
				if (hear_count == 1) {
					msg_print(
					    "You hear a noise in the water.");
				} else if (hear_count > 1) {
					msg_print("You hear some noises in the "
						  "water.");
				}
			}
		}
		/*{ End processing monsters       }*/
	}
	LEAVE("creatures", "c")
}

void mn__append_mon(long mon_num)
{
	char out_val[82];
	FILE *f1;

	f1 = (FILE *)fopen(MORIA_CST, "a");
	if (f1 == NULL) {
		sprintf(out_val, "Error: unable to open %s for append.",
			MORIA_CST);
		msg_print(out_val);
	} else {
		fprintf(f1, "%s\n", c_list[mon_num].name);
		fprintf(f1, "%ld\n", mon_num);
		fclose(f1);
	}
}

void mon_name()
{
	/*{name any monster you wish [currently virtual]}*/

	char virtual_name[28];
	long mon_num;

	prt("Monster to rename:", 1, 1);
	if (get_string(virtual_name, 1, 20, 26)) {
		mon_num = find_mon(virtual_name);
		if (mon_num != 0) {
			prt("New name:", 1, 1);
			if (get_string(virtual_name, 1, 11, 26)) {
				strcpy(c_list[mon_num].name, virtual_name);
				mn__append_mon(mon_num);
			}
		} else {
			msg_print(
			    "Hmm.... can't find a monster with that name");
		}
	}
	msg_print("");
}

long find_mon(const char *virtual_name)
{
	/*{returns number of monster in list specified by virtual_name}*/

	long count;
	boolean maybe = false;

	for (count = 1; (count <= MAX_CREATURES) && !maybe;) {
		if (!strcmp(virtual_name, c_list[count].name)) {
			maybe = true;
		} else {
			count++;
		}
	}
	if (!maybe) {
		count = 0;
	}
	return count;
}
