#include "../variables.h"
#include "../random.h"

#include "bank.h"

bool init__bank(void) {
  long starting = (randint(2000) + 1000) * 1000;
  bank[IRON] = starting / 8;
  bank[COPPER] = starting / 30;
  bank[SILVER] = starting / 50;
  bank[GOLD] = starting / 250;
  bank[PLATINUM] = starting / 5000;
  bank[MITHRIL] = starting / 100000;
  bank[TOTAL_] = (bank[MITHRIL] * coin_value[MITHRIL] +
      bank[PLATINUM] * coin_value[PLATINUM]) /
    GOLD_VALUE +
    bank[GOLD];

  return true;
}
