#include "kickout.h"

#include "configure.h"
#include "io.h"
#include "save.h"
#include "variables.h"

#include <stdbool.h>
#include <stdio.h>
#include <time.h>

/**
 *  Operating hours for Moria
 *  X = Open
 *  . = Closed
 */
char operating_hours[7][30] = {
    "SUN:XXXXXXXXXXXXXXXXXXXXXXXX|", "MON:XXXXXXXXXXXXXXXXXXXXXXXX|",
    "TUE:XXXXXXXXXXXXXXXXXXXXXXXX|", "WED:XXXXXXXXXXXXXXXXXXXXXXXX|",
    "THU:XXXXXXXXXXXXXXXXXXXXXXXX|", "FRI:XXXXXXXXXXXXXXXXXXXXXXXX|",
    "SAT:XXXXXXXXXXXXXXXXXXXXXXXX|"};


static bool is_inside_operating_hours(void) {
  time_t const cur_time = time(NULL);
  if (cur_time == -1) {
    fprintf(stderr, "Unable to fetch system time!\n");
    return false;
  }

  struct tm const *timeinfo = localtime(&cur_time);
  int const day = timeinfo->tm_wday;  // Days since sunday
  int const hour = timeinfo->tm_hour; // Hours since midight

  return operating_hours[day][hour + 4] == 'X';
}

void kick__kickout_player_if_time(void) {
  if (!kick__should_kickout()) return;

  find_flag = false;

  if (is_inside_operating_hours()) {
    msg_print("A new version of IMORIA is being installed.");
    msg_print("After your character is saved, wait a few "
        "minutes,");
    msg_print("And then try to run the game.");
    msg_print("");
  } else {
    msg_print("The gates to Moria are closed.");
    msg_print(" ");
  }

  for (int i = 0; i < 10; ++i) {
      if (sav__save_character()) {
          break;
      }
  }
  exit_game();
}

bool kick__should_kickout(void) {
  if (wizard1) return false;
  if (!is_inside_operating_hours()) return true;

  FILE *kick = fopen(KICKOUT_FILE, "r");

  if (kick != NULL) {
    fclose(kick);
    return true;
  }

  return false;
}

void kick__dump_operating_hours_to_file(FILE *file) {
  fprintf(file, "    Moria operating hours are:\n");
  fprintf(file, "    |    AM     |    PM     |\n");
  fprintf(file, "    1         111         111\n");
  fprintf(file, "    2123456789012123456789012\n");
  for (int i = 0; i < 7; i++) {
    fprintf(file, "%s\n", operating_hours[i]);
  }
  fprintf(file, "       (X=Open; .=Closed)\n");
}

