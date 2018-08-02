#ifndef CLASSES_H
#define CLASSES_H

/**
 * -OK-
 *  class_melee_bonus() - Get melee hit bonus for class
 *  @class Class name, e.g. C_WARRIOR
 */
signed char class_melee_bonus(enum class_t class);

/**
 * -OK-
 *  class_ranged_bonus() - Get ranged hit bonus for class
 *  @class Class name, e.g. C_WARRIOR
 */
signed char class_ranged_bonus(enum class_t class);

/**
 * -OK-
 *  class_uses_magic() - Can the class cast spells of a given type?
 *  @class: Class name, e.g. C_WARRIOR
 *  @magic_type: Type of magic, e.g. M_ARCANE
 */
boolean class_uses_magic(enum class_t class, enum magic_t magic_type);

/**
 * -OK-
 *  class_spell() - Get spell by class and slot
 *  @class: Class name, e.g. C_WARRIOR
 *  @slot: Spell slot, currently in range [0-40[
 */
spell_t *class_spell(enum class_t class, int slot);

#endif /* CLASSES_H */
