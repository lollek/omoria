use std::convert::TryFrom;

pub const MONSTER_MAX_MALLOC: usize = 101;

extern "C" {
    pub static mut m_list: [Monster; MONSTER_MAX_MALLOC + 1];
    pub static mut muptr: libc::c_long;
}

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize, Debug, Default)]
pub struct Monster {
    pub hp: i16,     // Hit points
    pub csleep: i16, // Inactive counter
    pub cdis: i16,   // Cur dis from player
    pub mptr: u16,   // Pointer into creature
    #[serde(skip_serializing, default)]
    pub nptr: u16, // Pointer to next block
    pub cspeed: i8,  // Movement speed

    // Note: FY and FX constrain dungeon size to 255
    pub fy: u8, // Y Pointer into map
    pub fx: u8, // X Pointer into map

    pub stunned: i8,  // Rounds stunned
    pub is_seen: u8,  // On if shown
    pub confused: u8, // On if confused
    pub moved: u8,    // On if water-moved
}

/// Copies seen instances from the active C list on the exclusive game thread.
pub(crate) fn seen_monsters() -> Vec<Monster> {
    let mut seen = Vec::new();
    let mut visited = [false; MONSTER_MAX_MALLOC + 1];
    let mut index = unsafe { muptr };
    while index != 0 {
        let index_usize = usize::try_from(index).expect("negative active monster index");
        let visited = visited
            .get_mut(index_usize)
            .expect("active monster index outside m_list");
        assert!(!*visited, "cycle in active monster list");
        *visited = true;
        let monster = unsafe { (*std::ptr::addr_of!(m_list))[index_usize] };
        if monster.is_seen != 0 {
            seen.push(monster);
        }
        index = monster.nptr.into();
    }
    seen
}

#[cfg(test)]
mod tests {
    use super::*;

    const BLANK: Monster = Monster {
        hp: 0,
        csleep: 0,
        cdis: 0,
        mptr: 0,
        nptr: 0,
        cspeed: 0,
        fy: 0,
        fx: 0,
        stunned: 0,
        is_seen: 0,
        confused: 0,
        moved: 0,
    };

    #[export_name = "m_list"]
    static mut TEST_MONSTERS: [Monster; MONSTER_MAX_MALLOC + 1] = [BLANK; MONSTER_MAX_MALLOC + 1];
    #[export_name = "muptr"]
    static mut TEST_HEAD: libc::c_long = 0;

    struct MonsterGuard([Monster; MONSTER_MAX_MALLOC + 1], libc::c_long);

    impl MonsterGuard {
        fn new(list: [Monster; MONSTER_MAX_MALLOC + 1], head: libc::c_long) -> Self {
            let previous = unsafe { Self(TEST_MONSTERS, TEST_HEAD) };
            unsafe {
                TEST_MONSTERS = list;
                TEST_HEAD = head;
            }
            previous
        }
    }

    impl Drop for MonsterGuard {
        fn drop(&mut self) {
            unsafe {
                TEST_MONSTERS = self.0;
                TEST_HEAD = self.1;
            }
        }
    }

    #[test]
    #[serial_test::serial]
    fn seen_monsters_reads_only_seen_active_c_instances_without_mutation() {
        let mut list = [BLANK; MONSTER_MAX_MALLOC + 1];
        list[7] = Monster {
            is_seen: 1,
            mptr: 2,
            cdis: 9,
            nptr: 3,
            ..BLANK
        };
        list[3] = Monster {
            is_seen: 0,
            nptr: 5,
            ..BLANK
        };
        list[5] = Monster {
            is_seen: 2,
            mptr: 4,
            cdis: 2,
            ..BLANK
        };
        list[8].is_seen = 1; // A non-active slot must not leak into the panel.
        let _guard = MonsterGuard::new(list, 7);
        let before = serde_json::to_value(list.to_vec()).unwrap();

        let seen = seen_monsters();

        assert_eq!(
            seen.iter().map(|m| (m.mptr, m.cdis)).collect::<Vec<_>>(),
            [(2, 9), (4, 2)]
        );
        assert_eq!(
            serde_json::to_value(unsafe { TEST_MONSTERS }.to_vec()).unwrap(),
            before
        );
        assert_eq!(unsafe { TEST_HEAD }, 7);
    }

    #[test]
    #[serial_test::serial]
    fn seen_monsters_is_empty_without_active_or_seen_monsters() {
        let mut list = [BLANK; MONSTER_MAX_MALLOC + 1];
        list[2].is_seen = 1;
        let _guard = MonsterGuard::new(list, 0);
        assert!(seen_monsters().is_empty());

        let _guard = MonsterGuard::new([BLANK; MONSTER_MAX_MALLOC + 1], 2);
        assert!(seen_monsters().is_empty());
    }

    #[test]
    #[serial_test::serial]
    #[should_panic(expected = "negative active monster index")]
    fn seen_monsters_rejects_a_negative_list_head() {
        let _guard = MonsterGuard::new([BLANK; MONSTER_MAX_MALLOC + 1], -1);
        seen_monsters();
    }

    #[test]
    #[serial_test::serial]
    #[should_panic(expected = "active monster index outside m_list")]
    fn seen_monsters_rejects_an_out_of_bounds_list_link() {
        let mut list = [BLANK; MONSTER_MAX_MALLOC + 1];
        list[2].nptr = (MONSTER_MAX_MALLOC + 1) as u16;
        let _guard = MonsterGuard::new(list, 2);
        seen_monsters();
    }

    #[test]
    #[serial_test::serial]
    #[should_panic(expected = "cycle in active monster list")]
    fn seen_monsters_rejects_a_list_cycle() {
        let mut list = [BLANK; MONSTER_MAX_MALLOC + 1];
        list[2].nptr = 2;
        let _guard = MonsterGuard::new(list, 2);
        seen_monsters();
    }

    #[test]
    fn test_serialize() {
        let monster = Monster::default();
        serde_json::to_string(&monster).expect("Failed to serialize Monster");
    }
}
