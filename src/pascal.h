#ifndef PASCAL_H
#define PASCAL_H

#include "types.h"

/* Things to save typing when converting pascal to c */

long pindex(char *s1, char c1);
boolean is_in(long const obj, obj_set const oset);
boolean is_vowel(char a_char);

#endif /* PASCAL_H */
