#ifndef WIZARD_H
#define WIZARD_H

#include "boolean.h"

void enter_wizard_mode(boolean ask_for_pass);
void wizard_command(void);
void game_version();
boolean check_pswd(char passw[134], boolean present);
void wizard_light();
void change_character();
void wizard_create();

#endif // WIZARD_H