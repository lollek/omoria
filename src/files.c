#include <sys/file.h> /* for flock     */

#include <curses.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "constants.h"
#include "desc.h"
#include "generate_item/generate_item.h"
#include "io.h"
#include "misc.h"
#include "player.h"
#include "types.h"
#include "variables.h"

#include "files.h"

/*	{ Output dungeon section sizes					} */
#define OUTPAGE_HEIGHT 44 /* { 44 lines of dungeon per section	} */
#define OUTPAGE_WIDTH 99  /* { 100 columns of dungeon per section	} */


void file_character(void) {
  /*{ Print the character to a file or device               -RAK-   }*/

  char filename1[82];
  prt("File name: ", 1, 1);
  if (get_string(filename1, 1, 12, 64)) {
    if (filename1[0] == 0) {
      strcpy(filename1, "MORIACHR.DAT");
    }
    FILE *file = fopen(filename1, "w");
    if (file != NULL) {
      char s3[82];
      char s2[82];
      char s1[82];
      const char new_page = 12;
      char prt1[120];
      char out_val[300];
      char xinfra[82];
      long i1;
      prt("Writing character sheet...", 1, 1);
      refresh();

      fprintf(file, " \n \n \n");
      fprintf(file,
              "  Name  : %24s  Age         :%4d     "
              "Strength     : %d\n",
              player_name, player_age, C_player_get_stat(STR));

      fprintf(file,
              "  Race  : %24s  Height      :%4d     "
              "Intelligence : %d\n",
              player_race, player_ht, C_player_get_stat(INT));

      fprintf(file,
              "  Sex   : %24s  Weight      :%4d     "
              "Wisdom       : %d\n",
              player_sex, player_wt, C_player_get_stat(WIS));

      fprintf(file,
              "  Class : %24s  Social Class:%4d     "
              "Dexterity    : %d\n",
              player_tclass, player_sc, C_player_get_stat(DEX));

      fprintf(file,
              "          %24s                       "
              "Constitution : %d\n",
              "", C_player_get_stat(CON));

      fprintf(file,
              "          %24s              %4s      "
              "Charisma     : %d\n",
              "", "", C_player_get_stat(CHR));

      fprintf(file, " \n \n \n \n");

      fprintf(file,
              "  + To Hit    :%3d        Level      "
              ":%9d     Max Hit Points :%4d\n",
              player_dis_th, player_lev, C_player_max_hp());

      fprintf(file,
              "  + To Damage :%3d        Experience "
              ":%9lld     Cur Hit Points :%4d\n",
              player_dis_td, player_exp, C_player_current_hp());

      fprintf(file,
              "  + To AC     :%3d        Gold       "
              ":%9lld     Max Mana       :%4d\n",
              player_dis_tac, player_money[TOTAL_], player_mana);

      fprintf(file,
              "    Total AC  :%3d        Account    "
              ":%9lld     Cur Mana       :%4d\n",
              player_dis_ac, player_account, player_mana);

      fprintf(file, " \n \n");

      const long xbth =
          player_bth + player_lev * BTH_LEV_ADJ + player_ptohit * BTH_PLUS_ADJ;
      const long xbthb =
          player_bthb + player_lev * BTH_LEV_ADJ + player_ptohit * BTH_PLUS_ADJ;
      long xfos = 27 - player_fos;
      if (xfos < 0) {
        xfos = 0;
      }
      const long xsrh = C_player_curr_search_skill();
      const long xstl = player_stl;
      const long xdis = player_disarm + player_lev + 2 * C_player_disarm_from_dex() +
                  C_player_mod_from_stat(INT);
      const long xsave = player_save + player_lev + C_player_mod_from_stat(WIS);
      const long xdev = player_save + player_lev + C_player_mod_from_stat(INT);
      const long xswm = player_flags.swim + 4;
      const long xrep = 6 + player_rep / 25;
      sprintf(xinfra, "%lld feet", player_flags.see_infra);

      fprintf(file, "\n");

      fprintf(file, "%50s\n \n", "(Miscellaneous Abilities)");

      fprintf(file,
              "  Fighting    : %10s  Stealth     : "
              "%10s  Perception  : %10s\n",
              likert(xbth, 12, s1), likert(xstl, 1, s2), likert(xfos, 3, s3));

      fprintf(file,
              "  Throw/Bows  : %10s  Disarming   : "
              "%10s  Searching   : %10s\n",
              likert(xbthb, 12, s1), likert(xdis, 8, s2), likert(xsrh, 6, s3));

      fprintf(file,
              "  Saving Throw: %10s  Magic Device: "
              "%10s  Infra-Vision: %10s\n",
              likert(xsave, 6, s1), likert(xdev, 7, s2), xinfra);

      fprintf(file,
              "  Reputation  : %10s                   "
              "         Swimming    : %10s\n",
              likert(xrep, 1, s1), likert(xswm, 1, s3));

      /*{ Write out the character's history     }*/

      fprintf(file, " \n \n%45s\n", "Character Background");
      for (i1 = 0; i1 < 5; i1++) {
        fprintf(file, "   %s\n", player_history[i1]);
      }

      /*{ Write out the time stats              }*/

      fprintf(file, " \n \n");

      /* with player_birth. do; */
      day_of_week_string(player_birth.day, 10, out_val);

      for (i1 = 0; out_val[i1];) {
        if (out_val[i1] == ' ') {
          out_val[i1] = 0;
        } else {
          i1++;
        }
      }

      fprintf(file, "  You were born at %s on %s, %s the %s, %lld AH.\n",
              time_string(player_birth.hour, player_birth.secs, s1), out_val,
              month_string(player_birth.month, s2),
              place_string(player_birth.day, s3), player_birth.year);

      fprintf(file, "  %s\n", show_char_age(s1));
      fprintf(file, "  The current time is %s.\n",
              full_date_string(player_cur_age, s1));
      if (player_flags.dead) {
        fprintf(file, "  You were killed by %s on level %ld.\n", died_from,
                dun_level);
      }
      fprintf(file, "  Maximum depth was %d feet.\n", player_max_lev * 50);

      /*{ Write out the equipment list...       }*/
      fprintf(file, " \n \n  [Character's Equipment List]\n \n \n");

      if (equip_ctr == 0) {
        fprintf(file, "  Character has no equipment in use.\n");
      } else {
        long i2 = 0;
        for (i1 = Equipment_min; i1 < EQUIP_MAX; i1++) {
          /* with equipment[i1]. do; */
          if (equipment[i1].tval > 0) {
            char prt2[82];
            switch (i1) {
            case Equipment_primary:
              strcpy(prt1, ") You are "
                           "wielding   : ");
              break;
            case Equipment_helm:
              strcpy(prt1, ") Worn "
                           "on head  "
                           "     : ");
              break;
            case Equipment_amulet:
              strcpy(prt1, ") Worn around "
                           "neck   : ");
              break;
            case Equipment_armor:
              strcpy(prt1, ") Worn "
                           "on body  "
                           "     : ");
              break;
            case Equipment_belt:
              strcpy(prt1, ") Worn around "
                           "body   : ");
              break;
            case Equipment_shield:
              strcpy(prt1, ") Worn on "
                           "shield arm : ");
              break;
            case Equipment_gloves:
              strcpy(prt1, ") Worn "
                           "on hands "
                           "     : ");
              break;
            case Equipment_bracers:
              strcpy(prt1, ") Worn on "
                           "wrists     : ");
              break;
            case Equipment_right_ring:
              strcpy(prt1, ") Right ring "
                           "finger  : ");
              break;
            case Equipment_left_ring:
              strcpy(prt1, ") Left  ring "
                           "finger  : ");
              break;
            case Equipment_boots:
              strcpy(prt1, ") Worn "
                           "on feet  "
                           "     : ");
              break;
            case Equipment_cloak:
              strcpy(prt1, ") Worn about "
                           "body    : ");
              break;
            case Equipment_light:
              strcpy(prt1, ") Light source "
                           "is    : ");
              break;
            case Equipment_secondary:
              strcpy(prt1, ") Secondary "
                           "weapon   : ");
              break;
            default:
              strcpy(prt1, ") *Unknown "
                           "equipment*: ");
              break;
            }
            i2++;
            inven_temp.data = equipment[i1];
            objdes(prt2, &inven_temp, true);
            if ((inven_temp.data.flags2 & Insured_bit) == 0) {
              sprintf(out_val, "  %c%s%s", (char)(i2 + 96), prt1, prt2);
            } else {
              sprintf(out_val, " (%c%s%s", (char)(i2 + 96), prt1, prt2);
            }
            fprintf(file, "%s\n", out_val);
          }
        } /* end for equipment */
      }   /* end else has equipment */

      /*{ Write out the character's inventory...        }*/

      fprintf(file, "%c \n \n \n  %s\n \n \n", new_page,
              "[General Inventory List]");

      if (inven_ctr == 0) {
        fprintf(file, "  Character has no objects in "
                       "inventory.\n");
      } else {
        i1 = 1;
        for (const treas_rec *curse = inventory_list; curse != NULL; curse = curse->next) {
          if (i1 % 50 == 0) {
            fprintf(file,
                    "%c\n \n \n \n  "
                    "%s %ld]\n \n",
                    new_page,
                    "[General "
                    "Inventory "
                    "List, Page",
                    i1 / 50 + 1);
          }
          inven_temp.data = curse->data;
          objdes(s1, &inven_temp, true);
          if (curse->is_in) {
            sprintf(prt1, "    %s", s1);
          } else {
            sprintf(prt1, "%s", s1);
          }

          if (i1 < 27) {
            if ((inven_temp.data.flags2 & Insured_bit) == 0) {
              sprintf(out_val, " %c) %s", (char)(i1 + 96), prt1);
            } else {
              sprintf(out_val, "(%c) %s", (char)(i1 + 96), prt1);
            }
          } else {
            if ((inven_temp.data.flags2 & Insured_bit) == 0) {
              sprintf(out_val, " *) %s", prt1);
            } else {
              sprintf(out_val, "(*) %s", prt1);
            }
          }

          if ((inven_temp.data.flags2 & Holding_bit) != 0) {
            bag_descrip(curse, s1);
            strcat(out_val, s1);
          }

          fprintf(file, "%s\n", out_val);
          i1++;
        } /* end for inventory */
      }   /* end else inventory not null */

      fclose(file);
      prt("Completed.", 1, 1);
    } /* end if file !NULL */
  }   /* end get_string */
}

void print_map(void) {
  /*{ Prints dungeon map to external file                       -RAK- }*/

  char filename1[81];
  char tmp;

  prt("File name: ", 1, 1);
  if (get_string(filename1, 1, 12, 64)) {
    if (strlen(filename1) == 0) {
      strcpy(filename1, "MORIAMAP.DAT");
    }
    FILE *file1 = fopen(filename1, "w");
    if (file1 != NULL) {
      long i6;
      long i5;
      prt("Writing Moria Dungeon Map...", 1, 1);
      refresh();
      long i1 = 1;
      long i7 = 0;
      do {
        long i2 = 1;
        long i3 = i1 + OUTPAGE_HEIGHT - 1;
        if (i3 > cur_height) {
          i3 = cur_height;
        }
        i7++;
        long i8 = 0;
        do {
          long i4 = i2 + OUTPAGE_WIDTH - 1;
          if (i4 > cur_width) {
            i4 = cur_width;
          }
          i8++;
          fprintf(file1, "%c", 12);
          fprintf(file1, "Section[%ld,%ld];     ", i7, i8);
          fprintf(file1, "Depth : %ld (feet)\n", dun_level * 50);
          fprintf(file1, " \n");
          fprintf(file1, "   ");
          for (i5 = i2; i5 <= i4; i5++) {
            i6 = i5 / 100;
            fprintf(file1, "%ld", i6);
          }
          fprintf(file1, "\n");
          fprintf(file1, "   ");
          for (i5 = i2; i5 <= i4; i5++) {
            i6 = (int)(i5 / 10) - (int)(i5 / 100) * 10;
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
            char dun_line[OUTPAGE_WIDTH + 10];
            sprintf(dun_line, "%3ld", i5);
            for (i6 = i2; i6 <= i4; i6++) {
              if (test_light(i5, i6)) {
                tmp = loc_symbol(i5, i6) & 0x7F;
              } else {
                tmp = ' ';
              }
              strncat(dun_line, &tmp, 1);
            }
            fprintf(file1, "%s\n", dun_line);
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

void print_objects(void) {
  /*{ Prints a list of random objects to a file.  Note that   -RAK-  }*/
  /*{ the objects produced is a sampling of objects which            }*/
  /*{ be expected to appear on that level.                           }*/

  long nobj, i2, level;
  char tmp_str[82];

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
  if (nobj > 0 && level > -1 && level < 1201) {
    char filename1[82];
    if (nobj > 99999) {
      nobj = 99999;
    }
    prt("File name: ", 1, 1);
    if (get_string(filename1, 1, 12, 64)) {
      if (filename1[0] == 0) {
        strcpy(filename1, "MORIAOBJ.DAT");
      }
      FILE *file1 = fopen(filename1, "w");
      if (file1 != NULL) {
        sprintf(tmp_str, "%ld random objects being produced...", nobj);
        prt(tmp_str, 1, 1);
        refresh();

        fprintf(file1, "*** Random Object Sampling:\n");
        fprintf(file1, "*** %ld objects\n", nobj);
        fprintf(file1, "*** For Level %ld\n\n\n", level);
        popt(&i2);

        for (long i1 = 1; i1 <= nobj; i1++) {
          t_list[i2] = generate_item_for_dungeon_level(level);
          inven_temp.data = t_list[i2];
          /* with inven_temp->data. do; */
          unquote(inven_temp.data.name);
          known1(inven_temp.data.name);
          known2(inven_temp.data.name);
          objdes(tmp_str, &inven_temp, true);
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

void print_monsters(void) {
  /*{ Prints a listing of monsters                              -RAK- }*/

  long atype, adesc, acount;
  char filename1[82];

  prt("File name for monsters: ", 1, 1);
  if (get_string(filename1, 1, 25, 64)) {
    if (filename1[0] == 0) {
      strcpy(filename1, "MORIAMON.DAT");
    }
    FILE *file1 = fopen(filename1, "w");
    if (file1 != NULL) {
      char attx[82];
      char attstr[82];
      char out_val[82];
      prt("Writing Monster Dictionary...", 1, 1);
      refresh();
      for (long i1 = 1; i1 <= MAX_CREATURES; i1++) {
        /* with monster_templates[i1]. do; */

        const unsigned long cmove = monster_templates[i1].cmove;
        const unsigned long cdefense = monster_templates[i1].cdefense;
        const unsigned long spells = monster_templates[i1].spells;

        /*{ Begin writing to file }*/
        fprintf(file1, "-------------------------------"
                       "-------------\n");

        fprintf(file1, "%3ld  %30s     (%c)\n", i1, monster_templates[i1].name,
                monster_templates[i1].cchar);

        fprintf(file1,
                "     Speed =%2d  Level     "
                "=%2d  Exp =%5lld\n",
                monster_templates[i1].speed, monster_templates[i1].level, monster_templates[i1].mexp);

        fprintf(file1,
                "     AC    =%2d  Eye-sight "
                "=%2d  HD  =%5s\n",
                monster_templates[i1].ac, monster_templates[i1].aaf, monster_templates[i1].hd);

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
          fprintf(file1, "     Creature is a dragon.\n");
        }
        if ((0x0002 & cdefense) != 0) {
          fprintf(file1, "     Creature is a monster.\n");
        }
        if ((0x0400 & cdefense) != 0) {
          fprintf(file1, "     Creature is a demon.\n");
        }
        if ((0x0004 & cdefense) != 0) {
          fprintf(file1, "     Creature is evil.\n");
        }
        if ((0x0008 & cdefense) != 0) {
          fprintf(file1, "     Creature is undead.\n");
        }
        if ((0x0010 & cdefense) != 0) {
          fprintf(file1, "     Creature harmed by cold.\n");
        }
        if ((0x0020 & cdefense) != 0) {
          fprintf(file1, "     Creature harmed by fire.\n");
        }
        if ((0x0040 & cdefense) != 0) {
          fprintf(file1, "     Creature harmed "
                         "by poison.\n");
        }
        if ((0x0080 & cdefense) != 0) {
          fprintf(file1, "     Creature harmed by acid.\n");
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
          fprintf(file1, "     Creature regenerates.\n");
        }
        if ((0x00010000 & cmove) != 0) {
          fprintf(file1, "     Creature is invisible.\n");
        }
        if ((0x00100000 & cmove) != 0) {
          fprintf(file1, "     Creature picks up "
                         "objects.\n");
        }
        if ((0x00200000 & cmove) != 0) {
          fprintf(file1, "     Creature multiplies.\n");
        }
        if ((0x01000000 & cmove) != 0) {
          fprintf(file1, "     Carries object(s).\n");
        }
        if ((0x02000000 & cmove) != 0) {
          fprintf(file1, "     Carries gold, gems, ect.\n");
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
          fprintf(file1, "       Has 1d2 object(s)/gold.\n");
        }
        if ((0x20000000 & cmove) != 0) {
          fprintf(file1, "       Has 2d2 object(s)/gold.\n");
        }
        if ((0x40000000 & cmove) != 0) {
          fprintf(file1, "       Has 4d2 object(s)/gold.\n");
        }

        /*{ Creature casts spells / Breaths Dragon
         * breath...      }*/
        if (spells > 0) {

          fprintf(file1, "   --Spells/Dragon Breath =\n");
          if ((spells & 0x80000000) != 0) {
            fprintf(file1,
                    "       Doesn't "
                    "cast spells 1 "
                    "out of %ld "
                    "turns.\n",
                    0xF & spells);
          } else {
            fprintf(file1,
                    "       Casts spells 1 "
                    "out of %ld turns.\n",
                    0xF & spells);
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
            fprintf(file1, "       Teleport "
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
            fprintf(file1, "       Hold person.\n");
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
            fprintf(file1, "       Cause fear.\n");
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
            fprintf(file1, "       Slow person.\n");
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
            fprintf(file1, "       Casts Illusion.\n");
          }
          if ((0x02000000 & spells) != 0) {
            fprintf(file1, "       Summon a demon.\n");
          }
          if ((0x04000000 & spells) != 0) {
            fprintf(file1, "       Summon "
                           "multiplying "
                           "monster.\n");
          }
          if ((0x08000000 & spells) != 0) {
            fprintf(file1, "       Gaze for "
                           "petrification.\n");
          }
        } /* end if spells > 0 */

        /*{ Movement for creature }*/
        fprintf(file1, "   --Movement =\n");
        if ((0x00000001 & cmove) != 0) {
          fprintf(file1, "       Move only to attack.\n");
        }
        if ((0x00000002 & cmove) != 0) {
          fprintf(file1, "       20%% random movement.\n");
        }
        if ((0x00000004 & cmove) != 0) {
          fprintf(file1, "       40%% random movement.\n");
        }
        if ((0x00000008 & cmove) != 0) {
          fprintf(file1, "       75%% random movement.\n");
        }
        if ((0x00400000 & cmove) != 0) {
          fprintf(file1, "      Creature can "
                         "anchor in water.\n");
        }
        if ((0x00800000 & cmove) != 0) {
          fprintf(file1, "       Creature flies.\n");
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
          fprintf(file1, "       Can open doors.\n");
        }
        if ((0x00040000 & cmove) != 0) {
          fprintf(file1, "       Can phase "
                         "through walls.\n");
        }

        fprintf(file1, "   --Creature attacks =\n");
        attstr[0] = 0;
        attx[0] = 0;
        strcpy(attstr, monster_templates[i1].damage);

        while (attstr[0] != 0) {
          char damstr[36];

          /* attstr looks like this: "1 32 4d4|2
           * 21 0d0" */

          char *achar = strstr(attstr, "|");
          if (achar != NULL) {
            strcpy(attx, attstr);
            achar = strstr(attx, "|");
            *achar = 0;
            achar++;
            strcpy(attstr, achar);
          } else {
            strcpy(attx, attstr);
            attstr[0] = 0;
          }

          sscanf(attx, "%ld %ld %s", &atype, &adesc, damstr);
          out_val[0] = 0;

          if ((achar = strstr(damstr, "-")) != NULL) {
            char s1[82];
            *achar = ' ';
            sscanf(damstr, "%ld %s", &acount, s1);
            strcpy(damstr, s1);
          } else {
            acount = 1;
          }

          for (long i5 = 1; i5 <= acount; i5++) {
            switch (adesc) {
            case 0:
              strcpy(out_val, "       < No "
                              "Print > for ");
              break;
            case 1:
              strcpy(out_val, "       Hits for ");
              break;
            case 2:
              strcpy(out_val, "       Bites "
                              "for ");
              break;
            case 3:
              strcpy(out_val, "       Claws "
                              "for ");
              break;
            case 4:
              strcpy(out_val, "       Stings "
                              "for ");
              break;
            case 5:
              strcpy(out_val, "       Touches "
                              "for ");
              break;
            case 6:
              strcpy(out_val, "       Kicks "
                              "for ");
              break;
            case 7:
              strcpy(out_val, "       Gazes "
                              "for ");
              break;
            case 8:
              strcpy(out_val, "       "
                              "Breathes for ");
              break;
            case 9:
              strcpy(out_val, "       Spits "
                              "for ");
              break;
            case 10:
              strcpy(out_val, "       Wails "
                              "for ");
              break;
            case 11:
              strcpy(out_val, "       "
                              "Embraces for ");
              break;
            case 12:
              strcpy(out_val, "       Crawls "
                              "on you for ");
              break;
            case 13:
              strcpy(out_val, "       Shoots "
                              "spores for ");
              break;
            case 14:
              strcpy(out_val, "       Begs "
                              "for money "
                              "for ");
              break;
            case 15:
              strcpy(out_val, "       Slimes "
                              "you for ");
              break;
            case 16:
              strcpy(out_val, "       Crushes "
                              "you for ");
              break;
            case 17:
              strcpy(out_val, "       "
                              "Tramples you "
                              "for ");
              break;
            case 18:
              strcpy(out_val, "       Drools "
                              "on you for ");
              break;
            case 19:
              strcpy(out_val, "       Insults "
                              "you for ");
              break;
            case 20:
              strcpy(out_val, "       UW's "
                              "you for ");
              break;
            case 21:
              strcpy(out_val, "       DMF's "
                              "you for ");
              break;
            case 22:
              strcpy(out_val, "       "
                              "Cultivates you "
                              "for ");
              break;
            case 23:
              strcpy(out_val, "       Charms "
                              "you for ");
              break;
            case 24:
              strcpy(out_val, "       Kisses "
                              "you for ");
              break;
            case 25:
              strcpy(out_val, "       Gores "
                              "you for ");
              break;
            case 26:
              strcpy(out_val, "       Moo's "
                              "you for ");
              break;
            case 27:
              strcpy(out_val, "       "
                              "Electrocutes "
                              "you for ");
              break;
            case 28:
              strcpy(out_val, "       Inks "
                              "you for ");
              break;
            case 29:
              strcpy(out_val, "      "
                              " Entan"
                              "gles "
                              "for ");
              break;
            case 30:
              strcpy(out_val, "       Sucks "
                              "blood for ");
              break;
            case 31:
              strcpy(out_val, "       Throat "
                              "attacks for ");
              break;
            case 32:
              strcpy(out_val, "       Blows "
                              "bubbles for ");
              break;
            case 33:
              strcpy(out_val, "       Squawks "
                              "for ");
              break;
            case 34:
              strcpy(out_val, "       Pecks "
                              "for ");
              break;
            case 35:
              strcpy(out_val, "       Barks "
                              "for ");
              break;
            case 36:
              strcpy(out_val, "       Rubs "
                              "leg for ");
              break;
            case 37:
              strcpy(out_val, "       Follows "
                              "for ");
              break;

            case 99:
              strcpy(out_val, "       Is "
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
              strcat(out_val, "normal damage.");
              break;
            case 2:
              strcat(out_val, "lowering "
                              "strength.");
              break;
            case 3:
              strcat(out_val, "confusion.");
              break;
            case 4:
              strcat(out_val, "fear.");
              break;
            case 5:
              strcat(out_val, "fire damage.");
              break;
            case 6:
              strcat(out_val, "acid damage.");
              break;
            case 7:
              strcat(out_val, "cold damage.");
              break;
            case 8:
              strcat(out_val, "lightning "
                              "damage.");
              break;
            case 9:
              strcat(out_val, "corrosion "
                              "damage.");
              break;
            case 10:
              strcat(out_val, "blindness.");
              break;
            case 11:
              strcat(out_val, "paralyzation.");
              break;
            case 12:
              strcat(out_val, "stealing money.");
              break;
            case 13:
              strcat(out_val, "stealing object.");
              break;
            case 14:
              strcat(out_val, "poison damage.");
              break;
            case 15:
              strcat(out_val, "lose dexterity.");
              break;
            case 16:
              strcat(out_val, "lose "
                              "constitution.");
              break;
            case 17:
              strcat(out_val, "lose "
                              "intelligence.");
              break;
            case 18:
              strcat(out_val, "lose wisdom.");
              break;
            case 19:
              strcat(out_val, "lose experience.");
              break;
            case 20:
              strcat(out_val, "aggravates "
                              "monsters.");
              break;
            case 21:
              strcat(out_val, "disenchants "
                              "objects.");
              break;
            case 22:
              strcat(out_val, "eating food.");
              break;
            case 23:
              strcat(out_val, "eating light "
                              "source.");
              break;
            case 24:
              strcat(out_val, "absorbing "
                              "charges.");
              break;
            case 25:
              strcat(out_val, "lose charisma.");
              break;
            case 26:
              strcat(out_val, "petrification.");
              break;
            case 27:
              strcat(out_val, "poision.");
              break;
            case 99:
              strcat(out_val, "blank message.");
              break;
            default:
              sprintf(out_val,
                      "** Unknown "
                      "damage: %ld**",
                      atype);
              break;
            } /* end switch */

            fprintf(file1, "%s (%s)\n", out_val, damstr);

          } /* end acount */
        }   /* end for attstr */

        fprintf(file1, "   --Magic Resistance : ");
        if (monster_templates[i1].mr == 0) {
          fprintf(file1, "None\n");
        } else if (monster_templates[i1].mr < 20) {
          fprintf(file1, "Very Low\n");
        } else if (monster_templates[i1].mr < 50) {
          fprintf(file1, "Low\n");
        } else if (monster_templates[i1].mr < 80) {
          fprintf(file1, "Medium\n");
        } else if (monster_templates[i1].mr < 110) {
          fprintf(file1, "High\n");
        } else if (monster_templates[i1].mr < 140) {
          fprintf(file1, "Very High\n");
        } else {
          fprintf(file1, "Extreme\n");
        }

        fprintf(file1, " \n \n");

      } /* end for i1 */

      fclose(file1);
      prt("Completed.", 1, 1);

    } /* end file1 != NULL */
  }   /* end get filename */
}


char *center(char str[134], int len, char result[134]) {
  int const in_str_len = strlen(str);
  int const j = (len - in_str_len) / 2;

  if (in_str_len >= len) {
    strncpy(result, str, len);
  } else {
    snprintf(result, 134, "%*s%s%*s", j, "", str, len - in_str_len - j, "");
  }

  return result;
}

bool open_crypt_file(char prompt[82], char fnam1[82], char fnam2[82],
                        FILE **f1, FILE **f2) {
  bool flag = true;

  if (fnam1[0] == 0) {
    prt(prompt, 1, 1);
    flag = get_string(fnam1, 1, strlen(prompt) + 1, 80);
  }

  if (flag) {
    char out_str[1026];

    *f1 = fopen(fnam1, "r");
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
          *f2 = fopen(fnam2, "w");
          if (*f2 == NULL) {
            sprintf(out_str, "Error Opening> %s", fnam2);
            prt(out_str, 1, 1);
            prt(" ", 2, 1);
            flag = false;
          }
        } else {
          sprintf(out_str, "Use differnt names> %s", fnam2);
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
