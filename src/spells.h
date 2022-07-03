#ifndef SPELLS_H
#define SPELLS_H

#include "types.h"

boolean mon_resists(unsigned char a_cptr);
boolean mon_save(long a_cptr, long bonus, enum spell_class_t spell_class);
boolean sleep_monsters1(long y, long x);
boolean detect_item(long typ);
boolean detect_trap();
boolean detect_sdoor();
boolean light_area(long y, long x);
boolean unlight_area(long y, long x);
boolean map_area();
boolean ident_spell();
boolean aggravate_monster(long dis_affect);
boolean trap_creation();
boolean door_creation();
boolean td_destroy();
boolean light_line(long dir, long y, long x, long power);
boolean starlite(long y, long x);
boolean disarm_all(long dir, long y, long x);
boolean detect_curse();
boolean detect_magic();
boolean lore_spell();
boolean fire_bolt(enum spell_effect_t typ, long dir, long y, long x, long dam,
                  char bolt_typ[28]);
boolean fire_ball(enum spell_effect_t typ, long dir, long y, long x,
                  long dam_hp, char descrip[28]);
boolean creeping_doom(long dir, long y, long x, long dam_hp, long range,
                      char ddesc[28]);
boolean fire_line(enum spell_effect_t typ, long dir, long y, long x,
                  long dam_hp, char descrip[28]);
boolean breath(enum spell_effect_t typ, long y, long x, long dam_hp,
               char ddesc[82]);
boolean recharge(long num);
boolean zap_monster(long dir, long y, long x, long aux, long zaptype);
boolean wall_to_mud(long dir, long y, long x);
boolean td_destroy2(long dir, long y, long x);
boolean poly_monster(long dir, long y, long x);
boolean build_wall(long dir, long y, long x);
boolean clone_monster(long dir, long y, long x);
boolean teleport_away(long monptr, long dis);
boolean teleport_to(long ny, long nx);
boolean teleport_monster(long dir, long y, long x);
boolean mass_genocide();
boolean genocide();
boolean mass_poly();
boolean detect_creatures(enum spell_effect_t typ);
boolean hp_player(long num, char kind[82]);
boolean cure_me(long *what_flag);
boolean earthquake();
boolean protect_evil();
boolean create_food(long t0, long t1, long t2, long t3, long t4);
boolean zap_area(long cflag, long dmge, long typ);
boolean warding_glyph();
void lower_stat(enum stat_t tstat, char msg1[82]);
boolean lose_stat(enum stat_t tstat, char msg1[82], char msg2[82]);
boolean restore_stat(enum stat_t tstat, char msg1[82]);
boolean gain_stat(enum stat_t tstat, char msg1[82]);
void lose_exp(long amount);
boolean slow_poison();
boolean bless(long amount);
boolean detect_inv2(long amount);
boolean destroy_area(long y, long x);
boolean enchant(short *pluses);
boolean remove_curse();
boolean restore_level();

#endif // SPELLS_H