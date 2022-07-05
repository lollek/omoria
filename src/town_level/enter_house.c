#include "../misc.h"
#include "../player.h"
#include "../random.h"
#include "../screen.h"
#include "../variables.h"
#include "../wizard.h"
#include "../stores.h"
#include "../player/hunger.h"

static void call_wizards() {
  msg_print("The mage calls for a Town Wizard to remove you.");
  monster_summon_by_name(char_row, char_col, "Town Wizard", true, false);
}

static void call_guards(char const *who) {
  char out_str[82];

  snprintf(out_str, sizeof(out_str), "The %s call(s) for the Town Guards!",
           who);
  msg_print(out_str);
  monster_summon_by_name(char_row, char_col, "Town Guard", true, false);
  monster_summon_by_name(char_row, char_col, "Town Guard", true, false);
}

static void kicked_out() { msg_print("The owner kicks you out..."); }

static void guild_or_not(boolean passed) {
  if (passed) {
    spend_time(600, "showing off your skills", false);
    msg_print("Good! You are invited to join the guild!");
    C_player_add_exp(5);
    change_rep(-3);
  } else {
    spend_time(400, "or lack thereof", false);
    msg_print("You fail to impress them.");
    if (randint(3) == 1) {
      msg_print("They think you are with the guard!");
      msg_print("You are stabbed by one of them before you escape");
      take_hit(randint(randint(16)), "Thieves Guild Member");
      prt_stat_block();
    }
  }
}

static void brothel_game() {
  if (get_yes_no("Do you accept?")) {
    change_rep(-3);
    /* with player_do; */
    if ((player_disarm + player_lev + 2 * C_player_disarm_from_dex() +
         C_player_mod_from_stat(INT)) > randint(100)) {
      msg_print("Good! You are invited to join the house!");
      C_player_add_exp(5);
      spend_time(600, "putting out for peasants", false);
    } else {
      msg_print("You fail to please your customers.");
      spend_time(400, "imitating a pine board", false);
    }
  }
}

static void battle_game(long plus, char kb_str[82]) {
  long score, i1, time;
  char out_val[82];

  if (get_yes_no("Do you accept their invitation?")) {
    msg_print("Good for you!");
    score = 0;
    time = 10;

    /* with player_do; */
    for (i1 = 1; i1 <= 7; i1++) {
      if (player_test_hit(player_bth, player_lev, plus, 20 * i1, false)) {
        score++;
        time = time * 2 + 10;
      }
    }

    sprintf(out_val, "with some %s", kb_str);
    spend_time(time, out_val, false);

    switch (score) {
    case 1:
      msg_print("They ridicule your clumsy performance...");
      msg_print("'Come back when you are more experienced!!'");
      change_rep(-2);
      break;

    case 2:
      msg_print("You do not do well...");
      break;

    case 3:
      msg_print("'Pretty good for a beginner!'");
      break;

    case 4:
      msg_print("They are quite impressed!");
      break;

    case 5:
      msg_print("They are amazed by your incredible prowess!");
      change_rep(2);
      break;

    case 6:
    case 7:
      msg_print("You handle them all with ease!");
      msg_print("'Thanks for the workout! Come back anytime!!'");
      C_player_add_exp(10);
      change_rep(5);
      break;

    default:
      msg_print("'Boy that was quick!! What a little wimp!'");
      msg_print("They pummel you senseless and toss you out "
                "into the street!");
      take_hit(damroll("2d4"), kb_str);
      change_rep(-5);
      player_flags.confused += 5 + randint(5);
      break;
    }
  }
}

static void thief_games() {
  if (randint(2) == 1) {
    msg_print("The thieves invite you to prove your ability to "
              "pick locks.");
    if (get_yes_no("Do you accept?")) {
      /* with player_do; */
      guild_or_not((player_disarm + player_lev +
                    2 * C_player_disarm_from_dex() +
                    C_player_mod_from_stat(INT)) > randint(100));
    }
  } else {
    msg_print("The thieves invite you to show your stealthiness.");
    if (get_yes_no("Do you accept?")) {
      guild_or_not(player_stl > randint(12));
    }
  }
}

/*  returns 0 to 10 -- SD 2.4; */
/*  x is average reaction for a 0 SC ugly half-troll*/
static long react(long x) {

  long ans = (C_player_get_stat(CHR) * 10 + (player_rep * 2) + randint(200) +
              randint(200) + randint(200)) /
                 50 +
             x - 4;

  if (ans < 0) {
    ans = 0;
  } else if (ans > 10) {
    ans = 10;
  }

  return ans;
}

static void eat_the_meal() {
  long yummers, old_food;

  old_food = player_flags.foodc;

  yummers = react(randint(8) - 2);
  if ((yummers == 10) && (randint(2) == 1)) {
    yummers = 15;
  }

  spend_time(50 + 50 * yummers, "eating like a pig", false);

  switch (yummers) {
  case 15:
    msg_print("It is a sumptuous banquet, and you feel quite stuffed.");
    player_hunger_set_status(BLOATED);
    prt_hunger();
    change_rep(3);
    break;

  case 6:
  case 7:
  case 8:
  case 9:
  case 10:
    msg_print("It is an ample meal, and you feel full.");
    player_hunger_set_status(FULL);
    prt_hunger();
    change_rep(1);
    break;

  default:
    if ((yummers > 0) ||
        player_saves(player_lev + 5 * C_player_mod_from_stat(CON))) {
      msg_print("It was a boring meal, and you eat very little.");
      player_flags.foodc = old_food;
      prt_hunger();
    } else {
      msg_print("Yuk!  That meal was AWFUL!");
      msg_print("You throw up!");
      if (player_flags.foodc > 150) {
        player_flags.foodc = 150;
      }
      msg_print("You get food poisoning.");
      player_flags.poisoned += randint(10) + 5;
      change_rep(-2);
    }
    break;
  }
}

static void invite_for_meal() {
  msg_print("The occupants invite you in for a meal.");
  if (get_yes_no("Do you accept?")) {
    eat_the_meal();
  }
}

static void party() {
  msg_print("The owner invites you to join the party!");
  if (get_yes_no("Do you accept?")) {
    spend_time(400 + randint(1600), "at a party", false);

    switch (randint(6)) {
    case 1:
      msg_print("Someone must have spiked the punch!");
      msg_print("Oh, your aching head!");
      player_flags.confused += 25 + randint(25);
      break;

    case 2:
      msg_print("Gee, those brownies were awfully unusual....");
      msg_print("You feel a little strange now.");
      player_flags.image += 200 + randint(100);
      break;

    case 3:
      msg_print("You smoked something strange at the party.");
      switch (randint(2)) {
      case 1:
        player_flags.hero += 25 + randint(25);
        break;
      case 2:
        player_flags.afraid += 25 + randint(25);
        break;
      }
      break;

    case 4:
    case 5:
    case 6:
      msg_print("It is an interesting party, and you enjoy "
                "yourself.");
      break;
    }
  }
}

static void spend_the_night(char who[82]) {
  char out_str[82];

  msg_print("The occupant(s) invite you to rest in his house.");
  if (get_yes_no("Do you accept?")) {
    sprintf(out_str, "at the home of the %s.", who);
    spend_time(1, out_str, true);
    change_rep(2);
  } else if (get_yes_no("Okay, how about staying for a meal?")) {
    eat_the_meal();
  }
}

static void worship() {
  long preachy, i1;

  msg_print("The priest invites you to participate in the service.");

  if (get_yes_no("Do you accept?")) {
    preachy = randint(4);

    switch (preachy) {
    case 1:
      msg_print("You sit through a fascinating church service.");
      break;
    case 2:
      msg_print("You sit through an interesting church service.");
      break;
    case 3:
      msg_print("You sit through a boring church service.");
      break;
    case 4:
      msg_print("You sit through a long, boring church service.");
      break;
    }

    spend_time(100 * (randint(7) + preachy * preachy), "at the Church", false);

    msg_print("The priest asks for donations for a new church.");
    if (get_yes_no("Will you give him some money?")) {
      /* with player_do; */
      if (player_money[TOTAL_] > 0) {
        msg_print("Bless you, dude!");

        i1 = ((randint(12) * player_money[TOTAL_]) / 1000 + 20) * GOLD_VALUE;
        if (i1 > player_money[TOTAL_] * GOLD_VALUE / 2) {
          i1 = player_money[TOTAL_] * GOLD_VALUE / 2;
        }

        subtract_money(i1, false);
        prt_stat_block();

        if (i1 > 20 * GOLD_VALUE) {
          change_rep(5);
        } else {
          change_rep((i1 + 5 * GOLD_VALUE - 1) / (5 * GOLD_VALUE));
        }

      } else {
        msg_print("He says 'It is the thought that "
                  "counts, my child.'");
        msg_print("Thank you for being willing to give.");
      }
    } else {
      msg_print("Syo problem, man?");
      change_rep(-5);
    }
  } else if (react(6) == 0) {
    msg_print("You heathen!  Get out of my temple!");
    change_rep(-5);
  }
}

/*
static void beg_food() {
       var      i2              : long;
                item_ptr        : treas_rec;*
       begin
        if (find_range([food],false,item_ptr,i2)) then
          begin
            msg_print("The occupants beg you for food.");
            if get_yes_no("Will you feed them?") then
              begin
                spend_time(200,"feeding people",false);
                msg_print("How kind of you!");
                inven_destroy(item_ptr);
                change_rep(5);
                prt_stat_block();
              end
            else
              begin
                msg_print("What a jerk!");
                change_rep(-10);
              end;
          end
        else
          beg_money();
       end;
}
*/

static void beg_money() {
  long i1;

  msg_print("The occupants beg you for money.");

  if (get_yes_no("Will you give them some?")) {
    /* with player_do; */
    if (player_money[TOTAL_] > 0) {
      msg_print("How kind of you!");
      spend_time(100, "giving handouts", false);
      i1 = ((randint(12) * player_money[TOTAL_]) / 1000 + 20) * GOLD_VALUE;
      if (i1 > player_money[TOTAL_] * GOLD_VALUE / 2) {
        i1 = player_money[TOTAL_] * GOLD_VALUE / 2;
      }
      subtract_money(i1, false);
      if (i1 > 20 * GOLD_VALUE) {
        change_rep(5);
      } else {
        change_rep((i1 + 5 * GOLD_VALUE - 1) / (5 * GOLD_VALUE));
      }
      prt_stat_block();
    } else {
      msg_print("They are disappointed because you have no money.");
    }
  } else {
    msg_print("What a jerk!");
    change_rep(-10); /*{bug fixed here; used to be 10 -- MAV }*/
  }
}


void enter_house(long y, long x) {
  switch (t_list[cave[y][x].tptr].p1) {
  case 1:
    msg_print("The building is empty.");
    if (react(10) == 0) {
      msg_print("The building is being guarded!");
      call_guards("Magic Mouth spell");
    }
    break;

  case 2:
    msg_print("There is a Thieves' Guild meeting here.");
    switch (react(6)) {
    case 0:
      call_guards("Guildmaster");
      break;

    case 1:
    case 2:
    case 3:
    case 4:
    case 5:
    case 6:
    case 7:
      kicked_out();
      break;

    case 8:
    case 9:
    case 10:
      thief_games();
      break;
    }
    break;

  case 3:
    msg_print("This is a town brothel.  Some young prostitutes are "
              "here.");
    switch (react(10)) {
    case 0:
      call_guards("prostitutes");
      break;

    case 1:
    case 2:
    case 3:
    case 4:
    case 5:
    case 6:
      kicked_out();
      break;

    default:
      if (characters_sex() != FEMALE) {
        msg_print("The girls invite you to prove your "
                  "abilities.");
        battle_game(C_player_mod_from_stat(CHR), "some playful prostitutes");
      } else {
        msg_print("The girls invite you to work with them.");
        brothel_game();
      }
    }
    break;

  case 4:
    msg_print("Some drunken fighters are telling tales here.");
    switch (react(8)) {
    case 0:
      call_guards("group of fighters");
      break;

    case 1:
    case 2:
    case 3:
    case 4:
    case 5:
    case 6:
      kicked_out();
      break;

    default:
      msg_print("They ask you to demonstrate your fighting skill.");
      battle_game(player_ptohit, "some drunken fighters");
      break;
    }
    break;

  case 5:
    msg_print("There is a party in progress here.");
    switch (react(8)) {
    case 0:
      call_guards("party's host");
      break;

    case 1:
    case 2:
    case 3:
    case 4:
    case 5:
      kicked_out();
      break;

    default:
      party();
    }
    break;

  case 6:
    switch (randint(2)) {
    case 1:
      msg_print("The building is a poorhouse.");
      break;
    case 2:
      msg_print("This is an orphanage.");
      break;
    }

    switch (react(12)) {
    case 0:
      call_guards("beggars");
      break;

    case 1:
    case 2:
    case 3:
    case 4:
      kicked_out();
      break;

    default:
      switch (randint(2)) {
      case 1: /*beg_food();  break;*/ /* beg_food is
                                         unfinished XXXX */
      case 2:
        beg_money();
        break;
      }
    }
    break;

  case 7:
  case 8:
    switch (randint(3)) {
    case 1:
      msg_print("This is the home of a peasant family.");
      break;
    case 2:
      msg_print("These are the quarters of a humble laborer.");
      break;
    case 3:
      msg_print(" This is the home of several poor families.");
      break;
    }

    switch (react(8)) {
    case 0:
      call_guards("peasant(s)");
      break;

    case 1:
    case 2:
    case 3:
      kicked_out();
      break;

    case 4:
    case 5:
    case 6:
    case 7:
      invite_for_meal();
      break;

    case 8:
    case 9:
    case 10:
      spend_the_night("peasant(s)");
      break;
    }
    break;

  case 9:
    switch (randint(3)) {
    case 1:
    case 2:
      msg_print("This is the home of a merchant.");
      break;
    case 3:
      msg_print("This is the house of an accomplished craftsman.");
      break;
    }

    switch (react(5)) {
    case 0:
      call_guards("owner");
      break;

    case 1:
    case 2:
    case 3:
    case 4:
      kicked_out();
      break;

    case 5:
    case 6:
    case 7:
    case 8:
    case 9:
      invite_for_meal();
      break;

    case 10:
      spend_the_night("gentleman");
      break;
    }
    break;

  case 10:
    msg_print("There is a religious service in progress here.");
    switch (react(8)) {
    case 0:
      call_guards("High Priest");
      break;

    case 1:
    case 2:
    case 3:
    case 4:
    case 5:
      kicked_out();
      break;

    default:
      worship();
      break;
    }
    break;

  case 11:
    switch (randint(3)) {
    case 1:
      msg_print("This is the house of a wealthy shopkeeper.");
      break;
    case 2:
      msg_print("This is the mansion of a affluent noble.");
      break;
    case 3:
      msg_print("This is the estate of an rich guildsman.");
      break;
    }

    switch (react(2)) {
    case 0:
      call_guards("master of the house");
      break;

    case 1:
    case 2:
    case 3:
      kicked_out();
      break;

    case 4:
    case 5:
    case 6:
    case 7:
    case 8:
    case 9:
      invite_for_meal();
      break;

    case 10:
      spend_the_night("master of the house");
      break;
    }
    break;

  case 12:
    msg_print("This is the home of a powerful mage.");
    switch (react(5)) {
    case 0:
      call_wizards();
      break;

    case 1:
    case 2:
    case 3:
      call_guards("mage");
      break;

    case 4:
    case 5:
    case 6:
    case 7:
    case 8:
    case 9:
      kicked_out();
      break;

    case 10:
      invite_for_meal();
      break;
    }
  }

  t_list[cave[y][x].tptr].p1 = 1;

  prt_stat_block();
}