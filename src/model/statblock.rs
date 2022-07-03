use model::Stat;

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct StatBlock {
    pub strength: i16,
    pub intelligence: i16,
    pub wisdom: i16,
    pub dexterity: i16,
    pub constitution: i16,
    pub charisma: i16,
}

impl StatBlock {
    pub fn new(initial_value: i16) -> StatBlock {
        StatBlock {
            strength:       initial_value,
            intelligence:   initial_value,
            wisdom:         initial_value,
            dexterity:      initial_value,
            constitution:   initial_value,
            charisma:       initial_value,
        }
    }

    pub fn get(&self, stat: Stat) -> i16 {
        match stat {
            Stat::Strength => self.strength,
            Stat::Intelligence => self.intelligence,
            Stat::Wisdom => self.wisdom,
            Stat::Dexterity => self.dexterity,
            Stat::Constitution => self.constitution,
            Stat::Charisma => self.charisma,
        }
    }
    pub fn set(&mut self, stat: Stat, new_value: i16) {
        match stat {
            Stat::Strength => self.strength = new_value,
            Stat::Intelligence => self.intelligence = new_value,
            Stat::Wisdom => self.wisdom = new_value,
            Stat::Dexterity => self.dexterity = new_value,
            Stat::Constitution => self.constitution = new_value,
            Stat::Charisma => self.charisma = new_value,
        }
    }
}