#ifndef DEBUG_H
#define DEBUG_H

#if DO_DEBUG
extern boolean const do_debug_objdes;
extern boolean const do_debug_funcall;

#define ENTER(rname, mark) enter((rname), (mark));
#define LEAVE(rname, mark) leave((rname), (mark));
#define RETURN(rname, mark, typestr, desc, valptr)                             \
	return_dbg((rname), (mark), (typestr), (desc), (valptr));
#define MSG(str) log_msg(str);

extern FILE *debug_file;
extern int call_depth;

#else /* !DO_DEBUG */

#define ENTER(rname, mark)
#define LEAVE(rname, mark)
#define RETURN(rname, mark, typestr, desc, valptr)
#define MSG(str)

#endif /* DO_DEBUG */

void init_debug(void);
void enter(char *routine_name, char *marker);
void leave(char *routine_name, char *marker);
void return_dbg(char *routine_name, char *marker, char typestr,
		       char *descript, void *valptr);
void log_msg(char *str);

#endif /* DEBUG_H */
