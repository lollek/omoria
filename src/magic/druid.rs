use types::{ Spell };

pub fn spell(level: usize) -> Spell {
    match level {
        0  => Spell { name: "Moon Beam", level: 1, mana: 1, fail: 15 },
        1  => Spell { name: "Detect Monster", level: 1, mana: 1, fail: 15 },
        2  => Spell { name: "Battle Song", level: 1, mana: 2, fail: 20 },
        3  => Spell { name: "Light", level: 1, mana: 2, fail: 25 },
        4  => Spell { name: "Minor Cure", level: 3, mana: 3, fail: 25 },
        5  => Spell { name: "Find Safe Path", level: 3, mana: 3, fail: 28 },
        6  => Spell { name: "Magical Jig", level: 3, mana: 3, fail: 27 },
        7  => Spell { name: "Warp Wood", level: 3, mana: 4, fail: 29 },
        8  => Spell { name: "Battle Dance", level: 5, mana: 4, fail: 30 },
        9  => Spell { name: "Cure Poison", level: 5, mana: 4, fail: 30 },
        10 => Spell { name: "Charm", level: 5, mana: 5, fail: 45 },
        11 => Spell { name: "Detect Curse", level: 5, mana: 5, fail: 50 },
        12 => Spell { name: "Summon Insects", level: 7, mana: 5, fail: 35 },
        13 => Spell { name: "Call Lightning", level: 7, mana: 5, fail: 38 },
        14 => Spell { name: "Magic Resistance", level: 7, mana: 6, fail: 40 },
        15 => Spell { name: "Starlight", level: 7, mana: 6, fail: 40 },
        16 => Spell { name: "Create Food", level: 9, mana: 6, fail: 40 },
        17 => Spell { name: "Remove Curse", level: 9, mana: 6, fail: 55 },
        18 => Spell { name: "Infravision", level: 9, mana: 7, fail: 40 },
        19 => Spell { name: "Major Cure", level: 9, mana: 7, fail: 40 },
        20 => Spell { name: "Resist Petrification", level: 11, mana: 7, fail: 45 },
        21 => Spell { name: "Transplant", level: 11, mana: 8, fail: 55 },
        22 => Spell { name: "Sunray", level: 11, mana: 9, fail: 60 },
        23 => Spell { name: "Dispel Magic", level: 11, mana: 9, fail: 50 },
        24 => Spell { name: "Fire Stream", level: 11, mana: 9, fail: 65 },
        25 => Spell { name: "Protection from Nature", level: 13, mana: 9, fail: 40 },
        26 => Spell { name: "Stone to Mud", level: 13, mana: 9, fail: 50 },
        27 => Spell { name: "Goodberry", level: 15, mana: 11, fail: 50 },
        28 => Spell { name: "Creeping Doom", level: 33, mana: 17, fail: 65 },
        29 => Spell { name: "Pillar of Fire", level: 15, mana: 11, fail: 55 },
        30 => Spell { name: "Word of Recall", level: 17, mana: 11, fail: 60 },
        31 => Spell { name: "Lightning Ball", level: 17, mana: 12, fail: 55 },
        32 => Spell { name: "Word of Blindness", level: 19, mana: 12, fail: 55 },
        33 => Spell { name: "Protection from Monsters", level: 21, mana: 14, fail: 55 },
        34 => Spell { name: "Control Temperature", level: 23, mana: 17, fail: 60 },
        35 => Spell { name: "Ring of Fire", level: 25, mana: 19, fail: 75 },
        36 => Spell { name: "Resist Charm", level: 27, mana: 21, fail: 75 },
        37 => Spell { name: "Battle Frenzy", level: 29, mana: 23, fail: 70 },
        38 => Spell { name: "Dispel Monster", level: 31, mana: 25, fail: 80 },
        39 => Spell { name: "Note of Destruction", level: 34, mana: 27, fail: 85 },
        _  => Spell { name: "", level: 99, mana: 99, fail: 0},
    }
}
