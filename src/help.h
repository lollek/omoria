#pragma once

enum help_page {
  HELP_PAGE_COMMANDS,
  HELP_PAGE_SHOPS,
};

/**
 * @brief Return the number of text lines in a help page.
 * @param page Help page identifier
 * @return Number of lines, or zero for an invalid page
 */
int C_help_page_line_count(int page);

/**
 * @brief Return the terminal row for a line in a help page.
 * @param page Help page identifier
 * @param line Zero-based line index
 * @return Terminal row, or zero for an invalid page or line
 */
int C_help_page_line_row(int page, int line);

/**
 * @brief Return the text for a line in a help page.
 * @param page Help page identifier
 * @param line Zero-based line index
 * @return NUL-terminated static text, or NULL for an invalid page or line
 */
char const *C_help_page_line_text(int page, int line);

/**
 * @brief Look up the description of a map character.
 * @param environment Town ('t'), dungeon ('d'), or water ('w')
 * @param character Character to identify
 * @return NUL-terminated static description, or NULL for '@' or an invalid
 * environment
 */
char const *C_ident_char_description(char environment, char character);

void ident_char();
void help();
void wizard_help();
void moria_help(char const *help_level);
