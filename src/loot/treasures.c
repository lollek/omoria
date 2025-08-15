#include "treasures.h"
#include "../constants.h"
#include "../debug.h"
#include "../random.h"
#include "../types.h"
#include "../variables.h"
#include <curses.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h> /* for ftruncate, usleep */

long t_level[MAX_OBJ_LEVEL + 1];

char const *syllables[MAX_SYLLABLES] = {
    "a",    "ab",   "ag",   "aks",  "ala",  "an",   "ankh", "app",  "arg",
    "arze", "ash",  "aus",  "ban",  "bar",  "bat",  "bek",  "bie",  "bin",
    "bit",  "bjor", "blu",  "brd",  "bu",   "byt",  "comp", "con",  "cos",
    "cre",  "dalf", "dan",  "den",  "doe",  "dok",  "eep",  "el",   "eng",
    "er",   "ere",  "erk",  "esh",  "evs",  "fa",   "fid",  "for",  "fri",
    "fu",   "gan",  "gar",  "glen", "gop",  "gre",  "ha",   "he",   "hyd",
    "i",    "ing",  "ion",  "ip",   "ish",  "it",   "ite",  "iv",   "jo",
    "kho",  "kli",  "klis", "la",   "lech", "man",  "mar",  "me",   "mi",
    "mic",  "mik",  "mon",  "mung", "mur",  "naed", "neg",  "nep",  "ner",
    "nes",  "nis",  "nih",  "nin",  "o",    "od",   "ood",  "ook",  "oook",
    "org",  "orn",  "ox",   "oxy",  "pay",  "pet",  "ple",  "plu",  "po",
    "pot",  "prok", "re",   "rea",  "rhov", "ri",   "ro",   "rog",  "rok",
    "rol",  "sa",   "san",  "sat",  "see",  "sef",  "seh",  "shu",  "si",
    "snd",  "sne",  "snik", "sno",  "so",   "sol",  "spam", "sri",  "sta",
    "sun",  "ta",   "taf",  "tem",  "ther", "ti",   "tox",  "trol", "tue",
    "turs", "u",    "ulk",  "um",   "un",   "uni",  "ur",   "val",  "viv",
    "vly",  "vom",  "wah",  "wed",  "werg", "wex",  "whon", "wlf",  "x",
    "yerg", "yp",   "zun"};

static const float obj_std_adj = 1.25;   // Adjust STD per level
static const long obj_std_min = 7;       // Minimum STD
static const long obj_base_magic = 12;   //  Base amount of magic
static const long obj_base_max = 100;    // Max amount of magic
static const long obj_div_special = 11;  //  magic_chance/# = special magic
static const float obj_div_cursed = 1.2; // magic_chance/# = cursed items

static long mt__m_bonus(const long base, const long max_std, const long level,
                        bool forceit) {
  /*{ Enchant a bonus based on degree desired -RAK- }*/
  long stand_dev = trunc(obj_std_adj * level) + obj_std_min;

  if (stand_dev > max_std) {
    stand_dev = max_std;
  }

  const long x = trunc(labs(randnor(0, stand_dev)) / 10.0) + base;

  if (x < base) {
    return base;
  }
  return x;
}

static void mt__gloves_and_gauntlets(treasure_type *treasure_ptr,
                                     const long level, const bool is_magic,
                                     const bool is_special,
                                     const bool is_cursed,
                                     const bool forceit) {
  if (is_magic) {
    treasure_ptr->toac = mt__m_bonus(1, 20, level, forceit);
    if (is_special) {
      /* gloves_and_gauntlets 5 is "Set of Cloth Gloves" */
      if (treasure_ptr->subval == 5 && randint(10) == 1) {
        strcat(treasure_ptr->name, " of the Hive");
        treasure_ptr->flags |= Dexterity_worn_bit;
        treasure_ptr->p1 = 2;
        treasure_ptr->cost += 50000;
      } else {
        switch (randint(5)) {
        case 1:
          strcat(treasure_ptr->name, " of Free Action");
          treasure_ptr->flags |= Free_Action_worn_bit;
          treasure_ptr->cost += 100000;
          break;
        case 2:
          strcat(treasure_ptr->name, " of Slaying");
          treasure_ptr->tohit = 1 + randint(3);
          treasure_ptr->todam = 1 + randint(3);
          treasure_ptr->cost +=
              (treasure_ptr->tohit + treasure_ptr->todam) * 25000;
          break;
        case 3:
          strcat(treasure_ptr->name, " of Thievery (%P1)");
          treasure_ptr->flags2 |= Disarm_worn_bit;
          treasure_ptr->flags |= Feather_Fall_worn_bit | See_Invisible_worn_bit;
          treasure_ptr->p1 = mt__m_bonus(5, 50, level, forceit);
          treasure_ptr->cost += 20000 + treasure_ptr->p1 * 5;
          break;
        case 4:
        case 5:
          strcat(treasure_ptr->name, " of Ogre Power");
          treasure_ptr->flags |= Slow_Digestion_worn_bit | Strength_worn_bit;
          treasure_ptr->p1 = randint(4);
          treasure_ptr->cost += 150000;
          break;
        }
      }
    }
  } else if (is_cursed) {
    if (is_special) {
      switch (randint(3)) {
      case 1:
        strcat(treasure_ptr->name, " of Clumsiness");
        treasure_ptr->flags |= Cursed_worn_bit | Dexterity_worn_bit;
        treasure_ptr->p1 = 1;
        break;
      case 2:
        strcat(treasure_ptr->name, " of Weakness");
        treasure_ptr->flags |= Cursed_worn_bit | Strength_worn_bit;
        treasure_ptr->p1 = 1;
        break;
      case 3:
        strcat(treasure_ptr->name, " of Ogre Intelligence");
        treasure_ptr->flags |= Cursed_worn_bit | Intelligence_worn_bit;
        treasure_ptr->p1 = 1;
        break;
      }
    }
    treasure_ptr->flags |= Cursed_worn_bit;
    treasure_ptr->toac = -mt__m_bonus(1, 40, level, forceit);
    treasure_ptr->p1 = -mt__m_bonus(1, 10, level, forceit);
    treasure_ptr->cost = 0;
  }
}

static void mt__boots(treasure_type *treasure_ptr, const long level,
                      const bool is_magic, const bool is_special,
                      const bool is_cursed, const bool forceit) {
  if (is_magic) {
    treasure_ptr->toac = mt__m_bonus(1, 20, level, forceit);
    if (is_special) {
      switch (randint(16)) {
      case 1:
        strcat(treasure_ptr->name, " of Speed");
        treasure_ptr->flags |= Speed_worn_bit;
        treasure_ptr->p1 = 1;
        treasure_ptr->cost += 500000;
        break;
      case 2:
      case 3:
      case 4:
      case 5:
        strcat(treasure_ptr->name, " of Stealth");
        treasure_ptr->flags |= Stealth_worn_bit;
        treasure_ptr->cost += 50000;
        break;
      default:
        if (treasure_ptr->subval == 4 && randint(6) == 1) {
          /* Pair of Sandals */
          strcat(treasure_ptr->name, " of Dryadkind");
          treasure_ptr->flags |= Charisma_worn_bit | Feather_Fall_worn_bit |
                                 See_Invisible_worn_bit | Free_Action_worn_bit;
          treasure_ptr->flags2 |= Magic_proof_worn_bit;
          treasure_ptr->p1 = 3;
          treasure_ptr->cost = 1; /*{see magi item}*/
          break;
        }
      }
    } else {
      strcat(treasure_ptr->name, " of Slow descent");
      treasure_ptr->flags |= Feather_Fall_worn_bit;
      treasure_ptr->cost += 25000;
    }
  } else if (is_cursed) {
    treasure_ptr->cost = 0;
    treasure_ptr->ac = -mt__m_bonus(2, 45, level, forceit);

    switch (randint(3)) {
    case 1:
      strcat(treasure_ptr->name, " of Slowness");
      treasure_ptr->flags |= Cursed_worn_bit | Speed_worn_bit;
      treasure_ptr->p1 = -1;
      break;
    case 2:
      strcat(treasure_ptr->name, " of Noise");
      treasure_ptr->flags |= Cursed_worn_bit | Aggravation_worn_bit;
      break;
    case 3:
      strcat(treasure_ptr->name, " of Great Mass");
      treasure_ptr->flags |= Cursed_worn_bit;
      treasure_ptr->weight *= 5;
      break;
    }
  }
}

static void mt__helms(treasure_type *treasure_ptr, const long level,
                      const bool is_magic, const bool is_special,
                      const bool is_cursed, const bool forceit) {
  if (is_magic) {
    treasure_ptr->toac = mt__m_bonus(1, 20, level, forceit);
    if (is_special)
      switch (treasure_ptr->subval) {
      case 1:  case 2:  case 3:  case 4:  case 5:
      case 12: case 13: case 14: case 15:
      case 16: case 17: case 18:
        switch (randint(3)) {
        case 1:
          strcat(treasure_ptr->name, " of Intelligence");
          treasure_ptr->flags |= Intelligence_worn_bit;
          treasure_ptr->p1 = randint(2);
          treasure_ptr->cost += treasure_ptr->p1 * 50000;
          break;
        case 2:
          strcat(treasure_ptr->name, " of Wisdom");
          treasure_ptr->flags |= Wisdom_worn_bit;
          treasure_ptr->p1 = randint(2);
          treasure_ptr->cost += treasure_ptr->p1 * 50000;
          break;
        case 3:
          strcat(treasure_ptr->name, " of Infra-Vision");
          treasure_ptr->flags |= Infra_Vision_worn_bit;
          treasure_ptr->p1 = 1 + randint(4);
          treasure_ptr->cost += treasure_ptr->p1 * 25000;
          break;
        }
        break;

      case 6:  case 7:  case 8:  case 9:  case 10:
      case 19: case 20: case 21: case 22: case 23:
        switch (randint(6)) {
        case 1:
          strcat(treasure_ptr->name, " of Might");
          treasure_ptr->flags |= Free_Action_worn_bit | Constitution_worn_bit |
                                 Strength_worn_bit | Dexterity_worn_bit;
          treasure_ptr->p1 = randint(3);
          treasure_ptr->cost += 100000 + treasure_ptr->p1 * 50000;
          break;
        case 2:
          strcat(treasure_ptr->name, " of Lordliness");
          treasure_ptr->flags |= Wisdom_worn_bit | Charisma_worn_bit;
          treasure_ptr->p1 = randint(3);
          treasure_ptr->cost += 100000 + treasure_ptr->p1 * 50000;
          break;
        case 3:
          strcat(treasure_ptr->name, " of the Magi");
          treasure_ptr->flags |= Free_Action_worn_bit | Strength_worn_bit |
                                 Constitution_worn_bit | Dexterity_worn_bit;
          treasure_ptr->p1 = randint(3);
          treasure_ptr->cost += 300000 + treasure_ptr->p1 * 50000;
          break;
        case 4:
          strcat(treasure_ptr->name, " of Beauty");
          treasure_ptr->flags |= Charisma_worn_bit;
          treasure_ptr->p1 = randint(3);
          treasure_ptr->cost += 75000;
          break;
        case 5:
          strcat(treasure_ptr->name, " of Seeing");
          treasure_ptr->flags |= See_Invisible_worn_bit | Searching_worn_bit;
          treasure_ptr->p1 = 1 + randint(4);
          treasure_ptr->cost += 100000 + treasure_ptr->p1 * 10000;
          break;
        case 6:
          strcat(treasure_ptr->name, " of Regeneration");
          treasure_ptr->flags |= Regeneration_worn_bit;
          treasure_ptr->cost += 150000;
          break;
        }
        break;
      case 11:
        strcat(treasure_ptr->name, " of Hobbitkind");
        treasure_ptr->flags |= Infra_Vision_worn_bit | See_Invisible_worn_bit |
                               Free_Action_worn_bit | Searching_worn_bit;
        treasure_ptr->p1 = 5;
        treasure_ptr->cost += 170000;
        break;
      }
  } else if (is_cursed) {
    treasure_ptr->flags |= Cursed_worn_bit;
    treasure_ptr->toac = -mt__m_bonus(1, 45, level, forceit);
    treasure_ptr->cost = 0;
    if (is_special) {
      switch (randint(15)) {
      case 1:
      case 2:
        strcat(treasure_ptr->name, " of Stupidity");
        treasure_ptr->flags |= Intelligence_worn_bit;
        treasure_ptr->p1 = -1;
        break;
      case 3:
      case 4:
        strcat(treasure_ptr->name, " of Dullness");
        treasure_ptr->flags |= Wisdom_worn_bit;
        treasure_ptr->p1 = -1;
        break;
      case 5:
      case 6:
        strcat(treasure_ptr->name, " of Blindness");
        treasure_ptr->flags |= Blindness_worn_bit;
        break;
      case 7:
      case 8:
        strcat(treasure_ptr->name, " of Timidness");
        treasure_ptr->flags |= Timidness_worn_bit;
        break;
      case 9:
      case 10:
        strcat(treasure_ptr->name, " of Weakness");
        treasure_ptr->flags |= Strength_worn_bit;
        treasure_ptr->p1 = -1;
        break;
      case 11:
      case 12:
        strcat(treasure_ptr->name, " of Teleportation");
        treasure_ptr->flags |= Teleportation_worn_bit;
        break;
      case 13:
      case 14:
        strcat(treasure_ptr->name, " of Ugliness");
        treasure_ptr->flags |= Charisma_worn_bit;
        treasure_ptr->p1 = -1;
        break;
      case 15:
        strcat(treasure_ptr->name, " of **TOTAL DOOM**");
        treasure_ptr->flags |=
            Cursed_worn_bit | Strength_worn_bit | Dexterity_worn_bit |
            Constitution_worn_bit | Intelligence_worn_bit | Wisdom_worn_bit |
            Charisma_worn_bit | Stealth_worn_bit | Aggravation_worn_bit |
            Teleportation_worn_bit | Blindness_worn_bit | Timidness_worn_bit;
        treasure_ptr->flags2 |= Hunger_worn_bit | Known_cursed_bit;
        treasure_ptr->p1 = -5;
        break;
      }
      treasure_ptr->p1 *= randint(5);
    }
  }
}

static void mt__belt(treasure_type *treasure_ptr, const long level,
                     const bool is_magic, const bool is_special,
                     const bool is_cursed, const bool forceit) {
  if (is_magic) {
    treasure_ptr->toac = mt__m_bonus(1, 20, level, forceit);
    if (is_special) {
      switch (treasure_ptr->subval) {

      case 1: /* Girdle */
        treasure_ptr->flags2 |= Increase_carry_worn_bit;

        switch (randint(16)) {
        case 1:
          if (randint(3) == 1) {
            strcat(treasure_ptr->name, " of Titan Strength");
            treasure_ptr->flags |= Resist_Lightning_worn_bit |
                                   Resist_Fire_worn_bit | Resist_Cold_worn_bit |
                                   Resist_Acid_worn_bit |
                                   Regeneration_worn_bit | Free_Action_worn_bit;
            treasure_ptr->flags2 |= Magic_proof_worn_bit;
            treasure_ptr->p1 = 7;
            treasure_ptr->cost += 7500000;
          } else {
            strcat(treasure_ptr->name, " of Storm Giant Strength");
            treasure_ptr->flags |=
                Resist_Lightning_worn_bit | Resist_Acid_worn_bit;
            treasure_ptr->flags2 |= Magic_proof_worn_bit;
            treasure_ptr->p1 = 6;
            treasure_ptr->cost += 3500000;
          }
          break;
        case 2:
          strcat(treasure_ptr->name, " of Cloud Giant Strength");
          treasure_ptr->flags |=
              Resist_Lightning_worn_bit | Resist_Acid_worn_bit;
          treasure_ptr->p1 = 5;
          treasure_ptr->cost += 2000000;
          break;
        case 3:
        case 4:
          strcat(treasure_ptr->name, " of Fire Giant Strength");
          treasure_ptr->flags |= Resist_Fire_worn_bit;
          treasure_ptr->p1 = 4;
          treasure_ptr->cost += 1750000;
          break;
        case 5:
        case 6:
        case 7:
          strcat(treasure_ptr->name, " of Frost Giant Strength");
          treasure_ptr->flags |= Resist_Cold_worn_bit;
          treasure_ptr->p1 = 3;
          treasure_ptr->cost += 1250000;
          break;
        case 8:
        case 9:
        case 10:
        case 11:
          strcat(treasure_ptr->name, " of Stone Giant Strength");
          treasure_ptr->p1 = 2;
          treasure_ptr->cost += 800000;
          break;
        case 12:
        case 13:
        case 14:
        case 15:
        case 16:
          strcat(treasure_ptr->name, " of Hill Giant Strength");
          treasure_ptr->p1 = 1;
          treasure_ptr->cost += 600000;
          break;
        }
        treasure_ptr->tohit = treasure_ptr->p1;
        treasure_ptr->todam = treasure_ptr->p1;
        break;

      case 10: /* Silver Belt Buckle, Gold Belt Buckle */
      case 11:
        switch (randint(2)) {
        case 1:
          strcat(treasure_ptr->name, " of Deflection");
          treasure_ptr->flags2 |= Magic_proof_worn_bit;
          treasure_ptr->toac += randint(5);
          treasure_ptr->cost += treasure_ptr->toac * 20000;
          break;
        case 2:
          strcat(treasure_ptr->name, " of Improved Digestion");
          treasure_ptr->flags |=
              Sustain_Stat_worn_bit | Slow_Digestion_worn_bit;
          treasure_ptr->p1 = 2;
          treasure_ptr->cost += 100000;
          break;
        }
        break;

      case 13: /* Leather Belt */
        strcat(treasure_ptr->name, " of Dwarvenkind");
        treasure_ptr->flags |=
            Infra_Vision_worn_bit | Tunneling_worn_bit | Sustain_Stat_worn_bit;
        treasure_ptr->flags2 |= Magic_proof_worn_bit;
        treasure_ptr->p1 = 2;
        treasure_ptr->cost += 70000;
        break;
      }
    }
  } else if (is_cursed) {
    treasure_ptr->flags |= Cursed_worn_bit;
    treasure_ptr->toac = -mt__m_bonus(1, 45, level, forceit);
    treasure_ptr->cost = 0;
    if (is_special) {
      switch (treasure_ptr->subval) {
      case 1: /* Girdle */
        switch (randint(2)) {
        case 1:
          strcat(treasure_ptr->name, " of Sex Change");
          treasure_ptr->flags |= Charisma_worn_bit;
          treasure_ptr->p1 = -2;
          break;
        case 2:
          strcat(treasure_ptr->name, " of Weakness");
          treasure_ptr->flags |= Strength_worn_bit;
          treasure_ptr->p1 = -1;
          break;
        }
        break;

      case 10: /* Silver Belt Buckle, Gold Belt Buckle */
      case 11:
        strcat(treasure_ptr->name, " of Fear");
        treasure_ptr->flags |= Cursed_worn_bit | Timidness_worn_bit;
        treasure_ptr->p1 = -1;
        break;
      case 13: /* Leather Belt */
        strcat(treasure_ptr->name, " of Hunger");
        treasure_ptr->flags |= Cursed_worn_bit;
        treasure_ptr->flags2 |= Hunger_worn_bit;
        treasure_ptr->p1 = -1;
        break;
      }
      treasure_ptr->p1 *= randint(5);
    }
  }
}

static void mt__ring(treasure_type *treasure_ptr, const long level,
                     const bool is_special, const bool is_cursed,
                     const bool forceit) {
  switch (treasure_ptr->subval) {
  case 1: /* stat rings */
  case 2:
  case 3:
  case 4:
  case 5:
  case 6:
    if (is_cursed) {
      treasure_ptr->flags |= Cursed_worn_bit;
      treasure_ptr->p1 = -mt__m_bonus(1, 20, level, forceit);
      treasure_ptr->cost *= -1;
    } else {
      treasure_ptr->p1 = mt__m_bonus(1, 10, level, forceit);
      treasure_ptr->cost += treasure_ptr->p1 * 10000;
    }
    break;

  case 7: /* speed */
    if (is_cursed) {
      treasure_ptr->flags |= Cursed_worn_bit;
      treasure_ptr->p1 = -randint(3);
      treasure_ptr->cost *= -1;
    } else {
      treasure_ptr->p1 = 1;
      if (is_special && randint(100) == 1) {
        /* added a rare, faster ring STK */
        treasure_ptr->p1 = 2;
        treasure_ptr->cost *= 10;
      }
    }
    break;

  case 8: /* searching */
    treasure_ptr->p1 = 5 * mt__m_bonus(1, 20, level, forceit);
    treasure_ptr->cost += treasure_ptr->p1 * 10000;
    break;

  case 22: /* Increase damage */
    treasure_ptr->todam = mt__m_bonus(1, 20, level, forceit);
    treasure_ptr->cost += treasure_ptr->todam * 10000;
    if (is_cursed) {
      treasure_ptr->flags |= Cursed_worn_bit;
      treasure_ptr->todam *= -1;
      treasure_ptr->cost *= -1;
    }
    break;

  case 23: /* Increase To-Hit */
    treasure_ptr->tohit = mt__m_bonus(1, 20, level, forceit);
    treasure_ptr->cost += treasure_ptr->tohit * 10000;
    if (is_cursed) {
      treasure_ptr->flags |= Cursed_worn_bit;
      treasure_ptr->tohit *= -1;
      treasure_ptr->cost *= -1;
    }
    break;

  case 24: /* Protection */
    treasure_ptr->toac = mt__m_bonus(1, 20, level, forceit);
    treasure_ptr->cost += treasure_ptr->toac * 10000;
    if (is_cursed) {
      treasure_ptr->flags |= Cursed_worn_bit;
      treasure_ptr->toac *= -1;
      treasure_ptr->cost *= -1;
    }
    break;

  case 33: /* Slaying */
    treasure_ptr->todam = mt__m_bonus(1, 25, level, forceit);
    treasure_ptr->tohit = mt__m_bonus(1, 25, level, forceit);
    treasure_ptr->cost += (treasure_ptr->tohit + treasure_ptr->todam) * 10000;
    if (is_cursed) {
      treasure_ptr->flags |= Cursed_worn_bit;
      treasure_ptr->tohit *= -1;
      treasure_ptr->todam *= -1;
      treasure_ptr->cost *= -1;
    }
    break;

  case 35: /* Speed -10 or worse */
    treasure_ptr->p1 = -(10 + randint(10));
    treasure_ptr->cost += 1000000 * treasure_ptr->p1;
    if (is_cursed) {
      treasure_ptr->flags |= Cursed_worn_bit;
      treasure_ptr->flags2 &= ~Known_cursed_bit;
    }
    break;

  default:
    break;
  }
}

static void mt__amulet(treasure_type *treasure_ptr, const long level,
                       const bool is_cursed, const bool forceit) {
  switch (treasure_ptr->subval) {
  case 1: /* 1,2,3,4 not found, 5=wisdom, 6=charisma */
  case 2: /* I assume the others are for other stats */
  case 3:
  case 4:
  case 5:
  case 6:
    if (is_cursed) {
      treasure_ptr->flags |= Cursed_worn_bit;
      treasure_ptr->p1 = -mt__m_bonus(1, 20, level, forceit);
      treasure_ptr->cost *= -1;
    } else {
      treasure_ptr->p1 = mt__m_bonus(1, 10, level, forceit);
      treasure_ptr->cost += treasure_ptr->p1 * 10000;
    }
    break;

  case 7: /* Searching */
    treasure_ptr->p1 = 5 * mt__m_bonus(1, 25, level, forceit);
    if (is_cursed) {
      treasure_ptr->flags |= Cursed_worn_bit;
      treasure_ptr->p1 *= -1;
      treasure_ptr->cost *= -1;
    } else {
      treasure_ptr->cost += 10000 * treasure_ptr->p1;
    }
    break;

  default:
    break;
  }
}

static void mt__horn(treasure_type *treasure_ptr, const bool forceit) {
  long p1;

  switch (treasure_ptr->subval) {
  case 1:
    p1 = randint(10) + 6;
    break;
  case 2:
    p1 = randint(6) + 3;
    break;
  case 3:
    p1 = randint(5) + 6;
    break;
  case 4:
    p1 = randint(3) + 1;
    break;
  case 5:
    p1 = randint(3) + 4;
    break;
  case 6:
    p1 = randint(3) + 4;
    break;
  case 7:
    p1 = randint(3) + 4;
    break;
  case 8:
    p1 = randint(10) + 3;
    break;
  case 9:
    p1 = randint(5) + 1;
    break;
  case 10:
    p1 = randint(3) + 1;
    break;
  case 11:
    p1 = randint(3) + 4;
    break;
  case 12:
    p1 = randint(3) + 4;
    break;
  case 13:
    p1 = randint(8) + 1;
    break;
  default:
    MSG(("WARNING: Unknown subval in mt__horn: %ld  %s", treasure_ptr->subval,
         treasure_ptr->name));
    p1 = 0;
    break;
  }

  if (forceit) {
    p1 += 5;
  }
  treasure_ptr->p1 = p1;
}

static void mt__cloak(treasure_type *treasure_ptr, const long level,
                      const bool is_magic, const bool is_special,
                      const bool is_cursed, const bool forceit) {
  if (is_magic) {
    if (is_special) {
      switch (randint(9)) {
      case 1:
      case 2:
      case 3:
      case 4:
        strcat(treasure_ptr->name, " of Protection");
        treasure_ptr->toac = mt__m_bonus(2, 40, level, forceit);
        treasure_ptr->cost += 25000 + treasure_ptr->toac * 10000;
        break;
      case 5:
      case 6:
      case 7:
      case 8:
        strcat(treasure_ptr->name, " of Stealth (%P1)");
        treasure_ptr->flags |= Stealth_worn_bit;
        treasure_ptr->toac = mt__m_bonus(1, 20, level, forceit);
        treasure_ptr->p1 = randint(3);
        treasure_ptr->cost +=
            treasure_ptr->p1 * 50000 + treasure_ptr->toac * 10000;
        break;
      case 9:
        strcat(treasure_ptr->name, " of Elvenkind");
        treasure_ptr->flags |= See_Invisible_worn_bit | Sustain_Stat_worn_bit |
                               Stealth_worn_bit | Charisma_worn_bit;
        treasure_ptr->p1 = 2;
        treasure_ptr->cost += 200000;
        break;
      } /* end switch */
    } else {
      /* not special */
      treasure_ptr->toac = mt__m_bonus(1, 20, level, forceit);
      treasure_ptr->cost += treasure_ptr->toac + 10000;
    }
  } else if (is_cursed) {
    switch (randint(3)) {
    case 1:
      strcat(treasure_ptr->name, " of Irritation");
      treasure_ptr->flags |= Cursed_worn_bit | Aggravation_worn_bit;
      treasure_ptr->ac = 0;
      treasure_ptr->toac = -mt__m_bonus(1, 10, level, forceit);
      treasure_ptr->tohit = -mt__m_bonus(1, 10, level, forceit);
      treasure_ptr->todam = -mt__m_bonus(1, 10, level, forceit);
      treasure_ptr->cost = 0;
      break;
    case 2:
      strcat(treasure_ptr->name, " of Vulnerability");
      treasure_ptr->flags |= Cursed_worn_bit;
      treasure_ptr->ac = 0;
      treasure_ptr->toac = -mt__m_bonus(10, 100, level + 50, forceit);
      treasure_ptr->cost = 0;
      break;
    case 3:
      strcat(treasure_ptr->name, " of Enveloping");
      treasure_ptr->flags |= Cursed_worn_bit;
      treasure_ptr->toac = -mt__m_bonus(1, 10, level, forceit);
      treasure_ptr->tohit = -mt__m_bonus(2, 40, level + 10, forceit);
      treasure_ptr->todam = -mt__m_bonus(2, 40, level + 10, forceit);
      treasure_ptr->cost = 0;
      break;
    }
  }
}

static void mt__chest(treasure_type *treasure_ptr, const long level) {
  /*
   * Items inside the chest will be created as if
   * found on dungeon level p1.
   */

        /*
         * Items inside the chest will be created as if
         * found on dungeon level p1.
         * treasure_ptr->p1 = max(1, dun_level + randint(10) - 5);
         */

  if (treasure_ptr->subval == 5) {
    /* dead human body */
    strcat(treasure_ptr->name, "^ (Looted)");
  } else {
    switch (randint(level) + 4) {
    case 1:
      strcat(treasure_ptr->name, "^ (Empty)");
      break;
    case 2:
      strcat(treasure_ptr->name, "^ (Locked)");
      treasure_ptr->flags |= 0x00000001;
      break;
    case 3:
    case 4:
      strcat(treasure_ptr->name, "^ (Poison Needle)");
      treasure_ptr->flags |= 0x00000011;
      break;
    case 5:
    case 6:
      strcat(treasure_ptr->name, "^ (Poison Needle)");
      treasure_ptr->flags |= 0x00000021;
      break;
    case 7:
    case 8:
    case 9:
      strcat(treasure_ptr->name, "^ (Gas Trap)");
      treasure_ptr->flags |= 0x00000041;
      break;
    case 10:
    case 11:
      strcat(treasure_ptr->name, "^ (Explosion Device)");
      treasure_ptr->flags |= 0x00000081;
      break;
    case 12:
    case 13:
    case 14:
      strcat(treasure_ptr->name, "^ (Summoning Runes)");
      treasure_ptr->flags |= 0x00000101;
      break;
    case 15:
    case 16:
    case 17:
      strcat(treasure_ptr->name, "^ (Multiple Traps)");
      treasure_ptr->flags |= 0x00000071;
      break;
    default:
      strcat(treasure_ptr->name, "^ (Multiple Traps)");
      treasure_ptr->flags |= 0x00000181;
      break;
    }
  }
}

void magic_treasure(const long x, const long level) {
  treasure_type *treasure_ptr = &t_list[x];
  bool is_magic;   /* Positive enchantment */
  bool is_cursed;  /* Negative enchantment */
  bool is_special; /* Enchantment is stronger than usual */

  /*
   * Depending on treasure type, it can have certain magical
   * properties
   */

  switch (treasure_ptr->tval) {
  case gloves_and_gauntlets:
    mt__gloves_and_gauntlets(treasure_ptr, level, is_magic, is_special,
                             is_cursed, false);
    break;

  case boots:
    mt__boots(treasure_ptr, level, is_magic, is_special, is_cursed, false);
    break;

  case helm:
    mt__helms(treasure_ptr, level, is_magic, is_special, is_cursed, false);
    break;

  /*{girdles, belts and buckles}*/
  case belt:
    mt__belt(treasure_ptr, level, is_magic, is_special, is_cursed, false);
    break;

  case ring:
    mt__ring(treasure_ptr, level, is_special, is_cursed, false);
    break;

  case amulet:
    mt__amulet(treasure_ptr, level, is_cursed, false);
    break;

  case horn:
    mt__horn(treasure_ptr, false);
    break;

  case cloak:
    mt__cloak(treasure_ptr, level, is_magic, is_special, is_cursed, false);
    break;

  case chest:
    mt__chest(treasure_ptr, level);
    break;
  }
}
