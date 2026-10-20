/// # Safety
/// `item` must be the live inventory node returned by C `get_item`, with
/// exclusive access for this action. Consuming its last item frees the node.
#[no_mangle]
pub unsafe extern "C" fn C_player_action_quaff_selected(item: *mut crate::model::InventoryItem) {
    unsafe { super::globals::quaff_global(item) }
}
