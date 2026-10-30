#pragma once

#include <curses.h>

#include "types.h"

chtype C_item_get_tchar(const treasure_type *item);

/**
 * @brief Returns the item's map glyph with its color attribute.
 * @param item The item whose glyph should be rendered.
 * @return The item glyph and color attribute for map rendering.
 */
chtype C_item_get_glyph(const treasure_type *item);

/**
 * @brief Returns the terminal-independent semantic color key for an item.
 * @param item Non-null pointer to a valid item whose color role should be resolved.
 * @return The semantic color key used for rendering and stack compatibility.
 */
int C_item_color_key(const treasure_type *item);
