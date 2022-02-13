#ifndef TREASURES_H
#define TREASURES_H

#include "constants.h"
#include "types.h"

extern long t_level[MAX_OBJ_LEVEL + 1];

extern void magic_treasure(long x, long level, boolean forceit);

#endif // TREASURES_H
