/* pascal.c */
/* code to help port pascal */

#include <curses.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "configure.h"
#include "constants.h"
#include "debug.h"
#include "magic.h"
#include "pascal.h"
#include "term.h"
#include "types.h"
#include "variables.h"

long pindex(char *s1, char c1) {
  /*
    pascal index function, return position of c1 in s1

    returns i if (s1[i-1] == c1), returns 0 if c1 not found
   */

  char *c2;

  c2 = strchr(s1, c1);

  if (c2 == NULL) {
    return 0;
  }

  return (c2 - s1 + 1);
}
/*//////////////////////////////////////////////////////////////////// */
boolean is_in(long const obj, obj_set const oset) {
#if DO_DEBUG
#endif
  boolean return_value = false;

#if DO_DEBUG
  long prev = 0;
  for (long i = 0; i < MAX_OBJ_SET; i++) {
    if (prev > oset[i] && oset[i] > 0) {
      printf("bad obj_set i = %ld\n\r", i);
      MSG(("bad obj_set: %d", oset[i]));
      for (i = 0; i < MAX_OBJ_SET; i++) {
        MSG(("%d  ", oset[i]));
      }
      fprintf(debug_file, "\n");
      abort();
    }
    prev = oset[i];
  }
#endif

  for (long i = 0; (i < MAX_OBJ_SET) && (oset[i] <= obj) && (!return_value);
       i++) {
    if (obj == oset[i]) {
      return_value = true;
    }
  }
  return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
boolean is_vowel(char a_char) {
  switch (a_char) {
  case 'a':
  case 'A':
  case 'e':
  case 'E':
  case 'i':
  case 'I':
  case 'o':
  case 'O':
  case 'u':
  case 'U':
    return true;
    break;

  default:
    return false;
    break;
  }
}
/*//////////////////////////////////////////////////////////////////// */

/*//////////////////////////////////////////////////////////////////// */

/* END FILE  pascal.c */
