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
