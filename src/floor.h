#pragma once

#include "variables.h"

typedef struct floor_type {
  uint8_t ftval;
  bool ftopen;
} floor_type;

enum floor_types {
  ft_blank_floor,
  ft_dark_open_floor,
  ft_light_open_floor,
  ft_unknown_3,
  ft_corr_open_floor,
  ft_corr_room_junction,
  ft_corr_door,
  ft_corr_secret_door,
  ft_unknown_8,
  ft_unknown_9,
  ft_wall_granite,
  ft_wall_magma,
  ft_wall_quartz,
  ft_unknown_13,
  ft_unknown_14,
  ft_boundry_wall,
  ft_water_on_floor,
  ft_water_on_room_floor,
  ft_water_on_floor_lit,
};

extern cave_type blank_floor;
extern const floor_type dark_open_floor;
extern const floor_type light_open_floor;
extern const floor_type corr_open_floor;
extern const floor_type corr_room_junction;
extern const floor_type corr_door;
extern const floor_type corr_secret_door;
extern const floor_type wall_granite;
extern const floor_type wall_magma;
extern const floor_type wall_quartz;
extern const floor_type water_on_floor;
extern const floor_type water_on_room_floor;
extern const floor_type water_on_floor_lit;
extern const floor_type boundry_wall;
