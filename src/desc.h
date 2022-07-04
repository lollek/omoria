#ifndef DESC_H
#define DESC_H

#include "boolean.h"
#include "types.h"

void desc_remain(treas_rec *item_ptr);
void desc_charges(treas_rec *item_ptr);
void rantitle(char *title);
void known1(char *object_str);
void known2(char *object_str);
void unquote(char *object_str);
void identify(treasure_type *item);
void objdes(char *out_val, treas_rec *ptr, boolean pref);
char *bag_descrip(treas_rec *bag, char result[134]);

#endif // DESC_H