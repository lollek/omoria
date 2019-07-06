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

impl Monster {
    pub fn new() -> Self {
        Monster {
            hp: 0,
            csleep: 0,
            cdis: 0,
            mptr: 0,
            nptr: 0,
            cspeed: 0,

            fy: 0,
            fx: 0,

            stunned: 0,
            ml: 0,
            confused: 0,
            moved: 0,
        }
    }
}
