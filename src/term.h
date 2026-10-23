#pragma once

#include <stdio.h>

typedef unsigned char int8u;

void moriaterm(void);
void bell(void);
void C_clear_screen(void);
void put_buffer(char const *str_buff, int row, int col);
void put_buffer_(char const *str_buff, int32_t row, int32_t col);
void put_buffer_attr(const char *out_str, long row, long col, int attrs);
#define clear_from(r) Clear_From((r)-1)
void Clear_From(int row);
#define pause_line(r) Pause_Line((r)-1)
void Pause_Line(int prt_line);
#define print_null(r, c)                                                       \
  (move_cursor((r)-panel_row_prt - 1, (c)-panel_col_prt - 1))
void move_cursor(int row, int col);

/* Dungeon size parameters					*/
#define MAX_HEIGHT 66 /* Multiple of 11; >= 22 */
#define MAX_WIDTH 198 /* Multiple of 33; >= 66 */

#undef CTRL_
#define CTRL_(x) (x & 0x1F)
#define DELETE 0x7f
#define ESCAPE '\033' /* ESCAPE character -CJS-  */
#define OMORIA_KEY_HOME ((char)0xf0)
#define OMORIA_KEY_END ((char)0xf1)
#define OMORIA_KEY_PAGE_UP ((char)0xf2)
#define OMORIA_KEY_PAGE_DOWN ((char)0xf3)

int C_translate_key_code(int key_code);
#define C_MOUSE_INPUT (-2)
int C_translate_terminal_input_code(int key_code);
void C_mouse_event_push(int row, int col, unsigned long state);
struct mouse_event {
  int row;
  int col;
  int kind;
  int button;
};
struct mouse_event C_mouse_event_pop(void);
void C_mouse_dispatch_pending_events(long panel_row, long panel_col,
                                     long world_height, long world_width);

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
