#include "imoria.h"

char const *class_title(enum class_t class)
{
	switch (class) {
	case C_WARRIOR:
		return "Warrior";
	case C_MAGE:
		return "Mage";
	case C_PRIEST:
		return "Priest";
	case C_ROGUE:
		return "Rogue";
	case C_RANGER:
		return "Ranger";
	case C_PALADIN:
		return "Paladin";
	case C_DRUID:
		return "Druid";
	case C_BARD:
		return "Bard";
	case C_ADVENTURER:
		return "Adventurer";
	case C_MONK:
		return "Monk";
	default:
		MSG(("Unknown class: %d", class));
		return "???";
	}
}

signed char const *class_stats(enum class_t class)
{
	static signed char const warrior_stats[] = {5, -2, -2, 2, 2, 1};
	static signed char const mage_stats[] = {-5, 3, 0, 0, -2, 0};
	static signed char const priest_stats[] = {0, -3, 3, -1, 1, 2};
	static signed char const rogue_stats[] = {2, 0, -2, 3, 1, 1};
	static signed char const ranger_stats[] = {2, 0, 1, 1, 1, 2};
	static signed char const paladin_stats[] = {3, -3, 1, 0, 2, 2};
	static signed char const druid_stats[] = {-1, -1, 2, 0, 0, 3};
	static signed char const bard_stats[] = {2, 1, -1, 2, 0, 3};
	static signed char const adventurer_stats[] = {2, 2, -1, 1, 1, 0};
	static signed char const monk_stats[] = {2, 0, 2, 3, 1, 1};
	switch (class) {
	case C_WARRIOR:
		return warrior_stats;
	case C_MAGE:
		return mage_stats;
	case C_PRIEST:
		return priest_stats;
	case C_ROGUE:
		return rogue_stats;
	case C_RANGER:
		return ranger_stats;
	case C_PALADIN:
		return paladin_stats;
	case C_DRUID:
		return druid_stats;
	case C_BARD:
		return bard_stats;
	case C_ADVENTURER:
		return adventurer_stats;
	case C_MONK:
		return monk_stats;
	default:
		MSG(("Unknown class: %d", class));
		return NULL;
	}
}

boolean class_uses_magic(enum class_t class, enum magic_t magic_type)
{
	switch (magic_type) {
	case M_ARCANE:
		return class == C_MAGE || class == C_ADVENTURER;
	case M_DIVINE:
		return class == C_PRIEST || class == C_PALADIN;
	case M_NATURE:
		return class == C_RANGER || class == C_DRUID;
	case M_SONG:
		return class == C_ROGUE || class == C_BARD;
	case M_CHAKRA:
		return class == C_MONK;
	default:
		return false;
	}
}

spell_t *class_spell(enum class_t class, int slot)
{
	static spell_t no_spell = {"", 99, 99, 0, 0, false};

	static spell_t mage_spells[] = {
	    {"Magic Missile", 1, 1, 5, 22, false},
	    {"Detect Monsters", 1, 1, 5, 23, false},
	    {"Phase Door", 1, 2, 4, 24, false},
	    {"Light Area", 1, 2, 4, 26, false},
	    {"Cure Light Wounds", 3, 3, 8, 25, false},
	    {"Find Hidden Traps/Doors", 3, 3, 6, 55, false},
	    {"Stinking Cloud", 3, 4, 8, 27, false},
	    {"Confusion", 3, 4, 6, 30, false},
	    {"Lightning Bolt", 5, 4, 25, 30, false},
	    {"Trap/Door Destruction", 5, 5, 30, 30, false},
	    {"Sleep I", 5, 5, 20, 30, false},
	    {"Cure Poison", 5, 5, 25, 35, false},
	    {"Shadow Door", 7, 6, 35, 35, false},
	    {"Remove Curse", 7, 6, 40, 50, false},
	    {"Frost Bolt", 7, 6, 50, 40, false},
	    {"Create Food", 7, 6, 50, 40, false},
	    {"Infravision", 7, 6, 50, 40, false},
	    {"Invisibility", 7, 6, 60, 35, false},
	    {"Turn Stone to Mud", 9, 7, 75, 44, false},
	    {"Recharge Item I", 9, 7, 90, 75, false},
	    {"Sleep II", 9, 7, 75, 45, false},
	    {"Phantasmal Force", 11, 7, 80, 50, false},
	    {"Polymorph Other", 11, 7, 100, 45, false},
	    {"Identify", 11, 7, 75, 99, false},
	    {"Ring of Frost", 13, 7, 75, 45, false},
	    {"Sleep III", 13, 7, 90, 50, false},
	    {"Hold Monster", 15, 9, 100, 50, false},
	    {"Fire Bolt", 15, 9, 100, 50, false},
	    {"Slow Creature", 17, 9, 125, 50, false},
	    {"Protection from Magic", 17, 9, 125, 55, false},
	    {"Frost Ball", 19, 12, 150, 55, false},
	    {"Death Spell", 19, 18, 200, 55, false},
	    {"Ring of Fire", 21, 12, 175, 60, false},
	    {"Recharge Item II", 21, 12, 175, 90, false},
	    {"Teleport Other", 23, 15, 200, 60, false},
	    {"Haste Self", 25, 15, 250, 65, false},
	    {"Fire Ball", 28, 18, 350, 65, false},
	    {"Power Word: Destruction", 31, 21, 500, 80, false},
	    {"Power Word: Kill", 34, 25, 600, 80, false},
	    {"Genocide", 37, 25, 800, 95, false}};

	static spell_t priest_spells[] = {
	    {"Detect Evil", 1, 1, 3, 10, false},
	    {"Cure Light Wounds", 1, 2, 4, 15, false},
	    {"Bless", 1, 2, 3, 20, false},
	    {"Remove Fear", 1, 2, 3, 25, false},
	    {"Call Light", 3, 2, 6, 25, false},
	    {"Find Traps", 3, 3, 8, 27, false},
	    {"Detect Doors/Stairs", 3, 3, 8, 27, false},
	    {"Slow Poison", 3, 3, 10, 28, false},
	    {"Blind Creature", 5, 4, 16, 29, false},
	    {"Portal", 5, 4, 20, 30, false},
	    {"Cure Medium Wounds", 5, 4, 20, 32, false},
	    {"Ray of Sanctification", 5, 4, 20, 40, false},
	    {"Heroism", 7, 4, 20, 40, false},
	    {"Sanctuary", 7, 5, 30, 36, false},
	    {"Remove Curse", 7, 6, 35, 38, false},
	    {"Resist Heat and Cold", 7, 7, 35, 38, false},
	    {"Silence", 9, 7, 40, 40, false},
	    {"Resist Petrification", 9, 7, 40, 40, false},
	    {"Neutralize Poison", 9, 7, 40, 38, false},
	    {"Cure Serious Wounds", 9, 7, 40, 40, false},
	    {"Chant", 11, 8, 20, 34, false},
	    {"Sense Invisible", 11, 8, 40, 42, false},
	    {"Protection from Evil", 11, 8, 50, 42, false},
	    {"Earthquake", 11, 9, 60, 55, false},
	    {"Create Food and Drink", 13, 9, 30, 38, false},
	    {"Sense Surroundings", 13, 10, 60, 45, false},
	    {"Orb of Draining", 13, 10, 40, 58, false},
	    {"Cure Critical Wounds", 15, 11, 65, 45, false},
	    {"Turn Undead", 15, 12, 80, 50, false},
	    {"Holy Prayer", 17, 14, 90, 50, false},
	    {"Dispel Undead", 17, 14, 125, 55, false},
	    {"Resist Paralysis", 19, 15, 150, 55, false},
	    {"Blade Barrier", 21, 16, 175, 60, false},
	    {"Dispel Evil", 23, 18, 200, 70, false},
	    {"Heal", 25, 20, 250, 60, false},
	    {"Resist Magic", 27, 22, 300, 80, false},
	    {"Holy of Thunder", 30, 23, 400, 70, false},
	    {"Glyph of Warding", 33, 24, 500, 90, false},
	    {"Hero's Feast", 35, 28, 600, 95, false},
	    {"Holy Word", 39, 32, 800, 99, false}};

	static spell_t rogue_spells[] = {
	    {"Detect Monsters", 3, 2, 6, 35, false},
	    {"Battle Song", 3, 2, 7, 35, false},
	    {"Blink", 3, 2, 8, 35, false},
	    {"Light Area", 5, 4, 8, 35, false},
	    {"Find Hidden Doors/Traps", 5, 5, 10, 45, false},
	    {"Magical Jig", 7, 8, 15, 40, false},
	    {"Detect Magic", 7, 9, 20, 70, false},
	    {"", 99, 99, 0, 0, false},
	    {"Battle Dance", 9, 10, 25, 45, false},
	    {"Charm Monsters", 9, 11, 40, 50, false},
	    {"Detect Curse", 11, 12, 30, 40, false},
	    {"Detect Invisible", 11, 12, 35, 40, false},
	    {"Cure Poison", 13, 14, 40, 45, false},
	    {"Invisibility", 15, 16, 50, 50, false},
	    {"Shadow Gate", 17, 18, 60, 55, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"Recharge Item", 19, 18, 70, 55, false},
	    {"Remove Curse", 21, 20, 50, 90, false},
	    {"Legend Lore", 23, 22, 80, 95, false},
	    {"", 99, 99, 0, 0, false},
	    {"Detect Treasure", 25, 25, 60, 50, false},
	    {"Detect Object", 25, 25, 60, 55, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"Word of Recall", 27, 27, 90, 60, false},
	    {"", 99, 99, 0, 0, false},
	    {"See Invisible", 29, 29, 100, 65, false},
	    {"Magic Mapping", 31, 30, 110, 70, false},
	    {"", 99, 99, 0, 0, false},
	    {"Battle Frenzy", 33, 31, 125, 70, false},
	    {"", 99, 99, 0, 0, false},
	    {"Resist Charm", 35, 32, 150, 70, false},
	    {"Item Lore", 37, 33, 200, 95, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false}};

	static spell_t ranger_spells[] = {
	    {"Moon Beam", 1, 2, 3, 20, false},
	    {"Detect Monster", 1, 2, 5, 33, false},
	    {"Battle Song", 2, 3, 8, 35, false},
	    {"Light", 2, 3, 13, 35, false},
	    {"Minor Cure", 3, 4, 15, 40, false},
	    {"Find Safe Path", 4, 5, 10, 45, false},
	    {"Magical Jig", 4, 5, 17, 45, false},
	    {"Warp Wood", 4, 5, 17, 45, false},
	    {"Battle Dance", 5, 5, 25, 40, false},
	    {"Cure Poison", 5, 5, 30, 48, false},
	    {"Charm", 7, 5, 35, 35, false},
	    {"Detect Curse", 7, 6, 40, 45, false},
	    {"Summon Insects", 7, 7, 40, 50, false},
	    {"Call Lightning", 9, 9, 40, 55, false},
	    {"Magic Resistance", 9, 10, 45, 45, false},
	    {"", 99, 99, 0, 0, false},
	    {"Create Food", 9, 10, 40, 55, false},
	    {"Remove Curse", 11, 10, 50, 45, false},
	    {"Infravision", 11, 10, 50, 50, false},
	    {"Major Cure", 11, 11, 55, 50, false},
	    {"Resist Petrification", 13, 11, 60, 55, false},
	    {"Transplant", 13, 11, 60, 45, false},
	    {"", 99, 99, 0, 0, false},
	    {"Dispel Magic", 15, 11, 60, 58, false},
	    {"Fire Stream", 15, 11, 60, 70, false},
	    {"Protection from Nature", 17, 12, 65, 55, false},
	    {"Stone to Mud", 17, 12, 65, 55, false},
	    {"Goodberry", 19, 14, 70, 65, false},
	    {"Creeping Doom", 37, 21, 120, 65, false},
	    {"Pillar of Fire", 23, 15, 80, 65, false},
	    {"Word of Recall", 25, 15, 85, 50, false},
	    {"Lightning Ball", 27, 15, 90, 90, false},
	    {"Word of Blindness", 29, 16, 95, 55, false},
	    {"Protection from Monsters", 31, 20, 100, 60, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"Resist Charm", 33, 22, 110, 65, false},
	    {"Battle Frenzy", 35, 25, 125, 65, false},
	    {"Dispel Monster", 37, 30, 200, 70, false},
	    {"", 99, 99, 0, 0, false}};

	static spell_t paladin_spells[] = {
	    {"Detect Evil", 1, 1, 4, 30, false},
	    {"Cure Light Wounds", 2, 2, 8, 35, false},
	    {"Bless", 3, 3, 12, 35, false},
	    {"Remove Fear", 5, 3, 20, 35, false},
	    {"Call Light", 5, 4, 20, 35, false},
	    {"Find Traps", 7, 5, 25, 40, false},
	    {"Detect Doors/Stairs", 7, 5, 25, 40, false},
	    {"Slow Poison", 9, 7, 30, 40, false},

	    {"Blind Creature", 9, 7, 30, 40, false},
	    {"Portal", 9, 8, 30, 40, false},
	    {"Cure Medium Wounds", 11, 9, 40, 40, false},
	    {"Ray of Sanctification", 11, 9, 40, 40, false},
	    {"Heroism", 11, 9, 40, 40, false},
	    {"Sanctuary", 13, 10, 40, 45, false},
	    {"Remove Curse", 13, 11, 50, 45, false},
	    {"Resist Heat and Cold", 15, 13, 60, 45, false},
	    {"Silence", 15, 13, 60, 45, false},
	    {"Resist Petrification", 15, 13, 60, 50, false},

	    {"Neutralize Poison", 15, 15, 60, 50, false},
	    {"Cure Serious Wounds", 17, 15, 70, 50, false},
	    {"Chant", 17, 15, 70, 50, false},
	    {"Sense Invisible", 19, 15, 75, 50, false},
	    {"Protection from Evil", 19, 15, 75, 50, false},
	    {"Earthquake", 21, 17, 80, 50, false},
	    {"Create Food and Drink", 21, 10, 50, 45, false},
	    {"Sense Surroundings", 23, 17, 80, 50, false},
	    {"Orb of Draining", 23, 20, 70, 85, false},
	    {"Cure Critical Wounds", 25, 20, 80, 50, false},
	    {"Turn Undead", 25, 21, 90, 50, false},
	    {"Holy Prayer", 27, 22, 100, 50, false},

	    {"Dispel Undead", 29, 24, 110, 60, false},
	    {"Resist Paralysis", 31, 26, 125, 55, false},
	    {"Blade Barrier", 33, 27, 150, 60, false},
	    {"Dispel Evil", 35, 28, 175, 70, false},
	    {"Heal", 37, 32, 200, 75, false},
	    {"Resist Magic", 38, 32, 250, 75, false},
	    {"Holy Thunder", 39, 30, 250, 80, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false}};

	static spell_t druid_spells[] = {
	    {"Moon Beam", 1, 1, 3, 15, false},
	    {"Detect Monster", 1, 1, 4, 15, false},
	    {"Battle Song", 1, 2, 4, 20, false},
	    {"Light", 1, 2, 4, 25, false},
	    {"Minor Cure", 3, 3, 8, 25, false},
	    {"Find Safe Path", 3, 3, 8, 28, false},
	    {"Magical Jig", 3, 3, 10, 27, false},
	    {"Warp Wood", 3, 4, 12, 29, false},
	    {"Battle Dance", 5, 4, 20, 30, false},
	    {"Cure Poison", 5, 4, 25, 30, false},
	    {"Charm", 5, 5, 35, 45, false},
	    {"Detect Curse", 5, 5, 40, 50, false},
	    {"Summon Insects", 7, 5, 35, 35, false},
	    {"Call Lightning", 7, 5, 35, 38, false},
	    {"Magic Resistance", 7, 6, 45, 40, false},
	    {"Starlight", 7, 6, 45, 40, false},
	    {"Create Food", 9, 6, 40, 40, false},
	    {"Remove Curse", 9, 6, 45, 55, false},
	    {"Infravision", 9, 7, 60, 40, false},
	    {"Major Cure", 9, 7, 70, 40, false},
	    {"Resist Petrification", 11, 7, 60, 45, false},
	    {"Transplant", 11, 8, 70, 55, false},
	    {"Sunray", 11, 9, 70, 60, false},
	    {"Dispel Magic", 11, 9, 80, 50, false},
	    {"Fire Stream", 11, 9, 80, 65, false},
	    {"Protection from Nature", 13, 9, 90, 40, false},
	    {"Stone to Mud", 13, 9, 100, 50, false},
	    {"Goodberry", 15, 11, 100, 50, false},
	    {"Creeping Doom", 33, 17, 200, 65, false},
	    {"Pillar of Fire", 15, 11, 110, 55, false},
	    {"Word of Recall", 17, 11, 100, 60, false},
	    {"Lightning Ball", 17, 12, 100, 55, false},
	    {"Word of Blindness", 19, 12, 120, 55, false},
	    {"Protection from Monsters", 21, 14, 140, 55, false},
	    {"Control Temperature", 23, 17, 160, 60, false},
	    {"Ring of Fire", 25, 19, 180, 75, false},
	    {"Resist Charm", 27, 21, 200, 75, false},
	    {"Battle Frenzy", 29, 23, 300, 70, false},
	    {"Dispel Monster", 31, 25, 400, 80, false},
	    {"Note of Destruction", 34, 27, 500, 85, false}};

	static spell_t bard_spells[] = {
	    {"Detect Monsters", 1, 1, 6, 35, false},
	    {"Battle Song", 1, 2, 7, 35, false},
	    {"Blink", 1, 2, 8, 35, false},
	    {"Light Area", 1, 2, 8, 35, false},
	    {"Detect Hidden Doors/Traps", 3, 3, 10, 45, false},
	    {"Magical Jig", 3, 4, 20, 40, false},
	    {"Detect Magic", 3, 4, 50, 60, false},
	    {"Minor Cure", 3, 4, 20, 40, false},
	    {"Battle Dance", 5, 5, 30, 45, false},
	    {"Charm Monsters", 5, 5, 40, 40, false},
	    {"Detect Curse", 5, 9, 60, 50, false},
	    {"Detect Invisible", 7, 8, 40, 45, false},
	    {"Cure Poison", 7, 8, 40, 45, false},
	    {"Invisibility", 7, 11, 60, 55, false},
	    {"Teleport Self", 9, 10, 60, 50, false},
	    {"Infravision", 9, 11, 50, 55, false},
	    {"Physical Humor", 9, 11, 70, 55, false},
	    {"Recharge Item", 11, 11, 60, 85, false},
	    {"Remove Curse", 13, 12, 50, 55, false},
	    {"Legend Lore", 11, 13, 80, 99, false},
	    {"Mass Charm", 11, 12, 70, 55, false},
	    {"Detect Treasure", 13, 11, 60, 80, false},
	    {"Detect Object", 13, 11, 60, 80, false},
	    {"Resist Petrification", 15, 12, 70, 60, false},
	    {"Create Food and Drink", 15, 13, 80, 60, false},
	    {"Panic", 15, 15, 80, 60, false},
	    {"Word of Recall", 17, 15, 80, 60, false},
	    {"Protection from Nature", 17, 16, 80, 65, false},
	    {"See Invisible", 17, 16, 80, 60, false},
	    {"Magic Mapping", 19, 18, 90, 65, false},
	    {"Joke of Death", 19, 18, 80, 60, false},
	    {"Battle Frenzy", 19, 18, 90, 80, false},
	    {"Slow Creature", 21, 19, 100, 65, false},
	    {"Resist Charm", 23, 22, 120, 65, false},
	    {"Item Lore", 25, 20, 110, 90, false},
	    {"Song of Protection", 27, 25, 200, 70, false},
	    {"Last Laugh", 29, 23, 150, 70, false},
	    {"Teleport Level", 31, 27, 300, 75, false},
	    {"Clairvoyance", 35, 29, 500, 92, false},
	    {"Song of Power", 39, 32, 800, 97, false}};

	static spell_t adventurer_spells[] = {
	    {"Magic Missile", 3, 1, 6, 30, false},
	    {"Detect Monsters", 3, 2, 6, 35, false},
	    {"Phase Door", 3, 2, 8, 35, false},
	    {"Light Area", 5, 3, 8, 26, false},
	    {"Cure Light Wounds", 5, 3, 8, 25, false},
	    {"Find Hidden Traps/Doors", 7, 4, 10, 45, false},
	    {"Stinking Cloud", 7, 6, 24, 40, false},
	    {"Confusion", 9, 6, 20, 40, false},
	    {"Lightning Bolt", 9, 7, 30, 40, false},
	    {"Trap/Door Destruction", 11, 8, 30, 45, false},
	    {"Sleep I", 11, 8, 40, 40, false},
	    {"Cure Poison", 13, 9, 40, 45, false},
	    {"Shadow Door", 13, 10, 50, 45, false},
	    {"Remove Curse", 15, 11, 50, 55, false},
	    {"Frost Bolt", 15, 12, 60, 50, false},
	    {"Create Food", 17, 12, 60, 55, false},
	    {"Infravision", 17, 12, 60, 50, false},
	    {"Invisibility", 17, 13, 60, 50, false},
	    {"Turn Stone to Mud", 19, 15, 70, 50, false},
	    {"Recharge Item I", 19, 17, 70, 90, false},
	    {"Sleep II", 21, 17, 70, 55, false},
	    {"Phantasmal Force", 21, 19, 60, 50, false},
	    {"Polymorph Other", 23, 19, 70, 60, false},
	    {"Identify", 23, 25, 70, 95, false},
	    {"Ring of Frost", 25, 19, 80, 60, false},
	    {"Sleep III", 25, 20, 80, 60, false},
	    {"Hold Monster", 27, 20, 80, 50, false},
	    {"Fire Bolt", 27, 20, 80, 60, false},
	    {"Slow Creature", 29, 21, 90, 65, false},
	    {"Protection From Magic", 31, 21, 90, 70, false},
	    {"Frost Ball", 31, 21, 90, 65, false},
	    {"Death Spell", 33, 21, 100, 60, false},
	    {"Ring of Fire", 35, 21, 110, 65, false},
	    {"Recharge Item II", 35, 23, 120, 95, false},
	    {"Teleport Other", 37, 25, 150, 70, false},
	    {"Haste Self", 37, 25, 200, 75, false},
	    {"Fire Ball", 39, 27, 250, 80, false},
	    {"Power Word: Destruction", 39, 30, 300, 95, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false}};

	static spell_t monk_spells[] = {
	    {"Self-Healing", 3, 3, 5, 50, false},
	    {"Courage", 5, 5, 10, 50, false},
	    {"Slow Poison", 7, 7, 10, 50, false},
	    {"Negate Hunger", 9, 9, 15, 50, false},
	    {"Sense Enemies", 11, 11, 20, 50, false},
	    {"Self-Healing II", 13, 13, 20, 50, false},
	    {"Night Vision", 15, 15, 30, 50, false},
	    {"Poison Immunity", 17, 17, 40, 50, false},
	    {"See Invisible", 19, 19, 50, 50, false},
	    {"Advanced Self-Healing", 21, 21, 60, 50, false},
	    {"Resist Petrification", 23, 23, 70, 50, false},
	    {"Stealth", 25, 25, 80, 50, false},
	    {"Free Action", 27, 27, 90, 50, false},
	    {"Improved Speed", 29, 29, 100, 50, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false},
	    {"", 99, 99, 0, 0, false}};

	ENTER(("class_spell", "%d, %d", class, slot));
	if (slot < 0 || MAX_SPELLS <= slot) {
		MSG(("ERROR: spell out of bounds"));
		abort();
		return &no_spell;
	}

	switch (class) {
	default:
	case C_WARRIOR:
		return &no_spell;
	case C_MAGE:
		return &mage_spells[slot];
	case C_PRIEST:
		return &priest_spells[slot];
	case C_ROGUE:
		return &rogue_spells[slot];
	case C_RANGER:
		return &ranger_spells[slot];
	case C_PALADIN:
		return &paladin_spells[slot];
	case C_DRUID:
		return &druid_spells[slot];
	case C_BARD:
		return &bard_spells[slot];
	case C_ADVENTURER:
		return &adventurer_spells[slot];
	case C_MONK:
		return &monk_spells[slot];
	}
	LEAVE("class_spell", "");
}
