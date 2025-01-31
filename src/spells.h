#pragma once

#include "types.h"

/*{ Teleport the player to a new location                 -RAK-   }*/
void teleport(long dis);
bool create_water(long y, long x);
bool destroy_water(long y, long x);
bool item_petrify(void);
bool move_to_creature(long dir, long *y, long *x);
bool bolt_to_creature(long dir, long *y, long *x, long *dist, long max_dist,
                         bool visable);
bool explode(enum spell_effect_t typ, long y, long x, long dam_hp,
                const char *descrip);
bool do_stun(unsigned char a_cptr, long save_bonus, long time);
bool mon_resists(unsigned char a_cptr);
bool mon_save(long a_cptr, long bonus, enum spell_class_t spell_class);
bool sleep_monsters1(long y, long x);
bool detect_item(long typ);
bool detect_trap(void);
bool detect_sdoor(void);
bool light_area(long y, long x);
bool unlight_area(long y, long x);
bool map_area(void);
bool ident_spell(void);
bool aggravate_monster(long dis_affect);
bool trap_creation(void);
bool door_creation(void);
bool td_destroy(void);
bool light_line(long dir, long y, long x, long power);
bool starlite(long y, long x);
bool disarm_all(long dir, long y, long x);
bool detect_curse(void);
bool detect_magic(void);
bool lore_spell(void);
bool fire_bolt(enum spell_effect_t typ, long dir, long y, long x, long dam,
                  char bolt_typ[28]);
bool fire_ball(enum spell_effect_t typ, long dir, long y, long x,
                  long dam_hp, char descrip[28]);
bool creeping_doom(long dir, long y, long x, long dam_hp, long range,
                      char ddesc[28]);
bool fire_line(enum spell_effect_t typ, long dir, long y, long x,
                  long dam_hp, char descrip[28]);
bool breath(enum spell_effect_t typ, long y, long x, long dam_hp,
               char ddesc[82]);
bool recharge(long num);
bool zap_monster(long dir, long y, long x, long aux, long zaptype);
bool wall_to_mud(long dir, long y, long x);
bool td_destroy2(long dir, long y, long x);
bool poly_monster(long dir, long y, long x);
bool build_wall(long dir, long y, long x);
bool clone_monster(long dir, long y, long x);
bool teleport_away(long monptr, long dis);
bool teleport_to(long ny, long nx);
bool teleport_monster(long dir, long y, long x);
bool mass_genocide(void);
bool genocide(void);
bool mass_poly(void);
bool detect_creatures(enum spell_effect_t typ);
bool hp_player(long num, char kind[82]);
/**
 * @what_flag:  confused, blind, poisoned, hoarse, afraid, image
 *
 * Return if the player had the effect
 */
bool cure_player_status_effect(int64_t *what_flag);
bool earthquake(void);
bool protect_evil(void);
bool create_food(long t0, long t1, long t2, long t3, long t4);
bool zap_area(long cflag, long dmge, long typ);
bool warding_glyph(void);
void lower_stat(enum stat_t tstat, char msg1[82]);
bool lose_stat(enum stat_t tstat, char msg1[82], char msg2[82]);
bool restore_stat(enum stat_t tstat, char msg1[82]);
bool gain_stat(enum stat_t tstat, char msg1[82]);
void lose_exp(long amount);
bool slow_poison(void);
bool bless(long amount);
bool detect_inv2(long amount);
bool destroy_area(long y, long x);
bool enchant(short *pluses);
bool remove_curse(void);
bool restore_player_drained_levels(void);
