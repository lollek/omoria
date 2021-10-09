#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <stdbool.h>

#include "../debug.h"
#include "../variables.h"
#include "../configure.h"

bool init__file_paths(void) {
  char const * const datapath = DATA_FILE_PATH;
  size_t datapath_len = strlen(DATA_FILE_PATH);

  char const * const hours_suffix = "/hours.dat";
  MORIA_HOU = malloc(datapath_len + strlen(hours_suffix) + 1);
  if (MORIA_HOU == NULL) {
    fprintf(stderr, "%s", "Virtual memory exhausted\n");
    return false;
  }
  sprintf((char *)MORIA_HOU, "%s%s", datapath, hours_suffix);

  char const * const trd_suffix = "/moriatrd.dat";
  MORIA_TRD = malloc(datapath_len + strlen(trd_suffix) + 1);
  if (MORIA_TRD == NULL) {
    fprintf(stderr, "%s", "Virtual memory exhausted\n");
    return false;
  }
  sprintf((char *)MORIA_TRD, "%s%s", datapath, trd_suffix);

  char const * const death_suffix = "/death.log";
  MORIA_DTH = malloc(datapath_len + strlen(death_suffix) + 1);
  if (MORIA_DTH == NULL) {
    fprintf(stderr, "%s", "Virtual memory exhausted\n");
    return false;
  }
  sprintf((char *)MORIA_DTH, "%s%s", datapath, death_suffix);

  char const * const monsters_suffix = "/monsters.dat";
  MORIA_MON = malloc(datapath_len + strlen(monsters_suffix) + 1);
  if (MORIA_MON == NULL) {
    fprintf(stderr, "%s", "Virtual memory exhausted\n");
    return false;
  }
  sprintf((char *)MORIA_MON, "%s%s", datapath, monsters_suffix);

  return true;
}
