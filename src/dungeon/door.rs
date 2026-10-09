use crate::model::{Cave, Item, ItemType};

#[derive(Debug, PartialEq)]
pub(crate) enum CloseResult {
    Closed,
    Monster(u8),
    Broken,
    NotDoor,
}

#[derive(Debug, PartialEq)]
pub(crate) enum JamTarget {
    Closed,
    Open,
    Monster(u8),
    NotDoor,
}

pub(crate) fn jam_target(cell: &Cave, item: &Item) -> JamTarget {
    if item.tval == u8::from(ItemType::ClosedDoor) {
        if cell.cptr == 0 {
            JamTarget::Closed
        } else {
            JamTarget::Monster(cell.cptr)
        }
    } else if item.tval == u8::from(ItemType::OpenDoor) {
        JamTarget::Open
    } else {
        JamTarget::NotDoor
    }
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
