#ifndef CLASSES_H
#define CLASSES_H

/**
 * -OK-
 *  class_title() - Get title for class
 *  @class Class name, e.g. C_WARRIOR
 */
char const *class_title(int class);

/**
 * -OK-
 *  class_expfactor() - Get experience factor for class
 *  @class Class name, e.g. C_WARRIOR
 *
 *  Experience factor adds how much experience a player needs to level up
 */
float class_expfactor(int class);

/**
 * -OK-
 *  class_extra_health() - Get starting health bonus for class
 *  @class Class name, e.g. C_WARRIOR
 */
signed char class_extra_health(int class);

/**
 * -OK-
 *  class_disarm_mod() - Get disarm modifier for class
 *  @class Class name, e.g. C_WARRIOR
 */
signed char class_disarm_mod(int class);

/**
 * -OK-
 *  class_search_mod() - Get search modifier for class
 *  @class Class name, e.g. C_WARRIOR
 */
signed char class_search_mod(int class);

/**
 * -OK-
 *  class_stealth_mod() - Get stealth modifier for class
 *  @class Class name, e.g. C_WARRIOR
 */
signed char class_stealth_mod(int class);

/**
 * -OK-
 *  class_search_freq() - Get search frequency for class
 *  @class Class name, e.g. C_WARRIOR
 */
signed char class_search_freq(int class);

/**
 * -OK-
 *  class_melee_bonus() - Get melee hit bonus for class
 *  @class Class name, e.g. C_WARRIOR
 */
signed char class_melee_bonus(int class);

/**
 * -OK-
 *  class_ranged_bonus() - Get ranged hit bonus for class
 *  @class Class name, e.g. C_WARRIOR
 */
signed char class_ranged_bonus(int class);

/**
 * -OK-
 *  class_save_mod() - Get save modifier for class
 *  @class Class name, e.g. C_WARRIOR
 */
signed char class_save_mod(int class);

/**
 * -OK-
 *  class_stats() - Get stats block for class
 *  @class Class name, e.g. C_WARRIOR
 *
 *  returns an array containing all 6 stats
 */
signed char const *class_stats(int class);

/**
 * -OK-
 *  class_uses_priest_magic() - Can the class cast priest spells?
 *  @class Class name, e.g. C_WARRIOR
 */
boolean class_uses_priest_magic(int class);

/**
 * -OK-
 *  class_uses_arcane_magic() - Can the class cast arcane spells?
 *  @class Class name, e.g. C_WARRIOR
 */
boolean class_uses_arcane_magic(int class);

/**
 * -OK-
 *  class_uses_druid_magic() - Can the class cast druid spells?
 *  @class Class name, e.g. C_WARRIOR
 */
boolean class_uses_druid_magic(int class);

/**
 * -OK-
 *  class_uses_bard_magic() - Can the class cast bard spells?
 *  @class Class name, e.g. C_WARRIOR
 */
boolean class_uses_bard_magic(int class);

/**
 * -OK-
 *  class_uses_monk_discipline() - Can the class use monk disciplines?
 *  @class Class name, e.g. C_WARRIOR
 */
boolean class_uses_monk_discipline(int class);

/**
 * -OK-
 *  class_magic_resist() - Get class magic resist modifier
 *  @class Class name, e.g. C_WARRIOR
 *
 *  This seems to affect the class' ability to pierce magic resistance?
 */
signed char class_magic_resist(int class);

#endif /* CLASSES_H */
