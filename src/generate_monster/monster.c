#include "../creature.h"
#include "../generate_monster.h"
#include "../io.h"
#include "../misc.h"
#include "../monsters.h"
#include "../pascal.h"
#include "../player.h"
#include "../random.h"
#include "../variables.h"
#include "monster_template.h"
#include <stdlib.h>
#include <string.h>
#include <sys/param.h>

// One in X to spawn an out-of-depth monster
static const long out_of_depth_chance = 50;

static void generate_monster(obj_set alloc_set, const long number_of_monsters,
                             const long min_distance_from_player,
                             const bool is_sleeping,
                             const bool water_monster) {
  long count = 0;
  long count2 = 0;

  for (int i = 0; i < number_of_monsters; i++) {
    long y;
    long x;
    do {
      y = randint(cur_height - 2) + 1;
      x = randint(cur_width - 2) + 1;
      if (++count2 > 7500)
        break;
    } while (!(is_in(cave[y][x].fval, alloc_set) && cave[y][x].cptr == 0 &&
               cave[y][x].fopen &&
               distance(y, x, char_row, char_col) > min_distance_from_player));

    while (count++ <= 10) {

      long monster_i;
      if (dun_level == 0) {
        monster_i = randint(m_level[0]);
      } else if (dun_level > MAX_MONS_LEVEL) {
        monster_i = randint(m_level[MAX_MONS_LEVEL]) + m_level[0];
      } else if (randint(out_of_depth_chance) == 1) {
        monster_i = dun_level + labs(randnor(0, 4)) + 1;
        if (monster_i > MAX_MONS_LEVEL) {
          monster_i = MAX_MONS_LEVEL;
        }
        const long i3 = m_level[monster_i] - m_level[monster_i - 1];
        monster_i = randint(i3) + m_level[monster_i - 1];
      } else {
        monster_i = randint(m_level[dun_level]) + m_level[0];
      }

      monster_template const *template = &monster_templates[monster_i];
      if (monster_template_has_attribute(template, ma_unspawnable)) {
        continue;
      }

      bool ok_monster_found;
      if (!water_monster) {
        ok_monster_found =
            monster_template_has_attribute(template, ma_land_based) ||
            !monster_template_has_attribute(template,
                                            ma_dies_in_wrong_element) ||
            monster_template_has_attribute(template, ma_flying);
      } else {
        ok_monster_found =
            monster_template_has_attribute(template, ma_water_based);
      }

      if (!ok_monster_found) {
        continue;
      }

      if (count2 < 7500) {
        place_monster(y, x, monster_i, is_sleeping);
      }
      break;
    }
  }
}

void generate_land_monster(obj_set alloc_set, const long number_of_monsters,
                           const long min_distance_from_player,
                           const bool is_sleeping) {
  generate_monster(alloc_set, number_of_monsters, min_distance_from_player,
                   is_sleeping, false);
}

void generate_water_monster(obj_set alloc_set, const long number_of_monsters,
                            const long min_distance_from_player,
                            const bool is_sleeping) {
  generate_monster(alloc_set, number_of_monsters, min_distance_from_player,
                   is_sleeping, true);
}

void place_monster(const long y, const long x, const long template,
                   const bool is_asleep) {

  long cur_pos;
  popm(&cur_pos);
  m_list[cur_pos].fy = y;
  m_list[cur_pos].fx = x;
  m_list[cur_pos].mptr = template;
  m_list[cur_pos].nptr = muptr;
  muptr = cur_pos;

  monster_template const *monster = &monster_templates[template];

  if ((monster->cdefense & 0x4000) != 0) {
    m_list[cur_pos].hp = max_hp(monster->hd);
  } else {
    m_list[cur_pos].hp = damroll(monster->hd);
  }

  m_list[cur_pos].cdis = distance(char_row, char_col, y, x);
  m_list[cur_pos].cspeed = monster->speed + player_flags.speed;
  m_list[cur_pos].stunned = 0;

  if (is_asleep) {
    m_list[cur_pos].csleep = monster->sleep / 5.0 + randint(monster->sleep);
  } else {
    m_list[cur_pos].csleep = 0;
  }

  cave[y][x].cptr = cur_pos;
}

/**
 * @brief Summon a monster and place it at [y, x] coordinates or just besides
 * it. Note that this function will never spawn a monster with the flag
 * ma_unspawnable.
 *
 * @param y                   Coordinate to place monster, will be modified to
 * the real y values where the monster was placed
 * @param x                   Coordinate to place monster, will be modified to
 * the real y values where the monster was placed
 * @param is_asleep           Should the monster spawn asleep?
 * @param fval_set            obj_set of cave floor values
 * acceptable for spawning the monster on
 * @param monster_attributes  Null terminated list of monster attributes which
 * must apply to spawn the monster.
 * @return Was the monster successfully spawned?
 */
static bool summon_monster(int64_t *y, int64_t *x, const bool is_asleep,
                              obj_set const fval_set,
                              monster_attribute const *const *monster_attributes) {
  long const max_monster_level =
      MIN(dun_level + MON_SUMMON_ADJ, MAX_MONS_LEVEL);

  // 10 attemps to spawn a monster
  for (int i = 0; i < 10; ++i) {
    const long monster_y = *y - 2 + randint(3);
    const long monster_x = *x - 2 + randint(3);

    // Position is outside the dungeon
    if (!in_bounds(monster_y, monster_x)) {
      continue;
    }

    // See if we match a floor type
    if (!is_in(cave[monster_y][monster_x].fval, fval_set)) {
      continue;
    }

    // Is the position occupied by a monster?
    if (cave[monster_y][monster_x].cptr != 0) {
      continue;
    }

    // is the position occupied by walls, etc?
    if (!cave[monster_y][monster_x].fopen) {
      continue;
    }

    for (int counter = 0; counter < 11; counter++) {
      // Town level has separate creatures from the rest of the levels
      long monster_i = dun_level == 0
                           ? randint(m_level[0])
                           : randint(m_level[max_monster_level]) + m_level[0];

      if (monster_i > MAX_CREATURES) {
        monster_i = MAX_CREATURES;
      }

      monster_template const *template = &monster_templates[monster_i];
      if (monster_template_has_attribute(template, ma_unspawnable)) {
        continue;
      }

      if (!monster_template_has_attributes(template, monster_attributes)) {
        continue;
      }

      place_monster(monster_y, monster_x, monster_i, is_asleep);
      *y = monster_y;
      *x = monster_x;
      return true;
    }
  }
  return false;
}

bool summon_land_monster(int64_t *y, int64_t *x, const bool is_asleep) {
  monster_attribute attr1 = ma_survives_on_land;
  monster_attribute const *attributes[] = {&attr1, NULL};
  return summon_monster(y, x, is_asleep, earth_set, attributes);
}

bool summon_water_monster(int64_t *y, int64_t *x, const bool is_asleep) {
  monster_attribute attr1 = ma_survives_in_water;
  monster_attribute const *attributes[] = {&attr1, NULL};
  return summon_monster(y, x, is_asleep, water_set, attributes);
}

bool summon_undead(long *y, long *x) {
  obj_set const undead_set = {1, 2, 4, 5, 0};
  monster_attribute attr1 = ma_undead;
  monster_attribute const *attributes[] = {&attr1, NULL};
  return summon_monster(y, x, false, undead_set, attributes);
}

bool summon_demon(long *y, long *x) {
  obj_set const demon_set = {1, 2, 4, 5, 0};
  monster_attribute attr1 = ma_demon;
  monster_attribute const *attributes[] = {&attr1, NULL};
  return summon_monster(y, x, false, demon_set, attributes);
}

bool summon_breed(long *y, long *x) {
  // This used to summon the correct breeder for its floor type, but that was
  // such a hassle. Now it's ground only.
  monster_attribute attr1 = ma_multiplies;
  monster_attribute const *attributes[] = {&attr1, NULL};
  return summon_monster(y, x, false, earth_set, attributes);
}

void monster_summon_by_name(long y, long x, char name[28],
                            const bool present, const bool is_asleep) {

  long i2;
  char monster[28];
  bool junk;

  if (!present) {
    prt("Monster desired:  ", 1, 1);
    junk = get_string(monster, 1, 19, 26);
  } else {
    strcpy(monster, name);
    junk = true;
  }

  if (junk) {
    long i4;
    long i3;
    long i1 = 0;
    i2 = 0;
    sscanf(monster, "%ld", &i2);
    if (i2 < 0) {
      i2 = 1;
    }
    if (i2 > MAX_CREATURES) {
      i2 = MAX_CREATURES;
    }

    if (i2 > 0 && i2 <= MAX_CREATURES) {
      /* summon by number */
      i1 = 0;
      do {
        i3 = y - 2 + randint(3);
        i4 = x - 2 + randint(3);
        if (in_bounds(i3, i4)) {
          /*with cave[i3,i4] do*/
          if (is_in(cave[i3][i4].fval, floor_set)) {
            if (cave[i3][i4].fopen) {
              place_monster(i3, i4, i2, is_asleep);
              i1 = 8;
              y = i3;
              x = i4;
            }
          }
        }
        i1++;
      } while (i1 <= 8);
    } else {
      /* find by name, then summon */
      for (i2 = 1; i2 <= MAX_CREATURES; i2++) {
        if (strstr(monster_templates[i2].name, monster) != NULL &&
            i1 != 10) {
          i1 = 0;
          do {
            i3 = y - 2 + randint(3);
            i4 = x - 2 + randint(3);
            if (in_bounds(i3, i4)) {
              /*with cave[i3,i4] do*/
              if (is_in(cave[i3][i4].fval, floor_set)) {
                if (cave[i3][i4].cptr == 0) {
                  if (cave[i3][i4].fopen) {
                    place_monster(i3, i4, i2, is_asleep);
                    i1 = 9;
                    y = i3;
                    x = i4;
                  }
                }
              }
            }
            i1++;
          } while (i1 <= 9);
        }
      }
    } /* end else */
  }   /* end if junk */

  if (!present) {
    erase_line(msg_line, msg_line);
  }
}

void multiply_monster(const long y, const long x, const long template,
                      const bool is_asleep) {

  long i1 = 0;

  do {
    const long i2 = y - 2 + randint(3);
    const long i3 = x - 2 + randint(3);

    if (in_bounds(i2, i3)) {
      if (is_in(cave[i2][i3].fval, floor_set)) {
        if (cave[i2][i3].tptr == 0 && cave[i2][i3].cptr != 1) {
          if (cave[i2][i3].cptr > 1) { /* { Creature there already?  }*/
            /*{ Some critters are * canabalistic!       }*/
            if ((monster_templates[template].cmove & 0x00080000) != 0) {
              delete_monster(cave[i2][i3].cptr);
              place_monster(i2, i3, template, is_asleep);
              check_mon_lite(i2, i3);
              mon_tot_mult++;
            }
          } else {
            /*{ All clear, place a monster * }*/
            place_monster(i2, i3, template, is_asleep);
            check_mon_lite(i2, i3);
            mon_tot_mult++;
          }
          i1 = 18;
        }
        i1++;
      }
    }
  } while (i1 <= 18);
}