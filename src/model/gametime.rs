use libc;

#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct GameTime {
    pub year: libc::int64_t,
    pub month: libc::uint8_t,
    pub day: libc::uint8_t,
    pub hour: libc::uint8_t,
    pub secs: libc::uint16_t,
}

impl GameTime {
    pub fn new() -> Self {
        GameTime {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            secs: 0,
        }
    }
}
