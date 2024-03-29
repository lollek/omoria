#ifndef PORT_H
#define PORT_H

void memory_error(int blocksize, char *message) __attribute__((noreturn));
void *safe_malloc(int size, char *message);
void dispose(void *ptr, int size, char *message);
char *chomp(char *input_line);
long min3(long i1, long i2, long i3);

#endif // PORT_H