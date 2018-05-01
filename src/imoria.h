#ifndef IMORIA_H
#define IMORIA_H
/* include flies for all the modlues */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <curses.h>
#include <math.h>
#include <time.h>
#include <unistd.h> /* for ftruncate, usleep */

/*#include <sgtty.h>*/
#include "patchlevel.h"
#include "configure.h"
#include "constants.h"
#include "types.h"
#include "magic.h"
#include "pascal.h"
#include "routines.h"
#include "term.h"
#include "classes.h"
#include "races.h"

#include "debug.h"

#include "variables.h"

#define PM (py.misc)
#define PF (py.flags)

#endif /* IMORIA_H */
