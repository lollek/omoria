#pragma once

extern long bet;
extern long gld;
extern long tics;
extern bool c_closed;

void c__display_gold(void);
bool c__get_response(char comment[82], long *num);
void c__change_money(void);
void c__check_casino_kickout(void);
