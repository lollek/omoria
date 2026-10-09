use crate::{
    constants,
    model::{Cave, Item},
};

pub(crate) fn in_bounds(y: i64, x: i64, height: i64, width: i64) -> bool {
    y > 1 && y < height && x > 1 && x < width
}

pub(crate) struct Cells<'a> {
    pub cave: &'a mut [[Cave; constants::MAX_WIDTH + 1]],
    pub items: &'a mut [Item],
    pub height: i64,
    pub width: i64,
}

impl Cells<'_> {
    pub fn read(&self, y: i64, x: i64) -> Option<(Cave, Option<Item>)> {
        if !in_bounds(y, x, self.height, self.width) {
            return None;
        }
        let cell = *self.cave.get(y as usize)?.get(x as usize)?;
        let item = (cell.tptr != 0)
            .then(|| self.items.get(cell.tptr as usize).copied())
            .flatten();
        Some((cell, item))
    }

    pub fn write(&mut self, y: i64, x: i64, cell: Cave, item: Item) {
        if self.read(y, x).is_some() && cell.tptr != 0 && (cell.tptr as usize) < self.items.len() {
            self.cave[y as usize][x as usize] = cell;
            self.items[cell.tptr as usize] = item;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::{blank_item, TestMap};

    #[test]
    fn cells_read_and_write_typed_snapshots() {
        let mut map = TestMap::reset();
        map.put(2, 2, 1, 2, b"a chest");
        let mut cells = map.cells();
        let (mut cell, item) = cells.read(2, 2).unwrap();
        let mut item = item.unwrap();
        cell.fval = 5;
        item.flags = 16;
        cells.write(2, 2, cell, item);
        let (cell, item) = cells.read(2, 2).unwrap();
        assert_eq!(cell.fval, 5);
        assert_eq!(item.unwrap().flags, 16);
        assert!(cells.read(3, 3).unwrap().1.is_none());
    }

    #[test]
    fn cells_reject_edges_and_invalid_item_indices() {
        let mut map = TestMap::reset();
        map.cave[2][2].tptr = u8::MAX;
        let mut cells = map.cells();
        for (y, x) in [
            (-1, 2),
            (2, -1),
            (1, 2),
            (2, 1),
            (6, 2),
            (2, 6),
            (i64::MAX, 2),
        ] {
            assert!(cells.read(y, x).is_none());
            cells.write(y, x, crate::model::Cave::default(), blank_item());
        }
        let (cell, item) = cells.read(2, 2).unwrap();
        assert!(item.is_none());
        cells.write(2, 2, cell, blank_item());
        assert_eq!(cells.read(2, 2).unwrap().0.tptr, u8::MAX);
    }
}

#[cfg(not(test))]
/// # Safety
/// C globals must be initialized and exclusively accessed on the game thread.
pub(crate) unsafe fn with_global_cells<T>(access: impl FnOnce(&mut Cells<'_>) -> T) -> T {
    extern "C" {
        static mut cave: [[Cave; constants::MAX_WIDTH + 1]; constants::MAX_HEIGHT + 1];
        static mut t_list: [Item; constants::MAX_TALLOC + 1];
        static mut cur_height: libc::c_long;
        static mut cur_width: libc::c_long;
    }
    access(&mut Cells {
        cave: &mut *std::ptr::addr_of_mut!(cave),
        items: &mut *std::ptr::addr_of_mut!(t_list),
        height: cur_height,
        width: cur_width,
    })
}

#[cfg(test)]
pub(crate) mod test_support {
    use super::*;
    use std::sync::{Mutex, MutexGuard};

    static MAP_LOCK: Mutex<()> = Mutex::new(());

    pub struct TestMap {
        pub cave: Vec<[Cave; constants::MAX_WIDTH + 1]>,
        pub items: Vec<Item>,
        _lock: MutexGuard<'static, ()>,
    }

    pub fn blank_item() -> Item {
        Item {
            name: [0; 70],
            tval: 0,
            flags2: 0,
            flags: 0,
            p1: 0,
            cost: 0,
            subval: 0,
            weight: 0,
            number: 1,
            tohit: 0,
            todam: 0,
            ac: 0,
            toac: 0,
            damage: [0; 7],
            level: 0,
            identified: 0,
        }
    }

    impl TestMap {
        pub fn reset() -> Self {
            Self {
                cave: vec![[Cave::default(); constants::MAX_WIDTH + 1]; 7],
                items: vec![blank_item(); 16],
                _lock: MAP_LOCK.lock().unwrap_or_else(|err| err.into_inner()),
            }
        }

        pub fn cells(&mut self) -> Cells<'_> {
            Cells {
                cave: &mut self.cave,
                items: &mut self.items,
                height: 6,
                width: 6,
            }
        }

        pub fn put(&mut self, y: usize, x: usize, index: u8, tval: u8, name: &[u8]) {
            self.cave[y][x].tptr = index;
            let item = &mut self.items[index as usize];
            item.tval = tval;
            item.subval = 2;
            for (target, byte) in item.name.iter_mut().zip(name) {
                *target = *byte as libc::c_char;
            }
        }
    }
}
