use crate::model::{InventoryItem, Item};
use std::ptr::NonNull;

/// A C-selected inventory node, live until `consume` destroys one item.
pub(crate) struct SelectedItem {
    node: NonNull<InventoryItem>,
}

impl SelectedItem {
    /// # Safety
    /// `node` must be a live, exclusively selected node in the C inventory list.
    /// No other action may remove it while this handle is alive.
    pub(crate) unsafe fn new(node: NonNull<InventoryItem>) -> Self {
        Self { node }
    }

    pub(crate) fn item(&self) -> Item {
        unsafe { self.node.as_ref().data }
    }

    pub(crate) fn identify(&mut self) {
        unsafe {
            crate::text_lines_extern::identify(std::ptr::addr_of_mut!((*self.node.as_ptr()).data))
        };
    }

    /// Reports the post-consumption count while the node is still valid.
    pub(crate) fn remaining(&self) {
        unsafe { crate::text_lines_extern::msg_remaining_of_item(self.node.as_ptr()) };
    }

    pub(crate) fn consume(self) {
        extern "C" {
            fn inven_destroy(item: *mut InventoryItem);
        }
        unsafe { inven_destroy(self.node.as_ptr()) };
    }
}
