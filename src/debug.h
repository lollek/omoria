#ifndef DEBUG_H
#define DEBUG_H

#include <stdio.h>

#include "boolean.h"

#if DO_DEBUG
extern boolean const do_debug_funcall;
extern boolean const do_debug_objdes;
extern boolean const do_debug_magic_cast;

#define ENTER(args) debug_enter args;
#define LEAVE(rname, mark) debug_leave((rname), (mark));
#define RETURN(rname, mark, typestr, desc, valptr)                             \
  debug_return_dbg((rname), (mark), (typestr), (desc), (valptr));
#define MSG(args) debug_log_msg args

extern FILE *debug_file;
extern int call_depth;

#else /* !DO_DEBUG */

#define ENTER(args) do {} while (0)
#define LEAVE(rname, mark) do {} while (0)
#define RETURN(rname, mark, typestr, desc, valptr) do {} while (0)
#define MSG(str) do {} while (0)

#endif /* DO_DEBUG */

void debug_init(void);
void debug_enter(char const *routine_name, char const *fmt, ...);
void debug_leave(char *routine_name, char *marker);
void debug_return_dbg(char *routine_name, char *marker, char typestr, char *descript,
                void *valptr);
void debug_log_msg(char const *fmt, ...);

#endif /* DEBUG_H */
