#include "../variables.h"
#include "../inven.h"
#include "../desc.h"
#include "../screen.h"

#define OBJ_LAMP_MAX 15000 /*{ Maximum amount that lamp can be filled} */

void player_action_refill_lamp(void) {
  long i2, i3;
  treas_rec *i1;
  obj_set this_be_oil = {flask_of_oil, 0};

  i3 = equipment[Equipment_light].subval;

  if ((i3 > 0) && (i3 < 10)) {
    if (find_range(this_be_oil, false, &i1, &i2)) {
      msg_print("Your lamp is full.");
      /* with equipment[Equipment_light]. do; */
      equipment[Equipment_light].p1 += i1->data.p1;
      if (equipment[Equipment_light].p1 > OBJ_LAMP_MAX) {
        equipment[Equipment_light].p1 = OBJ_LAMP_MAX;
      }
      desc_remain(i1);
      inven_destroy(i1);
      prt_stat_block();
    } else {
      msg_print("You have no oil.");
    }
  } else {
    msg_print("But you are not using a lamp.");
  }
}