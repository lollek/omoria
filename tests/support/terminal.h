#pragma once

#include <stdbool.h>
#include <stddef.h>

#include "../../src/io.h"
#include "../../src/term.h"

struct headless_terminal_counts {
  size_t input;
  size_t drawing;
  size_t erasure;
};

void headless_terminal_reset(void);
void headless_terminal_allow_drawing(bool allow);
void headless_terminal_script(const char *keys, size_t count);
struct headless_terminal_counts headless_terminal_counts(void);
char headless_inkey(void);
void headless_put_buffer(const char *message, int row, int col);
void headless_erase_line(long row, long col);
void headless_put_buffer_attr(const char *message, long row, long col, int attrs);
void headless_prt(const char *message, int row, int col);
void headless_print(chtype ch, int row, int col);
void headless_print_str(const char *message, int row, int col);
void headless_print_chstr(const chtype *message, int row, int col);
void headless_move_cursor(int row, int col);
void headless_print_equipment_block(void);
void headless_print_stat_block(void);
void headless_clear_screen(void);
void headless_flush(void);
int headless_refresh(void);
void headless_inkey_delay(char *key);
bool headless_get_com(const char *prompt, char *command);
bool headless_get_yes_no(const char *prompt);
bool headless_get_string(char *text, int row, int col, int length);