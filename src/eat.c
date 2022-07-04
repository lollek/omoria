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
#include "term.h"
#include "types.h"
#include "variables.h"
#include "desc.h"
#include "inven.h"
#include "screen.h"
#include "spells.h"
#include "misc.h"
#include "random.h"
#include "player/hunger.h"

static boolean __eat_eyeball_of_drong(void) {

  long damage = damroll("10d8") + 100;
  take_hit(damage, "the Wrath of Ned");

  cure_me(&player_flags.afraid);
  bless(randint(100) + 100);
  player_flags.hero += randint(100) + 100;
  player_flags.shero += randint(50) + 75;
  player_flags.invuln += randint(15) + 10;

  player_flags.image += randint(500) + randint(500) + randint(1000) + 5000;
  player_flags.poisoned += randint(100) + 150;
  player_flags.confused += randint(30) + 50;
  player_flags.blind += randint(3) + 10;
  player_flags.paralysis = 4;

  msg_print("You hear a distant rumble of laughter...");
  msg_print("You throw back your head and laugh back!");
  aggravate_monster(30);

  if (randint(5) == 1) {
    gain_stat(STR, "Ned smiles upon you.");
    gain_stat(INT, "");
    gain_stat(WIS, "");
    gain_stat(DEX, "");
    gain_stat(CON, "");
    gain_stat(CHR, "");
  }

  msg_print("Your head begins to spin. Spots swirl before you.");
  msg_print("A veil of darkness clings to your eyes.");
  msg_print("You are unable to move!");

  return true;
}

static treas_rec *__select_what_to_eat() {

  if (inven_ctr <= 0) {
    msg_print("But you are not carrying anything.");
    return NULL;
  }

  obj_set const things_to_eat = {Food, junk_food, 0};
  treas_rec *first;
  long count;
  if (!find_range(things_to_eat, false, &first, &count)) {
    msg_print("You are not carrying any food.");
    return NULL;
  }

  boolean redraw = false;
  treas_rec *item_ptr = NULL;
  char trash_char;
  boolean select_ok = get_item(&item_ptr, "Eat what?", &redraw, count,
                               &trash_char, false, false);

  if (redraw)
    draw_cave();

  if (select_ok)
    return item_ptr;
  else
    return NULL;
}

static boolean __apply_food_effects(treas_rec *item_ptr) {
  if (item_ptr->data.tval == junk_food && item_ptr->data.subval == 270) {
    return __eat_eyeball_of_drong();
  }

  long dam_pts = 0;
  boolean ident = false;
  for (unsigned long i = item_ptr->data.flags; i > 0;) {
    switch (bit_pos(&i) + 1) {
    case 1:
      player_flags.poisoned += randint(10) + item_ptr->data.level;
      ident = true;
      break;

    case 2:
      player_flags.blind += randint(250) + 10 * item_ptr->data.level + 100;
      draw_cave();
      msg_print("A veil of darkness surrounds you.");
      ident = true;
      break;

    case 3:
      player_flags.afraid += randint(10) + item_ptr->data.level;
      msg_print("You feel terrified!");
      ident = true;
      break;

    case 4:
      player_flags.confused += randint(10) + item_ptr->data.level;
      msg_print("You feel drugged.");
      break;

    case 5:
      player_flags.image += randint(200) + 25 * item_ptr->data.level + 200;
      break;

    case 6:
      ident = cure_me(&player_flags.poisoned);
      break;

    case 7:
      ident = cure_me(&player_flags.blind);
      break;

    case 8:
      if (player_flags.afraid > 1) {
        player_flags.afraid = 1;
        ident = true;
      }
      break;

    case 9:
      ident = cure_me(&player_flags.confused);
      break;

    case 10:
      ident = lose_stat(STR, "X", "X");
      break;

    case 11:
      ident = lose_stat(CON, "X", "X");
      break;

    case 12:
      ident = lose_stat(INT, "X", "X");
      break;

    case 13:
      ident = lose_stat(WIS, "X", "X");
      break;

    case 14:
      ident = lose_stat(DEX, "X", "X");
      break;

    case 15:
      ident = lose_stat(CHR, "X", "X");
      break;

    case 16:
      ident = restore_stat(STR, "You feel your strength returning.");
      break;

    case 17:
      ident = restore_stat(CON, "X");
      break;

    case 18:
      ident = restore_stat(INT, "Your head spins for a moment.");
      break;

    case 19:
      ident = restore_stat(WIS, "X");
      break;

    case 20:
      ident = restore_stat(DEX, "You feel more agile.");
      break;

    case 21:
      ident = restore_stat(CHR, "X");
      break;

    case 22:
      dam_pts += randint(3);
      break;

    case 23:
      dam_pts += randint(6);
      break;

    case 24:
      dam_pts += randint(12);
      break;

    case 25:
      dam_pts += damroll("2d12");
      break;

    case 26:
      dam_pts += damroll("4d12");
      break;

    case 27:
      dam_pts -= randint(4);
      break;

    case 28:
      dam_pts -= randint(8);
      break;

    case 29:
      ident = cure_me(&player_flags.hoarse);
      break;

      /* fill player to full, then adds food value */
    case 30:
      player_hunger_set_status(FULL);
      msg_print("Yum!");
      break;

    case 31:
      switch (bit_pos(&item_ptr->data.flags2) + 1) {
      case 1: /* crunchy */
        switch (randint(4)) {
        case 1:
          msg_print("*munch* *munch*");
          break;
        case 2:
          msg_print("*crunch* *crunch*");
          break;
        case 3:
          msg_print("*munch* *crunch*");
          break;
        case 4:
          msg_print("*crunch* *munch*");
          break;
        }
        break;

      case 2: /* liquid */
        switch (randint(3)) {
        case 1:
          msg_print("*guzzle* *guzzle*");
          break;
        case 2:
          msg_print("*glug* *glug* *glug*");
          break;
        case 3:
          msg_print("*slurp*");
          break;
        }

        if (randint(3) == 1) {
          msg_print("*Burp*");
          msg_print("('scuse me....)");
        } /* Belch */
        break;

      default:
        break;
      }

      break;

    default:
      break;
    }

    if (dam_pts != 0) {
      ident = hp_player(dam_pts, "poisonous food.");
    }
  }
  return ident;
}

void eat() {
  /* { Eat some food...					-RAK-	}*/

  reset_flag = true;
  treas_rec *item_ptr = __select_what_to_eat();
  if (item_ptr == NULL)
    return;

  reset_flag = false;
  boolean const identify_effect = __apply_food_effects(item_ptr);

  if (identify_effect) {
    identify(&(item_ptr->data));
  }

  if (item_ptr->data.flags != 0 && item_ptr->data.level > 0) {
    C_player_add_exp(((float)item_ptr->data.level / (float)player_lev) + .5);
    prt_stat_block();
  }

  player_hunger_eat(item_ptr->data.p1);
  desc_remain(item_ptr);
  inven_destroy(item_ptr);
  prt_stat_block();
}
