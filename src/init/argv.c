#include <stdio.h>

#include "../variables.h"
#include "../death.h"
#include "../highscore.h"
#include "../wizard.h"

#include "argv.h"

static void print_arg_usage(void) {
  printf("\n\r"
         "Invalid Moria option!  Valid qualifiers are:\n"
         "  -w         Warn about hearing things in water.\n"
         "  -s         List top 20 high scores.\n"
         "  -t num     List <num> high scores after death or for -s.\n"
         "  -V         Print version info.\n"
         "\n"
         "\n");
  printf("Wizard commands:\n"
         "  -Wpassword Enter wizard mode password.\n"
         "  -Rfile     Restore character in master file.\n"
         "  -Ufile     Change the dead flag in save file.\n"
         "  -Efile     Encrypt a character file.\n"
         "  -Dfile     Decrypt a character file.\n"
         "\n");
}

static void print_version_info(void) {
  printf("\n\r"
         "Linux Omoria Version %s\n\r"
         "\n\r",
         omoria_version());
}

bool init__argv(int argc, char *argv[]) {
  bool print_usage = false;

  /* parse the command line arguments */
  for (--argc, ++argv; argc > 0 && argv[0][0] == '-'; --argc, ++argv) {
    switch (argv[0][1]) {
    case 'V':
      /* version info */
      print_version_info();
      return false;

    case 's':
      /* print the high scores */
      C_highscore(max_score);
      return false;

    case 't':
      /* number of scores to show */
      if (--argc) {
        sscanf((++argv)[0], "%ld", &max_score);
      } else {
        printf("Missing <num> for -t\n\r");
        print_usage = true;
      }
      break;

    case 'w':
      /* warn about things in the water */
      want_warn = true;
      break;

    case 'W':
      /* kick into wizard mode */
      if (argv[0][2] == 0) {
        check_pswd("", false);
      } else {
        check_pswd(&argv[0][2], true);
      }
      break;

    default:
      printf("Unknown switch \"%s\"\n\r", argv[0]);
      print_usage = true;
      break;
    }
  }

  if (print_usage) {
    print_arg_usage();
    return false;
  }

  return true;
}
