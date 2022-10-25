pub const MONSTER_MAX_MALLOC: usize = 101;

extern "C" {
    pub static mut m_list: [Monster; MONSTER_MAX_MALLOC + 1];
    pub static mut muptr: libc::c_long;
}

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Monster {
    pub hp: i16,      // Hit points
    pub csleep: i16,  // Inactive counter
    pub cdis: i16,    // Cur dis from player
    pub mptr: u16,   // Pointer into creature
    #[serde(skip_serializing, default)]
    pub nptr: u16,   // Pointer to next block
    pub cspeed: i8,   // Movement speed

    // Note: FY and FX constrain dungeon size to 255
    pub fy: u8,      // Y Pointer into map
    pub fx: u8,      // X Pointer into map

    pub stunned: i8,  // Rounds stunned
    pub ml: u8,      // On if shown
    pub confused: u8,// On if confused
    pub moved: u8,   // On if water-moved
}
