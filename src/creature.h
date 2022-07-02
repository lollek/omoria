#ifndef CREATURE_H
#define CREATURE_H

long find_mon(const char *virtual_name);
void check_mon_lite(long y, long x);
void multiply_monster(long y, long x, long z, boolean slp);
void creatures(boolean attack);

#endif /* CREATURE_H */