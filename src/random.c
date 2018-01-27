/* random.c */
/**/

#include "imoria.h"
#include <sys/time.h>

unsigned long get_seed()
{
	/*	{ Use date and time to produce a random seed	-RAK-	} */
	struct timeval tv;
	unsigned long the_seed;

	ENTER(("get_seed", ""));

	gettimeofday(&tv, NULL);

	the_seed = tv.tv_usec ^ tv.tv_sec;

	/*  the_seed = 883993667; */

	RETURN("get_seed", "", 'u', "rand seed", &the_seed);
	return the_seed;
}

void set_seed(unsigned long the_seed)
{
	/* use the_seed to seed the generator */
	MSG(("set_seed: s= %ld\n", the_seed));
	srand(the_seed);
}

long randnor(long mean, long stand)
{
	/*{ Generates a random integer number of NORMAL distribution -RAK-}*/

	return (long)(sqrt(-2.0 * log(randint(9999999) / 10000000.0)) *
		      cos(6.283 * (randint(9999999) / 10000000.0)) * stand) +
	       mean;
}

long rand_rep(long num, long die)
{
	long i1, sum = 0;

	for (i1 = 0; i1 < num; i1++) {
		sum += randint(die);
	}

	return sum;
}

long randint(long maxval)
{
	/* Generates a random integer x where 1<=X<=MAXVAL	-RAK-	*/

	long r = 0;

	if (maxval) {
		r = ((rand() % maxval) + 1);
	}

	return r;
}

void *save_rand_state(__attribute__((unused)) void *randState) { return NULL; }

void restore_rand_state(__attribute__((unused)) void *randState) {}
