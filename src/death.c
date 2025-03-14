#include <curses.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "configure.h"
#include "constants.h"
#include "files.h"
#include "io.h"
#include "master.h"
#include "misc.h"
#include "player.h"
#include "save.h"
#include "term.h"
#include "unix.h"
#include "variables.h"

#include "death.h"

long max_score = 20;

static char *ud__fill_str(char *centered_str, char *in_str) {
  /* Centers a string within a 31 character string  -JWT- */

  const int i = strlen(in_str);
  const int j = 15 - i / 2;
  (void)sprintf(centered_str, "%*s%s%*s", j, "", in_str, 31 - i - j, "");
  return centered_str;
}

void dprint(char str[82], const long row) {
  /* Prints a line to the screen efficiently  -RAK- */

  char prt_str[82];
  long nblanks = 0;
  long xpos = 0;
  long const slen = strlen(str);
  long i1 = 0;
  long i2 = 0;

  prt_str[0] = 0;
  for (i1 = 0; i1 < slen; i1++) {
    /* printf("\tdo a char: %d >%c<\n",i1,str[i1]); fflush(stdout);
     */
    if (str[i1] == ' ') {
      if (xpos > 0) {
        nblanks++;
        if (nblanks > 5) {
          nblanks = 0;
          put_buffer(prt_str, row, xpos);
          prt_str[0] = 0;
          xpos = 0;
        }
      }
    } else {
      if (xpos == 0) {
        xpos = i1;
      }
      if (nblanks > 0) {
        for (i2 = 0; i2 < nblanks; i2++) {
          strcat(prt_str, " ");
        }
        nblanks = 0;
      }
      strncat(prt_str, &str[i1], 1);
    }
  }

  if (xpos > 0) {
    /*    printf("doing final put_buffer >%s<\n",prt_str); */
    /*    fflush(stdout); */
    put_buffer(prt_str, row, xpos);
  }
}

void ud__kingly(void) {
  /*
   * Change the player into a King!   -RAK-
   *
   * Change the character attributes...
   */

  dun_level = 0;
  strcpy(died_from, "Ripe Old Age");

  if (characters_sex() == MALE) {
    strcat(player_tclass, " King");
  } else {
    strcat(player_tclass, " Queen");
  }

  player_lev += MAX_PLAYER_LEVEL;
  player_account += 250000;
  player_max_exp += 5000000;
  player_exp = player_max_exp;

  /* Let the player know that he did good...  */
  clear_from(1);
  dprint("                                  #", 2);
  dprint("                                #####", 3);
  dprint("                                  #", 4);
  dprint("                            ,,,  $$$  ,,,", 5);
  dprint("                        ,,=$   \"$$$$$\"   $=,,", 6);
  dprint("                      ,$$        $$$        $$,", 7);
  dprint("                      *>         <*>         <*", 8);
  dprint("                      $$         $$$         $$", 9);
  dprint("                      \"$$        $$$        $$\"", 10);
  dprint("                       \"$$       $$$       $$\"", 11);
  dprint("                        *#########*#########*", 12);
  dprint("                        *#########*#########*", 13);
  dprint("                          Veni, Vidi, Vici!", 16);
  dprint("                     I came, I saw, I conquered!", 17);
  dprint("                      All Hail the Mighty King!", 18);
  flush();
  pause_game(24);
}

void ud__print_tomb(char dstr[][82]) {
  /*  Prints the gravestone of the character  -RAK-  */

  if (player_lev > 10) {
    char user[82];
    user_name(user);
    user[12] = 0;
    FILE *f1 = fopen(DEATH_FILE, "r+");
    if (f1 != NULL) {
      char temp[82];
      fseek(f1, 0, SEEK_END);
      if (player_cheated) {
        fprintf(f1, "*%-12s %2d %2d %2d %4ld %4d %s\n", user, player_prace,
                player_pclass, player_lev, dun_level, player_max_lev,
                died_from);
        fprintf(f1, "%50s %s\n", "", show_current_time(temp));
      } else {
        fprintf(f1, " %-12s %2d %2d %2d %4ld %4d %s\n", user, player_prace,
                player_pclass, player_lev, dun_level, player_max_lev,
                died_from);
        fprintf(f1, "%50s %s\n", "", show_current_time(temp));
        /* fprintf(f1," %44s %s\n", player_ssn, */
        /* show_current_time(temp)); */
      }
      fclose(f1);
    } /* endif f1 != NULL */
  }   /* endif level > 10 */
  make_tomb(dstr);
  write_tomb(dstr);
}

void upon_death(void) {
  /*  Handles the gravestone and top-twenty routines -RAK-  */
  char dstr[20][82];

  player_flags.dead = true;

  /*  What happens upon dying...    -RAK- */
  if (!mst__update_character(player_uid)) {
    msg_print("ERROR opening file MASTER. Contact your local wizard.");
    msg_print(" ");
  }
  C_delete_character();

  player_flags.dead = false;

  if (total_winner) {
    ud__kingly();
  }
  ud__print_tomb(dstr);
  print_dead_character();
  exit_game();
}

void print_dead_character(void) {
  /* Allow the bozo to print out his dead character... -KRC- */
  if (get_yes_no("Print character sheet to file?")) {
    file_character();
  }
}

static void sc(char *out_str, const char *in_str) { strcpy(out_str, in_str); }

static void si(char *out_str, char *part1, char *part2, char *part3) {
  sprintf(out_str, "%s%s%s", part1, part2, part3);
}

static void date(char *day) {
  const long clockvar = time(0);
  char *tmp = ctime(&clockvar);
  tmp[10] = '\0';
  (void)strcpy(day, tmp);
}

void death_by_quitting(void) {
  /* this can be called from signalquit in io.c */

  char command;

  flush();
  if (get_com("Enter 'Q' to quit (and kill this character)", &command)) {
    switch (command) {
    case 'q':
    case 'Q':
      strcpy(died_from, "Ripe Old Age");
      moria_flag = true;
      death = true;
      break;
    default:
      break;
    }
  }

  erase_line(1, 1);
  refresh();
}

void make_tomb(char dstr[][82]) {
  char str1[82];
  char str2[82];
  char str3[82];
  char str4[82];
  char str5[82];
  char str6[82];
  char str7[82];
  char str8[82];
  char temp[83];
  char day[11];

  date(day);
  ud__fill_str(str1, player_name);
  ud__fill_str(str2, "");
  ud__fill_str(str3, player_tclass);
  sprintf(temp, "Level : %d", player_lev);
  ud__fill_str(str4, temp);
  sprintf(temp, "%ld Exp", player_exp);
  ud__fill_str(str5, temp);
  sprintf(temp, "%ld Au", (player_account + player_money[TOTAL_]));
  ud__fill_str(str6, temp);
  sprintf(temp, "Died on Level : %ld", dun_level);
  ud__fill_str(str7, temp);
  sprintf(temp, "%s.", died_from);
  ud__fill_str(str8, temp);

  sc(dstr[0], " ");
  sc(dstr[1], "               _______________________");
  sc(dstr[2], "              /                       \\         ___");
  sc(dstr[3], "             /                         \\ ___   /   \\      ___");
  sc(dstr[4],
     "            /            RIP            \\   \\  :   :     /   \\");
  sc(dstr[5], "           /                             \\  : _;,,,;_    :   :");
  si(dstr[6], "          /", str1, "\\,;_          _;,,,;_");
  sc(dstr[7], "         |                                 |   ___");
  si(dstr[8], "         | ", str2, " |  /   \\");
  sc(dstr[9], "         |                                 |  :   :");
  si(dstr[10], "         | ", str3, " | _;,,,;_   ____");
  si(dstr[11], "         | ", str4, " |          /    \\");
  si(dstr[12], "         | ", str5, " |          :    :");
  si(dstr[13], "         | ", str6, " |          :    :");
  si(dstr[14], "         | ", str7, " |         _;,,,,;_");
  sc(dstr[15], "         |            killed by            |");
  si(dstr[16], "         | ", str8, " |");
  si(dstr[17], "         |           ", day, "            |");
  sc(dstr[18], "        *|   *     *     *    *   *     *  | *");
  sc(dstr[19], " _______)/\\\\_)_/___(\\/___(//_\\)/_\\//__\\\\(/_|_)_______");

  clear_from(1);
  for (long i1 = 0; i1 < 20; i1++) {
    /* printf(">%s<\n",dd[i1]); */
    dprint(dstr[i1], i1 + 1);
  }
  flush();
}

void write_tomb(char dstr[][82]) {

  if (!get_yes_no("Print to file?"))
    return;

  prt("Enter Filename:", 1, 1);
  bool flag = false;
  do {
    char fnam[82];
    if (get_string(fnam, 1, 17, 60)) {
      if (strlen(fnam) == 0) {
        strcpy(fnam, "MORIACHR.DIE");
      }
      FILE *f1 = fopen(fnam, "w");
      if (f1 == NULL) {
        char out_str[120];
        sprintf(out_str, "Error creating> %s", fnam);
        prt(out_str, 2, 1);
      } else {
        flag = true;
        for (long i1 = 0; i1 < 20; i1++) {
          fprintf(f1, "%s\n", dstr[i1]);
        }
      }
      fclose(f1);
    } else {
      flag = true;
    }
  } while (!flag);
}
