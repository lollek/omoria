#pragma once

#include <stdbool.h>
#include <stddef.h>

typedef bool (*main_loop_command_source)(void *context, char *command);

/* The source returns false on exhaustion; context is borrowed for this call.
 * max_turns bounds outer ticks, not command retries. Exhaustion can return
 * after a tick's updates but before dispatch/creature movement. Zero budget
 * or a NULL source is a no-op. Prompts and rendering still need separate seams.
 */
void main_loop_with_commands(main_loop_command_source source, void *context,
							 size_t max_turns);
int main_loop(void);
