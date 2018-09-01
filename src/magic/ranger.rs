use types::{ Spell };

pub fn spell(level: usize) -> Spell {
    match level {
        0  => Spell { name: "Moon Beam", level: 1, mana:  2, fail: 20 },
        1  => Spell { name: "Detect Monster", level: 1, mana: 2, fail: 33 },
        2  => Spell { name: "Battle Song", level: 2, mana: 3, fail: 35 },
        3  => Spell { name: "Light", level: 2, mana: 3, fail: 35 },
        4  => Spell { name: "Minor Cure", level: 3, mana: 4, fail: 40 },
        5  => Spell { name: "Find Safe Path", level: 4, mana: 5, fail: 45 },
        6  => Spell { name: "Magical Jig", level: 4, mana: 5, fail: 45 },
        7  => Spell { name: "Warp Wood", level: 4, mana: 5, fail: 45 },
        8  => Spell { name: "Battle Dance", level: 5, mana: 5, fail: 40 },
        9  => Spell { name: "Cure Poison", level: 5, mana: 5, fail: 48 },
        10 => Spell { name: "Charm", level: 7, mana: 5, fail: 35 },
        11 => Spell { name: "Detect Curse", level: 7, mana: 6, fail: 45 },
        12 => Spell { name: "Summon Insects", level: 7, mana: 7, fail: 50 },
        13 => Spell { name: "Call Lightning", level: 9, mana: 9, fail: 55 },
        14 => Spell { name: "Magic Resistance", level: 9, mana: 10, fail: 45 },

        16 => Spell { name: "Create Food", level: 9, mana: 10, fail: 55 },
        17 => Spell { name: "Remove Curse", level: 11, mana: 10, fail: 45 },
        18 => Spell { name: "Infravision", level: 11, mana: 10, fail: 50 },
        19 => Spell { name: "Major Cure", level: 11, mana: 11, fail: 50 },
        20 => Spell { name: "Resist Petrification", level: 13, mana: 11, fail: 55 },
        21 => Spell { name: "Transplant", level: 13, mana: 11, fail: 45 },

        23 => Spell { name: "Dispel Magic", level: 15, mana: 11, fail: 58 },
        24 => Spell { name: "Fire Stream", level: 15, mana: 11, fail: 70 },
        25 => Spell { name: "Protection from Nature", level: 17, mana: 12, fail: 55 },
        26 => Spell { name: "Stone to Mud", level: 17, mana: 12, fail: 55 },
        27 => Spell { name: "Goodberry", level: 19, mana: 14, fail: 65 },
        28 => Spell { name: "Creeping Doom", level: 37, mana: 21, fail: 65 },
        29 => Spell { name: "Pillar of Fire", level: 23, mana: 15, fail: 65 },
        30 => Spell { name: "Word of Recall", level: 25, mana: 15, fail: 50 },
        31 => Spell { name: "Lightning Ball", level: 27, mana: 15, fail: 90 },
        32 => Spell { name: "Word of Blindness", level: 29, mana: 16, fail: 55 },
        33 => Spell { name: "Protection from Monsters", level: 31, mana: 20, fail: 60 },


        36 => Spell { name: "Resist Charm", level: 33, mana: 22, fail: 65 },
        37 => Spell { name: "Battle Frenzy", level: 35, mana: 25, fail: 65 },
        38 => Spell { name: "Dispel Monster", level: 37, mana: 30, fail: 70 },
        _  => Spell { name: "", level: 99, mana: 99, fail: 0 },
    }
}
