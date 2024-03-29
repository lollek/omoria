#ifndef TREASURES_H
#define TREASURES_H

#include "constants.h"
#include "boolean.h"

//  Descriptive constants
#define MAX_SYLLABLES 156 // Used with scrolls

extern char const *syllables[MAX_SYLLABLES];
extern long t_level[MAX_OBJ_LEVEL + 1];

void magic_treasure(long x, long level, boolean forceit);

#endif // TREASURES_H
