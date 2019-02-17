/* pascal.c */
/* code to help port pascal */

#include "imoria.h"

/*//////////////////////////////////////////////////////////////////// */

void writeln(char *out_line) { printf("%s\n\r", out_line); }

/*//////////////////////////////////////////////////////////////////// */

long pindex(char *s1, char c1)
{
	/*
	  pascal index function, return position of c1 in s1

	  returns i if (s1[i-1] == c1), returns 0 if c1 not found
	 */

	char *c2;

	c2 = strchr(s1, c1);

	if (c2 == NULL) {
		return 0;
	}

	return (c2 - s1 + 1);
}
/*//////////////////////////////////////////////////////////////////// */
boolean is_in(long const obj, obj_set const oset)
{
	long i1;
	long tmp;
	boolean return_value = false;

/*    ENTER("is_in","m"); */

#if DO_DEBUG
	tmp = 0;
	for (i1 = 0; i1 < MAX_OBJ_SET; i1++) {
		if ((tmp > oset[i1]) && (oset[i1] > 0)) {
			printf("bad obj_set i1 = %ld\n\r", i1);
			MSG(("bad obj_set: %d", oset[i1]));
			for (i1 = 0; i1 < MAX_OBJ_SET; i1++) {
				MSG(("%d  ", oset[i1]));
			}
			fprintf(debug_file, "\n");
			abort();
		}
		tmp = oset[i1];
	}
#endif
	for (i1 = 0; (i1 < MAX_OBJ_SET) && (oset[i1] <= obj) && (!return_value);
	     i1++) {
		/*for (i1=0;(i1<MAX_OBJ_SET) && (!return_value); i1++) {*/
		if (obj == oset[i1]) {
			return_value = true;
		}
	}

	/*    RETURN("is_in","m",'b',"isin",&return_value); */
	return return_value;
}
/*//////////////////////////////////////////////////////////////////// */
boolean is_vowel(char a_char)
{
	switch (a_char) {
	case 'a':
	case 'A':
	case 'e':
	case 'E':
	case 'i':
	case 'I':
	case 'o':
	case 'O':
	case 'u':
	case 'U':
		return true;
		break;

	default:
		return false;
		break;
	}
}
/*//////////////////////////////////////////////////////////////////// */

/*//////////////////////////////////////////////////////////////////// */

/* END FILE  pascal.c */
