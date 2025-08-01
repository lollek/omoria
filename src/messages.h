#pragma once

#define record_message(message) _record_message(message)
void _record_message(char const *message);
void show_recorded_messages(void);
