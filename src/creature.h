#ifndef CREATURE_H
#define CREATURE_H

#include "boolean.h"

/* Returns number of monster in list specified by virtual_name */
long find_mon(const char *virtual_name);

/* Makes sure new creature gets lit up */
void check_mon_lite(long y, long x);

/* Places creature adjacent to given location
 * Rats and Flies are fun!
 */
void multiply_monster(long y, long x, long z, boolean slp);

void creatures(boolean attack);

/* Moves creature record from one space to another.
 *
 * NOTE! (x1,y1) might equal (x2,y2)
 */
void move_rec(long y1, long x1, long y2, long x2);

#endif /* CREATURE_H */