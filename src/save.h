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

#endif /* SAVE_H */
