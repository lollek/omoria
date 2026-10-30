#pragma once

#include <curses.h>

/**
 * @brief Initializes map color pairs for the active terminal.
 * @param color_count The number of colors supported by the terminal.
 */
void C_map_color_init(int color_count);

/**
 * @brief Returns the attribute bits for the water map color.
 * @return Color pair and bold bits to OR into a glyph, or 0 on terminals without color.
 */
chtype C_map_color_water_attr(void);

/**
 * @brief Returns the color attribute for a terrain floor value.
 * @param fval The terrain value from the cave cell.
 * @return Color pair bits, or 0 if the value is unmapped or colors are unavailable.
 */
chtype C_map_color_terrain_attr(unsigned char fval);
