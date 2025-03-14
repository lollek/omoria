#include "monster_template.h"

#include <stddef.h>

long m_level[MAX_MONS_LEVEL + 1] = {0};

monster_template const monster_templates[] = {
  // Level 0
    {10, 1, "<<Placeholder>>", 0x0010C000, 0x00009F52, 0x3000, 25000, 50, 1, 'p', "20d8", "1 1 3d3", 0, 20, {.multiplies = false, .can_move = true}},
    {10, 1, "Town Wizard", 0x0010C000, 0x00009F52, 0x3000, 25000, 50, 1, 'p', "20d8", "1 1 3d3", 0, 20, {.multiplies = false, .can_move = true}},
    {10, 1, "Town Guard", 0x0010C000, 0x00000000, 0x3000, 25000, 25, 1, 'p', "20d8", "1 1 4d4|1 1 4d4", 0, 0, {.multiplies = false, .can_move = true}},
    {4, 1, "Young Child", 0x14124342, 0x00000000, 0x2034, 50, 10, 1, 'p', "1d2", "1 6 0d0", 0, 0, {.multiplies = false, .can_move = true}},
    {20, 20, "Goldfish", 0x00004054, 0x00000000, 0x00F0, 0, 5, 1, 'f', "0d0", "1 32 0d0", 0, 0, {.multiplies = false, .can_move = true}},
    {20, 10, "Chicken", 0x00000348, 0x00000000, 0x20F0, 10, 5, 2, 'c', "1d1", "1 34 0d0|1 37 0d0", 0, 0, {.multiplies = false, .can_move = true}},
    {20, 10, "Duck", 0x00004348, 0x00000000, 0x20F0, 10, 5, 2, 'd', "1d1", "1 33 0d0", 0, 0, {.multiplies = false, .can_move = true}},
    {20, 10, "Mongrel Dog", 0x00004348, 0x00000000, 0x20F0, 10, 5, 2, 'd', "1d1", "1 35 0d0", 0, 0, {.multiplies = false, .can_move = true}},
    {20, 10, "Stray Kitten", 0x00004348, 0x00000000, 0x20F0, 10, 5, 2, 'k', "1d1", "1 36 0d0", 0, 0, {.multiplies = false, .can_move = true}},
    {20, 20, "Cow", 0x00004348, 0x00000000, 0x20F0, 10, 10, 1, 'B', "3d8", "1 26 0d0", 0, 0, {.multiplies = false, .can_move = true}},
    {20, 25, "Raging Bull", 0x00080348, 0x00000000, 0x20F0, 50, 0, 2, 'B', "4d8", "1 25 1d6", 0, 0, {.multiplies = false, .can_move = true}},
    {4, 1, "Filthy Street Urchin", 0x00120342, 0x00000000, 0x2034, 400, 0, 1, 'p', "1d4", "1 14 0d0|12 5 0d0", 0, 0, {.multiplies = false, .can_move = true}},
    {6, 1, "Blubbering Idiot", 0x00124342, 0x00000000, 0x2030, 0, 10, 1, 'p', "1d2", "1 18 0d0", 0, 0, {.multiplies = false, .can_move = true}},
    {10, 1, "Pitiful Looking Beggar", 0x00124342, 0x00000000, 0x2030, 400, 10, 1, 'p', "1d4", "1 14 0d0", 0, 0, {.multiplies = false, .can_move = true}},
    {10, 1, "Mangy Looking Leper", 0x00124342, 0x00000000, 0x2030, 500, 10, 1, 'p', "1d1", "1 14 0d0", 0, 0, {.multiplies = false, .can_move = true}},
    {10, 8, "Squint Eyed Rogue", 0x07120340, 0x00000000, 0x2034, 999, 0, 1, 'p', "2d8", "1 1 1d6|13 5 0d0", 0, 0, {.multiplies = false, .can_move = true}},
    {10, 1, "Singing, Happy Drunk", 0x0612434E, 0x00000000, 0x2030, 0, 10, 1, 'p', "2d3", "1 14 0d0", 0, 0, {.multiplies = false, .can_move = true}},
    {10, 20, "Mean Looking Mercenary", 0x0B120342, 0x00000000, 0x2034, 25000, 0, 1, 'p', "5d8", "1 1 1d10", 0, 0, {.multiplies = false, .can_move = true}},
    {10, 30, "Battle Scarred Veteran", 0x0B120342, 0x00000000, 0x2030, 25000, 0, 1, 'p', "7d8", "1 1 2d6", 0, 0, {.multiplies = false, .can_move = true}},
  // Level 1
    {6, 5, "Brown Imp", 0x00000204, 0x00000000, 0x00F6, 8, 2, 1, 'i', "3d4", "1 3 1d3", 1, 0, {.multiplies = true, .can_move = true}},
    {2, 1, "Mimic", 0x00000041, 0x00000028, 0x0000, 0, 2, 1, '<', "5d4", "10 5 0d0", 1, 0, {.multiplies = false, .can_move = true}},
    {2, 1, "Grey Mushroom patch", 0x00000040, 0x00000000, 0x10A0, 0, 1, 1, ',', "1d2", "3 13 1d4", 1, 0, {.multiplies = false, .can_move = false}},
    {8, 12, "Giant Yellow Centipede", 0x00000040, 0x00000000, 0x0002, 300, 2, 1, 'c', "2d6", "1 2 1d3|1 4 1d3", 1, 0, {.multiplies = false, .can_move = true}},
    {7, 10, "Giant White Centipede", 0x00000042, 0x00000000, 0x0002, 400, 2, 1, 'c', "3d5", "1 2 1d2|1 4 1d2", 1, 0, {.multiplies = false, .can_move = true}},
    {8, 4, "White Rat-Man", 0x00000202, 0x00000000, 0x2070, 200, 1, 1, 'r', "1d3", "1 2 1d2", 1, 0, {.multiplies = true, .can_move = true}},
    {4, 35, "Large Brown Snake", 0x00000142, 0x00000000, 0x00B0, 999, 3, 0, 'R', "4d6", "1 2 1d3|1 16 1d4", 1, 0, {.multiplies = false, .can_move = true}},
    {4, 30, "Large White Snake", 0x00000144, 0x00000000, 0x00B0, 999, 2, 1, 'R', "3d6", "1 2 1d1", 1, 0, {.multiplies = false, .can_move = true}},
    {20, 16, "Kobold", 0x07020200, 0x00000000, 0x2032, 100, 5, 1, 'k', "3d7", "1 1 1d6", 1, 0, {.multiplies = false, .can_move = true}},
    {6, 1, "White Worm mass", 0x00000148, 0x00000000, 0x01B0, 1, 2, 0, 'w', "4d4", "14 12 1d2", 1, 0, {.multiplies = true, .can_move = true}},
    {2, 6, "Floating Eye", 0x00800000, 0x0001000D, 0x2102, 100, 1, 1, 'e', "3d6", "11 7 0d0", 1, 0, {.multiplies = false, .can_move = false}},
  // Level 2
    {6, 5, "Black Imp", 0x00000204, 0x00000000, 0x00F6, 10, 3, 1, 'i', "2d3", "6 2 1d3|1 3 1d4", 2, 0, {.multiplies = true, .can_move = true}},
    {2, 1, "Shrieker Mushroom patch", 0x00000000, 0x00000000, 0x10A0, 0, 1, 1, ',', "1d1", "20 10 0d0", 2, 0, {.multiplies = false, .can_move = false}},
    {5, 4, "Metallic Green Centipede", 0x00000144, 0x00000000, 0x0000, 100, 3, 1, 'c', "4d4", "1 12 1d1", 2, 0, {.multiplies = false, .can_move = true}},
    {1, 1, "Glowing Chartreuse Potion", 0x00000040, 0x00000000, 0x11F2, 0, 2, 1, '!', "7d4", "1 2 1d3|1 3 1d3", 2, 0, {.multiplies = false, .can_move = false}},
    {20, 16, "Novice Bard", 0x07020200, 0x0000040C, 0x2070, 50, 6, 1, 'p', "7d4", "1 1 1d5|11 23 1d2", 2, 0, {.multiplies = false, .can_move = true}},
    {20, 16, "Novice Warrior", 0x07020200, 0x00000000, 0x2070, 50, 6, 1, 'p', "9d4", "1 1 1d7", 2, 0, {.multiplies = false, .can_move = true}},
    {20, 12, "Novice Rogue", 0x07020200, 0x00000000, 0x2070, 50, 6, 1, 'p', "8d4", "1 1 1d6|12 5 0d0", 2, 0, {.multiplies = false, .can_move = true}},
    {20, 10, "Novice Priest", 0x07020200, 0x0000108C, 0x2070, 100, 7, 1, 'p', "7d4", "1 1 1d5", 2, 0, {.multiplies = false, .can_move = true}},
    {20, 6, "Novice Mage", 0x07020200, 0x0000089C, 0x2070, 50, 7, 1, 'p', "6d4", "1 1 1d4", 2, 0, {.multiplies = false, .can_move = true}},
    {2, 1, "Yellow Mushroom patch", 0x00000040, 0x00000000, 0x10A0, 0, 2, 1, ',', "1d1", "4 13 1d6", 2, 0, {.multiplies = false, .can_move = false}},
    {2, 1, "White Jelly", 0x0B000040, 0x00000000, 0x01A0, 999, 10, 1, 'J', "8d8", "14 5 1d2", 2, 0, {.multiplies = false, .can_move = false}},
    {14, 8, "Giant Green Frog", 0x00001212, 0x00000000, 0x0082, 300, 6, 2, 'a', "2d8", "1 2 1d3", 2, 0, {.multiplies = false, .can_move = true}},
    {14, 20, "Giant Black Ant", 0x00000142, 0x00000000, 0x0002, 400, 8, 1, 'c', "3d6", "1 2 1d4", 2, 0, {.multiplies = false, .can_move = true}},
    {16, 16, "White Harpy", 0x00800304, 0x00000000, 0x2036, 100, 5, 1, 'h', "2d5", "1 3 1d1|1 3 1d1|1 2 1d2", 2, 0, {.multiplies = false, .can_move = true}},
    {18, 14, "Blue Yeek", 0x07020240, 0x00000000, 0x2030, 100, 4, 1, 'y', "2d6", "1 1 1d5", 2, 0, {.multiplies = false, .can_move = true}},
    {7, 3, "Green Worm mass", 0x00000148, 0x00000000, 0x01B0, 100, 3, 0, 'w', "6d4", "9 12 1d3", 2, 0, {.multiplies = true, .can_move = true}},
    {5, 38, "Large Black Snake", 0x00000142, 0x00000000, 0x00B0, 750, 9, 0, 'R', "4d8", "1 2 1d4|1 16 1d6", 2, 0, {.multiplies = false, .can_move = true}},
    {4, 10, "Giant Kelp", 0x00400012, 0x00000000, 0x1030, 0, 3, 1, 'w', "2d4", "1 29 1d8", 2, 0, {.multiplies = true, .can_move = false}},
  // Level 3
    {6, 10, "Fire Imp", 0x00000208, 0x00000000, 0x00D6, 15, 4, 1, 'i', "3d3", "5 3 2d3", 3, 0, {.multiplies = true, .can_move = true}},
    {8, 15, "Poltergeist", 0x0F95030E, 0x0000001F, 0x100C, 750, 6, 3, 'G', "2d5", "4 1 1d1", 3, 0, {.multiplies = false, .can_move = true}},
    {6, 6, "Metallic Blue Centipede", 0x00000144, 0x00000000, 0x0000, 150, 7, 2, 'c', "4d5", "1 12 1d2", 3, 0, {.multiplies = false, .can_move = true}},
    {1, 1, "Scroll Titled \"taf naed\"", 0x00000040, 0x00000000, 0x1010, 0, 7, 1, '?', "10d8", "1 1 3d2", 3, 0, {.multiplies = false, .can_move = false}},
    {16, 40, "Black Naga", 0x07100302, 0x00000000, 0x20E6, 1200, 20, 1, 'n', "6d8", "1 16 1d8", 3, 0, {.multiplies = false, .can_move = true}},
    {2, 1, "Spotted Mushroom patch", 0x00000040, 0x00000000, 0x10A0, 0, 3, 1, ',', "1d1", "14 13 2d4", 3, 0, {.multiplies = false, .can_move = false}},
    {2, 1, "Yellow Jelly", 0x0B000040, 0x0001000F, 0x01A0, 999, 12, 1, 'J', "10d8", "14 5 1d3", 3, 0, {.multiplies = false, .can_move = false}},
    {16, 8, "Scruffy looking Hobbit", 0x05020200, 0x00000000, 0x2070, 100, 4, 1, 'p', "3d5", "1 1 1d4|12 5 0d0", 3, 0, {.multiplies = false, .can_move = true}},
    {8, 12, "Huge Brown Bat", 0x00800308, 0x00000000, 0x2160, 400, 4, 3, 'b', "2d6", "1 2 1d2", 3, 0, {.multiplies = false, .can_move = true}},
    {8, 16, "Giant White Ant", 0x00000140, 0x00000000, 0x0002, 800, 7, 1, 'c', "3d6", "1 2 1d4", 3, 0, {.multiplies = false, .can_move = true}},
    {2, 10, "Yellow Mold", 0x0B000040, 0x00000000, 0x10A0, 999, 9, 1, 'm', "8d8", "1 1 1d4", 3, 0, {.multiplies = false, .can_move = false}},
    {8, 9, "Metallic Red Centipede", 0x00000142, 0x00000000, 0x0092, 200, 12, 2, 'c', "4d8", "1 12 1d2", 3, 0, {.multiplies = false, .can_move = true}},
    {7, 4, "Yellow Worm mass", 0x00000148, 0x00000000, 0x01B0, 100, 4, 0, 'w', "4d8", "15 12 1d3", 3, 0, {.multiplies = true, .can_move = true}},
    {5, 40, "Large Grey Snake", 0x00000242, 0x00000000, 0x00B0, 700, 10, 0, 'R', "6d8", "1 2 1d4|1 16 1d6", 3, 0, {.multiplies = false, .can_move = true}},
    {2, 6, "Radiation Eye", 0x00800300, 0x0001000B, 0x2102, 100, 6, 1, 'e', "3d6", "2 7 1d6", 3, 0, {.multiplies = false, .can_move = false}},
    {16, 22, "Drooling Harpy", 0x00800304, 0x00000000, 0x2036, 100, 7, 1, 'h', "2d8", "1 3 1d1|1 3 1d1|1 2 1d2|1 18 0d0", 3, 0, {.multiplies = false, .can_move = true}},
  // Level 4
    {6, 10, "Ice Imp", 0x00000204, 0x00000000, 0x00E6, 20, 4, 1, 'i', "5d4", "7 3 3d3", 4, 0, {.multiplies = true, .can_move = true}},
    {8, 5, "Silver Rat-Man", 0x00000202, 0x00000000, 0x2072, 100, 1, 1, 'r', "1d1", "23 1 1d1|1 1 1d1", 4, 0, {.multiplies = true, .can_move = true}},
    {8, 10, "Pirhana", 0x00000050, 0x00000000, 0x00F0, 0, 4, 2, 'f', "1d8", "1 2 2-1d2", 4, 0, {.multiplies = true, .can_move = true}},
    {2, 1, "Black Mushroom patch", 0x00000040, 0x00000000, 0x10A0, 0, 8, 1, ',', "8d8", "1 13 1d3", 4, 0, {.multiplies = false, .can_move = false}},
    {2, 1, "Blue Jelly", 0x0B000040, 0x00000000, 0x01A0, 999, 14, 1, 'J', "12d8", "7 5 1d6", 4, 0, {.multiplies = false, .can_move = false}},
    {3, 24, "Creeping Copper Coins", 0x06000300, 0x00000000, 0x0002, 100, 9, 0, '$', "7d8", "1 2 1d4|14 8 2d4", 4, 0, {.multiplies = false, .can_move = true}},
    {8, 7, "Giant White Rat", 0x00000202, 0x00000000, 0x2070, 300, 1, 1, 'r', "2d2", "14 2 1d3", 4, 0, {.multiplies = true, .can_move = true}},
    {8, 20, "Giant Black Centipede", 0x00000248, 0x00000000, 0x0002, 300, 11, 1, 'c', "5d8", "1 2 1d2|1 4 1d2", 4, 0, {.multiplies = false, .can_move = true}},
    {8, 20, "Giant Blue Centipede", 0x00000240, 0x00000000, 0x0002, 500, 10, 1, 'c', "4d8", "1 2 1d3|1 4 1d4", 4, 0, {.multiplies = false, .can_move = true}},
    {7, 12, "Blue Worm mass", 0x00000248, 0x00000000, 0x01B0, 100, 5, 0, 'w', "5d8", "7 12 1d4", 4, 0, {.multiplies = true, .can_move = true}},
    {6, 41, "Large Grey Snake", 0x00000242, 0x00000000, 0x00B0, 500, 14, 0, 'R', "6d8", "1 2 1d5|1 16 1d8", 4, 0, {.multiplies = false, .can_move = true}},
    {12, 16, "Jackal", 0x00000204, 0x00000000, 0x2030, 300, 8, 1, 'j', "3d8", "1 2 1d6", 4, 0, {.multiplies = false, .can_move = true}},
  // Level 5
    {20, 40, "Green Naga", 0x07100302, 0x00000000, 0x2066, 1200, 30, 1, 'n', "9d8", "1 16 1d8|6 9 2d6", 5, 0, {.multiplies = false, .can_move = true}},
    {10, 20, "Green Glutton Ghost", 0x0F950300, 0x0000003F, 0x100C, 100, 15, 3, 'G', "3d6", "22 15 1d1", 5, 0, {.multiplies = false, .can_move = true}},
    {2, 1, "White Mushroom patch", 0x00000040, 0x00000000, 0x10A0, 0, 5, 1, ',', "1d1", "11 13 2d4", 5, 0, {.multiplies = false, .can_move = false}},
    {2, 1, "Green Jelly", 0x0B000040, 0x00000000, 0x01A0, 999, 18, 1, 'J', "22d8", "9 5 1d2", 5, 0, {.multiplies = false, .can_move = false}},
    {20, 26, "Skeleton Kobold", 0x00020300, 0x00000000, 0x000C, 400, 12, 1, 's', "5d8", "1 1 1d6", 5, 0, {.multiplies = false, .can_move = true}},
    {2, 25, "Silver Jelly", 0x0B000040, 0x00000000, 0x00B0, 1, 15, 1, 'J', "20d8", "23 5 1d3", 5, 0, {.multiplies = false, .can_move = false}},
    {12, 18, "Giant Warty Frog", 0x00000312, 0x00000000, 0x00A2, 400, 12, 1, 'a', "4d8", "1 2 1d6", 5, 0, {.multiplies = false, .can_move = true}},
    {4, 10, "Disenchanter Eye", 0x00800303, 0x00010009, 0x2102, 100, 20, 1, 'e', "7d8", "21 7 0d0", 5, 5, {.multiplies = false, .can_move = true}},
    {18, 16, "Black Yeek", 0x07020240, 0x00000000, 0x2030, 100, 8, 1, 'y', "2d8", "1 1 1d5", 5, 0, {.multiplies = false, .can_move = true}},
    {7, 12, "Red Worm mass", 0x00000148, 0x00000000, 0x21B0, 100, 6, 0, 'w', "5d8", "5 12 1d6", 5, 0, {.multiplies = true, .can_move = true}},
    {12, 16, "Giant House Fly", 0x00800304, 0x00000000, 0x0062, 200, 10, 3, 'F', "3d8", "1 2 1d2", 5, 0, {.multiplies = false, .can_move = true}},
    {6, 20, "Copperhead Snake", 0x00000244, 0x00000000, 0x00B0, 1, 15, 1, 'R', "4d6", "14 2 2d4", 5, 0, {.multiplies = false, .can_move = true}},
    {2, 30, "Rot Jelly", 0x0B000040, 0x00000000, 0x00B0, 1, 15, 1, 'J', "20d8", "22 5 2d3", 5, 0, {.multiplies = false, .can_move = false}},
  // Level 6
    {35, 80, "Swirling Red Vapor", 0x00800308, 0x00000000, 0x1100, 20, 15, 2, 'v', "3d8", "1 5 1d4", 6, 0, {.multiplies = false, .can_move = true}},
    {12, 16, "Blink Dog", 0x00000304, 0x00000013, 0x2030, 300, 12, 1, 'j', "3d8", "1 2 1d6", 6, 0, {.multiplies = false, .can_move = true}},
    {2, 1, "Purple Mushroom patch", 0x00000040, 0x00000000, 0x10A0, 0, 12, 1, ',', "1d1", "16 13 1d2", 6, 0, {.multiplies = false, .can_move = false}},
    {2, 12, "Brown Mold", 0x0B000040, 0x00000000, 0x10A0, 999, 20, 1, 'm', "15d8", "3 1 1d4", 6, 0, {.multiplies = false, .can_move = false}},
    {10, 15, "Giant Brown Bat", 0x00800306, 0x00000000, 0x2160, 300, 10, 3, 'b', "3d8", "1 2 1d3", 6, 0, {.multiplies = false, .can_move = true}},
    {4, 30, "Creeping Silver Coins", 0x0A000300, 0x00000000, 0x0002, 100, 18, 0, '$', "12d8", "1 2 1d6|14 8 2d6", 6, 0, {.multiplies = false, .can_move = true}},
    {20, 32, "Orc", 0x0B020200, 0x00000000, 0x2036, 300, 16, 1, 'o', "9d8", "1 1 1d8", 6, 0, {.multiplies = false, .can_move = true}},
    {16, 20, "Grey Harpy", 0x00800310, 0x00000000, 0x2036, 100, 14, 2, 'h', "3d8", "1 3 1d2|1 3 1d2|1 2 1d2", 6, 0, {.multiplies = false, .can_move = true}},
    {6, 24, "Rattlesnake", 0x00000244, 0x00000000, 0x00B0, 1, 20, 1, 'R', "6d7", "14 2 2d5", 6, 0, {.multiplies = false, .can_move = true}},
  // Level 7
    {2, 6, "Bloodshot Eye", 0x00800300, 0x00010007, 0x2102, 100, 15, 1, 'e', "4d8", "10 7 2d6", 7, 0, {.multiplies = false, .can_move = false}},
    {20, 40, "Red Naga", 0x07100302, 0x00000000, 0x20E6, 1200, 40, 1, 'n', "11d8", "1 16 1d10|2 2 1d4", 7, 0, {.multiplies = false, .can_move = true}},
    {2, 1, "Red Jelly", 0x0B000040, 0x00000000, 0x01A0, 999, 26, 1, 'J', "26d8", "2 5 1d5", 7, 0, {.multiplies = false, .can_move = false}},
    {12, 16, "Giant Red Frog", 0x00000312, 0x00000000, 0x00A2, 500, 16, 1, 'a', "5d8", "2 2 2d4", 7, 0, {.multiplies = false, .can_move = true}},
    {20, 14, "Zombie Kobold", 0x00020300, 0x00000000, 0x002E, 300, 14, 1, 'z', "6d8", "1 1 1d2|1 1 1d2", 7, 0, {.multiplies = false, .can_move = true}},
    {20, 14, "Walking Dead", 0x00020302, 0x00001006, 0x002E, 300, 12, 0, 'z', "3d6", "1 1 1d2|1 1 1d2|14 8 1d2", 7, 0, {.multiplies = true, .can_move = true}},
    {4, 10, "Lost Soul", 0x00950306, 0x0001002F, 0x100C, 100, 18, 1, 'G', "2d8", "1 1 2d2|18 5 0d0", 7, 0, {.multiplies = false, .can_move = true}},
    {18, 14, "Greedy little Gnome", 0x0B020240, 0x00000000, 0x2070, 100, 13, 1, 'p', "3d8", "1 1 1d7|13 5 0d0", 7, 0, {.multiplies = false, .can_move = true}},
    {12, 14, "Giant Green Fly", 0x00800308, 0x00000000, 0x0062, 500, 15, 2, 'F', "3d8", "1 2 1d4", 7, 0, {.multiplies = false, .can_move = true}},
  // Level 8
    {10, 25, "Red Faerie Dragon", 0x11820302, 0x00008496, 0x4001, 50, 25, 2, 'F', "3d8", "3 3 1d5", 8, 0, {.multiplies = false, .can_move = true}},
    {18, 18, "Brown Yeek", 0x07020240, 0x00000000, 0x2030, 100, 11, 1, 'y', "3d8", "1 1 1d6", 8, 0, {.multiplies = false, .can_move = true}},
    {2, 14, "Green Mold", 0x0B000040, 0x00000000, 0x10A0, 750, 28, 1, 'm', "21d8", "4 1 1d4", 8, 0, {.multiplies = false, .can_move = false}},
    {20, 36, "Skeleton Orc", 0x00020300, 0x00000000, 0x100C, 400, 26, 1, 's', "10d8", "1 1 2d5", 8, 0, {.multiplies = false, .can_move = true}},
    {20, 26, "Seedy looking Human", 0x13020200, 0x00000000, 0x2034, 200, 22, 1, 'p', "8d8", "1 1 3d4", 8, 0, {.multiplies = false, .can_move = true}},
    {20, 24, "Bandit", 0x13020200, 0x00000000, 0x2030, 999, 26, 1, 'p', "8d8", "1 1 2d4|12 5 0d0", 8, 0, {.multiplies = false, .can_move = true}},
  // Level 9
    {20, 25, "Giant Squid", 0x00000250, 0x00000000, 0x0042, 500, 25, 2, 'C', "4d4", "10 28 0d0|1 11 1d8", 9, 0, {.multiplies = false, .can_move = true}},
    {20, 24, "Yeti", 0x00020200, 0x00000000, 0x2024, 100, 30, 1, 'Y', "11d8", "1 3 1d3|1 3 1d3|1 2 1d4", 9, 0, {.multiplies = false, .can_move = true}},
    {10, 12, "Giant Grey Rat", 0x00000202, 0x00000000, 0x2070, 200, 2, 1, 'r', "2d3", "14 2 1d4", 9, 0, {.multiplies = true, .can_move = true}},
    {16, 22, "Black Harpy", 0x00800302, 0x00000000, 0x2036, 100, 19, 2, 'h', "3d8", "1 3 1d2|1 3 1d2|1 2 1d3", 9, 0, {.multiplies = false, .can_move = true}},
    {12, 18, "Giant Black Bat", 0x00800304, 0x00000000, 0x2060, 250, 16, 3, 'b', "2d8", "1 2 1d6", 9, 0, {.multiplies = false, .can_move = true}},
    {18, 24, "Clear Yeek", 0x07030240, 0x00000000, 0x0030, 100, 14, 1, 'y', "3d6", "1 1 1d5", 9, 0, {.multiplies = false, .can_move = true}},
    {20, 15, "Orc Shaman", 0x0B020200, 0x00008085, 0x2036, 200, 30, 1, 'o', "7d8", "1 1 1d6", 9, 0, {.multiplies = false, .can_move = true}},
    {12, 34, "Giant Red Ant", 0x00000140, 0x00000000, 0x0002, 600, 22, 1, 'c', "4d8", "1 2 1d4|2 4 1d4", 9, 0, {.multiplies = false, .can_move = true}},
    {8, 30, "King Cobra", 0x00000244, 0x00000000, 0x00B0, 1, 28, 1, 'R', "8d8", "10 9 1d2|14 2 3d4", 9, 0, {.multiplies = false, .can_move = true}},
  // Level 10
    {4, 1, "Clear Mushroom patch", 0x00010040, 0x00000000, 0x00A0, 0, 1, 2, ',', "1d1", "1 13 1d1", 10, 0, {.multiplies = true, .can_move = false}},
    {12, 40, "Giant White Tick", 0x00000242, 0x00000000, 0x0022, 200, 27, 0, 't', "15d8", "14 2 2d6", 10, 0, {.multiplies = false, .can_move = true}},
    {2, 15, "Hairy Mold", 0x0B000040, 0x00000000, 0x10A0, 700, 32, 1, 'm', "15d8", "14 1 1d3", 10, 0, {.multiplies = false, .can_move = false}},
    {2, 20, "Disenchanter Mold", 0x0B000040, 0x0001000B, 0x10A0, 100, 40, 1, 'm', "16d8", "21 5 1d6", 10, 10, {.multiplies = false, .can_move = false}},
    {12, 26, "Giant Red Centipede", 0x00000240, 0x00000000, 0x0082, 500, 24, 2, 'c', "3d8", "1 2 1d2|14 4 1d2", 10, 0, {.multiplies = false, .can_move = true}},
    {5, 36, "Creeping Gold Coins", 0x0E000300, 0x00000000, 0x0002, 100, 32, 0, '$', "18d8", "1 2 2d5|14 8 3d5", 10, 0, {.multiplies = false, .can_move = true}},
    {8, 14, "Giant Fruit Fly", 0x00800308, 0x00000000, 0x0062, 500, 3, 2, 'F', "2d2", "1 2 1d2", 10, 0, {.multiplies = true, .can_move = true}},
    {15, 85, "Swirling Blue Vapor", 0x00800308, 0x80C80001, 0x1000, 1, 20, 2, 'v', "5d8", "7 5 2d4", 10, 0, {.multiplies = false, .can_move = true}},
    {20, 20, "Demon Larva", 0x00000308, 0x00010008, 0x0406, 20, 10, 0, 'w', "5d6", "14 12 1d4", 10, 0, {.multiplies = true, .can_move = true}},
    {20, 5, "Nymph", 0x13120210, 0x000004AA, 0x1000, 500, 30, 2, 'N', "8d8", "13 5 0d0", 10, 0, {.multiplies = false, .can_move = true}},
    {20, 12, "Siren", 0x06120202, 0x00018A43, 0x10F0, 400, 35, 1, 'p', "10d8", "18 5 1d1|1 1 2d3", 10, 0, {.multiplies = false, .can_move = true}},
    {20, 32, "Brigand", 0x13020200, 0x00000000, 0x2030, 100, 35, 1, 'p', "9d8", "1 1 2d4|13 5 0d0", 10, 0, {.multiplies = false, .can_move = true}},
    {5, 25, "Alligator", 0x00000310, 0x00000000, 0x2022, 150, 150, 1, 'C', "25d8", "1 2 3d6", 10, 0, {.multiplies = false, .can_move = true}},
    {6, 10, "Evil-Looking Man", 0x19000000, 0x00010486, 0x10F0, 50, 100, 1, 'p', "10d6", "1 1 6-2d2", 10, 0, {.multiplies = false, .can_move = true}},
  // Level 11
    {2, 20, "Giant Anemone", 0x14400050, 0x00000000, 0x00B2, 5000, 75, 0, 'A', "10d8", "11 5 1d8|14 4 4d4", 11, 0, {.multiplies = false, .can_move = false}},
    {20, 24, "Orc Zombie", 0x00020100, 0x00000000, 0x102E, 250, 30, 1, 'z', "11d8", "1 1 1d4|1 1 1d4", 11, 0, {.multiplies = false, .can_move = true}},
    {20, 36, "Orc Warrior", 0x0F020200, 0x00000000, 0x2036, 250, 34, 1, 'o', "11d8", "1 1 2d6", 11, 0, {.multiplies = false, .can_move = true}},
    {18, 10, "Nasty little Gnome", 0x0B020200, 0x000020B5, 0x2030, 10, 32, 1, 'p', "4d8", "1 1 1d5", 11, 0, {.multiplies = false, .can_move = true}},
    {20, 38, "Hobgoblin", 0x0F020200, 0x00000000, 0x2036, 300, 38, 1, 'H', "12d8", "1 1 1d10", 11, 0, {.multiplies = false, .can_move = true}},
    {10, 30, "Cockatrice", 0x00000042, 0x08000002, 0x1026, 250, 75, 1, 'c', "10d8", "1 33 0d0|1 34 1d1|26 7 7d2", 11, 0, {.multiplies = false, .can_move = true}},
  // Level 12
    {8, 10, "Unholy Pirhana", 0x00000050, 0x00000000, 0x00F4, 0, 4, 2, 'f', "2d8", "1 2 2-1d6", 12, 0, {.multiplies = true, .can_move = true}},
    {10, 50, "Water Moccasin", 0x00000210, 0x00000000, 0x00B2, 100, 75, 2, 'R', "10d8", "1 16 1d8|14 2 4d4", 12, 0, {.multiplies = false, .can_move = true}},
    {20, 32, "Black Mamba", 0x00000244, 0x00000000, 0x00B0, 1, 40, 2, 'R', "10d8", "14 2 4d4", 12, 0, {.multiplies = false, .can_move = true}},
    {2, 1, "Grape Jelly", 0x0B000040, 0x0001000B, 0x01A0, 999, 60, 1, 'J', "52d8", "19 5 5d8", 12, 0, {.multiplies = false, .can_move = false}},
    {18, 24, "Master Yeek", 0x07020240, 0x00008018, 0x2030, 100, 28, 1, 'y', "5d8", "1 1 1d8", 12, 0, {.multiplies = false, .can_move = true}},
    {20, 22, "Priest", 0x13020240, 0x00000285, 0x2030, 400, 36, 1, 'p', "7d8", "1 1 2d3", 12, 0, {.multiplies = false, .can_move = true}},
    {20, 22, "Bard", 0x13020200, 0x00000285, 0x2030, 400, 45, 1, 'p', "9d8", "1 1 2d5", 12, 0, {.multiplies = false, .can_move = true}},
    {20, 22, "Monk", 0x13020200, 0x00000000, 0x2030, 100, 36, 1, 'p', "9d8", "1 1 2-2d5", 12, 0, {.multiplies = false, .can_move = true}},
    {4, 60, "Animated Sword", 0x008A0300, 0x00000000, 0x5000, 10, 45, 2, '|', "4d8", "1 1 2d4", 12, 0, {.multiplies = false, .can_move = true}},
    {40, 90, "Swirling Green Vapor", 0x00800308, 0x80FE0001, 0x5000, 1, 40, 2, 'v', "7d8", "14 5 2d4", 12, 0, {.multiplies = false, .can_move = true}},
    {12, 18, "Giant Clear Ant", 0x00010240, 0x00000000, 0x0102, 600, 24, 1, 'c', "3d7", "1 2 1d4", 12, 0, {.multiplies = false, .can_move = true}},
    {12, 20, "Air Spirit", 0x00830308, 0x00000000, 0x0010, 200, 40, 3, 'E', "5d8", "1 1 1d3", 12, 0, {.multiplies = false, .can_move = true}},
    {20, 30, "Skeleton Human", 0x00020300, 0x00000000, 0x100C, 300, 38, 1, 's', "12d8", "1 1 1d8", 12, 0, {.multiplies = false, .can_move = true}},
    {20, 24, "Human Zombie", 0x00020300, 0x00000000, 0x102E, 200, 34, 1, 'z', "11d8", "1 1 1d4|1 1 1d4", 12, 0, {.multiplies = false, .can_move = true}},
    {6, 20, "Moaning Spirit", 0x0F950302, 0x0001002E, 0x100C, 300, 44, 1, 'G', "4d8", "4 10 0d0|15 5 1d8", 12, 0, {.multiplies = false, .can_move = true}},
    {10, 40, "Demonic Spirit", 0x0F950302, 0x00010008, 0x1404, 250, 35, 1, 'G', "4d7", "3 26 0d0| 25 5 1d8", 12, 0, {.multiplies = false, .can_move = true}},
    {20, 34, "Swordsman", 0x13020200, 0x00000000, 0x2030, 999, 40, 1, 'p', "11d8", "1 1 3d5", 12, 0, {.multiplies = false, .can_move = true}},
  // Level 13
    {8, 75, "Vorpal Bunny", 0x18000240, 0x00000000, 0x7000, 0, 100, 3, 'r', "12d8", "1 31 3-1d4", 13, 0, {.multiplies = false, .can_move = true}},
    {8, 10, "Kracken", 0x00000050, 0x00000000, 0x90F2, 0, 4, 2, 'R', "10d8", "1 2 2-2d6", 13, 0, {.multiplies = false, .can_move = true}},
    {10, 40, "Killer Brown Beetle", 0x00000242, 0x00000000, 0x0002, 300, 38, 1, 'K', "1d8", "1 2 3d4", 13, 0, {.multiplies = false, .can_move = true}},
    {20, 32, "Ogre", 0x07020200, 0x00000000, 0x2036, 300, 42, 1, 'o', "13d8", "1 1 2d8", 13, 0, {.multiplies = false, .can_move = true}},
    {12, 20, "Giant Red Speckled Frog", 0x00000312, 0x00000000, 0x00A2, 300, 32, 1, 'a', "6d8", "1 2 3d4", 13, 0, {.multiplies = false, .can_move = true}},
    {20, 10, "Magic User", 0x13020200, 0x00002413, 0x2030, 100, 35, 1, 'p', "7d8", "1 1 2d2", 13, 0, {.multiplies = false, .can_move = true}},
    {20, 10, "Ranger", 0x13020200, 0x00000094, 0x2030, 80, 40, 1, 'p', "11d8", "1 1 3d3", 13, 0, {.multiplies = false, .can_move = true}},
    {20, 36, "Black Orc", 0x0B020200, 0x00000000, 0x2036, 200, 40, 1, 'o', "12d8", "1 1 3d4", 13, 0, {.multiplies = false, .can_move = true}},
    {12, 20, "Giant Long-Eared Bat", 0x00800306, 0x00000000, 0x2160, 200, 20, 3, 'b', "5d8", "1 2 1d4|1 3 1d2|1 3 1d2", 13, 0, {.multiplies = false, .can_move = true}},
    {8, 4, "Giant Gnat", 0x00800308, 0x00000000, 0x0062, 100, 1, 3, 'F', "1d2", "1 2 1d1", 13, 0, {.multiplies = true, .can_move = true}},
  // Level 14
    {12, 45, "Killer Green Beetle", 0x00000244, 0x00000000, 0x0002, 300, 46, 1, 'K', "16d8", "1 2 4d4", 14, 0, {.multiplies = false, .can_move = true}},
    {8, 25, "Giant Flea", 0x00000248, 0x00000000, 0x0062, 100, 1, 2, 'F', "2d2", "1 2 1d2", 14, 0, {.multiplies = true, .can_move = true}},
    {20, 20, "Giant White Dragon Fly", 0x00800304, 0x0040000A, 0x0060, 500, 54, 1, 'F', "5d8", "7 2 1d6", 14, 0, {.multiplies = false, .can_move = true}},
    {20, 36, "Hill Giant", 0x07020200, 0x00000000, 0x2034, 500, 52, 1, 'P', "16d8", "1 1 3d6", 14, 0, {.multiplies = false, .can_move = true}},
    {20, 34, "Skeleton Hobgoblin", 0x00020300, 0x00000000, 0x100C, 300, 46, 1, 's', "13d8", "1 1 2d5", 14, 0, {.multiplies = false, .can_move = true}},
    {12, 10, "Flesh Golem", 0x00020200, 0x00000000, 0x11B0, 100, 48, 1, 'g', "12d8", "1 1 1d6|1 1 1d6", 14, 0, {.multiplies = false, .can_move = true}},
    {12, 20, "White Dragon Bat", 0x00800304, 0x00400004, 0x2150, 500, 40, 3, 'b', "2d6", "7 2 1d3", 14, 0, {.multiplies = false, .can_move = true}},
  // Level 15
    {12, 75, "Anaconda", 0x00000300, 0x00000000, 0x50F0, 50, 275, 1, 'R', "18d8", "1 16 2-3d8", 15, 0, {.multiplies = false, .can_move = true}},
    {20, 50, "Guardian Naga", 0x17100302, 0x00000000, 0x20E6, 1200, 60, 1, 'n', "24d8", "1 16 2d8|1 2 1d8", 15, 0, {.multiplies = false, .can_move = true}},
    {12, 22, "Giant Grey Bat", 0x00800304, 0x00000000, 0x2160, 150, 22, 3, 'b', "4d8", "1 2 1d6|1 3 1d2|1 3 1d2", 15, 0, {.multiplies = false, .can_move = true}},
    {10, 30, "Giant Clear Centipede", 0x00010240, 0x00000000, 0x0002, 300, 30, 1, 'c', "5d8", "1 2 2d4|1 4 2d4", 15, 0, {.multiplies = false, .can_move = true}},
    {12, 48, "Giant Yellow Tick", 0x00000242, 0x00000000, 0x0022, 200, 48, 0, 't', "20d8", "14 2 3d9", 15, 0, {.multiplies = false, .can_move = true}},
    {12, 24, "Giant Ebony Ant", 0x00000240, 0x00000000, 0x0002, 600, 3, 1, 'c', "3d4", "1 2 2d3", 15, 0, {.multiplies = true, .can_move = true}},
    {20, 38, "Frost Giant", 0x07020200, 0x00000000, 0x0024, 500, 54, 1, 'P', "17d8", "7 1 3d6", 15, 0, {.multiplies = false, .can_move = true}},
    {20, 36, "Ettin", 0x07020200, 0x00000000, 0x2034, 500, 65, 1, 'P', "15d8", "1 1 3d4|1 1 3d4", 15, 0, {.multiplies = false, .can_move = true}},
    {4, 20, "Statue", 0x00000040, 0x00000000, 0x0200, 200, 45, 1, '~', "17d8", "1 1 2d8", 15, 0, {.multiplies = false, .can_move = false}},
    {40, 95, "Swirling White Vapor", 0x00800320, 0x8FFE0001, 0x5000, 1, 50, 2, 'v', "8d8", "11 5 1d6| 23 5 1d6", 15, 0, {.multiplies = false, .can_move = true}},
    {12, 20, "Clay Golem", 0x00020140, 0x00000000, 0x1200, 100, 50, 1, 'g', "14d8", "1 1 1d8|1 1 1d8", 15, 0, {.multiplies = false, .can_move = true}},
    {7, 12, "Huge White Bat", 0x00800304, 0x00000000, 0x2160, 400, 3, 2, 'b', "3d8", "1 2 1d6", 15, 0, {.multiplies = true, .can_move = true}},
    {12, 18, "Giant Tan Bat", 0x00800304, 0x00000000, 0x2160, 400, 18, 2, 'b', "3d8", "4 2 1d2|1 3 1d1|1 3 1d1", 15, 0, {.multiplies = false, .can_move = true}},
    {2, 15, "Violet Mold", 0x0B000040, 0x00010009, 0x10A0, 700, 50, 1, 'm', "17d8", "11 1 1d2", 15, 0, {.multiplies = false, .can_move = false}},
    {5, 20, "Giant Tarantula", 0x00000100, 0x00000000, 0x0032, 200, 110, 1, 'x', "20d8", "11 2 1d2|1 2 5d6", 15, 0, {.multiplies = false, .can_move = true}},
    {15, 45, "Quickling", 0x13000200, 0x00009006, 0x3044, 15, 150, 4, 'q', "15d5", "14 1 2-1d3", 15, 0, {.multiplies = false, .can_move = true}},
  // Level 16
    {10, 30, "Red-Orange Faerie Dragon", 0x19820302, 0x00008C96, 0x4001, 40, 50, 2, 'F', "5d8", "3 3 2-1d4", 16, 0, {.multiplies = false, .can_move = true}},
    {2, 3, "Stone Fish", 0x00410050, 0x00000000, 0x0092, 500, 35, 0, 'f', "4d8", "26 4 1d2|14 4 5d8", 16, 0, {.multiplies = false, .can_move = false}},
    {20, 20, "Umber Hulk", 0x00020300, 0x00000000, 0x2126, 100, 75, 1, 'U', "20d8", "3 7 0d0|1 1 2-1d6|1 2 2d6", 16, 0, {.multiplies = false, .can_move = true}},
    {12, 18, "Gelatinous Cube", 0x2F180142, 0x00000000, 0x00A2, 1, 36, 0, 'C', "45d8", "6 5 1d10", 16, 0, {.multiplies = false, .can_move = true}},
    {8, 16, "Giant Black Rat", 0x00000306, 0x00000000, 0x2070, 200, 3, 1, 'r', "3d4", "14 2 1d5", 16, 0, {. multiplies = true, .can_move = true}},
    {8, 50, "Giant Jelly Fish", 0x00000050, 0x00000000, 0x00F0, 0, 300, 2, 'f', "13d8", "4 11 2-2d6|4 27 1d20", 16, 0, {.multiplies = false, .can_move = true}},
    {20, 20, "Giant Green Dragon Fly", 0x00800304, 0x0010000A, 0x0070, 500, 58, 1, 'F', "5d8", "14 2 1d6", 16, 0, {.multiplies = false, .can_move = true}},
    {20, 40, "Fire Giant", 0x07020200, 0x00000000, 0x2014, 500, 62, 1, 'P', "20d8", "5 1 3d7", 16, 0, {.multiplies = false, .can_move = true}},
    {12, 22, "Green Dragon Bat", 0x00800304, 0x00100004, 0x2150, 500, 44, 3, 'b', "2d7", "14 2 1d3", 16, 0, {.multiplies = false, .can_move = true}},
    {20, 30, "Quasit", 0x11030202, 0x000010FA, 0x0004, 200, 48, 1, 'q', "5d8", "15 2 1d6|1 3 1d3|1 3 1d3", 16, 0, {.multiplies = false, .can_move = true}},
  // Level 17
    {20, 40, "Troll", 0x07020300, 0x00000000, 0xA026, 400, 74, 1, 'T', "17d8", "1 1 2-1d4|1 2 1d6", 17, 0, {.multiplies = false, .can_move = true}},
    {6, 30, "Giant Water Imp", 0x00400210, 0x00000000, 0x0002, 8, 100, 2, 'I', "12d8", "1 30 5d4|11 2 1d3", 17, 0, {.multiplies = false, .can_move = true}},
    {12, 28, "Water Spirit", 0x00000152, 0x00000000, 0x0020, 400, 58, 0, 'E', "8d8", "1 1 2d4", 17, 0, {.multiplies = false, .can_move = true}},
    {14, 44, "Giant Brown Scorpion", 0x00000242, 0x00000000, 0x0002, 200, 62, 1, 'S', "11d8", "1 2 2d4|2 4 1d7", 17, 0, {.multiplies = false, .can_move = true}},
    {10, 40, "Earth Spirit", 0x00000142, 0x00000000, 0x0200, 500, 64, 1, 'E', "13d8", "1 1 1d8|1 1 1d8", 17, 0, {.multiplies = false, .can_move = true}},
  // Level 18
    {6, 25, "Giant Black Widow", 0x00000200, 0x00000000, 0x0032, 110, 150, 1, 'x', "12d8", "1 2 1d6|27 2 5d8", 18, 0, {.multiplies = false, .can_move = true}},
    {20, 30, "Fire Spirit", 0x00000140, 0x00000000, 0x2010, 200, 66, 2, 'E', "10d8", "5 1 2d6", 18, 0, {.multiplies = false, .can_move = true}},
    {20, 42, "Urik-Hai Orc", 0x0B020200, 0x00000000, 0x2036, 200, 68, 1, 'o', "14d8", "1 1 3d5", 18, 0, {.multiplies = false, .can_move = true}},
    {20, 24, "Devilkin", 0x11030202, 0x010010FA, 0x0404, 150, 75, 1, 'q', "7d8", "15 2 1d6|2 3 2-1d6|16 4 1d3", 18, 0, {.multiplies = false, .can_move = true}},
    {20, 40, "Stone Giant", 0x07020300, 0x00000000, 0x2204, 500, 80, 1, 'P', "22d8", "1 1 3d8", 18, 0, {.multiplies = false, .can_move = true}},
    {15, 30, "Black Hag", 0x09100302, 0x00008086, 0x2066, 300, 150, 1, 'p', "20d4", "1 6 2d6|10 9 2d6|25 19 1d1", 18, 0, {.multiplies = false, .can_move = true}},
    {10, 20, "Phase Imp", 0x00060300, 0x00000000, 0x8000, 110, 175, 1, 'i', "20d8", "1 3 6d6", 18, 0, {.multiplies = false, .can_move = true}},
  // Level 19
    {12, 75, "Stone Golem", 0x00020300, 0x00000000, 0x1200, 100, 100, 0, 'g', "28d8", "1 1 2-1d10", 19, 0, {.multiplies = false, .can_move = true}},
    {15, 10, "Grey Ooze", 0x071A0248, 0x00000000, 0x00A2, 1, 40, 1, 'O', "6d8", "7 5 2d6", 19, 0, {.multiplies = false, .can_move = true}},
    {15, 15, "Disenchanter Ooze", 0x071A0248, 0x00000000, 0x00A2, 1, 50, 1, 'O', "6d8", "21 5 0d0", 19, 15, {.multiplies = false, .can_move = true}},
    {8, 20, "Giant Spotted Rat", 0x00000302, 0x00000000, 0x2070, 200, 3, 1, 'r', "4d3", "14 2 1d5", 19, 0, {.multiplies = true, .can_move = true}},
    {20, 24, "Mummified Kobold", 0x0B020300, 0x00000000, 0x102C, 750, 46, 1, 'M', "13d8", "1 1 1d6|1 1 1d6", 19, 0, {.multiplies = false, .can_move = true}},
    {14, 46, "Killer Black Beetle", 0x00000242, 0x00000000, 0x0002, 300, 75, 1, 'K', "18d8", "1 2 4d5", 19, 0, {.multiplies = false, .can_move = true}},
    {2, 16, "Red Mold", 0x0B000040, 0x00000000, 0x30A0, 700, 64, 1, 'm', "17d8", "5 5 4d4", 19, 0, {.multiplies = false, .can_move = false}},
  // Level 20
    {10, 1, "Quythulg", 0x00010300, 0x00002010, 0x5000, 0, 200, 1, 'Q', "4d8", "99 0 0d0", 20, 0, {.multiplies = false, .can_move = true}},
    {15, 40, "Gargoyle", 0x17800000, 0x00008088, 0x9606, 200, 200, 1, 'g', "25d8", "4 1 1d4|1 3 2-4d4", 20, 0, {.multiplies = false, .can_move = true}},
    {30, 65, "<gp>'s Ghost", 0x14870300, 0x00000000, 0x500C, 50, 210, 2, 'G', "15d8", "1 1 2d8|4 5 2d6", 20, 0, {.multiplies = false, .can_move = true}},
    {20, 30, "Winter Wolf", 0x00000304, 0x00000000, 0x1020, 10, 200, 2, 'j', "10d8", "1 1 2d6| 7 8 3d6", 20, 0, {.multiplies = false, .can_move = true}},
    {2, 50, "Trapper", 0x00010340, 0x00000000, 0x1000, 500, 125, 1, '.', "20d8", "11 16 4d4", 20, 0, {.multiplies = false, .can_move = false}},
    {12, 24, "Giant Red Bat", 0x00800302, 0x00000000, 0x2060, 200, 40, 2, 'b', "5d8", "1 2 1d7|1 3 2-1d3", 20, 0, {.multiplies = false, .can_move = true}},
    {40, 95, "Swirling Clear Vapor", 0x00800308, 0x8FFE0001, 0x5000, 1, 90, 2, 'v', "8d8", "21 5 1d6", 20, 0, {.multiplies = false, .can_move = true}},
    {20, 50, "Kobold Chieftain", 0x13120300, 0x0000B145, 0x3034, 300, 150, 1, 'k', "18d8", "1 1 4d4|1 1 4d4", 20, 0, {.multiplies = false, .can_move = true}},
    {20, 50, "King Yeek", 0x13120300, 0x00000324, 0x3000, 300, 150, 1, 'y', "22d8", "1 1 4d3|1 1 4d3", 20, 0, {.multiplies = false, .can_move = true}},
    {10, 45, "Zombie Dragon", 0x00020300, 0x00009544, 0x100C, 700, 120, 1, 'z', "20d8", "1 3 2d3|1 3 2d3|1 2 3d4", 20, 15, {.multiplies = false, .can_move = true}},
    {20, 50, "Hydra", 0x22020302, 0x00001905, 0x3005, 1, 150, 1, 'd', "20d8", "1 3 2d3|1 3 2d3|1 2 2-3d3", 20, 0, {.multiplies = false, .can_move = true}},
    {20, 22, "Giant Black Dragon Fly", 0x00800304, 0x00200009, 0x0072, 500, 58, 1, 'F', "4d8", "10 2 1d6", 20, 0, {.multiplies = false, .can_move = true}},
    {20, 44, "Cloud Giant", 0x07020200, 0x00000000, 0x2134, 500, 125, 1, 'P', "24d8", "8 1 3d8", 20, 0, {.multiplies = false, .can_move = true}},
  // Level 21
    {12, 24, "Black Dragon Bat", 0x00800304, 0x00200004, 0x2150, 500, 50, 3, 'b', "2d8", "6 2 1d3", 21, 0, {.multiplies = false, .can_move = true}},
    {30, 60, "Immature Dragon", 0x12820302, 0x00000000, 0x4001, 500, 180, 1, 'd', "12d8", "1 6 2-1d8|1 2 3d4", 21, 70, {.multiplies = false, .can_move = true}},
    {12, 26, "Blue Dragon Bat", 0x00800304, 0x00080004, 0x2150, 500, 54, 3, 'b', "3d6", "8 2 1d3", 21, 0, {.multiplies = false, .can_move = true}},
    {20, 28, "Mummified Orc", 0x0B020300, 0x00000000, 0x102C, 750, 56, 1, 'M', "14d8", "1 1 2d4|1 1 2d4", 21, 0, {.multiplies = false, .can_move = true}},
    {12, 48, "Killer Boring Beetle", 0x00000240, 0x00000000, 0x0002, 300, 70, 1, 'K', "18d8", "1 2 4d5", 21, 0, {.multiplies = false, .can_move = true}},
  // Level 22
    {12, 50, "Killer Stag Beetle", 0x00000242, 0x00000000, 0x0002, 300, 80, 1, 'K', "20d8", "1 2 3d4|1 1 1d12", 22, 0, {.multiplies = false, .can_move = true}},
    {2, 18, "Black Mold", 0x0B000040, 0x00000000, 0x10A0, 500, 68, 1, 'm', "15d8", "1 1 4d3", 22, 0, {.multiplies = false, .can_move = false}},
    {12, 99, "Iron Golem", 0x00020300, 0x00000000, 0x1080, 100, 160, -1, 'g', "80d8", "1 1 1d12|1 1 1d12", 22, 0, {.multiplies = false, .can_move = true}},
    {12, 38, "Giant Yellow Scorpion", 0x00000242, 0x00000000, 0x0002, 200, 60, 1, 'S', "12d8", "1 2 1d8|14 4 2d5", 22, 0, {.multiplies = false, .can_move = true}},
    {15, 5, "Green Ooze", 0x00200040, 0x00000000, 0x00B2, 1, 6, 0, 'O', "4d8", "6 5 2d3", 22, 0, {.multiplies = false, .can_move = false}},
  // Level 23
    {10, 6, "Green Slime", 0x003A0044, 0x0001000B, 0x0192, 1, 7, -1, 'O', "8d8", "9 5 2d8", 23, 0, {.multiplies = false, .can_move = true}},
    {10, 6, "Black Ooze", 0x003A0044, 0x0001000B, 0x0192, 1, 7, -1, 'O', "6d8", "9 5 2d6", 23, 0, {.multiplies = false, .can_move = true}},
    {20, 40, "Warrior", 0x13120200, 0x00000000, 0x2030, 400, 60, 1, 'p', "15d8", "1 1 3d5", 23, 0, {.multiplies = false, .can_move = true}},
    {12, 28, "Red Dragon Bat", 0x00800004, 0x00800004, 0x2150, 500, 60, 3, 'b', "3d8", "5 2 1d3", 23, 0, {.multiplies = false, .can_move = true}},
    {15, 50, "Killer Blue Beetle", 0x00000242, 0x00000000, 0x0002, 300, 85, 1, 'K', "20d8", "1 2 4d5", 23, 0, {.multiplies = false, .can_move = true}},
    {15, 38, "Giant Silver Ant", 0x00000242, 0x00000000, 0x0002, 600, 45, 1, 'c', "6d8", "6 2 4d4", 23, 0, {.multiplies = false, .can_move = true}},
    {2, 18, "Crimson Mold", 0x0B000040, 0x00000000, 0x10A0, 500, 65, 1, 'm', "16d8", "1 1 1d3|4 5 0d0", 23, 0, {.multiplies = false, .can_move = false}},
    {20, 35, "Red Hag", 0x09100202, 0x00000295, 0x2066, 250, 250, 1, 'p', "20d6", "1 6 3d4|10 9 3d4|25 19 1d1", 23, 0, {.multiplies = false, .can_move = true}},
  // Level 24
    {10, 35, "Orange Faerie Dragon", 0x21820302, 0x00000E96, 0x4001, 30, 100, 2, 'F', "7d8", "3 3 2-1d6", 24, 0, {.multiplies = false, .can_move = true}},
    {20, 30, "Forest Wight", 0x0F000202, 0x0000100F, 0x112E, 300, 140, 1, 'W', "12d8", "1 1 2-1d6|19 5 12d8", 24, 0, {.multiplies = false, .can_move = true}},
    {20, 20, "Berzerker", 0x07020200, 0x00000000, 0x2030, 100, 65, 1, 'p', "15d8", "1 1 1d8|1 1 1d8", 24, 0, {.multiplies = false, .can_move = true}},
    {20, 34, "Mummified Human", 0x0B020200, 0x00000000, 0x102C, 600, 70, 1, 'M', "17d8", "1 1 2d4|1 1 2d4", 24, 0, {.multiplies = false, .can_move = true}},
    {20, 24, "Banshee", 0x0F950306, 0x0001002F, 0x100C, 100, 60, 2, 'G', "6d8", "4 10 0d0|19 5 14d8", 24, 0, {.multiplies = false, .can_move = true}},
  // Level 25
    {20, 40, "Giant Troll", 0x0F020200, 0x00000000, 0xA026, 500, 105, 1, 'T', "19d8", "1 1 2-1d6|1 2 3d4", 25, 0, {.multiplies = false, .can_move = true}},
    {12, 50, "Giant Brown Tick", 0x00000144, 0x00000000, 0x0022, 200, 70, 0, 't', "18d8", "14 2 1d10|10 4 1d1", 25, 0, {.multiplies = false, .can_move = true}},
    {20, 80, "Hobgoblin Chief", 0x12020300, 0x00000888, 0x3004, 300, 120, 3, 'H', "20d8", "1 1 4d4", 25, 0, {.multiplies = false, .can_move = true}},
    {20, 10, "Nymph Princess", 0x15130210, 0x000084F4, 0x3000, 1000, 100, 3, 'N', "20d8", "13 5 0d0|12 5 0d0", 25, 20, {.multiplies = false, .can_move = true}},
    {8, 12, "Mimic", 0x06000040, 0x00000000, 0x3020, 500, 100, 3, '&', "10d10", "1 16 3d3|1 1 3-2d4", 25, 0, {.multiplies = false, .can_move = false}},
    {15, 50, "Killer Red Beetle", 0x00000240, 0x00000000, 0x0002, 300, 85, 1, 'K', "20d8", "2 2 4d4", 25, 0, {.multiplies = false, .can_move = true}},
    {2, 50, "Wooden Mold", 0x00000040, 0x00000000, 0x10A0, 999, 100, 1, 'm', "25d8", "14 8 2d6", 25, 0, {.multiplies = false, .can_move = false}},
    {20, 24, "Giant Blue Dragon Fly", 0x00800310, 0x00080009, 0x0030, 500, 75, 1, 'F', "6d8", "1 2 1d6", 25, 0, {.multiplies = false, .can_move = true}},
    {10, 15, "Hippo", 0x00000210, 0x00000000, 0x20F2, 500, 250, 1, 'H', "50d8", "1 17 2-2d10|1 2 2d8", 25, 0, {.multiplies = false, .can_move = true}},
    {15, 45, "Orc Lord", 0x13000300, 0x00000000, 0x4000, 40, 400, 2, 'o', "15d8", "1 1 5d4", 25, 0, {.multiplies = false, .can_move = true}},
  // Level 26
    {10, 40, "Giant Grey Ant Lion", 0x00080242, 0x00000000, 0x0032, 400, 90, 1, 'A', "19d8", "1 2 2d12", 26, 0, {.multiplies = false, .can_move = true}},
    {14, 24, "Disenchanter Bat", 0x00800304, 0x00000000, 0x2060, 1, 75, 3, 'b', "4d8", "21 1 0d0", 26, 20, {.multiplies = false, .can_move = true}},
    {14, 54, "Giant Fire Tick", 0x00000142, 0x00000000, 0x2012, 200, 90, 1, 't', "16d8", "5 9 3d7", 26, 0, {.multiplies = false, .can_move = true}},
    {14, 40, "Hell Hound", 0x00000310, 0x0080000F, 0x3414, 200, 120, 1, 'j', "14d8", "5 8 3d5|1 2 2d5", 26, 0, {.multiplies = false, .can_move = true}},
    {20, 40, "White Wraith", 0x0F020302, 0x00000000, 0x112E, 100, 165, 1, 'W', "15d8", "1 1 2-1d6|19 5 15d8", 26, 0, {.multiplies = false, .can_move = true}},
    {12, 50, "Giant Black Scorpion", 0x00000240, 0x00000000, 0x0002, 200, 85, 1, 'S', "13d8", "1 2 1d10|14 4 2d5", 26, 0, {.multiplies = false, .can_move = true}},
    {10, 14, "Clear Ooze", 0x000B0148, 0x00000000, 0x00B2, 1, 12, 1, 'O', "4d8", "3 5 1d8", 26, 0, {.multiplies = false, .can_move = true}},
  // Level 27
    {14, 45, "Killer Fire Beetle", 0x00000144, 0x00000000, 0x2012, 300, 95, 1, 'K', "13d8", "1 2 3d4|5 9 4d5", 27, 0, {.multiplies = false, .can_move = true}},
    {20, 45, "Gluhtyuq", 0x17010300, 0x04000001, 0x0000, 100, 175, 1, 'Q', "2d6", "99 0 0d0", 27, 0, {.multiplies = false, .can_move = true}},
    {20, 45, "Vampire", 0x17000300, 0x00001209, 0x112E, 100, 175, 1, 'V', "20d8", "1 1 2-1d6|19 30 18d8", 27, 0, {.multiplies = false, .can_move = true}},
    {20, 24, "Giant Red Dragon Fly", 0x00800304, 0x00800008, 0x2050, 500, 75, 1, 'F', "7d8", "4 2 1d6", 27, 0, {.multiplies = false, .can_move = true}},
    {2, 24, "Shimmering Mold", 0x0B000040, 0x00000000, 0x10A0, 100, 180, 1, 'm', "32d8", "8 5 5d4", 27, 0, {.multiplies = false, .can_move = false}},
  // Level 28
    {20, 60, "Black Knight", 0x17020200, 0x0000010F, 0x2030, 100, 140, 1, 'p', "25d8", "1 1 5d5", 28, 0, {.multiplies = false, .can_move = true}},
    {20, 30, "Mage", 0x13020200, 0x00002C73, 0x2030, 100, 150, 1, 'p', "10d8", "1 1 2d5", 28, 0, {.multiplies = false, .can_move = true}},
    {20, 50, "Master Bard", 0x13020200, 0x13001A93, 0x2030, 85, 160, 1, 'p', "20d8", "1 1 4d4", 28, 0, {.multiplies = false, .can_move = true}},
    {20, 30, "Druid", 0x13020200, 0x13080503, 0x2030, 100, 160, 1, 'p', "16d8", "1 1 2d6", 28, 0, {.multiplies = false, .can_move = true}},
    {20, 50, "Master Monk", 0x13020200, 0x00000000, 0x2030, 85, 160, 2, 'p', "20d8", "1 1 2-3d3", 28, 0, {.multiplies = false, .can_move = true}},
    {20, 46, "Ice Troll", 0x0F020200, 0x00000000, 0x8026, 500, 190, 1, 'T', "22d8", "1 1 2-1d5|7 2 3d6", 28, 0, {.multiplies = false, .can_move = true}},
    {20, 35, "Night Hag", 0x17100302, 0x00000394, 0x2066, 250, 250, 1, 'p', "30d8", "1 6 3d6|10 9 4d6|25 19 1d2", 28, 20, {.multiplies = false, .can_move = true}},
    {8, 50, "Demon Sword", 0x00880310, 0x80FA0001, 0x5404, 700, 220, 2, '|', "10d9", "1 1 3d6|19 10 10d8", 28, 0, {.multiplies = false, .can_move = true}},
  // Level 29
    {14, 65, "Giant Purple Worm", 0x00000242, 0x00000000, 0x2030, 300, 400, 1, 'w', "65d8", "1 1 1d8|6 2 2d8|14 4 1d8", 29, 0, {.multiplies = false, .can_move = true}},
    {20, 50, "Young Blue Dragon", 0x1F000202, 0x0008100B, 0x2005, 700, 300, 1, 'd', "33d8", "1 3 2-1d4|1 2 1d6", 29, 25, {.multiplies = false, .can_move = true}},
    {20, 50, "Young White Dragon", 0x1F000202, 0x0040100B, 0x2005, 700, 275, 1, 'd', "32d8", "1 3 2-1d4|1 2 1d6", 29, 25, {.multiplies = false, .can_move = true}},
    {20, 50, "Young Green Dragon", 0x1F000202, 0x0010100B, 0x2005, 700, 290, 1, 'd', "32d8", "1 3 2-1d4|1 2 1d6", 29, 25, {.multiplies = false, .can_move = true}},
    {14, 30, "Giant Fire Bat", 0x00800310, 0x00000000, 0x2050, 100, 85, 2, 'b', "5d8", "5 2 3d6|1 3 2-1d4", 29, 0, {.multiplies = false, .can_move = true}},
    {8, 24, "Giant Glowing Rat", 0x00000302, 0x00000000, 0x2070, 200, 4, 1, 'r', "3d3", "8 2 2d6", 29, 0, {.multiplies = true, .can_move = true}},
  // Level 30
    {8, 15, "Hammerhead Shark", 0x00000050, 0x00000000, 0x6000, 100, 600, 2, 'S', "75d6", "1 2 3-4d4|1 25 10d4;", 30, 0, {.multiplies = false, .can_move = true}},
    {20, 55, "Skeleton Troll", 0x00020300, 0x00000000, 0x500C, 400, 225, 1, 's', "14d8", "1 1 2-1d6|1 2 3d4", 30, 0, {.multiplies = false, .can_move = true}},
    {15, 34, "Giant Lightning Bat", 0x00800302, 0x00000000, 0x2042, 100, 80, 2, 'b', "8d8", "8 2 3d8|1 3 2-1d5", 30, 0, {.multiplies = false, .can_move = true}},
    {10, 40, "Giant Static Ant", 0x00000242, 0x00000000, 0x0002, 600, 80, 1, 'c', "8d8", "8 2 5d5", 30, 0, {.multiplies = false, .can_move = true}},
    {30, 80, "Swirling Black Vapor", 0x00800308, 0x8FFE0001, 0xD100, 1, 350, 3, 'v', "11d8", "19 5 10d8", 30, 0, {.multiplies = false, .can_move = true}},
    {20, 40, "Lesser Demon", 0x12120300, 0x0280509A, 0xE414, 1000, 325, 1, 'B', "25d8", "5 1 3d3", 30, 25, {.multiplies = false, .can_move = true}},
    {20, 35, "Grave Wight", 0x0F020308, 0x0000190A, 0x512E, 300, 325, 1, 'W', "12d8", "1 1 2-1d7|19 5 20d8", 30, 0, {.multiplies = false, .can_move = true}},
    {14, 55, "Killer Slicer Beetle", 0x00000242, 0x00000000, 0x0012, 300, 200, 1, 'K', "22d8", "1 2 5d8", 30, 0, {.multiplies = false, .can_move = true}},
    {12, 45, "Giant White Ant Lion", 0x00080042, 0x00000000, 0x0022, 400, 175, 1, 'A', "20d8", "7 2 3d10", 30, 0, {.multiplies = false, .can_move = true}},
    {15, 4, "Tunneling Worm Mass", 0x00060344, 0x00000000, 0x21B2, 30, 7, 0, 'w', "4d8", "9 12 1d3", 30, 0, {.multiplies = true, .can_move = true}},
  // Level 31
    {20, 30, "Ghost", 0x17950308, 0x0001002F, 0x500C, 100, 350, 2, 'G', "13d8", "4 7 0d0|19 5 22d8|17 3 1d10", 31, 0, {.multiplies = false, .can_move = true}},
    {14, 45, "Giant Black Ant Lion", 0x00080042, 0x00000000, 0x0032, 400, 170, 1, 'A', "23d8", "1 2 2d12|6 9 3d6", 31, 0, {.multiplies = false, .can_move = true}},
    {16, 60, "Death Watch Beetle", 0x00000042, 0x00000000, 0x0012, 300, 190, 1, 'K', "25d8", "1 2 5d4|1 10 5d6", 31, 0, {.multiplies = false, .can_move = true}},
    {20, 55, "Horned Demon", 0x0F020300, 0x00000000, 0x7504, 300, 350, 2, 'B', "12d8", "1 25 2-1d8|21 8 1d6 31", 31, 30, {.multiplies = false, .can_move = true}},
    {20, 42, "Ogre Magi", 0x07020300, 0x0000A356, 0x403C, 300, 250, 1, 'o', "14d8", "1 1 3d6", 31, 30, {.multiplies = false, .can_move = true}},
    {5, 30, "Crystal Ooze", 0x07390243, 0x00000000, 0x0092, 600, 8, -1, 'O', "12d8", "7 5 4d4", 31, 0, {.multiplies = false, .can_move = true}},
  // Level 32
    {10, 40, "Yellow Faerie Dragon", 0x31820302, 0x01000E93, 0x4001, 40, 250, 3, 'F', "9d8", "3 3 2-1d8", 32, 0, {.multiplies = false, .can_move = true}},
    {20, 48, "Two-Headed Troll", 0x0F020300, 0x00000000, 0xE026, 500, 315, 1, 'T', "14d8", "1 1 2-1d8|1 2 2-1d6", 32, 0, {.multiplies = false, .can_move = true}},
    {20, 40, "Ice Demon", 0x0F020300, 0x00010106, 0x7444, 450, 350, 1, 'B', "20d8", "7 1 4d6", 32, 0, {.multiplies = false, .can_move = true}},
    {20, 46, "Invisible Stalker", 0x00830308, 0x00000000, 0x0000, 200, 200, 3, 'E', "19d8", "1 1 1d6", 32, 0, {.multiplies = false, .can_move = true}},
    {16, 40, "Giant Hunter Ant", 0x00000240, 0x00000000, 0x0002, 10, 150, 1, 'c', "12d8", "1 2 4d8", 32, 0, {.multiplies = false, .can_move = true}},
    {20, 65, "Ninja", 0x0F020200, 0x00000000, 0x6030, 100, 300, 1, 'p', "15d8", "14 1 3d4|2 1 3d4", 32, 0, {.multiplies = false, .can_move = true}},
    {5, 20, "Brontosaurus", 0x000A0312, 0x00000000, 0x6012, 250, 2000, 1, 'D', "500d8", "1 17 4-8d8", 32, 0, {.multiplies = false, .can_move = true}},
  // Level 33
    {20, 40, "Barrow Wight", 0x0F020302, 0x00001308, 0x512E, 100, 375, 1, 'W', "13d8", "1 1 2-1d8|19 5 26d8", 33, 0, {.multiplies = false, .can_move = true}},
    {20, 48, "Skeleton 2-Headed Troll", 0x00020300, 0x00000000, 0x500C, 200, 325, 1, 's', "20d8", "1 1 2-1d9|1 2 2-1d5", 33, 0, {.multiplies = false, .can_move = true}},
    {12, 36, "Water Elemental", 0x00000212, 0x00000000, 0x0020, 500, 325, 2, 'E', "25d8", "1 1 1d10|1 1 1d10", 33, 0, {.multiplies = false, .can_move = true}},
    {16, 40, "Fire Elemental", 0x000A0144, 0x00000000, 0x2010, 700, 350, 1, 'E', "25d8", "5 1 4d6", 33, 0, {.multiplies = false, .can_move = true}},
  // Level 34
    {20, 50, "Lich", 0x1F020300, 0x00019F75, 0x500C, 600, 750, 1, 'L', "25d8", "15 5 2d8|19 5 30d8|24 5 0d0", 34, 40, {.multiplies = false, .can_move = true}},
    {20, 55, "Master Vampire", 0x17000300, 0x00001307, 0x512E, 100, 700, 1, 'V', "23d8", "1 1 2-1d6|19 30 32d8", 34, 40, {.multiplies = false, .can_move = true}},
    {20, 56, "Spirit Troll", 0x00040300, 0x00000000, 0x402E, 100, 425, 1, 'T', "15d8", "1 3 2-1d5|1 2 1d6", 34, 0, {.multiplies = false, .can_move = true}},
    {12, 50, "Giant Red Scorpion", 0x00000242, 0x00000000, 0x0002, 400, 275, 2, 'S', "18d8", "1 2 1d6|14 4 1d4", 34, 0, {.multiplies = false, .can_move = true}},
    {10, 60, "Earth Elemental", 0x001A0140, 0x00000000, 0x0200, 900, 375, 1, 'E', "30d8", "1 1 4d6|1 1 4d6", 34, 0, {.multiplies = false, .can_move = true}},
  // Level 35
    {4, 65, "Giant Remora", 0x00000050, 0x00000C47, 0xE002, 50, 1500, 2, 'R', "250d4", "2 30 2-4d5|15 30 2-3d6", 35, 0, {.multiplies = false, .can_move = true}},
    {20, 55, "Young Black Dragon", 0x1F000202, 0x0020100B, 0x6005, 500, 600, 1, 'd', "32d8", "1 3 2-1d5|1 2 1d6", 35, 40, {.multiplies = false, .can_move = true}},
    {20, 60, "Young Red Dragon", 0x1F000102, 0x0080100A, 0x6015, 500, 650, 1, 'd', "36d8", "1 3 2-1d8|1 2 2d8", 35, 40, {.multiplies = false, .can_move = true}},
    {20, 40, "Necromancer", 0x13020200, 0x00005762, 0x6030, 100, 600, 1, 'p', "17d8", "1 1 2d6", 35, 50, {.multiplies = false, .can_move = true}},
    {20, 50, "High Priest", 0x13020200, 0x02024322, 0x7004, 100, 600, 1, 'p', "25d12", "1 1 2d6", 35, 0, {.multiplies = false, .can_move = true}},
    {20, 65, "Troll King", 0x17020300, 0x00003365, 0xF004, 200, 650, 2, 'T', "30d8", "1 3 3d3|1 3 3d3|1 2 5d5", 35, 0, {.multiplies = false, .can_move = true}},
    {20, 50, "Heirophant Druid", 0x13020200, 0x00880522, 0x7000, 100, 1000, 2, 'p', "50d8", "1 1 2d8|11 23 2d6", 35, 0, {.multiplies = false, .can_move = true}},
    {20, 50, "Titan", 0x13020200, 0x23083763, 0x7004, 100, 550, 1, 'P', "40d8", "1 1 3d12|8 1 3d8", 35, 0, {.multiplies = false, .can_move = true}},
    {20, 50, "Succubus", 0x13020200, 0x21011225, 0x3404, 100, 700, 1, 'p', "28d9", "18 5 3d3|3 24 4d4|19 2 38d8", 35, 50, {.multiplies = false, .can_move = true}},
    {29, 38, "Mummified Troll", 0x0F020300, 0x00000000, 0x502C, 500, 400, 1, 'M', "18d8", "1 1 2d6|1 1 2d6", 35, 0, {.multiplies = false, .can_move = true}},
    {14, 48, "Giant Red Ant Lion", 0x00080142, 0x00000000, 0x0032, 400, 350, 2, 'A', "23d8", "5 2 3d12", 35, 0, {.multiplies = false, .can_move = true}},
    {20, 65, "Mature White Dragon", 0x2F000202, 0x0040100A, 0x4005, 700, 1000, 2, 'd', "48d8", "1 3 2-1d8|1 2 2d8", 35, 50, {.multiplies = false, .can_move = true}},
    {20, 60, "Nazgul", 0x12830000, 0x00020008, 0xD504, 1, 900, 2, 'Z', "38d8", "4 1 2d8|2 17 3d10", 35, 50, {.multiplies = false, .can_move = true}},
    {25, 60, "Cerebus", 0x00000304, 0x00001048, 0x7002, 200, 800, 1, 'j', "26d8", "1 2 3-3d6|7 8 4d4|5 8 4d4|4 8 4d4", 35, 0, {.multiplies = false, .can_move = true}},
    {9, 25, "Mako Shark", 0x00080050, 0x00000000, 0xC000, 10, 1000, 3, 'S', "100d5", "1 2 2-3d5", 35, 75, {.multiplies = false, .can_move = true}},
    {20, 80, "Shadow Dragon", 0x23050300, 0x00035702, 0x510D, 25, 1250, 2, 'd', "45d8", "19 3 2-1d9|4 2 2d10", 35, 60, {.multiplies = false, .can_move = true}},
  // Level 36
    {20, 80, "Xorn", 0x00160300, 0x00000000, 0x4200, 100, 650, 1, 'X', "20d8", "1 1 3-1d6", 36, 0, {.multiplies = false, .can_move = true}},
    {14, 50, "Giant Mottled Ant Lion", 0x00080242, 0x00000000, 0x0032, 400, 350, 2, 'A', "24d8 ", "1 2 2d10", 36, 0, {.multiplies = false, .can_move = true}},
    {20, 50, "Grey Wraith", 0x0F020302, 0x00011308, 0x512E, 100, 700, 1, 'W', "23d8", "1 1 2-1d10|19 5 34d8", 36, 0, {.multiplies = false, .can_move = true}},
    {20, 55, "Young Multi-Hued Dragon", 0x7F000202, 0x00F81005, 0x6005, 500, 1250, 1, 'd', "40d8", "1 3 2-1d9|1 2 2d10", 36, 60, {.multiplies = false, .can_move = true}},
    {20, 75, "Mature Blue Dragon", 0x2F000202, 0x00081009, 0x6005, 400, 1200, 2, 'd', "48d8", "1 3 1d8|1 3 1d8|1 2 2d10", 36, 60, {.multiplies = false, .can_move = true}},
    {20, 60, "Major Demon", 0x12130300, 0x02815578, 0x3414, 800, 700, 2, 'B', "50d8", "5 3 4d4|5 3 4d4|4 2 2d2", 36, 60, {.multiplies = false, .can_move = true}},
    {20, 70, "Mature Green Dragon", 0x1F000202, 0x0010100A, 0x6005, 700, 1100, 2, 'd', "48d8", "1 3 1d4|1 3 1d4|1 2 1d6", 36, 60, {.multiplies = false, .can_move = true}},
  // Level 37
    {20, 75, "Pyrohydra", 0x230A0200, 0x00801103, 0x5012, 50, 2000, 2, 'd', "100d8", "1 3 2-3d4|1 2 3-4d4", 37, 0, {.multiplies = false, .can_move = true}},
    {20, 65, "Vampiric Demon", 0x17000300, 0x02001307, 0xD40C, 85, 1400, 1, 'V', "40d8", "1 3 2d6| 1 3 2d6|19 2 38d8", 37, 65, {.multiplies = false, .can_move = true}},
    {20, 60, "Demon Warrior", 0x0F020300, 0x00000000, 0x7504, 100, 400, 1, 'B', "20d8", "1 1 3d5|1 1 3d5|1 2 2d5", 37, 65, {.multiplies = false, .can_move = true}},
    {16, 60, "Iridescent Beetle", 0x00000242, 0x00000000, 0x0012, 300, 850, 1, 'K', "32d8", "1 2 4d6|1 1 1d12|11 7 0d0", 37, 0, {.multiplies = false, .can_move = true}},
    {20, 65, "King Vampire", 0x17000300, 0x00001307, 0x512E, 100, 1000, 1, 'V', "38d8", "1 1 2-1d6|19 30 38d8", 37, 70, {.multiplies = false, .can_move = true}},
    {20, 65, "King Lich", 0x1F020300, 0x00019F73, 0x500C, 500, 1400, 1, 'L', "52d8", "15 5 2d10|19 5 36d8|24 5 0d0", 37, 70, {.multiplies = false, .can_move = true}},
    {20, 80, "Mature Red Dragon", 0x2F000302, 0x00801808, 0x6015, 300, 1400, 2, 'd', "60d8", "1 3 2-1d10|1 2 2d12", 37, 80, {.multiplies = false, .can_move = true}},
    {20, 55, "Mature Black Dragon", 0x2F000302, 0x00201009, 0x6005, 700, 1350, 2, 'd', "58d8", "1 3 2-1d8|1 2 2d10", 37, 80, {.multiplies = false, .can_move = true}},
  // Level 38
    {20, 65, "Mature Multi-Hued Dragon", 0x7F000302, 0x00F81A05, 0x6005, 500, 1650, 2, 'd', "80d8", "1 3 2-1d10|1 2 2d12", 38, 80, {.multiplies = false, .can_move = true}},
    {20, 80, "Ancient White Dragon", 0x4F000300, 0x00401A08, 0x4005, 800, 1500, 3, 'D', "88d8", "1 3 1d8|1 3 1d8|1 2 2d8", 38, 90, {.multiplies = false, .can_move = true}},
    {20, 40, "Emperor Wight", 0x1B020302, 0x00001306, 0x512E, 100, 1600, 2, 'W', "48d8", "1 1 2-1d12|19 5 42d8", 38, 90, {.multiplies = false, .can_move = true}},
    {20, 55, "Black Wraith", 0x1F020302, 0x00001307, 0x512E, 100, 1700, 1, 'W', "50d8", "1 1 2-1d12|19 5 44d8", 38, 100, {.multiplies = false, .can_move = true}},
    {20, 40, "Beholder", 0x00820302, 0x8101B7C6, 0x7106, 100, 4000, 4, 'e', "60d8", "21 7 2d4", 38, 100, {.multiplies = false, .can_move = true}},
    {40, 90, "Swirling Multi-Hued Vapor", 0x00800304, 0x80FFE001, 0xD000, 1, 1000, 3, 'v', "20d8", "11 5 2d6|21 5 2d6|3 5 2d6|24 5 1d6", 38, 0, {.multiplies = false, .can_move = true}},
    {30, 40, "Shadow Hag", 0x1B110302, 0x00010394, 0x4046, 200, 1650, 2, 'p', "40d6", "1 6 3d6|10 9 4d6|25 19 1d2", 38, 100, {.multiplies = false, .can_move = true}},
  // Level 39
    {20, 55, "Nether Wraith", 0x1F070302, 0x00005316, 0x512E, 100, 2100, 1, 'W', "58d8", "1 1 2-1d12|19 5 52d8", 39, 100, {.multiplies = false, .can_move = true}},
    {20, 50, "Sorcerer", 0x13020300, 0x0200FF73, 0x6030, 100, 2150, 2, 'p', "30d8", "1 1 2d8", 39, 100, {.multiplies = false, .can_move = true}},
    {20, 90, "Ancient Blue Dragon", 0x4F000300, 0x00081A08, 0x6005, 800, 2500, 3, 'D', "87d8", "1 3 2-1d9|1 2 2d12", 39, 125, {.multiplies = false, .can_move = true}},
    {20, 85, "Ancient Green Dragon", 0x4F000300, 0x00101A09, 0x6005, 700, 2400, 3, 'D', "90d8", "1 3 2-1d8|1 2 2d10", 39, 125, {.multiplies = false, .can_move = true}},
    {20, 90, "Ancient Black Dragon", 0x4F000300, 0x00201A07, 0x6005, 700, 2500, 3, 'D', "90d8", "1 3 2-1d9|1 2 2d10", 39, 125, {.multiplies = false, .can_move = true}},
  // Level 40
    {15, 50, "Green Faerie Dragon", 0x41820302, 0x01001E93, 0x4001, 10, 500, 3, 'F', "14d8", "4 8 1d3|3 3 2-2d5", 40, 0, {.multiplies = false, .can_move = true}},
    {15, 45, "Giant Crocodile", 0x00000310, 0x00000000, 0xC000, 10, 1000, 2, 'C', "125d8", "1 2 6d6|1 3 2-3d4|1 17 6d10", 40, 0, {.multiplies = false, .can_move = true}},
    {10, 40, "Great White Shark", 0x00080050, 0x00000000, 0x9022, 0, 2750, 2, 'S', "100d6", "1 2 3-15d4|1 25 2d20", 40, 90, {.multiplies = false, .can_move = true}},
    {30, 50, "Queen Hag", 0x27100302, 0x000103A2, 0x4006, 200, 1150, 2, 'p', "60d6", "1 6 3d6|10 8 5d6|25 19 1d3", 40, 125, {.multiplies = false, .can_move = true}},
    {5, 5, "Disenchanter Worms", 0x00200248, 0x00000000, 0x01B0, 100, 30, 0, 'w', "10d8", "21 12 1d4", 40, 50, {.multiplies = false, .can_move = true}},
    {20, 1, "Rotting Quythulg", 0x00010340, 0x00004010, 0x5000, 0, 1000, 2, 'Q', "12d8", "99 0 0d0", 40, 0, {.multiplies = false, .can_move = true}},
    {20, 100, "Ancient Red Dragon", 0x7F000300, 0x00801E06, 0x6015, 500, 2750, 3, 'D', "105d8", "1 3 2-1d10|1 2 2d14", 40, 125, {.multiplies = false, .can_move = true}},
    {20, 80, "Death Quasit", 0x11030302, 0x000010FA, 0x0004, 0, 1000, 3, 'q', "55d8", "15 2 3d6|1 3 2-3d3", 40, 125, {.multiplies = false, .can_move = true}},
    {20, 75, "Emperor Lich", 0x2F020300, 0x00019F72, 0x500C, 500, 10000, 2, 'L', "190d8", "15 5 2d12|19 5 46d8|24 5 0d0", 40, 150, {.multiplies = false, .can_move = true}},
    {20, 100, "Ancient Multi-Hued Dragon", 0x7F000300, 0x00F89E05, 0x6005, 500, 12000, 3, 'D', "260d8", "1 3 2-1d12|1 2 3d12", 40, 150, {.multiplies = false, .can_move = true}},
  // Level 42
    {20, 90, "Anti-<gp>", 0x32130000, 0x02815775, 0x7414, 600, 1500, 2, 'p', "160d8", "24 5 0d0|5 3 2-5d5|1 1 3d3", 42, 150, {.multiplies = false, .can_move = true}},
    {20, 80, "Lesser Demon Lord", 0x321F0300, 0x02815775, 0x7414, 600, 1500, 2, 'B', "75d8", "24 5 0d0|5 3 2-5d5|4 2 3d3", 42, 160, {.multiplies = false, .can_move = true}},
  // Level 45
    {15, 30, "Empress Hag", 0x4B100302, 0x001103A2, 0x4006, 100, 10000, 3, 'p', "100d6", "1 6 7d6|10 8 6d6|25 19 1d3", 45, 150, {.multiplies = false, .can_move = true}},
  // Level 48
    {20, 60, "Blue Faerie Dragon", 0x49820302, 0x01801E93, 0x4001, 5, 1000, 3, 'F', "24d8", "4 8 2d5|3 3 2-2d8", 48, 0, {.multiplies = false, .can_move = true}},
  // Level 50
    {60, 80, "Sandgorgon", 0x441E0300, 0x00000000, 0x1002, 1, 1200, 4, 'S', "30d8", "1 3 2-4d8|1 2 2d10", 50, 0, {.multiplies = false, .can_move = true}},
    {10, 45, "Tyranosaurus Rex", 0x000A0200, 0x00000000, 0xE022, 25, 6000, 2, 'D', "200d8", "1 2 25d8|1 17 2-3d4", 50, 0, {.multiplies = false, .can_move = true}},
  // Level 56
    {20, 70, "Indigo Faerie Dragon", 0x51820302, 0x01801F13, 0x4001, 0, 2000, 3, 'F', "39d8", "4 8 4d5|3 3 2-3d8", 56, 0, {.multiplies = false, .can_move = true}},
  // Level 60
    {5, 40, "Longsword  +7,+7  of Doom", 0x00880310, 0x80FA0001, 0x5404, 700, 5000, 3, '|', "200d4", "1 1 2-12d8|19 10 46d8", 60, 0, {.multiplies = false, .can_move = true}},
    {15, 65, "Death Knight", 0x47970300, 0x00810374, 0x5014, 75, 1400, 3, 'p', "90d8", "1 1 2-2d10", 60, 150, {.multiplies = false, .can_move = true}},
  // Level 64
    {25, 80, "Purple Faerie Dragon", 0x61820302, 0x01811D12, 0x4001, 0, 4500, 4, 'F', "54d8", "4 8 6d5|3 3 2-4d8", 64, 0, {.multiplies = false, .can_move = true}},
  // Level 70
    {20, 100, "Chromatic Hydra  7-headed ", 0x7F000300, 0x01E89C05, 0x6005, 500, 13000, 3, 'D', "300d8", "1 2 7-2d10", 70, 150, {.multiplies = false, .can_move = true}},
  // Level 80
    {25, 90, "Noble Faerie Dragon", 0x71830302, 0x01811D12, 0x4001, 0, 10000, 4, 'F', "75d8", "4 8 10d5|3 3 2-5d8", 80, 0, {.multiplies = false, .can_move = true}},
  // Level 100
    {25, 100, "Emperor Faerie Dragon", 0x7D830302, 0x01811D12, 0x4001, 0, 25000, 5, 'F', "105d8", "4 8 14d5|3 3 2-7d8", 100, 0, {.multiplies = false, .can_move = true}},
    {20, 80, "Evil Iggy", 0x7F130300, 0x0201D713, 0x5004, 0, 18000, 3, 'p', "400d8", "2 1 4d6|13 19 0d0", 100, 175, {.multiplies = false, .can_move = true}},
    {20, 125, "Balrog", 0xFF1F0300, 0x0281C743, 0x5404, 0, 55000, 4, 'B', "475d8", "5 1 10d12|1 17 8d12|24 5 0d0", 100, 255, {.multiplies = false, .can_move = true}},};
long const monster_template_size = sizeof(monster_templates) / sizeof(monster_templates[0]);

bool monster_template_has_attribute(monster_template const *template,
                                       const monster_attribute attribute) {
  switch (attribute) {
  case ma_move_only_to_attack:
    return !template->attributes.can_move;
  case ma_20pc_random_movement:
    return template->cmove & 0x00000002;
  case ma_40pc_random_movement:
    return template->cmove & 0x00000004;
  case ma_75pc_random_movement:
    return template->cmove & 0x00000008;
  case ma_water_based:
    return template->cmove & 0x00000010;
  case ma_land_based:
    return (template->cmove & 0x00000010) == 0;
  case ma_dies_in_wrong_element:
    return template->cmove & 0x00000040;
  case ma_survives_in_water:
    return monster_template_has_attribute(template, ma_water_based) ||
           !monster_template_has_attribute(template,
                                           ma_dies_in_wrong_element) ||
           monster_template_has_attribute(template, ma_flying);
  case ma_survives_on_land:
    return monster_template_has_attribute(template, ma_land_based) ||
           !monster_template_has_attribute(template,
                                           ma_dies_in_wrong_element) ||
           monster_template_has_attribute(template, ma_flying);
  case ma_good_monster:
    return template->cmove & 0x00004000;
  case ma_unspawnable:
    return template->cmove & 0x00008000;
  case ma_invisible_movement:
    return template->cmove & 0x00010000;
  case ma_moves_through_door:
    return template->cmove & 0x00020000;
  case ma_moves_through_wall:
    return template->cmove & 0x00040000;
  case ma_moves_through_creatures:
    return template->cmove & 0x00080000;
  case ma_picks_up_objects:
    return template->cmove & 0x00100000;
  case ma_multiplies:
    return template->attributes.multiplies;
  case ma_anchors_in_water:
    return template->cmove & 0x00400000;
  case ma_flying:
    return template->cmove & 0x00800000;
  case ma_carries_objects:
    return template->cmove & 0x01000000;
  case ma_carries_gold:
    return template->cmove & 0x02000000;
  case ma_carries_60pc:
    return template->cmove & 0x04000000;
  case ma_carries_90pc:
    return template->cmove & 0x08000000;
  case ma_carries_1d2_things:
    return template->cmove & 0x10000000;
  case ma_carries_2d2_things:
    return template->cmove & 0x20000000;
  case ma_carries_4d2_things:
    return template->cmove & 0x40000000;
  case ma_wins_the_game:
    return template->cmove & 0x80000000;
  case ma_dragon:
    return template->cdefense & 0x0001;
  case ma_monster:
    return template->cdefense & 0x0002;
  case ma_evil:
    return template->cdefense & 0x0004;
  case ma_undead:
    return template->cdefense & 0x0008;
  case ma_demon:
    return template->cdefense & 0x0400;
  case ma_vulnerable_to_frost:
    return template->cdefense & 0x0010;
  case ma_vulnerable_to_fire:
    return template->cdefense & 0x0020;
  case ma_vulnerable_to_poison:
    return template->cdefense & 0x0040;
  case ma_vulnerable_to_acid:
    return template->cdefense & 0x0080;
  case ma_vulnerable_to_lightning:
    return template->cdefense & 0x0100;
  case ma_vulnerable_to_stone_to_mud:
    return template->cdefense & 0x0200;
  case ma_uncharmable:
    return template->cdefense & 0x1000;
  case ma_visible_with_infravision:
    return template->cdefense & 0x2000;
  case ma_max_hit_points:
    return template->cdefense & 0x4000;
  case ma_regenerates:
    return template->cdefense & 0x8000;
  }
  return false;
}

bool monster_template_has_attributes(
    monster_template const *template,
    monster_attribute const *const *monster_attributes) {
  for (monster_attribute const *attribute = *monster_attributes;
       attribute != NULL; attribute++) {
    if (!monster_template_has_attribute(template, *attribute)) {
      return false;
    }
  }
  return true;
}