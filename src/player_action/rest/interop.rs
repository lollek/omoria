/// # Safety
/// `input` must point to a valid NUL-terminated string for the duration of the call.
/// Calls must be serialized with all other access to the C player globals.
#[no_mangle]
pub unsafe extern "C" fn C_player_action_rest_input(input: *const libc::c_char) -> bool {
    unsafe { super::globals::rest_global(input) }
}
