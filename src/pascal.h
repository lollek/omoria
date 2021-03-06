#ifndef PASCAL_H
#define PASCAL_H

/* Things to save typing when converting pascal to c */

#define nil 0
#define false 0
#define true 1

#define trunc(xx) (xx)

#define uor(xx, yy) ((xx) | (yy))
#define uand(xx, yy) ((xx) & (yy))
#define uxor(xx, yy) ((xx) ^ (yy))

void writeln(char *out_line);
long pindex(char *s1, char c1);
boolean is_in(long const obj, obj_set const oset);
boolean is_vowel(char a_char);

#endif /* PASCAL_H */
