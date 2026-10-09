#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../src/io.h"
#include "../src/messages.h"
#include "../src/variables.h"
#include "support/terminal.h"

bool msg_print_pass_one(char *message);

void assert_headless_terminal(void);

static void assert_message(size_t index, const char *expected) {
  char buffer[256];
  assert(C_message_capture_get(index, buffer, sizeof(buffer)));
  assert(strcmp(buffer, expected) == 0);
}

static void assert_captured_messages_skip_terminal(void) {
  assert(!message_capture_active());
  msg_flag = false;
  msg_terse = false;
  last_printed_message[0] = '\0';
  C_message_capture_begin();
  assert(message_capture_active());
  assert(C_message_capture_count() == 0);
  assert(!msg_print("First"));
  assert(!msg_print("Second"));
  assert(C_message_capture_count() == 2);
  assert_message(0, "First");
  assert_message(1, "Second");
  assert(msg_flag && strcmp(last_printed_message, "Second") == 0);
  assert(!msg_print(""));
  assert_message(2, "");
  assert(msg_flag);
  assert(!msg_print_pass_one("Pass one"));
  assert_message(3, "Pass one");
  assert(!msg_print_pass_one(""));
  assert(!msg_flag && C_message_capture_count() == 4);
  assert(!msg_print_pass_one(NULL));
  assert(!msg_flag && C_message_capture_count() == 4);
  assert(!msg_print_pass_one(" "));
  assert_message(4, " ");
  assert(msg_flag);
  C_message_capture_end();
  assert(!message_capture_active());
  assert(C_message_capture_count() == 0);
}

static void assert_nested_captures_restore_outer(void) {
  C_message_capture_begin();
  assert(!msg_print("Outer"));
  C_message_capture_begin();
  assert(C_message_capture_count() == 0);
  assert(!msg_print("Inner"));
  assert_message(0, "Inner");
  C_message_capture_end();
  assert(message_capture_active());
  assert_message(0, "Outer");
  assert(!msg_print("Restored"));
  assert_message(1, "Restored");
  C_message_capture_end();
  assert(!message_capture_active());
}

static void assert_capture_copy_boundaries(void) {
  char buffer[8] = "Marker";
  assert(!C_message_capture_get(0, buffer, sizeof(buffer)));
  C_message_capture_end();
  C_message_capture_begin();
  assert(!C_message_capture_get(0, buffer, sizeof(buffer)));
  assert(!msg_print("Long message"));
  assert(!C_message_capture_get(1, buffer, sizeof(buffer)));
  assert(!C_message_capture_get(0, NULL, sizeof(buffer)));
  assert(!C_message_capture_get(0, buffer, 0));
  assert(strcmp(buffer, "Marker") == 0);
  assert(C_message_capture_get(0, buffer, 1));
  assert(buffer[0] == '\0' && buffer[1] == 'a');
  assert(C_message_capture_get(0, buffer, 5));
  assert(strcmp(buffer, "Long") == 0);
  C_message_capture_end();
}

static void assert_last_drop_restores_interactive_path(void) {
  assert(!message_capture_active());
  msg_flag = false;
  msg_terse = false;
  headless_terminal_reset();
  headless_terminal_allow_drawing(true);
  headless_terminal_script("\033\033", 2);
  assert(!msg_print("First interactive"));
  assert(msg_print("Second interactive"));
  assert(msg_print_pass_one("Pass one interactive"));
  const struct headless_terminal_counts counts = headless_terminal_counts();
  assert(counts.input == 2 && counts.drawing == 5 && counts.erasure == 3);
  assert(msg_flag && strcmp(last_printed_message, "Pass one interactive") == 0);
  headless_terminal_reset();
}

int main(void) {
  assert_headless_terminal();
  for (int repeat = 0; repeat < 2; repeat++) {
    assert_captured_messages_skip_terminal();
    assert_nested_captures_restore_outer();
    assert_capture_copy_boundaries();
    assert_last_drop_restores_interactive_path();
  }
  puts("Message C boundary checks passed");
  return 0;
}