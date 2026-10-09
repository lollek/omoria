#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "../src/io.h"
#include "../src/messages.h"
#include "../src/variables.h"

bool msg_print_pass_one(char *message);

static bool allow_terminal = false;
static size_t input_calls = 0;
static size_t drawing_calls = 0;
static size_t erasure_calls = 0;

char message_test_inkey(void) {
  if (!allow_terminal) {
    fputs("Unexpected interactive message input\n", stderr);
    abort();
  }
  input_calls++;
  return 27;
}

void message_test_put_buffer(const char *message, int row, int col) {
  (void)message;
  (void)row;
  (void)col;
  if (!allow_terminal) {
    fputs("Unexpected message drawing\n", stderr);
    abort();
  }
  drawing_calls++;
}

void message_test_erase_line(long row, long col) {
  (void)row;
  (void)col;
  if (!allow_terminal) {
    fputs("Unexpected message erasure\n", stderr);
    abort();
  }
  erasure_calls++;
}

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
  input_calls = 0;
  drawing_calls = 0;
  erasure_calls = 0;
  allow_terminal = true;
  assert(!msg_print("First interactive"));
  assert(msg_print("Second interactive"));
  assert(msg_print_pass_one("Pass one interactive"));
  assert(input_calls == 2 && drawing_calls == 5 && erasure_calls == 3);
  assert(msg_flag && strcmp(last_printed_message, "Pass one interactive") == 0);
  allow_terminal = false;
}

int main(void) {
  for (int repeat = 0; repeat < 2; repeat++) {
    assert_captured_messages_skip_terminal();
    assert_nested_captures_restore_outer();
    assert_capture_copy_boundaries();
    assert_last_drop_restores_interactive_path();
  }
  puts("Message C boundary checks passed");
  return 0;
}