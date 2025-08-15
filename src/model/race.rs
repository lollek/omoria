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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        serde_json::to_string(&Race::Human).expect("Failed to serialize Race");
    }
}