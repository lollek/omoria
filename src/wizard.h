#pragma once

#include <stdbool.h>

void enter_wizard_mode(bool ask_for_pass);
void wizard_command(void);
void game_version(void);
bool check_pswd(char const *passw, bool present);
void wizard_light(void);
void change_character(void);
void wizard_create(void);
