#pragma once

#include "../constants.h"

#include <stdbool.h>

//  Descriptive constants
#define MAX_SYLLABLES 156 // Used with scrolls

extern char const *syllables[MAX_SYLLABLES];
extern long t_level[MAX_OBJ_LEVEL + 1];

void magic_treasure(long x, long level, bool forceit);
