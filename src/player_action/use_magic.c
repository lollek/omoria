#include <string.h>

#include "../debug.h"
#include "../io.h"
#include "../inventory/inven.h"
#include "../magic.h"
#include "../misc.h"
#include "../player.h"
#include "../random.h"
#include "../screen.h"
#include "../spells.h"
#include "../variables.h"

static void nonmagic_song(void) {
  switch (C_player_mod_from_stat(CHR) + randint(4)) {
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

static void instrument_failure(void) {
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

static void song_failure(void) {
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

static void init_magic_books(const enum magic_t magic_type, obj_set *magic_books,
                             char const **book_name) {

  memset(*magic_books, 0, sizeof(*magic_books));
  switch (magic_type) {
  case M_ARCANE:
    (*magic_books)[0] = magic_book;
    *book_name = "spell-book";
    break;
  case M_DIVINE:
    (*magic_books)[0] = prayer_book;
    *book_name = "Holy Book";
    break;
  case M_NATURE:
    (*magic_books)[0] = instrument;
    *book_name = "instrument";
    break;
  case M_SONG:
    (*magic_books)[0] = song_book;
    *book_name = "instrument";
    break;
  case M_CHAKRA:
    (*magic_books)[0] = 0;
    *book_name = "discipline";
    break;
  }
}

static bool blind_check(const enum magic_t magic_type) {
  if (player_flags.blind > 0) {
    switch (magic_type) {
    case M_ARCANE:
      msg_print("You can't see to read your spell book!");
      return false;
    case M_DIVINE:
      msg_print("You can't see to read your prayer!");
      return false;
    case M_SONG:
      msg_print("You are too scared to play music!");
      return false;
    case M_NATURE:
    case M_CHAKRA:
      break;
    }
  }
  return true;
}

static bool light_check(const enum magic_t magic_type) {
  if (player_has_no_light()) {
    switch (magic_type) {
    case M_ARCANE:
    case M_DIVINE:
      msg_print("You have no light to read by.");
      return false;
    case M_NATURE:
    case M_SONG:
    case M_CHAKRA:
      break;
    }
  }
  return true;
}

static bool hoarse_check(const enum magic_t magic_type) {
  if (player_flags.hoarse > 0) {
    switch (magic_type) {
    case M_ARCANE:
    case M_DIVINE:
    case M_CHAKRA:
      break;
    case M_NATURE:
    case M_SONG:
      msg_print("You are too hoarse to sing!");
      return false;
    }
  }
  return true;
}

static bool scared_check(const enum magic_t magic_type) {
  if (player_flags.afraid > 0) {
    switch (magic_type) {
    case M_ARCANE:
    case M_DIVINE:
    case M_CHAKRA:
      break;
    case M_NATURE:
    case M_SONG:
      msg_print("You are too scared to play music!");
      return false;
    }
  }
  return true;
}

static bool hallucinate_check(const enum magic_t magic_type) {
  if (player_flags.image > 0) {
    switch (magic_type) {
    case M_ARCANE:
    case M_DIVINE:
    case M_NATURE:
    case M_SONG:
      break;
    case M_CHAKRA:
      msg_print("Colors and images run through your head, "
                "distracting you.");
      return false;
    }
  }
  return true;
}

static bool knowledge_check(const enum magic_t magic_type) {
  if (C_player_uses_magic(magic_type))
    return true;

  switch (magic_type) {
  case M_ARCANE:
    msg_print("You can't cast spells!");
    break;
  case M_DIVINE:
    msg_print("You pray briefly");
    break;
  case M_NATURE:
  case M_SONG:
    nonmagic_song();
    break;
  case M_CHAKRA:
    msg_print("You don't know any disciplines!");
    break;
  }
  return false;
}

static void drain_mana_failed(const enum magic_t magic_type,
                              const uint8_t mana_cost) {
  player_cmana = 0;

  switch (magic_type) {
  case M_ARCANE:
  case M_DIVINE:
    msg_print("You faint from fatigue!");
    player_flags.paralysis = randint(5 * (mana_cost - player_cmana));
    if (randint(3) == 1)
      lower_stat(CON, "You have damaged your health!");
    break;

  case M_NATURE:
  case M_SONG:
    msg_print("You lose your voice attempting the song!");
    player_flags.hoarse = randint(5 * (mana_cost - player_cmana));
    if (randint(3) == 1)
      lower_stat(CHR, "You have damaged your voice!");
    break;

  case M_CHAKRA:
    msg_print("You faint from fatigue!");
    player_flags.paralysis = randint(5 * mana_cost - player_cmana);
    if (randint(3) == 1)
      lower_stat(CON, "You have damaged your health!");
    break;
  }
}

static void drain_mana(const enum magic_t magic_type, const long choice) {
  const uint8_t mana_cost = C_magic_spell_mana(choice);
  if (mana_cost <= player_cmana)
    player_cmana -= mana_cost;
  else
    drain_mana_failed(magic_type, mana_cost);
  prt_stat_block();
}

/*{ Return spell number and failure chance                -RAK-   }*/
static bool cast_spell(char prompt[82], const treas_rec *item_ptr, long *sn,
                          long *sc, bool *redraw) {

  unsigned long i2, i4;
  long num;
  spl_type aspell;
  bool flag = false;

  long i1 = num = 0;
  i2 = item_ptr->data.flags;
  i4 = item_ptr->data.flags2;

  do {
    long i3 = bit_pos64(&i4, &i2) + 1;
    /*{ Avoid the cursed bit like the plague -DMF-   }*/
    if (i3 > 31) {
      i3--;
    }
    if (i3 > 0) {
      i3--;
      if (C_magic_spell_level(i3) <= player_lev &&
          C_player_knows_spell(i3)) {
        aspell[i1++].splnum = i3;
        num = i1;
      } else {
        aspell[i1++].splnum = -1; /* leave gaps for unknown spells */
      }
    }

  } while (!(i2 == 0 && i4 == 0));

  if (num > 0) {
    flag = get_spell(aspell, num, sn, sc, prompt, redraw);
  }

  if (*redraw) {
    draw_cave();
  }

  return flag;
}

static void _cast(const enum magic_t magic_type) {
  treas_rec *i1;
  treas_rec *item_ptr = &inven_temp;
  long choice;
  long chance;
  long i2;
  char trash_char;
  bool redraw;
  obj_set magic_books;
  char const *magic_book_name = "???";
  char msg_buf[80];

  init_magic_books(magic_type, &magic_books, &magic_book_name);

  reset_flag = true;
  redraw = false;

  if (!knowledge_check(magic_type))
    return;
  if (player_flags.confused > 0) {
    msg_print("You are too confused...");
    return;
  }
  if (!blind_check(magic_type))
    return;
  if (!light_check(magic_type))
    return;
  if (!hoarse_check(magic_type))
    return;
  if (!scared_check(magic_type))
    return;
  if (!hallucinate_check(magic_type))
    return;

  /* Check book users for books */
  if (magic_books[0] != 0) {
    if (inven_ctr <= 0 || !inventory_find_range(magic_books, false, &i1, &i2)) {
      sprintf(msg_buf, "But you are not carrying any %ss!", magic_book_name);
      msg_print(msg_buf);
      return;
    }

    sprintf(msg_buf, "Use which %s?", magic_book_name);
    if (!get_item(&item_ptr, msg_buf, &redraw, i2, &trash_char, false, false)) {
      if (redraw)
        draw_cave();
      return;
    }
  } else { /* Check non-book-user for magic knowledge */
    bool knows_magic = false;
    for (int i = 0; i < 40; ++i) {
      if (C_player_knows_spell(i)) {
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

  if (C_magic_spell_mana(choice) > player_cmana &&
      !get_yes_no("You are low on mana, are you sure?"))
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
  }

  if (reset_flag) {
    if (redraw)
      draw_cave();
    return;
  }

  drain_mana(magic_type, choice);
}

void player_action_use_magic(const enum magic_t magic_type) {
  ENTER(("cast", ""));
  _cast(magic_type);
  LEAVE("cast", "");
}
