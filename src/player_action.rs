pub(crate) mod attack;
mod bash;
mod close;
mod disarm_trap;
mod eat;
mod jam_door;
mod look;
#[cfg(not(test))]
pub(crate) use look::describe_at_global;
pub(crate) use look::MAX_SIGHT;
mod movement;
mod open;
mod quaff;
mod refill_lamp;
mod rest;
mod search;
mod stairs;
mod toggle_light_source;
mod tunnel;
