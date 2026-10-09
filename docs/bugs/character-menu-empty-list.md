# Character Menu Crashes On Down With No Saves

Status: fixed in code; terminal reproduction has not been run.

In [the character menu](../../src/pregame/menu.rs), pressing `j` computes
`characters.len() as u8 - 1`. With no characters, subtraction underflows:
debug builds panic, while release builds wrap and permit an invalid selection
index. Pressing Enter on an empty list is already guarded.

An empty list occurs with an empty save folder, when all filenames are rejected,
or when the persistence list operation fails and the menu logs the error.
This is a pre-existing navigation bug, not fixed by persistence injection.

To reproduce: open character selection with no selectable saves, then press `j`.
Do not remove personal saves to reproduce it; use isolated test storage.

Selection movement is extracted into `next_index` and `previous_index` in
[the character menu](../../src/pregame/menu.rs) and tested for empty,
single-entry, first and last entries, and lists larger than 255 entries.
Down on an empty list leaves the index at zero.

No end-to-end terminal coverage is claimed.