#pragma once

/**
 * Ask the player to select an ability to use
 *
 * Returns true if the turn was free (i.e. don't take a turn)
 */
bool C_select_ability(void);

/* Make passive abilities tick down and do their things */
void C_check_passive_abilities(void);