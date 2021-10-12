
#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Sex {
    Female,
    Male,
}

impl Sex {
    pub fn to_string(&self) -> &'static str {
        match self {
            Sex::Female => "Female",
            Sex::Male => "Male",
        }
    }
}

impl From<char> for Sex {
    fn from(ch: char) -> Self {
        match ch {
            'F' => Sex::Female,
            'M' => Sex::Male,
            _ => panic!(),
        }
    }
}
