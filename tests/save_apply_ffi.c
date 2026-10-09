#include <assert.h>
#include <stdbool.h>
#include <stdio.h>

bool C_save_test_reset(void);
bool C_save_test_reject_mismatched_uid_preserves_state(void);
bool C_save_test_apply_fixture_and_verify(void);

static void assert_fixture_applies_and_reads_back(void) {
  assert(C_save_test_reset());
  assert(C_save_test_reject_mismatched_uid_preserves_state());
  assert(C_save_test_apply_fixture_and_verify());
}

int main(void) {
  for (int repeat = 0; repeat < 2; repeat++) {
    assert_fixture_applies_and_reads_back();
  }
  puts("Save apply C boundary checks passed.");
  return 0;
}