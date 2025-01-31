#include "river.h"
#include "../c.h"
#include "../constants.h"
#include "../debug.h"
#include "../misc.h"
#include "../random.h"
#include "../types.h"
#include "../variables.h"
#include "models.h"

typedef struct river_deal {
  enum keypad_direction_t in1; // (keypad) directions; in is upstream
  enum keypad_direction_t in2; // (keypad) directions; in is upstream
  enum keypad_direction_t out; // (keypad) directions; in is upstream
  long flow;                   // water flow out of this river spot
  long pos; // in array of s_l_type; if > num_left then spot is no longer
            // available
} river_deal;

typedef struct s_l_type {
  coords loc;     // cross-ref back to river_deal
  bool is_active; // is still an unresolved river source
} s_l_type;

typedef struct river_args {
  river_deal gup[RIVER_SIZE_Y + 1][RIVER_SIZE_X + 1];
  s_l_type s_list[RIVER_TOTAL_SIZE + 1];
  const long max_wet; // # of river or next-to-river
  long num_left;
  long s_l_top;
  coords river_mouth;
} river_args;

static bool r__is_inside_river(const coords *coord) {
  return coord->y >= 1 && coord->y <= RIVER_SIZE_Y && coord->x >= 1 &&
         coord->x <= RIVER_SIZE_X;
}

/*{ returns position of (this + dir) in gup or this if out of bounds }*/
static bool r__try_to_move_to_new_coordinate(const enum keypad_direction_t dir,
                                             const coords *old_coordinate,
                                             coords *new_coordinate) {
  ENTER(("r__try_to_move_to_new_coordinate", "r"));
  bool was_able_to_move = false;

  new_coordinate->y = old_coordinate->y + y_from_keypad_direction(dir);
  new_coordinate->x = old_coordinate->x + x_from_keypad_direction(dir);

  if (r__is_inside_river(new_coordinate)) {
    was_able_to_move = true;
  } else {
    // reset to legal value
    *new_coordinate = *old_coordinate;
  }

  RETURN("r__try_to_move_to_new_coordinate", "r", 'b', "moved",
         &was_able_to_move);
  return was_able_to_move;
}

static void r__remove_river_deal_from_list(river_args *river,
                                           const coords *coord) {
  ENTER(("r__remove_river_deal_from_list", "r"));

  river_deal *gup_to_delete_ptr = &river->gup[coord->y][coord->x];
  const long position_of_deleted_element = gup_to_delete_ptr->pos;
  s_l_type *slist_to_delete_ptr = &river->s_list[position_of_deleted_element];

  const bool is_still_available =
      position_of_deleted_element <= river->num_left;
  if (is_still_available) {
    s_l_type *last_slist_ptr = &river->s_list[river->num_left];
    river_deal *last_gup_ptr =
        &river->gup[last_slist_ptr->loc.y][last_slist_ptr->loc.x];

    // Copy last element to take the place of the element to delete
    *gup_to_delete_ptr = *last_gup_ptr;
    *slist_to_delete_ptr = *last_slist_ptr;
    gup_to_delete_ptr->pos = position_of_deleted_element;

    // Delete the element
    last_gup_ptr->pos = RIVER_TOTAL_SIZE;
    last_slist_ptr->is_active = false;
    river->num_left--;
  }

  LEAVE("r__remove_river_deal_from_list", "r");
}

static void r__plot_water(const long y, const long x, const long font,
                          const enum keypad_direction_t tdir) {
  ENTER(("r__plot_water", "r"));

  coords dots[6];
  dots[1].y = y;
  dots[1].x = x;

  long num_dots;
  switch (font) {
  case 0:
    num_dots = 1;
    break;

  case 1:
    num_dots = 2;
    dots[2].y = y + x_from_keypad_direction(tdir);
    dots[2].x = x - y_from_keypad_direction(tdir);
    break;

  default:
    num_dots = 5;
    for (long i = 1; i <= 4; i++) {
      dots[i + 1].y = y + y_from_keypad_direction(2 * i);
      dots[i + 1].x = x + x_from_keypad_direction(2 * i);
    }
    break;
  }

  for (long i = 1; i <= num_dots; i++) {
    if (in_bounds(dots[i].y, dots[i].x)) {
      if (cave[dots[i].y][dots[i].x].fval == dopen_floor.ftval ||
          cave[dots[i].y][dots[i].x].fval == lopen_floor.ftval) {
        cave[dots[i].y][dots[i].x].fval = water2.ftval;
        cave[dots[i].y][dots[i].x].fopen = water2.ftopen;
      } else {
        cave[dots[i].y][dots[i].x].fval = water1.ftval;
        cave[dots[i].y][dots[i].x].fopen = water1.ftopen;
      }

      cave[dots[i].y][dots[i].x].h2o = 1;

      if (cave[dots[i].y][dots[i].x].tptr != 0 &&
          t_list[cave[dots[i].y][dots[i].x].tptr].tval > valuable_metal) {
        pusht(cave[dots[i].y][dots[i].x].tptr);
        cave[dots[i].y][dots[i].x].tptr = 0;
      }
    }
  }

  LEAVE("r__plot_water", "r");
}

static long pr__figure_out_path_of_water(const long y, const long x,
                                         const long oy, const long ox) {

  ENTER(("pr__figure_out_path_of_water", "r"));

  const long target_dy = y - oy;
  const long target_dx = x - ox;
  const long dist_squared = target_dy * target_dy + target_dx * target_dx;
  long start[9];
  start[0] = 1;

  for (long i = 0; i <= 7; i++) { /*{octant number}*/
    const long dot_product = target_dy * y_from_keypad_direction(key_of[i]) +
                             target_dx * x_from_keypad_direction(key_of[i]);

    /*{formula subtracts dist_squared to keep stream semi-normal}*/
    /*{diagonals give root2 inflated dot_products}*/

    long chance;
    if (dot_product > 0) {
      if (long_is_odd(i)) {
        chance = dot_product * dot_product * 2 - dist_squared;
      } else {
        chance = dot_product * dot_product * 4 - dist_squared;
      }
    } else {
      chance = 0;
    }

    if (chance > 0) {
      start[i + 1] = start[i] + chance;
    } else {
      start[i + 1] = start[i];
    }
  }

  /*{choose random directions; chances partitioned by start[]}*/
  const long rand_num = randint(start[8] - 1);

  long i1 = -1;
  bool flag;
  do {
    i1++;
    flag = start[i1 + 1] > rand_num;
  } while (!flag);

  long return_value = key_of[i1];
  RETURN("pr__figure_out_path_of_water", "r", 'd', "key_of[i1]", &return_value);
  return return_value;
}

/*{ A recursive procedure, starting at river mouth and moving upstream; connects
 * the dots laid out by chart_river. }*/
static void r__place_river(river_args *river, const enum keypad_direction_t dir,
                           const enum keypad_direction_t next_dir,
                           const coords this, coords wiggle) {

  long done_first; /*{ compute next direction }*/

  ENTER(("r__place_river", "r"));

  coords upstream_end_of_segment;
  r__try_to_move_to_new_coordinate(dir, &this, &upstream_end_of_segment);

  coords upstream_end_of_next_segment;
  r__try_to_move_to_new_coordinate(next_dir, &upstream_end_of_segment,
                                   &upstream_end_of_next_segment);

  const river_deal river_deal =
      river
          ->gup[upstream_end_of_next_segment.y][upstream_end_of_next_segment.x];
  const long river_size = (river_deal.flow - 1) / 2;

  /*{aim (y,x) toward upstream end of segment, randomize slightly}*/
  long oy = RIVER_SEGMENT_SIZE * this.y + wiggle.y;
  long ox = RIVER_SEGMENT_SIZE * this.x + wiggle.x;

  if (dir != next_dir) {
    enum keypad_direction_t i2;
    const long i1 =
        oct_of[next_dir] - oct_of[dir]; /*{ (1=left, -1 = right) mod 8}*/
    if (oct_of[dir] % 2 == 0) {
      i2 = rotate_dir(next_dir, i1);
    } else {
      i2 = rotate_dir(next_dir, 2 * i1);
    }
    wiggle.y = y_from_keypad_direction(i2) + (randint(3) - 2);
    wiggle.x = x_from_keypad_direction(i2) + (randint(3) - 2);
  }

  /*{y,x=(upstream) destination of river}*/
  const long y = RIVER_SEGMENT_SIZE * upstream_end_of_segment.y + wiggle.y;
  const long x = RIVER_SEGMENT_SIZE * upstream_end_of_segment.x + wiggle.x;

  long overflow = 0;
  while ((oy != y || ox != x) && overflow++ < 5000) {
    const long temp_dir = pr__figure_out_path_of_water(y, x, oy, ox);
    if (long_is_even(temp_dir)) {
      move_dir(temp_dir, &oy, &ox);
      r__plot_water(oy, ox, river_size, temp_dir);
    } else {
      if (randint(2) == 1) {
        done_first = 1;
      } else {
        done_first = -1;
      }
      move_dir(rotate_dir(temp_dir, done_first), &oy, &ox);
      r__plot_water(oy, ox, river_size, temp_dir);
      move_dir(rotate_dir(temp_dir, -done_first), &oy, &ox);
      r__plot_water(oy, ox, river_size, temp_dir);
    }
  }

  /*{branch rivers 1 move early to make branching more gradual}*/
  if (river_deal.in1 != KEYPAD_NONE) {
    r__place_river(river, next_dir, river_deal.in1, upstream_end_of_segment,
                   wiggle);
  }
  if (river_deal.in2 != KEYPAD_NONE) {
    r__place_river(river, next_dir, river_deal.in2, upstream_end_of_segment,
                   wiggle);
  }

  LEAVE("r__place_river", "r");
}

/*{determines next point(s) upstream depending on coordinates (this),
  previous direction (a->gup[this].out), and available positions. outputs
  # of branches}*/
static long cr__choose_stream_dirs(const river_args *river, coords *this,
                                   long dir, long that_dir[], bool that_ok[],
                                   coords that[], bool that_chosen[]) {

  long return_value = 0;

  ENTER(("cr__choose_stream_dirs", "r"));

  *this = river->s_list[river->s_l_top].loc;
  dir = river->gup[this->y][this->x].out;

  for (long i = 1; i <= 3; i++) { /*{left,straight,right}*/
    that_dir[i] = rotate_dir(dir, 2 - i);
    that_ok[i] = r__try_to_move_to_new_coordinate(that_dir[i], this, &that[i]);
    if (that_ok[i]) {
      that_ok[i] = river->gup[that[i].y][that[i].x].pos <= river->num_left;
    }
    that_chosen[i] = false;
  }

  bool done = false;

  if (randint(3 * river->gup[this->y][this->x].flow) == 1 ||
      !(that_ok[1] || that_ok[2] || that_ok[3])) {
    /*{end stream if blocked or small river and random}*/
    done = true;
    return_value = 0;
  } else if ((randint(5) == 1 || !(that_ok[1] || that_ok[3])) && that_ok[2]) {
    /*{straight stream (1/5 and ok) or sides blocked}*/
    done = true;
    that_chosen[2] = true;
    return_value = 1;
  } else if (randint(5) == 1 && (that_ok[1] && that_ok[3])) {
    /*{fork 1/5 and both sides ok}*/
    done = true;
    that_chosen[1] = true;
    that_chosen[3] = true;
    return_value = 2;
  }

  if (!done) { /*{ 1 or 3 must be open } */
    /*{check 1 side first; if it fails, second must be true}*/
    const long i = 2 * randint(2) - 1;
    that_chosen[i] = that_ok[i];
    that_chosen[4 - i] = !that_chosen[i];
    return_value = 1;
  }
  /*{no rivers adjacent each other (except connected segments)}*/

  RETURN("cr__choose_stream_dirs", "r", 'd', "branches", &return_value);
  return return_value;
}

/*{get highest unresolved river segment; a->s_l_top points to new segment if any
 * is found. }*/
static bool cr__dequeue_s_list(river_args *river) {
  ENTER(("cr__dequeue_s_list", "r"));

  bool some_left;
  while (river->s_l_top > river->num_left &&
         !river->s_list[river->s_l_top].is_active) {
    river->s_l_top--;
  }

  if (river->s_l_top > river->num_left) {
    river->s_list[river->s_l_top].is_active = false;
    some_left = true;
  } else {
    some_left = false;
  }

  RETURN("cr__dequeue_s_list", "r", 'b', "some left", &some_left);
  return some_left;
}

/*{ recursively charts basic path of stream upstream }*/
static void r__chart_river(river_args *river) {
  ENTER(("r__chart_river", "r"));

  r__remove_river_deal_from_list(river,
                                 &river->s_list[randint(river->num_left)].loc);

  river->s_list[river->s_l_top].is_active = true;
  coords this = river->s_list[river->s_l_top].loc;
  river->gup[this.y][this.x].flow = 4 + randint(3);
  river->river_mouth = this;

  bool that_chosen[4] = {false, false, false, false};

  coords that[4];
  enum keypad_direction_t dir;
  long attempt = 0;
  do { /*{ choose initial heading, in streams }*/
    dir = randint(8);
    if (dir == KEYPAD_NONE) {
      dir = KEYPAD_UP_RIGHT;
    }
    attempt++;
    if (r__try_to_move_to_new_coordinate(dir, &this, &that[2])) {
      that_chosen[2] = river->gup[that[2].y][that[2].x].pos <= river->num_left;
    }
  } while (!(that_chosen[2] || attempt >= 10));

  bool that_ok[4];
  long that_dir[4];
  that_dir[2] = dir;
  that_ok[2] = true;
  long branches = 1;

  bool starting_river = true;
  coords thing;
  while (cr__dequeue_s_list(river)) { /*{loop until river stops}*/
    if (starting_river) {
      starting_river = false;
    } else {
      branches = cr__choose_stream_dirs(river, &this, dir, that_dir, that_ok,
                                        that, that_chosen);
    }

    for (long i = 1; i <= 9; i++) {
      if (r__try_to_move_to_new_coordinate(i, &this, &thing)) {
        r__remove_river_deal_from_list(river, &thing);
      }
    }

    if (that_chosen[1]) { /*{ No sharp left turns }*/
      r__try_to_move_to_new_coordinate(rotate_dir(dir, 1), &this, &thing);
      if (r__try_to_move_to_new_coordinate(rotate_dir(dir, 2), &thing,
                                           &thing)) {
        r__remove_river_deal_from_list(river, &thing);
      }
    }

    if (that_chosen[3]) { /*{ No sharp right turns }*/
      r__try_to_move_to_new_coordinate(rotate_dir(dir, -1), &this, &thing);
      if (r__try_to_move_to_new_coordinate(rotate_dir(dir, -2), &thing,
                                           &thing)) {
        r__remove_river_deal_from_list(river, &thing);
      }
    }

    const long out_flow = river->gup[this.y][this.x].flow;
    long i2 = 1;
    for (long i = 1; i <= 3; i++) {
      if (that_chosen[i] &&
          RIVER_TOTAL_SIZE - river->num_left < river->max_wet) {
        long in_flow;
        if (branches == 1) {
          in_flow = out_flow;
        } else {
          in_flow = out_flow - randint(2);
        }

        if (in_flow > 0) {
          if (i2 == 1) {
            river->gup[this.y][this.x].in1 = that_dir[i];
          } else {
            river->gup[this.y][this.x].in2 = that_dir[i];
          }
          river->s_list[river->gup[that[i].y][that[i].x].pos].is_active = true;
          river->gup[that[i].y][that[i].x].out = that_dir[i];
          river->gup[that[i].y][that[i].x].flow = in_flow;
        }
        i2++;
      }
    }
  }

  LEAVE("r__chart_river", "r");
}

static void r__draw_river(river_args *river) {
  ENTER(("r__draw_river", "r"));

  const coords wiggle = {
      .y = randint(3) - 2,
      .x = randint(3) - 2,
  };

  const enum keypad_direction_t first_dir =
      river->gup[river->river_mouth.y][river->river_mouth.x].in1;

  coords next_coordinate_of_river;
  const bool could_move = r__try_to_move_to_new_coordinate(
      first_dir, &river->river_mouth, &next_coordinate_of_river);

  if (could_move) {
    const river_deal next_river_deal =
        river->gup[next_coordinate_of_river.y][next_coordinate_of_river.x];
    if (next_river_deal.in1 != KEYPAD_NONE) {
      r__place_river(river, first_dir, next_river_deal.in1, river->river_mouth,
                     wiggle);
    }
    if (next_river_deal.in2 != KEYPAD_NONE) {
      r__place_river(river, first_dir, next_river_deal.in2, river->river_mouth,
                     wiggle);
    }
  }

  LEAVE("r__draw_river", "r");
}

void generate_rivers(void) {
  ENTER(("all_the_river_stuff", "r"));
  river_args river = {
      .max_wet = min(randint(RIVER_TOTAL_SIZE) - 50, 0),
      .num_left = 0,
  };

  for (long y = 1; y <= RIVER_SIZE_Y; y++) {
    for (long x = 1; x <= RIVER_SIZE_X; x++) {
      river.num_left++;

      river.gup[y][x].in1 = KEYPAD_NONE;
      river.gup[y][x].in2 = KEYPAD_NONE;
      river.gup[y][x].out = KEYPAD_NONE;
      river.gup[y][x].flow = 0;
      river.gup[y][x].pos = river.num_left;

      river.s_list[river.num_left].loc.y = y;
      river.s_list[river.num_left].loc.x = x;
      river.s_list[river.num_left].is_active = false;
    }
  }

  if (river.num_left > RIVER_TOTAL_SIZE) {
    MSG(("ERROR! river.num_left > RIVER_TOTAL_SIZE"));
  }
  for (long i = river.num_left; i < RIVER_TOTAL_SIZE; i++) {
    river.s_list[i].is_active = false;
  }

  /*{remove borders of map}*/
  for (long i = 1; i <= river.num_left; i++) {
    if (river.s_list[i].loc.y == 1 || river.s_list[i].loc.y == RIVER_SIZE_Y ||
        river.s_list[i].loc.x == 1 || river.s_list[i].loc.x == RIVER_SIZE_X) {
      r__remove_river_deal_from_list(&river, &river.s_list[i].loc);
    }
  }

  river.s_l_top = river.num_left;

  while (RIVER_TOTAL_SIZE - river.num_left < river.max_wet) {
    r__chart_river(&river);
    r__draw_river(&river);
  }

  LEAVE("all_the_river_stuff", "r");
}
