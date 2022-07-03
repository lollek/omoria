#ifndef RANDOM_H
#define RANDOM_H

/* Generates a random integer x where 1<=X<=MAXVAL	-RAK-	*/
long randint(long maxval);
long rand_rep(long num, long die);
long randnor(long mean, long stand);

#endif // RANDOM_H