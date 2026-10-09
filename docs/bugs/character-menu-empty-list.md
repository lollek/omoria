# Character Menu Crashes On Down With No Saves

Status: fixed. Verified by focused Rust unit tests; terminal reproduction has not been run.

In [the character menu](../../src/pregame/menu.rs), pressing `j` previously
computed `characters.len() as u8 - 1`. With no characters, subtraction
underflowed: debug builds panicked, while release builds wrapped and permitted
an invalid selection index. The menu now moves the selection through pure
`usize` logic, keeping it at zero for an empty list and clamping it to the last
entry for non-empty lists. Pressing Enter on an empty list remains guarded.

An empty list occurs with an empty save folder, when all filenames are rejected,
or when the persistence list operation fails and the menu logs the error.
This is a pre-existing navigation bug, not fixed by persistence injection.

To reproduce: open character selection with no selectable saves, then press `j`.
Do not remove personal saves to reproduce it; use isolated test storage.

Unit tests cover empty, single-entry, first-entry, and last-entry lists, as well
as movement in a list with more than 255 entries. Down on an empty list leaves
the index at zero, and selection movement no longer truncates the list length.

No end-to-end terminal coverage is claimed.