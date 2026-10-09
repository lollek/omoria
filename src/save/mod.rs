#[cfg(test)]
pub(crate) use self::save::tests as test_support;
pub use self::save::*;
#[cfg(feature = "save-test-support")]
pub(crate) use self::save::{load_character_for_test, save_character_for_test};
#[cfg(feature = "save-test-support")]
pub(crate) use self::save::{
    test_apply_fixture_and_verify, test_reject_mismatched_uid_preserves_state, test_reset,
};
pub(crate) use self::save_record::SaveRecord;

mod dungeon;
mod equipment;
mod inventory;
mod monsters;
#[allow(clippy::module_inception)]
mod save;
mod save_interop;
mod save_record;
mod town;
