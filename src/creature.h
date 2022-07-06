#ifndef CREATURE_H
#define CREATURE_H

#include "boolean.h"

#define MAX_CREATURES 415     /*{ Number of monster templates defined for univ	} */
#define MAX_MONS_LEVEL 100    /*{ Maximum level of creatures		} */

extern long m_level[MAX_MONS_LEVEL + 1];

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

extern monster_template c_list[MAX_CREATURES + 1];

/* Returns number of monster in list specified by virtual_name */
long find_mon(const char *virtual_name);

/* Makes sure new creature gets lit up */
void check_mon_lite(long y, long x);

/* Places creature adjacent to given location
 * Rats and Flies are fun!
 */
void multiply_monster(long y, long x, long z, boolean slp);

void creatures(boolean attack);

/* Moves creature record from one space to another.
 *
 * NOTE! (x1,y1) might equal (x2,y2)
 */
void move_rec(long y1, long x1, long y2, long x2);

#endif /* CREATURE_H */