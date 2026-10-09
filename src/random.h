#pragma once

/* Generates a random integer x where 1<=X<=MAXVAL	-RAK-	*/
long randint(long maxval);
long rand_rep(long num, long die);
long randnor(long mean, long stand);

/* Seeds randint/rand_rep/randnor until the matching end; nests LIFO. */
void C_seeded_rng_begin(unsigned long long seed);
void C_seeded_rng_end(void);
