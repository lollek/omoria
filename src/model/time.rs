use libc;

#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Time {
    pub years: libc::uint16_t,
    pub months: libc::uint16_t,
    pub days: libc::uint16_t,
    pub hours: libc::uint16_t,
    pub minutes: libc::uint16_t,
    pub seconds: libc::uint16_t ,
    pub hundredths: libc::uint16_t,
}
