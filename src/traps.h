#ifndef TRAPS_H
#define TRAPS_H

void change_trap(long y, long x);
void hit_trap(long *y, long *x);
void place_trap(long y, long x, long typ, long subval);
void place_rubble(long y, long x);

#endif // TRAPS_H