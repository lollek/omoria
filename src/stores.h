#ifndef STORE_H
#define STORE_H

#include "types.h"

/**
 * store_init() - Initializes the stores with owners
 */
void store_init();

/**
 * bank_init() - Initialize the bank
 */
void bank_init();

/**
 * store_maint() - Initialize and up-keep the store's inventory.
 */
void store_maint();

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
long item_value(treasure_type const *const item);

/**
 * check_store_hours() - Check to see if a store is open, message when closed
 * @type: Store type
 * @store_visual: Same as type, or -1 for unnamed stores
 */
boolean check_store_hours(enum store_t type, long store_visual);

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
void spend_time(long days_spent, char place[82], boolean whole_days);

/**
 * display_store() - Print out a store
 */
void display_store(enum store_t store_type, long cur_top);

#endif /* STORE_H */
