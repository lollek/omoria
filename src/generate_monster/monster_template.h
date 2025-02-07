#pragma once

#include <stdbool.h>
#include <stdint.h>

#define MAX_MONS_LEVEL 100 /*{ Maximum level of creatures		} */

typedef enum monster_attribute {
  ma_move_only_to_attack,
  ma_20pc_random_movement,
  ma_40pc_random_movement,
  ma_75pc_random_movement,
  ma_water_based,
  ma_land_based,
  ma_dies_in_wrong_element,
  ma_survives_in_water,
  ma_survives_on_land,
  // Affects reputation if killed
  ma_good_monster,
  // Unspawnable by normal means, but spawnable by events
  ma_unspawnable,
  ma_invisible_movement,
  ma_moves_through_door,
  ma_moves_through_wall,
  ma_moves_through_creatures,
  ma_picks_up_objects,
  ma_multiplies,
  ma_anchors_in_water,
  ma_flying,
  ma_carries_objects,
  ma_carries_gold,
  ma_carries_60pc,
  ma_carries_90pc,
  ma_carries_1d2_things,
  ma_carries_2d2_things,
  ma_carries_4d2_things,
  ma_wins_the_game,
  ma_dragon,  // Hurt by Slay Dragon.
  ma_monster, // Hurt by Slay Monster.
  ma_evil,    // Hurt by Slay Evil
  ma_undead,  // Hurt by Slay Undead
  ma_demon,   // Hurt by Slay Demon
  ma_vulnerable_to_frost,
  ma_vulnerable_to_fire,
  ma_vulnerable_to_poison,
  ma_vulnerable_to_acid,
  ma_vulnerable_to_lightning,
  ma_vulnerable_to_stone_to_mud,
  ma_uncharmable, // Cannot be charmed or slept
  ma_visible_with_infravision,
  ma_max_hit_points,
  ma_regenerates,
} monster_attribute;

typedef struct monster_attributes {
  bool multiplies;
  bool can_move;
} monster_attributes;

typedef struct monster_template {
  uint8_t area_effect_radius;       // Area affect radius
  uint8_t ac;
  char name[28];
  uint64_t cmove;                   // Bit field
  uint64_t spells;                  // Creature spells
  uint64_t cdefense;                // Bit field
  int16_t sleep;                    // Inactive counter when spawned
  int64_t mexp;                     // Exp value for kill
  int8_t speed;                     // Movement speed
  char symbol;                      // Symbol on map
  char hit_die[7];
  char damage[36];                  // Type attack and damage
  int8_t level;                     // Minimum level creature is found at
  uint8_t magic_resistance;
  monster_attributes attributes;
} monster_template;

extern const monster_template monster_templates[];
extern long const monster_template_size;
extern long m_level[MAX_MONS_LEVEL + 1];

bool monster_template_has_attribute(monster_template const *template,
                                       monster_attribute attribute);
bool monster_template_has_attributes(
    monster_template const *template,
    monster_attribute const *const *monster_attributes);
