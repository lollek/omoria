#pragma once

#include <stdbool.h>

/**
 * init__argv() - Initialize argv data
 *
 * @return - true on success, false of failure.
 * Will print information to stderr on failure
 */
bool init__argv(int argc, char *argv[]);
