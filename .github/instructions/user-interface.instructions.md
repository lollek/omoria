---
description: "Use when adding or changing terminal UI fields, sidebar panels, equipment or inventory headings, HUD text, message logs, dimming, visibility, command/input lifecycle, spacing, or column alignment."
applyTo: "src/user_interface/**/*.rs,src/ncurses.rs,src/term*.rs,src/term.c,src/io.c,src/message.rs,src/main_loop/main_loop.c,tests/message_ffi.c"
---

# Terminal UI Behavior and Formatting

- Before changing transient styling or visibility, trace the production event order in `src/main_loop/main_loop.c` and the terminal input hooks. Distinguish internal game turns, accepted player commands, and prompt or `-more-` input; clarify which event starts and ends the requested visual state when ambiguous.
- Test the state visible while waiting for the next command, not just immediately after printing. For message-log styling, cover player messages, monster responses, turn advancement, status updates, redraws, and the next accepted command; assert the style of every affected line. Keep prompt acknowledgments separate from command boundaries.
- When adding label/value fields, match the neighboring value-start column rather than inserting a single space after each label. Preserve existing label and colon conventions.
- When adding or changing adjacent panels, use consistent heading style and alignment and explicit visual separation. Apply an approved heading convention to the neighboring panels in scope; do not leave one list titled and the next untitled unless preserving Classic behavior requires it.
- Test adjacent panel boundaries at the minimum supported terminal size, not only each list's text in isolation. Assert heading and separator positions, reserved prompt/map boundaries, retained item or equipment capacity, and overflow counts after subtracting heading/separator rows. If spacing requires changing layout rectangles or hiding existing slots, surface that scope decision before editing.
- Test the production renderer's exact text, including padding, so the tests detect value-column misalignment. Compare the expected value-start position with existing fields in the same column.
- When preserving an existing Classic display, derive expected lines from the legacy formatting (including label padding and empty values) instead of approximating spacing by eye.
- Keep text-formatting evidence separate from terminal-drawing evidence: passing string assertions does not verify live screen placement or visibility.
- For panels reaching the terminal's last row or column, exercise the production `src/ncurses.rs` drawing wrapper with an explicitly sized, file-backed curses screen (`newterm`), not just a string sink. Cover a padded line ending at the bottom-right cell, preservation of neighboring cells and attributes, and genuine overflow errors. Curses may write the last cell and still return `ERR` when cursor advancement fails; avoid that advancement rather than swallowing `ERR` or enabling scrolling.
