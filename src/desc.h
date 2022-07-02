#ifndef DESC_H
#define DESC_H

#include "boolean.h"
#include "types.h"

extern void rantitle(char *title);
extern void known1(char *object_str);
extern void known2(char *object_str);
extern void unquote(char *object_str);
extern void identify(treasure_type *item);
extern void objdes(char *out_val,
                   treas_rec *ptr,
                   boolean pref);

#endif // DESC_H