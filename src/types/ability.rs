#[derive(PartialEq, Clone, Copy)]
pub enum Ability {
    Rage,
}

impl Ability {
    pub fn name(&self) -> &'static str {
        match self {
            Ability::Rage => "Rage",
        }
    }
}
