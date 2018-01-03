#include "imoria.h"

char const *class_title(int class)
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
		MSG("Logic error in class_title")
		return "???";
	}
}

float class_expfactor(int class)
{
	switch (class) {
	case C_WARRIOR:
		return 0.00;
	case C_MAGE:
		return 0.30;
	case C_PRIEST:
		return 0.20;
	case C_ROGUE:
		return 0.10;
	case C_RANGER:
		return 0.30;
	case C_PALADIN:
		return 0.35;
	case C_DRUID:
		return 0.20;
	case C_BARD:
		return 0.30;
	case C_ADVENTURER:
		return 0.40;
	case C_MONK:
		return 0.10;
	default:
		MSG("Logic error in class_expfactor")
		return 1000.00;
	}
}

signed char class_extra_health(int class)
{
	switch (class) {
	case C_WARRIOR:
		return 10;
	case C_MAGE:
		return 0;
	case C_PRIEST:
		return 3;
	case C_ROGUE:
		return 6;
	case C_RANGER:
		return 4;
	case C_PALADIN:
		return 6;
	case C_DRUID:
		return 3;
	case C_BARD:
		return 4;
	case C_ADVENTURER:
		return 4;
	case C_MONK:
		return 4;
	default:
		MSG("Logic error in class_extra_health")
		return -100;
	}
}

signed char class_disarm_mod(int class)
{
	switch (class) {
	case C_WARRIOR:
		return 25;
	case C_MAGE:
		return 30;
	case C_PRIEST:
		return 25;
	case C_ROGUE:
		return 45;
	case C_RANGER:
		return 30;
	case C_PALADIN:
		return 20;
	case C_DRUID:
		return 25;
	case C_BARD:
		return 30;
	case C_ADVENTURER:
		return 30;
	case C_MONK:
		return 45;
	default:
		MSG("Logic error in class_disarm_mod")
		return -100;
	}
}

signed char class_search_mod(int class)
{
	switch (class) {
	case C_WARRIOR:
		return 14;
	case C_MAGE:
		return 16;
	case C_PRIEST:
		return 16;
	case C_ROGUE:
		return 32;
	case C_RANGER:
		return 24;
	case C_PALADIN:
		return 12;
	case C_DRUID:
		return 16;
	case C_BARD:
		return 22;
	case C_ADVENTURER:
		return 24;
	case C_MONK:
		return 24;
	default:
		MSG("Logic error in class_search_mod")
		return -100;
	}
}

signed char class_stealth_mod(int class)
{
	switch (class) {
	case C_WARRIOR:
		return 1;
	case C_MAGE:
		return 2;
	case C_PRIEST:
		return 2;
	case C_ROGUE:
		return 4;
	case C_RANGER:
		return 3;
	case C_PALADIN:
		return 1;
	case C_DRUID:
		return 1;
	case C_BARD:
		return 2;
	case C_ADVENTURER:
		return 3;
	case C_MONK:
		return 3;
	default:
		MSG("Logic error in class_stealth_mod")
		return -100;
	}
}

signed char class_search_freq(int class)
{
	switch (class) {
	case C_WARRIOR:
		return 38;
	case C_MAGE:
		return 36;
	case C_PRIEST:
		return 32;
	case C_ROGUE:
		return 16;
	case C_RANGER:
		return 24;
	case C_PALADIN:
		return 38;
	case C_DRUID:
		return 32;
	case C_BARD:
		return 28;
	case C_ADVENTURER:
		return 24;
	case C_MONK:
		return 24;
	default:
		MSG("Logic error in class_search_freq")
		return -100;
	}
}

signed char class_melee_bonus(int class)
{
	switch (class) {
	case C_WARRIOR:
		return 10;
	case C_MAGE:
		return 4;
	case C_PRIEST:
		return 6;
	case C_ROGUE:
		return 6;
	case C_RANGER:
		return 6;
	case C_PALADIN:
		return 8;
	case C_DRUID:
		return 4;
	case C_BARD:
		return 5;
	case C_ADVENTURER:
		return 6;
	case C_MONK:
		return 8;
	default:
		MSG("Logic error in class_melee_bonus")
		return -100;
	}
}

signed char class_ranged_bonus(int class)
{
	switch (class) {
	case C_WARRIOR:
		return 8;
	case C_MAGE:
		return 4;
	case C_PRIEST:
		return 5;
	case C_ROGUE:
		return 10;
	case C_RANGER:
		return 10;
	case C_PALADIN:
		return 6;
	case C_DRUID:
		return 7;
	case C_BARD:
		return 6;
	case C_ADVENTURER:
		return 6;
	case C_MONK:
		return 6;
	default:
		MSG("Logic error in class_ranged_bonus")
		return -100;
	}
}

signed char class_save_mod(int class)
{
	switch (class) {
	case C_WARRIOR:
		return 10;
	case C_MAGE:
		return 25;
	case C_PRIEST:
		return 20;
	case C_ROGUE:
		return 15;
	case C_RANGER:
		return 20;
	case C_PALADIN:
		return 15;
	case C_DRUID:
		return 20;
	case C_BARD:
		return 20;
	case C_ADVENTURER:
		return 20;
	case C_MONK:
		return 25;
	default:
		MSG("Logic error in class_save_mod")
		return -100;
	}
}

signed char const *class_stats(int class)
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
		MSG("Logic error in class_stats")
		return NULL;
	}
}

boolean class_uses_magic(int class, enum magic_t magic_type)
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
	}
}

signed char class_magic_resist(int class)
{
	switch (class) {
	case C_WARRIOR:
		return -10;
	case C_MAGE:
		return 0;
	case C_PRIEST:
		return 0;
	case C_ROGUE:
		return -5;
	case C_RANGER:
		return -5;
	case C_PALADIN:
		return -5;
	case C_DRUID:
		return -5;
	case C_BARD:
		return -5;
	case C_ADVENTURER:
		return -5;
	case C_MONK:
		return -5;
	default:
		MSG("Logic error in class_magic_resist")
		return 0;
	}
}
