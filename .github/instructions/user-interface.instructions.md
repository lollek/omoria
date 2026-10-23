---
description: "Use when adding or changing terminal UI fields, HUD text, message logs, dimming, visibility, command/input lifecycle, spacing, or column alignment."
applyTo: "src/user_interface/**/*.rs,src/term*.rs,src/term.c,src/io.c,src/message.rs,src/main_loop/main_loop.c,tests/message_ffi.c"
---

# Terminal UI Behavior and Formatting

- Before changing transient styling or visibility, trace the production event order in `src/main_loop/main_loop.c` and the terminal input hooks. Distinguish internal game turns, accepted player commands, and prompt or `-more-` input; clarify which event starts and ends the requested visual state when ambiguous.
- Test the state visible while waiting for the next command, not just immediately after printing. For message-log styling, cover player messages, monster responses, turn advancement, status updates, redraws, and the next accepted command; assert the style of every affected line. Keep prompt acknowledgments separate from command boundaries.
- When adding label/value fields, match the neighboring value-start column rather than inserting a single space after each label. Preserve existing label and colon conventions.
- Test the production renderer's exact text, including padding, so the tests detect value-column misalignment. Compare the expected value-start position with existing fields in the same column.
- Keep text-formatting evidence separate from terminal-drawing evidence: passing string assertions does not verify live screen placement or visibility.
