#pragma once

// Where savegames are put
#define SAVE_FILE_PATH "save"

/*
 * Note: Be sure to copy monsters.dat to this directory!  The game
 * will attempt to create the other files the first time that
 * it is run.
 */
#define DATA_FILE_PATH "data"

// If this file exists, players will get kicked out and cannot join
#define KICKOUT_FILE DATA_FILE_PATH "/morialock.dat"

// Hours when you are allowed to play
#define OPERATING_HOURS_FILE DATA_FILE_PATH "/hours.dat"

// File for handling trades
#define TRADE_FILE DATA_FILE_PATH "/moriatrd.dat"

// File for logging deaths
#define DEATH_FILE DATA_FILE_PATH "/death.log"

// Files for monster templates
#define MONSTERS_FILE DATA_FILE_PATH "/monsters.dat"

/*
 * Keep this one under 160 characters, it points to the help program...
 * You may have to edit mhelp.pl to set the path to perl.
 */
#define HELP_FILE_PATH DATA_FILE_PATH "/mhelp.pl"

/*
 * The curses I have on my system defines attr_get() and attrset(attr)
 * differently than the one that a lot of people using Red Hat have.
 *
 * I don't know what to use to get and set the attributes with the
 * newer library so change this to 1 if you want to try the nifty
 * attr changes I added.  Otherwise you can leave it set to 0 and
 * always wonder what the game could be like...
 *
 * This is only used in put_buffer_attr (term.c).  If you get it working
 * with a curses that it does not currently work with please let me know!
 */
#define USE_CURSES_ATTRS 1
