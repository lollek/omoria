#ifndef TERM_H
#define TERM_H

#include <stdio.h>

typedef unsigned char int8u;

void moriaterm();
void bell();
void C_clear_screen();

/* Dungeon size parameters					*/
#define MAX_HEIGHT 66 /* Multiple of 11; >= 22 */
#define MAX_WIDTH 198 /* Multiple of 33; >= 66 */

#undef CTRL_
#define CTRL_(x) (x & 0x1F)
#define DELETE 0x7f
#define ESCAPE '\033' /* ESCAPE character -CJS-  */

/* message line location */
#define MSG_LINE -1

#define CNIL (char *)0

/* number of messages to save in a buffer */
#define MAX_SAVE_MSG 22 /* How many messages to save -CJS- */

extern int command_count;
extern int eof_flag;
extern int character_generated, character_saved;
extern int sound_beep_flag;
extern int panic_save;
extern FILE *highscore_fp;

/* end file term.h */
#endif /* TERM_H */
