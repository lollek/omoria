#pragma once

#include "types.h"

extern money_type bank;           /* { Bank's money	 } */

/**
 * store_maint() - Initialize and up-keep the store's inventory.
 */
void store_maint(void);

/**
 * store_carry() - Add the item in INVEN_MAX to stores inventory.
 * @store_num: Type of store
 * @opis: ???
 */
void store_carry(enum store_t store_num, long *ipos);

/**
 * item_value() - Returns the value of the item
 * @item: Item in question
 */
long item_value(treasure_type const *item);

/**
 * check_store_hours() - Check to see if a store is open, message when closed
 * @type: Store type
 * @store_visual: Same as type, or -1 for unnamed stores
 */
bool check_store_hours(enum store_t store_type, long store_visual);

/**
 * check_store_hours_and_enter() -
 * @type: Store type
 * @store_visual: Same as type, or -1 for unnamed stores
 */
void check_store_hours_and_enter(enum store_t type, long store_visual);

/**
 * spend_time()
 * @days_spent:
 * @place:
 * @whole_days:
 *
 * if not whole_days then it is actually turns...
 */
void spend_time(long days_spent, char const *place, bool whole_days);

/**
 * display_store() - Print out a store
 */
void display_store(enum store_t store_type, long cur_top);
