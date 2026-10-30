#include "terminal.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../../src/variables.h"
#include "../../src/messages.h"

static bool drawing_allowed;
static struct headless_terminal_counts counts;
static struct headless_attributed_draw attributed_draw;
static const char *script;
static size_t script_count;
static size_t script_position;
static bool key_pending;
static const bool *poll_script;
static size_t poll_script_count;
static size_t poll_script_position;

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
  attributed_draw = (struct headless_attributed_draw){0};
  script = NULL;
  script_count = 0;
  script_position = 0;
  key_pending = false;
  poll_script = NULL;
  poll_script_count = 0;
  poll_script_position = 0;
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

void headless_terminal_poll_script(const bool *pending, size_t count) {
  if (!pending && count != 0) {
    fail("Invalid headless terminal poll script");
  }
  poll_script = pending;
  poll_script_count = count;
  poll_script_position = 0;
}

struct headless_terminal_counts headless_terminal_counts(void) {
  return counts;
}

struct headless_attributed_draw headless_terminal_last_attributed_draw(void) {
  return attributed_draw;
}

char headless_inkey(void) {
  if (script_position == script_count) {
    fail("Headless terminal scripted input exhausted (unexpected prompt)");
  }
  counts.input++;
  C_message_log_input();
  command_count = 0;
  msg_flag = false;
  return script[script_position++];
}

int headless_terminal_poll_key(void) {
  if (poll_script_position < poll_script_count)
    return poll_script[poll_script_position++];
  const bool pending = key_pending;
  key_pending = false;
  return pending;
}

void headless_terminal_set_key_pending(bool pending) { key_pending = pending; }

void headless_put_buffer(const char *message, int row, int col) {
  (void)row;
  (void)col;
  record_drawing();
  if (strcmp(message, " -more-") == 0)
    counts.more++;
}

void headless_erase_line(long row, long col) {
  (void)row;
  (void)col;
  if (!drawing_allowed)
    fail("Unexpected headless terminal erasure");
  counts.erasure++;
}

void headless_put_buffer_attr(const char *message, long row, long col, int attrs) {
  record_drawing();
  snprintf(attributed_draw.text, sizeof(attributed_draw.text), "%s", message);
  attributed_draw.row = row;
  attributed_draw.col = col;
  attributed_draw.attrs = attrs;
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

int headless_mvhline(int row, int col, chtype ch, int width) {
  (void)row;
  (void)col;
  (void)ch;
  (void)width;
  if (!drawing_allowed)
    fail("Unexpected headless terminal erasure");
  counts.erasure++;
  return 0;
}

void headless_move_cursor(int row, int col) {
  (void)row;
  (void)col;
  record_drawing();
}

void headless_print_equipment_block(void) { record_drawing(); }
void headless_print_stat_block(void) { record_drawing(); }
void headless_clear_screen(void) {
  counts.screen_clears++;
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