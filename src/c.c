#include "c.h"

bool int_is_even(int number) {
    return (number & 1) == 0;
}

bool int_is_odd(int number) {
    return (number & 1) == 1;
}
