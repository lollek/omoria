/* port.c */
/* code to help port the vax pascal into linux c */

#include <curses.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "io.h"

static long malloc_calls = 0;
static long malloc_bytes = 0;
static long free_calls = 0;
static long free_bytes = 0;

void memory_error(const int blocksize, char *message) {
  endwin();
  printf("\n\r\n\rMemory error (%d bytes)! Ref: %s.\n\r\n\r", blocksize,
         message);
  printf("malloc calls: %ld\tmalloc bytes: %ld\n\r", malloc_calls,
         malloc_bytes);
  printf("free   calls: %ld\tfree   bytes: %ld\n\r", free_calls, free_bytes);
  printf("\n\rdelta  calls: %ld\ndelta  bytes: %ld\n\r",
         malloc_calls - free_calls, malloc_bytes - free_bytes);
  printf("\n\r\n\r");
  exit_game();
}

void *safe_malloc(const int blocksize, char *message) {
  void *new_pointer = malloc(blocksize);

  if (new_pointer == NULL) {
    memory_error(blocksize, message);
  }

  malloc_calls++;
  malloc_bytes += blocksize;

  return new_pointer;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void dispose(void *ptr, const int size, const char *message) {
  (void)message;
  free_calls++;
  free_bytes += size;

  free(ptr);
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */

char *chomp(char *input_line) {
  /* remove \n from the end of a string if there is one */

  const long x = strlen(input_line);
  if (x && input_line[x - 1] == '\n') {
    input_line[x - 1] = 0;
  }

  return input_line;
}

/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */

long min3(const long i1, const long i2, const long i3) {
  if (i1 < i2) {
    return i1 < i3 ? i1 : i3;
  } else {
    return i2 < i3 ? i2 : i3;
  }
}

/*//////////////////////////////////////////////////////////////////// */

/* Something happens to disturb the player.		-CJS-
   The first arg indicates a major disturbance, which affects search.
   The second arg indicates a light change. */
/* see moria1.c from umoria */
void disturb(__attribute__((unused)) int s, __attribute__((unused)) int l) {
#if 0
  command_count = 0;
  if (s && search_flag)
    search_off();
  if (player_flags.rest > 0)
    rest_off();
  if (l || find_flag)
    {
      find_flag = FALSE;
      check_view();
    }
#endif
}

/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */

/* END FILE  port.c */
