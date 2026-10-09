#include "terminal.h"

#include <stdio.h>
#include <stdlib.h>

#include "../../src/variables.h"

static bool drawing_allowed;
static struct headless_terminal_counts counts;
static const char *script;
static size_t script_count;
static size_t script_position;

static void fail(const char *diagnostic) __attribute__((noreturn));
static void fail(const char *diagnostic) {
  fprintf(stderr, "%s\n", diagnostic);
  exit(EXIT_FAILURE);
}

static void record_drawing(void) {
  if (!drawing_allowed)
    fail("Unexpected headless terminal drawing");
  counts.drawing++;
}

static void mark_line(int row) {
  if (row < 0 || (size_t)row >= sizeof(used_line) / sizeof(used_line[0]))
    fail("Invalid headless map drawing row");
  used_line[row] = true;
}

void headless_terminal_reset(void) {
  drawing_allowed = false;
  counts = (struct headless_terminal_counts){0};
  script = NULL;
  script_count = 0;
  script_position = 0;
}

void headless_terminal_allow_drawing(bool allow) {
  drawing_allowed = allow;
}

void headless_terminal_script(const char *keys, size_t count) {
  if (!keys && count != 0) {
    fail("Invalid headless terminal script");
  }
  script = keys;
  script_count = count;
  script_position = 0;
}

struct headless_terminal_counts headless_terminal_counts(void) {
  return counts;
}

char headless_inkey(void) {
  if (script_position == script_count) {
    fail("Headless terminal scripted input exhausted (unexpected prompt)");
  }
  counts.input++;
  command_count = 0;
  msg_flag = false;
  return script[script_position++];
}

void headless_put_buffer(const char *message, int row, int col) {
  (void)message;
  (void)row;
  (void)col;
  record_drawing();
}

void headless_erase_line(long row, long col) {
  (void)row;
  (void)col;
  if (!drawing_allowed)
    fail("Unexpected headless terminal erasure");
  counts.erasure++;
}

void headless_put_buffer_attr(const char *message, long row, long col, int attrs) {
  (void)message;
  (void)row;
  (void)col;
  (void)attrs;
  record_drawing();
}

void headless_prt(const char *message, int row, int col) {
  headless_put_buffer(message, row, col);
  if (row == 0 && msg_flag)
    msg_print("");
}

void headless_print(chtype ch, int row, int col) {
  (void)ch;
  (void)col;
  record_drawing();
  mark_line(row - panel_row_prt + 1);
}

void headless_print_str(const char *message, int row, int col) {
  headless_put_buffer(message, row, col);
  mark_line(row - panel_row_prt);
}

void headless_print_chstr(const chtype *message, int row, int col) {
  (void)message;
  (void)col;
  record_drawing();
  mark_line(row - panel_row_prt);
}

void headless_move_cursor(int row, int col) {
  (void)row;
  (void)col;
  record_drawing();
}

void headless_print_equipment_block(void) { record_drawing(); }
void headless_print_stat_block(void) { record_drawing(); }
void headless_clear_screen(void) {
  record_drawing();
  if (msg_flag)
    msg_print("");
}
void headless_flush(void) { record_drawing(); }
int headless_refresh(void) {
  record_drawing();
  return OK;
}
void headless_inkey_delay(char *key) {
  headless_refresh();
  *key = 0;
}
bool headless_get_com(const char *prompt, char *command) {
  (void)command;
  fprintf(stderr, "Unexpected headless prompt: %s\n", prompt);
  exit(EXIT_FAILURE);
}
bool headless_get_yes_no(const char *prompt) {
  fprintf(stderr, "Unexpected headless prompt: %s\n", prompt);
  exit(EXIT_FAILURE);
}
bool headless_get_string(char *text, int row, int col, int length) {
  (void)text;
  (void)row;
  (void)col;
  (void)length;
  fail("Unexpected headless prompt: string input");
}