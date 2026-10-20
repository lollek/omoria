/* traps.c */
/* all sorts of nasty traps (stores too)! */

#include <math.h>
#include <stdio.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "casino/casino.h"
#include "combat/fighting.h"
#include "constants.h"
#include "debug.h"
#include "generate_monster/generate_monster.h"
#include "io.h"
#include "loot/loot.h"
#include "magic.h"
#include "misc.h"
#include "pascal.h"
#include "player.h"
#include "quest.h"
#include "random.h"
#include "screen.h"
#include "spells.h"
#include "stores.h"
#include "town_level/enter_bank.h"
#include "town_level/enter_house.h"
#include "trade.h"
#include "traps.h"
#include "types.h"
#include "variables.h"

/* These functions are now provided by Rust (see src/dungeon/trap/interop.rs). */
void place_trap(long y, long x, long typ, long subval);
void change_trap(long y, long x);
void place_rubble(long y, long x);
