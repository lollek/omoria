#ifndef INIT_DEBUG_H
#define INIT_DEBUG_H

#include <stdbool.h>

/**
 * init__debug() - Initialize debug data
 *
 * @return - true on success, false of failure.
 * Will print information to stderr on failure
 */
bool init__debug(void);

#endif /* INIT_DEBUG_H */
