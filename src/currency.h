#ifndef CURRENCY_H
#define CURRENCY_H

#include <stdint.h>

/**
 * @brief Get value of currency type
 * 
 * @param currency See conversion/currency.rs
 * @return value of currency 
 */
int64_t coin_value(uint8_t currency);

#endif // CURRENCY_H