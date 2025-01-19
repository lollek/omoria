#include "../debug.h"
#include "../variables.h"
#include "../random.h"

#include "stores.h"

bool init__stores(void) {
  ENTER(("init__stores", ""));

  const int max_owners = MAX_OWNERS / MAX_STORES;
  for (int i = 0; i < MAX_STORES; i++) {
    stores[i].owner = MAX_STORES * (randint(max_owners) - 1) + i;
    stores[i].insult_cur = 0;
    stores[i].store_open.year = 0;
    stores[i].store_open.month = 0;
    stores[i].store_open.day = 0;
    stores[i].store_open.hour = 0;
    stores[i].store_open.secs = 0;
    stores[i].store_ctr = 0;
    for (int j = 0; j <= STORE_INVEN_MAX; j++) {
      stores[i].store_inven[j].sitem = blank_treasure;
      stores[i].store_inven[j].scost = 0;
    }
  }

  LEAVE("init__stores", "s");
  return true;
}

