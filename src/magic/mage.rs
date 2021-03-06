use types::{ Spell };

pub fn spell(level: usize) -> Spell {
    match level {
        0  => Spell { name: "Magic Missile", level: 1, mana: 1, fail: 22 },
        1  => Spell { name: "Detect Monsters", level: 1, mana: 1, fail: 23 },
        2  => Spell { name: "Phase Door", level: 1, mana: 2, fail: 24 },
        3  => Spell { name: "Light Area", level: 1, mana: 2, fail: 26 },
        4  => Spell { name: "Cure Light Wounds", level: 3, mana: 3, fail: 25 },
        5  => Spell { name: "Find Hidden Traps/Doors", level: 3, mana: 3, fail: 55 },
        6  => Spell { name: "Stinking Cloud", level: 3, mana: 4, fail: 27 },
        7  => Spell { name: "Confusion", level: 3, mana: 4, fail: 30 },
        8  => Spell { name: "Lightning Bolt", level: 5, mana: 4, fail: 30 },
        9  => Spell { name: "Trap/Door Destruction", level: 5, mana: 5, fail: 30 },
        10 => Spell { name: "Sleep I", level: 5, mana: 5, fail: 30 },
        11 => Spell { name: "Cure Poison", level: 5, mana: 5, fail: 35 },
        12 => Spell { name: "Shadow Door", level: 7, mana: 6, fail: 35 },
        13 => Spell { name: "Remove Curse", level: 7, mana: 6, fail: 50 },
        14 => Spell { name: "Frost Bolt", level: 7, mana: 6, fail: 40 },
        15 => Spell { name: "Create Food", level: 7, mana: 6, fail: 40 },
        16 => Spell { name: "Infravision", level: 7, mana: 6, fail: 40 },
        17 => Spell { name: "Invisibility", level: 7, mana: 6, fail: 35 },
        18 => Spell { name: "Turn Stone to Mud", level: 9, mana: 7, fail: 44 },
        19 => Spell { name: "Recharge Item I", level: 9, mana: 7, fail: 75 },
        20 => Spell { name: "Sleep II", level: 9, mana: 7, fail: 45 },
        21 => Spell { name: "Phantasmal Force", level: 11, mana: 7, fail: 50 },
        22 => Spell { name: "Polymorph Other", level: 11, mana: 7, fail: 45 },
        23 => Spell { name: "Identify", level: 11, mana: 7, fail: 99 },
        24 => Spell { name: "Ring of Frost", level: 13, mana: 7, fail: 45 },
        25 => Spell { name: "Sleep III", level: 13, mana: 7, fail: 50 },
        26 => Spell { name: "Hold Monster", level: 15, mana: 9, fail: 50 },
        27 => Spell { name: "Fire Bolt", level: 15, mana: 9, fail: 50 },
        28 => Spell { name: "Slow Creature", level: 17, mana: 9, fail: 50 },
        29 => Spell { name: "Protection from Magic", level: 17, mana: 9, fail: 55 },
        30 => Spell { name: "Frost Ball", level: 19, mana: 12, fail: 55 },
        31 => Spell { name: "Death Spell", level: 19, mana: 18, fail: 55 },
        32 => Spell { name: "Ring of Fire", level: 21, mana: 12, fail: 60 },
        33 => Spell { name: "Recharge Item II", level: 21, mana: 12, fail: 90 },
        34 => Spell { name: "Teleport Other", level: 23, mana: 15, fail: 60 },
        35 => Spell { name: "Haste Self", level: 25, mana: 15, fail: 65 },
        36 => Spell { name: "Fire Ball", level: 28, mana: 18, fail: 65 },
        37 => Spell { name: "Power Word: Destruction", level: 31, mana: 21, fail: 80 },
        38 => Spell { name: "Power Word: Kill", level: 34, mana: 25, fail: 80 },
        39 => Spell { name: "Genocide", level: 37, mana: 25, fail: 95 },
        _  => Spell { name: "", level: 99, mana: 99, fail: 0},
    }
}
