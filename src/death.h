#ifndef DEATH_H
#define DEATH_H

// Characters to list for highscore
extern long max_score;

void death_by_quitting(void);
void upon_death(void);
void make_tomb(char dstr[][82]);
void write_tomb(char dstr[][82]);
void print_dead_character(void);

#endif /* DEATH_H */
