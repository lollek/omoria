#pragma once

#include "types.h"

void desc_remain(const treas_rec *item_ptr);
void desc_charges(const treas_rec *item_ptr);
void rantitle(char *title);
void known1(const char *object_str);
void known2(const char *object_str);
void unquote(char *object_str);
void identify(treasure_type *item);
void objdes(char *out_val, const treas_rec *ptr, bool pref);
char *bag_descrip(const treas_rec *bag, char result[134]);
