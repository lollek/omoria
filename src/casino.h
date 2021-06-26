#ifndef CASINO_H
#define CASINO_H

#include "boolean.h"

extern long bet;
extern long gld;
extern long tics;
extern boolean c_closed;

extern void c__display_gold();
extern boolean c__get_response(char comment[82], long *num);
extern void c__change_money();
extern void c__check_casino_kickout();

#endif /* CASINO_H */
