use std::ops::Range;

pub enum Stat {
    Strength = 0,
    Intelligence = 1,
    Wisdom = 2,
    Dexterity = 3,
    Constitution = 4,
    Charisma = 5,
}

pub fn stats_iter() -> Range<usize> {
    (Stat::Strength as usize)..(Stat::Charisma as usize + 1)
}

pub struct StatBlock {
    pub strength: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub charisma: u8,
}

impl StatBlock {
    pub fn new(initial_value: u8) -> StatBlock {
        StatBlock {
            strength:       initial_value,
            intelligence:   initial_value,
            wisdom:         initial_value,
            dexterity:      initial_value,
            constitution:   initial_value,
            charisma:       initial_value,
        }
    }

    pub fn get(&self, stat: Stat) -> u8 {
        match stat {
            Stat::Strength => self.strength,
            Stat::Intelligence => self.intelligence,
            Stat::Wisdom => self.wisdom,
            Stat::Dexterity => self.dexterity,
            Stat::Constitution => self.constitution,
            Stat::Charisma => self.charisma,
        }
    }
    pub fn get_pos(&self, stat: usize) -> u8 {
        match stat {
            0 => self.strength,
            1 => self.intelligence,
            2 => self.wisdom,
            3 => self.dexterity,
            4 => self.constitution,
            5 => self.charisma,
            _ => panic!(),
        }
    }
    pub fn set_pos(&mut self, stat: usize, new_value: u8) {
        match stat {
            0 => self.strength = new_value,
            1 => self.intelligence = new_value,
            2 => self.wisdom = new_value,
            3 => self.dexterity = new_value,
            4 => self.constitution = new_value,
            5 => self.charisma = new_value,
            _ => panic!(),
        }
    }
}

impl From<[u8; 6]> for StatBlock {
    fn from(array: [u8; 6]) -> Self {
        StatBlock {
            strength:       array[0],
            intelligence:   array[1],
            wisdom:         array[2],
            dexterity:      array[3],
            constitution:   array[4],
            charisma:       array[5],
        }
    }
}

impl From<Vec<u8>> for StatBlock {
    fn from(array: Vec<u8>) -> Self {
        assert!(array.len() == 6);
        StatBlock {
            strength:       array[0],
            intelligence:   array[1],
            wisdom:         array[2],
            dexterity:      array[3],
            constitution:   array[4],
            charisma:       array[5],
        }
    }
}
