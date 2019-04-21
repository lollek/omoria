#include "imoria.h"
#include "dungeon.h"

/* P_STATS */
uint8_t player_stats_perm[] = {0, 0, 0, 0, 0, 0};
uint8_t player_stats_curr[] = {0, 0, 0, 0, 0, 0};
int8_t  player_stats_mod[] = {0, 0, 0, 0, 0, 0};
uint8_t player_stats_lost[] = {0, 0, 0, 0, 0, 0};

/* P_MISC */
int64_t player_xtr_wgt = 0;
int64_t player_account = 0;
money_type player_money = {0, 0, 0, 0, 0, 0, 0};
game_time_type player_birth = {0, 0, 0, 0, 0};
game_time_type player_cur_age = {0, 0, 0, 0, 0};
time_type player_play_tm = {0, 0, 0, 0, 0, 0, 0};
char player_name[82] = "";
char player_race[82] = "";
char player_sex[82] = "";
char player_title[82] = "";
char player_tclass[82] = "";
int64_t player_max_exp = 0;
int64_t player_exp = 0;
int64_t player_rep = 0;
int64_t player_deaths = 0;
int64_t player_premium = 0;
uint16_t player_age = 0;
uint16_t player_ht = 0;
uint16_t player_wt = 0;
uint16_t player_lev = 0;
uint16_t player_max_lev = 0;
int16_t player_srh = 0;
int16_t player_fos = 0;
int16_t player_bth = 0;
int16_t player_bthb = 0;
int16_t player_mana = 0;
int16_t player_mhp = 0;
int16_t player_ptohit = 0;
int16_t player_ptodam = 0;
int16_t player_pac = 0;
int16_t player_ptoac = 0;
int16_t player_dis_th = 0;
int16_t player_dis_td = 0;
int16_t player_dis_ac = 0;
int16_t player_dis_tac = 0;
int16_t player_disarm = 0;
int16_t player_save = 0;
int16_t player_sc = 0;
enum class_t player_pclass = 0;
uint8_t player_prace = 0;
uint8_t player_hitdie = 0;
uint8_t player_stl = 0;
float player_expfact = 0;
float player_cmana = 0;
float player_chp = 0;
char player_history[][82] = {"", "", "", "", ""};
boolean player_cheated = false;
int64_t  player_mr = 0;
uint8_t player_quests = 0;
uint16_t player_cur_quest = 0;
time_t player_creation_time = 0;
int64_t player_save_count = 0;
int64_t player_claim_check = 0;
int64_t player_uid = 0;

void search_off()
{
	search_flag = false;
	find_flag = false;
	move_char(5);
	change_speed(-1);
	player_flags.status &= ~IS_SEARCHING;
	prt_search();
}

void search_on()
{
	/*{ Search Mode enhancement                               -RAK-   }*/

	search_flag = true;
	change_speed(+1);
	player_flags.status |= IS_SEARCHING;
	prt_search();
	/* with player_flags do; */
}

void rest_off()
{
	player_flags.rest = 0;
	player_flags.status &= ~IS_RESTING;
	player_flags.resting_till_full = false;
	if (msg_flag) {
		erase_line(1, 1);
	}
	prt_rest();
}

void regenhp(float percent)
{
	/*{ Regenerate hit points		-RAK-	}*/

	player_chp += player_mhp * percent + PLAYER_REGEN_HPBASE;
}

void regenmana(float percent)
{
	/*{ Regenerate mana points		-RAK-	}*/

	player_cmana += player_mana * percent + PLAYER_REGEN_MNBASE;
}

void take_hit(long damage, char hit_from[82])
{
	/*{ Decreases players hit points and sets death flag if neccessary}*/

	ENTER(("take_hit", "%d, %s", damage, hit_from));

	if (player_flags.invuln > 0) {
		damage = 0;
	}

	player_chp -= damage;

	if (search_flag) {
		search_off();
	}

	if (player_flags.rest > 0) {
		rest_off();
	}

	flush();

	if (player_chp <= -1) {
		if (!death) {
			/*{ Hee, hee... Ain't I mean?     }*/
			death = true;
			strcpy(died_from, hit_from);
			total_winner = false;
		}
		moria_flag = true;
	} else {
                prt_stat_block();
	}

	LEAVE("take_hit", "");
}
