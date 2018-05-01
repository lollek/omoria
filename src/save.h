#ifndef SAVE_H
#define SAVE_H

/**
 * -OK-
 * save_file_remove() - Remove the current save game file
 */
void save_file_remove(void);

/**
 * -OK-
 *  save_file_name_set() - Set file to save to
 *  @path: path to save file
 */
void save_file_name_set(char path[82]);

/**
 * -OK-
 *  save_file_name_is_set() - Is there a save file?
 */
boolean save_file_name_is_set(void);

boolean save_char(boolean quick);
boolean get_char(boolean prop);

/**
 * restore_char() - Wizard command for restoring or reviving a character
 * @fnam: Path to save file
 * @present: If fnam is set?
 * @undead: If the character should be revived
 *
 * This function seems to revive the character in a given path.
 * If fnam is not set, it will try a default one.
 */
void restore_char(char fnam[82], boolean present, boolean undead);

#endif /* SAVE_H */
