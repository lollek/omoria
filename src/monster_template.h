#ifndef MONSTER_TEMPLATE_H
#define MONSTER_TEMPLATE_H

#include <stdint.h>

#include "boolean.h"

#define MAX_CREATURES 415  /*{ Number of monster templates defined for univ    \
                              } */
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

typedef struct monster_template {
  uint8_t aaf;       /*: [bit(7),pos(0)] 0..127; Area affect radius */
  uint8_t ac;        /*: [bit(7),pos(8)] 0..127;	 AC */
  char name[28];     /* Descrip of creature */
  uint64_t cmove;    /*: unsigned;	Bit field */
  uint64_t spells;   /*: unsigned;	Creature spells	 */
  uint64_t cdefense; /*: unsigned;	Bit field */
  int16_t sleep;     /* Inactive counter */
  int64_t mexp;      /* Exp value for kill */
  int8_t speed;      /* Movement speed */
  char cchar;        /* Character rep. */
  char hd[7];        /* Creatures hit die */
  char damage[36];   /* Type attack and damage */
  int8_t level;      /* Level of creature */
  uint8_t mr;        /* Magic Resistance */
} monster_template;

extern const monster_template monster_templates[MAX_CREATURES + 1];
extern long const m_level[MAX_MONS_LEVEL + 1];

boolean monster_template_has_attribute(monster_template const *template,
                                       monster_attribute attribute);

#endif // MONSTER_TEMPLATE_H