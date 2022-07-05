/* wizard.c */
/**/

#include <curses.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "configure.h"
#include "constants.h"
#include "debug.h"
#include "main_loop.h"
#include "magic.h"
#include "pascal.h"
#include "player.h"
#include "stores.h"
#include "term.h"
#include "types.h"
#include "variables.h"
#include "creature.h"
#include "desc.h"
#include "files.h"
#include "screen.h"
#include "player_action.h"
#include "spells.h"
#include "misc.h"
#include "random.h"
#include "monsters.h"

#define LOW_NUM -98765432

#define LOW_PASSWORD "fragrance"
#define HIGH_PASSWORD "mopwillow"

#define PB(x, y, z) put_buffer((x), (y), (z))

static boolean became_wizard = false;

void game_version() {
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
  PB("Student/University of Oklahoma    Student/University of Oklahoma", 18, 1);
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

boolean check_pswd(char passwd[134], boolean present) {
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
  /* lesser op password */
  if (!(strcmp(tpw, LOW_PASSWORD))) {
    checked_out = true;
    wizard1 = true;
    /* full op password */
  } else if (!strcmp(tpw, HIGH_PASSWORD)) {
    checked_out = true;
    wizard1 = true;
    wizard2 = true;
  }

  msg_flag = false;
  if (!present) {
    erase_line(msg_line, msg_line);
  }

  player_cheated |= checked_out;
  became_wizard = checked_out;

  LEAVE("check_pswd", "");

  return checked_out;
}

void wizard_light() {
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
                            boolean sleepy) {
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
          if (is_in(cave[i3][i4].fval, floor_set)) {
            if (cave[i3][i4].fopen) {
              place_monster(i3, i4, i2, sleepy);
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
        if ((strstr(c_list[i2].name, monster) != NULL) && (i1 != 10)) {
          i1 = 0;
          do {
            i3 = y - 2 + randint(3);
            i4 = x - 2 + randint(3);
            if (in_bounds(i3, i4)) {
              /*with cave[i3,i4] do*/
              if (is_in(cave[i3][i4].fval, floor_set)) {
                if (cave[i3][i4].cptr == 0) {
                  if (cave[i3][i4].fopen) {
                    place_monster(i3, i4, i2, sleepy);
                    i1 = 9;
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
  }   /* end if junk */

  if (!present) {
    erase_line(msg_line, msg_line);
  }
}

void enter_wizard_mode(boolean ask_for_pass) {
  if (wizard1) {
    msg_print("Wizard mode off.");
    wizard1 = false;
    wizard2 = false;
    reset_flag = true;
    no_controly();
  } else {
    if (became_wizard || !ask_for_pass) {
      /*if (check_pswd("doublespeak",true)) {*/
      if (check_pswd(LOW_PASSWORD, true)) {
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

void esf__display_commands() {
  prt("You may:", 21, 1);
  prt(" d) Delete an entry.              b) Browse to next page.", 22, 1);
  prt(" c) Change an entry.", 23, 1);
  prt(" q) Quit and save changes       Esc) Exit without saving.", 24, 1);
}

void esf__display_list(int start, char list[][134], int n1, int *blegga,
                       int *cur_display_size) {
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
                         int *cur_display_size) {
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

boolean esf__get_list_entry(int *l_command, char pmt[82], int cur_top, int i1,
                            int i2) {
  char out_val[82];
  boolean flag = true;

  *l_command = 0;

  sprintf(out_val, "(Entries %c-%c, Esc to exit) %s", (char)(i1 + 96),
          (char)(i2 + 96), pmt);

  for (; (((*l_command < i1) || (*l_command > i2)) && (flag));) {
    prt(out_val, 1, 1);
    *l_command = inkey();

    switch (*l_command) {
    case 3:
    case 25:
    case 26:
    case 27:
      flag = false;
      break;
    default:
      (*l_command) += (cur_top - 97);
      break;
    }
  }

  erase_line(1, 1);
  return flag;
}

void esf__delete_entry(int cur_top, char list[][134], int *n1,
                       int cur_display_size) {
  int which, i1;

  if (cur_display_size > 0) {
    if (esf__get_list_entry(&which, " Delete which one?", cur_top, 1,
                            cur_top + cur_display_size - 1)) {

      for (i1 = which; i1 < *n1; i1++) {
        strcpy(list[i1], list[i1 + 1]);
      }
      (*n1)--;
    }
  }
}

void esf__parse_command(char list[][134], int *cur_top, int *n1, int *blegga,
                        int *cur_display_size, boolean *exit_flag,
                        boolean *want_save) {
  char command;

  if (get_com("", &command)) {

    switch (command) {

    case 18: /*{^R}*/
      esf__display_screen(*cur_top, list, *n1, blegga, cur_display_size);
      break;

    case 32: /*{ }*/
    case 98: /*{b}*/
      if (*cur_top == *blegga) {
        prt("Entire list is displayed.", 1, 1);
      } else {
        *cur_top = *blegga;
        esf__display_list(*cur_top, list, *n1, blegga, cur_display_size);
      }
      break;

    case 100: /*{d}*/
      esf__delete_entry(*cur_top, list, n1, *cur_display_size);
      esf__display_list(*cur_top, list, *n1, blegga, cur_display_size);
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
                        boolean *ok) {
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

void change_character() {
  /*{ Wizard routine for gaining on stats                   -RAK-   }*/

  long tmp_val;
  char tmp_str[82];
  enum stat_t tstat;
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
    tmp_val = 0;
    if (cc__input_field("(1-32767) Hit points = ", &tmp_val, 1, 32767, &flag)) {
      if (flag) {
        C_player_modify_max_hp(tmp_val);
        prt_stat_block();
      }
    } else {
      abort = true;
    }
  }

  if (!abort) {
    tmp_val = player_mana;
    if (cc__input_field("(0-32767) Mana = ", &tmp_val, 0, 32767, &flag)) {
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
    if (cc__input_field("(0-200) Searching = ", &tmp_val, 0, 200, &flag)) {
      C_player_mod_search_skill(tmp_val);
    } else {
      abort = true;
    }
  }

  if (!abort) {
    tmp_val = player_stl;
    if (cc__input_field("(0-10) Stealth = ", &tmp_val, 0, 10, &flag)) {
      player_stl = tmp_val;
    } else {
      abort = true;
    }
  }

  if (!abort) {
    tmp_val = player_disarm;
    if (cc__input_field("(0-200) Disarming = ", &tmp_val, 0, 200, &flag)) {
      player_disarm = tmp_val;
    } else {
      abort = true;
    }
  }

  if (!abort) {
    tmp_val = player_save;
    if (cc__input_field("(0-100) Save = ", &tmp_val, 0, 100, &flag)) {
      player_save = tmp_val;
    } else {
      abort = true;
    }
  }

  if (!abort) {
    tmp_val = player_bth;
    if (cc__input_field("(0-200) Base to hit = ", &tmp_val, 0, 200, &flag)) {
      player_bth = tmp_val;
    } else {
      abort = true;
    }
  }

  if (!abort) {
    tmp_val = player_bthb;
    if (cc__input_field("(0-200) Bows/Throwing = ", &tmp_val, 0, 200, &flag)) {
      player_bthb = tmp_val;
    } else {
      abort = true;
    }
  }

  if (!abort) {
    tmp_val = player_money[TOTAL_];
    if (cc__input_field("Player Gold = ", &tmp_val, 0, 100000000, &flag)) {
      if (flag) {
        tmp_val = (tmp_val - player_money[TOTAL_]) * GOLD_VALUE;
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
    if (!cc__input_field("Account Gold = ", &player_account, 0, 1000000000,
                         &flag)) {
      abort = true;
    }
  }

  if (!abort) {
    tmp_val = inven_weight;
    if (cc__input_field("Current Weight (100/unit weight) = ", &tmp_val, 0,
                        900000, &flag)) {
      inven_weight = tmp_val;
      prt_stat_block();
    }
  }

  erase_line(msg_line, msg_line);
  py_bonuses(&blank_treasure, 0);
}

void wizard_create() {
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
    strcpy(inven_temp.data.name, tmp_str);
  } else {
    strcpy(inven_temp.data.name, "& Wizard Object!");
  }

  do {
    prt("Tval   : ", 1, 1);
    get_string(tmp_str, 1, 10, 10);
    tmp_val = 0;
    sscanf(tmp_str, "%ld", &tmp_val);
    flag = true;

  } while (!flag);

  inven_temp.data.tval = tmp_val;

  prt("Flags  (In HEX): ", 1, 1);
  inven_temp.data.flags = get_hex_value(1, 18, 8);

  prt("Flags2 (In HEX): ", 1, 1);
  inven_temp.data.flags2 = get_hex_value(1, 18, 8);

  prt("P1     : ", 1, 1);
  get_string(tmp_str, 1, 10, 10);
  tmp_val = 0;
  sscanf(tmp_str, "%ld", &tmp_val);
  inven_temp.data.p1 = tmp_val;

  prt("Cost : ", 1, 1);
  get_string(tmp_str, 1, 10, 10);
  tmp_val = 0;
  sscanf(tmp_str, "%ld", &tmp_val);
  inven_temp.data.cost = tmp_val;

  prt("Subval : ", 1, 1);
  get_string(tmp_str, 1, 10, 10);
  tmp_val = 1;
  sscanf(tmp_str, "%ld", &tmp_val);
  inven_temp.data.subval = tmp_val;

  prt("Weight : ", 1, 1);
  get_string(tmp_str, 1, 10, 10);
  tmp_val = 1;
  sscanf(tmp_str, "%ld", &tmp_val);
  inven_temp.data.weight = tmp_val;

  prt("Number : ", 1, 1);
  get_string(tmp_str, 1, 10, 10);
  tmp_val = 1;
  sscanf(tmp_str, "%ld", &tmp_val);
  inven_temp.data.number = tmp_val;

  prt("+To hit: ", 1, 1);
  get_string(tmp_str, 1, 10, 10);
  tmp_val = 0;
  sscanf(tmp_str, "%ld", &tmp_val);
  inven_temp.data.tohit = tmp_val;

  prt("+To dam: ", 1, 1);
  get_string(tmp_str, 1, 10, 10);
  tmp_val = 0;
  sscanf(tmp_str, "%ld", &tmp_val);
  inven_temp.data.todam = tmp_val;

  prt("AC     : ", 1, 1);
  get_string(tmp_str, 1, 10, 10);
  tmp_val = 0;
  sscanf(tmp_str, "%ld", &tmp_val);
  inven_temp.data.ac = tmp_val;

  prt("+To AC : ", 1, 1);
  get_string(tmp_str, 1, 10, 10);
  tmp_val = 0;
  sscanf(tmp_str, "%ld", &tmp_val);
  inven_temp.data.toac = tmp_val;

  prt("Damage : ", 1, 1);
  get_string(tmp_str, 1, 10, 5);
  strcpy(inven_temp.data.damage, tmp_str);

  prt("Level  : ", 1, 1);         /* added code to specify item's */
  get_string(tmp_str, 1, 10, 10); /* level.  --jb 2/5/00 */
  tmp_val = 0;
  sscanf(tmp_str, "%ld", &tmp_val);
  if (tmp_val < 0)
    tmp_val = 0;
  inven_temp.data.level = tmp_val;

  if (get_yes_no("Allocate?")) {
    popt(&tmp_val);
    t_list[tmp_val] = inven_temp.data;
    /* with cave[char_row][char_col]. do; */
    if (cave[char_row][char_col].tptr > 0) {
      delete_object(char_row, char_col);
    }
    cave[char_row][char_col].tptr = tmp_val;
    msg_print("Allocated...");
  } else {
    msg_print("Aborted...");
  }

  inven_temp.data = blank_treasure;
  player_action_move(5);
  creatures(false);
}

void wizard_help() {
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

void wizard_command(void) {
  char tmp_str[82];
  enum stat_t tstat;
  treas_rec *trash_ptr;
  long y, x;

  prt("Wizard command: ", 1, 1);
  switch (inkey()) {
  case 'a':
    hp_player(1000, "cheating");
    player_cmana = player_mana;
    prt_stat_block();
    remove_curse();
    cure_me(&((player_flags).blind));
    cure_me(&((player_flags).hoarse));
    cure_me(&((player_flags).afraid));
    cure_me(&((player_flags).poisoned));
    cure_me(&((player_flags).confused));
    for (tstat = STR; tstat <= CHR; tstat++)
      restore_stat(tstat, "");
    if ((player_flags).slow > 1)
      (player_flags).slow = 1;
    if ((player_flags).image > 1)
      (player_flags).image = 1;
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
