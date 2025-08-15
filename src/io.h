#pragma once

#include <curses.h>


void override_signals(void);
void stop_override_signals(void);
void exit_ncurses(void);
void exit_game(void) __attribute__((noreturn));
/* extern void inkey(char *getchar); */
char inkey(void);
void inkey_delay(char *getchar);
void flush(void);
void inkey_flush(char *x);
#define erase_line(r, c) Erase_Line((r)-1, (c)-1)
void Erase_Line(long row, long col);
void clear_rc(long row, long col);
#define print(s, r, c) Print((s), (r)-1, (c)-1)
void Print(chtype ch, int row, int col);
void print_str(char const *str_buff, int row, int col);
void print_chstr(chtype const *str_buff, int row, int col);


/* use prt */
void prt(char const *str_buff, int row, int col);
void prt_(char const *str_buff, int row, int col);
void prt2(char *str_buff1, /*	: varying[a] of char; */
                 char *str_buff2, int row, int col);

void msg_printf(char const *fmt, ...);
bool msg_print(const char *str_buff); /* : varying[a] of char); */
bool get_com(char const *prompt, char *command);

/**
 *  get_yes_no() - Ask a yes/no question
 *  @prompt: Question to write to player
 *
 *  Ask a yes/no question to the player, which adds a question to the end of the
 *  prompt. Will not return until player pressed 'y' or 'n'.
 */
bool get_yes_no(char const *prompt);

#define get_string(s, r, c, l) Get_String((s), (r)-1, (c)-1, (l))
bool Get_String(char *in_str, /* : varying[a] of char; */
                          int row, int column, int slen);
long get_hex_value(long row, long col, long slen);
void print_hex_value(long num, long row, long col);
void pause_game(long prt_line);
/* use pause_exit */
