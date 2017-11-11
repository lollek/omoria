#ifndef SAVE_H
#define SAVE_H

boolean save_char(boolean quick);
boolean get_char(vtype fnam, boolean prop);

/**
 * restore_char() - Wizard command for restoring or reviving a character
 * @fnam: Path to save file
 * @present: If fnam is set?
 * @undead: If the character should be revived
 *
 * This function seems to revive the character in a given path.
 * If fnam is not set, it will try a default one.
 */
void restore_char(vtype fnam, boolean present, boolean undead);

#endif /* SAVE_H */
