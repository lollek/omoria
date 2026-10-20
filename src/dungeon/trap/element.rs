//! Elemental trap effects.

#[cfg(not(test))]
use std::ffi::CString;

trait ElementEffects {
    fn corrode_gas(&mut self, cause: &'static str);
    fn fire_dam(&mut self, damage: i64, cause: &'static str);
    fn acid_dam(&mut self, damage: i64, cause: &'static str);
    fn poison_gas(&mut self, damage: i64, cause: &'static str);
    fn message(&mut self, message: &'static str);
}

fn corrode_gas_with_effects(effects: &mut impl ElementEffects) {
    effects.corrode_gas("corrosion gas.");
    effects.message("A strange red gas surrounds you.");
}

fn fire_with_effects(effects: &mut impl ElementEffects, damage: i64) {
    effects.fire_dam(damage, "a fire trap.");
    effects.message("You are enveloped in flames!");
}

fn acid_with_effects(effects: &mut impl ElementEffects, damage: i64) {
    effects.acid_dam(damage, "an acid trap.");
    effects.message("You are splashed with acid!");
}

fn poison_gas_with_effects(effects: &mut impl ElementEffects, damage: i64) {
    effects.poison_gas(damage, "a poison gas trap.");
    effects.message("A pungent green gas surrounds you!");
}

#[cfg(not(test))]
extern "C" {
    #[link_name = "corrode_gas"]
    fn effect_corrode_gas(cause: *const libc::c_char);
    #[link_name = "fire_dam"]
    fn effect_fire_dam(damage: libc::c_long, cause: *const libc::c_char);
    #[link_name = "acid_dam"]
    fn effect_acid_dam(damage: libc::c_long, cause: *const libc::c_char);
    #[link_name = "poison_gas"]
    fn effect_poison_gas(damage: libc::c_long, cause: *const libc::c_char);
}

#[cfg(not(test))]
struct GlobalElementEffects;

#[cfg(not(test))]
impl ElementEffects for GlobalElementEffects {
    fn corrode_gas(&mut self, cause: &'static str) {
        let cause = CString::new(cause).expect("trap effect causes contain no NUL bytes");
        unsafe { effect_corrode_gas(cause.as_ptr()) }
    }

    fn fire_dam(&mut self, damage: i64, cause: &'static str) {
        let cause = CString::new(cause).expect("trap effect causes contain no NUL bytes");
        unsafe { effect_fire_dam(damage as libc::c_long, cause.as_ptr()) }
    }

    fn acid_dam(&mut self, damage: i64, cause: &'static str) {
        let cause = CString::new(cause).expect("trap effect causes contain no NUL bytes");
        unsafe { effect_acid_dam(damage as libc::c_long, cause.as_ptr()) }
    }

    fn poison_gas(&mut self, damage: i64, cause: &'static str) {
        let cause = CString::new(cause).expect("trap effect causes contain no NUL bytes");
        unsafe { effect_poison_gas(damage as libc::c_long, cause.as_ptr()) }
    }

    fn message(&mut self, message: &'static str) {
        crate::term::msg_print(message);
    }
}

#[cfg(not(test))]
pub(super) fn corrode_gas() {
    corrode_gas_with_effects(&mut GlobalElementEffects);
}

#[cfg(not(test))]
pub(super) fn fire(damage: i64) {
    fire_with_effects(&mut GlobalElementEffects, damage);
}

#[cfg(not(test))]
pub(super) fn acid(damage: i64) {
    acid_with_effects(&mut GlobalElementEffects, damage);
}

#[cfg(not(test))]
pub(super) fn poison_gas(damage: i64) {
    poison_gas_with_effects(&mut GlobalElementEffects, damage);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    enum Call {
        CorrodeGas(&'static str),
        FireDam(i64, &'static str),
        AcidDam(i64, &'static str),
        PoisonGas(i64, &'static str),
        Message(&'static str),
    }

    #[derive(Default)]
    struct Recorder(Vec<Call>);

    impl ElementEffects for Recorder {
        fn corrode_gas(&mut self, cause: &'static str) {
            self.0.push(Call::CorrodeGas(cause));
        }

        fn fire_dam(&mut self, damage: i64, cause: &'static str) {
            self.0.push(Call::FireDam(damage, cause));
        }

        fn acid_dam(&mut self, damage: i64, cause: &'static str) {
            self.0.push(Call::AcidDam(damage, cause));
        }

        fn poison_gas(&mut self, damage: i64, cause: &'static str) {
            self.0.push(Call::PoisonGas(damage, cause));
        }

        fn message(&mut self, message: &'static str) {
            self.0.push(Call::Message(message));
        }
    }

    #[test]
    fn corrode_gas_calls_effect_before_its_message() {
        let mut effects = Recorder::default();
        corrode_gas_with_effects(&mut effects);
        assert_eq!(
            effects.0,
            [
                Call::CorrodeGas("corrosion gas."),
                Call::Message("A strange red gas surrounds you."),
            ]
        );
    }

    #[test]
    fn fire_passes_damage_and_cause_before_its_message() {
        let mut effects = Recorder::default();
        fire_with_effects(&mut effects, 23);
        assert_eq!(
            effects.0,
            [
                Call::FireDam(23, "a fire trap."),
                Call::Message("You are enveloped in flames!"),
            ]
        );
    }

    #[test]
    fn acid_passes_damage_and_cause_before_its_message() {
        let mut effects = Recorder::default();
        acid_with_effects(&mut effects, 23);
        assert_eq!(
            effects.0,
            [
                Call::AcidDam(23, "an acid trap."),
                Call::Message("You are splashed with acid!"),
            ]
        );
    }

    #[test]
    fn poison_gas_passes_damage_and_cause_before_its_message() {
        let mut effects = Recorder::default();
        poison_gas_with_effects(&mut effects, 23);
        assert_eq!(
            effects.0,
            [
                Call::PoisonGas(23, "a poison gas trap."),
                Call::Message("A pungent green gas surrounds you!"),
            ]
        );
    }
}
