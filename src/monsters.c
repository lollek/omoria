/* monsters.c */
/**/

#include "imoria.h"

static const long mon_nasty = 50; /// Dun_level/x chance of high level creature

/*{ Places a monster at given location -RAK- }*/
void place_monster(long y, long x, long z, boolean slp)
{
	long cur_pos;

	popm(&cur_pos);
	m_list[cur_pos].fy = y;
	m_list[cur_pos].fx = x;
	m_list[cur_pos].mptr = z;
	m_list[cur_pos].nptr = muptr;
	muptr = cur_pos;

	if ((c_list[z].cdefense & 0x4000) != 0) {
		m_list[cur_pos].hp = max_hp(c_list[z].hd);
	} else {
		m_list[cur_pos].hp = damroll(c_list[z].hd);
	}

	m_list[cur_pos].cdis = distance(char_row, char_col, y, x);
	m_list[cur_pos].cspeed = c_list[z].speed + player_flags.speed;
	m_list[cur_pos].stunned = 0;

	if (slp) {
		m_list[cur_pos].csleep =
			(c_list[z].sleep / 5.0) + randint(c_list[z].sleep);
	} else {
		m_list[cur_pos].csleep = 0;
	}

	cave[y][x].cptr = cur_pos;
}

/*{ Allocates a random land monster -RAK- }*/
void alloc_land_monster(obj_set alloc_set, long num, long dis, boolean slp,
		boolean water)
{
	long count = 0;
	long count2 = 0;

	for (int i = 0; i < num; i++) {
		long y;
		long x;
		do {
			y = randint(cur_height - 2) + 1;
			x = randint(cur_width - 2) + 1;
			if (++count2 > 7500)
				break;
		} while (!(is_in(cave[y][x].fval, alloc_set) &&
					cave[y][x].cptr == 0 &&
					cave[y][x].fopen &&
					distance(y, x, char_row, char_col) > dis));

		for (;;) {
			long monster_i;
			if (dun_level == 0) {
				monster_i = randint(m_level[0]);
			} else if (dun_level > MAX_MONS_LEVEL) {
				monster_i = randint(m_level[MAX_MONS_LEVEL]) +
					m_level[0];
			} else if (randint(mon_nasty) == 1) {
				monster_i = dun_level + labs(randnor(0, 4)) + 1;
				if (monster_i > MAX_MONS_LEVEL) {
					monster_i = MAX_MONS_LEVEL;
				}
				long i3 = m_level[monster_i] - m_level[monster_i - 1];
				monster_i = randint(i3) + m_level[monster_i - 1];
			} else {
				monster_i = randint(m_level[dun_level]) + m_level[0];
			}

			boolean ok_monster_found;
			if (!water) {
				ok_monster_found =
					(((c_list[monster_i].cmove & 0x00008000) == 0) &&
					 (((c_list[monster_i].cmove & 0x00000010) == 0) ||
					  ((c_list[monster_i].cmove & 0x00000040) == 0) ||
					  ((c_list[monster_i].cmove & 0x00800000) != 0)));
			} else {
				ok_monster_found =
					(((c_list[monster_i].cmove & 0x00008000) == 0) &&
					 ((c_list[monster_i].cmove & 0x00000010) != 0));
			}

			if (ok_monster_found) {
				if (count2 < 7500) {
					place_monster(y, x, monster_i, slp);
				}
				break;
			} else if (++count > 10) {
				break;
			}
		}
	}
}
