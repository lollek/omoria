#pragma once

#include "types.h"

/*{ Describe amount of item remaining...                  -RAK-   }*/
void msg_remaining_of_item(const treas_rec *item_ptr);

/*{ Describe number of remaining charges...               -RAK-   }*/
void msg_charges_remaining(const treas_rec *item_ptr);

/*{ Remove 'Secret' symbol for identity of objects * }*/
void known1(const char *object_str);

/*{ Remove 'Secret' symbol for identity of pluses * }*/
void known2(const char *object_str);

/*	{ Return string without quoted portion }*/
void unquote(char *object_str);

/*{ Something has been identified }*/
void identify(treasure_type *item);

/**
 * objdes() - Returns a description of item for inventory
 * @out_val: Where to put the return string
 * @ptr: Pointer to the object to describe
 * @pref: ???
 */
void objdes(char *out_val, const treas_rec *ptr, bool pref);

/*{ Return description about the contents of a bag	-DMF-	}*/
char *bag_descrip(const treas_rec *bag, char result[134]);
