#ifndef CLASSES_H
#define CLASSES_H

/**
 * -OK-
 *  class_title_get() - Get title for class
 *  @class Class name, e.g. C_WARRIOR
 */
char const *class_title_get(int class);

/**
 * -OK-
 *  class_expfactor_get() - Get experience factor for class
 *  @class Class name, e.g. C_WARRIOR
 *
 *  Experience factor adds how much experience a player needs to level up
 */
float class_expfactor_get(int class);

/**
 * -OK-
 *  class_extra_health_get() - Get starting health bonus for class
 *  @class Class name, e.g. C_WARRIOR
 */
signed char class_extra_health_get(int class);

/**
 * -OK-
 *  class_disarm_mod_get() - Get disarm modifier for class
 *  @class Class name, e.g. C_WARRIOR
 */
signed char class_disarm_mod_get(int class);

/**
 * -OK-
 *  class_search_mod_get() - Get search modifier for class
 *  @class Class name, e.g. C_WARRIOR
 */
signed char class_search_mod_get(int class);

/**
 * -OK-
 *  class_stealth_mod_get() - Get stealth modifier for class
 *  @class Class name, e.g. C_WARRIOR
 */
signed char class_stealth_mod_get(int class);

/**
 * -OK-
 *  class_search_freq_get() - Get search frequency for class
 *  @class Class name, e.g. C_WARRIOR
 */
signed char class_search_freq_get(int class);

/**
 * -OK-
 *  class_melee_bonus_get() - Get melee hit bonus for class
 *  @class Class name, e.g. C_WARRIOR
 */
signed char class_melee_bonus_get(int class);

/**
 * -OK-
 *  class_ranged_bonus_get() - Get ranged hit bonus for class
 *  @class Class name, e.g. C_WARRIOR
 */
signed char class_ranged_bonus_get(int class);

/**
 * -OK-
 *  class_save_mod_get() - Get save modifier for class
 *  @class Class name, e.g. C_WARRIOR
 */
signed char class_save_mod_get(int class);

/**
 * -OK-
 *  class_stats_get() - Get stats block for class
 *  @class Class name, e.g. C_WARRIOR
 *
 *  returns an array containing all 6 stats
 */
signed char const *class_stats_get(int class);

/**
 * -OK-
 *  class_priest_spellcaster_get() - Can the class cast priest spells?
 *  @class Class name, e.g. C_WARRIOR
 */
boolean class_priest_spellcaster_get(int class);

/**
 * -OK-
 *  class_arcane_spellcaster_get() - Can the class cast arcane spells?
 *  @class Class name, e.g. C_WARRIOR
 */
boolean class_arcane_spellcaster_get(int class);

/**
 * -OK-
 *  class_druid_spellcaster_get() - Can the class cast druid spells?
 *  @class Class name, e.g. C_WARRIOR
 */
boolean class_druid_spellcaster_get(int class);

/**
 * -OK-
 *  class_bard_spellcaster_get() - Can the class cast bard spells?
 *  @class Class name, e.g. C_WARRIOR
 */
boolean class_bard_spellcaster_get(int class);

/**
 * -OK-
 *  class_monk_discipline_get() - Can the class use monk disciplines?
 *  @class Class name, e.g. C_WARRIOR
 */
boolean class_monk_discipline_get(int class);

/**
 * -OK-
 *  class_magic_resist_get() - Get class magic resist modifier
 *  @class Class name, e.g. C_WARRIOR
 *
 *  This seems to affect the class' ability to pierce magic resistance?
 */
signed char class_magic_resist_get(int class);

#endif /* CLASSES_H */
