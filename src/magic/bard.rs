use types::{ Spell };

pub fn spell(level: usize) -> Spell {
    match level {
        0  => Spell { name: "Detect Monsters", level: 1, mana: 1, fail: 35},
        1  => Spell { name: "Battle Song", level: 1, mana: 2, fail: 35},
        2  => Spell { name: "Blink", level: 1, mana: 2, fail: 35},
        3  => Spell { name: "Light Area", level: 1, mana: 2, fail: 35},
        4  => Spell { name: "Detect Hidden Doors/Traps", level: 3, mana: 3, fail: 45},
        5  => Spell { name: "Magical Jig", level: 3, mana: 4, fail: 40},
        6  => Spell { name: "Detect Magic", level: 3, mana: 4, fail: 60},
        7  => Spell { name: "Minor Cure", level: 3, mana: 4, fail: 40},
        8  => Spell { name: "Battle Dance", level: 5, mana: 5, fail: 45},
        9  => Spell { name: "Charm Monsters", level: 5, mana: 5, fail: 40},
        10 => Spell { name: "Detect Curse", level: 5, mana: 9, fail: 50},
        11 => Spell { name: "Detect Invisible", level: 7, mana: 8, fail: 45},
        12 => Spell { name: "Cure Poison", level: 7, mana: 8, fail: 45},
        13 => Spell { name: "Invisibility", level: 7, mana: 11, fail: 55},
        14 => Spell { name: "Teleport Self", level: 9, mana: 10, fail: 50},
        15 => Spell { name: "Infravision", level: 9, mana: 11, fail: 55},
        16 => Spell { name: "Physical Humor", level: 9, mana: 11, fail: 55},
        17 => Spell { name: "Recharge Item", level: 11, mana: 11, fail: 85},
        18 => Spell { name: "Remove Curse", level: 13, mana: 12, fail: 55},
        19 => Spell { name: "Legend Lore", level: 11, mana: 13, fail: 99},
        20 => Spell { name: "Mass Charm", level: 11, mana: 12, fail: 55},
        21 => Spell { name: "Detect Treasure", level: 13, mana: 11, fail: 80},
        22 => Spell { name: "Detect Object", level: 13, mana: 11, fail: 80},
        23 => Spell { name: "Resist Petrification", level: 15, mana: 12, fail: 60},
        24 => Spell { name: "Create Food and Drink", level: 15, mana: 13, fail: 60},
        25 => Spell { name: "Panic", level: 15, mana: 15, fail: 60},
        26 => Spell { name: "Word of Recall", level: 17, mana: 15, fail: 60},
        27 => Spell { name: "Protection from Nature", level: 17, mana: 16, fail: 65},
        28 => Spell { name: "See Invisible", level: 17, mana: 16, fail: 60},
        29 => Spell { name: "Magic Mapping", level: 19, mana: 18, fail: 65},
        30 => Spell { name: "Joke of Death", level: 19, mana: 18, fail: 60},
        31 => Spell { name: "Battle Frenzy", level: 19, mana: 18, fail: 80},
        32 => Spell { name: "Slow Creature", level: 21, mana: 19, fail: 65},
        33 => Spell { name: "Resist Charm", level: 23, mana: 22, fail: 65},
        34 => Spell { name: "Item Lore", level: 25, mana: 20, fail: 90},
        35 => Spell { name: "Song of Protection", level: 27, mana: 25, fail: 70},
        36 => Spell { name: "Last Laugh", level: 29, mana: 23, fail: 70},
        37 => Spell { name: "Teleport Level", level: 31, mana: 27, fail: 75},
        38 => Spell { name: "Clairvoyance", level: 35, mana: 29, fail: 92},
        39 => Spell { name: "Song of Power", level: 39, mana: 32, fail: 97},
        _  => Spell { name: "", level: 99, mana: 99, fail: 0},
    }
}