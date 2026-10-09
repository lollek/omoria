#include "terminal.h"

#include <assert.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>

#include "../../src/dungeon/light.h"
#include "../../src/messages.h"
#include "../../src/player.h"
#include "../../src/screen.h"
#include "../../src/variables.h"
#include "terminal_redirects.h"

static void exhausted_input(void) {
  headless_terminal_script("a", 1);
  assert(headless_inkey() == 'a');
  headless_inkey();
}

static void unexpected_prompt(void) {
  char command;
  headless_terminal_allow_drawing(true);
  headless_terminal_script("l", 1);
  get_com("Choose an item", &command);
}

static void unexpected_yes_no(void) {
  get_yes_no("Continue?");
}

static void unexpected_string(void) {
  char text[8];
  Get_String(text, 1, 1, sizeof(text));
}

static void unscripted_input(void) { headless_inkey(); }
static void forbidden_drawing(void) { headless_put_buffer("Oops", 1, 1); }
static void forbidden_erasure(void) { headless_erase_line(1, 1); }
static void invalid_script(void) { headless_terminal_script(NULL, 1); }
static void invalid_map_row(void) {
  headless_terminal_allow_drawing(true);
  headless_print_chstr(NULL, -1, 1);
}
static void reset_discards_script(void) {
  headless_terminal_script("unused", 6);
  headless_terminal_reset();
  headless_inkey();
}

static void assert_message_row_side_effects(void) {
  headless_terminal_reset();
  headless_terminal_allow_drawing(true);
  C_message_capture_begin();
  msg_flag = true;
  headless_prt("Message row", 0, 1);
  assert(C_message_capture_count() == 1);
  headless_clear_screen();
  assert(C_message_capture_count() == 2);
  char message[8] = "Marker";
  assert(C_message_capture_get(0, message, sizeof(message)));
  assert(message[0] == '\0');
  assert(C_message_capture_get(1, message, sizeof(message)));
  assert(message[0] == '\0');
  msg_flag = false;
  headless_prt("Message row", 0, 1);
  headless_clear_screen();
  assert(C_message_capture_count() == 2);
  C_message_capture_end();
  headless_terminal_reset();
}

static void assert_failure(void (*scenario)(void), const char *diagnostic) {
  int descriptors[2];
  assert(pipe(descriptors) == 0);
  const pid_t child = fork();
  assert(child >= 0);
  if (child == 0) {
    close(descriptors[0]);
    assert(dup2(descriptors[1], STDERR_FILENO) >= 0);
    close(descriptors[1]);
    alarm(5);
    headless_terminal_reset();
    scenario();
    _exit(EXIT_SUCCESS);
  }
  close(descriptors[1]);
  int child_status;
  assert(waitpid(child, &child_status, 0) == child);
  char output[256] = {0};
  const ssize_t length = read(descriptors[0], output, sizeof(output) - 1);
  close(descriptors[0]);
  assert(WIFEXITED(child_status));
  assert(WEXITSTATUS(child_status) == EXIT_FAILURE);
  assert(length > 0 && strstr(output, diagnostic) != NULL);
}

static void assert_rendering_leaves(void) {
  headless_terminal_reset();
  headless_terminal_allow_drawing(true);
  headless_put_buffer("Text", 1, 1);
  assert(headless_terminal_counts().drawing == 1);
  headless_put_buffer_attr("Attribute", 1, 1, A_BOLD);
  assert(headless_terminal_counts().drawing == 2);
  headless_prt("Status", 1, 1);
  assert(headless_terminal_counts().drawing == 3);
  panel_row_prt = 0;
  panel_col_prt = 0;
  used_line[2] = false;
  headless_print('@', 1, 1);
  assert(headless_terminal_counts().drawing == 4 && used_line[2]);
  used_line[2] = false;
  headless_print_str("Map", 2, 2);
  assert(headless_terminal_counts().drawing == 5 && used_line[2]);
  const chtype text[] = {'@' | A_BOLD, 0};
  used_line[2] = false;
  headless_print_chstr(text, 2, 2);
  assert(headless_terminal_counts().drawing == 6 && used_line[2]);
  headless_move_cursor(1, 1);
  assert(headless_terminal_counts().drawing == 7);
  headless_print_equipment_block();
  assert(headless_terminal_counts().drawing == 8);
  headless_print_stat_block();
  assert(headless_terminal_counts().drawing == 9);
  headless_clear_screen();
  assert(headless_terminal_counts().drawing == 10);
  headless_flush();
  assert(headless_terminal_counts().drawing == 11);
  assert(headless_refresh() == OK);
  assert(headless_terminal_counts().drawing == 12);
  headless_erase_line(1, 1);
  assert(headless_terminal_counts().erasure == 1);
  char key = 'x';
  headless_terminal_script("k", 1);
  headless_inkey_delay(&key);
  assert(key == 0 && headless_terminal_counts().input == 0);
  assert(headless_inkey() == 'k');

  panel_row_prt = 10;
  panel_col_prt = 20;
  used_line[2] = used_line[3] = false;
  Print('@', 12, 21);
  assert(!used_line[2] && used_line[3]);
  used_line[3] = false;
  print_str("Map", 13, 22);
  assert(!used_line[2] && used_line[3]);
  used_line[3] = false;
  print_chstr(text, 13, 22);
  assert(!used_line[2] && used_line[3]);
  panel_row_prt = panel_col_prt = 0;
}

static void assert_real_screen_and_blind_lighting(void) {
  headless_terminal_reset();
  headless_terminal_allow_drawing(true);
  memset(&player_flags, 0, sizeof(player_flags));
  prt_equipment();
  assert(headless_terminal_counts().drawing == 1);
  prt_stat_block();
  assert(headless_terminal_counts().drawing == 11);
  panel_row_min = panel_row_max = 2;
  panel_col_min = panel_col_max = 2;
  memset(&cave[2][2], 0, sizeof(cave[2][2]));
  cave[2][2].cptr = 1;
  used_line[2] = true;
  redraw = true;
  prt_map();
  assert(!redraw && used_line[2]);
  assert(headless_terminal_counts().drawing == 12);
  assert(headless_terminal_counts().erasure == 1);
  player_flags.blind = 2;
  dungeon_light_move(2, 2, 2, 3);
  assert(headless_terminal_counts().drawing == 14);
  draw_cave();
  assert(headless_terminal_counts().drawing == 29);
}

void assert_headless_terminal(void) {
  const p_flags saved_flags = player_flags;
  const long saved_panels[] = {panel_row_min, panel_row_max, panel_col_min,
                             panel_col_max, panel_row_prt, panel_col_prt};
  bool saved_lines[sizeof(used_line) / sizeof(used_line[0])];
  unsigned char saved_cell[sizeof(cave[2][2])];
  char saved_message[sizeof(last_printed_message)];
  const bool saved_msg_flag = msg_flag;
  const bool saved_redraw = redraw;
  const bool saved_find_flag = find_flag;
  const int saved_command_count = command_count;
  memcpy(saved_lines, used_line, sizeof(used_line));
  memcpy(saved_cell, &cave[2][2], sizeof(saved_cell));
  memcpy(saved_message, last_printed_message, sizeof(saved_message));

  headless_terminal_reset();
  headless_terminal_script("ab", 2);
  msg_flag = true;
  command_count = 3;
  assert(headless_inkey() == 'a');
  assert(!msg_flag && command_count == 0);
  assert(headless_inkey() == 'b');
  assert(headless_terminal_counts().input == 2);
  headless_terminal_reset();
  assert(headless_terminal_counts().input == 0);
  assert_failure(exhausted_input, "scripted input exhausted");
  assert_failure(unscripted_input, "unexpected prompt");
  assert_failure(unexpected_prompt, "Unexpected headless prompt");
  assert_failure(unexpected_yes_no, "Unexpected headless prompt");
  assert_failure(unexpected_string, "Unexpected headless prompt");
  assert_failure(forbidden_drawing, "Unexpected headless terminal drawing");
  assert_failure(forbidden_erasure, "Unexpected headless terminal erasure");
  assert_failure(invalid_script, "Invalid headless terminal script");
  assert_failure(invalid_map_row, "Invalid headless map drawing row");
  assert_failure(reset_discards_script, "scripted input exhausted");
  for (int repeat = 0; repeat < 2; repeat++) {
    msg_flag = false;
    find_flag = false;
    assert_rendering_leaves();
    assert_real_screen_and_blind_lighting();
    assert_message_row_side_effects();
  }
  headless_terminal_reset();
  player_flags = saved_flags;
  panel_row_min = saved_panels[0];
  panel_row_max = saved_panels[1];
  panel_col_min = saved_panels[2];
  panel_col_max = saved_panels[3];
  panel_row_prt = saved_panels[4];
  panel_col_prt = saved_panels[5];
  memcpy(used_line, saved_lines, sizeof(used_line));
  memcpy(&cave[2][2], saved_cell, sizeof(saved_cell));
  memcpy(last_printed_message, saved_message, sizeof(saved_message));
  msg_flag = saved_msg_flag;
  redraw = saved_redraw;
  find_flag = saved_find_flag;
  command_count = saved_command_count;
  puts("Headless terminal support checks passed");
}