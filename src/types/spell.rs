use libc;


#[repr(C)]
pub struct Spell {
    sname: [libc::c_char; 28],
    slevel: libc::uint8_t,
    smana: libc::uint8_t,
    sfail: libc::uint8_t,
    learned: libc::uint8_t,
}

impl Spell {
    pub fn learned(&self) -> bool {
        return self.learned != 0
    }

    pub fn set_learned(&mut self, yn: bool) {
        self.learned = if yn { 255 } else { 0 };
    }
}
