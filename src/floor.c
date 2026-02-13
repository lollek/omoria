#include "floor.h"

cave_type blank_floor = {0, 0, 0, false, false, false, false, false, 0, 0};

floor_type const dark_open_floor = {ft_dark_open_floor, true};
floor_type const light_open_floor = {ft_light_open_floor, true};
floor_type const corr_open_floor = {ft_corr_open_floor, true};
floor_type const corr_room_junction = {ft_corr_room_junction, true};
floor_type const corr_door = {ft_corr_door, true};
floor_type const corr_secret_door = {ft_corr_secret_door, false};
/*{ Floor values 8 and 9 are used in generate		} */
floor_type const wall_granite = {ft_wall_granite, false};
floor_type const wall_magma = {ft_wall_magma, false};
floor_type const wall_quartz = {ft_wall_quartz, false};
floor_type const boundry_wall = {ft_boundry_wall, false};
floor_type const water_on_floor = {ft_water_on_floor, true};
floor_type const water_on_room_floor = {ft_water_on_room_floor, true};
floor_type const water_on_floor_lit = {ft_water_on_floor_lit, true};

