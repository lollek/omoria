#ifndef PLAYER_H
#define PLAYER_H

/* array[stat_set] of {permanent} */
extern uint8_t player_stats_perm[STAT_SET_MAX + 1];
/* array[stat_set] of {current=p-l+m*10} */
extern uint8_t player_stats_curr[STAT_SET_MAX + 1];
/* array[stat_set] of {net magical adj} */
extern int8_t  player_stats_mod[STAT_SET_MAX + 1];
/* array[stat_set] of {amt lost} */
extern uint8_t player_stats_lost[STAT_SET_MAX + 1];

#endif /* PLAYER_H */
