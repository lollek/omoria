use crate::model::{Cave, Item, ItemType};

#[derive(Debug, PartialEq)]
pub(crate) enum CloseResult {
    Closed,
    Monster(u8),
    Broken,
    NotDoor,
}

pub(crate) fn close(cell: &mut Cave, item: &mut Item, closed_door: Item) -> CloseResult {
    if item.tval != u8::from(ItemType::OpenDoor) {
        return CloseResult::NotDoor;
    }
    if cell.cptr != 0 {
        return CloseResult::Monster(cell.cptr);
    }
    if item.p1 != 0 {
        return CloseResult::Broken;
    }
    *item = closed_door;
    cell.fopen = 0;
    CloseResult::Closed
}
