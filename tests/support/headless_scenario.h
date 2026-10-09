#pragma once

#include <stdbool.h>
#include <stddef.h>

struct headless_scenario_result {
  long start_row;
  long start_col;
  long row;
  long col;
  long turn;
  long turn_counter;
  unsigned char origin_occupant;
  unsigned char destination_occupant;
  size_t command_calls;
  size_t commands_consumed;
  size_t message_count;
  size_t drawing_count;
  size_t input_count;
};

void headless_scenario_run(const char *commands, size_t command_count,
                           bool block_west, struct headless_scenario_result *result);