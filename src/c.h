#pragma once

#include <stdbool.h>

#define max(xx, yy) (((xx) > (yy)) ? (xx) : (yy))
#define min(xx, yy) (((xx) < (yy)) ? (xx) : (yy))

bool int_is_even(int number);
bool long_is_even(long number);
bool int_is_odd(int number);
bool long_is_odd(long number);
void memory_error(int blocksize, char *message) __attribute__((noreturn));
void *safe_malloc(int size, char *message);
void safe_free(void *ptr, int size, const char *message);
long min3(long i1, long i2, long i3);
