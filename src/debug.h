#pragma once

#include <stdbool.h>
#include <stdio.h>

#if DO_DEBUG
extern bool const do_debug_funcall;
extern bool const do_debug_objdes;
extern bool const do_debug_magic_cast;

#define ENTER(args) dbg__enter args;
#define LEAVE(rname, mark) dbg__leave((rname), (mark));
#define RETURN(rname, mark, typestr, desc, valptr)                             \
  dbg__return_dbg((rname), (mark), (typestr), (desc), (valptr));
#define MSG(args) dbg__log_msg args

extern int call_depth;

#else /* !DO_DEBUG */

#define ENTER(args) do {} while (0)
#define LEAVE(rname, mark) do {} while (0)
#define RETURN(rname, mark, typestr, desc, valptr) do {} while (0)
#define MSG(str) do {} while (0)

#endif /* DO_DEBUG */

extern FILE *debug_file;

void dbg__enter(char const *routine_name, char const *fmt, ...);
void dbg__leave(char *routine_name, char *marker);
void dbg__return_dbg(char *routine_name, char *marker, char typestr, char *descript,
                void *valptr);
void dbg__log_msg(char const *fmt, ...);
