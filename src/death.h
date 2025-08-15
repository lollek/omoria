#pragma once

#include <stdbool.h>

// Characters to list for highscore
extern long max_score;

void death_by_quitting(void);
void upon_death(bool can_respawn);
void make_tomb(char dstr[][82]);
