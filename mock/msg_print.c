#include <stdarg.h>
#include <stddef.h>
#include <stdint.h>

// Test-only hook: capture the last msg_print() output.
// This file is only compiled into Rust unit tests via build.rs.

static char LAST_MSG_PRINT[512];

const char *test_last_msg_print(void) { return LAST_MSG_PRINT; }
void test_clear_last_msg_print(void) { LAST_MSG_PRINT[0] = '\0'; }

// Override the C symbol used by Rust's term::msg_print -> C_msg_print
void msg_print(const char *str_buff) {
  if (!str_buff) {
    LAST_MSG_PRINT[0] = '\0';
    return;
  }

  // Simple bounded copy.
  size_t i = 0;
  for (; i + 1 < sizeof(LAST_MSG_PRINT) && str_buff[i] != '\0'; i++) {
    LAST_MSG_PRINT[i] = str_buff[i];
  }
  LAST_MSG_PRINT[i] = '\0';
}
