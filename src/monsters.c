#include "imoria.h"

void alloc_land_monster(obj_set alloc_set, long num, long dis, boolean slp,
			boolean water)
{
	/*{ Allocates a random land monster			-RAK-	}*/
	long count = 0;
	long count2 = 0;
	long i;

	for (i = 0; i < num; i++) {
		boolean flag;
		long y;
		long x;
		do {
			y = randint(cur_height - 2) + 1;
			x = randint(cur_width - 2) + 1;
			count2++;
		} while (!(((is_in(cave[y][x].fval, alloc_set)) &&
			    (cave[y][x].cptr == 0) && (cave[y][x].fopen) &&
			    (distance(y, x, char_row, char_col) > dis)) ||
			   (count2 > 7500)));

		do {
			long i2;
			if (dun_level == 0) {
				i2 = randint(m_level[0]);
			} else if (dun_level > MAX_MONS_LEVEL) {
				i2 = randint(m_level[MAX_MONS_LEVEL]) +
				     m_level[0];
			} else if (randint(mon_nasty) == 1) {
				long i3;
				i2 = dun_level + labs(randnor(0, 4)) + 1;
				if (i2 > MAX_MONS_LEVEL) {
					i2 = MAX_MONS_LEVEL;
				}
				i3 = m_level[i2] - m_level[i2 - 1];
				i2 = randint(i3) + m_level[i2 - 1];
			} else {
				i2 = randint(m_level[dun_level]) + m_level[0];
			}

			if (!water) {
				flag =
				    (((c_list[i2].cmove & 0x00008000) == 0) &&
				     (((c_list[i2].cmove & 0x00000010) == 0) ||
				      ((c_list[i2].cmove & 0x00000040) == 0) ||
				      ((c_list[i2].cmove & 0x00800000) != 0)));
			} else {
				flag =
				    (((c_list[i2].cmove & 0x00008000) == 0) &&
				     ((c_list[i2].cmove & 0x00000010) != 0));
			}

			if (flag) {
				if (count2 < 7500) {
					/*      printf ("\n             placing
					 */
					/*      monster");  fflush(stdout); */
					place_monster(y, x, i2, slp);
					flag = true;
					/*      printf ("\n             placed
					 */
					/*      monster");  fflush(stdout); */
				}
			}

			count++;
		} while (!(flag || count > 10));
	} /* end for */
}
