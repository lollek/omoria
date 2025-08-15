#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Time {
    pub years: u16,
    pub months: u16,
    pub days: u16,
    pub hours: u16,
    pub minutes: u16,
    pub seconds: u16,
    pub hundredths: u16,
}

impl Time {
    pub fn new() -> Self {
        Time {
            years: 0,
            months: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            hundredths: 0,
        }
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
    }
}
