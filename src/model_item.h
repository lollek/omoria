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
