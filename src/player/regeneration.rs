/// Player regeneration logic.
///
/// Determines how much health/mana the player regenerates per tick,
/// based on hunger status, regeneration ability, and rest status.

/// Hunger status levels that affect regeneration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HungerStatus {
    Dying,   // Fainting from lack of food
    Weak,    // Weak from lack of food
    Hungry,  // Hungry, but no real adverse effects
    Full,    // Perfect
    Bloated, // Eaten too much
}

/// Input state for calculating regeneration amount.
#[derive(Debug, Clone, Copy)]
pub struct RegenerationInput {
    pub hunger_status: HungerStatus,
    pub has_regeneration: bool,
    pub is_resting: bool,
}

/// Calculate the regeneration amount based on player state.
///
/// Returns a multiplier used to determine health/mana regeneration per tick.
pub fn get_regeneration_amount(input: RegenerationInput) -> f32 {
    let mut regen_amount = match input.hunger_status {
        HungerStatus::Dying => 0.0005,
        HungerStatus::Weak => 0.0015,
        HungerStatus::Hungry | HungerStatus::Full | HungerStatus::Bloated => 0.0030,
    };

    if input.has_regeneration {
        regen_amount *= 1.5;
    }

    if input.is_resting {
        regen_amount *= 2.0;
    }

    regen_amount
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(hunger: HungerStatus, regenerate: bool, resting: bool) -> RegenerationInput {
        RegenerationInput {
            hunger_status: hunger,
            has_regeneration: regenerate,
            is_resting: resting,
        }
    }

    #[test]
    fn base_regeneration_when_full() {
        let amount = get_regeneration_amount(input(HungerStatus::Full, false, false));
        assert!((amount - 0.003).abs() < 0.0001, "Expected 0.003, got {}", amount);
    }

    #[test]
    fn base_regeneration_when_hungry() {
        let amount = get_regeneration_amount(input(HungerStatus::Hungry, false, false));
        assert!((amount - 0.003).abs() < 0.0001, "Expected 0.003, got {}", amount);
    }

    #[test]
    fn base_regeneration_when_bloated() {
        let amount = get_regeneration_amount(input(HungerStatus::Bloated, false, false));
        assert!((amount - 0.003).abs() < 0.0001, "Expected 0.003, got {}", amount);
    }

    #[test]
    fn reduced_regeneration_when_weak() {
        let amount = get_regeneration_amount(input(HungerStatus::Weak, false, false));
        assert!((amount - 0.0015).abs() < 0.0001, "Expected 0.0015, got {}", amount);
    }

    #[test]
    fn minimal_regeneration_when_dying() {
        let amount = get_regeneration_amount(input(HungerStatus::Dying, false, false));
        assert!((amount - 0.0005).abs() < 0.0001, "Expected 0.0005, got {}", amount);
    }

    #[test]
    fn regeneration_ability_multiplies_by_1_5() {
        let amount = get_regeneration_amount(input(HungerStatus::Full, true, false));
        let expected = 0.003 * 1.5;
        assert!((amount - expected).abs() < 0.0001, "Expected {}, got {}", expected, amount);
    }

    #[test]
    fn resting_doubles_regeneration() {
        let amount = get_regeneration_amount(input(HungerStatus::Full, false, true));
        let expected = 0.003 * 2.0;
        assert!((amount - expected).abs() < 0.0001, "Expected {}, got {}", expected, amount);
    }

    #[test]
    fn regeneration_and_resting_stack_multiplicatively() {
        let amount = get_regeneration_amount(input(HungerStatus::Full, true, true));
        let expected = 0.003 * 1.5 * 2.0;
        assert!((amount - expected).abs() < 0.0001, "Expected {}, got {}", expected, amount);
    }

    #[test]
    fn dying_with_regeneration_and_resting() {
        let amount = get_regeneration_amount(input(HungerStatus::Dying, true, true));
        let expected = 0.0005 * 1.5 * 2.0;
        assert!((amount - expected).abs() < 0.0001, "Expected {}, got {}", expected, amount);
    }
}
