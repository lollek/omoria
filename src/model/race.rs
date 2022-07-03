use enum_iterator;

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize, enum_iterator::Sequence)]
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
    Dryad
}

impl Race {
    pub fn iter() -> impl Iterator<Item=Race> {
        return enum_iterator::all::<Race>()
    }

}