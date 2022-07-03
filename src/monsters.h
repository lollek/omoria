#ifndef MONSTERS_H
#define MONSTERS_H

#include "types.h"

void place_monster(long y, long x, long z, boolean slp);
void alloc_land_monster(obj_set alloc_set, long num, long dis, boolean slp,
                        boolean water);

#endif // MONSTERS_H