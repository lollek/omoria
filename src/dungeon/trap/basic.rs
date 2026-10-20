pub(super) trait TrapEffects {
    fn feather_fall(&self) -> bool;
    fn arrow_hits(&mut self) -> bool;
    fn damage(&mut self, damage: i64, cause: &'static std::ffi::CStr);
    fn message(&mut self, message: &'static str);
}

pub(super) fn open_pit(context: &mut impl TrapEffects, damage: i64) {
    context.message("You fell into a pit!");
    if context.feather_fall() {
        context.message("You gently float down.");
    } else {
        context.damage(
            damage,
            std::ffi::CStr::from_bytes_with_nul(b"an open pit\0").unwrap(),
        );
    }
}

pub(super) fn arrow(context: &mut impl TrapEffects, damage: i64) {
    if context.arrow_hits() {
        context.damage(
            damage,
            std::ffi::CStr::from_bytes_with_nul(b"an arrow trap\0").unwrap(),
        );
        context.message("An arrow hits you.");
    } else {
        context.message("An arrow barely misses you.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Context {
        feather_fall: bool,
        arrow_hits: bool,
        events: Vec<String>,
    }

    impl TrapEffects for Context {
        fn feather_fall(&self) -> bool {
            self.feather_fall
        }

        fn arrow_hits(&mut self) -> bool {
            self.events.push("hit check".into());
            self.arrow_hits
        }

        fn damage(&mut self, damage: i64, cause: &'static std::ffi::CStr) {
            self.events
                .push(format!("damage {damage}: {}", cause.to_str().unwrap()));
        }

        fn message(&mut self, message: &'static str) {
            self.events.push(message.into());
        }
    }

    #[test]
    fn open_pit_announces_fall_before_damage() {
        let mut context = Context::default();
        open_pit(&mut context, 17);
        assert_eq!(
            context.events,
            ["You fell into a pit!", "damage 17: an open pit"]
        );
    }

    #[test]
    fn feather_fall_prevents_open_pit_damage() {
        let mut context = Context {
            feather_fall: true,
            ..Context::default()
        };
        open_pit(&mut context, 17);
        assert_eq!(
            context.events,
            ["You fell into a pit!", "You gently float down."]
        );
    }

    #[test]
    fn arrow_hit_damages_before_announcing_hit() {
        let mut context = Context {
            arrow_hits: true,
            ..Context::default()
        };
        arrow(&mut context, 23);
        assert_eq!(
            context.events,
            [
                "hit check",
                "damage 23: an arrow trap",
                "An arrow hits you."
            ]
        );
    }

    #[test]
    fn arrow_miss_only_announces_miss() {
        let mut context = Context::default();
        arrow(&mut context, 23);
        assert_eq!(context.events, ["hit check", "An arrow barely misses you."]);
    }
}
