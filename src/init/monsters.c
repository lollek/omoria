#include <stdbool.h>
#include <stdio.h>
#include <string.h>

#include "../configure.h"
#include "../creature.h"
#include "../debug.h"
#include "../variables.h"

/**
 * init_m_level() - Initializes M_LEVEL array for use with PLACE_MONSTER
 * */
static bool init_m_level(void) {
  int i1 = 1;
  int i2 = 0;
  int i3 = MAX_CREATURES - WIN_MON_TOT;

  ENTER(("init_m_level", ""));

  do {
    m_level[i2] = 0;
    while ((i1 <= i3) && (c_list[i1].level == i2)) {
      m_level[i2]++;
      i1++;
    }
    i2++;
  } while (i2 <= MAX_MONS_LEVEL);

  for (i1 = 2; i1 <= MAX_MONS_LEVEL; i1++) {
    m_level[i1] += m_level[i1 - 1];
  }

  /*  for (i1 = 0; i1 < MAX_MONS_LEVEL+1; i1++) { */
  /*    printf ("\n m_level[%d] : %d",i1,m_level[i1]);  fflush(stdout); */
  /*  } */
  LEAVE("init_m_level", "");
  return true;
}

bool init__monsters(void) {
  if (!init_m_level())
    return false;
  return true;
}
