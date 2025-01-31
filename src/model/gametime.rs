#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct GameTime {
    pub year: i64,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub secs: u16,
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

impl Default for GameTime {
    fn default() -> Self {
        Self::new()
    }
}
