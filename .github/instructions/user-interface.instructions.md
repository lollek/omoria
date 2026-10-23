---
description: "Use when adding or changing terminal UI fields, character-screen labels, HUD text, spacing, or column alignment."
applyTo: "src/user_interface/**/*.rs"
---

# Terminal UI Formatting

- When adding label/value fields, match the neighboring value-start column rather than inserting a single space after each label. Preserve existing label and colon conventions.
- Test the production renderer's exact text, including padding, so the tests detect value-column misalignment. Compare the expected value-start position with existing fields in the same column.
- Keep text-formatting evidence separate from terminal-drawing evidence: passing string assertions does not verify live screen placement or visibility.
