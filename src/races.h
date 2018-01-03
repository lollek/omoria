#ifndef RACES_H
#define RACES_H

/**
 * -OK-
 *  race_name() - What a race is called in game
 *  @race: race enum, e.g. R_HUMAN
 */
char const *race_name(int race);

/**
 * -OK-
 *  race_stats() - Return stats array for race
 *  @race: race enum, e.g. R_HUMAN
 */
signed char const *race_stats(int race);

/**
 * -OK-
 *  race_rand_starting_age() - Generate starting age for player of a given race
 *  @race: race enum, e.g. R_HUMAN
 */
unsigned race_rand_starting_age(int race);

/**
 * -OK-
 *  race_rand_starting_height() - Generate starting height for player
 *  @race: race enum, e.g. R_HUMAN
 *  @male: is the character a male?
 */
unsigned race_rand_starting_height(int race, boolean male);

/**
 * -OK-
 *  race_weight_base() - Get weight base for a race
 *  @race: race enum, e.g. R_HUMAN
 *  @male: is the character a male?
 */
unsigned race_weight_base(int race, boolean male);

/**
 * -OK-
 *  race_weight_modifier() - Get weight modifier for a race
 *  @race: race enum, e.g. R_HUMAN
 *  @male: is the character a male?
 */
unsigned race_weight_modifier(int race, boolean male);

/**
 * -OK-
 *  race_rand_starting_weight() - Generate starting weight for race
 *  @race: race enum, e.g. R_HUMAN
 *  @male: is the character a male?
 */
unsigned race_rand_starting_weight(int race, boolean male);

/**
 * -OK-
 *  race_expfactor() - Experience factor for race
 *  @race: race enum, e.g. R_HUMAN
 */
float race_expfactor(int race);

/**
 * -OK-
 *  race_disarm_mod() - Racial disarm modifier
 *  @race: race enum, e.g. R_HUMAN
 */
signed char race_disarm_mod(int race);

/**
 * -OK-
 *  race_search_mod() - Racial search modifier
 *  @race: race enum, e.g. R_HUMAN
 */
signed char race_search_mod(int race);

/**
 * -OK-
 *  race_stealth_mod() - Racial stealth modifier
 *  @race: race enum, e.g. R_HUMAN
 */
signed char race_stealth_mod(int race);

/**
 * -OK-
 *  race_search_freq() - Racial search frequency
 *  @race: race enum, e.g. R_HUMAN
 */
signed char race_search_freq(int race);

/**
 * -OK-
 *  race_melee_bonus() - Racial melee bonus to hit
 *  @race: race enum, e.g. R_HUMAN
 */
signed char race_melee_bonus(int race);

/**
 * -OK-
 *  race_ranged_bonus() - Racial ranged bonus to hit
 *  @race: race enum, e.g. R_HUMAN
 */
signed char race_ranged_bonus(int race);

/**
 * -OK-
 *  race_save_mod() - Racial save modifier
 *  @race: race enum, e.g. R_HUMAN
 */
signed char race_save_mod(int race);

/**
 * -OK-
 *  race_health_bonus() - Racial bonus to starting health
 *  @race: race enum, e.g. R_HUMAN
 */
signed char race_health_bonus(int race);

/**
 * -OK-
 *  race_infravision() - Racial infravision
 *  @race: race enum, e.g. R_HUMAN
 */
signed char race_infravision(int race);

/**
 * -OK-
 *  race_swim_speed() - Racial swim speed
 *  @race: race enum, e.g. R_HUMAN
 */
signed char race_swim_speed(int race);

/**
 * -OK-
 *  race_class_field() - Field matching available classes for a race
 *  @race: race enum, e.g. R_HUMAN
 *
 *  TODO: Change this into something easier to understand
 */
unsigned long race_class_field(int race);

#endif /* RACES_H */
