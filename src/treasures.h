#ifndef TREASURES_H
#define TREASURES_H

#include "constants.h"

//  Descriptive constants
#define MAX_COLORS 68     // Used with potions
#define MAX_MUSH 29       // Used with mushrooms
#define MAX_WOODS 41      // Used with staffs
#define MAX_METALS 32     // Used with wands
#define MAX_HORNS 13      // Used with horns
#define MAX_ROCKS 53      // Used with rings
#define MAX_CLOTHS 7      // Used with bags/sacks
#define MAX_AMULETS 39    // Used with amulets
#define MAX_SYLLABLES 156 // Used with scrolls

extern char const *colors[MAX_COLORS];
extern char const *mushrooms[MAX_MUSH];
extern char const *woods[MAX_WOODS];
extern char const *metals[MAX_METALS];
extern char const *horns[MAX_HORNS];
extern char const *rocks[MAX_ROCKS];
extern char const *amulets[MAX_AMULETS];
extern char const *cloths[MAX_CLOTHS];
extern char const *syllables[MAX_SYLLABLES];

extern long t_level[MAX_OBJ_LEVEL + 1];

#endif // TREASURES_H
