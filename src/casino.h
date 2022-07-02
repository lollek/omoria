#ifndef CASINO_H
#define CASINO_H

#include "boolean.h"

extern long bet;
extern long gld;
extern long tics;
extern boolean c_closed;

void c__display_gold();
boolean c__get_response(char comment[82], long *num);
void c__change_money();
void c__check_casino_kickout();

void enter_casino();

#endif /* CASINO_H */
