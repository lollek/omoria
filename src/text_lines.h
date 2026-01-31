#pragma once

#include "types.h"

/*{ Describe amount of item remaining...                  -RAK-   }*/
void msg_remaining_of_item(const treas_rec *item_ptr);

/*{ Describe number of remaining charges...               -RAK-   }*/
void msg_charges_remaining(const treas_rec *item_ptr);

/*{ Remove 'Secret' symbol for identity of objects * }*/
void known1(char *object_str);

/*{ Remove 'Secret' symbol for identity of pluses * }*/
void known2(char *object_str);

/*	{ Return string without quoted portion }*/
void unquote(char *object_str);

/*{ Something has been identified }*/
void identify(treasure_type *item);

/**
 * item_name() - Returns a description of item for inventory (new item names)
 * @out_val: Where to put the return string
 * @ptr: Pointer to the object to describe
 */
void item_name(char out_val[82], const treas_rec *ptr);

/*{ Return description about the contents of a bag	-DMF-	}*/
char *bag_descrip(const treas_rec *bag, char result[134]);
