#ifndef HORSE_H
#define HORSE_H
/* declarations for the horse racing module in the casino */

#define RACES_PER_DAY 10
#define MAX_HORSE_NAMES 55

typedef long h_stat[MAX_HORSE_NAMES + 1];
typedef char h_name[MAX_HORSE_NAMES + 1][82];
typedef boolean h_bool[MAX_HORSE_NAMES + 1];
typedef float statr[11];

extern void hr__game_horse();

#endif /* HORSE_H */
