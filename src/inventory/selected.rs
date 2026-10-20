use crate::model::{InventoryItem, Item};
use std::ptr::NonNull;

/// Exclusive handle to a C-selected node, invalidated by `consume`.
/// Dropping the handle alone does not consume or free the inventory node.
pub(crate) struct SelectedItem {
    node: NonNull<InventoryItem>,
}

impl SelectedItem {
    /// Wraps a node selected by C without taking ownership of its allocation.
    /// # Safety
    /// `node` must be a live, exclusively selected node in the C inventory list.
    /// No other action may remove it while this handle is alive.
    pub(crate) unsafe fn new(node: NonNull<InventoryItem>) -> Self {
        Self { node }
    }

    /// Copies the current item without keeping a borrow across C calls.
    pub(crate) fn item(&self) -> Item {
        unsafe { self.node.as_ref().data }
    }

    /// Clears a secondary flag without replacing unrelated item attributes or
    /// retaining a borrow across calls into C.
    pub(crate) fn clear_secondary_flag(&mut self, flag: u64) {
        unsafe { self.node.as_mut().data.flags2 &= !flag };
    }

    /// Identifies the item and matching inventory subtypes via the shared API.
    pub(crate) fn identify(&mut self) {
        unsafe {
            crate::text_lines_extern::identify(std::ptr::addr_of_mut!((*self.node.as_ptr()).data))
        };
    }

    /// Reports the post-consumption count while the node is still valid.
    /// Call before `consume`, which may free the selected node.
    pub(crate) fn remaining(&self) {
        unsafe { crate::text_lines_extern::msg_remaining_of_item(self.node.as_ptr()) };
    }

    /// Applies C inventory bookkeeping and invalidates this handle. Ordinary
    /// stacks (`subval < 512`) lose one item; other selections may be removed
    /// entirely. An emptied node is unlinked and freed.
    pub(crate) fn consume(self) {
        extern "C" {
            fn inven_destroy(item: *mut InventoryItem);
        }
        unsafe { inven_destroy(self.node.as_ptr()) };
    }
}
