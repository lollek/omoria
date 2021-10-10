#ifndef INIT_ARGV_H
#define INIT_ARGV_H

#include <stdbool.h>

/**
 * init__argv() - Initialize argv data
 *
 * @return - true on success, false of failure.
 * Will print information to stderr on failure
 */
bool init__argv(int argc, char *argv[]);

#endif /* INIT_ARGV_H */
