#ifndef ROUTINES_H
#define ROUTINES_H
/**/

/* START - FROM RUST */
boolean C_master_update_character(int64_t uid);
boolean C_master_character_exists(int64_t uid);
int64_t C_master_add_character();

boolean C_save_character();
boolean C_load_character();

boolean C_player_knows_spell(int32_t slot);
void C_player_set_knows_spell(int32_t slot, boolean yn);
boolean C_player_uses_magic(enum magic_t magic_type);

signed char C_class_melee_bonus(enum class_t class);
signed char C_class_ranged_bonus(enum class_t class);

uint8_t C_magic_spell_level(int32_t slot);
uint8_t C_magic_spell_mana(int32_t slot);
uint8_t C_magic_spell_failchance(int32_t slot);

/* END - FROM RUST */

/* { CASINO.PAS		} */
extern void enter_casino();

/* { CREATE.PAS		} */
/*	{ Chances the name of the character			-JWT- * } */
extern void change_name();
extern void create_character();
extern void set_gem_values();

/* { CREATURE.PAS		} */
extern void load_monsters();
extern void mon_name();
extern long find_mon(const char *virtual_name);
extern void check_mon_lite(long y, long x);
extern void multiply_monster(long y, long x, long z, boolean slp);
extern void creatures(boolean attack);

/* debug routine */
extern void print_creature(creature_type *c, int c_num, int style);

/* { DEATH.PAS		} */
extern void upon_death();
extern void make_tomb(char dstr[][82]);
extern void replace_name();
extern void write_tomb(char dstr[][82]);
extern void print_dead_character();
extern void top_twenty(long this_many);
extern long total_points();

/* { DESC.PAS		} */
extern void randes();
extern void rantitle(char *title); /* : varying[a] of char); */
extern void magic_init(unsigned long random_seed);
extern void known1(char *object_str);  /* : varying[a] of char); */
extern void known2(char *object_str);  /* : varying[a] of char); */
extern void unquote(char *object_str); /* : varying[a] of char); */
extern void identify(treasure_type *item);
extern void objdes(char *out_val, /*: varying[a] of char; */
		   treas_ptr ptr, /*	: treas_ptr; */
		   boolean pref); /*	: boolean); */

/* { DUNGEON.PAS		} */
extern void move_rec(long y1, long x1, long y2, long x2);
extern void update_stat(stat_set tstat);
extern void change_stat(stat_set tstat, long amount, long factor);
extern void change_speed(long num);
extern void py_bonuses(treasure_type *tobj, long factor);
extern boolean get_panel(long y, long x, boolean forceit);
extern void search(long y, long x, long chance);
extern void area_affect(long dir, long y, long x);
extern void carry(long y, long x);
extern void move_light(long y1, long x1, long y2, long x2);
extern void light_room(long y, long x);
extern void lite_spot(long y, long x);
extern void unlite_spot(long y, long x);
extern boolean pick_dir(long dir);
extern boolean panel_contains(long y, long x);
extern boolean no_light();
extern void change_trap(long y, long x);
extern void kicked_out();
extern void call_guards(char who[82]);
extern void call_wizards();
extern void beg_food();
extern void beg_money();
extern void invite_for_meal();
extern void party();
extern void spend_the_night(char who[82]);
extern void worship();
extern void battle_game(long plus, char kb_str[82]);
extern void brothel_game();
extern void thief_games();
extern long react(long x);
extern void change_rep(long amt);
extern boolean check_store_hours(long st, long sh);
extern void check_store_hours_and_enter(long st, long sh, long store_num);
extern void hit_trap(long *y, long *x);
extern boolean minus_ac(long typ_dam);
extern void corrode_gas(char kb_str[82]);
extern void poison_gas(long dam, char kb_str[82]);
extern void fire_dam(long dam, char kb_str[82]);
extern void acid_dam(long dam, char kb_str[82]);
extern void cold_dam(long dam, char kb_str[82]);
extern void light_dam(long dam, char kb_str[82]);
extern void monster_death(long y, long x, unsigned long flags);
extern long mon_take_hit(long monptr, long dam);
extern long tot_dam(treasure_type *item, long tdam, creature_type *monster);
extern boolean py_attack(long y, long x);
extern boolean find_range(obj_set const item_val, boolean inner,
			  treas_ptr *first, long *count);
extern boolean player_test_hit(long bth, long level, long pth, long ac,
			       boolean was_fired);
extern boolean test_hit(long bth, long level, long pth, long ac);
extern void delete_monster(long i2);
extern void summon_object(long y, long x, long num, long typ);

/**
 * -DMF-
 * get_money_type() - Prompt for what type of money to use
 */
long get_money_type(char prompt[134], boolean *back, boolean no_check);

/**
 * coin_stuff() - Figure out what kind of coin is beign asked about
 * @typ: Initial of coin metal
 * @type_num: ???
 */
boolean coin_stuff(char typ, long *type_num);
extern boolean set_money(char typ, long coin_num);

extern long movement_rate(long cspeed, long mon);
extern void get_player_move_rate();
extern void xp_loss(long amount);
extern boolean twall(long y, long x, long t1, long t2);
extern void dungeon();

/* { FILES.PAS		} */
extern void intro(int argc, char *argv[]);
extern void print_map();
extern void print_objects();
extern void print_monsters();
extern void file_character();
extern boolean read_top_scores(FILE **f1, char *fnam, char list[][134],
			       int max_high, int *n1, char *openerr);
extern boolean write_top_scores(FILE **f1, char list[][134], int max_high);
extern boolean close_top_scores(FILE **f1);
extern char *format_top_score(char out_str[82], char *username, long score,
			      int diffic, char *charname, int level, char *race,
			      char *class);
extern FILE *priv_fopen(char *path, char *mode);
extern void encrypt_file(char fnam[82]);
extern void decrypt_file(char fnam[82]);

/* { GENERATE.PAS		} */
extern void generate_cave();

/* { HELP.PAS		} */
extern void ident_char();
extern void help();
extern void wizard_help();
extern void moria_help(char help_level[82]);
extern void enter_wizard_mode(boolean ask_for_pass);

/* { INVEN.PAS		} */
extern long change_all_ok_stats(boolean nok, boolean nin);
extern char cur_char1();
extern char cur_char2();
extern char cur_insure();
extern boolean inven_command(char command, treas_ptr *item_ptr, char prompt[82]);
extern void delete_inven_item(treas_ptr ptr);
extern void inven_destroy(treas_ptr item_ptr);
extern void inven_drop(treas_ptr item_ptr, long y, long x, boolean mon);
extern long inven_damage(obj_set typ, long perc);
extern boolean inven_check_weight();
extern boolean inven_check_num();
extern treas_ptr add_inven_item(treasure_type item);
extern treas_ptr inven_carry();
extern boolean get_item(treas_ptr *com_ptr, char pmt[82], boolean *redraw,
			long count, char *choice, boolean mon,
			boolean no_wait); /*	: boolean := false); */
treas_ptr ic__remove(long item_val, boolean show_message);

/**
 * -OK-
 *  inv__equip_pos_string() - Returns a string describing equipment
 *  @out_val: Where the text will be put
 *  @equip_pos: Which equipment position to describe
 *  @counter: Counter position, will be a), b), c), etc
 */
void inv__equip_pos_string(char out_val[82], long equip_pos, long counter);

/* { IO.PAS		} */
extern void init_priv_switch();
extern void priv_switch(long switch_val);
extern void no_controly();
extern void controly();
extern void exit_game() __attribute__((noreturn));
/* extern void inkey(char *getchar); */
extern char inkey();
extern void msg_record(char message[82], boolean save);
extern void inkey_delay(char *getchar, long delay);
extern void flush();
extern void inkey_flush(char *x);
/* use erase_line */
extern void Erase_Line(long row, long col);
extern void clear_rc(long row, long col);
/* use print */
extern void Print(chtype const ch, int row, int col);
extern void print_str(char const *str_buff, int row, int col);
extern void print_chstr(chtype const *str_buff, int row, int col);

extern void put_buffer(char const *str_buff, int row, int col);
extern void put_buffer_(char const *str_buff, int row, int col);

/* use prt */
extern void prt(char const *str_buff, int row, int col);
extern void prt_(char const *str_buff, int row, int col);
extern void prt2(char *str_buff1, /*	: varying[a] of char; */
		 char *str_buff2, int row, int col);

extern boolean msg_print(char *str_buff); /* : varying[a] of char); */
extern boolean get_com(char const *prompt, char *command);

/**
 *  get_yes_no() - Ask a yes/no question
 *  @prompt: Question to write to player
 *
 *  Ask a yes/no question to the player, which adds a question to the end of the
 *  prompt. Will not return until player pressed 'y' or 'n'.
 */
extern boolean get_yes_no(char const *prompt);

/* use get_string */
extern boolean Get_String(char *in_str, /* : varying[a] of char; */
			  int row, int column, int slen);
extern long get_hex_value(long row, long col, long slen);
extern void print_hex_value(long num, long row, long col);
extern void pause_game(long prt_line);
/* use pause_exit */
extern void get_paths();

/* { PLAYER.PAS		} */
extern void find_off();
extern void search_off();
extern void search_on();
extern void rest_off();
extern void take_hit(long damage, char hit_from[82]);
extern void regenhp(float percent);
extern void regenmana(float percent);

/* { QUEST.PAS		} */
extern void enter_fortress();

/* { encrypt.c } */
extern void encrypt_init(encrypt_state *state, unsigned char key[],
			 boolean doit);
extern void encrypt_write(FILE *f1, encrypt_state *state, char line[1026]);
extern void encrypt_flush(FILE *f1, encrypt_state *state);
extern void read_decrypt(FILE *f1, encrypt_state *state, char line[1026],
			 boolean *got_eof);

/* { SCREEN.PAS		} */
#include "screen.h"

/* { STORE.PAS		} */
extern void prt_comment1();
extern void prt_comment2(long offer, long asking, long final);
extern void prt_comment3(long offer, long asking, long final);
extern void prt_comment4();
extern void prt_comment5();
extern void prt_comment6();
extern void display_commands();
extern void haggle_commands(long typ);
extern void display_inventory(long store_num, long start);
extern void display_cost(long store_num, long pos);
extern void store_prt_gold();
extern void display_store(long store_num, long cur_top);
extern boolean get_store_item(long *com_val, char pmt[82], long i1, long i2);
extern void shut_store(long store_num);
extern boolean increase_insults(long store_num);
extern void decrease_insults(long store_num);
extern boolean haggle_insults(long store_num);
extern long receive_offer(long store_num, char comment[82], long *new_offer,
			  long last_offer, long factor);
extern long purchase_haggle(long store_num, long *price, treasure_type *item,
			    boolean blitz);
extern long sell_haggle(long store_num, long *price, treasure_type *item,
			boolean blitz);
extern void spend_time(long days_spent, char place[82], boolean whole_days);
extern boolean store_purchase(long store_num, long *cur_top, boolean blitz);
extern boolean store_sell(long store_num, long cur_top, boolean blitz);
extern void enter_store(long store_num);
extern long item_value(treasure_type *item);
extern long sell_price(long snum, long *max_sell, long *min_sell,
		       treasure_type *item);
extern boolean store_check_num(long store_num);
extern void store_carry(long store_num, long *ipos);
extern void store_destroy(long store_num, long item_val, boolean one_of);
extern void store_init();
extern void bank_init();
extern void store_create(long store_num);
extern void store_maint();

/* { TERMDEF.PAS		} */
extern void termdef();

/* { WIZARD.PAS		}*/
extern void wizard_command(void);
extern void game_version();
extern void bpswd();
extern boolean check_pswd(char passw[134], boolean present);
extern void wizard_light();
extern void monster_summon_by_name(long y, long x, char name[28], boolean present,
				   boolean sleepy);
extern boolean summon_item(long y, long x, char name1[70], char name2[70], long count,
			   boolean present);
extern void change_character();
extern void edit_score_file();
extern void wizard_create();

extern void py_bonuses(treasure_type *tobj, long factor);
extern boolean delete_object(long y, long x);
extern char *center(char str[134], long len, char result[134]); /* was func */
extern void eat();
extern void discipline();
extern void move_char(long dir);
extern void quaff();
extern void pray();
extern void play();
extern void sing();
extern void read_scroll();
extern void use_staff();
extern boolean mon_resists(unsigned char a_cptr);
extern boolean mon_save(long a_cptr, long bonus, long spell_class);
extern boolean sleep_monsters1(long y, long x);
extern boolean detect_item(long typ);
extern boolean detect_trap();
extern boolean detect_sdoor();
extern boolean light_area(long y, long x);
extern boolean unlight_area(long y, long x);
extern boolean map_area();
extern boolean ident_spell();
extern boolean aggravate_monster(long dis_affect);
extern boolean trap_creation();
extern boolean door_creation();
extern boolean td_destroy();
extern boolean light_line(long dir, long y, long x, long power);
extern boolean starlite(long y, long x);
extern boolean disarm_all(long dir, long y, long x);
extern boolean detect_curse();
extern void get_flags(long typ, long *weapon_type, long *harm_type,
		      obj_set **destroy);
extern boolean detect_magic();
extern boolean purify();
extern boolean lore_spell();
extern boolean fire_bolt(long typ, long dir, long y, long x, long dam,
			 char bolt_typ[28]);
extern boolean fire_ball(long typ, long dir, long y, long x, long dam_hp,
			 char descrip[28]);
extern boolean creeping_doom(long dir, long y, long x, long dam_hp, long range,
			     char ddesc[28]);
extern boolean fire_line(long typ, long dir, long y, long x, long dam_hp,
			 char descrip[28]);
extern boolean breath(long typ, long y, long x, long dam_hp, char ddesc[82]);
extern boolean recharge(long num);
extern boolean zap_monster(long dir, long y, long x, long aux, long zaptype);
extern boolean wall_to_mud(long dir, long y, long x);
extern boolean td_destroy2(long dir, long y, long x);
extern boolean poly_monster(long dir, long y, long x);
extern boolean build_wall(long dir, long y, long x);
extern boolean clone_monster(long dir, long y, long x);
extern boolean teleport_away(long monptr, long dis);
extern boolean teleport_to(long ny, long nx);
extern boolean teleport_monster(long dir, long y, long x);
extern boolean mass_genocide();
extern boolean genocide();
extern boolean mass_poly();
extern boolean detect_creatures(long typ);
extern boolean hp_player(long num, char kind[82]);
extern boolean cure_me(long *what_flag);
extern boolean earthquake();
extern boolean protect_evil();
extern boolean create_food(long t0, long t1, long t2, long t3, long t4);
extern boolean zap_area(long cflag, long dmge, long typ);
extern boolean warding_glyph();
extern void lower_stat(stat_set tstat, char msg1[82]);
extern boolean lose_stat(stat_set tstat, char msg1[82], char msg2[82]);
extern boolean restore_stat(stat_set tstat, char msg1[82]);
extern boolean gain_stat(stat_set tstat, char msg1[82]);
extern void lose_exp(long amount);
extern boolean slow_poison();
extern boolean bless(long amount);
extern boolean detect_inv2(long amount);
extern boolean destroy_area(long y, long x);
extern boolean enchant(short *pluses);
extern boolean remove_curse();
extern boolean restore_level();
extern void use();
extern void aim_wand();

/* Generates a random integer x where 1<=X<=MAXVAL	-RAK-	*/
extern long randint(long maxval);
extern long rand_rep(long num, long die);
extern long maxmin(long x, long y, long z);
extern long minmax(long x, long y, long z);
extern long bit_pos(unsigned long *test);
extern long bit_pos64(unsigned long *high, unsigned long *low);
extern long distance(long y1, long x1, long y2, long x2);
extern void insert_str(char *object_str,	/* : varying[a] of char; */
		       char const *mtc_str,     /*	: varying[b] of char; */
		       char const *insert_str); /* : varying[c] of char); */
extern void shell_out();
extern long users();
extern long max_users();
extern boolean get_uw_id();
extern boolean net_trade();
extern void get_account(char *account);
extern void sys_gettim(quad_type *bin_time);
extern void sys_numtim(time_type *weird_num, quad_type *bin_time);
extern void sys_asctim(
    unsigned short *timlin, /*: [reference] unsigned short := %immed 0; */
    char *timbuf,      /*: [class_s] packed array [$l1..$u1:long] of char; */
    quad_type *timadr, /*: [reference] quad_type := %immed 0; */
    long *cvtflg);     /*: [reference] long := %immed 0); */

/* MISC */
const char *imoria_version();
const char *omoria_version();
extern void init_m_level();
extern void init_t_level();
extern void price_adjust();
extern void item_weight_adjust();
extern void set_difficulty(long diff);
extern long day_num();
extern long hour_num();
extern boolean check_kickout();
extern boolean check_time();
extern long randnor(long mean, long stand);
extern boolean in_bounds(long y, long x);
extern long next_to4(long y, long x, obj_set group_set);
extern long next_to8(long y, long x, obj_set group_set);
extern long rotate_dir(long dir, long rot);
extern long get_hexdecant(long dy, long dx);
extern void tlink();
extern void mlink();
extern long damroll(char const *dice);
extern boolean los(long y1, long x1, long y2, long x2);
extern chtype get_loc_symbol(long y, long x);
extern chtype loc_symbol(long y, long x);
extern boolean test_light(long y, long x);
extern void validate_monsters();
extern void compact_monsters();
extern void popm(long *x);
extern void pushm(long x);
extern long max_hp(char const *hp_str);
extern void place_monster(long y, long x, long z, boolean slp);
extern void place_win_monster();
extern void alloc_land_monster(obj_set alloc_set, long num, long dis,
			       boolean slp, boolean water);

extern boolean summon_land_monster(long *y, long *x, boolean slp);
extern boolean summon_water_monster(long *y, long *x, boolean slp);
extern boolean summon_undead(long *y, long *x);
extern boolean summon_demon(long *y, long *x);
extern boolean summon_breed(long *y, long *x);
extern void petrify(long amt);
extern void compact_objects();
extern void popt(long *x);
extern void pusht(long x);
extern void sort_objects();
extern void magic_treasure(long x, long level, boolean forceit);
extern void place_trap(long y, long x, long typ, long subval);
extern void place_rubble(long y, long x);
extern void place_open_door(long y, long x);
extern void place_broken_door(long y, long x);
extern void place_closed_door(long y, long x);
extern void place_locked_door(long y, long x);
extern void place_stuck_door(long y, long x);
extern void place_secret_door(long y, long x);
extern void place_door(long y, long x);
extern void place_a_staircase(long y, long x, long typ);
extern void place_stairs(long typ, long num, long walls);
extern void place_gold(long y, long x);
extern long get_obj_num(long level, long tries);
extern void place_object(long y, long x);
extern void alloc_object(obj_set alloc_set, long typ, long num);
extern void random_object(long y, long x, long num);
extern void cnv_stat(unsigned char stat, stat_s_type out_val);
extern long spell_adj(stat_set attr);
extern long bard_adj();
extern long druid_adj();
extern long monk_adj();
extern float chr_adj();
extern long con_adj();
extern char *place_string(long num, char result[134]);
extern char *day_of_week_string(long day, unsigned wid, char result[134]);
extern char *month_string(long mon, char result[134]);
extern char *time_string(long hour, long sec, char result[134]);
extern void time_diff(game_time_type a, game_time_type b, game_time_type *c);
extern void add_days(game_time_type *ti, long d);
extern char *full_date_string(game_time_type time, char result[134]);
extern void adv_time(boolean flag);
extern char *play_time(time_type *t, char result[134]);
extern void add_play_time(time_type *res, time_type add);
extern time_type *convert_seconds_to_time(time_t seconds, time_type *tim);
extern time_t convert_time_to_seconds(time_type *tim);
extern char *show_char_age(char result[134]);
extern char *show_current_time(char result[134]);
extern char *show_play_time(char result[134]);
extern char *bag_descrip(treas_ptr bag, char result[134]);
extern uint8_t squish_stat(int32_t stat);
extern unsigned char in_statp(unsigned char stat);
extern unsigned char de_statp(unsigned char stat);
extern long tohit_adj();
extern long toac_adj();
extern long todis_adj();
extern long todam_adj();
extern char *likert(long x, long y, char *result);
extern unsigned char characters_sex();
extern unsigned short max_allowable_weight();
extern unsigned short min_allowable_weight();
extern long weight_limit();
extern treas_ptr money_carry();
extern char *cost_str(long amt, char result[134]);
extern void total_cash();

/*{ recomputes cash totals for player and bank }*/
extern void reset_total_cash();
extern void add_money(long amount);
extern void subtract_money(long amount, boolean make_change);
extern boolean send_page(long to_bank);
extern void spell_chance(spl_rec *spell);
extern void print_new_spells(spl_type spell, long num, boolean *redraw);
extern boolean get_spell(spl_type spell, long num, long *sn, long *sc,
			 char prompt[82], boolean *redraw);
extern long num_new_spells(long smarts);
extern boolean learn_spell(boolean *redraw);
extern boolean learn_prayer();
extern boolean learn_song(boolean *redraw);
extern boolean learn_druid();
extern void gain_mana(long amount);
extern void gain_level();
extern void insert_num(char *object_str, /*	: varying[a] of char; */
		       char *mtc_str,    /*	: varying[b] of char; */
		       long number, boolean show_sign);
extern long attack_blows(long weight, long *wtohit);
extern long critical_blow(long weight, long plus, boolean cs_sharp,
			  boolean is_fired);
extern boolean move_dir(long dir, long *y, long *x); /* was move */
extern boolean player_saves(long adjust);
extern boolean player_spell_saves();
extern void char_inven_init();
extern void find_monster_name(char m_name[82], const long ptr, boolean begin_sentence);
extern void check_kickout_time(long num, long check);

/* { BANK.INC		} */
extern void enter_bank();

/* { INSURANCE.INC	} */
extern void buy_insurance();

/* { river.c } */
extern void all_the_river_stuff();

/* { rooms.c } */
extern void gc__build_room(long yval, long xval);
extern void gc__build_type1(long yval, long xval);
extern void gc__build_type2(long yval, long xval);
extern void gc__build_type3(long yval, long xval);

/* { port.c		} */
extern void memory_error(int blocksize, char *message)
    __attribute__((noreturn));
extern void *safe_malloc(int size, char *message);
extern void dispose(void *ptr, int size, char *message);
extern char *chomp(char *input_line);
extern long min3(long i1, long i2, long i3);

extern void ignore_signals();
extern void restore_signals();
extern void default_signals();

/* { term.c		} */
extern void init_curses();

/* use put_buffer */
extern void put_buffer_(char const *out_str, int32_t row, int32_t col);
extern void put_buffer_attr(const char *out_str, /*	: varying [a] of char; */
			    long row, long col, int attrs);
extern void put_qio();
/* use clear_from */
extern void Clear_From(int row);
extern void move_cursor_relative(int row, int col);
/* use pause_line */
extern void Pause_Line(int prt_line);
extern void move_cursor(int row, int col);
extern unsigned sleep();
extern void screen_map();
extern void show_location();

/* { unix.c		} */
extern void user_name(char *buf);
extern int check_input(int microsec);

#endif /* ROUTINES_H */
