#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>

#include "../debug.h"
#include "../variables.h"
#include "../configure.h"

bool init__file_paths(void) {
  char const *const datapath = DATA_FILE_PATH;

  if (strlen(datapath) > (sizeof(MORIA_HOU) - 20)) {
    printf("Umm, DATA_FILE_PATH is too long (%lu chars).\n\r",
           strlen(datapath));
    printf("Keep it under %lu chars or change the type\n\r",
           sizeof(MORIA_HOU) - 20);
    printf("of MORIA_HOU and friends in variables.h.\n\r");
    printf("Or fix the get_paths() code in io.c.  Your choice.\n\r");
    return false;
  }

  sprintf(MORIA_HOU, "%s/hours.dat", datapath);
  sprintf(MORIA_LCK, "%s/morialock.lock", datapath);
  sprintf(MORIA_MON, "%s/monsters.dat", datapath);
  sprintf(MORIA_DTH, "%s/death.log", datapath);
  sprintf(MORIA_GCST, "%s/moria_gcustom.mst", datapath);
  sprintf(MORIA_TRD, "%s/moriatrd.dat", datapath);

  sprintf(MORIA_CST, "moria_custom.mst");
  return true;
}
