#pragma once

#include <curses.h>

/**
 * @brief Returns the attribute bits for the water map color.
 * @return Color pair and bold bits to OR into a glyph, or 0 on terminals without color.
 */
chtype C_map_color_water_attr(void);
