#[cfg(not(test))]
mod globals;
#[cfg(not(test))]
mod interop;

/// Equipped-light state and display effects needed to toggle the player's light.
///
/// The game implementation bridges C equipment, flags, and UI calls; tests keep
/// light state and event records in memory. Toggling does not consume fuel.
trait LightContext {
    /// Marks the command as not consuming a game turn, including rejected toggles.
    fn reset_turn(&mut self);
    /// Returns the equipped light's raw item type (`tval`) and remaining fuel turns.
    /// A nonpositive type means no light; nonpositive fuel means it has gone out.
    fn light_source(&self) -> (i64, i64);
    /// Returns whether the player's light is switched on.
    fn light_on(&self) -> bool;
    /// Updates both the light-on flag and the player's active-light state.
    fn set_light(&mut self, on: bool);
    /// Refreshes the light's on/off status indicator after changing its state.
    fn status(&mut self);
    /// Sends a player-facing message to the game's message display.
    fn message(&mut self, message: &str);
    /// Recalculates and displays dungeon lighting at the player's current position.
    fn redraw(&mut self);
}

fn toggle_light_source(context: &mut impl LightContext) {
    context.reset_turn();
    let (tval, fuel) = context.light_source();
    if tval <= 0 {
        context.message("You are not carrying a light.");
        return;
    }
    if fuel <= 0 {
        context.message("Your light has gone out!");
        return;
    }
    let on = !context.light_on();
    context.set_light(on);
    context.status();
    let status = if on { "On" } else { "Off" };
    context.message(&format!("Light {status}.  {fuel} turns left."));
    context.redraw();
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Context {
        tval: i64,
        fuel: i64,
        light_on: bool,
        player_light: bool,
        reset_flag: bool,
        events: Vec<String>,
    }

    impl Context {
        fn new(light_on: bool) -> Self {
            Self {
                tval: 15,
                fuel: 123,
                light_on,
                player_light: !light_on,
                reset_flag: false,
                events: Vec::new(),
            }
        }

        fn assert_refused(&self, on: bool, message: &str) {
            assert!(self.reset_flag);
            assert_eq!(self.light_on, on);
            assert_eq!(self.player_light, !on);
            assert_eq!(self.events, [message]);
        }
    }

    impl LightContext for Context {
        fn reset_turn(&mut self) {
            self.reset_flag = true;
        }
        fn light_source(&self) -> (i64, i64) {
            (self.tval, self.fuel)
        }
        fn light_on(&self) -> bool {
            self.light_on
        }
        fn set_light(&mut self, on: bool) {
            assert!(self.reset_flag);
            self.light_on = on;
            self.player_light = on;
            self.events.push("flags".to_owned());
        }
        fn status(&mut self) {
            assert_eq!(self.player_light, self.light_on);
            self.events.push("status".to_owned());
        }
        fn message(&mut self, message: &str) {
            assert!(self.reset_flag);
            self.events.push(message.to_owned());
        }
        fn redraw(&mut self) {
            self.events.push("redraw".to_owned());
        }
    }

    #[test]
    fn turns_light_on_before_status_message_and_redraw() {
        let mut context = Context::new(false);
        toggle_light_source(&mut context);
        assert!(context.reset_flag);
        assert!(context.light_on);
        assert!(context.player_light);
        assert_eq!(
            context.events,
            ["flags", "status", "Light On.  123 turns left.", "redraw"]
        );
        assert_eq!(context.fuel, 123);
    }

    #[test]
    fn turns_light_off_before_status_message_and_redraw() {
        let mut context = Context::new(true);
        context.fuel = i64::MAX;
        toggle_light_source(&mut context);
        assert!(context.reset_flag);
        assert!(!context.light_on);
        assert!(!context.player_light);
        assert_eq!(
            context.events,
            [
                "flags",
                "status",
                "Light Off.  9223372036854775807 turns left.",
                "redraw"
            ]
        );
        assert_eq!(context.fuel, i64::MAX);
    }

    #[test]
    fn missing_light_preserves_flags_and_takes_precedence_over_empty_fuel() {
        for tval in [-1, 0] {
            for on in [false, true] {
                let mut context = Context::new(on);
                context.tval = tval;
                context.fuel = 0;
                toggle_light_source(&mut context);
                context.assert_refused(on, "You are not carrying a light.");
            }
        }
    }

    #[test]
    fn exhausted_light_preserves_flags_without_status_or_redraw() {
        for fuel in [-1, 0] {
            for on in [false, true] {
                let mut context = Context::new(on);
                context.fuel = fuel;
                toggle_light_source(&mut context);
                context.assert_refused(on, "Your light has gone out!");
            }
        }
    }
}
