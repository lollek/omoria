#pragma once

enum hunger_status_t {
  DYING,   // Fainting, etc from lack of food
  WEAK,    // Weak from lack of food
  HUNGRY,  // Hungry, but no real adverse effects
  FULL,    // Perfect
  BLOATED, // Eaten too much, which might be a hindrance
};

enum hunger_status_t player_hunger_status(void);

/*
 * Recalculate player hunger level.
 * This function both recalculates values and gives feedback.
 */
void player_hunger_recalculate(void);

/*
 * Sets food count to the relevant level and resets flags.
 */
void player_hunger_set_status(enum hunger_status_t);

/*
 * Player eats food giving amount of food count.
 * This function both recalculates values and gives feedback.
 */
void player_hunger_eat(long amount);
