#ifndef RACES_H
#define RACES_H

enum race_t {
	R_HUMAN = 0,
	R_HALF_ELF = 1,
	R_ELF = 2,
	R_HALFLING = 3,
	R_GNOME = 4,
	R_DWARF = 5,
	R_HALF_ORC = 6,
	R_HALF_TROLL = 7,
	R_PHRAINT = 8,
	R_DRYAD = 9
};

/**
 * -OK-
 *  race_rand_starting_age() - Generate starting age for player of a given race
 *  @race: race enum, e.g. R_HUMAN
 */
unsigned race_rand_starting_age(enum race_t race);

/**
 * -OK-
 *  race_rand_starting_height() - Generate starting height for player
 *  @race: race enum, e.g. R_HUMAN
 *  @male: is the character a male?
 */
unsigned race_rand_starting_height(enum race_t race, boolean male);

/**
 * -OK-
 *  race_weight_base() - Get weight base for a race
 *  @race: race enum, e.g. R_HUMAN
 *  @male: is the character a male?
 */
unsigned race_weight_base(enum race_t race, boolean male);

/**
 * -OK-
 *  race_weight_modifier() - Get weight modifier for a race
 *  @race: race enum, e.g. R_HUMAN
 *  @male: is the character a male?
 */
unsigned race_weight_modifier(enum race_t race, boolean male);

/**
 * -OK-
 *  race_rand_starting_weight() - Generate starting weight for race
 *  @race: race enum, e.g. R_HUMAN
 *  @male: is the character a male?
 */
unsigned race_rand_starting_weight(enum race_t race, boolean male);

/**
 * -OK-
 *  race_expfactor() - Experience factor for race
 *  @race: race enum, e.g. R_HUMAN
 */
float race_expfactor(enum race_t race);

/**
 * -OK-
 *  race_disarm_mod() - Racial disarm modifier
 *  @race: race enum, e.g. R_HUMAN
 */
signed char race_disarm_mod(enum race_t race);

/**
 * -OK-
 *  race_search_mod() - Racial search modifier
 *  @race: race enum, e.g. R_HUMAN
 */
signed char race_search_mod(enum race_t race);

/**
 * -OK-
 *  race_melee_bonus() - Racial melee bonus to hit
 *  @race: race enum, e.g. R_HUMAN
 */
signed char race_melee_bonus(enum race_t race);

/**
 * -OK-
 *  race_save_mod() - Racial save modifier
 *  @race: race enum, e.g. R_HUMAN
 */
signed char race_save_mod(enum race_t race);

/**
 * -OK-
 *  race_health_bonus() - Racial bonus to starting health
 *  @race: race enum, e.g. R_HUMAN
 */
signed char race_health_bonus(enum race_t race);

/**
 * -OK-
 *  race_infravision() - Racial infravision
 *  @race: race enum, e.g. R_HUMAN
 */
signed char race_infravision(enum race_t race);

/**
 * -OK-
 *  race_swim_speed() - Racial swim speed
 *  @race: race enum, e.g. R_HUMAN
 */
signed char race_swim_speed(enum race_t race);

/**
 * -OK-
 *  race_class_field() - Field matching available classes for a race
 *  @race: race enum, e.g. R_HUMAN
 *
 *  TODO: Change this into something easier to understand
 */
unsigned long race_class_field(enum race_t race);

#endif /* RACES_H */
