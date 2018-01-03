#include "imoria.h"

char const *race_name(int race) {
	switch (race) {
		case R_HUMAN: return "Human";
		case R_HALF_ELF: return "Half-Elf";
		case R_ELF: return "Elf";
		case R_HALFLING: return "Halfling";
		case R_GNOME: return "Gnome";
		case R_DWARF: return "Dwarf";
		case R_HALF_ORC: return "Half-Orc";
		case R_HALF_TROLL: return "Half-Troll";
		case R_PHRAINT: return "Phraint";
		case R_DRYAD: return "Dryad";
		default:
			      MSG("Unknown race");
			      return "???";
	}
}

signed char const *race_stats(int race) {
	static signed char const human_stats[] = {0, 0, 0, 0, 0, 0};
	static signed char const halfelf_stats[] = {-1, 1, 0, 1, -1, 1};
	static signed char const elf_stats[] = {-1, 2, 1, 1, -2, 1};
	static signed char const halfling_stats[] = {-2, 2, 1, 3, 1, 1};
	static signed char const gnome_stats[] = {-1, 2, 0, 2, 1, -2};
	static signed char const dwarf_stats[] = {2, -3, 1, -2, 2, -3};
	static signed char const halforc_stats[] = {2, -1, 0, 0, 1, -4};
	static signed char const halftroll_stats[] = {4, -4, -3, -4, 4, -6};
	static signed char const phraint_stats[] = {0, 0, -4, 5, 0, -3};
	static signed char const dryad_stats[] = {-1, 0, 3, 0, -2, 3};
	switch (race) {
		case R_HUMAN: return human_stats;
		case R_HALF_ELF: return halfelf_stats;
		case R_ELF: return elf_stats;
		case R_HALFLING: return halfling_stats;
		case R_GNOME: return gnome_stats;
		case R_DWARF: return dwarf_stats;
		case R_HALF_ORC: return halforc_stats;
		case R_HALF_TROLL: return halftroll_stats;
		case R_PHRAINT: return phraint_stats;
		case R_DRYAD: return dryad_stats;
		default:
			      MSG("Unknown race");
			      return NULL;
	}
}

unsigned race_rand_starting_age(int race) {
	switch (race) {
		case R_HUMAN: return 14 + randint(6);
		case R_HALF_ELF: return 24 + randint(16);
		case R_ELF: return 75 + randint(75);
		case R_HALFLING: return 21 + randint(12);
		case R_GNOME: return 50 + randint(40);
		case R_DWARF: return 35 + randint(15);
		case R_HALF_ORC: return 11 + randint(4);
		case R_HALF_TROLL: return 20 + randint(10);
		case R_PHRAINT: return 15 + randint(10);
		case R_DRYAD: return 75 + randint(75);
		default:
			      MSG("Unknown race");
			      return -1000;
	}
}

unsigned race_rand_starting_height(int race, boolean male) {
	switch (race) {
		case R_HUMAN: return male ? randnor(72, 6) : randnor(66, 4);
		case R_HALF_ELF: return male ? randnor(66, 6) : randnor(62, 6);
		case R_ELF: return male ? randnor(60, 4) : randnor(54, 4);
		case R_HALFLING: return male ? randnor(36, 3) : randnor(33, 3);
		case R_GNOME: return male ? randnor(42, 3) : randnor(39, 3);
		case R_DWARF: return male ? randnor(48, 3) : randnor(46, 3);
		case R_HALF_ORC: return male ? randnor(66, 1) : randnor(62, 3);
		case R_HALF_TROLL: return male ? randnor(96, 10) : randnor(84, 8);
		case R_PHRAINT: return male ? randnor(96, 24) : randnor(84, 12);
		case R_DRYAD: return male ? randnor(60, 4) : randnor(40, 4);
		default:
			      MSG("Unknown race");
			      return -100;
	}
}

unsigned race_weight_base(int race, boolean male) {
	switch (race) {
		case R_HUMAN: return male ? 180 : 120;
		case R_HALF_ELF: return male ? 130 : 100;
		case R_ELF: return male ? 100 : 80;
		case R_HALFLING: return male ? 60 : 50;
		case R_GNOME: return male ? 90 : 75;
		case R_DWARF: return male ? 150 : 120;
		case R_HALF_ORC: return male ? 150 : 120;
		case R_HALF_TROLL: return male ? 300 : 260;
		case R_PHRAINT: return male ? 100 : 95;
		case R_DRYAD: return male ? 85 : 70;
		default:
			      MSG("Unknown race");
			      return -100;
	}
}

unsigned race_weight_modifier(int race, boolean male) {
	switch (race) {
		case R_HUMAN: return male ? 25 : 20;
		case R_HALF_ELF: return male ? 15 : 10;
		case R_ELF: return 6;
		case R_HALFLING: return 3;
		case R_GNOME: return male ? 6 : 3;
		case R_DWARF: return 10;
		case R_HALF_ORC: return 5;
		case R_HALF_TROLL: return male ? 50 : 40;
		case R_PHRAINT: return male ? 20 : 16;
		case R_DRYAD: return 6;
		default:
			      MSG("Unknown race");
			      return -100;
	}
}

unsigned race_rand_starting_weight(int race, boolean male) {
	return randnor(race_weight_base(race, male), race_weight_modifier(race, male));
}

float race_expfactor(int race) {
	switch (race) {
		case R_HUMAN: return 1.00;
		case R_HALF_ELF: return 1.10;
		case R_ELF: return 1.20;
		case R_HALFLING: return 1.10;
		case R_GNOME: return 1.25;
		case R_DWARF: return 1.20;
		case R_HALF_ORC: return 1.10;
		case R_HALF_TROLL: return 1.20;
		case R_PHRAINT: return 1.20;
		case R_DRYAD: return 1.20;
		default:
			      MSG("Unknown race");
			      return -100;
	}
}

signed char race_disarm_mod(int race) {
	switch (race) {
		case R_HUMAN: return 0;
		case R_HALF_ELF: return 2;
		case R_ELF: return 5;
		case R_HALFLING: return 15;
		case R_GNOME: return 10;
		case R_DWARF: return 2;
		case R_HALF_ORC: return -3;
		case R_HALF_TROLL: return -5;
		case R_PHRAINT: return 15;
		case R_DRYAD: return 2;
		default:
			      MSG("Unknown race");
			      return -100;
	}
}

signed char race_search_mod(int race) {
	switch (race) {
		case R_HUMAN: return 0;
		case R_HALF_ELF: return 6;
		case R_ELF: return 8;
		case R_HALFLING: return 12;
		case R_GNOME: return 6;
		case R_DWARF: return 7;
		case R_HALF_ORC: return 0;
		case R_HALF_TROLL: return -1;
		case R_PHRAINT: return 10;
		case R_DRYAD: return 6;
		default:
			      MSG("Unknown race");
			      return -100;
	}
}

signed char race_stealth_mod(int race) {
	switch (race) {
		case R_HUMAN: return 0;
		case R_HALF_ELF: return 1;
		case R_ELF: return 1;
		case R_HALFLING: return 4;
		case R_GNOME: return 3;
		case R_DWARF: return 0;
		case R_HALF_ORC: return -1;
		case R_HALF_TROLL: return -2;
		case R_PHRAINT: return 5;
		case R_DRYAD: return 1;
		default:
			      MSG("Unknown race");
			      return -100;
	}
}

signed char race_search_freq(int race) {
	switch (race) {
		case R_HUMAN: return 0;
		case R_HALF_ELF: return -1;
		case R_ELF: return -2;
		case R_HALFLING: return -5;
		case R_GNOME: return -3;
		case R_DWARF: return 0;
		case R_HALF_ORC: return 3;
		case R_HALF_TROLL: return 5;
		case R_PHRAINT: return 4;
		case R_DRYAD: return -1;
		default:
			      MSG("Unknown race");
			      return -100;
	}
}

signed char race_melee_bonus(int race) {
	switch (race) {
		case R_HUMAN: return 0;
		case R_HALF_ELF: return 0;
		case R_ELF: return -5;
		case R_HALFLING: return -10;
		case R_GNOME: return -8;
		case R_DWARF: return 15;
		case R_HALF_ORC: return 12;
		case R_HALF_TROLL: return 20;
		case R_PHRAINT: return 3;
		case R_DRYAD: return 0;
		default:
			      MSG("Unknown race");
			      return -100;
	}
}

signed char race_ranged_bonus(int race) {
	switch (race) {
		case R_HUMAN: return 0;
		case R_HALF_ELF: return 5;
		case R_ELF: return 15;
		case R_HALFLING: return 20;
		case R_GNOME: return 12;
		case R_DWARF: return 0;
		case R_HALF_ORC: return -5;
		case R_HALF_TROLL: return -10;
		case R_PHRAINT: return 5;
		case R_DRYAD: return 5;
		default:
			      MSG("Unknown race");
			      return -100;
	}
}

signed char race_save_mod(int race) {
	switch (race) {
		case R_HUMAN: return 0;
		case R_HALF_ELF: return 3;
		case R_ELF: return 6;
		case R_HALFLING: return 18;
		case R_GNOME: return 12;
		case R_DWARF: return 9;
		case R_HALF_ORC: return -3;
		case R_HALF_TROLL: return -9;
		case R_PHRAINT: return -3;
		case R_DRYAD: return 3;
		default:
			      MSG("Unknown race");
			      return -100;
	}
}

signed char race_health_bonus(int race) {
	switch (race) {
		case R_HUMAN: return 10;
		case R_HALF_ELF: return 9;
		case R_ELF: return 8;
		case R_HALFLING: return 6;
		case R_GNOME: return 7;
		case R_DWARF: return 9;
		case R_HALF_ORC: return 10;
		case R_HALF_TROLL: return 12;
		case R_PHRAINT: return 8;
		case R_DRYAD: return 7;
		default:
			      MSG("Unknown race");
			      return -100;
	}
}

signed char race_infravision(int race) {
	switch (race) {
		case R_HUMAN: return 0;
		case R_HALF_ELF: return 0;
		case R_ELF: return 0;
		case R_HALFLING: return 4;
		case R_GNOME: return 3;
		case R_DWARF: return 5;
		case R_HALF_ORC: return 3;
		case R_HALF_TROLL: return 3;
		case R_PHRAINT: return 5;
		case R_DRYAD: return 3;
		default:
			      MSG("Unknown race");
			      return 0;
	}
}

signed char race_swim_speed(int race) {
	switch (race) {
		case R_HUMAN: return 0;
		case R_HALF_ELF: return 1;
		case R_ELF: return 2;
		case R_HALFLING: return -2;
		case R_GNOME: return -1;
		case R_DWARF: return -2;
		case R_HALF_ORC: return 0;
		case R_HALF_TROLL: return 2;
		case R_PHRAINT: return -1;
		case R_DRYAD: return -1;
		default:
			      MSG("Unknown race");
			      return 0;
	}
}

unsigned long race_class_field(int race) {
	switch (race) {
		case R_HUMAN: return 0x3FF;
		case R_HALF_ELF: return 0x3FF;
		case R_ELF: return 0x1DF;
		case R_HALFLING: return 0x2BA;
		case R_GNOME: return 0x04E;
		case R_DWARF: return 0x045;
		case R_HALF_ORC: return 0x20D;
		case R_HALF_TROLL: return 0x005;
		case R_PHRAINT: return 0x39B;
		case R_DRYAD: return 0x2D4;
		default:
			      MSG("Unknown race");
			      return 0;
	}
}
