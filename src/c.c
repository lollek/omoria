#include "c.h"

#include "io.h"

#include <curses.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h> /* for ftruncate, usleep */

static long malloc_calls = 0;
static long malloc_bytes = 0;
static long free_calls = 0;
static long free_bytes = 0;

bool int_is_even(int number) {
    return (number & 1) == 0;
}

bool int_is_odd(int number) {
    return (number & 1) == 1;
}

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

void *safe_malloc(const int size, char *message) {
  void *new_pointer = malloc(size);

  if (new_pointer == NULL) {
    memory_error(size, message);
  }

  malloc_calls++;
  malloc_bytes += size;

  return new_pointer;
}

void safe_free(void *ptr, const int size, const char *message) {
  (void)message;
  free_calls++;
  free_bytes += size;

  free(ptr);
}

long min3(const long i1, const long i2, const long i3) {
  if (i1 < i2) {
    return i1 < i3 ? i1 : i3;
  }
  return i2 < i3 ? i2 : i3;
}

