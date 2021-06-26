#ifndef DEBUG_H
#define DEBUG_H

#include <stdio.h>

#include "boolean.h"

#if DO_DEBUG
extern boolean const do_debug_funcall;
extern boolean const do_debug_objdes;
extern boolean const do_debug_magic_cast;

#define ENTER(args) enter args;
#define LEAVE(rname, mark) leave((rname), (mark));
#define RETURN(rname, mark, typestr, desc, valptr)                             \
  return_dbg((rname), (mark), (typestr), (desc), (valptr));
#define MSG(args) log_msg args

extern FILE *debug_file;
extern int call_depth;

#else /* !DO_DEBUG */

#define ENTER(args)
#define LEAVE(rname, mark)
#define RETURN(rname, mark, typestr, desc, valptr)
#define MSG(str)

#endif /* DO_DEBUG */

void init_debug(void);
void enter(char const *routine_name, char const *fmt, ...);
void leave(char *routine_name, char *marker);
void return_dbg(char *routine_name, char *marker, char typestr, char *descript,
                void *valptr);
void log_msg(char const *fmt, ...);

#endif /* DEBUG_H */
