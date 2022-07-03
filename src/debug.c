#include <stdarg.h>
#include <stdio.h>
#include <execinfo.h>

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

/* Toggleables */
boolean const do_debug_funcall = true;
boolean const do_debug_objdes = false;
boolean const do_debug_magic_cast = true;

/* Debug variables */
FILE *debug_file = NULL;
int call_depth = 0;

void dbg__enter(char const *routine_name, char const *fmt, ...) {
  va_list args;

  if (!do_debug_funcall)
    return;

  call_depth++;

  va_start(args, fmt);
  fprintf(debug_file, ":::%4d: ENTER | %s | ", call_depth, routine_name);
  vfprintf(debug_file, fmt, args);
  fprintf(debug_file, "|\n");
  va_end(args);
  fflush(debug_file);
}

void dbg__leave(char *routine_name, char *marker) {
  if (!do_debug_funcall)
    return;

  /*  fprintf(debug_file,":  Out   %ld */
  /*  %ld\n",panel_row_min,panel_row_max); */

  fprintf(debug_file, ":::%4d: LEAVE %s | %s |\n", call_depth, routine_name,
          marker);

  fflush(debug_file);

  call_depth--;
}

void dbg__return_dbg(char *routine_name, char *marker, char typestr, char *descript,
                void *valptr) {
  if (!do_debug_funcall)
    return;

  /*  fprintf(debug_file,":  Ret   %ld */
  /*  %ld\n",panel_row_min,panel_row_max); */

  switch (typestr) {

  case 'b':
    fprintf(debug_file, ":::%4d: RETUR %s | %s | %s = %s\n", call_depth,
            routine_name, marker, descript,
            (*((boolean *)valptr)) == 0 ? "false" : "true");
    break;

  case 'd':
    fprintf(debug_file, ":::%4d: RETUR %s | %s | %s = %ld\n", call_depth,
            routine_name, marker, descript, *((long *)valptr));
    break;

  case 'u':
    fprintf(debug_file, ":::%4d: RETUR %s | %s | %s = %lu\n", call_depth,
            routine_name, marker, descript, *((unsigned long *)valptr));
    break;

  case 's':
    fprintf(debug_file, ":::%4d: RETUR %s | %s | %s = %s\n", call_depth,
            routine_name, marker, descript, (char *)valptr);
    break;

  case 'y':
    fprintf(debug_file, ":::%4d: RETUR %s | %s | %s = %ld\n", call_depth,
            routine_name, marker, descript, (long)(*((signed char *)valptr)));
    break;

  default:
    fprintf(debug_file, ":::%4d: RETUR %s | %s | %s = (unknown type %c)\n",
            call_depth, routine_name, marker, descript, typestr);
    break;
  }

  fflush(debug_file);

  call_depth--;
}

void dbg__log_msg(char const *fmt, ...) {
  va_list args;
  va_start(args, fmt);
  fprintf(debug_file, ">            ");
  vfprintf(debug_file, fmt, args);
  fprintf(debug_file, "\n");
  va_end(args);
  fflush(debug_file);
}
