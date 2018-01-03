#include "imoria.h"
#include "dungeon.h"

static void nonmagic_song(void)
{
	switch (bard_adj() + randint(4)) {
	case 0:
	case 1:
	case 2:
		msg_print("You utter a gutteral cry.");
		break;
	case 3:
	case 4:
		msg_print("You mumble a simple tune.");
		break;
	case 5:
	case 6:
		msg_print("You sing a fair song.");
		break;
	case 7:
	case 8:
		msg_print("You sing a very nice song.");
		break;
	default:
		msg_print("You sing quite beautifully.");
		break;
	}
}

static void instrument_failure(void)
{
	switch (randint(5)) {
	case 1:
		msg_print("*Twang!*");
		break;
	case 2:
		msg_print("*Boink!*");
		break;
	case 3:
		msg_print("*Ding!*");
		break;
	case 4:
		msg_print("*Plunk!*");
		break;
	case 5:
		msg_print("*Clang!*");
		break;
	}
	switch (randint(2)) {
	case 1:
		msg_print("You play a sour note!");
		break;
	case 2:
		msg_print("You play an awful note!");
		break;
	}
}

static void song_failure(void)
{
	switch (randint(5)) {
	case 1:
		msg_print("*Squak*");
		break;
	case 2:
		msg_print("*Gag!*");
		break;
	case 3:
		msg_print("*Aaugh!*");
		break;
	case 4:
		msg_print("*Yargh!*");
		break;
	case 5:
		msg_print("*Cough!*");
		break;
	}
	switch (randint(2)) {
	case 1:
		msg_print("You sing a sour note!");
		break;
	case 2:
		msg_print("You sing an awful note!");
		break;
	}
}

static void _cast(enum magic_t magic_type)
{
	treas_ptr i1;
	treas_ptr item_ptr = inven_temp;
	integer choice;
	integer chance;
	integer i2;
	char trash_char;
	boolean redraw;
	obj_set magic_books;
	char const *magic_book_name = "???";
	char msg_buf[80];

	memset(magic_books, 0, sizeof(magic_books));
	switch (magic_type) {
	case M_ARCANE:
		magic_books[0] = Magic_Book;
		magic_book_name = "spell-book";
		break;
	case M_DIVINE:
		magic_books[0] = Prayer_Book;
		magic_book_name = "Holy Book";
		break;
	case M_NATURE:
		magic_books[0] = Instrument;
		magic_book_name = "instrument";
		break;
	case M_SONG:
		magic_books[0] = Song_Book;
		magic_book_name = "instrument";
		break;
	case M_CHAKRA:
		magic_books[0] = 0;
		magic_book_name = "discipline";
		break;
	}

	reset_flag = true;
	redraw = false;

	if (py.flags.confused > 0) {
		msg_print("You are too confused...");
		return;
	}

	if (py.flags.blind > 0) {
		switch (magic_type) {
		case M_ARCANE:
			msg_print("You can't see to read your spell book!");
			return;
		case M_DIVINE:
			msg_print("You can't see to read your prayer!");
			return;
		case M_SONG:
			msg_print("You are too scared to play music!");
			return;
		case M_NATURE:
		case M_CHAKRA:
			break;
		}
	}

	if (no_light()) {
		switch (magic_type) {
		case M_ARCANE:
		case M_DIVINE:
			msg_print("You have no light to read by.");
			return;
		case M_NATURE:
		case M_SONG:
		case M_CHAKRA:
			break;
		}
		return;
	}

	if (py.flags.hoarse > 0) {
		switch (magic_type) {
		case M_ARCANE:
		case M_DIVINE:
		case M_CHAKRA:
			break;
		case M_NATURE:
		case M_SONG:
			msg_print("You are too hoarse to sing!");
			return;
		}
	}

	if (py.flags.afraid > 0) {
		switch (magic_type) {
		case M_ARCANE:
		case M_DIVINE:
		case M_CHAKRA:
			break;
		case M_NATURE:
		case M_SONG:
			msg_print("You are too scared to play music!");
			return;
		}
	}

	if (py.flags.image > 0) {
		switch (magic_type) {
		case M_ARCANE:
		case M_DIVINE:
		case M_NATURE:
		case M_SONG:
			break;
		case M_CHAKRA:
			msg_print("Colors and images run through your head, "
				  "distracting you.");
			return;
		}
	}

	if (!class_uses_magic(py.misc.pclass, magic_type)) {
		switch (magic_type) {
		case M_ARCANE:
			msg_print("You can't cast spells!");
			return;
		case M_DIVINE:
			msg_print("You pray briefly");
			return;
		case M_NATURE:
		case M_SONG:
			nonmagic_song();
			return;
		case M_CHAKRA:
			msg_print("You don't know any disciplines!");
			return;
		}
	}

	/* Check book users for books */
	if (magic_books[0] != 0) {
		if (inven_ctr <= 0 || !find_range(magic_books, false, &i1, &i2)) {
			sprintf(msg_buf, "But you are not carrying any %ss!", magic_book_name);
			msg_print(msg_buf);
			return;
		}

		sprintf(msg_buf, "Use which %s?", magic_book_name);
		if (!get_item(&item_ptr, msg_buf, &redraw, i2,
			      &trash_char, false, false)) {
			if (redraw)
				draw_cave();
			return;
		}
	} else { /* Check non-book-user for magic knowledge */
		boolean knows_magic = false;
		int i;
		for (i = 0; i < 40; ++i) {
			if (magic_spell[py.misc.pclass][i].learned) {
				knows_magic = true;
				break;
			}
		}
		if (!knows_magic) {
			msg_print("You don't know any disciplines!");
			return;
		}
	}


	switch (magic_type) {
		case M_ARCANE:
			strcpy(msg_buf, "Cast which spell?");
			break;
		case M_DIVINE:
			strcpy(msg_buf, "Recite which prayer?");
			break;
		case M_NATURE:
		case M_SONG:
			strcpy(msg_buf, "Play which song?");
			break;
		case M_CHAKRA:
			strcpy(msg_buf, "Use which disciplines!");
			break;
	}
	if (!cast_spell(msg_buf, item_ptr, &choice, &chance, &redraw)) {
		return;
	}

	if (magic_spell[PM.pclass][choice].smana > PM.cmana &&
	    !get_check("You are low on mana, are you sure?"))
		return;

	reset_flag = false;
	if (randint(100) < chance) {
		switch (magic_type) {
		case M_ARCANE:
			msg_print("You failed to get the spell off!");
			break;
		case M_DIVINE:
			msg_print("You lost your concentration!");
			break;
		case M_NATURE:
			instrument_failure();
			break;
		case M_SONG:
			song_failure();
			break;
		case M_CHAKRA:
			msg_print("You lost your concentration!");
			break;
		}
	} else {
		switch (magic_type) {
		case M_ARCANE:
			arcane_spell_effects(choice);
			break;
		case M_DIVINE:
			divine_spell_effects(choice);
			break;
		case M_NATURE:
			nature_spell_effects(choice);
			break;
		case M_SONG:
			song_spell_effects(choice);
			break;
		case M_CHAKRA:
			chakra_spell_effects(choice);
			break;
		}
		if (!reset_flag) {
			PM.exp += magic_spell[PM.pclass][choice].sexp;
			prt_experience();
			magic_spell[PM.pclass][choice].sexp = 0;
		}
	}

	if (reset_flag) {
		if (redraw)
			draw_cave();
		return;
	}

	if (magic_spell[PM.pclass][choice].smana > PM.cmana) {
		if (magic_type == M_ARCANE || magic_type == M_DIVINE) {
			msg_print("You faint from fatigue!");
			py.flags.paralysis = randint(
			    5 * trunc(magic_spell[PM.pclass][choice].smana -
				      PM.cmana));
			PM.cmana = 0;
			if (randint(3) == 1)
				lower_stat(CON,
					   "You have damaged your health!");

		} else if (magic_type == M_NATURE || magic_type == M_SONG) {
			msg_print("You lose your voice attempting the song!");
			py.flags.hoarse = randint(
			    5 * (magic_spell[PM.pclass][choice].smana - PM.cmana));
			PM.cmana = 0;
			if (randint(3) == 1)
				lower_stat(CHR, "You have damaged your voice!");

		} else if (magic_type == M_CHAKRA) {
			msg_print("You faint from fatigue!");
			py.flags.paralysis = randint(
			    5 * trunc(magic_spell[PM.pclass][choice].smana -
				      PM.cmana));
			PM.cmana = 0;
			if (randint(3) == 1)
				lower_stat(CON,
					   "You have damaged your health!");
		}
	} else {
		PM.cmana -= magic_spell[PM.pclass][choice].smana;
	}
	prt_mana();
}

void cast(enum magic_t magic_type)
{
	ENTER("cast", "");
	_cast(magic_type);
	LEAVE("cast", "");
}
