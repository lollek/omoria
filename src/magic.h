#ifndef MAGIC_H
#define MAGIC_H

enum magic_t { M_ARCANE, M_DIVINE, M_NATURE, M_SONG, M_CHAKRA };

/**
 * -RAK-
 *  cast() - Cast a magic spell
 *  @magic: Which type of magic is to be cast
 */
void cast(enum magic_t magic_type);

/* Do not use outside magic_* */
void arcane_spell_effects(long effect);
void divine_spell_effects(long effect);
void nature_spell_effects(long effect);
void song_spell_effects(long effect);
void chakra_spell_effects(long effect);

#endif /* MAGIC_H */
