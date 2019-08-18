pub const MONSTER_MAX_MALLOC: usize = 101;

extern "C" {
    pub static mut m_list: [Monster; MONSTER_MAX_MALLOC + 1];
    pub static mut muptr: libc::c_long;
}

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Monster {
    pub hp: libc::int16_t,      // Hit points
    pub csleep: libc::int16_t,  // Inactive counter
    pub cdis: libc::int16_t,    // Cur dis from player
    pub mptr: libc::uint16_t,   // Pointer into creature
    #[serde(skip_serializing, default)]
    pub nptr: libc::uint16_t,   // Pointer to next block
    pub cspeed: libc::int8_t,   // Movement speed

    // Note: FY and FX constrain dungeon size to 255
    pub fy: libc::uint8_t,      // Y Pointer into map
    pub fx: libc::uint8_t,      // X Pointer into map

    pub stunned: libc::int8_t,  // Rounds stunned
    pub ml: libc::uint8_t,      // On if shown
    pub confused: libc::uint8_t,// On if confused
    pub moved: libc::uint8_t,   // On if water-moved
}
