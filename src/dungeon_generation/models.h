#ifndef GENERATE_H
#define GENERATE_H

#include "../boolean.h"

#define RIVER_SIZE_Y 10
#define RIVER_SIZE_X 31
#define RIVER_TOTAL_SIZE 310
#define RIVER_SEGMENT_SIZE 6

typedef struct coords {
  long y;
  long x;
} coords;

typedef struct river_deal {
  long in1, in2, out; // (keypad) directions; in is upstream
  long flow;          // water flow out of this river spot
  long pos;           // in array of s_l_type; if > num_left then spot is no longer available
} river_deal;

typedef struct s_l_type {
  coords loc;        // cross-ref back to river_deal
  boolean is_active; // is still an unresolved river source
} s_l_type;

typedef struct river_args {
  river_deal gup[RIVER_SIZE_Y + 1][RIVER_SIZE_X + 1];

  s_l_type s_list[RIVER_TOTAL_SIZE + 1];

  long max_wet; // # of river or next-to-river
  long num_left, s_l_top;
  coords river_mouth;

} river_args;

#endif /* GENERATE_H */
