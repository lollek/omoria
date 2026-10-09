#pragma once

#include <stdbool.h>
#include <stddef.h>

#define record_message(message) _record_message(message)
void _record_message(char const *message);
void show_recorded_messages(void);
bool message_capture_active(void);
void C_message_capture_begin(void);
void C_message_capture_end(void);
size_t C_message_capture_count(void);
/* A non-null buffer must be writable for length bytes; successful copies are NUL-terminated. */
bool C_message_capture_get(size_t index, char *buffer, size_t length);
