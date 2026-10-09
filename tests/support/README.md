# Headless C Terminal Support (HT1)

`make test-messages` runs the original message boundary assertions and the
terminal support checks without initializing curses. The support is C because
the behavior under test is caller-side C macro redirection.

## Linking a Harness

Compile `terminal.c` normally and link it into the harness. Compile caller
objects such as `screen.c`, `dungeon/light.c`, and `main_loop/main_loop.c`
with `-include tests/support/terminal_redirects.h`. Exclude those objects' normal
production versions from the harness link. The screen redirects include the
two Rust HUD entry points, which otherwise call ncurses directly.

Do **not** force-include the redirects in `terminal.c`, `io.c`, or `term.c`:
these files define some of the symbols being redirected. For the message
caller object, use only `-Dinkey=headless_inkey`,
`-Dput_buffer=headless_put_buffer`, and `-DErase_Line=headless_erase_line`,
as shown in the Makefile. This io object is not a general headless io layer;
unredirected prompts and other helpers can still reach curses. Redirect their
callers rather than invoking them through this object.

## Controls and Boundaries

* Call `headless_terminal_reset()` before and after each scenario. It clears
  scripts and counters and forbids drawing. It does not reset gameplay globals.
  The support is process-global and must not be used concurrently.
* Enable drawing explicitly with `headless_terminal_allow_drawing(true)`.
  Drawing is counted, not rendered. Glyph and map-string calls preserve
  `used_line` updates; invalid map rows fail. Screen clearing and message-row
  `prt` preserve empty-message effects, so use real message capture when needed.
* `headless_terminal_script(keys, count)` installs a **borrowed** byte sequence;
  keep it alive until reset or replacement. It need not be NUL-terminated.
  Each `headless_inkey()` consumes one key and clears `command_count` and
  `msg_flag`. Exhaustion, including after reset, prints a diagnostic and exits
  with `EXIT_FAILURE`, never returning a synthesized gameplay command.
* Script only explicitly expected low-level prompt keys, such as `-more-`
  acknowledgments. Gameplay commands go through the HL1 command source
  (`main_loop_with_commands` in `main_loop.h`), not this script. Selection, confirmation, and string prompts are unsupported:
  their doubles fail even if keys remain in the low-level script.
* `headless_inkey_delay()` models the current production nonblocking poll: it
  refreshes through a double, returns no key, and consumes no scripted input.
  Actual rest timing and asynchronous input are not covered.

Failure checks run in child processes and assert both the exit status and
diagnostic. A child alarm turns an accidental hang into a failed test, not a
passed fail-fast check. Rendering scenarios repeat twice and restore their
fixture globals before the original message tests run.

This is terminal-boundary coverage, including real screen and blind-light
callers. Bounded main-loop scenarios are in the `test-headless*` targets. Other production
objects and direct Rust ncurses callers are not automatically intercepted.