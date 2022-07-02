#ifndef DEATH_H
#define DEATH_H

// Characters to list for highscore
extern long max_score;

void upon_death();
void make_tomb(char dstr[][82]);
void write_tomb(char dstr[][82]);
void print_dead_character();

#endif /* DEATH_H */
