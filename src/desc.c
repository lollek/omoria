#include <curses.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "configure.h"
#include "constants.h"
#include "debug.h"
#include "magic.h"
#include "pascal.h"
#include "routines.h"
#include "term.h"
#include "treasures.h"
#include "types.h"
#include "variables.h"

char *bag_descrip(treas_rec *bag, char result[134]) /* was func */
{
  /*{ Return description about the contents of a bag	-DMF-	}*/

  long count, wgt;
  treas_rec *ptr;

  if ((bag->next == NULL) || (bag->next->is_in == false)) {
    sprintf(result, " (empty)");
  } else {
    count = 0;
    wgt = 0;

    for (ptr = bag->next; (ptr != NULL) && (ptr->is_in); ptr = ptr->next) {
      count += ptr->data.number;
      wgt += ptr->data.weight * ptr->data.number;
    }

    sprintf(result, " (%ld%% full, containing %ld item%s)",
            (wgt * 100 / bag->data.p1), count, ((count != 1) ? "s" : ""));
  }
  return result;
}
