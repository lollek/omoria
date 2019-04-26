pub enum Stat {
    Strength = 0,
    Intelligence = 1,
    Wisdom = 2,
    Dexterity = 3,
    Constitution = 4,
    Charisma = 5,
}

impl From<u8> for Stat {
    fn from(stat: u8) -> Self {
        match stat {
            0 => Stat::Strength,
            1 => Stat::Intelligence,
            2 => Stat::Wisdom,
            3 => Stat::Dexterity,
            4 => Stat::Constitution,
            5 => Stat::Charisma,
            _ => panic!(),
        }
    }
}

pub fn stats_iter() -> impl Iterator<Item=usize> {
    (Stat::Strength as usize)..(Stat::Charisma as usize + 1)
}

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
    pub fn get_pos(&self, stat: usize) -> i16 {
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
    pub fn set_pos(&mut self, stat: usize, new_value: i16) {
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

impl From<[i32; 6]> for StatBlock {
    fn from(array: [i32; 6]) -> Self {
        StatBlock {
            strength:       array[0] as i16,
            intelligence:   array[1] as i16,
            wisdom:         array[2] as i16,
            dexterity:      array[3] as i16,
            constitution:   array[4] as i16,
            charisma:       array[5] as i16,
        }
    }
}

impl From<[i16; 6]> for StatBlock {
    fn from(array: [i16; 6]) -> Self {
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

impl From<[i8; 6]> for StatBlock {
    fn from(array: [i8; 6]) -> Self {
        StatBlock {
            strength:       array[0].into(),
            intelligence:   array[1].into(),
            wisdom:         array[2].into(),
            dexterity:      array[3].into(),
            constitution:   array[4].into(),
            charisma:       array[5].into(),
        }
    }
}

impl From<[u8; 6]> for StatBlock {
    fn from(array: [u8; 6]) -> Self {
        StatBlock {
            strength:       array[0] as i16,
            intelligence:   array[1] as i16,
            wisdom:         array[2] as i16,
            dexterity:      array[3] as i16,
            constitution:   array[4] as i16,
            charisma:       array[5] as i16,
        }
    }
}

impl From<Vec<i16>> for StatBlock {
    fn from(array: Vec<i16>) -> Self {
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
