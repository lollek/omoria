#ifndef FILES_H
#define FILES_H

#include "boolean.h"

void print_map();
void print_objects();
void print_monsters();
void file_character();
boolean read_top_scores(FILE **f1, char *fnam, char list[][134],
                               int max_high, int *n1, char *openerr);
boolean write_top_scores(FILE **f1, char list[][134], int max_high);
boolean close_top_scores(FILE **f1);

#endif // FILES_H