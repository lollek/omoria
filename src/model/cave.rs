use libc;

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Cave {
    #[serde(skip_serializing, default)]
    pub cptr: libc::uint8_t,
    #[serde(skip_serializing, default)]
    pub tptr: libc::uint8_t,
    pub fval: libc::uint8_t,
    pub fopen: libc::uint8_t,
    pub fm: libc::uint8_t,
    pub pl: libc::uint8_t,
    #[serde(skip_serializing, default)]
    pub tl: libc::uint8_t,
    #[serde(skip_serializing, default)]
    pub moved: libc::uint8_t,
    #[serde(skip_serializing, default)]
    pub oct: libc::uint8_t,
    #[serde(skip_serializing, default)]
    pub h2o: libc::uint8_t,
}
