## Plan: Configurable Combat Calculations

**Status: Implemented**

Add a session-only `M-V` toggle for separate in-game calculation messages. When enabled, show the actual attack roll versus defence and hit result next to the normal attack message; on damage, show the applied amount and the target’s HP before and after. Keep combat formulas and RNG behavior unchanged.

**Steps**
1. **RED:** Test `M-V` on/off feedback and confirm it does not consume a turn. Add deterministic checks for player and monster roll messages, natural-one fallbacks, RNG parity, setting gating, and damage/HP formatting in the existing message and headless interaction harnesses.
2. **GREEN:** Add an initially-off, C-owned session setting following the existing `M-R` toggle pattern. Use `M-V`, show confirmation when toggled, set `reset_flag`, and update the help entry. Keep the setting out of character saves.
3. Capture the player’s existing roll values for melee and ranged/thrown attacks; do not reroll. For monster attacks, add a detailed Rust roll result with a narrow C ABI bridge, retaining the existing boolean API for other callers.
4. When enabled, show the roll/result message immediately before the ordinary hit/miss message. After HP-affecting hits, show applied damage and HP before/after; status-only attacks get no damage line.
5. **REFACTOR:** Keep the output disabled by default and preserve current messages, turn behavior, formulas, and RNG consumption when off. Add an Unreleased changelog note.

**Relevant files**
- Toggle and help: `command.c`, `variables.c`, `variables.h`, `help.c`
- Player attacks: `player.c`, `attack.c`, `ranged.c`
- Monster attacks: `fighting.rs`, `fighting.h`, `creature.c`
- Diagnostics, tests, and release note: `diagnostics.c`, `diagnostics.h`, `headless_interaction.c`, `message_ffi.c`, `CHANGELOG.md`

**Verification**
1. Passed: `cargo test combat::fighting --lib`, `make test-headless-interaction`, and `make test-messages`.
2. Passed: `make check`; `test-traps` is skipped on macOS because its C caller doubles require Linux GNU ld `--wrap`.
3. Not run: manual `make run` gameplay smoke test for melee, ranged/thrown, and monster attacks.

**Decisions**
- `M-V`, session-only, initially off; no settings menu or save-file changes.
- Use separate messages in the ordinary game history, not debug files.
- Cover direct player and monster attacks; exclude spells, traps, and other non-direct damage sources.