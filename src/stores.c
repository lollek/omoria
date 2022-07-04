/* store.c */
/**/

#include <curses.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "configure.h"
#include "kickout.h"
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
#include "generate_item/generate_item.h"
#include "desc.h"
#include "inven.h"
#include "screen.h"
#include "misc.h"
#include "random.h"

typedef struct owner_type {
  char owner_name[82];
  int64_t max_cost;
  float max_inflate;
  float min_inflate;
  float haggle_per;
  uint8_t owner_race;
  uint8_t insult_max;
} owner_type;

enum trade_status_t {
  TS_SUCCESS = 0,             // Successful trade
  TS_ABORTED = 1,             // We changed our mind
  TS_REFUSED_INSULTED = 2,    // Other party feels insulted
  TS_REFUSED_TRASH = 3,       // Other party will not buy your old junk
  TS_REFUSED_BLACKMARKET = 4, // Other party will not buy black market items
};

typedef struct trade_result {
  enum trade_status_t status;
  long price;
} trade_result_t;

// Turns in a day
#define DAY_LENGTH 9600

// Number of defined races
#define MAX_RACES 10

// Buying and selling adjustments for character race VS store owner race
static float rgold_adj[MAX_RACES][MAX_RACES] = {
    /*               Hum,  HfE,  Elf,  Hal,  Gno,  Dwa,  HfO,  HfT,  Phr, Dry */
    /*Human     */ {0.00, 0.05, 0.05, 0.10, 0.13, 0.15, 0.20, 0.25, 0.20, 0.05},
    /*Half-Elf  */ {0.10, 0.00, 0.00, 0.05, 0.10, 0.20, 0.25, 0.30, 0.25, 0.05},
    /*Elf       */ {0.10, 0.05, 0.00, 0.05, 0.10, 0.20, 0.25, 0.30, 0.30, 0.00},
    /*Halfling */ {0.15, 0.10, 0.05, -0.05, 0.05, 0.10, 0.15, 0.30, 0.25, 0.05},
    /*Gnome    */ {0.15, 0.15, 0.10, 0.05, -0.05, 0.10, 0.15, 0.30, 0.20, 0.15},
    /*Dwarf    */ {0.15, 0.20, 0.20, 0.10, 0.10, -0.05, 0.25, 0.35, 0.15, 0.30},
    /*Half-Orc  */ {0.15, 0.20, 0.25, 0.15, 0.15, 0.30, 0.10, 0.15, 0.15, 0.25},
    /*Half-Troll*/ {0.10, 0.15, 0.15, 0.10, 0.10, 0.30, 0.10, 0.10, 0.15, 0.25},
    /*Phraint  */ {0.20, 0.25, 0.30, 0.25, 0.20, 0.15, 0.15, 0.15, -0.10, 0.20},
    /*Dryad   */ {0.10, 0.05, 0.05, 0.05, 0.15, 0.30, 0.30, 0.25, 0.20, -0.05}};

// Store owners have different characteristics for pricing and haggling
// Note: Store owners should be added in groups, one for each store
static owner_type owners[MAX_OWNERS] = {
    /* {set number one} */
    {"Erick the Honest       (Human)      General Store", 2500, 0.75, 0.08,
     0.04, 0, 12},
    {"Mauglin the Grumpy     (Dwarf)      Armory", 32000, 1.00, 0.12, 0.04, 5,
     5},
    {"Arndal Beast-Slayer    (Half-Elf)   Weaponsmith", 10000, 0.85, 0.10, 0.05,
     1, 8},
    {"Hardblow the Humble    (Human)      Temple", 3500, 0.75, 0.09, 0.06, 0,
     15},
    {"Ga-nat the Greedy      (Gnome)      Alchemist", 12000, 1.20, 0.15, 0.04,
     4, 9},
    {"Valeria Starshine      (Elf)        Magic Shop", 32000, 0.75, 0.10, 0.05,
     2, 11},
    {"Tika Majere            (Human)      Inn", 32000, 1.00, 0.08, 0.05, 0, 7},
    {"Socrates the Philosopher  (Human)  Library", 5000, 1.00, 0.10, 0.05, 0,
     10},
    {"Dysella of Oakglade    (Dryad)      Music Shop", 10000, 1.00, 0.10, 0.05,
     9, 10},
    {"The Dragon Master      (Human)       Gem Store", 15000, 0.95, 0.10, 0.05,
     0, 5},
    {"Grond the Grotesque   (Half-Orc)     All-Nite Deli", 3000, 1.00, 0.10,
     0.05, 6, 5},
    {"Ugluk the Ugly        (Orc)          Black Market", 500000, 2.50, 1.5,
     0.01, 6, 6},

    /*{set number two} */
    {"Andy the Friendly      (Halfling)   General Store", 2000, 0.70, 0.08,
     0.05, 3, 15},
    {"Darg-Low the Grim      (Human)      Armory", 10000, 0.90, 0.11, 0.04, 0,
     9},
    {"Oglign Dragon-Slayer   (Dwarf)      Weaponsmith", 32000, 0.95, 0.12, 0.04,
     5, 8},
    {"Gunnar the Paladin     (Human)      Temple", 5000, 0.85, 0.10, 0.05, 0,
     23},
    {"Mauser the Chemist     (Half-Elf)   Alchemist", 10000, 0.90, 0.11, 0.05,
     1, 8},
    {"K'rek Kwith the Quick  (Phraint)    Magic Shop", 32000, 0.90, 0.10, 0.05,
     8, 5},
    {"Samwise                (Halfling)   Inn", 32000, 0.70, 0.10, 0.05, 3, 12},
    {"Elrond the Wise        (Elf)        Library", 5000, 1.00, 0.10, 0.05, 2,
     10},
    {"Shaun the Bard         (Human)    Music Shop", 10000, 1.00, 0.10, 0.05, 0,
     10},
    {"Ari-San                (Elf)       Gem Store", 15000, 1.00, 0.10, 0.05, 2,
     10},
    {"Gerald Ciceau		 (Human)     All-Nite Deli", 3000, 1.00, 0.07,
     0.05, 0, 5},
    {"Gloin the Fierce       (Dwarf)     Black Market", 500000, 2.5, 1.5, 0.01,
     5, 4},

    /*{set number three} */
    {"Lyar-el the Comely     (Elf)        General Store", 3000, 0.65, 0.07,
     0.06, 2, 18},
    {"Mauglim the Horrible   (Half-Orc)   Armory", 3000, 1.00, 0.13, 0.05, 6,
     9},
    {"Ithyl-Mak the Beastly	 (Half-Troll) Weaponsmith", 3000, 1.10, 0.15,
     0.06, 7, 8},
    {"Delihla the Pure       (Half-Elf)   Temple", 25000, 0.80, 0.07, 0.06, 1,
     20},
    {"Wizzle the Chaotic     (Halfling)   Alchemist", 10000, 0.90, 0.10, 0.06,
     3, 8},
    {"Inglorian the Mage     (Human?)     Magic Shop", 32000, 1.00, 0.10, 0.07,
     0, 10},
    {"Lucas the Portly       (Human)      Inn", 32000, 0.75, 0.10, 0.03, 0, 3},
    {"Dyxel the Beautiful    (Dryad)      Library", 5000, 1.00, 0.10, 0.05, 9,
     15},
    {"Roland the Melodic     (Halfling)   Music Shop", 10000, 1.00, 0.10, 0.05,
     3, 10},
    {"Galton the turrible    (Half-Orc)   Gem Store", 15000, 0.95, 0.20, 0.05,
     6, 4},
    {"Joseph Tansli		 (Human)      All-Nite Deli", 3000, 1.00, 0.10,
     0.05, 0, 10},
    {"Grima Wormtongue       (Human?)     Black Market", 500000, 2.5, 1.5, 0.01,
     0, 5},

    /*{set number four} */
    {"Felimid mac Fal        (Half-Elf)   General Store", 3500, 1.10, 0.15,
     0.10, 1, 5},
    {"Andre the Dull         (Half-Troll) Armory", 10000, 1.00, 0.08, 0.04, 6,
     8},
    {"Vlad Taltos            (Human)      Weaponsmith", 25000, 0.80, 0.10, 0.03,
     0, 15},
    {"Brother Maynard        (Human)      Temple", 15000, 1.00, 0.15, 0.08, 0,
     5},
    {"Questor Thews	         (Gnome)      Alchemist", 20000, 0.70, 0.10,
     0.02, 4, 10},
    {"Gopher the Great!      (Gnome)      Magic Shop", 20000, 1.15, 0.13, 0.06,
     4, 10},
    {"Mike the *Very* large  (Phraint)    Inn", 32000, 0.90, 0.10, 0.05, 8, 1},
    {"Kelstor the Sage       (Human)      Library", 5000, 1.00, 0.10, 0.05, 0,
     10},
    {"K'phelt the Drone     (Phraint)    Music Shop", 10000, 1.00, 0.10, 0.05,
     8, 10},
    {"Daphnea the Tender     (Dryad)      Gem Store", 15000, 0.80, 0.10, 0.05,
     9, 3},
    {"Clarion the Exotic 	 (Dryad)      All-Nite Deli", 3000, 1.00, 0.10,
     0.05, 9, 5},
    {"Netta Winsome          (Dryad)      Black Market", 500000, 2.5, 1.4, 0.02,
     9, 9}};

/*	{ Each store will buy only certain items, based on TVAL } */
static obj_set store_buy[MAX_STORES] = {
    /* {General Store	} */
    {1, 2, 3, 13, 15, 25, 30, 32, 38, 77, 80, 85, 86, 0, 0, 0},
    /* {Armory		} */
    {29, 30, 31, 33, 34, 35, 36, 37, 0, 0, 0, 0, 0, 0, 0, 0},
    /* {Weaponsmith	} */
    {10, 11, 12, 20, 21, 22, 23, 24, 26, 0, 0, 0, 0, 0, 0, 0},
    /* {Temple		} */
    {21, 70, 71, 75, 76, 91, 94, 0, 0, 0, 0, 0, 0, 0, 0, 0},
    /* {Alchemy shop	} */
    {70, 71, 75, 76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
    /* {Magic Shop	} */
    {1, 3, 6, 40, 45, 55, 60, 65, 90, 0, 0, 0, 0, 0, 0, 0},
    /* {Inn	 	} */
    {106, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
    /* {Library	} */
    {71, 70, 90, 91, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
    /* {Music Shop	} */
    {85, 86, 92, 93, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
    /* {Jewelry Shop	} */
    {4, 5, 7, 29, 37, 38, 40, 45, 0, 0, 0, 0, 0, 0, 0, 0},
    /* {All-Night Deli } */
    {81, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
    /* {Black Market } */
    {0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}};

/*	{ Hours that a store is open, in two hour increments. */
/*		  = Open */
/*		N = 'Closed for the night' */
/*		W = 'Closed for the weekend' */
/*		D = 'Closed for the day' */
/*		B = Bribeable */
/*					} */
static char store_hours[MAX_STORES + MAX_UNNAMED][7][14] =
    /* Mon, Tue, Wed, Thu, Fri, Sat, Sun */
    {
        /*{General Store	} */
        {"NNB       BN", "NNB       BN", "NNB       BN", "NNB       BN",
         "NNB       BN", "NNB       BN", "NNB       BN"},
        /*{Armory		} */
        {"NNNB     NNN", "NNNB     NNN", "WWWB     NNN", "NNNB     NNN",
         "NNNB     NNN", "NNNB     WWW", "WWWWWWWWWWWW"},
        /*{Weapon Smiths	} */
        {"NNNB     NNN", "NNNB     NNN", "WWWB     NNN", "NNNB     NNN",
         "NNNB     NNN", "NNNB     WWW", "WWWWWWWWWWWW"},
        /*{Temple		} */
        {"            ", "            ", "            ", "            ",
         "            ", "            ", "            "},
        /*{Alchemy Shop	} */
        {"NNNN    NNNN", "NNNN    NNNN", "NNNN    NNNN", "NNNN    NNNN",
         "NNNN    NNNN", "NNNN    NNNN", "NNNN    NNNN"},
        /*{Magic Shop	} */
        {"NNNN    NNNN", "NNNN    NNNN", "NNNN    NNNN", "NNNN    NNNN",
         "NNNN    NNNN", "NNNN    NNNN", "NNNN    NNNN"},
        /*{Inn		} */
        {"            ", "            ", "            ", "            ",
         "            ", "            ", "            "},
        /*{Library	} */
        {"NNN        N", "NNN        N", "NNN        N", "NNN        N",
         "NNN        N", "NNN        N", "NNN        N"},
        /*{Music Shop 	} */
        {"NNNB    BNNN", "NNNB   BNNNN", "NNNB    BNNN", "NNNB    BNNN",
         "NNNB    BNNN", "NNNB    BNNN", "NNNB    BNNN"},
        /*{Gem Store	} */
        {"NNNB     BNN", "NNNB   BNNNN", "NNNB     BNN", "NNNB     BNN",
         "NNNB     BNN", "NNNB     BNN", "NNNB     BNN"},
        /*{All-Night Deli } */
        {"            ", "            ", "            ", "            ",
         "            ", "            ", "            "},
        /*{Black Market } */
        {"BBBBBBBBBBBB", "BBBBBBBBBBBB", "BBBBBBBBBBBB", "BBBBBBBBBBBB",
         "BBBBBBBBBBBB", "BBBBBBBBBBBB", "BBBBBBBBBBBB"},
        /*{Trading Post	} */
        {"NNNB      BN", "NNNB      BN", "WWWB      BN", "NNNB      BN",
         "NNNB      BN", "NNNB      BW", "WWWWWWWWWWWW"},
        /*{Insurance Shop	} */
        {"BBBB     BBB", "BBBB     BBB", "BBBB     BBB", "BBBB     BBB",
         "BBBB     BBB", "BBBB     BBB", "BBBB     BBB"},
        /*{Bank		} */
        {"NNNN     NNN", "NNNN     NNN", "WWWW     NNN", "NNNN     NNN",
         "NNNN     NNN", "NNNN    WWWW", "WWWWWWWWWWWW"},
        /*{Money Exchange	} */
        {"NNNB     BNN", "NNNB     BNN", "WWWB     BNN", "NNNB     BNN",
         "NNNB     BNN", "NNNB     BWW", "WWWWWWWWWWWW"},
        /*{Casino		} */
        {"            ", "            ", "   DDDDDD   ", "   DDDDDD   ",
         "   DDDDDD   ", "   DDDDDD   ", "   DDDDDD   "},
        /*{Quest Store    } */
        {"            ", "            ", "            ", "            ",
         "            ", "            ", "            "}};

/*	{ Store owners can be bribed to open up their shop during */
/*	  certain hours (so that you can always have the opportunity to */
/*	  buy insurance, and suchlike.)					} */
static long store_bribe[MAX_STORES + MAX_UNNAMED] = {
    50,  /*  0 general */
    150, /*  1 armory */
    150, /*  2 weapons */
    75,  /*  3 temple */
    75,  /*  4 alchemy */
    300, /*  5 magic */
    25,  /*  6 inn */
    100, /*  7 library */
    50,  /*  8 music */
    75,  /*  9 gem */
    0,   /* 10 deli */
    100, /* 11 black market */
    50,  /* 12 trade post */
    100, /* 13 insurance */
    100, /* 14 bank */
    25,  /* 15 money changer */
    100, /* 16 casino */
    0    /* 17 fortress */
};

static void __randomize_cash(long *amt, long target) {
  *amt = (199 * (*amt) + randint(2 * target)) / 200;
}

/**
 * @__store_has_space_for() - Check to see if store has space for item
 * @store_num: Type of store
 * @item: Item to add
 */
static boolean __store_has_space_for(enum store_t store_num,
                                     treas_rec const *const item) {

  ENTER(("__store_has_space_for", ""));

  if (stores[store_num].store_ctr < STORE_INVEN_MAX) {
    LEAVE("__store_has_space_for", "");
    return true;
  }

  if (item->data.subval > 255 && item->data.subval < 512) {
    for (long i = 0; i < stores[store_num].store_ctr; i++) {
      if (stores[store_num].store_inven[i].sitem.tval != item->data.tval)
        continue;

      if (stores[store_num].store_inven[i].sitem.subval != item->data.subval)
        continue;

      LEAVE("__store_has_space_for", "");
      return true;
    }
  }
  LEAVE("__store_has_space_for", "");
  return false;
}

/**
 * __add_item_to_store() - Creates an item and inserts it into store's inven
 * @store_num: Type of store
 */
static void __add_item_to_store(enum store_t store_num) {

  long cur_pos;
  popt(&cur_pos);

  for (int tries = 0; tries < 4; ++tries) {
    switch (store_num) {
      case S_GENERAL:
        t_list[cur_pos] = generate_item_for_general_store();
        break;
      case S_ARMORY:
        t_list[cur_pos] = generate_item_for_armorsmith();
        break;
      case S_WEAPONS:
        t_list[cur_pos] = generate_item_for_weaponsmith();
        break;
      case S_TEMPLE:
        t_list[cur_pos] = generate_item_for_temple();
        break;
      case S_ALCHEMY:
        t_list[cur_pos] = generate_item_for_alchemist_store();
        break;
      case S_MAGIC:
        t_list[cur_pos] = generate_item_for_magic_store();
        break;
      case S_INN:
        t_list[cur_pos] = generate_item_for_inn();
        break;
      case S_LIBRARY:
        t_list[cur_pos] = generate_item_for_library();
        break;
      case S_MUSIC:
        t_list[cur_pos] = generate_item_for_music_store();
        break;
      case S_GEM:
        t_list[cur_pos] = generate_item_for_gem_store();
        break;
      case S_DELI:
        t_list[cur_pos] = generate_item_for_all_night_deli();
        break;
      case S_BLACK_MARKET:
        t_list[cur_pos] = generate_item_for_black_market();
        break;
      default: return; // Should panic
    }

    unquote(t_list[cur_pos].name);
    known1(t_list[cur_pos].name);
    known2(t_list[cur_pos].name);

    // TODO: Stop using inven_temp
    inven_temp.data = t_list[cur_pos];
    if (__store_has_space_for(store_num, &inven_temp)) {
      if (t_list[cur_pos].cost > 0) { /*{ Item must be good	}*/
        if (t_list[cur_pos].cost <
            (owners[stores[store_num].owner].max_cost * GOLD_VALUE)) {
          long dummy;
          store_carry(store_num, &dummy);
          break;
        }
      }
    }
  }
  pusht(cur_pos);
}

/**
 * __remove_item_from_store() - Destroy an item in the stores inventory.
 * @store_num: Store type
 * @item_val: ?
 * @one_of: If false, the entire slot is destroyed
 */
static void __remove_item_from_store(enum store_t store_num, long item_val,
                                     boolean one_of) {

  inven_temp.data = stores[store_num].store_inven[item_val].sitem;
  if ((stores[store_num].store_inven[item_val].sitem.number > 1) &&
      (stores[store_num].store_inven[item_val].sitem.subval < 512) &&
      (one_of)) {
    inven_temp.data.number = 1;
    stores[store_num].store_inven[item_val].sitem.number--;
  } else {
    long store_ctr = stores[store_num].store_ctr;
    for (int i = item_val; i < store_ctr; i++) {
      stores[store_num].store_inven[i] = stores[store_num].store_inven[i + 1];
    }
    stores[store_num].store_inven[store_ctr].sitem = blank_treasure;
    stores[store_num].store_inven[store_ctr].scost = 0;
    stores[store_num].store_ctr--;
  }
}

/**
 * __insert_item_in_store() - Insert INVEN_MAX at given location
 * @store_num: Type of store
 * @pos:
 * @icost:
 */
static void __insert_item_in_store(enum store_t store_num, long pos,
                                   long icost) {

  for (long i = stores[store_num].store_ctr; i >= pos; i--) {
    stores[store_num].store_inven[i + 1] = stores[store_num].store_inven[i];
  }

  // TODO: Shouldn't use inven_temp
  stores[store_num].store_inven[pos].sitem = inven_temp.data;
  stores[store_num].store_inven[pos].scost = -icost * GOLD_VALUE;
  stores[store_num].store_ctr++;
}

/**
 * __calc_purchase_price() - Base price of item depending on like/dislike
 * @store_type: Type of store
 * @item: Item to appraise
 */
static long __calc_purchase_price(enum store_t store_type,
                                  treasure_type const *const item) {
  if (item->cost <= 0)
    return 1;

  long value = item_value(item);
  value += trunc(
      value *
      rgold_adj[owners[stores[store_type].owner].owner_race][player_prace]);
  if (value < 1) {
    value = 1;
  }
  return value;
}

/**
 * __calc_sell_price() - Base price of item depending on like/dislike
 * @store_type: Type of store
 * @item: Item to appraise
 */
static long __calc_sell_price(enum store_t store_type,
                              treasure_type const *const item) {
  if (item->cost <= 0)
    return 0;

  long value = item_value(item);
  value -= trunc(
      value *
      rgold_adj[owners[stores[store_type].owner].owner_race][player_prace]);
  if (value < 1) {
    value = 1;
  }
  return value;
}

/**
 *__store_max_inflated_price() - Max inflated price a store will ask for
 * @store_type: Store type
 * @base_price: Base price of item to inflate
 */
static long __store_max_inflated_price(enum store_t store_type,
                                       long base_price) {
  long max_price =
      trunc(base_price * (1 + owners[stores[store_type].owner].max_inflate));
  max_price += (max_price * C_player_cost_modifier_from_charisma());
  if (max_price < 0) {
    max_price = 1;
  }
  return max_price;
}

/**
 *__store_min_inflated_price() - Minimum a store will allow to sell for
 * @store_type: Store type
 * @base_price: Base price of item to inflate
 */
static long __store_min_inflated_price(enum store_t store_type,
                                       long base_price) {
  long min_price =
      trunc(base_price * (1 + owners[stores[store_type].owner].min_inflate));
  min_price += (min_price * C_player_cost_modifier_from_charisma());
  if (min_price < 0) {
    min_price = 1;
  }
  return min_price;
}

/**
 *__store_max_deflated_price() - Max price a store will purchase for
 * @store_type: Store type
 * @base_price: Base price of item to inflate
 */
static long __store_max_deflated_price(enum store_t store_type,
                                       long base_price) {
  long max_price =
      trunc(base_price * (1 - owners[stores[store_type].owner].max_inflate));
  max_price -= (max_price * C_player_cost_modifier_from_charisma());
  if (max_price < 0) {
    max_price = 1;
  }
  return max_price;
}

/**
 *__store_min_deflated_price() - Minimum a store try to purchase for
 * @store_type: Store type
 * @base_price: Base price of item to inflate
 */
static long __store_min_deflated_price(enum store_t store_type,
                                       long base_price) {
  long min_price =
      trunc(base_price * (1 - owners[stores[store_type].owner].min_inflate));
  min_price -= (min_price * C_player_cost_modifier_from_charisma());
  if (min_price < 0) {
    min_price = 1;
  }
  return min_price;
}

static void __tick_down(long time_spent, long *flag) {
  if (*flag <= 1)
    return;

  *flag -= time_spent;
  if (*flag < 1) {
    *flag = 1;
  }
}

/**
 * __store_print_inventory() - Display's a store's inventory
 * @store_type: Type of store
 * @start: ???
 */
static void __store_print_inventory(enum store_t store_type, long start) {
  ENTER(("__store_print_inventory", ""));

  long i = ((start - 1) % 12);
  long stop = (((start - 1) / 12) + 1) * 12;

  if (stop > stores[store_type].store_ctr) {
    stop = stores[store_type].store_ctr;
  }

  for (; (start <= stop);) {
    inven_temp.data = stores[store_type].store_inven[start].sitem;
    if ((inven_temp.data.subval > 255) && (inven_temp.data.subval < 512)) {
      inven_temp.data.number = 1;
    }
    char out_val1[82];
    objdes(out_val1, &inven_temp, true);
    char out_val2[85];
    sprintf(out_val2, "%c) %s", (97 + (int)i), out_val1);
    prt(out_val2, i + 6, 1);
    if (stores[store_type].store_inven[start].scost < 0) {
      /*{quack}*/
      long cost = labs(stores[store_type].store_inven[start].scost);
      cost = cost + trunc(cost * C_player_cost_modifier_from_charisma());
      sprintf(out_val2, "%6ld", ((cost + GOLD_VALUE - 1) / GOLD_VALUE));
    } else {
      sprintf(out_val2, "%6ld [Fixed]",
              (stores[store_type].store_inven[start].scost / GOLD_VALUE));
    }
    prt(out_val2, i + 6, 60);
    i++;
    start++;
  }
  if (i < 12) {
    for (long j = 1; j <= (12 - i + 1); j++) {
      prt("", j + i + 5, 1);
    }
  }

  LEAVE("__store_print_inventory", "");
}

/**
 * __store_print_store_gold() - Displays player's gold in store
 */
static void __store_print_store_gold() {

  ENTER(("__store_print_store_gold", ""));

  char out_val[82];
  sprintf(out_val, "Gold Remaining : %ld", player_money[TOTAL_]);
  prt(out_val, 19, 18);

  LEAVE("__store_print_store_gold", "");
}

/**
 * __store_redraw_cost() - Re-displays only a single cost
 * @store_type: Type of store
 * @pos: Item position in list
 */
static void __store_redraw_cost(enum store_t store_type, long pos) {

  char out_val[82];

  long i = ((pos - 1) % 12);
  if (stores[store_type].store_inven[pos].scost < 0) {
    long cost = labs(stores[store_type].store_inven[pos].scost);
    cost += (long)(cost * C_player_cost_modifier_from_charisma());
    sprintf(out_val, "%6ld", (cost / GOLD_VALUE));
  } else {
    sprintf(out_val, "%6ld [FIXED]",
            (stores[store_type].store_inven[pos].scost / GOLD_VALUE));
  }

  prt(out_val, i + 6, 60);
}

/**
 * __store_close() - Deny the player entrance to the store
 * @store_type: Type of store
 */
static void __store_close(long store_type) {
  stores[store_type].store_open.year = player_cur_age.year;
  stores[store_type].store_open.month = player_cur_age.month;
  stores[store_type].store_open.day = player_cur_age.day;
  stores[store_type].store_open.hour = player_cur_age.hour;
  stores[store_type].store_open.secs = player_cur_age.secs;

  stores[store_type].store_open.day++;
  stores[store_type].store_open.hour = 6;
  stores[store_type].store_open.secs = randint(400) - 1;

  if (stores[store_type].store_open.day > 28) {
    stores[store_type].store_open.day = 1;
    stores[store_type].store_open.month++;
    if (stores[store_type].store_open.month > 13) {
      stores[store_type].store_open.month = 1;
      stores[store_type].store_open.year++;
    }
  }
}

/*{ Comment one : Finished haggling                               }*/
/**
 * __print_trade_accepted() - Print something the store keeper says to accept
 * the trade
 */
static void __print_trade_accepted() {

  msg_flag = false;
  switch (randint(15)) {
  case 1:
    msg_print("Done!");
    break;
  case 2:
    msg_print("Accepted!");
    break;
  case 3:
    msg_print("Fine...");
    break;
  case 4:
    msg_print("Agreed!");
    break;
  case 5:
    msg_print("Ok...");
    break;
  case 6:
    msg_print("Taken!");
    break;
  case 7:
    msg_print("You drive a hard bargain, but taken...");
    break;
  case 8:
    msg_print("You'll force me bankrupt, but it's a deal...");
    break;
  case 9:
    msg_print("Sigh...  I'll take it...");
    break;
  case 10:
    msg_print("My poor sick children may starve, but done!");
    break;
  case 11:
    msg_print("Finally!  I accept...");
    break;
  case 12:
    msg_print("Robbed again...");
    break;
  case 13:
    msg_print("A pleasure to do business with you!");
    break;
  case 14:
    msg_print("My spouse shall skin me, but accepted.");
    break;
  case 15:
    msg_print("Fine! Just be that way!");
    break;
  }
}

static void prt_comment2(long offer, long asking, long final) {
  /*{ %A1 is offer, %A2 is asking...                }*/

  char comment[82];

  if (final > 0) {
    switch (randint(3)) {
    case 1:
      strcpy(comment, "%A2 is my final offer; take it or leave it...");
      break;
    case 2:
      strcpy(comment, "I'll give you no more than %A2.");
      break;
    case 3:
      strcpy(comment, "My patience grows thin...  %A2 is final.");
      break;
    }
  } else {
    switch (randint(16)) {
    case 1:
      strcpy(comment, "%A1 for such a fine item?  HA!  No "
                      "less than %A2.");
      break;
    case 2:
      strcpy(comment, "%A1 is an insult!  Try %A2 gold pieces...");
      break;
    case 3:
      strcpy(comment, "%A1???  Thou would rob my poor "
                      "starving children?");
      break;
    case 4:
      strcpy(comment, "Why I'll take no less than %A2 gold pieces.");
      break;
    case 5:
      strcpy(comment, "Ha!  No less than %A2 gold pieces.");
      break;
    case 6:
      strcpy(comment, "Thou blackheart!  No less than %A2 gold pieces.");
      break;
    case 7:
      strcpy(comment, "%A1 is far too little, how about %A2?");
      break;
    case 8:
      strcpy(comment, "I paid more than %A1 for it myself, try %A2.");
      break;
    case 9:
      strcpy(comment, "%A1?  Are you mad???  How about %A2 gold pieces?");
      break;
    case 10:
      strcpy(comment, "As scrap this would bring %A1.  Try %A2 in gold.");
      break;
    case 11:
      strcpy(comment, "May fleas of a 1000 orcs molest you.  "
                      "I want %A2.");
      break;
    case 12:
      strcpy(comment, "My mother you can get for %A1, this costs %A2.");
      break;
    case 13:
      strcpy(comment, "May your chickens grow lips.  I want "
                      "%A2 in gold!");
      break;
    case 14:
      strcpy(comment, "Sell this for such a pittance.  Give "
                      "me %A2 gold.");
      break;
    case 15:
      strcpy(comment, "May the Balrog find you tasty!  %A2 gold pieces?");
      break;
    case 16:
      strcpy(comment, "Your mother was a Troll!  %A2 or I'll tell...");
      break;
    }

    insert_num(comment, "%A1", offer, false);
    insert_num(comment, "%A2", asking, false);
    msg_print(comment);
  }
}

static void prt_comment3(long offer, long asking, long final) {
  char comment[82];

  if (final > 0) {
    switch (randint(3)) {
    case 1:
      strcpy(comment, "I'll pay no more than %A1; take it or leave it.");
      break;
    case 2:
      strcpy(comment, "You'll get no more than %A1 from me...");
      break;
    case 3:
      strcpy(comment, "%A1 and that's final.");
      break;
    }
  } else {
    switch (randint(15)) {
    case 1:
      strcpy(comment, "%A2 for that piece of junk?  No more than %A1");
      break;
    case 2:
      strcpy(comment, "For %A2 I could own ten of those.  Try %A1.");
      break;
    case 3:
      strcpy(comment, "%A2?  NEVER!  %A1 is more like it...");
      break;
    case 4:
      strcpy(comment, "Let's be reasonable... How about %A1 "
                      "gold pieces?");
      break;
    case 5:
      strcpy(comment, "%A1 gold for that junk, no more...");
      break;
    case 6:
      strcpy(comment, "%A1 gold pieces and be thankful for it!");
      break;
    case 7:
      strcpy(comment, "%A1 gold pieces and not a copper more...");
      break;
    case 8:
      strcpy(comment, "%A2 gold?  HA!  %A1 is more like it...");
      break;
    case 9:
      strcpy(comment, "Try about %A1 gold...");
      break;
    case 10:
      strcpy(comment, "I wouldn't pay %A2 for your children, try %A1.");
      break;
    case 11:
      strcpy(comment, "*CHOKE* For that!?  Let's say %A1.");
      break;
    case 12:
      strcpy(comment, "How about %A1.");
      break;
    case 13:
      strcpy(comment, "That looks war surplus!  Say %A1 gold.");
      break;
    case 14:
      strcpy(comment, "I'll buy it as scrap for %A1.");
      break;
    case 15:
      strcpy(comment, "%A2 is too much, let us say %A1 gold.");
      break;
    }
  }
  insert_num(comment, "%A1", offer, false);
  insert_num(comment, "%A2", asking, false);
  msg_print(comment);
}

static void prt_comment4() {
  /*{ Kick 'da bum out...                                   -RAK-   }*/

  msg_flag = false;
  switch (randint(5)) {
  case 1:
    msg_print("ENOUGH!  Thou hath abused me once too often!");
    msg_print("Out of my place!");
    msg_print(" ");
    break;

  case 2:
    msg_print("THAT DOES IT!  You shall waste my time no more!");
    msg_print("out... Out... OUT!!!");
    msg_print(" ");
    break;

  case 3:
    msg_print("This is getting no where...  I'm going home!");
    msg_print("Come back tomorrow...");
    msg_print(" ");
    break;

  case 4:
    msg_print("BAH!  No more shall you insult me!");
    msg_print("Leave my place...  Begone!");
    msg_print(" ");
    break;

  case 5:
    msg_print("Begone!  I have had enough abuse for one day.");
    msg_print("Come back when thou art richer...");
    msg_print(" ");
    break;
  }
  msg_flag = false;
}

static void prt_comment5() {
  switch (randint(10)) {
  case 1:
    msg_print("You will have to do better than that!");
    break;
  case 2:
    msg_print("That's an insult!");
    break;
  case 3:
    msg_print("Do you wish to do business or not?");
    break;
  case 4:
    msg_print("Hah!  Try again...");
    break;
  case 5:
    msg_print("Ridiculous!");
    break;
  case 6:
    msg_print("You've got to be kidding!");
    break;
  case 7:
    msg_print("You better be kidding!!");
    break;
  case 8:
    msg_print("You try my patience.");
    break;
  case 9:
    msg_print("I don't hear you.");
    break;
  case 10:
    msg_print("Hmmm, nice weather we're having...");
    break;
  }
}

static void prt_comment6() {
  switch (randint(5)) {
  case 1:
    msg_print("I must of heard you wrong...");
    break;
  case 2:
    msg_print("What was that?");
    break;
  case 3:
    msg_print("I'm sorry, say that again...");
    break;
  case 4:
    msg_print("What did you say?");
    break;
  case 5:
    msg_print("Sorry, what was that again?");
    break;
  }
}

static void __wizard_print_insult(enum store_t store_type) {
  if (wizard1) {
    char out_val[82];
    sprintf(out_val, "Insult : %d / %d", stores[store_type].insult_cur,
            owners[stores[store_type].owner].insult_max);
    prt(out_val, 20, 18);
  }
}

/**
 * __store_get_insulted() - Increase the insult counter and get pissed if too
 * many
 * @store_type: Type of store
 * @return: True if you are now barred from the store
 */
static boolean __store_get_insulted(enum store_t store_type) {

  stores[store_type].insult_cur++;

  __wizard_print_insult(store_type);

  if (stores[store_type].insult_cur <=
      owners[stores[store_type].owner].insult_max)
    return false;

  prt_comment4();
  stores[store_type].insult_cur = 0;
  change_rep(-5);
  __store_close(store_type);
  return true;
}

/**
 * __store_lower_insult_level() - Lower insult counter for shop
 * @store_type: Type of store
 */
static void __store_lower_insult_level(enum store_t store_type) {
  /* with stores[store_type] do; */
  stores[store_type].insult_cur -= 2;
  if (stores[store_type].insult_cur < 0) {
    stores[store_type].insult_cur = 0;
  }
}

/**
 * __store_display_commands() - Display set of store commands
 */
static void __store_display_commands() {
  ENTER(("__store_display_commands", ""));

  prt("You may:", 21, 1);
  prt(" p/P) Purchase an item.          <space> browse store's inventory.", 22,
      1);
  prt(" s/S) Sell an item.              i) Inventory and Equipment Lists.", 23,
      1);
  prt(" ^R) Redraw the screen.        Esc) Exit from building.", 24, 1);

  LEAVE("__store_display_commands", "");
}

/**
 * __haggle_insults() - Have insulted while haggling
 */
static boolean __haggle_insults(enum store_t store_num) {

  if (__store_get_insulted(store_num)) {
    return true;
  } else {
    prt_comment5();
    return false;
  }
}

static enum trade_status_t __receive_offer(long store_num, char comment[82],
                                           long *new_offer, long last_offer,
                                           long factor) {

  long const comment_len = strlen(comment) + 1;
  for (;;) {
    // START
    msg_print(comment);
    msg_flag = false;

    char player_input[82];
    if (!get_string(player_input, 1, comment_len, 40)) {
      erase_line(msg_line, msg_line);
      return TS_ABORTED;
    }

    long player_input_as_long;
    if (sscanf(player_input, "%ld", &player_input_as_long) != 1) {
      continue;
    }
    if (player_input_as_long <= 0) {
      continue;
    }

    *new_offer = player_input_as_long;

    if ((*new_offer * factor) >= (last_offer * factor)) {
      break;
    }

    if (__haggle_insults(store_num)) {
      return TS_REFUSED_INSULTED;
    }
  }

  return TS_SUCCESS;
}

/**
 * __purchase_blitz() - Purchase something without haggling
 * @store_type: Type of store
 * @item: Item to purchase
 * @returns: Trade result
 */
static trade_result_t __purchase_blitz(enum store_t store_type,
                                       treasure_type const *item) {
  long const base_price = __calc_purchase_price(store_type, item);
  long const max_price = __store_max_inflated_price(store_type, base_price);
  long const min_price = __store_min_inflated_price(store_type, base_price);

  long const max_min_diff = max_price - min_price;
  long const last_offer = min_price + (max_min_diff / 4);
  long price = last_offer + ((stores[store_type].insult_cur * max_min_diff) /
                             owners[stores[store_type].owner].insult_max);
  msg_printf("In a hurry, eh?  It's yours for a mere %ld       ", price);
  msg_print(" ");
  prt("", 2, 1);
  __store_display_commands();
  return (trade_result_t){.status = TS_SUCCESS, .price = price};
}

/**
 * __purchase_haggle() - Haggling routine
 */
static trade_result_t __purchase_haggle(enum store_t store_num, treasure_type *item) {

  msg_flag = false;

  long const base_price = __calc_purchase_price(store_num, item);
  long const max_price = __store_max_inflated_price(store_num, base_price);
  long const min_price = __store_min_inflated_price(store_num, base_price);
  long const max_buy = __store_min_deflated_price(store_num, base_price);
  float const min_per = owners[stores[store_num].owner].haggle_per;
  float const max_per = min_per * 3.0;

  long cur_ask = max_price;
  long const final_ask = min_price;
  long min_offer = max_buy;
  long last_offer = min_offer;

  char comment[82];
  strcpy(comment, "Asking : ");

  // Haggling
  long final_flag = 0;
  long new_offer;

  prt("Specify an offer in gold pieces.", 22, 1);
  prt("Esc) Quit Haggling.", 23, 1);
  prt("", 24, 1);

  for (;;) {
    char out_val[100];
    sprintf(out_val, "%s%ld          ", comment, cur_ask);
    put_buffer(out_val, 2, 1);
    enum trade_status_t offer_result = __receive_offer(
        store_num, "What do you offer? ", &new_offer, last_offer, 1);

    if (offer_result == TS_ABORTED)
      return (trade_result_t){.status = TS_ABORTED, .price = 0};
    else if (offer_result == TS_REFUSED_INSULTED)
      return (trade_result_t){.status = TS_REFUSED_INSULTED, .price = 0};
    else if (offer_result == TS_SUCCESS) {
      if (new_offer == cur_ask)
        return (trade_result_t){.status = TS_SUCCESS, .price = new_offer};
      else if (new_offer > cur_ask) {
        prt_comment6(); // Player probably typed wrong
        continue;
      }
    }

    msg_flag = false;
    float x1 = (float)(new_offer - last_offer) / (float)(cur_ask - last_offer);
    if (x1 < min_per) {
      if (__haggle_insults(store_num))
        return (trade_result_t){.status = TS_REFUSED_INSULTED, .price = 0};
      continue;
    }

    if (x1 > max_per) {
      x1 *= 0.75;
      if (x1 < max_per) {
        x1 = max_per;
      }
    }
    float x2 = (x1 + (randint(5) - 3) / 100.0);
    long x3 = trunc((cur_ask - new_offer) * x2) + 1;
    cur_ask -= x3;
    if (cur_ask < final_ask) {
      cur_ask = final_ask;
      strcpy(comment, "Final Offer : ");
      final_flag++;

      if (final_flag > 3) {
        if (__store_get_insulted(store_num))
          return (trade_result_t){.status = TS_REFUSED_INSULTED, .price = 0};
        else
          return (trade_result_t){.status = TS_ABORTED, .price = 0};
      }
    } else if (new_offer >= cur_ask)
      return (trade_result_t){.status = TS_SUCCESS, .price = new_offer};

    last_offer = new_offer;
    prt("", 2, 1);
    sprintf(out_val, "Your last offer : %ld   ", last_offer);
    put_buffer(out_val, 2, 40);
    prt_comment2(last_offer, cur_ask, final_flag);
  }
}

/**
 * __store_purchase_select_item() - Select item to purchase
 * @store_type: Type of store
 * @cur_top: ???
 * @blitz: Are we blitz-buying?
 * @return: Selected item index, or -1 if we changed our mind
 */
static int __store_purchase_select_item(enum store_t store_type, long cur_top,
                                        boolean blitz) {
  int objects_shown_on_screen;
  if (cur_top == 13) {
    objects_shown_on_screen = stores[store_type].store_ctr - 12;
  } else if (stores[store_type].store_ctr > 12) {
    objects_shown_on_screen = 12;
  } else {
    objects_shown_on_screen = stores[store_type].store_ctr;
  }

  char prt_buf[82];
  if (blitz) {
    sprintf(prt_buf, "(Items a-%c, Esc to exit) BLITZ-PURCHASE Item? ",
            objects_shown_on_screen + 'a' - 1);
  } else {
    sprintf(prt_buf, "(Items a-%c, Esc to exit) Purchase which item? ",
            objects_shown_on_screen + 'a' - 1);
  }

  int selected_item = -1;
  while (selected_item < 1 || objects_shown_on_screen < selected_item) {
    prt(prt_buf, 1, 1);
    char command = inkey();
    switch (command) {
    case 3:
    case 25:
    case 26:
    case 27:
      selected_item = -1;
      goto break_outer;
    default:
      selected_item = (int)command - 96;
      break;
    }
  }
break_outer:

  msg_flag = false;
  erase_line(msg_line, msg_line);

  return selected_item + cur_top - 1;
}

/**
 * __store_purchase() - Buy an item from a store
 * @store_type: Type of store
 * @cur_top: ???
 * @blitz: If we are blitz-buying
 */
static boolean __store_purchase(enum store_t store_type, long *cur_top,
                                boolean blitz) {

  if (stores[store_type].store_ctr < 1) {
    msg_print("I am currently out of stock.");
    return false;
  }

  int selected_item = __store_purchase_select_item(store_type, *cur_top, blitz);
  if (selected_item == -1)
    return false;

  inven_temp.data = stores[store_type].store_inven[selected_item].sitem;

  long save_number;
  if ((inven_temp.data.subval > 255) && (inven_temp.data.subval < 512)) {
    save_number = inven_temp.data.number;
    inven_temp.data.number = 1;
  } else {
    save_number = 1;
  }

  if (!inven_check_weight() && store_type != S_INN) {
    prt("You can not carry that much weight.", 1, 1);
    return false;
  }

  if (!inven_check_num() && store_type != S_INN) {
    prt("You cannot carry that many different items.", 1, 1);
    return false;
  }

  trade_result_t trade_result;
  if (stores[store_type].store_inven[selected_item].scost > 0) {
    trade_result.price =
        stores[store_type].store_inven[selected_item].scost / GOLD_VALUE;
    trade_result.status = TS_SUCCESS;
  } else if (blitz) {
    trade_result = __purchase_blitz(store_type, &inven_temp.data);
  } else {
    trade_result = __purchase_haggle(store_type, &inven_temp.data);
    prt("", 2, 1);
    __store_display_commands();
  }

  if (trade_result.status == TS_ABORTED) {
    prt("", 2, 1);
    return false;
  }

  if (trade_result.status != TS_SUCCESS) {
    prt("", 2, 1);
    return true;
  }

  long price = trade_result.price;

  // Check if we can pay for it
  boolean could_pay;
  if (player_money[TOTAL_] >= trade_result.price) {
    subtract_money(trade_result.price * GOLD_VALUE, true);
    could_pay = true;
  } else {
    long to_bank = price - player_money[TOTAL_];
    could_pay = send_page(to_bank);
  }

  if (!could_pay) {
    boolean barred_from_store = __store_get_insulted(store_type);
    if (barred_from_store) {
      prt("", 2, 1);
      return true;
    } else {
      msg_print("Liar!  You have not the gold!");
      prt("", 2, 1);
      return false;
    }
  }

  __print_trade_accepted();
  __store_lower_insult_level(store_type);
  boolean return_value;
  if (store_type == S_INN) {
    if (stores[store_type].store_inven[selected_item].scost < 0) {
      stores[store_type].store_inven[selected_item].scost = price * GOLD_VALUE;
    }
    spend_time(stores[store_type].store_inven[selected_item].sitem.p1,
               "at the Inn.", true);
    if (stores[store_type].store_inven[selected_item].sitem.subval == 303) {
      spend_time(600, "eating.", false);
      msg_print("You eat a leisurely meal of buckwheat cakes and bacon.");
      player_flags.foodc = PLAYER_FOOD_FULL;
      player_flags.status &= ~(IS_WEAK | IS_HUNGERY);
      msg_print(" ");
    }
    return_value = true;
  } else {
    __remove_item_from_store(store_type, selected_item, true);
    treas_rec *item_new = inven_carry();
    char out_val[82];
    objdes(out_val, item_new, true);
    char out2[100];
    sprintf(out2, "You have %s", out_val);
    msg_print(out2);
    if (*cur_top > stores[store_type].store_ctr) {
      *cur_top = 1;
      __store_print_inventory(store_type, *cur_top);
    } else {
      if (save_number > 1) {
        if (stores[store_type].store_inven[selected_item].scost < 0) {
          stores[store_type].store_inven[selected_item].scost =
              price * GOLD_VALUE;
          __store_redraw_cost(store_type, selected_item);
        }
      } else {
        __store_print_inventory(store_type, selected_item);
      }
    }
    __store_print_store_gold();
  }
  prt("", 2, 1);

  return return_value;
}

/**
 * __sell_blitz() - Blitz sell an item to a store
 * @store_type: Type of store
 * @item: Item to sell
 * @return: Agreed sell price
 */
static long __sell_blitz(enum store_t store_type, treasure_type const *item) {
  long const cost = __calc_sell_price(store_type, item);
  long const max_buy = __store_max_deflated_price(store_type, cost);
  long const min_buy = __store_min_deflated_price(store_type, cost);

  long const min_max_diff = (min_buy - max_buy);
  long last_offer = min_buy - (min_max_diff / 7);
  long price = last_offer - ((stores[store_type].insult_cur * min_max_diff) /
                             owners[stores[store_type].owner].insult_max);
  msg_printf("Need cash quick?  I'll pay you %ld   ", price);
  msg_print(" ");
  prt(" ", 2, 1);
  __store_display_commands();
  return price;
}

/**
 * __sell_haggle() - Haggling routine
 * @store_type: Type of store
 * @price:
 * @item:
 * @blitz:
 * @return: 0 = Sold, 2 = Aborted, 3 or 4 = Owner will not buy
 */
static trade_result_t __sell_haggle(enum store_t store_type, treasure_type *item) {

  msg_flag = false;

  long cost = __calc_sell_price(store_type, item);
  if (cost < 1) {
    return (trade_result_t){.status = TS_REFUSED_TRASH, .price = 0};
  }

  if (item->flags2 & Blackmarket_bit) {
    return (trade_result_t){.status = TS_REFUSED_BLACKMARKET, .price = 0};
  }

  long const max_sell = __store_max_inflated_price(store_type, cost);
  long const max_buy = __store_max_deflated_price(store_type, cost);
  long const min_buy = __store_min_deflated_price(store_type, cost);
  float const min_per = (owners[stores[store_type].owner]).haggle_per;
  float const max_per = min_per * 3.0;

  // Haggling
  prt("Specify an asking-price in gold pieces.", 22, 1);
  prt("Esc) Quit Haggling.", 23, 1);
  prt("", 24, 1);

  long const store_max_gold = owners[stores[store_type].owner].max_cost;
  long cur_ask;
  long final_ask;
  long final_flag = 0;
  char comment[82];
  if (max_buy > store_max_gold) {
    strcpy(comment, "Final offer : ");
    final_flag = 1;
    cur_ask = store_max_gold;
    final_ask = store_max_gold;
    msg_print(
        "I am sorry, but I have not the money to afford such a fine item.");
    msg_print(" ");
  } else {
    cur_ask = max_buy;
    final_ask = min_buy;
    if (final_ask > store_max_gold) {
      final_ask = store_max_gold;
    }
    strcpy(comment, "Offer : ");
  }

  long min_offer = max_sell;
  long last_offer = min_offer;
  if (cur_ask < 1) {
    cur_ask = 1;
  }

  long new_offer;
  for (;;) {
    char out_val[100];
    sprintf(out_val, "%s%ld       ", comment, cur_ask);
    put_buffer(out_val, 2, 1);
    enum trade_status_t offer_result = __receive_offer(store_type, "What price do you ask? ",
        &new_offer, last_offer, -1);

    if (offer_result == TS_ABORTED)
      return (trade_result_t){.status = TS_ABORTED, .price = 0};
    else if (offer_result == TS_REFUSED_INSULTED)
      return (trade_result_t){.status = TS_REFUSED_INSULTED, .price = 0};
    else if (offer_result == TS_SUCCESS) {
      if (new_offer == cur_ask)
        return (trade_result_t){.status = TS_SUCCESS, .price = new_offer};
      else if (new_offer < cur_ask) {
          prt_comment6();
          continue;
      }
    }

    msg_flag = false;
    float x1 = (float)(last_offer - new_offer) / (float)(last_offer - cur_ask);
    if (x1 < min_per) {
      if (__haggle_insults(store_type))
        return (trade_result_t){.status = TS_REFUSED_INSULTED, .price = 0};
      continue;
    }

    if (x1 > max_per) {
      x1 *= 0.75;
      if (x1 < max_per) {
        x1 = max_per;
      }
    }
    float x2 = (x1 + (randint(5) - 3) / 100.0);
    long x3 = trunc((new_offer - cur_ask) * x2) + 1;
    cur_ask += x3;
    if (cur_ask > final_ask) {
      cur_ask = final_ask;
      strcpy(comment, "Final Offer : ");
      final_flag++;

      if (final_flag > 3) {
        if (__store_get_insulted(store_type))
          return (trade_result_t){.status = TS_REFUSED_INSULTED, .price = 0};
        else
          return (trade_result_t){.status = TS_ABORTED, .price = 0};
      }
    } else if (new_offer <= cur_ask)
      return (trade_result_t){.status = TS_SUCCESS, .price = new_offer};

    last_offer = new_offer;
    prt(" ", 2, 1);
    sprintf(out_val, "Your last bid   : %ld   ", last_offer);
    put_buffer(out_val, 2, 40);
    prt_comment3(cur_ask, last_offer, final_flag);
  }
}

/**
 * __store_sell() - Sell an item to the store
 * @store_type: Type of store
 * @cur_top: ???
 * @blitz: If we are blitz-selling
 */
static boolean __store_sell(enum store_t store_type, long cur_top,
                            boolean blitz) {

  ENTER(("__store_sell", ""));

  char foo[82];
  if (blitz) {
    strcpy(foo, "BLITZ-SELLING item? ");
  } else {
    strcpy(foo, "Which one? ");
  }

  long count = 0;
  treas_rec *item_ptr = NULL;
  if (!find_range(store_buy[store_type], false, &item_ptr, &count)) {
    msg_print("You have nothing the store wishes to buy.");
    LEAVE("__store_sell", "");
    return false;
  }

  boolean return_value = false;
  boolean redraw = false;
  char trash_char;
  if (!get_item(&item_ptr, foo, &redraw, count, &trash_char, false, false)) {
    if (redraw) {
      display_store(store_type, cur_top);
    }
    LEAVE("__store_sell", "");
    return false;
  }

  if (redraw) {
    display_store(store_type, cur_top);
  }
  inven_temp.data = item_ptr->data;
  if ((inven_temp.data.subval > 255) && (inven_temp.data.subval < 512)) {
    inven_temp.data.number = 1; /*{But why????}*/
  }

  char out_val[82];
  char out2[100];
  objdes(out_val, &inven_temp, true);
  sprintf(out2, "Selling %s", out_val);
  msg_print(out2);
  msg_print(" ");
  if (!is_in(inven_temp.data.tval, store_buy[store_type])) {
    prt("I do not buy such items.", 1, 1);
    LEAVE("__store_sell", "");
    return false;
  }

  // TODO: Stop using inven_temp
  if (!__store_has_space_for(store_type, &inven_temp)) {
    prt("I have not the room in my store to keep it...", 1, 1);
    LEAVE("__store_sell", "");
    return false;
  }

  trade_result_t trade_result;
  if (blitz) {
    trade_result.price = __sell_blitz(store_type, &inven_temp.data);
    trade_result.status = TS_SUCCESS;
  } else {
    trade_result = __sell_haggle(store_type, &inven_temp.data);
    prt(" ", 2, 1);
    __store_display_commands();
  }

  switch (trade_result.status) {
  case TS_SUCCESS:
    __print_trade_accepted();
    add_money(trade_result.price * GOLD_VALUE);
    inven_destroy(item_ptr);
    long item_pos;
    store_carry(store_type, &item_pos);
    if (item_pos > 0) {
      if (item_pos < 13) {
        if (cur_top < 13) {
          __store_print_inventory(store_type, item_pos);
        } else {
          __store_print_inventory(store_type, cur_top);
        }
      } else if (cur_top > 12) {
        __store_print_inventory(store_type, item_pos);
      }
    }
    __store_print_store_gold();
    break;

  case TS_REFUSED_INSULTED:
    return_value = true;
    break;

  case TS_REFUSED_TRASH:
    msg_print("How dare you!");
    msg_print("I will not buy that!");
    return_value = __store_get_insulted(store_type);
    break;

  case TS_REFUSED_BLACKMARKET:
    /* black market or insured stuff after a death */
    msg_print("Hmmmmm, that looks hot. I will not buy it!");
    msg_print("I am an honest merchant!");

  default:
    break;
  }

  LEAVE("__store_sell", "");
  return return_value;
}

/**
 * __store_enter() - Entering a store
 * @store_type: type of store
 */
static void __store_enter(enum store_t store_type) {

  ENTER(("__store_enter", ""));

  treas_rec *trash_ptr;

  long tics = 1;
  boolean exit_flag = false;
  long cur_top = 1;

  display_store(store_type, cur_top);

  do {
    char command;
    if (get_com("", &command)) {
      msg_flag = false;
      switch (command) {
      case 18:
        display_store(store_type, cur_top);
        break;
      case 'I': /*{ Selective Inventory   }*/
        if (inven_command('I', &trash_ptr, "")) {
          display_store(store_type, cur_top);
        }
        break;
      case ' ':
        if (cur_top == 1) {
          if (stores[store_type].store_ctr > 12) {
            cur_top = 13;
            __store_print_inventory(store_type, cur_top);
          } else {
            prt("Entire inventory is "
                "shown.",
                1, 1);
          }
        } else {
          cur_top = 1;
          __store_print_inventory(store_type, cur_top);
        }
        break;
      case 'e': /*{ Equipment List        }*/
        if (inven_command('e', &trash_ptr, "")) {
          display_store(store_type, cur_top);
        }
        break;
      case 'i': /*{ Inventory             }*/
        if (inven_command('i', &trash_ptr, "")) {
          display_store(store_type, cur_top);
        }
        break;
      case 't': /*{ Take off              }*/
        if (inven_command('t', &trash_ptr, "")) {
          display_store(store_type, cur_top);
        }
        break;
      case 'w': /*{ Wear                  }*/
        if (inven_command('w', &trash_ptr, "")) {
          display_store(store_type, cur_top);
        }
        break;
      case 'x': /*{ Switch weapon         }*/
        if (inven_command('x', &trash_ptr, "")) {
          display_store(store_type, cur_top);
        }
        break;
      case 'p':
        exit_flag = __store_purchase(store_type, &cur_top, false);
        break;
      case 'P':
        exit_flag = __store_purchase(store_type, &cur_top, true);
        break;
      case 's':
        exit_flag = __store_sell(store_type, cur_top, false);
        break;
      case 'S':
        exit_flag = __store_sell(store_type, cur_top, true);
        break;
      default:
        prt("Invalid Command.", 1, 1);
        break;
      } /* end switch */
    } else {
      exit_flag = true;
    }
    adv_time(false);
    tics++;
    if (tics % 2 == 1) {
      kick__kickout_player_if_time();
    }
  } while (!exit_flag);

  if (moria_flag) {
    clear_rc(1, 1);
    prt_stat_block();
  } else {
    draw_cave();
  }

  LEAVE("__store_enter", "");
}

void store_maint() {

  while (stores[S_BLACK_MARKET].store_ctr > 0) {
    __remove_item_from_store(S_BLACK_MARKET, stores[S_BLACK_MARKET].store_ctr,
                             false);
  }

  for (int i = 0; i < MAX_STORES; i++) {
    stores[i].insult_cur = 0;

    long num_things_in_store = stores[i].store_ctr;
    if (num_things_in_store > STORE_MAX_INVEN) {
      // Clean up if too much stuff
      for (int j = 0, cnt = (num_things_in_store - STORE_MAX_INVEN + 2);
           j < cnt; j++) {
        __remove_item_from_store(i, randint(stores[i].store_ctr), false);
      }
    } else if (num_things_in_store < STORE_MIN_INVEN) {
      // Add new stuff if needed
      for (int j = 0, cnt = (STORE_MIN_INVEN - num_things_in_store + 2);
           j < cnt; j++) {
        __add_item_to_store(i);
      }
    } else {
      // Change the stock a bit
      for (int j = 0, cnt = (1 + randint(STORE_TURN_AROUND)); j < cnt; j++) {
        __remove_item_from_store(i, randint(stores[i].store_ctr), true);
      }
      for (int j = 0, cnt = (1 + randint(STORE_TURN_AROUND)); j < cnt; j++) {
        __add_item_to_store(i);
      }
    }

    num_things_in_store = stores[i].store_ctr;
    if (i == S_INN) {
      if (randint(8) == 1) {
        for (int j = 0; j < num_things_in_store; j++) {
          __remove_item_from_store(i, j, false);
        }
        for (int j = 0; j < STORE_MIN_INVEN + 2; j++) {
          __add_item_to_store(i);
        }
      }
    }
  }

  __randomize_cash(&(bank[IRON]), 500000);
  __randomize_cash(&(bank[COPPER]), 200000);
  __randomize_cash(&(bank[SILVER]), 100000);
  __randomize_cash(&(bank[GOLD]), 50000);
  __randomize_cash(&(bank[PLATINUM]), 5000);
  __randomize_cash(&(bank[MITHRIL]), 1000);
  bank[TOTAL_] =
      (bank[MITHRIL] * MITHRIL_VALUE + bank[PLATINUM] * PLATINUM_VALUE) /
          GOLD_VALUE +
      bank[GOLD];
}

void store_carry(enum store_t store_num, long *ipos) {

  // TODO: Stop using inven_temp
  *ipos = 0;
  identify(&(inven_temp.data));
  unquote(inven_temp.data.name);
  known1(inven_temp.data.name);
  known2(inven_temp.data.name);

  long item_base_price = __calc_purchase_price(store_num, &inven_temp.data);
  long max_price = __store_max_inflated_price(store_num, item_base_price);

  if (max_price <= 0)
    return;

  long item_val = 0;
  long item_num = inven_temp.data.number;
  long typ = inven_temp.data.tval;
  long subt = inven_temp.data.subval;
  boolean flag = false;

  do {
    item_val++;
    if (typ == stores[store_num].store_inven[item_val].sitem.tval) {
      if (subt == stores[store_num].store_inven[item_val].sitem.subval) {
        /*{ Adds to other item	}*/
        if (subt > 255) {
          if (stores[store_num].store_inven[item_val].sitem.number < 24) {
            stores[store_num].store_inven[item_val].sitem.number += item_num;
          }
          flag = true;
        }
      }
    } else if (typ > stores[store_num].store_inven[item_val].sitem.tval) {
      /*{ Insert into list		}*/
      __insert_item_in_store(store_num, item_val, max_price);
      flag = true;
      *ipos = item_val;
    }
    /* end of thrid with */
  } while (!((item_val >= stores[store_num].store_ctr) || (flag)));
  if (!(flag)) { /*{ Becomes last item in list	}*/
    __insert_item_in_store(store_num, stores[store_num].store_ctr + 1,
                           max_price);
    *ipos = stores[store_num].store_ctr;
  }
}

long item_value(treasure_type const *const item) {
  long return_value = item->cost / GOLD_VALUE;

  switch (item->tval) {

    // Weapons and armor
  case bow_crossbow_or_sling:
  case hafted_weapon:
  case pole_arm:
  case sword:
  case dagger:
  case maul:
  case boots:
  case gloves_and_gauntlets:
  case gem_helm:
  case cloak:
  case helm:
  case shield:
  case hard_armor:
  case soft_armor:

    if (strstr(item->name, "^") != NULL) {
      return_value *= item->number;

    } else {
      switch (item->tval) {
      case bow_crossbow_or_sling:
      case hafted_weapon:
      case pole_arm:
      case sword:
      case dagger:
      case maul:
        if (item->tohit < 0) {
          return_value = 0;
        } else if (item->todam < 0) {
          return_value = 0;
        } else if (item->toac < 0) {
          return_value = 0;
        } else {
          return_value = ((item->cost / GOLD_VALUE +
                           (item->tohit + item->todam + item->toac) * 100) *
                          item->number);
        }
        break;

      default:
        if (item->toac < 0) {
          return_value = 0;
        } else {
          return_value =
              ((item->cost / GOLD_VALUE + item->toac * 100) * item->number);
        }
        break;
      }
    }
    break;

    // Ammo
  case sling_ammo:
  case bolt:
  case arrow:
  case spike:

    if (strstr(item->name, "^") != NULL) {
      return_value *= (item->number);
    } else {
      if (item->tohit < 0) {
        return_value = 0;
      } else if (item->todam < 0) {
        return_value = 0;
      } else if (item->toac < 0) {
        return_value = 0;
      } else {
        return_value = ((item->cost / GOLD_VALUE +
                         (item->tohit + item->todam + item->toac) * 10) *
                        (item->number));
      }
    }
    break;

    // Potions, Scrolls, and Food
  case scroll1:
  case scroll2:
  case potion1:
  case potion2:
  case Food:

    if (strstr(item->name, "|") != NULL) {
      switch (item->tval) {
      case scroll1:
      case scroll2:
        return_value = 20;
        break;

      case potion1:
      case potion2:
        return_value = 20;
        break;

      case Food:
        return_value = 1;
        break;
      }
    }
    break;

    // Rings and amulets
  case amulet:
  case ring:

    if (strstr(item->name, "|") != NULL) {
      switch (item->tval) {
      case amulet:
        return_value = 45;
        break;
      case ring:
        return_value = 45;
        break;
      }
    } else if (strstr(item->name, "^") != NULL) {
      return_value = (item->cost > 0) ? (item->cost / GOLD_VALUE) : 0;
    } else {
      return_value =
          (item->cost > 0)
              ? ((item->cost / GOLD_VALUE) +
                 trunc(item->cost / COST_ADJ / 20.0) *
                     (item->tohit + item->todam + item->toac + 2 * item->p1))
              : 0;
    }
    break;

    // Horns and Chimes
  case chime:
  case horn:

    if (strstr(item->name, "|") != NULL) {
      switch (item->tval) {
      case chime:
        return_value = 50;
        break;
      case horn:
        return_value = 80;
        break;
      }
    } else if (strstr(item->name, "^") == NULL) {
      return_value = ((item->cost / GOLD_VALUE) +
                      trunc(item->cost / COST_ADJ / 20.0) * (item->p1));
    }
    break;

    // Wands rods, and staffs
  case staff:
  case rod:
  case wand:

    if (strstr(item->name, "|") != NULL) {
      switch (item->tval) {
      case staff:
        return_value = 70;
        break;
      case rod:
        return_value = 60;
        break;
      case wand:
        return_value = 50;
        break;
      }
    } else if (strstr(item->name, "^") == NULL) {
      return_value = ((item->cost / GOLD_VALUE) +
                      trunc(item->cost / COST_ADJ / 20.0) * (item->p1));
    }
    break;

    // Gems and jewelry of all types
  case valuable_jewelry:
  case valuable_gems:

    if (strstr(item->name, "|") != NULL) {
      switch (item->tval) {
      case valuable_jewelry:
        return_value = 20;
        break;
      case valuable_gems:
        return_value = 20;
        break;
      }
    } else if (strstr(item->name, "^") == NULL) {
      return_value = (item->cost / GOLD_VALUE);
    }
    break;
  }

  return return_value;
}

boolean check_store_hours(enum store_t store_type, long store_visual) {
  boolean is_open;

  ENTER(("check_store_hours", ""));

  if (store_visual != -1) {
    is_open =
        ((player_cur_age.year > stores[store_visual].store_open.year) ||
         ((player_cur_age.year == stores[store_visual].store_open.year) &&
          ((player_cur_age.month > stores[store_visual].store_open.month) ||
           ((player_cur_age.month == stores[store_visual].store_open.month) &&
            ((player_cur_age.day > stores[store_visual].store_open.day) ||
             ((player_cur_age.day == stores[store_visual].store_open.day) &&
              ((player_cur_age.hour > stores[store_visual].store_open.hour) ||
               ((player_cur_age.hour == stores[store_visual].store_open.hour) &&
                ((player_cur_age.secs >
                  stores[store_visual].store_open.secs))))))))));
  } else {
    is_open = true;
    store_visual = store_type;
  }

  if (!is_open) {
    msg_print("The doors are locked.");
    LEAVE("check_store_hours", "");
    return false;
  }

  char availability = store_hours[store_visual][player_cur_age.day % 7 + 0]
                                 [player_cur_age.hour / 2 + 0];
  switch (availability) {
  case ' ':
    LEAVE("check_store_hours", "");
    return true;

  case 'N':
  case 'W':
  case 'D': {
    if (wizard2) {
      msg_print("Being a wizard, you break into the shop.");
      msg_print("");
      LEAVE("check_store_hours", "");
      return true;
    }

    char const *time;
    switch (availability) {
    case 'N':
      time = "night.";
      break;
    case 'W':
      time = "weekend.";
      break;
    case 'D':
      time = "day.";
      break;
    }
    char name[134];
    strcpy(name, store_door[store_type].name);
    insert_str(name, "the entrance to the ", "");
    char out_val[300];
    sprintf(out_val, "Sorry, the %s is closed for the %s", name, time);
    msg_print(out_val);
    LEAVE("check_store_hours", "");
    return false;
  } break;

  case 'B': {
    char prop[134];
    sprintf(prop, "Do you wish to play %ld gold to bribe the owner?",
            store_bribe[store_type]);
    if (!get_yes_no(prop)) {
      msg_print("The owner complains bitterly about "
                "being woken up for no reason.");
      LEAVE("check_store_hours", "");
      return false;
    }

    if (player_money[TOTAL_] >= store_bribe[store_type]) {
      subtract_money(store_bribe[store_type] * GOLD_VALUE, false);
      msg_print("The owner reluctantly lets you in.");
      msg_print("");
      LEAVE("check_store_hours", "");
      return true;
    } else {
      msg_print("You haven't the money to bribe the owner!");
      LEAVE("check_store_hours", "");
      return false;
    }
  } break;

  default:
    LEAVE("check_store_hours", "");
    return false;
  }
}

void check_store_hours_and_enter(enum store_t type, long store_visual) {

  ENTER(("check_store_hours_and_enter", ""));

  if (check_store_hours(type, store_visual))
    __store_enter(type);

  LEAVE("check_store_hours_and_enter", "");
}

void spend_time(long days_spent, char place[82], boolean whole_days) {

  long const turns_today = player_cur_age.hour * 400 + player_cur_age.secs;

  long mornings;
  long time_spent;
  boolean new_screen;
  if (!whole_days) {
    time_spent = days_spent; /*{if a 6:00 threshold is passed}*/

    new_screen = ((turns_today + time_spent + 2400) / 4800) >
                 ((turns_today + 2400) / 4800);

    mornings =
        (turns_today + time_spent - 2400) / 9600 - (turns_today - 2400) / 9600;
    days_spent = 0;
  } else {
    time_spent = DAY_LENGTH * days_spent - turns_today;
    new_screen = true;
    mornings = days_spent;
  }

  switch (days_spent) {
  case 0:
    player_cur_age.secs += time_spent;
    player_cur_age.hour += player_cur_age.secs / 400;
    player_cur_age.secs = player_cur_age.secs % 400;
    add_days(&(player_cur_age), player_cur_age.hour / 24);
    player_cur_age.hour = player_cur_age.hour % 24;
    break;

  case 1:
    if (player_cur_age.hour < 6) {
      char out_val[82];
      sprintf(out_val, "You spend the remainder of the night %s", place);
      msg_print(out_val);
      player_cur_age.hour = 8; /*{why get up before shops open?}*/
      player_cur_age.secs = randint(400) - 1;
      time_spent = (time_spent - DAY_LENGTH + 400 * player_cur_age.hour +
                    player_cur_age.secs);
    } else {
      char out_val[82];
      sprintf(out_val, "You spend the night %s", place);
      msg_print(out_val);
      player_cur_age.hour = 8;
      add_days(&(player_cur_age), 1);
      player_cur_age.secs = randint(400) - 1;
      time_spent += 400 * player_cur_age.hour + player_cur_age.secs;
    }
    break;

  case 3:
    msg_print("You spend three days in the inn.");
    /* 10/26/00 --JEB.  what the hell was "28+randint(3)" doing in
     */
    /* this context? */
    /*    add_days(&(player_cur_age),28+randint(3)); */
    add_days(&(player_cur_age), 3);
    player_cur_age.hour = 8 + randint(4);
    player_cur_age.secs = randint(400) - 1;
    time_spent += 400 * player_cur_age.hour + player_cur_age.secs;
    break;

  case 7:
    msg_print("You spend the week in the inn.");
    add_days(&(player_cur_age), 7);
    player_cur_age.hour = 8 + randint(4);
    player_cur_age.secs = randint(400) - 1;
    time_spent += 400 * player_cur_age.hour + player_cur_age.secs;
    break;
  }

  refresh();
  turn += time_spent;
  turn_counter += QUEST_DELAY;

  if (new_screen) {
    sleep(1);
  }

  for (long i = time_spent; (player_flags.poisoned > 0) && (i > 0);) {
    player_flags.poisoned--;
    time_spent--;

    switch (C_player_hp_from_con()) {
    case -4:
      take_hit(4, "poison");
      break;
    case -3:
    case -2:
      take_hit(3, "poison");
      break;
    case -1:
      take_hit(2, "poison");
      break;
    case 0:
      take_hit(1, "poison");
      break;
    case 1:
    case 2:
    case 3:
      if (i % 2 == 0) {
        take_hit(1, "poison");
      }
      break;
    case 4:
    case 5:
      if (i % 3 == 0) {
        take_hit(1, "poison.");
      }
      break;
    case 6:
      if (i % 4 == 0) {
        take_hit(1, "poison.");
      }
      break;
    } /* end switch */

    if (player_flags.poisoned == 0) {
      player_flags.status &= ~IS_POISONED;
      msg_print("You feel better.");
      refresh();
    }
  } /* end for */

  __tick_down(time_spent, &player_flags.blind);
  __tick_down(time_spent, &player_flags.confused);
  __tick_down(time_spent, &player_flags.protection);
  __tick_down(time_spent, &player_flags.fast);
  __tick_down(time_spent, &player_flags.slow);
  __tick_down(time_spent, &player_flags.afraid);
  __tick_down(time_spent, &player_flags.image);
  __tick_down(time_spent, &player_flags.protevil);
  __tick_down(time_spent, &player_flags.invuln);
  __tick_down(time_spent, &player_flags.hero);
  __tick_down(time_spent, &player_flags.shero);
  __tick_down(time_spent, &player_flags.blessed);
  __tick_down(time_spent, &player_flags.resist_heat);
  __tick_down(time_spent, &player_flags.resist_cold);
  __tick_down(time_spent, &player_flags.detect_inv);
  __tick_down(time_spent, &player_flags.word_recall);
  __tick_down(time_spent, &player_flags.tim_infra);
  __tick_down(time_spent, &player_flags.resist_lght);
  __tick_down(time_spent, &player_flags.free_time);
  __tick_down(time_spent, &player_flags.ring_fire);
  __tick_down(time_spent, &player_flags.protmon);
  __tick_down(time_spent, &player_flags.hoarse);
  __tick_down(time_spent, &player_flags.magic_prot);
  __tick_down(time_spent, &player_flags.ring_ice);
  __tick_down(time_spent, &player_flags.temp_stealth);
  __tick_down(time_spent, &player_flags.resist_petri);
  __tick_down(time_spent, &player_flags.blade_ring);

  switch (days_spent) {
  case 0:
  case 1:
    player_flags.foodc -= time_spent;
    if (player_flags.foodc <= PLAYER_FOOD_ALERT) {
      /* free food if you were hungry when you got here? */
      player_flags.foodc = PLAYER_FOOD_ALERT + 1;
    }
    break;

  default:
    player_flags.foodc = PLAYER_FOOD_FULL - 1;
    break;
  }

  player_flags.confuse_monster = false;

  for (long i = 1; i <= mornings; i++) {
    store_maint();
  }

  float regen_percent = regen_amount * 2 * time_spent;
  if (regen_percent > 1.00)
    regen_percent = 1.00;
  if (C_player_current_hp() < C_player_max_hp())
    C_player_regen_hp(regen_percent);
  if (C_player_current_hp() > C_player_max_hp())
    C_player_reset_current_hp();
  if (player_cmana < player_mana)
    regenmana(regen_percent);
  if (player_cmana > player_mana)
    player_cmana = player_mana;

  if (new_screen) {
    moria_flag = true;
    msg_print("");
  }
}

void display_store(enum store_t store_type, long cur_top) {
  ENTER(("display_store", ""));

  C_clear_screen();
  prt(owners[stores[store_type].owner].owner_name, 4, 10);
  prt("   Item", 5, 1);
  prt("Asking Price", 5, 61);
  __store_print_store_gold();
  __store_display_commands();
  __store_print_inventory(store_type, cur_top);
  __wizard_print_insult(store_type);

  LEAVE("display_store", "");
}
