# Character Menu Crashes On Down With No Saves

Status: open. Identified by code inspection; terminal reproduction has not been run.

In [the character menu](../../src/pregame/menu.rs), pressing `j` computes
`characters.len() as u8 - 1`. With no characters, subtraction underflows:
debug builds panic, while release builds wrap and permit an invalid selection
index. Pressing Enter on an empty list is already guarded.

An empty list occurs with an empty save folder, when all filenames are rejected,
or when the persistence list operation fails and the menu logs the error.
This is a pre-existing navigation bug, not fixed by persistence injection.

To reproduce: open character selection with no selectable saves, then press `j`.
Do not remove personal saves to reproduce it; use isolated test storage.

A bounded follow-up should extract selection movement into pure logic and test
empty, single-entry, first-entry, and last-entry lists before changing the menu.
Down on an empty list must leave the index at zero. Also cover lists larger than
255 entries: the current `u8` length/index conversion can truncate them.

No end-to-end terminal coverage is claimed.