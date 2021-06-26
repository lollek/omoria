/* desc.c */
/**/

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

/*	{ Descriptive constants						} */
#define MAX_COLORS 68     /*{ Used with potions	} */
#define MAX_MUSH 29       /*{ Used with mushrooms	} */
#define MAX_WOODS 41      /*{ Used with staffs	} */
#define MAX_METALS 32     /*{ Used with wands	} */
#define MAX_HORNS 13      /*{ Used with horns	} */
#define MAX_ROCKS 53      /*{ Used with rings	} */
#define MAX_CLOTHS 7      /*{ Used with bags/sacks	} */
#define MAX_AMULETS 39    /*{ Used with amulets	} */
#define MAX_SYLLABLES 156 /*{ Used with scrolls	} */
static char const *colors[MAX_COLORS] = {"Amber",
                                         "Azure",
                                         "Blue",
                                         "Blue Speckled",
                                         "Blue Spotted",
                                         "Black",
                                         "Black Speckled",
                                         "Black Spotted",
                                         "Brown",
                                         "Brown Speckled",
                                         "Brown Spotted",
                                         "Bubbling",
                                         "Chartreuse",
                                         "Clear",
                                         "Cloudy",
                                         "Copper",
                                         "Copper Spotted",
                                         "Crimson",
                                         "Cyan",
                                         "Dark Blue",
                                         "Dark Green",
                                         "Dark Red",
                                         "Ecru",
                                         "Gold",
                                         "Gold Spotted",
                                         "Green",
                                         "Green Speckled",
                                         "Green Spotted",
                                         "Grey",
                                         "Grey Spotted",
                                         "Hazy",
                                         "Indigo",
                                         "Light Blue",
                                         "Light Green",
                                         "Magenta",
                                         "Metallic Blue",
                                         "Metallic Red",
                                         "Metallic Green",
                                         "Metallic Purple",
                                         "Misty",
                                         "Orange",
                                         "Orange Speckled",
                                         "Orange Spotted",
                                         "Pink",
                                         "Pink Speckled",
                                         "Plaid",
                                         "Puce",
                                         "Purple",
                                         "Purple Speckled",
                                         "Purple Spotted",
                                         "Red",
                                         "Red Speckled",
                                         "Red Spotted",
                                         "Silver",
                                         "Silver Speckled",
                                         "Silver Spotted",
                                         "Smokey",
                                         "Tan",
                                         "Tangerine",
                                         "Topaz",
                                         "Turquoise",
                                         "Violet",
                                         "Vermilion",
                                         "White",
                                         "White Speckled",
                                         "White Spotted",
                                         "Yellow",
                                         "Daggy"};
static char const *mushrooms[MAX_MUSH] = {
    "Blue",        "Black",     "Brown",    "Copper",  "Crimson", "Dark blue",
    "Dark green",  "Dark red",  "Gold",     "Green",   "Grey",    "Light Blue",
    "Light Green", "Orange",    "Pink",     "Plaid",   "Purple",  "Red",
    "Tan",         "Turquoise", "Violet",   "White",   "Yellow",  "Wrinkled",
    "Wooden",      "Slimey",    "Speckled", "Spotted", "Furry"};
static char const *woods[MAX_WOODS] = {
    "Applewood",  "Ashen",      "Aspen",     "Avocado wood", "Balsa",
    "Banyan",     "Birch",      "Cedar",     "Cherrywood",   "Cinnibar",
    "Cottonwood", "Cypress",    "Dogwood",   "Driftwood",    "Ebony",
    "Elm wood",   "Eucalyptus", "Grapevine", "Hawthorn",     "Hemlock",
    "Hickory",    "Iron wood",  "Juniper",   "Locust",       "Mahogany",
    "Magnolia",   "Manzanita",  "Maple",     "Mulberry",     "Oak",
    "Pecan",      "Persimmon",  "Pine",      "Redwood",      "Rosewood",
    "Spruce",     "Sumac",      "Sycamore",  "Teak",         "Walnut",
    "Zebra wood"};
static char const *metals[MAX_METALS] = {"Aluminium",
                                         "Bone",
                                         "Brass",
                                         "Bronze",
                                         "Cast Iron",
                                         "Chromium",
                                         "Copper",
                                         "Gold",
                                         "Iron",
                                         "Lead",
                                         "Magnesium",
                                         "Molybdenum",
                                         "Nickel",
                                         "Pewter",
                                         "Rusty",
                                         "Silver",
                                         "Steel",
                                         "Tin",
                                         "Titanium",
                                         "Tungsten",
                                         "Zirconium",
                                         "Zinc",
                                         "Aluminium Plated",
                                         "Brass Plated",
                                         "Copper Plated",
                                         "Gold Plated",
                                         "Nickel Plated",
                                         "Silver Plated",
                                         "Steel Plated",
                                         "Tin Plated",
                                         "Zinc Plated",
                                         "Uranium"};
static char const *horns[MAX_HORNS] = {
    "Bag Pipes", "Bugle",  "Conch Shell", "Fife",     "Harmonica",
    "Horn",      "Picolo", "Pipes",       "Recorder", "Reed",
    "Trumpet",   "Tuba",   "Whistle"};
static char const *rocks[MAX_ROCKS] = {
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
static char const *amulets[MAX_AMULETS] = {
    "Birch",     "Cedar",    "Dogwood",   "Driftwood", "Elm wood", "Hemlock",
    "Hickory",   "Mahogany", "Maple",     "Oak",       "Pine",     "Redwood",
    "Rosewood",  "Walnut",   "Aluminium", "Bone",      "Brass",    "Bronze",
    "Copper",    "Iron",     "Lead",      "Nickel",    "Agate",    "Amethyst",
    "Diamond",   "Emerald",  "Flint",     "Garnet",    "Jade",     "Obsidian",
    "Onyx",      "Opal",     "Pearl",     "Quartz",    "Ruby",     "Sapphire",
    "Tiger eye", "Topaz",    "Turquoise"};
static char const *cloths[MAX_CLOTHS] = {
    "Burlap",     "Cotton",      "Wool",     "Sack-cloth",
    "Rabbit-fur", "Lizard-skin", "Goat-skin"};
static char const *syllables[MAX_SYLLABLES] = {
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

/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void magic_init(__attribute__((unused)) unsigned long random_seed) {
  /*	{ Initialize all potions, wands, staves, scrolls, etc...
   * }*/

  long i1, tmpv;
  char tmps[82];

  ENTER(("magic-init", ""));

  randes();

  for (i1 = 1; i1 <= MAX_OBJECTS; i1++) {

    /*
     * The arrays of the object materals all start at 0.
     * Object subvals start at 1.  When doing the lookup
     * subtract one from subval!
     */
    tmpv = (0xFF & object_list[i1].subval);

    switch (object_list[i1].tval) {

    case potion1:
    case potion2:
      if (tmpv <= MAX_COLORS) {
        insert_str(object_list[i1].name, "%C", colors[tmpv - 1]);
      }
      break;

    case scroll1:
    case scroll2:
      rantitle(tmps);
      insert_str(object_list[i1].name, "%T", tmps);
      break;

    case ring:
      if (tmpv <= MAX_ROCKS) {
        insert_str(object_list[i1].name, "%R", rocks[tmpv - 1]);
      }
      break;

    case valuable_gems:
      if (tmpv <= MAX_ROCKS) {
        insert_str(object_list[i1].name, "%R", rocks[tmpv - 1]);
      }
      break;

    case valuable_gems_wear:
      if (tmpv <= MAX_ROCKS) {
        insert_str(object_list[i1].name, "%R", rocks[tmpv - 1]);
      }
      break;

    case amulet:
      if (tmpv <= MAX_AMULETS) {
        insert_str(object_list[i1].name, "%A", amulets[tmpv - 1]);
      }
      break;

    case wand:
      if (tmpv <= MAX_METALS) {
        insert_str(object_list[i1].name, "%M", metals[tmpv - 1]);
      }
      break;

    case chime:
      if (tmpv <= MAX_METALS) {
        insert_str(object_list[i1].name, "%M", metals[tmpv - 1]);
      }
      break;

    case horn:
      if (tmpv <= MAX_HORNS) {
        insert_str(object_list[i1].name, "%H", horns[tmpv - 1]);
      }
      break;

    case staff:
      if (tmpv <= MAX_WOODS) {
        insert_str(object_list[i1].name, "%W", woods[tmpv - 1]);
      }
      break;

    case Food:
      if (tmpv <= MAX_MUSH) {
        insert_str(object_list[i1].name, "%M", mushrooms[tmpv - 1]);
      }
      break;

    case rod:
      /* what happened to the rods? */
      /*
      if (tmpv <= MAX_RODS) {
        insert_str(object_list[i1].name,"%D",rods[tmpv-1]);
      }
      */
      break;

    case bag_or_sack:
      if (tmpv <= MAX_CLOTHS) {
        insert_str(object_list[i1].name, "%N", cloths[tmpv - 1]);
      }
      break;

    case misc_usable:
      if (tmpv <= MAX_ROCKS) {
        insert_str(object_list[i1].name, "%R", rocks[tmpv - 1]);
      }
      if (tmpv <= MAX_WOODS) {
        insert_str(object_list[i1].name, "%W", woods[tmpv - 1]);
      }
      if (tmpv <= MAX_METALS) {
        insert_str(object_list[i1].name, "%M", metals[tmpv - 1]);
      }
      if (tmpv <= MAX_AMULETS) {
        insert_str(object_list[i1].name, "%A", amulets[tmpv - 1]);
      }
      break;
    default:
      break;
    } /* end switch */
  }   /* end for */

#if DO_DEBUG && 0
  for (i1 = 1; i1 <= MAX_OBJECTS; i1++) {
    MSG(("object_list[%ld] = %s\n", i1, object_list[i1].name));
  }
#endif
  LEAVE("magic-init", "");
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */

void r__randdesloop(char const *str_array[], int count) {
  long i1;
  for (i1 = 0; i1 < count; i1++) {
    long i2 = randint(count) - 1;
    if (i1 != i2) {
      char const *tmp = str_array[i1];
      str_array[i1] = str_array[i2];
      str_array[i2] = tmp;
#if DO_DEBUG && 0
      MSG(("%2ld:%2ld \"%s\" swapped with \"%s\"\n", i1, i2, str_array[i1],
           str_array[i2]));
#endif
    }
  }
}

/*//////////////////////////////////////////////////////////////////// */

void randes() {
  /*{ Randomize colors, woods, and metals				}*/

  r__randdesloop(colors, MAX_COLORS);
  r__randdesloop(woods, MAX_WOODS);
  r__randdesloop(metals, MAX_METALS);
  r__randdesloop(horns, MAX_HORNS);
  r__randdesloop(rocks, MAX_ROCKS);
  r__randdesloop(amulets, MAX_AMULETS);
  r__randdesloop(mushrooms, MAX_MUSH);
  r__randdesloop(cloths, MAX_CLOTHS);
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */

void rantitle(char *title) {
  /*{ Return random title						}*/

  long i1, i2, i3, i4;

  i3 = randint(2) + 1; /* two or three words */
  strcpy(title, "Titled \"");

  for (i1 = 0; i1 < i3; i1++) {
    i4 = randint(2); /* one or two syllables each */
    for (i2 = 0; i2 < i4; i2++) {
      strcat(title, syllables[randint(MAX_SYLLABLES) - 1]);
    }

    if (i1 != i3 - 1) {
      strcat(title, " ");
    }
  }
  strcat(title, "\"");
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void identify(treasure_type *item) {
  /*{ Something has been identified }*/

  long i1, x1, x2;
  treas_rec *curse;

  x1 = item->tval;
  x2 = item->subval;

  if (strstr(item->name, "|") != NULL) {
    for (i1 = 0; i1 < MAX_TALLOC; i1++) {
      /* with t_list[i1] do; */
      if ((t_list[i1].tval == x1) && (t_list[i1].subval == x2)) {
        unquote(t_list[i1].name);
        known1(t_list[i1].name);
      }
    }
    for (i1 = Equipment_min; i1 <= Equipment_secondary; i1++) {
      /* with equipment[i1] do; */
      if ((equipment[i1].tval == x1) && (equipment[i1].subval == x2)) {
        unquote(equipment[i1].name);
        known1(equipment[i1].name);
      }
    }

    for (curse = inventory_list; curse != NULL; curse = curse->next) {
      /* with curse^.data do; */
      if ((curse->data.tval == x1) && (curse->data.subval == x2)) {
        unquote(curse->data.name);
        known1(curse->data.name);
      }
    }

    i1 = 0;
    do {
      i1++;
      /* with object_list[i1] do; */
      if ((object_list[i1].tval == x1) && (object_list[i1].subval == x2)) {
        if (strstr(object_list[i1].name, "%T") != NULL) {
          insert_str(object_list[i1].name, " %T|", "");
          object_ident[i1] = true;
        } else {
          unquote(object_list[i1].name);
          known1(object_list[i1].name);
          object_ident[i1] = true;
        }
      }
    } while (i1 != MAX_OBJECTS);
  } /* end if | */
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void known1(char *object_str) {
  /*{ Remove 'Secret' symbol for identity of object
   * }*/

  insert_str(object_str, "|", "");
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void known2(char *object_str) {
  /*{ Remove 'Secret' symbol for identity of pluses
   * }*/

  insert_str(object_str, "^", "");
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
void unquote(char *object_str) {
  /*	{ Return string without quoted portion }*/

  long pos0, pos1, pos2, olen;
  char str1[82], str2[82];

  pos0 = pindex(object_str, '"');
  if (pos0 > 0) {
    pos1 = pindex(object_str, '~');
    pos2 = pindex(object_str, '|');
    olen = strlen(object_str);
    strncpy(str1, object_str, pos1);
    str1[pos1] = 0;
    strncpy(str2, &(object_str[pos2]), olen - pos2);
    str2[olen - pos2] = 0;
    sprintf(object_str, "%s%s", str1, str2);
  }
}

/**
 * objdes() - Returns a description of item for inventory
 * @out_val: Where to put the return string
 * @ptr: Pointer to the object to describe
 * @pref: ???
 */
void objdes(char *out_val, treas_rec *ptr, boolean pref) {
  char *cpos;
  char tmp_val[82];

  ENTER(("objdes", "i"));

  strcpy(tmp_val, ptr->data.name);

#if DO_DEBUG
  if (do_debug_objdes) {
    MSG(("obj start: >%s<\n", tmp_val));
  }
#endif

  cpos = strstr(tmp_val, "|");
  if (cpos != NULL) {
    *cpos = '\0';
  }

#if DO_DEBUG
  if (do_debug_objdes) {
    MSG(("obj thmp1: >%s<\n", tmp_val));
  }
#endif

  cpos = strstr(tmp_val, "^");
  if (cpos != NULL) {
    *cpos = '\0';
  }

#if DO_DEBUG
  if (do_debug_objdes) {
    MSG(("obj thmp2: >%s<\n", tmp_val));
  }
#endif

  if (!pref) {
    cpos = strstr(tmp_val, " (");
    if (cpos != NULL) {
      *cpos = '\0';
    }
  }

#if DO_DEBUG
  if (do_debug_objdes) {
    MSG(("obj thmp3: >%s<\n", tmp_val));
  }
#endif

  insert_str(tmp_val, "%P0", ptr->data.damage);
  insert_num(tmp_val, "%P1", ptr->data.p1, true);
  insert_num(tmp_val, "%P2", ptr->data.tohit, true);
  insert_num(tmp_val, "%P3", ptr->data.todam, true);
  insert_num(tmp_val, "%P4", ptr->data.toac, true);
  insert_num(tmp_val, "%P5", ptr->data.p1, false);
  insert_num(tmp_val, "%P6", ptr->data.ac, false);

#if DO_DEBUG
  if (do_debug_objdes) {
    MSG(("obj thmp4: >%s<\n", tmp_val));
  }
#endif

  if (ptr->data.number != 1) {
    insert_str(tmp_val, "ch~", "ches");
    insert_str(tmp_val, "y~", "ies");
    insert_str(tmp_val, "~", "s");
  } else {
    insert_str(tmp_val, "~", "");
  }

#if DO_DEBUG
  if (do_debug_objdes) {
    MSG(("obj thmp5: >%s<\n", tmp_val));
  }
#endif

  if (pref) {
    if (strstr(tmp_val, "&") != NULL) {
      insert_str(tmp_val, "&", "");

      if (ptr->data.number > 1) {
        sprintf(out_val, "%d%s", ptr->data.number, tmp_val);
      } else if (ptr->data.number < 1) {
        sprintf(out_val, "no more%s", tmp_val);
      } else if (is_vowel(tmp_val[1])) {
        sprintf(out_val, "an%s", tmp_val);
      } else {
        sprintf(out_val, "a%s", tmp_val);
      }

    } else {
      if (ptr->data.number > 0) {
        strcpy(out_val, tmp_val);
      } else {
        sprintf(out_val, "no more %s", tmp_val);
      }
    }

  } else {
    insert_str(tmp_val, "& ", "");
    strcpy(out_val, tmp_val);
  }

#if DO_DEBUG
  if (do_debug_objdes) {
    MSG(("obj final: >%s<\n", out_val));
  }
#endif

  LEAVE("objdes", "i");
}

char *bag_descrip(treas_rec *bag, char result[134]) /* was func */
{
  /*{ Return description about the contents of a bag	-DMF-	}*/

  long count, wgt;
  treas_rec *ptr;

  if ((bag->next == NULL) || (bag->next->is_in == false)) {
    sprintf(result, " (empty)");
  } else {
    count = 0;
    wgt = 0;

    for (ptr = bag->next; (ptr != NULL) && (ptr->is_in); ptr = ptr->next) {
      count += ptr->data.number;
      wgt += ptr->data.weight * ptr->data.number;
    }

    sprintf(result, " (%ld%% full, containing %ld item%s)",
            (wgt * 100 / bag->data.p1), count, ((count != 1) ? "s" : ""));
  }
  return result;
}
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */

/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */

/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */

/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */
/*//////////////////////////////////////////////////////////////////// */

/* END FILE  desc.c */
