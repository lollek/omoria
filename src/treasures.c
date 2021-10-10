#include <curses.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h> /* for ftruncate, usleep */

#include "configure.h"
#include "constants.h"
#include "debug.h"
#include "magic.h"
#include "pascal.h"
#include "routines.h"
#include "term.h"
#include "types.h"
#include "variables.h"

#include "treasures.h"

long t_level[MAX_OBJ_LEVEL + 1];

char const *colors[MAX_COLORS] = {
  "Amber", "Azure", "Blue", "Blue Speckled", "Blue Spotted", "Black", "Black Speckled",
  "Black Spotted", "Brown", "Brown Speckled", "Brown Spotted", "Bubbling", "Chartreuse",
  "Clear", "Cloudy", "Copper", "Copper Spotted", "Crimson", "Cyan", "Dark Blue",
  "Dark Green", "Dark Red", "Ecru", "Gold", "Gold Spotted", "Green", "Green Speckled",
  "Green Spotted", "Grey", "Grey Spotted", "Hazy", "Indigo", "Light Blue", "Light Green",
  "Magenta", "Metallic Blue", "Metallic Red", "Metallic Green", "Metallic Purple", "Misty",
  "Orange", "Orange Speckled", "Orange Spotted", "Pink", "Pink Speckled", "Plaid", "Puce",
  "Purple", "Purple Speckled", "Purple Spotted", "Red", "Red Speckled", "Red Spotted",
  "Silver", "Silver Speckled", "Silver Spotted", "Smokey", "Tan", "Tangerine", "Topaz",
  "Turquoise", "Violet", "Vermilion", "White", "White Speckled", "White Spotted", "Yellow",
  "Daggy"};
char const *mushrooms[MAX_MUSH] = {
    "Blue",        "Black",     "Brown",    "Copper",  "Crimson", "Dark blue",
    "Dark green",  "Dark red",  "Gold",     "Green",   "Grey",    "Light Blue",
    "Light Green", "Orange",    "Pink",     "Plaid",   "Purple",  "Red",
    "Tan",         "Turquoise", "Violet",   "White",   "Yellow",  "Wrinkled",
    "Wooden",      "Slimey",    "Speckled", "Spotted", "Furry"};
char const *woods[MAX_WOODS] = {
    "Applewood",  "Ashen",      "Aspen",     "Avocado wood", "Balsa",
    "Banyan",     "Birch",      "Cedar",     "Cherrywood",   "Cinnibar",
    "Cottonwood", "Cypress",    "Dogwood",   "Driftwood",    "Ebony",
    "Elm wood",   "Eucalyptus", "Grapevine", "Hawthorn",     "Hemlock",
    "Hickory",    "Iron wood",  "Juniper",   "Locust",       "Mahogany",
    "Magnolia",   "Manzanita",  "Maple",     "Mulberry",     "Oak",
    "Pecan",      "Persimmon",  "Pine",      "Redwood",      "Rosewood",
    "Spruce",     "Sumac",      "Sycamore",  "Teak",         "Walnut",
    "Zebra wood"};
char const *metals[MAX_METALS] = {
  "Aluminium", "Bone", "Brass", "Bronze", "Cast Iron", "Chromium", "Copper", "Gold",
  "Iron", "Lead", "Magnesium", "Molybdenum", "Nickel", "Pewter", "Rusty", "Silver",
  "Steel", "Tin", "Titanium", "Tungsten", "Zirconium", "Zinc", "Aluminium Plated",
  "Brass Plated", "Copper Plated", "Gold Plated", "Nickel Plated", "Silver Plated",
  "Steel Plated", "Tin Plated", "Zinc Plated", "Uranium"};
char const *horns[MAX_HORNS] = {
    "Bag Pipes", "Bugle",  "Conch Shell", "Fife",     "Harmonica",
    "Horn",      "Picolo", "Pipes",       "Recorder", "Reed",
    "Trumpet",   "Tuba",   "Whistle"};
char const *rocks[MAX_ROCKS] = {
    "Amber",      "Agate",     "Alexandrite", "Amethyst",     "Antlerite",
    "Aquamarine", "Argentite", "Azurite",     "Beryl",        "Bloodstone",
    "Calcite",    "Carnelian", "Coral",       "Corundum",     "Cryolite",
    "Diamond",    "Diorite",   "Emerald",     "Flint",        "Fluorite",
    "Gabbro",     "Garnet",    "Granite",     "Gypsum",       "Hematite",
    "Jade",       "Jasper",    "Kryptonite",  "Lapus lazuli", "Limestone",
    "Malachite",  "Manganite", "Marble",      "Mica",         "Moonstone",
    "Neptunite",  "Obsidian",  "Onyx",        "Opal",         "Pearl",
    "Pyrite",     "Quartz",    "Quartzite",   "Rhodonite",    "Rhyolite",
    "Ruby",       "Sapphire",  "Sphalerite",  "Staurolite",   "Tiger eye",
    "Topaz",      "Turquoise", "Zircon"};
char const *amulets[MAX_AMULETS] = {
    "Birch",     "Cedar",    "Dogwood",   "Driftwood", "Elm wood", "Hemlock",
    "Hickory",   "Mahogany", "Maple",     "Oak",       "Pine",     "Redwood",
    "Rosewood",  "Walnut",   "Aluminium", "Bone",      "Brass",    "Bronze",
    "Copper",    "Iron",     "Lead",      "Nickel",    "Agate",    "Amethyst",
    "Diamond",   "Emerald",  "Flint",     "Garnet",    "Jade",     "Obsidian",
    "Onyx",      "Opal",     "Pearl",     "Quartz",    "Ruby",     "Sapphire",
    "Tiger eye", "Topaz",    "Turquoise"};
char const *cloths[MAX_CLOTHS] = {
    "Burlap",     "Cotton",      "Wool",     "Sack-cloth",
    "Rabbit-fur", "Lizard-skin", "Goat-skin"};
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

static void mt__ego_sword(treasure_type *treasure_ptr) {
  switch (randint(5)) {

  case 1: /*{Holy Avenger}*/

    strcat(treasure_ptr->name, " (HA)");
    treasure_ptr->flags |=
        (See_Invisible_worn_bit | Sustain_Stat_worn_bit | Resist_Acid_worn_bit |
         Resist_Fire_worn_bit | Strength_worn_bit | Slay_Undead_worn_bit |
         Slay_Evil_worn_bit);

    treasure_ptr->tohit += 5;
    treasure_ptr->todam += 5;
    treasure_ptr->toac = randint(4);
    treasure_ptr->p1 = randint(4) - 1;
    treasure_ptr->cost += treasure_ptr->p1 * 50000;
    treasure_ptr->cost += 1000000;
    break;

  case 2: /*{Defender}*/

    strcat(treasure_ptr->name, " [%P4] (DF)");
    treasure_ptr->flags |=
        (Feather_Fall_worn_bit | See_Invisible_worn_bit |
         Resist_Lightning_worn_bit | Free_Action_worn_bit |
         Resist_Cold_worn_bit | Resist_Acid_worn_bit | Resist_Fire_worn_bit |
         Regeneration_worn_bit | Stealth_worn_bit);

    treasure_ptr->tohit += 3;
    treasure_ptr->todam += 3;
    treasure_ptr->toac = 5 + randint(5);
    treasure_ptr->p1 = randint(3);
    treasure_ptr->cost += treasure_ptr->p1 * 50000;
    treasure_ptr->cost += 750000;
    break;

  case 3: /*{Demon Bane}*/

    strcat(treasure_ptr->name, " (DB)");
    treasure_ptr->flags |= Resist_Fire_worn_bit;
    treasure_ptr->flags2 |= Slay_demon_worn_bit;
    treasure_ptr->tohit += 3;
    treasure_ptr->todam += 3;
    treasure_ptr->cost += 500000;
    break;

  case 4: /* {Soul Sword}*/

    strcat(treasure_ptr->name, " (SS)");
    treasure_ptr->flags |=
        (Intelligence_worn_bit | Wisdom_worn_bit | Charisma_worn_bit |
         See_Invisible_worn_bit | Regeneration_worn_bit);
    treasure_ptr->flags2 |= (Soul_Sword_worn_bit | Bad_repute_worn_bit);

    treasure_ptr->tohit += 5;
    treasure_ptr->todam += 10;
    treasure_ptr->p1 = -randint(3) - 2;
    treasure_ptr->cost += 800000 + treasure_ptr->p1 * 40000;
    break;

  case 5: /*{Vorpal Sword}*/

    strcat(treasure_ptr->name, " (V)");
    treasure_ptr->flags |= Sustain_Stat_worn_bit;
    treasure_ptr->flags2 |= Sharp_worn_bit;
    treasure_ptr->p1 = 1;
    treasure_ptr->tohit += 5;
    treasure_ptr->todam += 5;
    treasure_ptr->cost += 750000;
    break;
  }
}

static void mt__slaying_sword(treasure_type *treasure_ptr) {
  switch (randint(4)) {
  case 1: /* {Slay Monster}*/
    strcat(treasure_ptr->name, " (SM)");
    treasure_ptr->flags |= (See_Invisible_worn_bit | Slay_Monster_worn_bit);
    treasure_ptr->tohit += 3;
    treasure_ptr->todam += 3;
    treasure_ptr->cost += 500000;
    break;

  case 2: /* {Slay Dragon}*/
    strcat(treasure_ptr->name, " (SD)");
    treasure_ptr->flags |= Slay_Dragon_worn_bit;
    treasure_ptr->tohit += 3;
    treasure_ptr->todam += 3;
    treasure_ptr->cost += 400000;
    break;

  case 3: /* {Slay Undead}*/
    strcat(treasure_ptr->name, " (SU)");
    treasure_ptr->flags |= Slay_Undead_worn_bit;
    treasure_ptr->tohit += 2;
    treasure_ptr->todam += 2;
    treasure_ptr->cost += 300000;
    break;

  case 4: /* {Slay Regenerative}*/
    strcat(treasure_ptr->name, " (SR)");
    treasure_ptr->flags2 |= Slay_regen_worn_bit;
    treasure_ptr->tohit += 2;
    treasure_ptr->todam += 2;
    treasure_ptr->cost += 150000;
    break;
  }
}

static void mt__magic_sword(treasure_type *treasure_ptr) {
  switch (randint(4)) {
  case 1: /* {Flame Tongue}*/
    strcat(treasure_ptr->name, " (FT)");
    treasure_ptr->flags |= Flame_Brand_worn_bit;
    treasure_ptr->tohit += 1;
    treasure_ptr->todam += 3;
    treasure_ptr->cost += 200000;
    break;

  case 2: /* {Frost Brand}*/
    strcat(treasure_ptr->name, " (FB)");
    treasure_ptr->flags |= Cold_Brand_worn_bit;
    treasure_ptr->tohit += 1;
    treasure_ptr->todam += 1;
    treasure_ptr->cost += 120000;
    break;

  case 3: /* {Wizards Blade}*/
    strcat(treasure_ptr->name, " (WB)");
    treasure_ptr->flags2 |= Magic_proof_worn_bit;
    treasure_ptr->weight = trunc(treasure_ptr->weight * 4 / 5);
    treasure_ptr->tval = dagger; /* let mages use it */
    treasure_ptr->tohit += 3;
    treasure_ptr->todam += 1;
    treasure_ptr->cost += 80000;
    break;

  case 4: /* {Blessed Blade}*/
    strcat(treasure_ptr->name, " (BB)");
    treasure_ptr->flags2 |= Magic_proof_worn_bit;
    treasure_ptr->tval = maul; /* let priests use it */
    treasure_ptr->tohit += 2;
    treasure_ptr->todam += 4;
    treasure_ptr->cost += 80000;
    break;
  }
}

static long mt__m_bonus(long base, long max_std, long level, boolean forceit) {
  /*{ Enchant a bonus based on degree desired -RAK- }*/
  long x, stand_dev;
  long return_value;

  if (forceit) {
    /*    base += 2; */
  }

  stand_dev = trunc(obj_std_adj * level) + obj_std_min;

  if (stand_dev > max_std) {
    stand_dev = max_std;
  }

  x = trunc(labs(randnor(0, stand_dev)) / 10.0) + base;

  if (x < base) {
    return_value = base;
  } else {
    return_value = x;
  }

  return return_value;
}

static void mt__gems(treasure_type *treasure_ptr) {
  long p1;

  switch (treasure_ptr->subval) {
  case 1:
    p1 = randint(10) + 10;
    break;
  case 2:
    p1 = randint(5) + 2;
    break;
  case 3:
    p1 = randint(8) + 7;
    break;
  case 4:
    p1 = randint(3) + 3;
    break;
  case 5:
    p1 = randint(10) + 10;
    break;
  case 6:
    p1 = randint(5) + 5;
    break;
  case 7:
    p1 = randint(15) + 15;
    break;
  case 8:
    p1 = randint(3) + 2;
    break;
  case 9:
    p1 = randint(5) + 3;
    break;
  case 10:
    p1 = randint(3) + 2;
    break;
  case 11:
    p1 = randint(6) + 4;
    break;
  default:
    MSG(("WARNING: Unknown subval in mt__gems: %ld  %s", treasure_ptr->subval,
         treasure_ptr->name));
    p1 = 0;
    break;
  }

  treasure_ptr->p1 = p1;
}

static void mt__misc_usable(treasure_type *treasure_ptr) {
  switch (treasure_ptr->subval) {
  case 14: /* statues */
    switch (randint(3)) {
    case 1: /* summoning undead */
      strcat(treasure_ptr->name, " Major of Undead Summoning");
      treasure_ptr->flags |= 0x00000100;
      treasure_ptr->cost = 0;
      treasure_ptr->p1 = randint(4) + 2;
      break;
    case 2: /* summon demon */
      strcat(treasure_ptr->name, " Major of Demon Summoning");
      treasure_ptr->flags |= 0x00000200;
      treasure_ptr->cost = 0;
      treasure_ptr->p1 = randint(3) + 1;
      break;
    case 3: /* Life giving */
      strcat(treasure_ptr->name, " Life Giving");
      treasure_ptr->flags |= 0x00000400;
      treasure_ptr->cost = 900000;
      treasure_ptr->p1 = randint(5) + 3;
      break;
    }
    break;

  case 15: /* teeth ? */
    switch (randint(4)) {
    case 1:
      strcat(treasure_ptr->name, " from a Dragon");
      treasure_ptr->p1 = randint(4) + 2;
      treasure_ptr->cost += treasure_ptr->p1 * 20000;
      treasure_ptr->flags |= 0x20000000;
      break;
    case 2:
      strcat(treasure_ptr->name, " of a Demon");
      treasure_ptr->flags |= 0x40000000;
      treasure_ptr->p1 = randint(4) + 2;
      treasure_ptr->cost += treasure_ptr->p1 * 20000;
      break;
    case 3:
    case 4:
      break;
    }
    break;

  case 16: /* crucifixes */
  case 17:
  case 18:
    switch (randint(4)) {
    case 1:
    case 2:
    case 3:
      strcat(treasure_ptr->name, " of Turning");
      treasure_ptr->flags |= 0x00000001;
      treasure_ptr->p1 = randint(treasure_ptr->p1 * 2) + 2;
      treasure_ptr->cost += treasure_ptr->p1 * 20000;
      break;
    case 4:
      strcat(treasure_ptr->name, " of Demon Dispelling");
      treasure_ptr->flags |= 0x00000002;
      treasure_ptr->p1 = randint(trunc(treasure_ptr->subval / 2));
      treasure_ptr->cost += treasure_ptr->p1 * 50000;
    } /* end rand switch */
    break;

  case 19:
    strcat(treasure_ptr->name, " of Summon Undead");
    treasure_ptr->flags |= 0x00000004;
    treasure_ptr->cost = 0;
    treasure_ptr->p1 = 2;
    break;

  case 20:
    strcat(treasure_ptr->name, " of Demon Summoning");
    treasure_ptr->flags |= 0x00000008;
    treasure_ptr->cost = 0;
    treasure_ptr->p1 = 2;
    break;

  case 21:
    switch (randint(3)) {
    case 1:
      strcat(treasure_ptr->name, " containing a Djinni");
      treasure_ptr->flags |= 0x00000010;
      treasure_ptr->cost = 200000;
      treasure_ptr->p1 = 1;
      break;
    case 2:
    case 3:
      strcat(treasure_ptr->name, " containing some Demons");
      treasure_ptr->flags |= 0x00000020;
      treasure_ptr->cost = 0;
      treasure_ptr->p1 = 1;
      break;
    }
  }
}

static void mt__armor_and_shields(treasure_type *treasure_ptr, long level,
                                  boolean is_magic, boolean is_special,
                                  boolean is_cursed, boolean forceit) {
  if (treasure_ptr->tval == soft_armor && treasure_ptr->subval == 6) {
    /* Looks like soft_armor 6 is Woven Cord Armor */

    if (forceit || randint(4) == 1) {
      *treasure_ptr = yums[11 + randint(3)];
      treasure_ptr->weight *= WEIGHT_ADJ;
      treasure_ptr->cost *= COST_ADJ;
    }

  } else if (treasure_ptr->tval == hard_armor && treasure_ptr->subval == 13) {
    /* I think hard_armor 13 is Bronze Plate Mail */

    if (is_magic || randint(5) == 1) {
      if (is_special || randint(5) == 1) {
        if (randint(3) == 1) {
          /* Mithril Plate Armor */
          *treasure_ptr = yums[17];
        } else {
          /* Mithril Chain Mail */
          *treasure_ptr = yums[16];
        }
      } else {
        /* Elven Chain Mail */
        *treasure_ptr = yums[15];
      }

      treasure_ptr->weight *= WEIGHT_ADJ;
      treasure_ptr->cost = trunc(treasure_ptr->cost * COST_ADJ);
    }
  }

  if (is_magic) {
    treasure_ptr->toac = mt__m_bonus(1, 30, level, forceit);
    if (is_special) {
      switch (randint(9)) {

      case 1: /*{ Resist }*/
        strcat(treasure_ptr->name, " (R)");
        treasure_ptr->flags |=
            (Resist_Lightning_worn_bit | Resist_Cold_worn_bit |
             Resist_Acid_worn_bit | Resist_Fire_worn_bit);
        treasure_ptr->toac += 5;
        treasure_ptr->cost += 250000;
        break;

      case 2: /*{ Resist Acid	}*/
        strcat(treasure_ptr->name, " (RA)");
        treasure_ptr->flags |= Resist_Acid_worn_bit;
        treasure_ptr->cost += 100000;
        break;

      case 3: /*{ Resist Fire	}*/
      case 4:
        strcat(treasure_ptr->name, " (RF)");
        treasure_ptr->flags |= Resist_Fire_worn_bit;
        treasure_ptr->cost += 60000;
        break;

      case 5: /*{ Resist Cold	}*/
      case 6:
        strcat(treasure_ptr->name, " (RC)");
        treasure_ptr->flags |= Resist_Cold_worn_bit;
        treasure_ptr->cost += 60000;
        break;

      case 7: /*{ Resist Lightning}*/
      case 8:
      case 9:
        strcat(treasure_ptr->name, " (RL)");
        treasure_ptr->flags |= Resist_Lightning_worn_bit;
        treasure_ptr->cost += 50000;
        break;
      }
    }
  } else if (is_cursed) {
    treasure_ptr->flags |= Cursed_worn_bit;
    treasure_ptr->toac = -mt__m_bonus(1, 40, level, forceit);
    treasure_ptr->cost = 0;
  }
}

static void mt__weapons(treasure_type *treasure_ptr, long level,
                        boolean is_magic, boolean is_special, boolean is_cursed,
                        boolean forceit) {
  long wpn_type;

  if (is_magic) {
    treasure_ptr->tohit = mt__m_bonus(0, 40, level, forceit);
    treasure_ptr->todam = mt__m_bonus(0, 40, level, forceit);
    if (!is_special)
      return;

    /* filthy rags? */
    if (treasure_ptr->subval == 99 && randint(5) == 1) {
      strcat(treasure_ptr->name, " of Trollkind");
      treasure_ptr->flags |=
          (Charisma_worn_bit | Searching_worn_bit | Stealth_worn_bit |
           Regeneration_worn_bit | Resist_Acid_worn_bit | Resist_Cold_worn_bit);
      treasure_ptr->p1 = -5;
      treasure_ptr->cost = 120000;
      strcpy(treasure_ptr->damage, "3d4");
    } else {
      wpn_type = randint(100);
      if (wpn_type < 61) {
        /* FT  FB  WB  BB */
        mt__magic_sword(treasure_ptr);
      } else if (wpn_type < 81) {
        /* SM  SD  SU  SR     */
        mt__slaying_sword(treasure_ptr);
      } else if (wpn_type < 96) {
        /* HA  DF  DB  SS  V  */
        mt__ego_sword(treasure_ptr);
      } else {
        /* FT  FB  WB  BB */
        mt__magic_sword(treasure_ptr);
        /* HA  DF  DB  SS  V  */
        mt__ego_sword(treasure_ptr);
      }
    }
  } else if (is_cursed) {
    treasure_ptr->flags |= Cursed_worn_bit;
    treasure_ptr->tohit = -mt__m_bonus(1, 55, level, forceit);
    treasure_ptr->todam = -mt__m_bonus(1, 55, level, forceit);
    treasure_ptr->cost = 0;
  }
}

static void mt__bows_and_slings(treasure_type *treasure_ptr, long level,
                                boolean is_magic, boolean is_special,
                                boolean is_cursed, boolean forceit) {
  if (is_magic) {
    treasure_ptr->tohit = mt__m_bonus(1, 30, level, forceit);
    if (is_special) {
      strcat(treasure_ptr->name, " of Criticals");
      treasure_ptr->flags2 |= Sharp_worn_bit;
      treasure_ptr->tohit += 5;
      treasure_ptr->cost += 300000;
    }
  } else if (is_cursed) {
    treasure_ptr->flags |= Cursed_worn_bit;
    treasure_ptr->tohit = -mt__m_bonus(1, 50, level, forceit);
    treasure_ptr->cost = 0;
  }
}

static void mt__pick_or_shovel(treasure_type *treasure_ptr, long level,
                               boolean is_magic, boolean forceit) {
  if (is_magic) {
    switch (randint(3)) {
    case 1:
    case 2: /*{25}*/
      treasure_ptr->p1 = mt__m_bonus(2, 25, level, forceit);
      treasure_ptr->cost += treasure_ptr->p1 * 10000;
      break;
    case 3:
      treasure_ptr->flags |= Cursed_worn_bit;
      treasure_ptr->p1 = -mt__m_bonus(1, 30, level, forceit);
      treasure_ptr->cost = 0;
      break;
    }
  }
}

static void mt__gloves_and_gauntlets(treasure_type *treasure_ptr, long level,
                                     boolean is_magic, boolean is_special,
                                     boolean is_cursed, boolean forceit) {
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
          treasure_ptr->flags |=
              (Feather_Fall_worn_bit | See_Invisible_worn_bit);
          treasure_ptr->p1 = mt__m_bonus(5, 50, level, forceit);
          treasure_ptr->cost += 20000 + treasure_ptr->p1 * 5;
          break;
        case 4:
        case 5:
          strcat(treasure_ptr->name, " of Ogre Power");
          treasure_ptr->flags |= (Slow_Digestion_worn_bit | Strength_worn_bit);
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
        treasure_ptr->flags |= (Cursed_worn_bit | Dexterity_worn_bit);
        treasure_ptr->p1 = 1;
        break;
      case 2:
        strcat(treasure_ptr->name, " of Weakness");
        treasure_ptr->flags |= (Cursed_worn_bit | Strength_worn_bit);
        treasure_ptr->p1 = 1;
        break;
      case 3:
        strcat(treasure_ptr->name, " of Ogre Intelligence");
        treasure_ptr->flags |= (Cursed_worn_bit | Intelligence_worn_bit);
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

static void mt__boots(treasure_type *treasure_ptr, long level, boolean is_magic,
                      boolean is_special, boolean is_cursed, boolean forceit) {
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
          treasure_ptr->flags |=
              (Charisma_worn_bit | Feather_Fall_worn_bit |
               See_Invisible_worn_bit | Free_Action_worn_bit);
          treasure_ptr->flags2 |= (Magic_proof_worn_bit);
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
      treasure_ptr->flags |= (Cursed_worn_bit | Speed_worn_bit);
      treasure_ptr->p1 = -1;
      break;
    case 2:
      strcat(treasure_ptr->name, " of Noise");
      treasure_ptr->flags |= (Cursed_worn_bit | Aggravation_worn_bit);
      break;
    case 3:
      strcat(treasure_ptr->name, " of Great Mass");
      treasure_ptr->flags |= Cursed_worn_bit;
      treasure_ptr->weight *= 5;
      break;
    }
  }
}

static void mt__helms(treasure_type *treasure_ptr, long level, boolean is_magic,
                      boolean is_special, boolean is_cursed, boolean forceit) {
  if (is_magic) {
    treasure_ptr->toac = mt__m_bonus(1, 20, level, forceit);
    if (is_special)
      switch (treasure_ptr->subval) {
      case 1:
      case 2:
      case 3:
      case 4:
      case 5:
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
      case 6:
      case 7:
      case 8:
      case 9:
      case 10:
        switch (randint(6)) {
        case 1:
          strcat(treasure_ptr->name, " of Might");
          treasure_ptr->flags |= (Free_Action_worn_bit | Constitution_worn_bit |
                                  Strength_worn_bit | Dexterity_worn_bit);
          treasure_ptr->p1 = randint(3);
          treasure_ptr->cost += 100000 + treasure_ptr->p1 * 50000;
          break;
        case 2:
          strcat(treasure_ptr->name, " of Lordliness");
          treasure_ptr->flags |= (Wisdom_worn_bit | Charisma_worn_bit);
          treasure_ptr->p1 = randint(3);
          treasure_ptr->cost += 100000 + treasure_ptr->p1 * 50000;
          break;
        case 3:
          strcat(treasure_ptr->name, " of the Magi");
          treasure_ptr->flags |= (Free_Action_worn_bit | Strength_worn_bit |
                                  Constitution_worn_bit | Dexterity_worn_bit);
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
          treasure_ptr->flags |= (See_Invisible_worn_bit | Searching_worn_bit);
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
        treasure_ptr->flags |= (Infra_Vision_worn_bit | See_Invisible_worn_bit |
                                Free_Action_worn_bit | Searching_worn_bit);
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
            (Cursed_worn_bit | Strength_worn_bit | Dexterity_worn_bit |
             Constitution_worn_bit | Intelligence_worn_bit | Wisdom_worn_bit |
             Charisma_worn_bit | Stealth_worn_bit | Aggravation_worn_bit |
             Teleportation_worn_bit | Blindness_worn_bit | Timidness_worn_bit);
        treasure_ptr->flags2 |= (Hunger_worn_bit | Known_cursed_bit);
        treasure_ptr->p1 = -5;
        break;
      }
      treasure_ptr->p1 *= randint(5);
    }
  }
}

static void mt__belt(treasure_type *treasure_ptr, long level, boolean is_magic,
                     boolean is_special, boolean is_cursed, boolean forceit) {
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
            treasure_ptr->flags |=
                (Resist_Lightning_worn_bit | Resist_Fire_worn_bit |
                 Resist_Cold_worn_bit | Resist_Acid_worn_bit |
                 Regeneration_worn_bit | Free_Action_worn_bit);
            treasure_ptr->flags2 |= Magic_proof_worn_bit;
            treasure_ptr->p1 = 7;
            treasure_ptr->cost += 7500000;
          } else {
            strcat(treasure_ptr->name, " of Storm Giant Strength");
            treasure_ptr->flags |=
                (Resist_Lightning_worn_bit | Resist_Acid_worn_bit);
            treasure_ptr->flags2 |= (Magic_proof_worn_bit);
            treasure_ptr->p1 = 6;
            treasure_ptr->cost += 3500000;
          }
          break;
        case 2:
          strcat(treasure_ptr->name, " of Cloud Giant Strength");
          treasure_ptr->flags |=
              (Resist_Lightning_worn_bit | Resist_Acid_worn_bit);
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
              (Sustain_Stat_worn_bit | Slow_Digestion_worn_bit);
          treasure_ptr->p1 = 2;
          treasure_ptr->cost += 100000;
          break;
        }
        break;

      case 13: /* Leather Belt */
        strcat(treasure_ptr->name, " of Dwarvenkind");
        treasure_ptr->flags |= (Infra_Vision_worn_bit | Tunneling_worn_bit |
                                Sustain_Stat_worn_bit);
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
        treasure_ptr->flags |= (Cursed_worn_bit | Timidness_worn_bit);
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

static void mt__ring(treasure_type *treasure_ptr, long level,
                     boolean is_special, boolean is_cursed, boolean forceit) {
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
    treasure_ptr->cost += (1000000 * treasure_ptr->p1);
    if (is_cursed) {
      treasure_ptr->flags |= Cursed_worn_bit;
      treasure_ptr->flags2 &= (~Known_cursed_bit);
    }
    break;

  default:
    break;
  }
}

static void mt__amulet(treasure_type *treasure_ptr, long level,
                       boolean is_cursed, boolean forceit) {
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

static void mt__lamp_or_torch(treasure_type *treasure_ptr) {
  /* Subval should be even for store, odd for dungeon
   * Dungeon found ones will be partially charged
   */

  if ((treasure_ptr->subval & 1) == 1) {
    treasure_ptr->p1 = randint(treasure_ptr->p1);
  }
}

static void mt__wand(treasure_type *treasure_ptr, boolean forceit) {
  long p1;

  switch (treasure_ptr->subval) {
  case 1:
    p1 = randint(10) + 6;
    break;
  case 2:
    p1 = randint(8) + 6;
    break;
  case 3:
    p1 = randint(5) + 6;
    break;
  case 4:
    p1 = randint(8) + 6;
    break;
  case 5:
    p1 = randint(4) + 3;
    break;
  case 6:
    p1 = randint(8) + 6;
    break;
  case 7:
    p1 = randint(20) + 12;
    break;
  case 8:
    p1 = randint(20) + 12;
    break;
  case 9:
    p1 = randint(10) + 6;
    break;
  case 10:
    p1 = randint(12) + 6;
    break;
  case 11:
    p1 = randint(10) + 12;
    break;
  case 12:
    p1 = randint(3) + 3;
    break;
  case 13:
    p1 = randint(8) + 6;
    break;
  case 14:
    p1 = randint(10) + 6;
    break;
  case 15:
    p1 = randint(5) + 3;
    break;
  case 16:
    p1 = randint(5) + 3;
    break;
  case 17:
    p1 = randint(5) + 6;
    break;
  case 18:
    p1 = randint(5) + 4;
    break;
  case 19:
    p1 = randint(8) + 4;
    break;
  case 20:
    p1 = randint(6) + 2;
    break;
  case 21:
    p1 = randint(4) + 2;
    break;
  case 22:
    p1 = randint(8) + 6;
    break;
  case 23:
    p1 = randint(5) + 2;
    break;
  case 24:
    p1 = randint(12) + 12;
    break;
  case 25:
    p1 = randint(20) + 10;
    break;
  default:
    MSG(("WARNING: Unknown subval in mt__wand: %ld  %s", treasure_ptr->subval,
         treasure_ptr->name));
    p1 = 0;
    break;
  }

  if (forceit) {
    p1 += 8;
  }
  treasure_ptr->p1 = p1;
}

static void mt__staff(treasure_type *treasure_ptr, boolean forceit) {
  long p1;

  switch (treasure_ptr->subval) {
  case 1:
    p1 = randint(20) + 12;
    break;
  case 2:
    p1 = randint(8) + 6;
    break;
  case 3:
    p1 = randint(5) + 6;
    break;
  case 4:
    p1 = randint(20) + 12;
    break;
  case 5:
    p1 = randint(15) + 6;
    break;
  case 6:
    p1 = randint(4) + 5;
    break;
  case 7:
    p1 = randint(5) + 3;
    break;
  case 8:
    p1 = randint(3) + 1;
    break;
  case 9:
    p1 = randint(3) + 1;
    break;
  case 10:
    p1 = randint(3) + 1;
    break;
  case 11:
    p1 = randint(5) + 6;
    break;
  case 12:
    p1 = randint(10) + 12;
    break;
  case 13:
    p1 = randint(5) + 6;
    break;
  case 14:
    p1 = randint(5) + 6;
    break;
  case 15:
    p1 = randint(5) + 6;
    break;
  case 16:
    p1 = randint(10) + 12;
    break;
  case 17:
    p1 = randint(3) + 4;
    break;
  case 18:
    p1 = randint(5) + 6;
    break;
  case 19:
    p1 = randint(5) + 6;
    break;
  case 20:
    p1 = randint(3) + 4;
    break;
  case 21:
    p1 = randint(10) + 12;
    break;
  case 22:
    p1 = randint(3) + 4;
    break;
  case 23:
    p1 = randint(3) + 4;
    break;
  case 24:
    p1 = randint(3) + 1;
    break;
  case 25:
    p1 = randint(10) + 6;
    break;
  case 26:
    p1 = randint(6) + 6;
    break;
  default:
    MSG(("WARNING: Unknown subval in mt__staff: %ld  %s", treasure_ptr->subval,
         treasure_ptr->name));
    p1 = 0;
    break;
  }

  if (forceit) {
    p1 += 5;
  }
  treasure_ptr->p1 = p1;
}

static void mt__chime(treasure_type *treasure_ptr, boolean forceit) {
  long p1;

  switch (treasure_ptr->subval) {
  case 1:
    p1 = randint(20) + 12;
    break;
  case 2:
    p1 = randint(8) + 6;
    break;
  case 3:
    p1 = randint(5) + 6;
    break;
  case 4:
    p1 = randint(4) + 5;
    break;
  case 5:
    p1 = randint(5) + 3;
    break;
  case 6:
    p1 = randint(3) + 1;
    break;
  case 7:
    p1 = randint(10) + 10;
    break;
  case 8:
    p1 = randint(10) + 12;
    break;
  case 9:
    p1 = randint(5) + 6;
    break;
  case 10:
    p1 = randint(5) + 6;
    break;
  case 11:
    p1 = randint(5) + 6;
    break;
  case 12:
    p1 = randint(5) + 6;
    break;
  case 13:
    p1 = randint(3) + 4;
    break;
  case 14:
    p1 = randint(3) + 4;
    break;
  case 15:
    p1 = randint(3) + 4;
    break;
  case 16:
    p1 = randint(10) + 6;
    break;
  default:
    MSG(("WARNING: Unknown subval in mt__chime: %ld  %s", treasure_ptr->subval,
         treasure_ptr->name));
    p1 = 0;
    break;
  }

  if (forceit) {
    p1 += 5;
  }
  treasure_ptr->p1 = p1;
}

static void mt__horn(treasure_type *treasure_ptr, boolean forceit) {
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

static void mt__cloak(treasure_type *treasure_ptr, long level, boolean is_magic,
                      boolean is_special, boolean is_cursed, boolean forceit) {
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
        treasure_ptr->flags |= (See_Invisible_worn_bit | Sustain_Stat_worn_bit |
                                Stealth_worn_bit | Charisma_worn_bit);
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
      treasure_ptr->flags |= (Cursed_worn_bit | Aggravation_worn_bit);
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

static void mt__chest(treasure_type *treasure_ptr, long level) {
  /*
   * Items inside the chest will be created as if
   * found on dungeon level p1.
   */

  treasure_ptr->p1 = dun_level + randint(10) - 5;
  if (treasure_ptr->p1 < 1) {
    treasure_ptr->p1 = 1;
  }

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

static void mt__ammo(treasure_type *treasure_ptr, long level, boolean is_magic,
                     boolean is_special, boolean is_cursed, boolean forceit) {
  long i1;

  if (treasure_ptr->tval == bolt || treasure_ptr->tval == arrow) {
    if (is_magic) {
      treasure_ptr->tohit = mt__m_bonus(1, 35, level, forceit);
      treasure_ptr->todam = mt__m_bonus(1, 35, level, forceit);

      if (is_special) {
        switch (treasure_ptr->tval) {

        case sling_ammo:
          break;

        case spike:
          break;

        case arrow:
        case bolt:
          switch (randint(10)) {

          case 1:
          case 2:
          case 3:
            strcat(treasure_ptr->name, " of Slaying");
            treasure_ptr->tohit += 5;
            treasure_ptr->todam += 5;
            treasure_ptr->cost += 2000;
            break;
          case 4:
          case 5:
            strcat(treasure_ptr->name, " of Fire");
            treasure_ptr->flags |= Flame_Brand_worn_bit;
            treasure_ptr->tohit += 2;
            treasure_ptr->todam += 4;
            treasure_ptr->cost += 2500;
            break;
          case 6:
          case 7:
            strcat(treasure_ptr->name, " of Slay Evil");
            treasure_ptr->flags |= Slay_Evil_worn_bit;
            treasure_ptr->tohit += 3;
            treasure_ptr->todam += 3;
            treasure_ptr->cost += 2500;
            break;
          case 8:
          case 9:
            strcat(treasure_ptr->name, " of Slay Monster");
            treasure_ptr->flags |= Slay_Monster_worn_bit;
            treasure_ptr->tohit += 2;
            treasure_ptr->todam += 2;
            treasure_ptr->cost += 3000;
            break;
          case 10:
            strcat(treasure_ptr->name, " of Dragon Slaying");
            treasure_ptr->flags |= Slay_Dragon_worn_bit;
            treasure_ptr->tohit += 10;
            treasure_ptr->todam += 10;
            treasure_ptr->cost += 3500;
            break;
          } /* end switch randint */

        default:
          break;
        }
      }
    }
  } else if (is_cursed) {
    treasure_ptr->flags |= Cursed_worn_bit;
    treasure_ptr->tohit = -mt__m_bonus(5, 55, level, forceit);
    treasure_ptr->todam = -mt__m_bonus(5, 55, level, forceit);
    treasure_ptr->cost = 0;
  }

  treasure_ptr->number = forceit ? 15 : 0;
  for (i1 = 0; i1 < 7; i1++) {
    treasure_ptr->number += randint(6);
  }

  missle_ctr++;
  if (missle_ctr > 65534) {
    missle_ctr = 1;
  }

  treasure_ptr->subval = missle_ctr + 512;
}

static void mt__food(treasure_type *treasure_ptr, boolean is_special,
                     boolean is_cursed) {
  if (is_special && is_cursed && treasure_ptr->subval == 319) {
    /* Eyeballs of Unhealth */
    *treasure_ptr = yums[18]; /* Eyeballs of Drong */
    treasure_ptr->weight *= WEIGHT_ADJ;
    treasure_ptr->cost *= COST_ADJ;
  }
}

static void mt__get_chances(long level, boolean *is_magic, boolean *is_special,
                            boolean *is_cursed, boolean forceit) {
  /* Chance of treasure having magic abilities		-RAK-	*/
  /* Chance increases with each dungeon level			*/

  long magic = obj_base_magic + (level * (obj_base_max - obj_base_magic)) / 100;
  long special;
  long cursed;

  if (magic > obj_base_max)
    magic = obj_base_max;

  special = trunc(magic / obj_div_special);
  cursed = trunc(magic / obj_div_cursed);

  *is_magic = randint(150) <= magic;
  *is_special = randint(150) <= special;
  *is_cursed = randint(15) <= cursed;
  if (forceit)
    *is_magic = true;
  if (*is_magic)
    *is_cursed = false;
}

void magic_treasure(long x, long level, boolean forceit) {
  treasure_type *treasure_ptr = &t_list[x];
  boolean is_magic;   /* Positive enchantment */
  boolean is_cursed;  /* Negative enchantment */
  boolean is_special; /* Enchantment is stronger than usual */

  mt__get_chances(level, &is_magic, &is_special, &is_cursed, forceit);

  /*
   * Depending on treasure type, it can have certain magical
   * properties
   */

  switch (treasure_ptr->tval) {

  /*{ Money money everywhere }*/
  case valuable_gems:
    mt__gems(treasure_ptr);
    break;

  /*{ Miscellaneous Objects }*/
  case misc_usable:
    if (forceit || (is_magic && is_special))
      mt__misc_usable(treasure_ptr);
    break;

  case shield:
  case hard_armor:
  case soft_armor:
    mt__armor_and_shields(treasure_ptr, level, is_magic, is_special, is_cursed,
                          forceit);
    break;

  /*{ Weapons }*/
  case hafted_weapon:
  case pole_arm:
  case sword:
  case dagger:
  case maul:
    mt__weapons(treasure_ptr, level, is_magic, is_special, is_cursed, forceit);
    break;

  case bow_crossbow_or_sling:
    mt__bows_and_slings(treasure_ptr, level, is_magic, is_special, is_cursed,
                        forceit);
    break;

  case pick_or_shovel:
    mt__pick_or_shovel(treasure_ptr, level, is_magic, forceit);
    break;

  case gloves_and_gauntlets:
    mt__gloves_and_gauntlets(treasure_ptr, level, is_magic, is_special,
                             is_cursed, forceit);
    break;

  case boots:
    mt__boots(treasure_ptr, level, is_magic, is_special, is_cursed, forceit);
    break;

  case helm:
    mt__helms(treasure_ptr, level, is_magic, is_special, is_cursed, forceit);
    break;

  /*{girdles, belts and buckles}*/
  case belt:
    mt__belt(treasure_ptr, level, is_magic, is_special, is_cursed, forceit);
    break;

  case ring:
    mt__ring(treasure_ptr, level, is_special, is_cursed, forceit);
    break;

  case amulet:
    mt__amulet(treasure_ptr, level, is_cursed, forceit);
    break;

  case lamp_or_torch:
    mt__lamp_or_torch(treasure_ptr);
    break;

  case wand:
    mt__wand(treasure_ptr, forceit);
    break;

  case staff:
    mt__staff(treasure_ptr, forceit);
    break;

  case chime:
    mt__chime(treasure_ptr, forceit);
    break;

  case horn:
    mt__horn(treasure_ptr, forceit);
    break;

  case cloak:
    mt__cloak(treasure_ptr, level, is_magic, is_special, is_cursed, forceit);
    break;

  case chest:
    mt__chest(treasure_ptr, level);
    break;

  case sling_ammo:
  case arrow:
  case bolt:
  case spike:
    mt__ammo(treasure_ptr, level, is_magic, is_special, is_cursed, forceit);
    break;

  case Food:
    mt__food(treasure_ptr, is_special, is_cursed);
    break;
  }
}
