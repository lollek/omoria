#include <curses.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "../ability.h"
#include "../blow.h"
#include "../c.h"
#include "../commands.h"
#include "../constants.h"
#include "../death.h"
#include "../debug.h"
#include "../fighting/ranged.h"
#include "../help.h"
#include "../inventory/inven.h"
#include "../io.h"
#include "../magic.h"
#include "../menu.h"
#include "../misc.h"
#include "../player.h"
#include "../player_action.h"
#include "../save.h"
#include "../screen.h"
#include "../types.h"
#include "../variables.h"
#include "../wizard.h"

#include "command.h"

void C_print_known_spells();

void command(long *command) {
  treas_rec *trash_ptr;
  char out_val[82];
  char out2[82];

  ENTER(("command", "%d, '%c'", *command, *command));

  switch (*command) {

    /* case   1  :     ^A = Cure all     W1 */
    /* case   2  :     ^B = objects      W1 */
    /* case   4  :     ^D = up/down      W1 */
    /* case   5  :     ^E = wizchar      W2 */
    /* case   6  :     ^F = genocide     W2 */
    /* case   7  :     ^G = treasure     W2 */
    /* case   8  :     ^H = wizhelp      W1 */
    /* case   9  :     ^I = identify     W1 */
    /* case  10  :     ^J = gain exp     W2 */
    /* case  11  :     ^K = summon       W2 */
    /* case  12  :     ^L = wizlight     W1 */
    /* case  14  :     ^N = mon map      W1 */
    /* case  15  :     ^O = summon       W2 */
    /* case  20  :     ^T = teleport     W1 */
    /* case  22  :     ^V = restore      W1 */
    /* case  21  :     ^U = summon       W2 */
    /* case  23  :     ^W = create       W2 */
    /* case  24  :     ^X = ed score     W2 */
    /* case  27  :     ^3 = store maint  W2 */
    /* case  31  :     ^_                W1 */

  case 0:      /* \0 */
  case CTRL_C: // ^C signalquit in io.c handles this one, it calls d__quit
  case '@':
    death_by_quitting();
    reset_flag = true;
    break;

  case CTRL_A:
    reset_flag = C_select_ability();
    draw_cave();
    break;

  case CTRL_B:
    find_flag = true;
    player_action_move(1);
    break;

  case CTRL_H:
    find_flag = true;
    player_action_move(4);
    break;
  case CTRL_I:
    blow();
    break;
  case CTRL_J:
    find_flag = true;
    player_action_move(2);
    break;
  case CTRL_K:
    find_flag = true;
    player_action_move(8);
    break;
  case CTRL_L:
    find_flag = true;
    player_action_move(6);
    break;

  case CTRL_N:
    find_flag = true;
    player_action_move(3);
    break;

  case CTRL_P:
    msg_record("", false);
    reset_flag = true;
    break;

  case CTRL_R: /* redraw */
    draw_cave();
    reset_flag = true;
    break;
  case CTRL_S:
    player_action_jam_door();
    break;

  case CTRL_U:
    find_flag = true;
    player_action_move(9);
    break;

  case CTRL_W: /* Password */
    if (!wizard1)
      enter_wizard_mode(true);
    else
      wizard_command();
    reset_flag = true;
    break;

  case CTRL_X:
    player_action_look();
    reset_flag = true;
    break;
  case CTRL_Y:
    find_flag = true;
    player_action_move(7);
    break;
  case CTRL_Z: /* suspend  (we never get this, look at signalsuspend) */
    reset_flag = true;
    break;

  case 27: /* ALT */
    *command = inkey();
    MSG(("command: %d '%c'\n", (int)*command, (int)*command));
    switch (*command) {
    case 'a': /* Armor help */
      moria_help("Adventuring Armor_Class Armor_List");
      draw_cave();
      reset_flag = true;
      break;
    case 'b':
      player_action_move(1);
      break;
    case 'c':
      C_commands_show_class_restrictions();
      draw_cave();
      break;
    case 'd':
      sprintf(out_val, "The date is %s",
              full_date_string(player_cur_age, out2));
      msg_print(out_val);
      reset_flag = true;
      break;
    case 'e':
      sprintf(out_val, "Character Classes Experience %4.2f", player_expfact);
      moria_help(out_val);
      draw_cave();
      reset_flag = true;
      break;

    case 'h':
      player_action_move(4);
      break;
    case 'i':
      reset_flag = true;
      if (inven_command('i', &trash_ptr, "")) {
        draw_cave();
      }
      break;
    case 'j':
      player_action_move(2);
      break;
    case 'k':
      player_action_move(8);
      break;
    case 'l':
      player_action_move(6);
      break;
    case 'm':
      moria_help("");
      draw_cave();
      reset_flag = true;
      break;
    case 'n':
      player_action_move(3);
      break;

    case 'r':
      msg_terse = !msg_terse;
      if (msg_terse) {
        msg_print("Question '-More-' toggled off");
        msg_terse = true; /* try to only use true and false */
      } else {
        msg_print("Question '-More-' toggled on");
        msg_terse = false;
      }
      reset_flag = true;
      break;
    case 's': /* Save and quit */
      if (total_winner) {
        msg_print("You are a Total Winner, your "
                  "character must "
                  "be retired...");
        msg_print("Use '@' when you are ready to quit.");
      } else {
        if (search_flag)
          search_off();
        if (sav__save_character())
          exit_game();
      }
      reset_flag = true;
      break;
    case 't':
      sprintf(out_val, "The current time is %s", show_current_time(out2));
      msg_print(out_val);
      reset_flag = true;
      break;
    case 'u':
      player_action_move(9);
      break;

    case 'w':
      moria_help("Adventuring Weapons Weapon_List");
      draw_cave();
      reset_flag = true;
      break;

    case 'y':
      player_action_move(7);
      break;
    }
    break;

  case '/': /* identify */
    ident_char();
    reset_flag = true;
    break;

  case '<':
    player_action_ascend_stairs();
    break;
  case '>':
    player_action_descend_stairs();
    break;

  case '?': /* help */
    help();
    reset_flag = true;
    break;

  case '.': /* Rest one turn */
    player_action_move(5);
    usleep(10);
    flush();
    break;

  case 'A':
    throw_something();
    break;
  case 'B':
    find_flag = true;
    player_action_move(1);
    break;
  case 'C': /* Show character */
    change_name();
    draw_cave();
    reset_flag = true;
    break;
  case 'D':
    player_action_disarm_trap();
    break;
  case 'E':
    player_action_eat();
    break;
  case 'F':
    player_action_refill_lamp();
    break;

  case 'H':
    find_flag = true;
    player_action_move(4);
    break;
  case 'I': /* Selected inv */
    reset_flag = true;
    if (inven_command('I', &trash_ptr, "")) {
      draw_cave();
    }
    break;
  case 'J':
    find_flag = true;
    player_action_move(2);
    break;
  case 'K':
    find_flag = true;
    player_action_move(8);
    break;
  case 'L':
    find_flag = true;
    player_action_move(6);
    break;
  case 'N':
    find_flag = true;
    player_action_move(3);
    break;

  case 'P':
    C_print_known_spells();
    draw_cave();
    break;
  case 'Q':
    if (player_flags.quested) {
      sprintf(out_val, "Current quest is to kill a %s",
              monster_templates[player_cur_quest].name);
      msg_print(out_val);
    } else {
      msg_print("No quest currently.");
    }
    reset_flag = true;
    break;
  case 'R':
    player_action_rest();
    break;
  case 'S': /* Search mode */
    if (search_flag) {
      search_off();
      reset_flag = true;
    } else if (player_flags.blind > 0) {
      msg_print("You are incapable of searching while blind.");
    } else {
      search_on();
      reset_flag = true;
    }
    break;
  case 'T':
    player_action_tunnel();
    break;
  case 'U':
    find_flag = true;
    player_action_move(9);
    break;

  case 'X':
    player_action_toggle_light_source();
    break;
  case 'Y':
    find_flag = true;
    player_action_move(7);
    break;
  case 'Z':
    player_action_use_staff();
    break;

  case 'a':
    shoot_something();
    break;
  case 'b':
    player_action_move(1);
    break;
  case 'c':
    player_action_close();
    break;
  case 'd':
    player_action_drop();
    break;
  case 'f':
    player_action_bash();
    break;
  case 'h':
    player_action_move(4);
    break;
  case 'i': /* Inventory */
    reset_flag = true;
    display_inventory();
    draw_cave();
    break;
  case 'j':
    player_action_move(2);
    break;
  case 'k':
    player_action_move(8);
    break;
  case 'l':
    player_action_move(6);
    break;
  case 'm': /* magick, monk, music */
    if (C_player_uses_magic(M_NATURE)) {
      player_action_use_magic(M_NATURE); /* play */
    } else if (C_player_uses_magic(M_ARCANE)) {
      player_action_use_magic(M_ARCANE); /*  magick   } */
    } else if (C_player_uses_magic(M_CHAKRA)) {
      player_action_use_magic(M_CHAKRA); /* m = monk? :) */
    } else {
      player_action_use_magic(M_SONG); /* music */
    }
    break;
  case 'n':
    player_action_move(3);
    break;
  case 'o':
    player_action_open();
    break;
  case 'p': /* pray */
    if (C_player_uses_magic(M_DIVINE)) {
      player_action_use_magic(M_DIVINE);
    } else {
      msg_print("You pray for a moment");
    }
    break;
  case 'q':
    player_action_quaff_potion();
    break;
  case 'r':
    player_action_read_scroll();
    break;
  case 's': /* Search */
    player_action_search(char_row, char_col, C_player_curr_search_skill());
    break;
  case 't': /* take off */
    reset_flag = true;
    if (inven_command('t', &trash_ptr, "")) {
      draw_cave();
    }
    break;
  case 'u':
    player_action_move(9);
    break;
  case 'v': /* version */
    reset_flag = true;
    game_version();
    break;
  case 'w': /* wear */
    reset_flag = true;
    if (inven_command('w', &trash_ptr, "")) {
      draw_cave();
    } else {
      prt_stat_block();
    }
    break;
  case 'x': /* exchange weapon */
    reset_flag = true;
    if (inven_command('x', &trash_ptr, "")) {
      draw_cave();
    }
    break;
  case 'y':
    player_action_move(7);
    break;
  case 'z':
    player_action_aim_wand();
    break;
  default:
    reset_flag = true;
    prt("Type '?' for help...", 1, 1);
    break;
  }

  LEAVE("command", "d");
}
