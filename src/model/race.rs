use enum_iterator;

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize, enum_iterator::Sequence, Debug)]
pub enum Race {
    Human,
    HalfElf,
    Elf,
    Halfling,
    Gnome,
    Dwarf,
    HalfOrc,
    HalfTroll,
    Phraint,
    Dryad,
}

impl Race {
    pub fn iter() -> impl Iterator<Item = Race> {
        enum_iterator::all::<Race>()
    }
}
